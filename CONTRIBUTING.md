# Contributing to either

Thank you for your interest. either is built toward 1.0.0 with a **fixed scope**: each release has a defined set of features and exit criteria (see the [roadmap](docs/roadmap.md)). Bug fixes and work that belongs to the current release are welcome. For a new feature, open an issue first, because a new component enters 1.0 only if another one leaves.

## Before you start

1. Read the [README](README.md) and the [architecture](docs/architecture.md).
2. Set up your machine: [docs/development/setup.md](docs/development/setup.md), then run `just setup`.
3. Pick a ready item from the backlog of the current release. Items that wait on an open decision are not worked on; the open decisions are listed in the [decision records](docs/adr/README.md). The backlog is not published yet, so ask in an issue if you are unsure.

## Workflow

1. Create a short-lived branch from the latest `main`: `<type>/<topic>`, for example `feat/ipc-envelope`.
2. Work in small commits, write tests with the code, and update the documentation your change affects.
3. `just check` must pass before you push.
4. Open a pull request with a Conventional Commit title. Describe what changed and why, which roadmap item it serves, what you tested and what you could not test. `just pr "<title>" <body-file>` checks the title and body for you.
5. Pull requests are squash-merged, so the title becomes the commit message on `main`.

The full flow, the commit format, the pull request checklist, and the review rules are in [docs/development/workflow.md](docs/development/workflow.md). Code conventions are in [docs/development/coding-standards.md](docs/development/coding-standards.md), and changes to the UI also follow the [design system](docs/development/design-system.md).

## Good to know

- **Commits.** `type(scope): description`, in English, imperative, at most 72 characters. A local hook rejects other formats.
- **Language.** Code, comments, commit messages, and documentation are in English. `just check` verifies that tracked text files are English.
- **Decisions.** Architecture decisions are written as records in `docs/adr/`, not in code comments or commit messages.
- **Licensing.** Code is GPL-3.0-or-later, and every new source file starts with an SPDX identifier (`just spdx-check`). Brand assets are CC BY-SA 4.0. A contributor sign-off policy has not been decided.
- **Gates are not negotiable.** Do not disable, loosen, or bypass a check to make a change pass. Changing a gate is its own pull request with a stated reason.
- **Security.** Report vulnerabilities privately; see [SECURITY.md](SECURITY.md).
