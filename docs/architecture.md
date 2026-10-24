# Architecture

either is a layered system. One Rust daemon, `eitherd`, is the single owner of state. Everything else, the shell, the CLI, and later the settings and lock applications, is a client of that daemon and speaks to it through one versioned IPC contract.

## Layers

| Layer | Components | Language |
| --- | --- | --- |
| Kernel and system | Linux (with `linux-lts` as a fallback), systemd, Btrfs with snapper | C (upstream) |
| Session and display | Hyprland, xdg-desktop-portal-hyprland, PipeWire and WirePlumber, uwsm | C++ (upstream) |
| Core daemon | `eitherd` | Rust |
| Shell | `either-shell` on Quickshell, C++ plugins for heavy paths | QML, C++ |

Hyprland is the foundation and is not forked. Rust and QML run in separate processes. C++ is used only for QML plugins on paths where QML is measurably not enough.

## Processes

Every component is a systemd user unit, not an `exec-once` entry. That gives a defined start order, `Restart=on-failure`, central logs in journald, and per-process resource limits.

| Process | Language | Started by | If it crashes |
| --- | --- | --- | --- |
| `Hyprland` | C++ | uwsm (user unit) | the session ends; Btrfs snapshots stay safe |
| `eitherd.service` | Rust | systemd, after `graphical-session.target` | restarts automatically; clients reconnect |
| `either-shell.service` (Quickshell) | QML, C++ | systemd, `Requires=eitherd.service` | restarts itself and fetches state again from `eitherd` |
| `either` (CLI) | Rust | the user or a keybinding | nothing is lost |

The session is started by `uwsm`, which activates `graphical-session.target`.

## Repository layout

A single monorepo holds code, default configuration, packaging, and ISO profiles, so one commit always describes one complete version of the system. Directories appear when their release begins.

```text
either/
├── core/                     Cargo workspace (Rust)
│   ├── crates/
│   │   ├── either-ipc/       protocol contract: messages, events, versions
│   │   ├── either-hypr/      Hyprland socket client (requests and events)
│   │   ├── either-dbus/      D-Bus bridge
│   │   ├── either-config/    config schema and validation
│   │   └── either-theme/     tokens to QML, Hyprland, GTK, Qt, terminal
│   ├── eitherd/              the daemon
│   └── either-cli/           the package that builds the `either` binary
├── shell/                    Quickshell configuration (QML)
│   ├── components/           component library (Button, Toggle, Tile, ...)
│   ├── modules/              bar, launcher, notifications, lock, OSD
│   ├── services/             QML singletons: IPC client and Quickshell API wrappers
│   └── theme/                generated Tokens.qml
├── shell-native/             C++ QML plugins (only when measured to be needed)
├── design/tokens/            the single source of colors, typography, motion
├── dotfiles/                 default configuration: hypr, gtk, qt, kitty
├── scripts/                  small glue and quality checks
├── packaging/                PKGBUILDs, the meta-package, the repository
├── iso/  installer/          archiso and archinstall profiles
├── tests/                    integration tests (QEMU, IPC contract)
└── docs/
```

No Cargo package may be named `either`, because a popular crate already uses that name. The `either` binary is produced by the `either-cli` package through `[[bin]] name = "either"`.

## Configuration

One `config.toml`, validated by `either-config`, with a `schema` field for automatic migration. Package upgrades never overwrite changes made by the user.

| Layer | Location | Owner | Changes when |
| --- | --- | --- | --- |
| Compiled defaults | inside `either-config` | code | a new release ships |
| Distribution defaults | `/usr/share/either/` | the pacman package | the package is upgraded |
| User overrides | `~/.config/either/` | the user | never overwritten |
| Generated output | `~/.local/state/either/` | `eitherd` | the theme or config changes |

- `either config check` points to the faulty line. When validation fails, `eitherd` keeps the last valid config and publishes `config.error`.
- File changes are detected with inotify and applied without a restart (`config.changed`).
- Changes from the UI go through `config.set`, preserving comments and ordering.
- Before a schema migration the file is backed up as `config.toml.bak-<version>`.

Hyprland's own config is written in Lua ([ADR 0004](adr/0004-hyprland-config-in-lua.md)). The user stub `~/.config/hypr/hyprland.lua` loads, in order, the distribution defaults, the generated output, and finally the user's own file.

## IPC

All event sources (Hyprland, D-Bus) are normalized by `eitherd` into one `Event` format. Clients subscribe to topics on one socket.

| Channel | Location | Format |
| --- | --- | --- |
| Core IPC | `$XDG_RUNTIME_DIR/either/core.sock` (mode 0600) | NDJSON, JSON-RPC 2.0 with subscription notifications |
| Hyprland commands | `$XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket.sock` | text request and response (like `hyprctl`) |
| Hyprland events | `.../.socket2.sock` | one line per event, `EVENT>>DATA` |
| D-Bus | session and system bus | standard freedesktop interfaces |

