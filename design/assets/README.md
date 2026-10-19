# Brand assets

The logo, the app icons, and the sub-brand lockups. The rules for using them are in [the design system](../../docs/development/design-system.md#logo-and-the-orbit-symbol).

The files are the master export of the either design system (2026-10-13). Do not redraw a logo or edit these files by hand; export them again from the source. `just tokens-check` verifies that every color in an SVG is a token value and that no file contains a script, an image, a style, live text, or a link.

| Folder | Files |
| --- | --- |
| `logos/` | `either-lockup-dusk.svg`, `either-lockup-dawn.svg`, `either-lockup-mono-light.svg`, `either-lockup-mono-dark.svg`, `either-lockup-two-tone-dusk.svg` |
| `icons/` | `either-icon-dusk.svg`, `either-icon-dawn.svg`, `either-icon-transparent.svg`, `either-icon-two-tone.svg`, `either-icon-mono-light.svg`, `either-icon-mono-dark.svg`, `either-icon-small-dusk.svg`, `either-icon-small-mono-light.svg`, `either-icon-small-mono-dark.svg` |
| `sub-brands/` | `either-{shell,companion,cli}-{dusk,dawn}.svg` |

Use the horizontal lockup as the main logo, the app icon for the launcher and packages, the small master (`icon-small-*`) at 16 to 24 px, and the mono master below 32 px on Dawn. The wordmark is already outlined Manrope Light, so no font is needed to render it.

Licenses and sources: [LICENSES.md](LICENSES.md).
