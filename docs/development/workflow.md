# Workflow and lifecycle

One flow for every contributor. Releases are decided by the exit criteria in the [roadmap](../roadmap.md), not by dates.

## Lifecycle

| # | Stage | What happens | Output | Exit gate |
| --- | --- | --- | --- | --- |
| 1 | Plan | The release scope comes from the roadmap. Open questions are answered as decision records. | release scope, decision records | exit criteria are clear |
| 2 | Prepare | Needs are split into small work items with checkable outcomes. | work items | Definition of Ready |
| 3 | Implement | A short-lived branch from `main`, small commits. | code, tests, docs | `just check` is green locally |
| 4 | Review | A pull request to `main`. | approval | pull request checklist |
| 5 | Integrate | Squash merge, CI on `main`. | green `main` | CI is green |
| 6 | Release candidate | End-to-end, visual and performance tests; dogfooding. | candidate | candidate gate |
| 7 | Release | Tag, changelog, packages and ISO. | release | release checklist |
| 8 | Maintain | Patches, deprecations, upstream version bumps, security fixes. | patch release | the same gates |

Stages 6 and 7 start to matter at 0.6.0, when distribution begins. Before that, a release is a tag on `main` once its exit criteria are met.

## Working on an item

1. Pick an item from the backlog of the current release, highest priority first. Do not start an item that fails the Definition of Ready.
2. Mark it in progress. At most two items are in progress at the same time.
3. Create a branch `<type>/<topic>` from the latest `main`. One item, one branch. If several people or tools share one working tree, use `git worktree` instead of switching branches.
4. Work in small commits. Write the tests together with the code. Update the documentation your change affects.
5. `just check` must be green before you push. State in the pull request what you tested and what you could not test.
6. Open a pull request and move the item to review. The title uses the Conventional Commits format, because it becomes the commit message on `main`.
7. After the merge, close the item with evidence (the commit and what was tested). New findings become separate draft items, not additions to the same pull request.

## Work items

An item has a release, a type (the Conventional Commit it will produce), a priority, a size, dependencies, the decisions it needs, and "done when" and "out of scope" sections.

- **Priority.** P0 is on the critical path of the release (a blocker, or a dependency of other items). P1 is in the release scope. P2 may move to the next release if the release is under pressure.
- **Size.** S is under half a day, M is one to two days, L must be split first.

| Stage | Meaning |
| --- | --- |
| `draft` | a raw idea or need; not yet a defined item |
| `backlog` | fully defined; waiting for its turn, a dependency, or a decision |
| `progress` | being worked on; at most two at a time |
| `review` | finished; waiting for the pull request to merge or for verification |
| `hold` | paused; the reason is recorded |
| `completed` | done, with evidence (the commit and what was tested) |
| `canceled` | will not be done; the reason is recorded |

The usual path is `draft` → `backlog` → `progress` → `review` → `completed`. `hold` and `canceled` are possible from any stage before `completed`. A finished item goes to `review` and becomes `completed` only after the merge and when its evidence is recorded.

New ideas start as `draft`, never in the current release. They join a release only when the maintainers agree, and a new component enters 1.0 only if another one leaves. A scope change is recorded with one line of reasoning; silently growing an item is not allowed.

**Definition of Ready.** An item may start when all of these are true:

1. "Done when" lists checkable outcomes, not "implement X".
2. There is an "out of scope" section.
3. Every dependency is complete.
4. Every decision it needs is settled by a decision record (accepted or rejected).
5. Its size is S or M.
6. It does not touch a final decision, does not implement an open proposal, does not decide an open question, and is inside the 1.0 inventory.

**Definition of Done.**

- Every "done when" statement is true and was verified, not assumed.
- The code is merged to `main`; `just check` and CI are green.
- Tests are added or updated; IPC contract changes carry the `ipc-change` label.
- Affected documentation is updated; a new architecture decision has a decision record.
- A keybinding or menu entry exists where relevant, and the change works in both either Dusk and either Dawn if it touches the UI.
- Evidence is recorded: the commit, what was tested, and what was not. A failed or unmeasured criterion does not become a success by changing a status.

## Decisions

Architecture decisions (open questions, proposals, inconsistencies) are analysed and decided when the work that depends on them is about to start, and recorded as decision records. The procedure, the deadlines, and the list of open decisions are in [docs/adr/README.md](../adr/README.md). Work that depends on an open decision does not start; until the decision exists, only work that follows the baseline proceeds.

## Git

