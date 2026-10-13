# tokens

Sumber tunggal warna, tipografi, spasi, radius, dan motion: [tokens.json](tokens.json). Nilainya berasal dari design system either dan diadopsi lewat [ADR 0002](../../docs/adr/0002-design-system-sebagai-sumber-token.md); aturan pemakaiannya di [docs/development/design.md](../../docs/development/design.md).

- Format: `color.tokens` (nama, nilai per tema `dusk` dan `dawn`, kegunaan; `{nama}` merujuk token lain), `type`, `spacing`, `radius`, `motion`.
- `either-theme` (0.2.0) membangkitkan `Tokens.qml` dan keluaran Hyprland, GTK, Qt, dan terminal dari berkas ini. Tidak ada kode yang membaca berkas ini sebelum itu.
- `just tokens-check` memeriksa struktur, referensi, dan kontras teks 4,5:1. Jalankan sebelum mengubah berkas ini.
- Nilai tinggi bar, lebar panel, dan motion selain `duration-fast` dan `duration-panel` belum ada di sini; ditetapkan saat kalibrasi 0.2.0.
