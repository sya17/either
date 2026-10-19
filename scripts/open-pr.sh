#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Open a pull request. Usage: just pr "<title>" <body-file>
# The title and body get the same scrutiny as a commit message: Conventional Commit title, local `pr-text`
# hook (scripts/lib/hooks.sh), clean working tree, never from main.
set -euo pipefail

die() { echo "pr: $*" >&2; exit 1; }
(($# == 2)) || die "usage: just pr \"<type(scope): title>\" <body-file>"
title="$1"
body="$2"
[[ -f "$body" ]] || die "body file not found: $body"

cd "$(git rev-parse --show-toplevel)"
# shellcheck source=scripts/lib/hooks.sh
. scripts/lib/hooks.sh

branch="$(git branch --show-current)"
[[ -n "$branch" && "$branch" != main ]] || die "refusing to open a pull request from '${branch:-detached HEAD}'"

re='^(feat|fix|perf|refactor|docs|style|test|build|ci|chore|revert)(\([a-z0-9._/-]+\))?!?: .+$'
if [[ ! "$title" =~ $re ]] || ((${#title} > 72)); then
    die "title must be 'type(scope): description', at most 72 characters"
fi

run_local_hook pr-text "$title" "$body" || die "rejected by the local pr-text hook"

if ! git diff --quiet || ! git diff --cached --quiet; then
    die "uncommitted changes; commit them first"
fi
command -v gh >/dev/null 2>&1 || die "'gh' is not installed"

git push -u origin "$branch"
gh pr create --base main --head "$branch" --title "$title" --body-file "$body"
