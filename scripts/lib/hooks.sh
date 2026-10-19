#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Sourced by the git hooks, scripts/open-pr.sh and scripts/local-check.sh; do not run it.
#
# Optional local hooks. A maintainer or an organisation can add policy checks of their own without
# changing this repository. For a hook called NAME the first executable file found is run with the
# repository as working directory and the hook's own arguments:
#   1. <git dir>/hooks.d/NAME
#   2. ${EITHER_HOOKS_DIR:-~/.config/either-dev/hooks.d}/NAME   ($XDG_CONFIG_HOME aware)
# A non-zero exit blocks the operation. `git config either.requireLocalHooks true` turns a missing
# hook into an error instead of a no-op.

run_local_hook() {
    local name="$1" dir candidate
    shift
    for dir in "$(git rev-parse --git-path hooks.d)" \
        "${EITHER_HOOKS_DIR:-${XDG_CONFIG_HOME:-$HOME/.config}/either-dev/hooks.d}"; do
        candidate="$dir/$name"
        if [[ -x "$candidate" ]]; then
            "$candidate" "$@"
            return
        fi
    done
    if [[ "$(git config --get --type=bool either.requireLocalHooks 2>/dev/null)" == true ]]; then
        echo "no local '$name' hook found, but either.requireLocalHooks is set (see docs/development/setup.md)" >&2
        return 1
    fi
    return 0
}
