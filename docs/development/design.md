# Design dan standar UI

Aturan desain untuk siapa pun yang menyentuh tampilan: shell, tema aplikasi, instalasi, dokumen. Sumber nilai adalah design system either, disimpan sebagai [design/tokens/tokens.json](../../design/tokens/tokens.json) dan diadopsi lewat [ADR 0002](../adr/0002-design-system-sebagai-sumber-token.md). Yang tidak diatur design system (permukaan, komponen, interaksi, penulisan, aksesibilitas) tetap mengikuti baseline B12 dan B13.

## Status nilai

| Hal | Status |
| --- | --- |
| Nama (F1), palet brand (F7), logo Orbit (F8), font Manrope dan JetBrains Mono (F9) | final, hanya pemilik proyek yang mengubah |
| Nilai token di `tokens.json` | diadopsi lewat ADR 0002; dikalibrasi di 0.2.0 (Q9) tanpa mengubah peran dan nama token |
| Ukuran permukaan (tinggi bar, lebar panel, tinggi baris) | nilai sementara dari B12; dikalibrasi di 0.2.0 |
| Parameter motion selain `duration-fast` dan `duration-panel` | ditetapkan saat kalibrasi 0.2.0 |

## Identitas dan suara

- Nama selalu **either**, huruf kecil, termasuk di awal kalimat, judul, dan CLI. Tema: **either Dusk** (gelap) dan **either Dawn** (terang); kata tema berhuruf besar. Modul: `either shell`, `either companion`, `either cli`.
- Dusk dan Dawn memakai geometri, ukuran, dan gerak yang sama; hanya warna yang berbeda.
- Karakter: tenang, presisi, compact. Tulis seperti insinyur teliti yang peduli pada rasa pakai.
- Kalimat pendek dan datar: "Tema diterapkan.", bukan "Tema Anda berhasil diterapkan!". Tanpa tanda seru, emoji, atau kata promosi. Teknis hanya bila membantu: "Config Hyprland dimuat ulang (12 ms)".
- Sentence case untuk semua label dan judul. Tombol memakai kata kerja yang menyebut hasilnya ("Sambungkan", "Hapus riwayat"); hindari "OK" untuk keputusan penting.
- Pesan error tidak menyalahkan user dan menyebut langkah berikutnya; detail teknis di balik "Detail". Angka selalu dengan satuan. Satu konsep, satu istilah.

## Logo dan simbol Orbit

Bingkai terbuka (shell) di sekitar inti (core) dengan satelit di celah bingkai. Geometri sama di Dusk dan Dawn.

- Konstruksi mengikuti rasio emas φ pada kotak B = 36u; tidak digambar ulang tangan.
- Dua master optik: display (garis B/φ⁶) untuk 32 px ke atas, kecil (garis B/φ⁵, satelit ×√φ) untuk 16–24 px.
- Lockup horizontal: simbol, lalu "either" dalam Manrope Light 300. Lebar minimal 89 px; di bawahnya pakai simbol saja. Ruang bebas H/φ di semua sisi (sudah ada di berkas SVG).
- Ikon aplikasi: simbol = tile/φ, sudut tile = tile/φ³ (23,6%).
- Di bawah 32 px pada Dawn pakai master mono: Coral di atas `#F7F3F0` hanya 1,8:1.
- Coral hanya di inti. Varian: Dusk, Dawn, mono terang, mono gelap, dan tanpa aksen (inti `brand-secondary`, untuk konteks yang bersaing dengan status error).
- **Jangan**: mengubah spasi atau berat, menulis "Either" atau "EITHER", menambah efek atau gradien, memindahkan satelit, menambah satelit kedua, lencana, atau warna per produk.
- Simbol bukan ikon UI, kecuali tombol launcher dan menu either.
- Berkas master (SVG) ada di `design/assets/` ([README](../../design/assets/README.md)). Jangan membuat logo baru dari spesifikasi ini; pakai berkas master.
- **Hindari** pada identitas: logo atau warna Arch, Tux, Hyprland, Qt, Rust; prompt terminal, tanda kurung, roda gigi, monitor, perisai; ikon berkilau, otak, robot, maskot; yin-yang atau matahari-bulan harfiah; gaya cyberpunk, gaming, atau cloud; gradien dan 3D.

## Warna

