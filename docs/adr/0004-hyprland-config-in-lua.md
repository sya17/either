# ADR 0004: The Hyprland configuration is written in Lua from 0.1.0

- **Status:** accepted
- **Date:** 2026-10-19
- **Resolves:** U1, I1, A4
- **Decided by:** project owner

## Context

The earlier baseline wrote a stub that uses `source =` in the hyprlang format, on the assumption that hyprlang stays supported (A4). That assumption no longer holds: according to the project research, hyprlang is deprecated since Hyprland 0.55 and will be removed, and Lua becomes the main format. The stub, the keybinding generator, and the Hyprland theme files are built in 0.1.0, so the format has to be settled before that.

## Options

| Option | Advantages | Drawbacks | Reversible? |
| --- | --- | --- | --- |
| Lua from 0.1.0 | no rewrite of the generator later; follows the direction of Hyprland | a Lua config is code: values must be escaped, and how `require()` resolves modules is not yet verified | yes, through a superseding record; the generator output is thin |
| Wait until the module resolution is verified | a decision based on facts | slows the keybinding generator | yes |
| hyprlang first, migrate later | matches the earlier baseline | builds on a format that will be removed, then rewrites | yes, at the cost of a rewrite |

## Decision

The Hyprland configuration is written in Lua from 0.1.0. Chosen by the project owner on 2026-10-19 from a list of options: "Lua from 0.1.0".

The form that applies:

- The user stub `~/.config/hypr/hyprland.lua` uses `require()` in order: the distribution defaults (`/usr/share/either/hypr/`), the generated output (`~/.local/state/either/hypr/`), and then the user's own file, last.
- `either-hypr` and `either-theme` generate Lua (keybindings, monitors, window rules, permissions). The Lua API names are verified against the installed Hyprland; examples in the research are only a guide.
- Values from `config.toml` are escaped as Lua strings. The user file loads last. `either doctor` warns when a system Lua file was changed.
- hypridle and hyprlock keep using hyprlang.

## Consequences

- A4 stays void and I1 is settled. A9 (a way to verify a config without a session) and A10 (uwsm reading environment files from the distribution directory) still have to be verified. If the result changes how modules are resolved, the stub is revised through a superseding record; the format is not reopened.
- The rule "do not write a Hyprland generator before a decision exists" in the coding standards is lifted. The keybinding generator and the Hyprland theme files may be built, subject to the Definition of Ready of their work items.
- The hyprlang stub example in the research becomes an archive.
- Updates the coding standards, the quality gates, the design system, and the decision list.
