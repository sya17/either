#!/usr/bin/env bash
# Fails when tracked content, file names, commit messages, branches or tags match the per-clone
# word list (.git/info/forbidden-words). No list on this clone means nothing to check.
set -uo pipefail

cd "$(git rev-parse --show-toplevel)" || exit 1
list="$(git rev-parse --git-path info/forbidden-words)"
[[ -s "$list" ]] || { echo "check-clean: no word list on this clone, skipped"; exit 0; }

fail=0
git grep -InEi --cached -f "$list" && fail=1
git ls-files | grep -Ei -f "$list" && fail=1
git log --all --format='%H %s%n%b' 2>/dev/null | grep -Ei -f "$list" && fail=1
git for-each-ref --format='%(refname)' | grep -Ei -f "$list" && fail=1

((fail)) && echo "check-clean: matches above; fix them before pushing" >&2 || echo "check-clean: ok"
exit "$fail"
