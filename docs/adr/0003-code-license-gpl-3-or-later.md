# ADR 0003: The code is licensed GPL-3.0-or-later

- **Status:** accepted
- **Date:** 2026-10-19
- **Resolves:** Q5
- **Decided by:** project owner

## Context

The license of the project is the project owner's decision. The baseline was GPL-3.0-or-later, with MIT or Apache-2.0 as alternatives. The repository was public on GitHub without a `LICENSE` file, so the code was visible but legally not usable by others. The design assets already have their own license (CC BY-SA 4.0, see `design/assets/LICENSES.md`).

## Options

| Option | Advantages | Drawbacks | Reversible? |
| --- | --- | --- | --- |
| GPL-3.0-or-later | copyleft: derivatives stay open, in line with the goal of a fully free stack; fits the main dependencies (Hyprland BSD-3-Clause, Qt and Quickshell LGPL) and CC BY-SA 4.0 assets, which can be combined one way into GPLv3 | closed forks are not possible | hard once outside contributors exist: it needs the agreement of every copyright holder |
| Apache-2.0 | permissive, with a patent grant | forks may be closed; using the CC BY-SA assets inside needs separate handling | same |
| MIT | the simplest | no patent grant and no duty to share back | same |
| Keep the repository private for now | a decision without pressure | delays contributions and visibility | yes |

## Decision

GPL-3.0-or-later. Chosen by the project owner on 2026-10-19 from a list of options: "GPL-3.0-or-later".

## Consequences

- A `LICENSE` file (the GPL-3.0 text from gnu.org) is added at the repository root.
- `license = "GPL-3.0-or-later"` in `core/Cargo.toml` is final; pacman packages use `license=('GPL-3.0-or-later')`.
- Assets in `design/assets/` stay CC BY-SA 4.0 and are not covered by this decision.
- SPDX headers are required in code files; a contributor sign-off (DCO) is not required yet.
- The audit of dependency licenses is not finished; `cargo-deny` checks the dependencies that enter, and its allowlist already includes GPL and LGPL.
- Updates `README.md`, `CONTRIBUTING.md`, and the decision list in `docs/adr/README.md`.
