#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Runs the optional local `check` hook (see scripts/lib/hooks.sh). Without one there is nothing to do.
set -uo pipefail

cd "$(git rev-parse --show-toplevel)" || exit 1
# shellcheck source=scripts/lib/hooks.sh
. scripts/lib/hooks.sh

if run_local_hook check; then
    echo "local-check: ok"
else
    echo "local-check: failed" >&2
    exit 1
fi
