# Quality gates

The gates are defined once, in the `justfile` (`just check`). CI only calls them, so their content does not depend on the CI platform (see [ADR 0005](../adr/0005-github-actions-as-temporary-ci.md)).

## Gates

| Gate | Contents | When |
| --- | --- | --- |
| **Pull request, merge to `main`** | `just check`: format, lint, tests, `cargo-deny`, documentation links, language, tokens, QML literals, SPDX headers, scope, and the optional local checks. IPC contract changes carry the `ipc-change` label. The commit message or pull request title is a Conventional Commit. | every pull request |
| **Release candidate** | end-to-end tests (QEMU), visual tests (Dusk and Dawn), performance tests. Visual regressions are approved by hand. | every candidate, from 0.6.0 to 0.9.0 |
| **Stable release** | the candidate is used daily with no open blocking bug | every release |

## Checks

| Check | Tool | Command | Status |
| --- | --- | --- | --- |
| Rust formatting | rustfmt | `just fmt-check` | active |
| Rust lints | clippy with `-D warnings`, workspace lints in `core/Cargo.toml` | `just lint-rust` | active |
| Rust tests | `cargo test --locked` | `just test` | active; there are no tests yet |
| Licenses and advisories | `cargo-deny`, allowlist in `core/deny.toml` | `just deny` | active |
| Shell scripts | `shellcheck` | `just lint-shell` | active |
| QML lint | `qmllint` | `just lint-qml` | active; there is no QML yet |
| Documentation links | `scripts/check-docs.py` | `just docs-check` | active |
| Language | `scripts/check-language.py`: tracked text files are English | `just language-check` | active |
| Design tokens | `scripts/check-tokens.py`: structure, references, text contrast of 4.5:1, and that SVG assets only use token colors with no scripts or links | `just tokens-check` | active |
| QML literals | `scripts/check-qml-literals.py`: no literal color, duration, size, radius, or spacing | `just qml-check` | active; checked against examples, no QML yet |
| SPDX headers | `scripts/check-spdx.py` | `just spdx-check` | active |
| Scope | `scripts/check-scope.py`: only crates, binaries, and top-level directories from the 1.0 inventory | `just scope-check` | active |
| Commit message format | the `commit-msg` hook | `just setup` | active |
| Secrets | the `pre-commit` hook rejects a private key; `.gitignore` blocks `*.pem *.key .env* *.kdbx` | `just setup` | active, as a safety net and not a replacement for care |
| Full gate before a push | the `pre-push` hook runs `just check` | `just setup` | active |
| Pull request title and body | `scripts/open-pr.sh` checks the title format and runs the local `pr-text` hook | `just pr` | active; protects only pull requests opened through it |
| Optional local checks | hooks in `<git dir>/hooks.d/` or `~/.config/either-dev/hooks.d/` | `just local-check` | optional; see [setup](setup.md#7-local-hook-extensions) |
| Accessibility lint for QML | not available yet | | waits for a decision record |
| IPC contract (`insta` snapshots, old client against new daemon) | `insta` | `just test` | planned, with the IPC envelope |
| `either keybinds check` | the CLI | | planned, with the keybinding generator |
| Generated Hyprland config against the latest Hyprland | a headless session, weekly | | planned; depends on a check that Hyprland can validate a config without a session |
| Daily advisory audit | scheduled `cargo-audit` | | planned, with CI |
| Dependency review | `cargo-vet` | | proposed |
| End-to-end, visual, performance, suspend | QEMU, `grim`, `QSG_RENDER_TIMING` | | 0.6.0 to 0.8.0 |

## Who runs what

| Environment | May run | Examples |
| --- | --- | --- |
| A development machine, without touching a desktop session (anyone, including automation) | static checks, unit tests, integration tests with fake sockets and a private bus, `cargo-deny`, `systemd-analyze verify` on units, package builds in a clean chroot | `just check`, `cargo test`, `namcap` (once a PKGBUILD exists) |
| A maintainer's desktop session or a VM | QML appearance, the bar, nested Hyprland, `makepkg -si`, the lock screen, suspend | per-release verification, dogfooding |
| CI ([ADR 0005](../adr/0005-github-actions-as-temporary-ci.md)) | everything in the first row; end-to-end QEMU tests and ISO builds where the runner supports them | pull request gate, release candidates |

Results from the second row are recorded by the maintainer in the pull request. Results from the first row are reported with the command and its exit code; a command that could not be run is reported as untested.

## Dependencies

- `Cargo.lock` is committed; builds and tests use `--locked`.
- The Rust toolchain is pinned in `core/rust-toolchain.toml`. Bumping it is its own pull request together with the `Cargo.lock` changes.
- Dependency licenses must be on the allowlist in `core/deny.toml` (MIT, Apache-2.0, BSD-2/3, ISC, Zlib, MPL-2.0, LGPL, GPL). Adding a license to the list changes a gate: a pull request with a reason.
- A new dependency has a license that passes, is maintained, and cannot be replaced by a few lines of code. The reason is written in the pull request.
- Assets (fonts, icons, sounds, wallpapers) must be freely licensed, and their sources are recorded in [design/assets/LICENSES.md](../../design/assets/LICENSES.md).
- A Hyprland or Quickshell version bump: see [workflow](workflow.md#maintenance).
- The CI image is pinned by digest. The meaning of "pinned packages" depends on the version-strategy decision.

## Security

The principles and the rules for code are in [coding-standards.md](coding-standards.md#security-in-code). At repository level:

| Topic | Rule | Status |
| --- | --- | --- |
| Threat model | a decision record with STRIDE per trust boundary | planned |
| Secrets in the repository | not allowed; hooks and `.gitignore` are a safety net | active |
| Repository signing key | offline, subkeys on a hardware key, never signing in public CI | proposed |
| Signed commits and tags, contributor sign-off | proposed | not required yet |
| Vulnerability reporting | [SECURITY.md](../../SECURITY.md) and GitHub private vulnerability reporting | the policy exists; the repository owner enables the channel |
| Permission rules for Hyprland, lock screen, clipboard, system hardening | planned with the release that needs them | decision records first |
| Plugins | no plugin API in 1.0 | decision record first |

Report security findings privately, not in a public issue.

## Performance budget

- No dropped frame when a panel or the launcher opens, at the monitor's refresh rate (60, 120, or 144 Hz).
- No polling; every update is driven by an event.
- Memory and start-time baselines are measured in 0.8.0. From 0.9.0, a regression above the baseline fails CI.
