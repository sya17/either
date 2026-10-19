# Coding standards

These rules apply to every contributor. A rule that is not backed by a decision record is a default of the project: change it with a pull request that explains why, or with a decision record if it touches the architecture.

## Principles

- **Keep it simple.** Reuse what is in the repository first, then the standard library, then dependencies already in use. A new dependency needs a reason in the pull request: its license passes `cargo-deny`, it is maintained, and a few lines of code cannot replace it. No abstraction for a single user.
- **Event-driven, no polling.** `eitherd` and the shell never check state in a loop; idle CPU stays close to zero.
- **One owner of state: `eitherd`.** The shell and the CLI keep no state that cannot be rebuilt from `state.snapshot`.
- **Fail closed.** A broken config does not break the desktop, IPC input that fails validation is rejected, and an approval that times out is a denial.
- **Comments explain why**, not what. No TODO without a reference to an issue or work item: `TODO(<id>)`.
- **One change, one reason.** A refactor and a feature do not share a pull request.

## Language

| What | Language |
| --- | --- |
| Code, identifiers, comments, commits, branch names, logs, error messages, CLI strings | English |
| Documentation (`docs/`, READMEs, decision records) and templates | English |
| Strings shown in the shell UI | English in code, wrapped in `qsTr()`; Indonesian and English translations arrive in 0.8.0 |

The product name is always written "either" in lowercase, including at the start of a sentence. `just language-check` verifies that tracked text files are English.

## Naming

| What | Rule | Example |
| --- | --- | --- |
| Cargo package | `either-<name>`; no package is named plain `either` | `either-ipc`, `either-cli` |
| Binary | `either` (CLI) and `eitherd` (daemon) | `[[bin]] name = "either"` |
| systemd unit | `either-<name>.service`; the daemon is `eitherd.service` | `either-shell.service` |
| pacman package | `either-<name>`; the meta-package is `either-desktop` | `either-core` |
| Environment variable | `EITHER_` plus UPPER_CASE | `EITHER_LOG`, `EITHER_RUNTIME_DIR` |
| CLI subcommand | `either <group> <action>`, lowercase | `either theme set`, `either ipc ping` |
| IPC method | `<domain>.<action>`, lowercase; multi-word with `_` | `hypr.dispatch`, `state.snapshot` |
| Event topic | `<domain>.<object>.<event>`, lowercase | `hypr.workspace.focused` |
| IPC error name | snake_case | `version_mismatch` |
| Config key | snake_case | `modules_left` |
| Rust code | rustc conventions: `snake_case`, `UpperCamelCase`, `SCREAMING_SNAKE_CASE` | |
| QML file | types in `PascalCase.qml`; module folders in lowercase | `components/Button.qml`, `modules/bar/` |
| Design token | semantic kebab-case as in `design/tokens/tokens.json`, never a value | `surface-1`, `fg-muted`, not `gray-800` |
| Script and document | kebab-case | `check-docs.py` |
| Decision record | `docs/adr/NNNN-title-in-kebab-case.md` | `0004-hyprland-config-in-lua.md` |
| Branch | `<type>/<topic-in-kebab-case>` | `feat/ipc-envelope` |

## Code structure

