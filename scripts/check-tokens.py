#!/usr/bin/env python3
"""Validate design/tokens/tokens.json (structure, references, contrast rules in docs/development/design.md)
and the SVG files in design/assets (only token colors, no scripts, no external references, no live text)."""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
THEMES = ("dusk", "dawn")
HEX = re.compile(r"^#[0-9A-Fa-f]{6}$")
REQUIRED = ("surface-crust", "surface-0", "surface-1", "surface-2", "highlight-med", "fg", "fg-muted",
            "fg-faint", "brand", "on-brand", "accent", "status-error", "status-warning", "status-success", "status-info")
# Text-like colors that must reach 4.5:1 on surface-0 (WCAG AA). accent, brand-secondary and border are exempt by design.
UNSAFE_SVG = re.compile(r"<script|<image|<foreignObject|<style|<text\b|href\s*=|\bon\w+\s*=|font-family", re.I)
TEXT = ("fg", "fg-muted", "fg-faint", "brand", "ansi-red", "ansi-green", "ansi-yellow", "ansi-blue",
        "ansi-magenta", "ansi-cyan", "ansi-orange")


def luminance(hex_color):
    channels = [int(hex_color[i:i + 2], 16) / 255 for i in (1, 3, 5)]
    r, g, b = [c / 12.92 if c <= 0.03928 else ((c + 0.055) / 1.055) ** 2.4 for c in channels]
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def contrast(a, b):
    hi, lo = sorted((luminance(a), luminance(b)), reverse=True)
    return (hi + 0.05) / (lo + 0.05)


def main() -> int:
    data = json.loads((ROOT / "design/tokens/tokens.json").read_text())
    tokens = {t["name"]: t["value"] for t in data["color"]["tokens"]}
    errors = []

    def resolve(name, theme, seen=()):
        value = tokens[name]
        value = value[theme] if isinstance(value, dict) else value
        ref = re.fullmatch(r"\{([\w-]+)\}", value)
        if ref:
            if ref[1] in seen or ref[1] not in tokens:
                raise ValueError(f"{name}: bad reference {value}")
            return resolve(ref[1], theme, seen + (name,))
        return value

    colors = {}
    for name, value in tokens.items():
        for theme in THEMES:
            try:
                color = resolve(name, theme)
            except ValueError as e:
                errors.append(str(e))
                continue
            if not HEX.match(color):
                errors.append(f"{name} ({theme}): '{color}' is not #RRGGBB")
            colors[(name, theme)] = color
    for name in REQUIRED:
        if name not in tokens:
            errors.append(f"missing required token {name}")
    if not errors:
        for theme in THEMES:
            base = colors[("surface-0", theme)]
            for name in TEXT:
                ratio = contrast(colors[(name, theme)], base)
                if ratio < 4.5:
                    errors.append(f"{name} on surface-0 ({theme}): {ratio:.2f}:1, needs 4.5:1")
            for fg, bg in (("fg", "highlight-med"), ("on-brand", "brand")):
                ratio = contrast(colors[(fg, theme)], colors[(bg, theme)])
                if ratio < 4.5:
                    errors.append(f"{fg} on {bg} ({theme}): {ratio:.2f}:1, needs 4.5:1")
    names = [t["name"] for group in ("spacing", "radius", "motion") for t in data[group]["tokens"]]
    if len(names) != len(set(names)) or len(tokens) != len(data["color"]["tokens"]):
        errors.append("duplicate token names")
    palette = {c.upper() for c in colors.values()}
    svgs = sorted((ROOT / "design/assets").rglob("*.svg"))
    for svg in svgs:
        raw = svg.read_text(encoding="utf-8")
        rel = svg.relative_to(ROOT)
        for color in sorted({c.upper() for c in re.findall(r"#[0-9a-fA-F]{6}\b", raw)} - palette):
            errors.append(f"{rel}: color {color} is not a token value")
        if UNSAFE_SVG.search(raw):
            errors.append(f"{rel}: contains script, image, style, live text, link or font reference")
    print("\n".join(errors) if errors else f"tokens: {len(tokens)} colors, {len(names)} other tokens, {len(svgs)} svg assets, contrast ok")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
