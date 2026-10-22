# ADR 0002: The either design system is the source of visual tokens

- **Status:** accepted
- **Date:** 2026-10-13
- **Resolves:** I17
- **Decided by:** project owner

## Context

Two sources disagreed on the visual token values (identifier I17): the spacing grid, radii, motion durations, type scale, icon size and style, and the meaning of the word `accent`. Only the Orbit logo and the Manrope font were final; the other values waited for a decision.

The project owner supplied the either design system as the reference and asked for the project foundation to be completed from it on 2026-10-13: "complete all the basics for this project from this design, set up and write the rules, design, and other standards" (translated). That instruction is read as choosing the design system side of I17.

## Options

| Option | Advantages | Drawbacks | Reversible? |
| --- | --- | --- | --- |
| The earlier baseline scale (a 4 px grid, radii 6/10/14, motion 100 to 400 ms, Lucide icons, `accent` as periwinkle) | written down in the early design documents; round numbers | does not match the identity that was designed: the logo, palette, and tokens follow another logic | yes, before 0.2.0 generates tokens |
| The design system scale (Fibonacci spacing, radii 5/8/13, motion 144/233 ms, 1.5 px line icons, `brand` and `accent`) | one logic with the Orbit logo (golden ratio) and the palette; token values and contrast are already computed | some earlier sizes (bar height, panel widths, row heights) do not land on the scale and need calibration | yes, before 0.2.0 |

## Decision

Token values follow the either design system and live in `design/tokens/tokens.json` as the single source:

- colors: the design system's roles and values, with `brand` as periwinkle (Dusk) or deep indigo (Dawn) and `accent` as coral;
- typography: Manrope and JetBrains Mono, with a √φ scale of 11, 14, 18, 23, 29, and 37 px;
- Fibonacci spacing of 3, 5, 8, 13, 21, and 34 px; radii of 5, 8, and 13 px, and 23.6% for app icons;
- motion `duration-fast` 144 ms and `duration-panel` 233 ms; UI icons as 1.5 px lines on a 24 px grid;
- token names in kebab-case as in the design system (`surface-1`, `fg-muted`), replacing the dotted names of the earlier baseline (`surface.raised`).

Still in force from the earlier baseline: the five design principles, the surfaces and the panel behaviour, the component library and its six states, the interaction patterns, the writing rules, the accessibility rules, and the rules for keeping the standard (see [the design system](../development/design-system.md)).

Calibration of the values in Quickshell and Hyprland remains open for 0.2.0. It may change numbers through a pull request as long as the roles and names of tokens stay the same; changing the character of the brand remains a decision of the project owner.

## Consequences

- The earlier baseline is replaced for the values named above; its other parts are unchanged.
- `design/tokens/tokens.json` becomes the input of `either-theme`. `just tokens-check` verifies structure and contrast (4.5:1 for text).
- Sizes that the design system does not define (bar height 36 px, panel widths, row and tile heights) stay provisional values and move into the 0.2.0 calibration.
- Motion beyond `duration-fast` and `duration-panel` (the theme crossfade, spring parameters) is set during calibration; no loose numbers in components.
- The source of the UI icon set is not chosen. Lucide, from the earlier baseline, is a candidate if used with a 1.5 px stroke.
- Updates [the design system](../development/design-system.md) and [the coding standards](../development/coding-standards.md).
