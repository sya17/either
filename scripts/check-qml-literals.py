#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Fail on literal colors, durations, font sizes, radii, spacing, padding and margins in shell QML.

Everything visual comes from the tokens (docs/development/design-system.md). `shell/theme/` is exempt.
A line may opt out with a trailing `// lint-ok: <reason>`.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
NUM = r"-?[1-9]\d*(?:\.\d+)?"
RULES = (
    (re.compile(r"#[0-9a-fA-F]{3,8}\b"), "hex color"),
    (re.compile(r"\bQt\.(?:rgba?|hsla?|hsva?|darker|lighter|tint)\("), "color function"),
    (re.compile(r"\bcolor\s*:\s*\"(?!transparent\")"), "color name"),
    (re.compile(rf"\bduration\s*:\s*{NUM}\b"), "duration"),
    (re.compile(rf"\bfont\.(?:pixelSize|pointSize)\s*:\s*{NUM}\b"), "font size"),
    (re.compile(rf"\b(?:radius|spacing)\s*:\s*{NUM}\b"), "radius or spacing"),
    (re.compile(rf"\b(?:left|right|top|bottom)?[Pp]adding\s*:\s*{NUM}\b"), "padding"),
    (re.compile(rf"\b(?:anchors\.)?(?:left|right|top|bottom)?[Mm]argins?\s*:\s*{NUM}\b"), "margin"),
)


def main() -> int:
    bad = []
    for qml in sorted((ROOT / "shell").rglob("*.qml")):
        rel = qml.relative_to(ROOT)
        if rel.parts[:2] == ("shell", "theme"):
            continue
        for n, line in enumerate(qml.read_text().splitlines(), 1):
            if "lint-ok:" in line or line.lstrip().startswith("//"):
                continue
            code = line.split("//", 1)[0]
            for pattern, what in RULES:
                if pattern.search(code):
                    bad.append(f"{rel}:{n}: literal {what}; use a token")
    print("\n".join(bad) if bad else "qml: no literal visual values")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