The folder map is in [architecture.md](../architecture.md#repository-layout). Things that are easy to miss:

- IPC message types are defined only in `core/crates/either-ipc`. Other crates import them and never copy them.
- Important logic is a Rust subcommand (`either theme set`, `either doctor`), so it can be tested and has `--help`. `scripts/` holds only short glue without state.
- Keybindings call `either ...`, never ad-hoc scripts.
- Generated files (`shell/theme/`, the output of `either-theme`) are not edited by hand; change their source in `design/tokens/`. Until `either-theme` generates `Tokens.qml`, that file is written by hand (see the QML section). Do not edit `tokens.json` for the sake of one component; changing a value or a name needs a pull request that explains why and runs `just tokens-check`.
- Distribution defaults live in `/usr/share/either/`, user files in `~/.config/either/` (never overwritten), and generated output in `~/.local/state/either/`. Code honours `XDG_*` and `EITHER_RUNTIME_DIR` so tests can redirect them.

## Rust

- Edition, version, and lints are set once in `core/Cargo.toml` (`[workspace.lints]`). A new crate must have `[lints] workspace = true` and use `*.workspace = true` for metadata. CI runs `clippy -D warnings`.
- `unsafe` is forbidden. An exception needs `#[allow(unsafe_code)]`, a `// SAFETY:` comment, and a mention in the pull request.
- Typed errors in library crates. No `unwrap()` in non-test code; `expect("reason the invariant holds")` only for an invariant that is truly guaranteed. The daemon must never panic because of input from a socket: validate at the boundary.
- Logging uses `tracing` to journald, with the level set by `EITHER_LOG`. Logs never contain secrets or raw payloads.
- A dependency used by more than one crate is declared once in `[workspace.dependencies]`. `Cargo.lock` is committed and builds use `--locked`.
- Time and sleeping: logic that depends on time takes `now` as a parameter. Tests never sleep for real.
- Unit tests live in the same file (`#[cfg(test)]`); integration tests in the crate's `tests/`. See the next section for the full rules.

## Tests

- Tests never touch real state: the `eitherd` socket, `XDG_*`, a D-Bus session, the network, or a running desktop. Use a temporary directory, a separate `EITHER_RUNTIME_DIR`, and a private bus (`dbus-run-session`).
- Tests are deterministic and independent: time is injected, there is no `sleep`, no fixed port, and no dependence on the order of other tests.
- New logic comes with a test that fails without the change and passes with it. A bug fix starts with a failing regression test.
- A test name states the behaviour (`handshake_rejects_unknown_major_version`). Assertions check the observable contract, not implementation details.
- A test is never deleted, `#[ignore]`d, or weakened to make it pass. A wrong test is fixed in the same pull request, with the reason written down.
- Mocks are for outer boundaries only (the Hyprland socket, D-Bus, the clock). A test that mocks the thing under test proves nothing.
- Snapshots (`insta`) are reviewed in the diff before they are accepted.
- A flaky test is a bug: it is tracked and fixed. There are no automatic retries that hide it.
- Whatever cannot be tested automatically (QML appearance, a desktop session, booting) is stated in the pull request as untested, with steps for the maintainer who has a desktop session.
- A green `cargo test` with zero tests is not evidence. The pull request names the tests that prove the change.

## Async and daemon code (Rust)

- No blocking calls in async tasks (large files, synchronous subprocesses, `std::thread::sleep`). Blocking work goes through `spawn_blocking` or a dedicated thread.
- Channels and queues are always bounded, and the behaviour when full is chosen explicitly (IPC contract: `resync` for a slow client).
- Every I/O operation that leaves the process (sockets, D-Bus, subprocesses) has a timeout.
- A spawned task is owned by its caller (`JoinSet` or a handle) and stops on shutdown. There are no orphan tasks.
- A lock (`Mutex`) is never held across an `.await`. Dropping a future never leaves half-finished state behind.
- Shutdown is orderly on SIGTERM and SIGINT: stop accepting connections, close the running ones, remove the socket, then exit. Readiness and stopping are reported to systemd through `sd_notify`.
- No polling; waiting is always on an event.

## Filesystem

- Config and state are written atomically: write a temporary file in the same directory, `fsync`, then `rename`. A backup before a migration follows the config rules below.
- Permissions are set at creation, not afterwards: runtime directories 0700, sockets and secret files 0600.
- Do not follow symlinks when writing or deleting in a directory others can write to: open with `O_NOFOLLOW`, or check the type and owner first.
- A stale socket is removed only if the file really is a socket, is owned by the same uid, and nothing is listening on it.
- Write only in either's own directories (`~/.config/either`, `~/.local/state/either`, `$XDG_RUNTIME_DIR/either`). Writing another application's files needs a separate decision and a backup.
- Tests and development commands use `mktemp -d` or `$XDG_RUNTIME_DIR`, never a fixed name in `/tmp`.

## Gates and licensing

- Gates are not weakened to make a change pass: lints, `cargo-deny`, hooks, `scripts/check-*`, the `justfile`, and tests are not disabled, loosened, or skipped. Changing a gate is its own pull request with a stated reason, not part of a feature change.
- `#[allow(...)]`, `// lint-ok:`, and `unsafe` carry a one-line reason and are mentioned in the pull request.
- A failing hook is fixed at its cause; `--no-verify` is not used.
- Every new code file (`.rs`, `.qml`, `.py`, `.sh`, and scripts with a shebang) starts with `SPDX-License-Identifier: GPL-3.0-or-later` ([ADR 0003](../adr/0003-code-license-gpl-3-or-later.md)). `just spdx-check` verifies it. Only the license identifier is required, not a copyright line.
- A new component (crate, top-level directory, binary) enters only through a decision record that changes the inventory. `just scope-check` rejects anything outside the list in `scripts/check-scope.py`; extending that list is a change to a gate.

## QML (shell)

- Every color, size, spacing, radius, and duration comes from tokens (`Tokens.qml`), never a literal ([ADR 0002](../adr/0002-design-system-as-token-source.md)). `just qml-check` fails on literal hex values, color names, durations, font sizes, radii, spacing, padding, and margins outside `shell/theme/`; a single line may opt out with `// lint-ok: reason`. Until `either-theme` generates `Tokens.qml` (0.2.0), the file is written by hand in `shell/theme/` with the exact token names from `tokens.json`, so migrating only replaces its content.
- The full design rules (color, typography, motion, components, states, UI copy) are in the [design system](design-system.md).
- Quickshell APIs are called only from `shell/services/`, which isolates the shell from a 0.x API that changes. `modules/` and `components/` use those services only.
- State arrives from `eitherd` through the IPC service, and commands always go through `eitherd`. The one exception: Quickshell's Hyprland module may **read** workspace and window state directly.
- After a connection recovers, fetch `state.snapshot` again. The shell must not crash when `eitherd` restarts.
- One panel is open at a time; Escape and a click outside close it.
- Accessibility: `Accessible.role` and `Accessible.name` on interactive controls. This is recommended now and becomes mandatory and linted once a decision record accepts it.
- Every component is checked in both either Dusk and either Dawn, and `qmllint` is clean.

## C++ (`shell-native/`)

Only for QML plugins on heavy paths. A pull request must attach a measurement showing that QML is not enough. No business logic. Built with CMake and Ninja. There is no C++ code yet, and no measured need.

## Shell scripts

POSIX `sh`, passing `shellcheck`, short glue without state. The git hooks in `scripts/hooks/` use bash and also pass `shellcheck`.

## Config and generators

- One `config.toml` with a `schema` field. A structure change raises `schema`, ships an automatic migration, and makes a backup as `config.toml.bak-<version>`.
- A key that will be removed is deprecated at least one minor release earlier, and `either config check` warns about it.
- Hyprland's config is written in Lua ([ADR 0004](../adr/0004-hyprland-config-in-lua.md)). The Lua API names and the way `require()` resolves modules are checked against the installed Hyprland before a generator is considered correct.

## IPC contract

- The envelope is locked from 0.1.0: NDJSON, JSON-RPC 2.0, `hello` as the first message, events as `{v, seq, ts, topic, data}`, at most 1 MiB per line.
- Within one `proto` version, only optional fields, methods, and topics may be added. Clients ignore what they do not know. Removing or renaming anything raises `proto`.
- A contract change carries the `ipc-change` label, updates the JSON Schema snapshot in the same pull request, and updates the IPC documentation.
- `hypr.dispatch` uses an allowlist. There is no `exec` through IPC; launching an application uses a dedicated method that takes a `.desktop` file name.

## Security in code

- A single uid is not a trust boundary: `SO_PEERCRED` only filters other uids. Dangerous methods are restricted by client identity.
- Sockets live in `$XDG_RUNTIME_DIR/either/` (a 0700 directory) with mode 0600.
- External processes run with an argument list, never through a shell. Text from outside is preceded by `--`.
- Text from outside (window titles, connection names, notification bodies) is escaped before it enters markup (Pango, rich text). Config values that end up in Lua are escaped as strings.
- Secrets live only in the Secret Service (`oo7`): never in `config.toml`, logs, arguments, or the repository.
- No `sudo` in the daemon. Privileges go through polkit.
- No telemetry in 1.0. The only built-in network connection is the update check.

## Documentation

- Behaviour changes update the documents that describe them, in the same pull request. User documentation describes what is implemented, not what is planned.
- Keybinding, config, and IPC documentation is generated from its source (target 0.9.0), not written by hand.
- Architecture decisions are written as decision records in `docs/adr/`, not in code comments or commit messages.
