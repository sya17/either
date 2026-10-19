#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-3.0-or-later
"""Generate the README images in docs/assets from the design tokens and the master logo files.

    gen-docs-assets.py

Writes banner-dark.svg, banner-light.svg, social-preview.svg and palette.svg. Nothing is drawn by hand:
the banners place the master lockups from design/assets/logos on a token background, and the palette is
read from design/tokens/tokens.json. Render the social preview to PNG with
`rsvg-convert -w 1280 docs/assets/social-preview.svg -o docs/assets/social-preview.png`.
"""
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "docs" / "assets"
TOKENS = json.loads((ROOT / "design/tokens/tokens.json").read_text())
COLORS = {t["name"]: t["value"] for t in TOKENS["color"]["tokens"]}


def color(name: str, theme: str) -> str:
    value = COLORS[name]
    value = value[theme] if isinstance(value, dict) else value
    ref = re.fullmatch(r"\{([\w-]+)\}", value)
    return color(ref[1], theme) if ref else value


def lockup(theme_file: str):
    """(width, height, inner markup without the background rect) of a master lockup."""
    raw = (ROOT / "design/assets/logos" / theme_file).read_text()
    m = re.match(r'<svg[^>]*viewBox="0 0 ([\d.]+) ([\d.]+)"[^>]*>\s*<rect[^>]*></rect>(.*)</svg>\s*$', raw, re.S)
    assert m, f"unexpected lockup structure in {theme_file}"
    return float(m[1]), float(m[2]), m[3]


def banner(width: int, height: int, theme: str, lockup_file: str, lockup_width: float) -> str:
    w, h, inner = lockup(lockup_file)
    scale = lockup_width / w
    x, y = (width - w * scale) / 2, (height - h * scale) / 2
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">\n'
        f'<rect width="{width}" height="{height}" fill="{color("surface-0", theme)}"/>\n'
        f'<g transform="translate({x:.2f} {y:.2f}) scale({scale:.4f})">{inner}</g>\n</svg>\n'
    )


SWATCHES = ["surface-crust", "surface-0", "surface-1", "surface-2", "highlight-med", "fg-faint", "fg-muted", "fg",
            "brand", "accent", "ansi-red", "ansi-green", "ansi-yellow", "ansi-blue", "ansi-magenta", "ansi-cyan"]


def palette() -> str:
    pitch, sw_w, sw_h, pad = 124, 76, 44, 28
    panel_w = pad * 2 + pitch * 8 - (pitch - sw_w)
    panel_h = 36 + pad + 2 * (sw_h + 44) + 4
    gap = 16
    height = panel_h * 2 + gap
    parts = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {panel_w} {height}" width="{panel_w}" height="{height}" '
             f'font-family="ui-monospace, \'JetBrains Mono\', monospace">']
    for i, (theme, title) in enumerate((("dusk", "either Dusk"), ("dawn", "either Dawn"))):
        top = i * (panel_h + gap)
        bg, fg, muted, faint, line = (color(n, theme) for n in ("surface-0", "fg", "fg-muted", "fg-faint", "border"))
        parts.append(f'<rect x="0" y="{top}" width="{panel_w}" height="{panel_h}" rx="13" fill="{bg}"/>')
        parts.append(f'<text x="{pad}" y="{top + 34}" font-family="Manrope, system-ui, sans-serif" font-size="18" '
                     f'font-weight="500" fill="{fg}">{title}</text>')
        for n, name in enumerate(SWATCHES):
            col, row = n % 8, n // 8
            x = pad + col * pitch
            y = top + 52 + row * (sw_h + 48)
            parts.append(f'<rect x="{x}" y="{y}" width="{sw_w}" height="{sw_h}" rx="8" fill="{color(name, theme)}" '
                         f'stroke="{line}" stroke-width="1"/>')
            parts.append(f'<text x="{x}" y="{y + sw_h + 16}" font-size="11" fill="{muted}">{name}</text>')
            parts.append(f'<text x="{x}" y="{y + sw_h + 30}" font-size="10" fill="{faint}">{color(name, theme)}</text>')
    parts.append("</svg>\n")
    return "\n".join(parts)


def main() -> int:
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "banner-dark.svg").write_text(banner(1440, 400, "dusk", "either-lockup-dusk.svg", 560))
    (OUT / "banner-light.svg").write_text(banner(1440, 400, "dawn", "either-lockup-dawn.svg", 560))
    (OUT / "social-preview.svg").write_text(banner(1280, 640, "dusk", "either-lockup-dusk.svg", 640))
    (OUT / "palette.svg").write_text(palette())
    print("wrote", ", ".join(sorted(p.name for p in OUT.glob("*.svg"))))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
