# core

The Cargo workspace for the Rust components. Run cargo commands from this folder; `just` already does that.

| Package | Purpose | Gets its code in |
| --- | --- | --- |
| `crates/either-ipc` | the IPC protocol contract: messages, events, versions, JSON Schema | 0.1.0 |
| `crates/either-hypr` | the Hyprland socket client (requests and events) | 0.1.0 |
| `crates/either-dbus` | the D-Bus bridge (zbus) | 0.1.0 if the D-Bus name decision requires it; MPRIS in 0.3.0 |
| `crates/either-config` | the `config.toml` schema and validation | 0.2.0 |
| `crates/either-theme` | tokens to QML, Hyprland, GTK, Qt, and the terminal | 0.2.0 |
| `eitherd` | the core daemon | 0.1.0 |
| `either-cli` | the package that builds the `either` binary (`[[bin]] name = "either"`) | 0.1.0 |

- No package is named plain `either`, because a popular crate already uses that name.
- The crates are skeletons so the workspace compiles; their code arrives with the release in the right-hand column.
- `Cargo.lock` is committed and builds use `--locked`.
- Coding conventions: [docs/development/coding-standards.md](../docs/development/coding-standards.md).
