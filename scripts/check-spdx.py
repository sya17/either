#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Fail when a code file lacks the SPDX license identifier in its first lines.

    check-spdx.py            every tracked .rs .qml .py .sh file and every tracked script with a shebang
    check-spdx.py FILE...    only those files (the pre-commit hook passes the staged ones)

Rule and reason: docs/development/coding-standards.md. The identifier only; copyright lines are not required.
"""
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
IDENTIFIER = "SPDX-License-Identifier: GPL-3.0-or-later"
SUFFIXES = {".rs", ".qml", ".py", ".sh"}
HEAD_LINES = 5


def tracked() -> list[str]:
    out = subprocess.run(["git", "ls-files", "-z"], cwd=ROOT, capture_output=True, check=True).stdout
    return [p for p in out.decode().split("\0") if p]


def is_code(path: Path, head: str) -> bool:
    return path.suffix in SUFFIXES or head.startswith("#!")


def main() -> int:
    names = sys.argv[1:] or tracked()
    missing = []
    for name in names:
        path = ROOT / name
        if not path.is_file() or path.is_symlink():
            continue
        try:
            head = "".join(path.open(encoding="utf-8").readlines()[:HEAD_LINES])
        except (UnicodeDecodeError, OSError):
            continue
        if is_code(path, head) and IDENTIFIER not in head:
            missing.append(name)
    print("\n".join(f"{n}: missing '{IDENTIFIER}' in the first {HEAD_LINES} lines" for n in missing)
          if missing else "spdx: every code file has the license identifier")
    return 1 if missing else 0


if __name__ == "__main__":
    sys.exit(main())
