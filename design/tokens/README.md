# Design tokens

The single source of colors, typography, spacing, radii, and motion: [tokens.json](tokens.json). The values come from the either design system and were adopted through [ADR 0002](../../docs/adr/0002-design-system-as-token-source.md); the rules for using them are in [the design system](../../docs/development/design-system.md).

- Format: `color.tokens` (a name, a value for each theme `dusk` and `dawn`, and a usage note; `{name}` refers to another token), `type`, `spacing`, `radius`, `motion`.
- `either-theme` (0.2.0) generates `Tokens.qml` and the Hyprland, GTK, Qt, and terminal output from this file. No code reads it before that.
- `just tokens-check` verifies structure, references, and text contrast of 4.5:1. Run it before changing this file.
- Bar height, panel width, and motion beyond `duration-fast` and `duration-panel` are not in this file yet; they are set during the 0.2.0 calibration.
