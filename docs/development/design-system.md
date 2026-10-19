# Design system

The design rules for anyone who touches the interface: the shell, application themes, the installer, documents. The values live in [design/tokens/tokens.json](../../design/tokens/tokens.json) and were adopted through [ADR 0002](../adr/0002-design-system-as-token-source.md). What the design system does not define (surfaces, components, interaction, writing, accessibility) is described here as the project's baseline.

## Status of the values

| What | Status |
| --- | --- |
| Name, brand palette, the Orbit logo, the Manrope and JetBrains Mono fonts | final; only the project owner changes them |
| Token values in `tokens.json` | adopted in ADR 0002; calibrated in 0.2.0 without changing the role or the name of a token |
| Sizes of surfaces (bar height, panel width, row height) | provisional starting values, calibrated in 0.2.0 |
| Motion parameters other than `duration-fast` and `duration-panel` | set during the 0.2.0 calibration |

## Identity and voice

- The name is always **either**, in lowercase, including at the start of a sentence, in titles, and in the CLI. The themes are **either Dusk** (dark) and **either Dawn** (light); the theme word is capitalized. Modules are `either shell`, `either cli`, and so on.
- Dusk and Dawn share the same geometry, sizes, and motion. Only the colors differ.
- The character is calm, precise, and compact. Write like a careful engineer who also cares how things feel.
- Short, flat sentences: "Theme applied.", not "Your theme was applied successfully!". No exclamation marks, emoji, or hype words. Be technical only where it helps: "Hyprland config reloaded (12 ms)".
- Sentence case for every label and title. A button uses a verb that names the result ("Connect", "Clear history"); avoid "OK" for important decisions.
- An error message does not blame the user and names the next step; technical detail sits behind "Details". Numbers always carry a unit. One concept, one term.

## Logo and the Orbit symbol

An open frame (the shell) around a core, with a satellite in the opening of the frame. The geometry is the same in Dusk and Dawn.

- The construction follows the golden ratio φ on a box B = 36u; it is never redrawn by hand.
- Two optical masters: the display master (stroke B/φ⁶) for 32 px and up, and the small master (stroke B/φ⁵, satellite ×√φ) for 16 to 24 px.
- The horizontal lockup is the symbol followed by "either" in Manrope Light 300. The minimum width is 89 px; below that, use the symbol alone. The clear space H/φ on all sides is built into the SVG files.
- App icon: symbol = tile/φ, tile corner = tile/φ³ (23.6%).
- Below 32 px on Dawn, use the mono master: coral on `#F7F3F0` is only 1.8:1.
- Coral appears only in the core. Variants: Dusk, Dawn, mono light, mono dark, and no-accent (the core in `brand-secondary`, for contexts where coral would compete with status colors).
- **Do not**: change the spacing or the weight, write "Either" or "EITHER", add effects or gradients, move the satellite, add a second satellite, a badge, or a color per product.
- The symbol is not a UI icon, except for the launcher and menu button.
- The master files (SVG) are in `design/assets/` ([README](../../design/assets/README.md)). Do not create a new logo from this description; use the masters.
- **Avoid** in the identity: marks or colors of Arch, Tux, Hyprland, Qt, or Rust; terminal prompts, brackets, gears, monitors, shields; sparkles, brains, robots, mascots; a literal yin-yang or sun and moon; cyberpunk, gaming, or cloud styles; gradients and 3D.

## Color

All colors are read from tokens and never written as hex in a component. The complete list is in `tokens.json`; the chart below is generated from it by `scripts/gen-docs-assets.py`.

![The either palette in both themes](../assets/palette.svg)

| Role | Token | Dusk | Dawn |
| --- | --- | --- | --- |
| Deepest layer (bar, lock) | `surface-crust` | `#11121D` | `#EDE6E0` |
| Base (desktop, windows) | `surface-0` | `#171826` | `#F7F3F0` |
| Panels above the base | `surface-1` | `#1F2133` | `#FFFDFB` |
| Overlays (menus, tooltips, hovered rows) | `surface-2` | `#2A2C42` | `#EFE9E4` |
| Selection | `highlight-med` | `#424874` | `#DCD6F7` |
| Window border, cursor | `highlight-high` | `#4A4E6B` | `#C9C3E6` |
| Primary text | `fg` | `#E7E9F6` | `#2A2B3F` |
| Secondary text | `fg-muted` | `#A6B1E1` | `#5E6288` |
| Comments, placeholders, disabled | `fg-faint` | `#7F85AD` | `#6A6E8C` |
| Symbol, focus, active workspace, primary button | `brand` | `#8DA2FF` | `#424874` |
| Text on a `brand` fill | `on-brand` | `#171826` | `#F7F3F0` |
| Small accent | `accent` | `#FF9E7D` | `#FF9E7D` |

