# ADR 0002: Design system either sebagai sumber token visual

- **Status:** diterima
- **Tanggal:** 2026-10-13
- **Menyelesaikan:** I17
- **Diputuskan oleh:** pemilik proyek

## Konteks

Nilai token visual di baseline B12 berbeda dari design system either (inkonsistensi I17): grid spasi, radius, durasi motion, skala tipe, ukuran dan gaya ikon, dan arti kata `accent`. Hanya logo Orbit (F8) dan font Manrope (F9) yang sudah final; nilai lainnya menunggu keputusan.

Pemilik proyek menyerahkan design system either sebagai acuan dan meminta fondasi project dilengkapi darinya, 2026-10-13: "lengkapi semua kebutuhan dasar untuk project ini dari design ... setup dan tulis aturan, rules, design, dan standar lainnya". Instruksi itu dibaca sebagai keputusan memilih sisi design system pada I17.

## Pilihan

| Pilihan | Kelebihan | Kekurangan | Bisa dibalik? |
| --- | --- | --- | --- |
| Skala baseline B12 (grid 4 px, radius 6/10/14, motion 100–400 ms, ikon Lucide, `accent` = Periwinkle) | sudah tertulis di dokumen desain awal; angka bulat | tidak sama dengan identitas yang sudah dirancang: logo, palet, dan token memakai logika berbeda | ya, sebelum 0.2.0 membangkitkan token |
| Skala design system (Fibonacci, radius 5/8/13, motion 144/233 ms, ikon garis 1.5 px, `brand` dan `accent`) | satu logika dengan logo Orbit (rasio emas) dan palet; token dan kontras sudah dihitung | beberapa ukuran B12 (tinggi bar, panel, baris) tidak jatuh di skala ini dan perlu dikalibrasi | ya, sebelum 0.2.0 |

## Keputusan

Nilai token mengikuti design system either, disimpan di `design/tokens/tokens.json` sebagai sumber tunggal:

- warna: peran dan nilai design system, dengan `brand` = Periwinkle (Dusk) atau Deep Indigo (Dawn) dan `accent` = Coral;
- tipografi: Manrope dan JetBrains Mono (F9), skala √φ 11, 14, 18, 23, 29, 37 px;
- spasi Fibonacci 3, 5, 8, 13, 21, 34 px; radius 5, 8, 13 px dan 23,6% untuk ikon aplikasi;
- motion `duration-fast` 144 ms dan `duration-panel` 233 ms; ikon UI garis 1,5 px pada grid 24 px;
- nama token memakai kebab-case seperti design system (`surface-1`, `fg-muted`), menggantikan penamaan titik di B12 (`surface.raised`).

Tetap berlaku dari B12 dan [dokumen design language](../development/design.md): lima prinsip desain, permukaan UI dan perilaku panel, library komponen dan enam state, pola interaksi, penulisan UX, aksesibilitas, dan aturan menjaga standar.

Q9 tetap terbuka untuk kalibrasi nilai di Quickshell dan Hyprland pada 0.2.0. Kalibrasi boleh mengubah angka lewat PR selama peran dan nama token tidak berubah; mengubah karakter brand tetap keputusan pemilik proyek.

## Konsekuensi

- B12 digantikan pada nilai yang disebut di atas; bagian B12 lainnya tidak berubah.
- `design/tokens/tokens.json` menjadi masukan `either-theme`. `just tokens-check` memeriksa struktur dan kontras (4,5:1 untuk teks).
- Ukuran yang tidak ada di design system (tinggi bar 36 px, lebar panel, tinggi baris dan tile) tetap nilai baseline sementara dan masuk kalibrasi 0.2.0.
- Motion selain `duration-fast` dan `duration-panel` (misalnya crossfade tema, parameter spring) ditetapkan saat kalibrasi; jangan menaruh angka lepas di komponen.
- Sumber ikon UI belum dipilih: Lucide dari B12 adalah kandidat bila dipakai dengan garis 1,5 px.
- `docs/development/rules.md`, `docs/development/design.md`, dan `docs/development/readiness.md` diperbarui bersama ADR ini.