Semua warna dibaca dari token, tidak pernah ditulis sebagai hex di komponen. Daftar lengkap dan nilainya di `tokens.json`.

| Peran | Token | Dusk | Dawn |
| --- | --- | --- | --- |
| Lapisan terdalam (bar, lock) | `surface-crust` | `#11121D` | `#EDE6E0` |
| Dasar (desktop, window) | `surface-0` | `#171826` | `#F7F3F0` |
| Panel di atas dasar | `surface-1` | `#1F2133` | `#FFFDFB` |
| Overlay (menu, tooltip, baris hover) | `surface-2` | `#2A2C42` | `#EFE9E4` |
| Seleksi | `highlight-med` | `#424874` | `#DCD6F7` |
| Border window, kursor | `highlight-high` | `#4A4E6B` | `#C9C3E6` |
| Teks utama | `fg` | `#E7E9F6` | `#2A2B3F` |
| Teks sekunder | `fg-muted` | `#A6B1E1` | `#5E6288` |
| Komentar, placeholder, disabled | `fg-faint` | `#7F85AD` | `#6A6E8C` |
| Simbol, fokus, workspace aktif, tombol primary | `brand` | `#8DA2FF` | `#424874` |
| Teks di atas isi `brand` | `on-brand` | `#171826` | `#F7F3F0` |
| Aksen kecil | `accent` | `#FF9E7D` | `#FF9E7D` |

Status memakai `status-error`, `status-warning`, `status-success`, `status-info` (merujuk `ansi-red`, `ansi-yellow`, `ansi-green`, `ansi-cyan`). Terminal memakai `ansi-*`; varian terang 8–15 memakai hue yang sama, hanya hitam dan putih yang bergeser. `ansi-orange` adalah warna tambahan untuk editor.

Aturan:

- **Periwinkle memimpin**, Soft Indigo (`fg-muted`, `brand-secondary`) mendukung. `brand-secondary` hanya untuk lapisan kedua logo dua-nada: 1,9:1 di Dawn, jangan jadi teks atau ikon tunggal.
- **Coral paling banyak satu titik kecil**: penanda belum dibaca, indikator rekam. Bukan warna error, bukan di logo selain inti, bukan teks di Dawn (1,8:1).
- Kedalaman lewat nada, bukan bayangan: `surface-0` → `surface-1` → `surface-2`. `border` (garis tipis) hanya bila nada tidak cukup, dan tidak pernah menjadi satu-satunya penanda.
- Kontras teks minimal 4,5:1 terhadap `surface-0` di tema masing-masing; elemen UI dan ikon penting minimal 3:1. Pasangan yang gagal tidak boleh masuk token; `just tokens-check` memeriksanya.
- Warna bukan satu-satunya penanda: status selalu disertai ikon atau teks.

## Tipografi

Manrope untuk UI shell dan wordmark (Light 300 untuk display, 400–600 untuk UI); JetBrains Mono untuk terminal dan CLI. Skala √φ dari 14 px; line height φ (1,618) untuk teks dan √φ (1,272) untuk judul. Semua ukuran dikalikan `font.scale`.

| Token | Ukuran / berat | Dipakai untuk |
| --- | --- | --- |
| `display` | 37 / 300 | jam lock screen dan greeter, layar sambutan |
| `headline` | 29 / 400 | judul halaman Settings |
| `title` | 23 / 500 | judul panel |
| `subtitle` | 18 / 500 | judul notifikasi dan dialog |
| `body` | 14 / 400 | teks UI umum |
| `body-strong` | 14 / 600 | judul baris, modul bar |
| `caption` | 11 / 500 | waktu, petunjuk, label; pakai `fg-muted` |
| `mono` | 13 / 400 | keluaran CLI, nilai config, terminal |

## Spasi, radius, dan ikon

| Token | Nilai | Dipakai untuk |
| --- | --- | --- |
| `space-1` | 3 px | selisih tipis, jarak ikon ke label |
| `space-2` | 5 px | jarak antar modul bar |
| `space-3` | 8 px | padding baris; `gaps_in` Hyprland |
| `space-4` | 13 px | padding kartu dan panel; `gaps_out` Hyprland |
| `space-5` | 21 px | jarak antar grup di Settings dan control center |
| `space-6` | 34 px | margin halaman, jarak bagian |
| `radius-sm` | 5 px | chip, input, pill workspace |
| `radius-md` | 8 px | tombol, baris daftar, `rounding` window Hyprland |
| `radius-lg` | 13 px | panel: launcher, kartu notifikasi, control center |
| `radius-icon` | 23,6% | sudut ikon aplikasi (1/φ³) |