Status colors are `status-error`, `status-warning`, `status-success`, and `status-info` (they refer to `ansi-red`, `ansi-yellow`, `ansi-green`, and `ansi-cyan`). The terminal uses `ansi-*`; the bright variants 8 to 15 reuse the same hue, and only black and white step. `ansi-orange` is an extra color for editors.

Rules:

- **Periwinkle leads**; Soft Indigo (`fg-muted`, `brand-secondary`) supports. `brand-secondary` is only for the second layer of the two-tone logo: it is 1.9:1 on Dawn and must not be text or a lone icon.
- **Coral is one small dot at most**: an unread marker, a recording indicator. It is not an error color, not in the logo except the core, and not text on Dawn (1.8:1).
- Depth comes from tone, not shadows: `surface-0` → `surface-1` → `surface-2`. A hairline `border` is used only where tone is not enough, and never as the only cue.
- Text contrast is at least 4.5:1 on `surface-0` in its theme; important UI elements and icons are at least 3:1. A pair that fails must not become a token; `just tokens-check` verifies this.
- Color is never the only cue: status always comes with an icon or text.

## Typography

Manrope for the shell UI and the wordmark (Light 300 for display, 400 to 600 for UI), JetBrains Mono for the terminal and CLI. The scale is √φ from 14 px; line height is φ (1.618) for text and √φ (1.272) for headings. Every size is multiplied by `font.scale`.

| Token | Size / weight | Used for |
| --- | --- | --- |
| `display` | 37 / 300 | lock screen and greeter clock, welcome screens |
| `headline` | 29 / 400 | settings page titles |
| `title` | 23 / 500 | panel titles |
| `subtitle` | 18 / 500 | notification and dialog titles |
| `body` | 14 / 400 | default UI text |
| `body-strong` | 14 / 600 | row titles, bar modules |
| `caption` | 11 / 500 | timestamps, hints, labels; use `fg-muted` |
| `mono` | 13 / 400 | CLI output, config values, terminal |

## Spacing, radius, and icons

| Token | Value | Used for |
| --- | --- | --- |
| `space-1` | 3 px | hairline offsets, icon-to-label gap |
| `space-2` | 5 px | gap between bar modules |
| `space-3` | 8 px | row padding; Hyprland `gaps_in` |
| `space-4` | 13 px | card and panel padding; Hyprland `gaps_out` |
| `space-5` | 21 px | gap between groups in settings and the control center |
| `space-6` | 34 px | page margins, section spacing |
| `radius-sm` | 5 px | chips, inputs, workspace pills |
| `radius-md` | 8 px | buttons, list rows, Hyprland window rounding |
| `radius-lg` | 13 px | panels: launcher, notification cards, control center |
| `radius-icon` | 23.6% | app icon corner (1/φ³) |

- An element inside a panel uses a smaller radius than the panel.
- Layouts use a 12-column grid with Fibonacci margins and gutters (for example 55 and 21 px) and row heights from 89, 144, and 233 px.
- UI icons: 1.5 px stroke, rounded joins, a 24 px grid, in `fg` or `fg-muted`. The icon set is not chosen yet (ADR 0002). The minimum click target is 32 × 32 px.
- Hyprland windows: `gaps_in` = `space-3`, `gaps_out` = `space-4`, `rounding` = `radius-md`, active border `brand` 2 px, inactive border `border`. The configuration is written in Lua ([ADR 0004](../adr/0004-hyprland-config-in-lua.md)).

## Motion

- `duration-fast` 144 ms for hover, press, and small color changes. `duration-panel` 233 ms, ease-out, for panels, popovers, the launcher, and workspace moves. `duration-instant` 0 ms for reduced motion. Switching between Dusk and Dawn is one crossfade of the whole desktop.
- Motion has an origin and a destination: a panel appears from the element that opens it and closes back into it. While one panel moves, the rest of the screen is still.
- Animate only opacity, scale, and position through transforms, not width, height, or anchors. Total stagger never exceeds one `duration-panel`, and an animation can be cancelled or reversed midway.
- No loose duration in a component; everything goes through tokens. Blur is done by the compositor (layer rules for the `either-*` namespaces), not in QML. A heavy panel uses `Loader { asynchronous: true }`; `layer.enabled` and shaders only after measuring.
- **Reduced motion**: movement and scale become a `duration-fast` crossfade, springs are off, stagger is removed. Information is never conveyed by animation alone.

## Surfaces

The bar is the center of navigation. Every panel drops from the element in the bar that opens it, only one panel is open at a time, and the island in the middle of the bar is the only element that grows into another surface.

