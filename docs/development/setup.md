# Development setup

Target platform: Arch Linux with a Hyprland session. Parts marked **unverified** are plans that have not yet been run in this repository.

## 1. Prerequisites

```bash
sudo pacman -S --needed base-devel git rustup just shellcheck cargo-deny qt6-declarative python
```

`qt6-declarative` provides `qmllint`. For shell work also install `quickshell hyprland uwsm`. Optional: `github-cli` (for `just pr`), `cargo-audit`, `git-cliff`, `socat` (to debug the socket), and `qemu-desktop` with `devtools` (VM tests and clean-chroot package builds, needed from the first package onward).

The Rust toolchain is pinned in `core/rust-toolchain.toml`. Install it with:

```bash
cd core && rustup toolchain install   # without arguments: installs the toolchain from rust-toolchain.toml
```

## 2. Clone and install the hooks

```bash
git clone https://github.com/sya17/either.git
cd either
just setup    # enables the commit-msg, pre-commit and pre-push hooks
just check    # every gate a pull request must pass
```

`core/Cargo.lock` is committed. Every dependency change updates it in the same pull request, and builds and tests use `--locked`.

## 3. Everyday commands

| Command | Purpose |
| --- | --- |
| `just` | list all recipes |
| `just check` | every pull request gate: format, lint, tests, deny, docs, language, tokens, QML literals, SPDX headers, scope |
| `just fmt` | format the Rust code |
| `just test` | run the workspace tests |
| `just pr "<title>" <body-file>` | open a pull request; the title and body are checked like a commit message |

Recipes such as `just dev`, `just pkg`, `just iso`, and `just vm` are added when the work that needs them begins.

## 4. Running the project

Nothing is runnable yet. The order that will apply in 0.1.0:

| Once this exists | Command |
| --- | --- |
| `eitherd` | `cd core && EITHER_RUNTIME_DIR="$XDG_RUNTIME_DIR/either-dev" EITHER_LOG=debug cargo run -p eitherd` |
| `either ipc ping` | `cd core && EITHER_RUNTIME_DIR="$XDG_RUNTIME_DIR/either-dev" cargo run -p either-cli -- ipc ping` |
| the minimal bar | `quickshell -p shell` |
| the local PKGBUILD | `cd packaging/pkgbuilds/either-core && makepkg -si` |

`EITHER_RUNTIME_DIR` keeps the development socket apart from the real `eitherd` of your session, so the two never collide. Use a directory under `$XDG_RUNTIME_DIR` or one made with `mktemp -d`, never a fixed name in `/tmp`. The variable is part of the design and is implemented in `eitherd`.

### Nested Hyprland (unverified)

The plan is to develop the shell inside a Hyprland instance that runs as a window in the current session, so the daily session is never at risk. This has not been verified yet. Until it is, do not start nested or headless Hyprland from scripts; the maintainer tries it and records the commands and limits here.

## 5. Debugging

| Need | How |
| --- | --- |
| Daemon logs | `journalctl --user -u eitherd -f`; set the level with `EITHER_LOG` (`trace`, `debug`, `info`) |
| Talk to the core socket | `socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/either/core.sock`, then send one JSON line, for example `{"jsonrpc":"2.0","id":1,"method":"hello","params":{"proto":1,"client":"debug","version":"0.1.0"}}` |
| Hyprland state | `hyprctl`; log at `$XDG_RUNTIME_DIR/hypr/<instance>/hyprland.log` |
| Crashes | `coredumpctl`; Hyprland crash reports at `~/.cache/hyprland/hyprlandCrashReport<PID>.txt` |
| Failed units | `systemctl --user --failed` |
| IPC contract | JSON Schema is generated from the structs in `either-ipc`; the snapshots live next to the crate's tests |

## 6. Keeping your daily session safe

A development machine is often also a daily desktop, so test against throwaway copies:

- run `eitherd` and the dev shell with a separate `EITHER_RUNTIME_DIR`, never the real socket;
- point `XDG_CONFIG_HOME` and `XDG_STATE_HOME` at a temporary directory (`mktemp -d`); do not overwrite `~/.config/hypr`, `~/.config/either`, or `~/.local/state/either`;
- do not test a lock screen in the session you are using, because a failed unlock locks you out; use a VM or a nested session;
- restore any state you changed while testing (theme, do-not-disturb, power profile).

## 7. Local hook extensions

The hooks in `scripts/hooks/` run the same checks for everyone. A maintainer or an organisation can add policy checks of their own without touching the repository: an executable named after the hook (`commit-msg`, `pre-commit`, `pr-text`, `check`) placed in `<git dir>/hooks.d/` or in `~/.config/either-dev/hooks.d/` runs after the built-in checks, and a non-zero exit blocks the operation. `just check` runs the `check` hook through `just local-check`.

To make a missing extension an error on a machine that relies on one:

```bash
git config --global either.requireLocalHooks true
```

Contributors do not need extensions and are not affected.

## 8. Contributing

Read [CONTRIBUTING.md](../../CONTRIBUTING.md), then pick an item from the backlog of the current release.
