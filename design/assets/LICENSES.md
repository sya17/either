# Licenses of the design assets

This file describes the license of every file in `design/assets/`. The license of the either code is a separate decision ([ADR 0003](../../docs/adr/0003-code-license-gpl-3-or-later.md)) and does **not** apply to the assets in this folder.

## Logos, icons, and sub-brands

| Folder | Contents | License |
| --- | --- | --- |
| `logos/` | 5 either lockups (dusk, dawn, mono-light, mono-dark, two-tone-dusk) | CC BY-SA 4.0 |
| `icons/` | 9 Orbit icons (app and launcher, display, small optical master) | CC BY-SA 4.0 |
| `sub-brands/` | lockups for either shell, either cli, and one reserved module (dusk and dawn) | CC BY-SA 4.0 |

- **SPDX:** `CC-BY-SA-4.0`
- **Copyright:** © 2026 The either Project Authors
- **Full license text:** `LICENSES/CC-BY-SA-4.0.txt` at the repository root. Summary: <https://creativecommons.org/licenses/by-sa/4.0/>
- **Original source:** the either design system, version 3 (the Orbit symbol, golden-ratio construction). The files here are the original export and are not redrawn.
- The images in `docs/assets/` are generated from these assets and from the design tokens, and carry the same license.

Anyone may use, copy, modify, and distribute these files, provided they give attribution and share derivatives under the same license. CC BY-SA 4.0 can be combined one way into GPLv3.

## The name and logo as a brand

CC BY-SA 4.0 only governs the copyright of the files, **not** rights in a trademark. The name "either" and the Orbit logo identify the project. Until a formal trademark policy is published (planned before 1.0), these interim rules apply:

- The logo may be used unchanged to refer to the either project, for example in an article, a review, a screenshot, or a link.
- A modified logo, and any derivative distribution or fork, **must not** present itself as the official either and must not use the name "either" as its product name.
- For any other use, contact the project owner.

## Fonts inside the assets

The wordmark in `logos/` and `sub-brands/` uses **Manrope** (Light 300), already converted to outlines. The font files are not distributed in this folder.

- **Manrope:** SIL Open Font License 1.1, © 2018 The Manrope Project Authors (<https://github.com/googlefonts/manrope>). There is no Reserved Font Name.
- According to the OFL FAQ (1.1 and 1.13), artwork made with an OFL font, including a logo made from its outlines, is not itself covered by the OFL. The attribution above is given as courtesy, not as an obligation.
- If the repository ever **bundles the Manrope font files** (`.ttf`, `.otf`, `.woff2`), `OFL.txt` must be included with them.

Audit status: Manrope was audited on 2026-10-13. The other components in the project's license table are not audited yet.
