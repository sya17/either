# ADR 0005: GitHub Actions is the temporary CI

- **Status:** accepted
- **Date:** 2026-10-19
- **Resolves:** Q4
- **Decided by:** project owner

## Context

"CI is green" is an exit criterion of 0.1.0, so the platform has to be chosen first. The repository is already on GitHub (`sya17/either`, public). The project requires every tool and the whole stack to be free and open source; GitHub Actions is free for public repositories but is not open-source software. The platform must also support private vulnerability reporting and, later, building packages and an ISO.

## Options

| Option | Advantages | Drawbacks | Reversible? |
| --- | --- | --- | --- |
| GitHub Actions, without a time limit | the fastest; private vulnerability reporting and branch protection available | not open source; vendor dependence | yes |
| GitHub Actions for now, then move to an open-source platform | fast now; the open-source requirement is kept through a deadline | the cost of a later migration | yes |
| Forgejo or Codeberg with Woodpecker | fully open source | moving the repository, running own runners (including archiso and QEMU), a smaller reach for contributors | yes |
| Postpone CI and rely on `just check` | no setup cost | no enforcement on the server side, while the Definition of Done mentions green CI | yes |

## Decision

GitHub Actions is used as a temporary CI, with a plan to move to an open-source platform before 1.0.0. Chosen by the project owner on 2026-10-19 from a list of options: "GitHub Actions for now, then move to open source". The open-source requirement is not changed: this is a conscious exception with a deadline.

## Consequences

- The CI job only calls `just check`; the list of checks is not duplicated in the workflow file. That keeps the cost of changing platform small.
- The workflow token is read-only and there are no secrets in pull request jobs; actions are pinned to a commit SHA; the Arch image is pinned by digest. What "pinned packages" means waits for the version-strategy decision.
- Not yet verified: whether GitHub's runners can run archiso and QEMU with KVM, which end-to-end tests and ISO builds need (0.6.0 to 0.7.0). If not, those tests need a runner of our own.
- Repository settings are made by the repository owner: branch protection (green CI and a pull request required for `main`), private vulnerability reporting, and the security policy.
- Before 1.0.0 a new record chooses an open-source platform and supersedes this one.
- Updates the justfile comment, the quality gates, the workflow, the contributing guide, and the decision list.