- Elemen di dalam panel memakai radius lebih kecil dari panelnya.
- Layout bergrid 12 kolom dengan margin dan gutter Fibonacci (contoh 55 dan 21 px) dan tinggi baris dari 89, 144, 233 px.
- Ikon UI: garis 1,5 px, sambungan membulat, grid 24 px, warna `fg` atau `fg-muted`. Sumber set ikon belum dipilih (ADR 0002). Target klik minimal 32 × 32 px.
- Window Hyprland: `gaps_in` = `space-3`, `gaps_out` = `space-4`, `rounding` = `radius-md`, border aktif `brand` 2 px, border tidak aktif `border`. Penulisannya menunggu ADR format config (U1).

## Motion

- `duration-fast` 144 ms untuk hover, press, perubahan warna kecil. `duration-panel` 233 ms, ease-out, untuk panel, popover, launcher, dan pindah workspace. `duration-instant` 0 ms untuk reduced motion. Pergantian Dusk dan Dawn berupa satu crossfade seluruh desktop.
- Gerak punya asal dan tujuan: panel muncul dari elemen yang membukanya dan menutup kembali ke sana. Saat satu panel bergerak, bagian layar lain diam.
- Hanya animasikan opacity, scale, dan posisi lewat transform; bukan width, height, atau anchor. Total stagger tidak melebihi satu `duration-panel`; animasi bisa dibatalkan atau dibalik di tengah jalan.
- Tidak ada angka durasi lepas di komponen; semua lewat token. Blur dikerjakan compositor (layer rule untuk namespace `either-*`), bukan QML. Panel berat memakai `Loader { asynchronous: true }`; `layer.enabled` dan shader hanya setelah diukur.
- **Reduced motion**: gerak posisi dan scale diganti crossfade `duration-fast`, spring dimatikan, stagger dihapus. Informasi tidak pernah disampaikan lewat animasi saja.

## Permukaan UI

Bar adalah pusat navigasi (B13). Setiap panel turun dari elemen bar yang membukanya, hanya satu panel terbuka pada satu waktu, dan island di tengah bar adalah satu-satunya elemen yang bisa membesar menjadi permukaan lain.

| Permukaan | Lebar | Posisi |
| --- | --- | --- |
| Bar | penuh, tinggi 36 px | mengambang, jarak `space-4` dari tepi (sama dengan `gaps_out`); zona kiri (logo, workspace, judul window), tengah (island), kanan (tray, status, jam) |
| Menu either | 280 px | turun dari logo, sejajar tepi kiri |
| Launcher | 560 px | tengah, tumbuh dari island; tinggi maksimum 60% layar |
| Control center | 360 px | turun dari ikon status, sejajar tepi kanan |
| Pusat notifikasi | 380 px | turun dari jam, sejajar tepi kanan |
| Toast | 360 px | kanan atas di bawah bar; maksimal 3 bertumpuk |
| Island | lebar isi, hingga 420 px | tengah bar; tinggi 36 px menjadi hingga 120 px |
| Dialog (polkit, power) | 400 px | tengah, latar diredupkan |
| Settings | window biasa dengan sidebar | dikelola Hyprland |

Lebar dan tinggi di tabel ini adalah nilai sementara dari B12 dan dikalibrasi di 0.2.0 (ADR 0002). Padding panel `space-4`; isi yang melebihi tinggi maksimum di-scroll di dalam panel. Setiap monitor punya bar sendiri; panel dan launcher terbuka di monitor yang difokus; island lengkap hanya di monitor utama. Semua ukuran dalam piksel logis; fractional scaling diuji di 1.0, 1.25, 1.5, 2.0; di lebar logis 1366 px atau kurang, `density = compact` aktif otomatis.

Tiling mengikuti F4 dan F6: dwindle default, master dan scrolling per workspace, smart gaps (satu window berarti tanpa gap dan border), dialog dan jendela kecil otomatis floating.

## Komponen dan interaksi

