# Architecture decision records

Every architecture decision is written down as one file named `NNNN-title-in-kebab-case.md`, created from [0000-template.md](0000-template.md). Numbers increase in the order records are created; 0001 is reserved for the threat model.

## Rules

- A record states which question or proposal it **resolves**. The identifiers (Q5, U1, I17, and so on) come from the project's design dossier; every record restates the question in its own context, so no record depends on reading the dossier.
- Status: `proposed` → `accepted` | `rejected` → `superseded`. An accepted record is not edited; a new record that supersedes it changes the decision.
- The project owner decides whatever touches the final decisions, the 1.0 scope, or the license. A maintainer decides the rest. Write the technical reasons; do not copy the research, link it.
- A record becomes `accepted` or `rejected` only when the person who decides approves it. Until then its status is `proposed`.
- Work that depends on an open decision does not start until a record that resolves it is accepted or rejected. Until then, only work that follows the baseline proceeds; do not implement an open proposal, and do not write "temporary" code that quietly picks a side.

## Deciding when needed

Decisions are not analysed up front. They are analysed and decided when the first work that depends on them is about to start, or earlier when the project owner asks.

1. **Recognise.** Work that names an open decision cannot start.
2. **Analyse.** Create a `proposed` record from the template. Collect evidence: the related material, upstream documentation (note the version), and a spike or a measurement when needed. Write a table of options with advantages, drawbacks, and whether each can be reversed.
3. **Recommend.** Write the recommendation and its technical reasons in the Decision section, clearly marked as a proposal.
4. **Decide.** The decider answers. The record becomes `accepted` or `rejected`, with the decider's words and the date.
5. **Apply.** Update the table below; work that was waiting can now start.

A decision that is hard to reverse (disk layout and bootloader, the repository key, the IPC contract, names) is not postponed past its deadline. A release is not closed while a decision that gates it is open, unless the project owner moves it in writing.

## Decisions and their deadlines

| # | Topic | Reference | First needed by | Deadline | Status |
| --- | --- | --- | --- | --- | --- |
| 1 | Hyprland config format | U1, I1, A4 | keybinding generator, theme, and Hyprland stub | blocks 0.1.0 | [ADR 0004](0004-hyprland-config-in-lua.md), accepted |
| 2 | Version strategy for Hyprland, Quickshell, and Qt | I8, U16, U17, A7 | local PKGBUILD, CI image | blocks 0.1.0 | open |
| 3 | CI platform and hosting | Q4 | the CI pipeline | blocks 0.1.0 ("CI is green" is an exit criterion) | [ADR 0005](0005-github-actions-as-temporary-ci.md), accepted; move to an open-source platform before 1.0.0 |
| 4 | D-Bus name | Q7 | `eitherd`, if the record requires claiming a name | blocks 0.1.0 | open |
| 5 | Quickshell API spike | A1 | the minimal bar | blocks 0.1.0 | spike |
| 6 | Nested Hyprland for development | A6 | running the dev shell | blocks 0.1.0 | verification |
| 7 | Threat model (record 0001) | U22 | the first security feature | 0.1.0 | open |
| 8 | Baseline decisions | | | 0.1.0 | not yet written |
| 9 | Bootloader, disk layout, encryption | Q3, U3, U23, I10, I11, I18, I19 | the installer | proposed for 0.1.0; hard deadline: before the disk layout is locked (installer gate, 0.6.0 or 0.7.0) | open |
| 10 | Plugin model | I13, U26 | plugin code or a widget socket | proposed for 0.1.0 or 1.x; no plugin API until a record exists; scope belongs to the project owner | open |
| 11 | Settings as a separate instance and shared QML modules | U18 | splitting `shell/` into shared modules | proposed for 0.1.0; hard deadline: before the settings skeleton | open |
| 12 | Third-party theming ("colors only") | U19 | GTK and Qt targets in `either-theme` (0.2.0) | proposed for 0.1.0 to 0.2.0 | open |
| 13 | Quickshell config location | Q14, U21 | systemd units and PKGBUILD | 0.1.0 | open |
| 14 | NVIDIA support scope | U2, I16 | GPU detection in the installer | proposed for 0.1.0; hard deadline: before 0.7.0 | open |
| 15 | Repository key and the `either-keyring` package | U27 | the signed repository (0.6.0) | proposed for 0.1.0; hard deadline: key-compromise plan before 0.6.0 | open |
| 16 | Name availability | U30, Q20 | choosing the organisation and domain | 0.1.0; trademark policy before 1.0 | check |
| 17 | Validation of A9 and A10 | A9, A10 | the config generator and the weekly CI job | 0.1.0 | verification |
| 18 | Project license | Q5 | making the repository public | before the repository is public | [ADR 0003](0003-code-license-gpl-3-or-later.md), accepted |
| 19 | Source of the visual token values | I17 | tokens and theme | 0.2.0 | [ADR 0002](0002-design-system-as-token-source.md), accepted |
