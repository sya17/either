#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Fail when a tracked text file is not written in English.

Everything in the repository is English (docs/development/coding-standards.md). The check is a heuristic:
it counts common Indonesian function words and flags a file with three or more. It cannot judge quality,
only catch a file that was left in the wrong language.
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SKIP_NAMES = {"LICENSE", "Cargo.lock"}
SKIP_SUFFIXES = {".svg", ".json", ".png", ".jpg", ".webp", ".woff2", ".ttf", ".otf"}
WORDS = re.compile(
    r"\b(yang|dan|untuk|dengan|tidak|adalah|dari|pada|bisa|akan|atau|sudah|belum|jangan|sebagai|dalam|harus|setelah|sebelum)\b",
    re.I,
)
LIMIT = 3


def main() -> int:
    out = subprocess.run(["git", "ls-files", "-z"], cwd=ROOT, capture_output=True, check=True).stdout
    bad = []
    for name in (p for p in out.decode().split("\0") if p):
        path = ROOT / name
        if name.startswith("LICENSES/") or path.name in SKIP_NAMES or path.suffix in SKIP_SUFFIXES:
            continue
        if path == Path(__file__).resolve():  # holds the word list itself
            continue
        if not path.is_file() or path.is_symlink():
            continue
        try:
            hits = WORDS.findall(path.read_text(encoding="utf-8"))
        except (UnicodeDecodeError, OSError):
            continue
        if len(hits) >= LIMIT:
            bad.append(f"{name}: {len(hits)} Indonesian words (e.g. '{hits[0].lower()}'); write it in English")
    print("\n".join(bad) if bad else "language: tracked text files are in English")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
