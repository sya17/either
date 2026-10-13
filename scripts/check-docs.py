#!/usr/bin/env python3
"""Fail when a relative link in a Markdown file points to a file that does not exist.

Anchors (#...) and external links are not checked. Code blocks and inline code are skipped.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SKIP = {"target", "node_modules"}
KEEP_HIDDEN = {".github"}
LINK = re.compile(r"\]\(([^)\s]+)")
FENCE = re.compile(r"^(```|~~~).*?^\1", re.S | re.M)
INLINE = re.compile(r"`[^`\n]*`")


def main() -> int:
    bad = []
    for md in sorted(ROOT.rglob("*.md")):
        parts = md.relative_to(ROOT).parts[:-1]
        if SKIP & set(parts) or any(p.startswith(".") and p not in KEEP_HIDDEN for p in parts):
            continue
        text = INLINE.sub("", FENCE.sub("", md.read_text()))
        for target in LINK.findall(text):
            if re.match(r"[a-z][a-z0-9+.-]*:", target) or target.startswith("#"):
                continue
            path = target.split("#", 1)[0]
            if "researchs/" in path:  # local-only folder, absent in other clones
                continue
            if path and not (md.parent / path).exists():
                bad.append(f"{md.relative_to(ROOT)}: broken link -> {target}")
    print("\n".join(bad) if bad else "docs: all relative links resolve")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
