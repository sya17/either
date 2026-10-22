# One entry point for every check. CI runs `just check`, so the gate is defined once here
# and does not depend on the CI platform (see docs/adr/0005-github-actions-as-temporary-ci.md).

set shell := ["bash", "-euo", "pipefail", "-c"]

# List the recipes
default:
    @just --list

# Install the git hooks (once per clone)
setup:
    scripts/hooks/install.sh

# Every pull request gate; the same list CI runs
check: fmt-check lint test deny docs-check language-check tokens-check qml-check spdx-check scope-check local-check

# Format the Rust code
fmt:
    cd core && cargo fmt --all

fmt-check:
    cd core && cargo fmt --all --check

lint: lint-rust lint-shell lint-qml

lint-rust:
    cd core && cargo clippy --workspace --all-targets --locked -- -D warnings

lint-shell:
    find scripts -type f -exec grep -lE '^#!.*(ba)?sh' {} + | xargs -r shellcheck

lint-qml:
    find shell -name '*.qml' | xargs -r qmllint

test:
    cd core && cargo test --workspace --locked

# Advisories and licenses
deny:
    cd core && cargo deny check

# Relative links in Markdown resolve
docs-check:
    python3 scripts/check-docs.py

# Tracked text files are written in English
language-check:
    python3 scripts/check-language.py

# Token file structure and WCAG contrast (docs/development/design-system.md)
tokens-check:
    python3 scripts/check-tokens.py

# No literal colors, durations or sizes in shell QML
qml-check:
    python3 scripts/check-qml-literals.py

# Every code file starts with the license identifier (docs/development/coding-standards.md)
spdx-check:
    python3 scripts/check-spdx.py

# The tree only holds components from the 1.0 inventory (docs/development/coding-standards.md)
scope-check:
    python3 scripts/check-scope.py

# Optional per-machine policy checks (scripts/lib/hooks.sh); a no-op without them
local-check:
    scripts/local-check.sh

# Open a pull request; the title and body get the same checks as a commit message
pr title body:
    scripts/open-pr.sh "{{title}}" "{{body}}"