The shell may read workspace and window state straight from Quickshell's Hyprland module, so animations do not wait for an IPC hop. That access is read-only. Every command goes through `eitherd`.

### Contract

- One JSON object per line, at most 1 MiB (1,048,576 bytes, not counting the `\n`). A sender refuses a longer message; a reader reports a longer line and skips the rest of it. Large data, such as images, is exchanged as a file path.
- The first message must be `hello {proto, client, version}`. The daemon rejects an unsupported major version with `version_mismatch`.
- Every event has the shape `{v, seq, ts, topic, data}`, sent as the params of an `event` notification. `ts` is in milliseconds since the Unix epoch. `seq` only grows, so a client can detect a missed event.
- `subscribe` and `unsubscribe` take `{topics}` and return an empty object; `state.snapshot` returns `{seq, domains}`. Results are objects so that fields can be added within one `proto`.
- A response carries exactly one of `result` or `error`. For the codes below, the error `message` is the error name, such as `version_mismatch`.
- Message types are Rust structs in `either-ipc`. JSON Schema is generated from them (`schemars`) and kept as a snapshot in `core/crates/either-ipc/tests/snapshots/`; a change to the wire format fails the tests until the snapshot is updated.

```json
{"jsonrpc":"2.0","id":1,"method":"hello","params":{"proto":1,"client":"shell","version":"0.1.0"}}
{"jsonrpc":"2.0","id":1,"result":{"proto":1,"daemon":"0.1.0","seq":4812}}
{"jsonrpc":"2.0","id":2,"method":"subscribe","params":{"topics":["hypr.*","media.player.*"]}}
{"jsonrpc":"2.0","method":"event","params":{"v":1,"seq":4813,"ts":1760000000123,"topic":"hypr.workspace.focused","data":{"id":3,"monitor":"DP-1"}}}
```

| Method | Purpose |
| --- | --- |
| `hello` | handshake and version negotiation |
| `state.snapshot` | full state per domain plus the latest `seq` |
| `subscribe`, `unsubscribe` | choose topics (globs) |
| `hypr.dispatch` | a Hyprland dispatcher from an allowlist |
| `theme.set`, `theme.build` | change and generate the theme |
| `config.get`, `config.set` | read and change validated config |
| `media.control` | play, pause, next through MPRIS |

| Error code | Name | Meaning |
| --- | --- | --- |
| -32700, -32600, -32601, -32602 | standard JSON-RPC | parse error, invalid request, unknown method, invalid params |
| -32001 | `version_mismatch` | the major `proto` version is not supported |
| -32002 | `unauthorized` | the client may not call this method |
| -32003 | `not_ready` | a bridge (Hyprland, D-Bus) is not connected yet |
| -32004 | `rate_limited` | too many requests |

### Compatibility

- Within one major `proto`, changes may only add optional fields, methods, or topics. Clients must ignore what they do not know.
- Removing or renaming anything requires a new `proto`. The daemon supports the current and the previous major version for one release cycle.
- Binary serialization is added only if profiling proves it necessary, negotiated in `hello`.

### Resilience

- Each subscriber has a bounded queue. A slow client receives `resync` and fetches a snapshot, without holding back other clients.
- After reconnecting, a client asks for `state.snapshot` and continues from the latest `seq`.
- Connections to Hyprland and D-Bus reconnect with backoff, and their state is published as `core.health`.

## Security model

All daemons run as the regular user, never as root. Privileged actions go through polkit.

- **Sockets.** Created in `$XDG_RUNTIME_DIR/either/` (a 0700 directory owned by the user) with mode 0600. The daemon reads the peer's uid and pid with `SO_PEERCRED` and rejects other uids.
- **A single uid is not a trust boundary.** Every application of the user runs under the same uid, so `SO_PEERCRED` only filters other users. Dangerous methods are restricted by client identity.
- **Dispatchers.** `hypr.dispatch` uses an allowlist. There is no `exec` through IPC; launching an application uses a dedicated method that accepts a `.desktop` file name.
- **Secrets.** Stored in the Secret Service (GNOME Keyring or KeePassXC) through `oo7`, never in `config.toml`, logs, or arguments.
- **Privileges.** No `sudo` in any daemon. Network, Bluetooth, and power changes go through D-Bus services protected by polkit.
- **Polkit agent.** Provided by the shell so dialogs look consistent.
- **Supply chain.** Packages are built in a clean chroot and signed. `cargo-deny` checks advisories and licenses, and every version is pinned.

The threat model is recorded as a decision record when it is written.
