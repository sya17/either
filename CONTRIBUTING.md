# Berkontribusi

either dibangun menuju 1.0.0 dengan scope yang dikunci. Perbaikan dan tugas yang sudah ada di backlog rilis berjalan diterima; fitur baru dibicarakan dulu di issue atau lewat pemilik proyek, karena komponen baru hanya masuk 1.0 bila ada yang dikeluarkan.

## Sebelum mulai

Dokumen riset sumber (`docs/researchs/`, tempat ID F, B, A, Q, U, I didefinisikan) tidak ikut repo ini. Minta ekspor terbarunya ke pemilik proyek dan taruh di folder itu; folder tersebut sengaja tidak dilacak git.

1. Baca [README](README.md) dan [docs/overview.md](docs/overview.md). Perhatikan arti F, B, A, Q, U, dan I: yang final tidak diubah, dan usulan belum boleh diimplementasikan sebelum ADR menerimanya.
2. Siapkan mesin: [docs/development/setup.md](docs/development/setup.md), lalu `just setup`.
3. Ambil tugas dari backlog rilis berjalan, prioritas tertinggi dulu. Tugas yang menunggu keputusan terbuka tidak dikerjakan. Platform issue menunggu keputusan hosting (Q4); sampai itu, tanyakan ke pemilik proyek.

## Alur singkat

1. Pindahkan tugas ke `progress`, buat cabang `<type>/<topik>` dari `main`.
2. Kerjakan dengan commit kecil, tulis tes bersama kodenya, perbarui dokumen yang terdampak.
3. `just check` harus hijau sebelum push.
4. Buka PR dengan judul Conventional Commit dan isi apa yang diuji dan apa yang tidak; pindahkan tugas ke `review`.
5. Setelah merge, pindahkan tugas ke `completed` dengan bukti.

Detail alur, format commit, checklist PR, dan aturan review ada di [docs/development/workflow.md](docs/development/workflow.md). Aturan kode dan penamaan di [docs/development/rules.md](docs/development/rules.md); perubahan tampilan juga mengikuti [docs/development/design.md](docs/development/design.md).

## Yang perlu diketahui

- **Commit:** `type(scope): deskripsi`, bahasa Inggris, maksimal 72 karakter. Hook lokal menolak format yang salah.
- **Bahasa:** kode, komentar, dan commit Inggris; dokumen Indonesia.
- **Keputusan arsitektur** ditulis sebagai ADR di `docs/adr/`, bukan di komentar atau pesan commit.
- **Kerentanan keamanan:** laporkan secara privat ke maintainer, bukan di issue publik. Saluran pastinya menunggu keputusan platform (Q4).
- **Lisensi dan sertifikasi kontribusi** (DCO atau lainnya) belum ditetapkan pemilik proyek (Q5, U30); berkas ini diperbarui saat keputusannya ada.
