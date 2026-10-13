# One entry point for every check. CI must call `just check`, so the gate is defined here once
# and does not depend on the CI platform (Q4).

set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Install the git hooks (once per clone)
setup:
    scripts/hooks/install.sh

# Every PR gate. Same list CI runs.
check: fmt-check lint test deny docs-check tokens-check qml-check clean-check

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

# Advisories and licenses (B10)
deny:
    cd core && cargo deny check

docs-check:
    python3 scripts/check-docs.py

# Token file structure and WCAG contrast (docs/development/design.md)
tokens-check:
    python3 scripts/check-tokens.py

# No literal colors, durations or sizes in shell QML
qml-check:
    python3 scripts/check-qml-literals.py

# Per-clone word list check; a no-op without .git/info/forbidden-words
clean-check:
    scripts/check-clean.sh
