#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Fail when the tree grows a component outside the 1.0 inventory (B4, B15).

Checks tracked files only: top-level entries and Cargo packages must be on the allowlists below.
Adding to a list is a change to the gate: its own PR, with the ADR that admits the component.
"""
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TOP_DIRS = {".github", "core", "shell", "shell-native", "design", "dotfiles", "scripts", "packaging", "iso",
            "installer", "tests", "docs", "LICENSES"}
TOP_FILES = {"CHANGELOG.md", ".editorconfig", ".gitattributes", ".gitignore", "CODE_OF_CONDUCT.md", "CODEOWNERS", "CONTRIBUTING.md",
             "LICENSE", "README.md", "SECURITY.md", "justfile"}
PACKAGES = {"either-ipc", "either-hypr", "either-dbus", "either-config", "either-theme", "eitherd", "either-cli"}
BINARIES = {"either", "eitherd"}


def main() -> int:
    out = subprocess.run(["git", "ls-files", "-z"], cwd=ROOT, capture_output=True, check=True).stdout
    files = [p for p in out.decode().split("\0") if p]
    errors = []
    for top in sorted({f.split("/", 1)[0] for f in files}):
        is_dir = any(f.startswith(top + "/") for f in files)
        if top not in (TOP_DIRS if is_dir else TOP_FILES):
            errors.append(f"top-level {'directory' if is_dir else 'file'} '{top}' is not in the inventory")
    for f in files:
        if f.endswith("Cargo.toml"):
            data = tomllib.loads((ROOT / f).read_text())
            name = data.get("package", {}).get("name")
            if name and name not in PACKAGES:
                errors.append(f"{f}: package '{name}' is not in the inventory")
            for binary in data.get("bin", []):
                if binary.get("name") not in BINARIES:
                    errors.append(f"{f}: binary '{binary.get('name')}' is not in the inventory")
    print("\n".join(errors) if errors else "scope: tree matches the inventory")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
