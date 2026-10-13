# Architecture Decision Records

Setiap keputusan baseline (B) dan jawaban atas pertanyaan terbuka (Q), usulan (U), atau inkonsistensi (I) ditulis sebagai satu berkas `NNNN-judul-kebab.md`, dibuat dari [0000-template.md](0000-template.md). Nomor naik sesuai urutan dibuat; ADR 0001 dicadangkan untuk threat model.

## Aturan

- ADR menyelesaikan satu atau lebih ID dari `docs/researchs/` lewat field `Menyelesaikan`. Pekerjaan yang bergantung pada `U1` tidak dimulai sampai ada ADR `diterima` atau `ditolak` yang memuat `U1`.
- Status: `diusulkan` → `diterima` | `ditolak` → `digantikan`. ADR yang sudah diterima tidak diedit isinya; ubah dengan ADR baru yang menggantikannya.
- **F tidak bisa diubah oleh ADR.** Keputusan yang menyentuh F, scope 1.0, atau Q5, Q12, Q17, Q20 diputuskan pemilik proyek: tulis kata-katanya dan tanggalnya di bagian Keputusan.
- Keputusan lain diputuskan Tech Lead. Alasan teknis ditulis; jangan menyalin ulang isi dokumen riset, cukup tautkan.
- ADR berstatus `diterima` hanya setelah pemutus yang berwenang menyetujuinya; sebelum itu statusnya `diusulkan`.

## Memutuskan saat dibutuhkan

Keputusan tidak dianalisis di muka. Dianalisis dan diputuskan saat pekerjaan pertama yang bergantung padanya akan dimulai, atau lebih awal bila pemilik proyek meminta.

1. **Kenali.** Pekerjaan yang menyebut keputusan terbuka tidak bisa mulai; daftar keputusan yang sedang menghambat ada di papan tugas.
2. **Analisis.** Buat ADR `diusulkan` dari template. Kumpulkan bukti: bagian dokumen riset yang terkait, dokumentasi upstream (catat versinya), dan spike atau pengukuran bila perlu. Tulis tabel pilihan dengan kelebihan, kekurangan, dan apakah bisa dibalik.
3. **Rekomendasikan.** Tulis rekomendasi dan alasan teknisnya di bagian Keputusan, tandai jelas sebagai usulan.
4. **Putuskan.** Pemutus menjawab. ADR menjadi `diterima` atau `ditolak` dengan kata-kata pemutus dan tanggalnya.
5. **Terapkan.** Perbarui baris di tabel bawah dan `docs/development/readiness.md`; pekerjaan yang menunggu menjadi bisa dimulai.

Aturan tambahan:

- Keputusan harus selesai sebelum pekerjaan yang bergantung padanya mulai dikerjakan. Selama belum, kerjakan hanya yang mengikuti baseline (B); jangan mengimplementasikan U atau I, dan jangan menulis kode "sementara" yang diam-diam memilih sisi.
- Kolom "Batas" di bawah mengikuti dokumen riset. Keputusan yang sulit dibalik (layout disk dan bootloader, kunci repo, kontrak IPC, nama) tidak ditunda melewati batasnya.
- Sebuah rilis tidak ditutup selagi keputusan gerbangnya terbuka, kecuali pemilik proyek menggesernya secara tertulis di ADR atau tugas terkait.

## Keputusan terbuka dan batasnya

Turunan dari "Gerbang 0.1.0" di dokumen riset (`01-keputusan-status.md`, lokal). Perbarui kolom Status dan tautkan berkasnya saat ADR dibuat.

| # | Topik | Menyelesaikan | Dibutuhkan pertama oleh | Batas | Status |
| --- | --- | --- | --- | --- | --- |
| 1 | Format config Hyprland | U1, I1, A4 | generator keybind, tema dan stub Hyprland | blocker 0.1.0 | terbuka |
| 2 | Strategi versi Hyprland, Quickshell, Qt | I8, U16, U17, B7, A7 | PKGBUILD lokal, image CI | blocker 0.1.0 | terbuka |
| 3 | Platform CI dan hosting | Q4 | pipeline CI | blocker 0.1.0 ("CI hijau" adalah exit criteria) | terbuka |
| 4 | Nama D-Bus | Q7 | `eitherd`, bila ADR mengharuskan klaim nama | blocker 0.1.0 | terbuka |
| 5 | Spike API Quickshell | A1 | bar minimal | blocker 0.1.0 | spike |
| 6 | Hyprland nested untuk development | A6 | menjalankan shell dev | blocker 0.1.0 | verifikasi |
| 7 | Threat model (ADR 0001) | U22 | fitur keamanan pertama | 0.1.0 | terbuka |
| 8 | ADR baseline B1–B19 | B1–B19 | tidak ada | 0.1.0 | belum ditulis |
| 9 | Bootloader, layout disk, enkripsi | Q3, U3, U23, I10, I11, I18, I19 | installer | usulan 0.1.0; keras: sebelum layout disk dikunci (gerbang installer 0.6.0 atau 0.7.0) | terbuka |
| 10 | Model plugin | I13, U26 | kode plugin atau `widget.sock` | usulan 0.1.0 atau 1.x; B16 berlaku sampai ada ADR; scope: pemilik proyek | terbuka |
| 11 | Settings sebagai instance terpisah dan modul QML bersama | U18 | pemecahan `shell/` menjadi modul bersama | usulan 0.1.0; keras: sebelum kerangka Settings | terbuka |
| 12 | Theming pihak ketiga "hanya warna" | U19 | target GTK dan Qt di `either-theme` (0.2.0) | usulan 0.1.0 sampai 0.2.0 | terbuka |
| 13 | Lokasi config Quickshell | Q14, U21 | unit systemd dan PKGBUILD | 0.1.0 | terbuka |
| 14 | Cakupan dukungan NVIDIA | U2, I16 | deteksi GPU di installer | usulan 0.1.0; keras: sebelum 0.7.0 | terbuka |
| 15 | Kunci repo dan paket `either-keyring` | U27 | repo bertanda tangan (0.6.0) | usulan 0.1.0; keras: rencana insiden kunci sebelum 0.6.0 | terbuka |
| 16 | Cek ketersediaan nama | U30, Q20 | memilih org forge dan domain (Q4) | 0.1.0; kebijakan trademark sebelum 1.0; Q20: pemilik proyek | pengecekan |
| 17 | Validasi A9 dan A10 | A9, A10 | generator config dan job CI mingguan | 0.1.0 | verifikasi |
| 18 | Lisensi proyek | Q5 | membuat repo publik | sebelum repo publik; **pemilik proyek** | terbuka |
| 19 | Sumber nilai token visual | I17 | token dan tema | 0.2.0 | [ADR 0002](0002-design-system-sebagai-sumber-token.md), diterima |
