# Kualitas dan keamanan

Gerbang kualitas didefinisikan sekali di `justfile` (`just check`). CI hanya memanggilnya, sehingga isinya tidak bergantung pada platform CI (Q4 masih terbuka). Sumber aturan: [03-build-rilis-distribusi.md](../researchs/03-build-rilis-distribusi.md) (B10) dan [04-keamanan.md](../researchs/04-keamanan.md).

## Gerbang

| Gerbang | Isi | Kapan |
| --- | --- | --- |
| **PR / merge ke `main`** | `just check`: format, lint, tes, `cargo-deny`, cek dokumen, token, dan literal QML. Perubahan kontrak IPC berlabel `ipc-change`. Pesan commit atau judul PR Conventional. | setiap PR |
| **Kandidat rilis** | uji end-to-end (QEMU), visual (Dusk dan Dawn), performa. Regresi visual disetujui manual. | setiap kandidat, mulai 0.6.0 sampai 0.9.0 |
| **Rilis stabil** | kandidat dipakai harian tanpa bug blocker terbuka. | setiap rilis |

## Alat

| Cek | Alat | Perintah | Status |
| --- | --- | --- | --- |
| Format Rust | rustfmt | `just fmt-check` | siap, hijau |
| Lint Rust | clippy `-D warnings`, lint workspace di `core/Cargo.toml` | `just lint-rust` | siap, hijau |
| Tes Rust | `cargo test --locked` | `just test` | siap, hijau (belum ada tes) |
| Lisensi dan advisory | `cargo-deny`, allowlist di `core/deny.toml` | `just deny` | siap, hijau |
| Lint shell | `shellcheck` | `just lint-shell` | siap, hijau |
| Lint QML | `qmllint` | `just lint-qml` | siap; belum ada QML |
| Tautan dokumen | `scripts/check-docs.py` | `just docs-check` | siap, dijalankan |
| Format pesan commit | hook `commit-msg` | `just setup` | siap, diuji |
| Rahasia | hook `pre-commit` menolak private key; `.gitignore` memblokir `*.pem *.key .env* *.kdbx` | `just setup` | siap |
| Daftar kata lokal per clone (`.git/info/forbidden-words`) terhadap isi, nama berkas, pesan commit, branch, tag | `scripts/check-clean.sh`, hook `commit-msg`, `pre-commit`, `pre-push` | `just clean-check` | siap; tanpa daftar, tidak memeriksa apa pun |
| Token: struktur, referensi, kontras teks 4,5:1; SVG aset hanya berwarna token dan tanpa skrip atau tautan | `scripts/check-tokens.py` | `just tokens-check` | siap, hijau |
| Lint QML kustom: tanpa literal warna, durasi, ukuran, radius, spasi | `scripts/check-qml-literals.py` | `just qml-check` | siap, diuji dengan contoh; belum ada QML sungguhan |
| Lint aksesibilitas QML | belum ada | — | menunggu ADR U10 |
| Kontrak IPC (snapshot `insta`, client lama vs daemon baru) | `insta` | `just test` | menunggu amplop IPC (0.1.0) |
| `either keybinds check` | CLI | — | menunggu generator keybind (0.1.0) dan ADR format Hyprland (U1) |
| Config Hyprland hasil generate vs Hyprland terbaru | sesi headless mingguan | — | menunggu ADR U1 dan validasi A9 |
| Audit advisory harian | `cargo-audit` terjadwal | — | menunggu platform CI (Q4) |
| Review dependensi baru | `cargo-vet` | — | usulan U27, target 0.5.0 |
| End-to-end, visual, performa, suspend | QEMU, `grim`, `QSG_RENDER_TIMING` | — | 0.6.0 sampai 0.8.0 |

## Dependensi

- `Cargo.lock` di-commit; build dan tes memakai `--locked`.
- Toolchain Rust dipin lewat `core/rust-toolchain.toml`. Saat ini 1.99.0; menaikkannya adalah PR tersendiri bersama perubahan `Cargo.lock`.
- Lisensi dependensi harus ada di allowlist `core/deny.toml` (MIT, Apache-2.0, BSD-2/3, ISC, Zlib, MPL-2.0, LGPL, GPL). Menambah lisensi ke daftar adalah perubahan baseline B10: PR dengan alasan, dicatat di `docs/licenses.md` bila dibuat.
- Dependensi baru: lisensi lolos, masih dirawat, dan tidak bisa diganti beberapa baris kode. Alasannya ditulis di PR.
- Aset (font, ikon, suara, wallpaper) wajib berlisensi bebas; sumbernya dicatat di `design/LICENSES.md`. Manrope (F9, OFL 1.1) sudah tercatat dan diaudit di sana; komponen lain di tabel lisensi A5 belum.
- Kenaikan versi Hyprland atau Quickshell: lihat [workflow](workflow.md#pemeliharaan).
- Image CI dikunci per digest (baseline). Platform CI belum dipilih, jadi belum ada berkasnya.

## Keamanan

Prinsip dan aturan kode ada di [rules.md](rules.md#keamanan-dalam-kode). Yang berlaku di level repo:

| Hal | Aturan | Status |
| --- | --- | --- |
| Threat model | ADR 0001, STRIDE per batas kepercayaan (U22) | menunggu ADR |
| Rahasia di repo | tidak boleh; hook dan `.gitignore` sebagai jaring pengaman, bukan pengganti kehati-hatian | siap |
| Kunci tanda tangan repo | offline, subkey di hardware key, tanda tangan tidak di CI publik (U27) | menunggu ADR |
| Commit dan tag bertanda tangan, DCO, header SPDX | usulan KEA (U27, U30) | menunggu ADR dan lisensi (Q5); belum diwajibkan |
| Pelaporan kerentanan | `SECURITY.md` dan pelaporan privat di forge (U31) | menunggu platform (Q4) |
| Permission Hyprland, lock screen, clipboard, hardening sistem | U5, U24, U25, U28 | sesuai rilis masing-masing, menunggu ADR |
| Plugin | tidak ada API plugin di 1.0 (B16); model tiga tingkat menunggu ADR (U26, I13) | menunggu keputusan pemilik proyek |

Temuan keamanan dilaporkan ke maintainer secara privat, bukan di issue publik. Saluran pastinya ditetapkan bersama platform (Q4).

## Performance budget

- Tidak ada frame terlewat saat membuka panel atau launcher pada refresh rate monitor (60, 120, 144 Hz).
- Tidak ada polling; semua update digerakkan event.
- Baseline memori dan waktu start diukur di 0.8.0. Mulai 0.9.0, regresi di atas baseline menggagalkan CI.
