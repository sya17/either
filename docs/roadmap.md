# Roadmap

either is released in ten steps, from 0.1.0 to 1.0.0. Releases are driven by **exit criteria, not dates**: a release is done when its criteria are met. Time estimates are made only after the velocity of 0.1.0 is measured.

- 0.1.0 to 0.5.0 build a complete desktop for the maintainers' own daily use.
- 0.6.0 to 0.7.0 make it installable by other people.
- 0.8.0 to 1.0.0 make it mature.

## Rules for 0.x

- Minor releases may contain breaking changes, provided they are listed in the changelog and ship with a config migration. Patch releases only fix bugs.
- The IPC envelope (NDJSON, JSON-RPC, `hello`, the event format) is locked from 0.1.0. Methods and topics may be added. The contract is declared stable as `proto = 1` in 0.9.0.
- Feature freeze starts at 0.9.0. After that only `1.0.0-rc.N` releases containing fixes are published.
- New components enter 1.0 only if something else leaves it.

## Releases

| Release | Scope | Exit criteria |
| --- | --- | --- |
| **0.1.0** Walking skeleton | monorepo, CI, `just`; IPC envelope (`hello`, `subscribe`, `state.snapshot`, `Event`); `eitherd` and the Hyprland socket bridge; minimal bar over IPC; `either ipc ping`; basic keybinding generator; local PKGBUILD and systemd units; baseline decision records | installs with `makepkg -si`; the bar shows the active workspace through IPC; restarting `eitherd` does not crash the shell; CI is green |
| **0.2.0** Theme and config | design tokens and `either-theme` (QML, Hyprland, GTK, Qt, kitty); Dusk and Dawn with a keybinding to switch; motion tokens; validated `config.toml` with hot reload; wallpaper | switching theme changes the shell, Hyprland, GTK, Qt and the terminal together; a broken config never breaks the desktop; in daily use |
| **0.3.0** Core shell | launcher; notification server, popups, notification center, do-not-disturb; OSD; media (MPRIS); all keybindings and a cheat sheet | everyday desktop work without a terminal; `either keybinds check` passes |
| **0.4.0** System integration | control center (NetworkManager, BlueZ, PipeWire, UPower); clipboard and emoji; capture; power and either menus; polkit agent; portal configuration; default applications | Wi-Fi, Bluetooth, audio and browser screen sharing work from the UI; polkit dialogs match the design |
| **0.5.0** Session | lock screen and idle; greeter; power profiles; suspend and hibernate | login, lock, suspend, wake and unlock without returning to a TTY |
| **0.6.0** Distribution | package repositories (`[either]`, `[either-testing]`); the `either-desktop` meta-package; `either setup`; `either update` with an indicator; snapshots; Flatpak; a first diagnostics report | a clean Arch install plus `either setup` yields the same desktop as the development machine; rollback from a snapshot is tested |
| **0.7.0** Installer | archiso and archinstall profiles; Btrfs subvolumes; bootloader with boot-to-snapshot; optional LUKS2; Plymouth; end-to-end QEMU tests in CI | the ISO installs in a VM without intervention and passes the smoke test; booting a snapshot works |
| **0.8.0** Polish | hardware matrix and per-monitor config; the settings application; first-run wizard; keyboard navigation, font scaling, high contrast, reduced motion; English and Indonesian UI; performance baseline | tested on at least one AMD and one Intel machine; the shell is usable without a mouse; memory and frame baselines are recorded |
| **0.9.0** Beta | feature freeze; `proto = 1` and `schema = 1` frozen; performance and visual regressions fail CI; documentation site; then `1.0.0-rc.N` | no blocking bugs; old clients still pass the compatibility tests; docs cover installation, keybindings, config and IPC |
| **1.0.0** Stable | the last release candidate, with no code changes | every quality gate passes; no blocking bugs since the last candidate; ISO, packages, checksums and signatures are published |

## After 1.0

Later releases add a workspace overview with live previews, a dock, desktop widgets, and a public plugin API. These are not part of 1.0.
