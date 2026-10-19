#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
# Point git at the shared hooks. Run once per clone: `just setup`.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"
chmod +x scripts/hooks/*
git config core.hooksPath scripts/hooks
echo "hooks: core.hooksPath = scripts/hooks"