- Semua permukaan dibangun dari library `shell/components/`. Panel tidak boleh membuat tombol, toggle, atau slider sendiri. Library v1: `Button` (primary, secondary, ghost, icon; satu primary per tampilan), `Toggle` (berlaku langsung, tanpa Simpan), `Tile`, `Slider`, `ListItem`, `Card`, `Popover` dan `Menu`, `SearchField`, `Toast`, `Dialog`, `Island`.
- Setiap komponen interaktif punya enam state: rest, hover, pressed, focus (cincin `brand` 2 px saat navigasi keyboard), disabled, selected (latar `highlight-med`).
- Keyboard dan mouse setara: Escape dan klik di luar selalu menutup panel. Detail di tempat (panah di tile membuka daftar di dalam panel). Undo lebih baik daripada konfirmasi.
- Umpan balik: perubahan kecil inline, hasil aksi lewat `Toast`, kejadian aplikasi lewat notifikasi, keputusan berisiko lewat `Dialog`. Ingat state terakhir, kecuali launcher yang selalu kosong.
- State kosong: ikon 32 px, satu kalimat, satu aksi bila ada. Loading: di bawah 150 ms tidak menampilkan apa pun; daftar yang lebih lama memakai skeleton; aksi tanpa progres memakai spinner kecil di tombol. Error: di dekat sumbernya, apa yang terjadi dan cara memperbaiki, tombol "Coba lagi". Tidak tersedia: kontrol disabled dengan alasan singkat saat hover atau fokus.
- Suara memakai freedesktop sound theme, hanya untuk kejadian penting, dan diam saat DND aktif.

## Aksesibilitas

- Kontras seperti di bagian Warna. Cincin fokus selalu terlihat saat navigasi keyboard; target minimal 32 × 32 px dengan jarak minimal `space-1`.
- Layout utuh hingga `font.scale` 150%; teks panjang dipotong dengan elipsis dan tooltip.
- Setiap komponen interaktif memberi nama dan peran lewat `Accessible`. Usulan U10 mewajibkan `Accessible.role` dan `Accessible.name` dan memeriksanya dengan lint; belum diterima, jadi dianjurkan tetapi belum dilint.
- Navigasi penuh dengan keyboard, kontras tinggi, dan reduced motion tersedia di 0.8.0. Screen reader penuh tidak dijanjikan di 1.0.

## Menjaga standar

1. **Token sebagai satu-satunya sumber.** Warna, ukuran, radius, spasi, dan durasi hanya dari `Tokens.qml`, yang dibangkitkan dari `tokens.json` oleh `either-theme` (0.2.0). Jangan mengedit berkas hasil generate dan jangan mengedit `tokens.json` untuk akal-akalan sebuah komponen.
2. **Pemeriksaan otomatis** (bagian `just check`):
   - `just tokens-check`: struktur token, referensi, kontras WCAG, dan isi SVG di `design/assets/`;
   - `just qml-check`: QML di luar `shell/theme/` tidak boleh memuat hex, nama warna, durasi, ukuran font, radius, spacing, padding, atau margin literal. Pengecualian satu baris: komentar `// lint-ok: alasan`.
3. **Galeri komponen** (`either gallery`, direncanakan): setiap komponen dalam enam state, di Dusk dan Dawn, termasuk reduced motion; screenshot-nya menjadi baseline uji visual.
4. **Checklist review UI** untuk setiap PR yang menyentuh shell:
   - [ ] Hanya memakai token dan komponen dari library
   - [ ] Tampil benar di either Dusk dan either Dawn
   - [ ] Bisa dioperasikan penuh dengan keyboard; Escape menutup
   - [ ] Animasi punya asal yang jelas dan memakai token motion
   - [ ] Bekerja dengan reduced motion dan `font.scale` besar
   - [ ] Coral tidak muncul di luar titik aksen; teks memenuhi kontras

## Di luar shell

Konsistensi di GTK, Qt, terminal, kursor, font, wallpaper, ikon, lock screen, greeter, dan Plymouth diatur pipeline tema (`either theme build`). Aturan minimalnya: bila sebuah aplikasi tidak bisa dibuat persis sama, warna latar, warna aksen, dan mode gelap atau terangnya harus cocok. Cara tiap target (apakah hanya mewarnai ulang atau mengganti tema) menunggu ADR theming pihak ketiga (U19); format config Hyprland menunggu ADR U1. Tema Firefox dan VS Code di luar scope 1.0.
