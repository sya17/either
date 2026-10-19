# shell

The Quickshell configuration (QML). The planned structure:

| Folder | Purpose |
| --- | --- |
| `components/` | the component library (Button, Toggle, Tile, and so on) |
| `modules/` | the bar, launcher, notifications, lock screen, and OSD |
| `services/` | QML singletons: the IPC client and wrappers around the Quickshell API |
| `theme/` | the generated `Tokens.qml`; do not edit it by hand |

The first content is the minimal bar of release 0.1.0. Every visual value comes from tokens, and `just qml-check` enforces it (`theme/` is exempt). Quickshell APIs are called only from `services/`. Splitting the shell into shared modules (`either.ui`, `either.theme`, `either.ipc`) waits for a decision record.

Conventions: [docs/development/coding-standards.md](../docs/development/coding-standards.md#qml-shell) and the [design system](../docs/development/design-system.md).
