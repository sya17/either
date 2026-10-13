# SDLC dan workflow

Satu alur untuk semua kontributor. Rilis ditentukan oleh exit criteria di [roadmap](../researchs/08-roadmap.md), bukan tanggal.

## Tahapan

| # | Tahap | Yang terjadi | Keluaran | Pintu keluar |
| --- | --- | --- | --- | --- |
| 1 | Rencana | Scope rilis dari roadmap. Pertanyaan terbuka dijawab sebagai ADR. | scope rilis, ADR | exit criteria rilis jelas |
| 2 | Siapkan | Kebutuhan dipecah menjadi tugas kecil dengan hasil yang bisa dicek. | tugas | Definition of Ready |
| 3 | Implementasi | Cabang pendek dari `main`, commit kecil. | kode, tes, dokumen | `just check` hijau lokal |
| 4 | Review | Pull request ke `main`. | persetujuan | checklist PR |
| 5 | Integrasi | Squash merge, CI di `main`. | `main` hijau | CI hijau |
| 6 | Kandidat rilis | Uji end-to-end, visual, performa; dogfooding. | kandidat | gerbang kandidat |
| 7 | Rilis | Tag, changelog, paket dan ISO. | rilis | checklist rilis |
| 8 | Pemeliharaan | Patch, deprecation, kenaikan versi upstream, keamanan. | rilis patch | gerbang yang sama |

Tahap 6 dan 7 mulai relevan di 0.6.0 (distribusi). Sebelum itu rilis berarti tag di `main` dengan exit criteria terpenuhi.

## Alur satu tugas

1. Pilih tugas dari backlog rilis berjalan, prioritas tertinggi dulu. Jangan mulai tugas yang belum memenuhi Definition of Ready.
2. Pindahkan tugas ke `progress`. Maksimal dua tugas `progress` bersamaan.
3. Buat cabang `<type>/<topik>` dari `main` terbaru. Satu tugas, satu cabang. Dua sesi atau lebih yang berbagi satu working tree memakai `git worktree`, tidak berpindah cabang.
4. Kerjakan dalam commit kecil. Tulis tes bersama kodenya. Perbarui dokumen yang terdampak.
5. `just check` harus hijau sebelum push. Tulis di PR apa yang diuji dan apa yang tidak bisa diuji.
6. Buka PR dan pindahkan tugas ke `review`. Judul PR memakai format Conventional Commits karena menjadi pesan commit di `main`.
7. Setelah merge, pindahkan tugas ke `completed` dengan bukti (commit dan apa yang diuji). Hal baru yang muncul menjadi tugas terpisah di `draft`, bukan ditambahkan ke PR yang sama.

## Tugas

Satu tugas punya: rilis, tipe (Conventional Commit yang akan dihasilkan), prioritas, ukuran, dependensi, keputusan yang harus sudah selesai, dan bagian "selesai bila" dan "di luar scope". Setiap tugas berada di satu tahap: Prioritas: **P0** jalur kritis rilis (blocker atau dependensi tugas lain), **P1** scope rilis, **P2** boleh digeser ke rilis berikutnya bila rilis tertekan. Ukuran: **S** kurang dari setengah hari, **M** satu sampai dua hari, **L** dipecah dulu.

| Tahap | Artinya |
| --- | --- |
| `draft` | ide atau kebutuhan mentah; belum memenuhi definisi tugas |
| `backlog` | terdefinisi penuh; menunggu giliran, dependensi, atau keputusan |
| `progress` | sedang dikerjakan; maksimal dua sekaligus |
| `review` | selesai dikerjakan; menunggu PR di-merge atau verifikasi |
| `hold` | berhenti sementara; alasannya dicatat |
| `completed` | selesai dengan bukti (commit dan apa yang diuji) |
| `canceled` | tidak dikerjakan; alasannya dicatat |

Alurnya `draft` → `backlog` → `progress` → `review` → `completed`; `hold` dan `canceled` bisa dari tahap mana pun sebelum `completed`. Tugas yang selesai dikerjakan masuk `review`, dan baru `completed` setelah merge dan bukti tercatat.

Ide dan usulan baru masuk `draft`, bukan rilis berjalan. Masuk ke rilis hanya setelah pemilik proyek setuju; komponen baru hanya masuk 1.0 bila ada yang dikeluarkan (B15). Mengubah scope dicatat dengan satu baris alasan; memperbesar tugas diam-diam tidak diperbolehkan.

**Definition of Ready** (tugas boleh dimulai bila semua benar):

1. "Selesai bila" berisi hasil yang bisa dicek, bukan "implementasikan X".
2. Ada bagian "di luar scope".
3. Semua tugas yang menjadi dependensi sudah selesai.
4. Semua keputusan yang dibutuhkan sudah diselesaikan ADR (`diterima` atau `ditolak`).
5. Ukuran S atau M.
6. Tidak menyentuh F, tidak mengimplementasikan U atau I yang belum diterima, tidak memutuskan Q, dan masuk inventaris 1.0.

**Definition of Done**:

- Setiap butir "selesai bila" benar dan sudah diperiksa, bukan diasumsikan.
- Kode ter-merge ke `main`; `just check` dan CI hijau.
- Tes ditambahkan atau diperbarui; perubahan kontrak IPC berlabel `ipc-change`.
- Dokumen yang terdampak diperbarui; keputusan arsitektur baru punya ADR.
- Punya keybind atau entri menu bila relevan, dan bekerja di either Dusk dan either Dawn bila menyentuh UI.
- Bukti tercatat: commit, apa yang diuji, apa yang tidak. Kriteria yang gagal atau belum terukur tidak menjadi sukses lewat perubahan status.

