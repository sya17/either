# Documentation

| Document | What it covers |
| --- | --- |
| [Architecture](architecture.md) | Layers, processes, configuration, the IPC contract, and the security model |
| [Roadmap](roadmap.md) | Releases 0.1.0 to 1.0.0, their scope and exit criteria |
| [Development setup](development/setup.md) | Prerequisites, building, running, and debugging |
| [Workflow](development/workflow.md) | Lifecycle of a change, git conventions, pull requests, releases |
| [Coding standards](development/coding-standards.md) | Naming, Rust, tests, async code, filesystem, QML, IPC, security |
| [Quality gates](development/quality.md) | The checks that every change must pass, and what each one proves |
| [Design system](development/design-system.md) | Identity, tokens, typography, motion, surfaces, components |
| [Decision records](adr/README.md) | Why the project is built the way it is, and which decisions are still open |

## Images

The README banners, the palette, and the social preview in `docs/assets/` are generated from the design tokens and the master logo files, not drawn by hand. Regenerate them with:

```bash
python3 scripts/gen-docs-assets.py
rsvg-convert -w 1280 docs/assets/social-preview.svg -o docs/assets/social-preview.png
```
