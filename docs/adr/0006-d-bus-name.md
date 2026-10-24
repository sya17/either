# ADR 0006: The D-Bus name

- **Status:** proposed
- **Date:** 2026-10-24
- **Resolves:** Q7
- **Decided by:** maintainer; the domain comes from the project owner

## Context

either needs one well-known D-Bus bus name: the name `eitherd` would own on the session bus once either offers a D-Bus interface of its own. The question is which name, and whether `eitherd` claims it in 0.1.0 or later. Names are hard to reverse, so the [decision list](README.md) puts this before 0.1.0.

The baseline is `io.github.<account>.Either1` until the project has a domain of its own. The repository is `sya17/either` on GitHub, so the baseline spells out as `io.github.sya17.Either1`. The project owner prefers a name based on a domain they control, but has no such domain yet. On 2026-10-24 the project owner wrote (translated from Indonesian): "okay, for now, before we have our own domain, use io.github.sya17.either1 first."

The main IPC of either does not use D-Bus. It is JSON-RPC over a Unix socket (see [architecture](../architecture.md)). `eitherd` uses D-Bus as a client of other services: NetworkManager, BlueZ, logind, and MPRIS players from 0.3.0. A client gets a unique connection name from the bus automatically and needs no well-known name. The notification server and the polkit agent use names and APIs that freedesktop defines, not a name of ours. A portal backend of our own would also use the portal naming scheme, not this name. So only an either D-Bus interface or D-Bus activation needs the name.

Evidence, read on 2026-10-24:

- [D-Bus Specification](https://dbus.freedesktop.org/doc/dbus-specification.html), version 0.43 (2024-10-29), section "Bus names":
  - A well-known bus name has at least two elements separated by `.`, and at most 255 characters.
  - Each element uses only `[A-Z][a-z][0-9]_-`, and must not begin with a digit.
  - `-` is discouraged in new names. New names should use `_` instead.
  - A well-known name should start with the reversed DNS domain of its author, in lower case. A hyphen in the domain becomes `_`, and a leading digit gets a `_` prefix.
  - The rest of the name is conventionally words run together, and interface names use initial capitals. This is a convention, not a syntax rule.
  - The spec says a version number in the bus name is a good idea, so several versions can be owned at the same time. Its example interface is `com.example.MusicPlayer1`.
  - The spec defines no case folding for names. `io.github.sya17.either1` and `io.github.sya17.Either1` are two different names.
- `io.github.sya17.either1` is valid: four elements, only lower-case letters and digits, no element starts with a digit, 23 characters.
- D-Bus has no central registry. A name is unique only because its author controls the domain or account in it. The GitHub account `sya17` belongs to the project owner and hosts the repository.
- [Flathub requirements](https://docs.flathub.org/docs/for-app-authors/requirements) (page without a version; copyright 2022-2026): IDs on code-hosting prefixes are `io.github.<user>.<repo>`, `page.codeberg.<user>.<repo>`, and similar. The domain must be under the author's control. Flathub reviews renames conservatively and refuses frequent ones. This is not a D-Bus rule. It shows how hard reverse-DNS identifiers are to change once published.
- [ADR 0005](0005-github-actions-as-temporary-ci.md) plans to move from GitHub to an open-source platform before 1.0.0.

## Options

| Option | Advantages | Drawbacks | Reversible? |
| --- | --- | --- | --- |
| `io.github.sya17.either1` (baseline prefix, the owner's interim choice) | available now, at no cost; the account is already controlled; valid D-Bus syntax; lower case matches the product name | ties the project's identity to a personal account and to GitHub, while ADR 0005 plans to leave GitHub before 1.0.0; departs from the capitalised convention for the last element | yes, until either ships a D-Bus interface; after that, only by owning the old and the new name in parallel for a deprecation period |
| `io.github.sya17.Either1` (baseline as written) | same as above; follows the capitalised convention | same ties as above; capital `E` in an identifier that carries the product name | same as above |
| reversed owner domain + `.either1` | matches the spec recommendation literally; independent of the forge, so it survives the move in ADR 0005 | not possible yet: the owner has no domain; needs a domain that the owner keeps renewing, and a lapsed domain can be registered by someone else | same as above |
| `io.github.<organisation>.either1` | not tied to one person; free | still tied to GitHub; the organisation name is part of the open name-availability check | same as above |
| `page.codeberg.<account>.either1` | fits a likely open-source forge | picks the platform before the record that replaces ADR 0005 does | same as above |

The last element carries the major version of either's D-Bus interface. That `1` is separate from the IPC `proto` and from the package version. Interface names and object paths usually reuse the bus name, with `/` in place of `.` in paths, so they inherit the same prefix.

## Decision

Recommendation (not yet decided): use `io.github.sya17.either1`, as the project owner wrote it on 2026-10-24.

- **Interim name.** The name holds until the project owner has a domain of their own. Then a new record supersedes this one and moves to the reversed domain with the same last element. The main reason is the planned move away from GitHub (ADR 0005): a name based on a domain does not depend on the forge.
- **Case.** The baseline writes `.Either1`; the owner wrote `either1`. Lower case keeps the product name in lower case, as it is everywhere else. Bus names are not case-folded, so only one spelling can be right. The project owner confirms the final spelling when this record is accepted.
- **When to claim.** `eitherd` does not claim a well-known name in 0.1.0. 0.1.0 has no D-Bus consumer and no D-Bus interface of either, so a claimed name would advertise nothing. The name is fixed now as a documented constant. `eitherd` claims it in the first release that exports an either D-Bus interface or needs D-Bus activation. Until then, `eitherd` uses D-Bus only as a client.

If the domain arrives before either ships a D-Bus interface, nothing outside this record and the constant has to change. The replacement is cheapest then.

## Consequences

- The bus name is defined once, as a constant in `eitherd`. The code that claims it belongs to the work on the first D-Bus interface, not to this record.
- `crates/either-dbus` does not need code in 0.1.0 because of this record. The row in `core/README.md` can say that the D-Bus code starts with MPRIS in 0.3.0.
- When the name is claimed, the systemd user unit of `eitherd` may use `BusName=`, and a D-Bus activation file may follow.
- Moving to a domain-based name after either ships a D-Bus interface means owning both names for a deprecation period, with a changelog entry.
- A domain-based name later makes renewing the domain a project duty. Losing the domain does not break the bus, but the name then stops proving who owns it.
- After the decision: update the D-Bus name row in the decision list in [docs/adr/README.md](README.md), and mark this record `accepted` or `rejected` with the decider's words and the date.