## Keputusan

Keputusan arsitektur (pertanyaan terbuka, usulan, inkonsistensi) dianalisis dan diputuskan saat pekerjaan yang bergantung padanya akan dimulai, sebagai ADR. Prosedur, batas waktu, dan daftar keputusan terbuka ada di [docs/adr/README.md](../adr/README.md). Tugas yang bergantung pada keputusan terbuka tidak dimulai; sampai keputusannya ada, kerjakan hanya yang mengikuti baseline.

## Git

- **Trunk-based.** `main` selalu bisa dirilis. Cabang hidup singkat; tidak ada cabang jangka panjang selain `release/x.y` untuk perbaikan rilis lama.
- **Riwayat linear.** Rebase ke `main` sebelum PR; merge dengan squash. Tidak ada merge commit.
- **Conventional Commits**: `type(scope): deskripsi`, imperatif, maksimal 72 karakter, tanpa titik akhir.
  - Type: `feat` (minor), `fix` dan `perf` (patch), `refactor`, `docs`, `style`, `test`, `build`, `ci`, `chore`, `revert`.
  - Scope: nama area kode tanpa awalan `either-`: `ipc`, `eitherd`, `hypr`, `cli`, `shell`, `theme`, `config`, `keybinds`, `packaging`, `adr`.
  - `!` atau footer `BREAKING CHANGE:` menandai perubahan yang merusak. Selama `0.x`, itu menaikkan MINOR dan wajib ada di changelog beserta migrasi config.
  - Footer yang boleh: `Refs`, `Fixes`, `Closes`, `BREAKING CHANGE`. Tidak ada footer atau teks atribusi lain di pesan commit, judul PR, label, nama branch, atau tag. Hook lokal (`just setup`) menolak pesan yang salah format.
- Changelog dibangkitkan dengan `git-cliff` dari pesan commit di `main`, jadi tulis baris pertama untuk orang yang meng-update.
- Tag dan rilis hanya dibuat maintainer.

## Pull request

Judul: Conventional Commit. Isi: apa yang berubah dan kenapa, tugas atau ADR terkait, apa yang diuji dan apa yang tidak.

Checklist yang diperiksa penulis dan reviewer:

- [ ] Ada tugas, dan perubahan tidak keluar dari bagian "Out of scope"-nya
- [ ] Tidak menyentuh F; tidak mengimplementasikan U atau I yang belum diterima ADR; tidak memutuskan Q
- [ ] `just check` hijau; tes baru untuk logika baru dan setiap bug yang diperbaiki
- [ ] Kontrak IPC: bila berubah, label `ipc-change` dan snapshot skema diperbarui
- [ ] Tidak ada nilai visual literal (QML), tidak ada rahasia, input luar divalidasi
- [ ] Dokumen yang terdampak diperbarui; keputusan arsitektur baru punya ADR
- [ ] Bekerja di either Dusk dan either Dawn (bila menyentuh UI)

## Review

- Semua perubahan lewat PR; CI hijau sebelum merge. Merge dengan CI merah tidak diperbolehkan.
- Selama hanya ada satu maintainer, penulis meninjau diff sendiri memakai checklist di atas. Setelah ada dua maintainer atau lebih, setiap PR butuh satu persetujuan dari orang lain.
- Aturan branch protection adalah pengaturan forge dan menunggu keputusan platform (Q4).

## Rilis

| Hal | Aturan |
| --- | --- |
| Skema versi | paket `either-*`: `MAJOR.MINOR.PATCH`; kontrak IPC: bilangan bulat `proto`; config: bilangan bulat `schema`; ISO: kalender (`2026.11.0`, usulan, belum diputuskan) |
| Sebelum 1.0 | MINOR boleh merusak asal tercatat di changelog dan disertai migrasi; PATCH hanya perbaikan. Amplop IPC terkunci sejak 0.1.0; `proto = 1` dan `schema = 1` dibekukan di 0.9.0. Feature freeze di 0.9.0, lalu `1.0.0-rc.N` hanya berisi perbaikan |
| Tag | dari `main` (atau `release/x.y` untuk patch), oleh maintainer |
| Gerbang kandidat | semua tes PR, end-to-end (QEMU), visual, dan performa lolos; regresi visual disetujui manual |
| Gerbang stabil | kandidat dipakai harian tanpa bug blocker terbuka |
| Penandatanganan | menunggu keputusan kunci repo (U27); sampai ada, tidak ada artefak yang diterbitkan |
| Checklist rilis | `docs/release.md`, dibuat sebelum rilis distribusi pertama (0.6.0) |

## Pemeliharaan

- Perbaikan rilis lama dikerjakan di `release/x.y`, dipetik dari `main`.
- Kenaikan versi Hyprland atau Quickshell selalu PR tersendiri yang wajib lolos uji end-to-end, dan jangan mengirim Hyprland rilis `.0`. Ini usulan U17; dokumen sumber menyatakannya berlaku apa pun hasil strategi versi, jadi dipakai sebagai praktik hati-hati dan diformalkan di ADR strategi versi.
- Baca diff setiap kenaikan versi upstream, bukan hanya changelog.
- Perbaikan keamanan kritis didahulukan dari rilis terjadwal.
- Fitur atau key config yang dihapus diumumkan deprecated minimal satu MINOR sebelumnya.