| Surface | Width | Position |
| --- | --- | --- |
| Bar | full width, 36 px high | floating, `space-4` from the edge (the same as `gaps_out`); left zone (logo, workspaces, window title), center (island), right zone (tray, status, clock) |
| either menu | 280 px | drops from the logo, aligned to the left edge |
| Launcher | 560 px | centered, grows from the island; at most 60% of the screen height |
| Control center | 360 px | drops from the status icons, aligned to the right edge |
| Notification center | 380 px | drops from the clock, aligned to the right edge |
| Toast | 360 px | top right below the bar; at most 3 stacked |
| Island | content width, up to 420 px | center of the bar; 36 px growing to at most 120 px |
| Dialog (polkit, power) | 400 px | centered, with the background dimmed |
| Settings | a normal window with a sidebar | managed by Hyprland |

These widths and heights are provisional starting values and are calibrated in 0.2.0. Panel padding is `space-4`; content taller than the maximum scrolls inside the panel. Every monitor has its own bar. Panels and the launcher open on the focused monitor, and the full island appears only on the primary monitor. All sizes are logical pixels; fractional scaling is tested at 1.0, 1.25, 1.5, and 2.0, and at a logical width of 1366 px or less `density = compact` turns on automatically.

Tiling keeps Hyprland's model: dwindle by default, master and scrolling per workspace, smart gaps (one window means no gap and no border), and dialogs and small windows float automatically.

## Components and interaction

- Every surface is built from the library in `shell/components/`. A panel does not make its own button, toggle, or slider. Library v1: `Button` (primary, secondary, ghost, icon; one primary per view), `Toggle` (applies immediately, no Save), `Tile`, `Slider`, `ListItem`, `Card`, `Popover` and `Menu`, `SearchField`, `Toast`, `Dialog`, `Island`.
- Every interactive component has six states: rest, hover, pressed, focus (a 2 px `brand` ring during keyboard navigation), disabled, and selected (a `highlight-med` background).
- Keyboard and mouse are equal: Escape and a click outside always close a panel. Detail stays in place (an arrow on a tile opens a list inside the panel). Undo is better than a confirmation.
- Feedback: small changes inline, results of actions through a `Toast`, application events through a notification, risky decisions through a `Dialog`. The last state is remembered, except for the launcher, which is always empty.
- Empty state: a 32 px icon, one sentence, one action if there is one. Loading: under 150 ms show nothing, a longer list uses a skeleton, an action without progress uses a small spinner in the button. Error: near its source, what happened and how to fix it, and a "Try again" button. Unavailable: a disabled control with a short reason on hover or focus.
- Sound uses the freedesktop sound theme, only for important events, and is silent when do-not-disturb is on.

## Accessibility

- Contrast as in the Color section. The focus ring is always visible during keyboard navigation; targets are at least 32 × 32 px with at least `space-1` between them.
- The layout holds up to `font.scale` 150%; long text is truncated with an ellipsis and a tooltip.
- Every interactive component gives its name and role through `Accessible`. Requiring `Accessible.role` and `Accessible.name` with a lint is still a proposal, so it is recommended but not yet linted.
- Full keyboard navigation, high contrast, and reduced motion arrive in 0.8.0. Full screen-reader support is not promised for 1.0.

## Keeping the standard

1. **Tokens are the single source.** Colors, sizes, spacing, radii, and durations come only from `Tokens.qml`, generated from `tokens.json` by `either-theme` (0.2.0). Do not edit generated files, and do not edit `tokens.json` to suit one component.
2. **Automatic checks** (part of `just check`):
   - `just tokens-check`: token structure, references, WCAG contrast, and the contents of the SVG files in `design/assets/`;
   - `just qml-check`: QML outside `shell/theme/` may not contain a literal hex value, color name, duration, font size, radius, spacing, padding, or margin. One line may opt out with `// lint-ok: reason`.
3. **Component gallery** (`either gallery`, planned): every component in its six states, in Dusk and Dawn, including reduced motion; its screenshots become the baseline of the visual tests.
4. **UI review checklist** for every pull request that touches the shell:
   - [ ] Only tokens and components from the library are used
   - [ ] It looks right in both either Dusk and either Dawn
   - [ ] It can be fully operated with the keyboard, and Escape closes it
   - [ ] Animation has a clear origin and uses the motion tokens
   - [ ] It works with reduced motion and a large `font.scale`
   - [ ] Coral does not appear outside the accent dot, and text meets the contrast rules

## Outside the shell

Consistency in GTK, Qt, the terminal, cursor, fonts, wallpaper, icons, the lock screen, the greeter, and Plymouth is handled by the theme pipeline (`either theme build`). The minimum rule: if an application cannot look exactly the same, its background color, accent color, and dark or light mode must match. How each target is handled (recolor only, or replace the theme) waits for a decision record on third-party theming. Themes for Firefox and VS Code are outside 1.0.