- **Trunk-based.** `main` is always releasable. Branches are short-lived; there are no long-lived branches other than `release/x.y` for fixing old releases.
- **Linear history.** Rebase onto `main` before the pull request and merge with squash. There are no merge commits.
- **Conventional Commits.** `type(scope): description`, imperative, at most 72 characters, no trailing period.
  - Types: `feat` (minor), `fix` and `perf` (patch), `refactor`, `docs`, `style`, `test`, `build`, `ci`, `chore`, `revert`.
  - Scope: the code area without the `either-` prefix: `ipc`, `eitherd`, `hypr`, `cli`, `shell`, `theme`, `config`, `keybinds`, `packaging`, `adr`.
  - `!` or a `BREAKING CHANGE:` footer marks a breaking change. During `0.x` it raises the minor version and must appear in the changelog with a config migration.
  - Allowed footers: `Refs`, `Fixes`, `Closes`, `BREAKING CHANGE`. No other footer or attribution text appears in commit messages, pull request titles, labels, comments, branch names, or tags. Author and committer are the people accountable for the change. A local hook (`just setup`) rejects badly formatted messages.
- The changelog is produced by `git-cliff` from the commit messages on `main`, so write the first line for the person who updates.
- Tags and releases are created by maintainers only.

## Pull requests

Title: a Conventional Commit. Body: what changed and why, the related roadmap item or decision record, and what was tested and what was not (see `.github/pull_request_template.md`). Open it with `just pr "<title>" <body-file>`, which checks the title format and the local hooks, refuses a dirty working tree, and refuses to open a pull request from `main`.

**Scope.** One pull request serves one item of the current release roadmap and names it in its body. Changes outside that item (drive-by refactors, new components, tidying) are not included and become separate items. A new component needs a decision record and an inventory change; `just scope-check` rejects crates and top-level directories outside the inventory.

Checklist for the author and the reviewer:

- [ ] The change serves a roadmap item and stays within its "out of scope" limits
- [ ] No final decision is touched, no open proposal is implemented, no open question is decided
- [ ] `just check` is green; new tests exist for new logic and for every fixed bug
- [ ] IPC contract change: the `ipc-change` label is set and the schema snapshot is updated
- [ ] No literal visual values (QML), no secrets, external input is validated
- [ ] Affected documentation is updated; a new architecture decision has a record
- [ ] It looks right in both either Dusk and either Dawn (when it touches the UI)

## Review

- Every change goes through a pull request, and CI must be green before the merge. Merging with a red CI is not allowed.
- While there is a single maintainer, the author reviews their own diff against the checklist. With two or more maintainers, every pull request needs one approval from someone else.
- Branch protection is a repository setting managed by the repository owner (see [ADR 0005](../adr/0005-github-actions-as-temporary-ci.md)).
- **A red `main`.** A fix or a revert lands before any other change is merged. Nothing is merged on top of red. A flaky test is reported as a bug and fixed; automatic retries are not used to hide it.

## Releases

| Topic | Rule |
| --- | --- |
| Versioning | `either-*` packages: `MAJOR.MINOR.PATCH`; IPC contract: integer `proto`; config: integer `schema`; ISO: calendar version such as `2026.11.0` (proposed, not yet decided) |
| Before 1.0 | Minor releases may break if recorded in the changelog with a config migration; patch releases only fix bugs. The IPC envelope is locked from 0.1.0; `proto = 1` and `schema = 1` freeze at 0.9.0. Feature freeze at 0.9.0, then `1.0.0-rc.N` contains only fixes |
| Tag | from `main` (or `release/x.y` for patches), by a maintainer |
| Candidate gate | all pull request tests, end-to-end (QEMU), visual and performance tests pass; visual regressions are approved by hand |
| Stable gate | the candidate is used daily with no open blocking bug |
| Signing | waits for the repository key decision; until then no artifact is published |
| Release checklist | `docs/release.md`, written before the first distribution release (0.6.0) |

## Maintenance

- Fixes for an old release are made on `release/x.y` and cherry-picked from `main`.
- A Hyprland or Quickshell version bump is always its own pull request and must pass the end-to-end tests. A Hyprland `.0` release is never shipped; the project follows the last patch of the previous minor. The rule is applied as cautious practice and will be formalised in the version-strategy decision record.
- Read the diff of every upstream bump, not only the changelog.
- Critical security fixes take precedence over scheduled releases.
- A feature or config key that will be removed is announced as deprecated at least one minor release earlier.
