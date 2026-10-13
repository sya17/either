# assets

Berkas identitas: logo, ikon aplikasi, dan sub-brand. Aturan pemakaiannya ada di [docs/development/design.md](../../docs/development/design.md#logo-dan-simbol-orbit).

Berkas master diekspor dari design system either (2026-10-13). Jangan menggambar ulang logo atau mengubah berkas ini dengan tangan; ekspor ulang dari sumbernya. `just tokens-check` memeriksa bahwa setiap warna di SVG adalah nilai token dan tidak ada skrip, gambar, gaya, teks hidup, atau tautan.

Berkas (20):

| Folder | Berkas |
| --- | --- |
| `logos/` | `either-lockup-dusk.svg`, `either-lockup-dawn.svg`, `either-lockup-mono-light.svg`, `either-lockup-mono-dark.svg`, `either-lockup-two-tone-dusk.svg` |
| `icons/` | `either-icon-dusk.svg`, `either-icon-dawn.svg`, `either-icon-transparent.svg`, `either-icon-two-tone.svg`, `either-icon-mono-light.svg`, `either-icon-mono-dark.svg`, `either-icon-small-dusk.svg`, `either-icon-small-mono-light.svg`, `either-icon-small-mono-dark.svg` |
| `sub-brands/` | `either-{shell,companion,cli}-{dusk,dawn}.svg` |

Pemakaian: lockup horizontal sebagai logo utama, ikon aplikasi untuk launcher dan paket, master kecil (`icon-small-*`) untuk 16–24 px, master mono di bawah 32 px pada Dawn. Wordmark sudah berupa outline Manrope Light, jadi tidak membutuhkan font saat dirender.

Setiap aset (font, ikon, suara, wallpaper) wajib berlisensi bebas. Lisensi dan sumber aset di folder ini dicatat di [../LICENSES.md](../LICENSES.md).
