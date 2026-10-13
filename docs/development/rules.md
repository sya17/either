# Aturan development

Berlaku untuk semua kontributor. Kode dalam tanda kurung (F2, B2, U10) menunjuk sumbernya di `docs/researchs/`. Aturan tanpa kode adalah default setup awal: ubah lewat PR yang menjelaskan alasannya, atau lewat ADR bila menyentuh B atau F.

## Prinsip

- **Sederhana dulu.** Pakai yang sudah ada di repo, lalu stdlib, lalu dependensi yang sudah terpasang. Dependensi baru butuh alasan di PR: lisensi lolos `cargo-deny`, masih dirawat, dan tidak bisa diganti beberapa baris kode. Tidak ada abstraksi untuk satu pemakai.
- **Event-driven, tanpa polling.** `eitherd` dan shell tidak memeriksa state berulang; CPU saat idle mendekati nol.
- **Satu pemilik state: `eitherd` (B1).** Shell dan CLI tidak menyimpan state yang tidak bisa dibangun ulang dari `state.snapshot`.
- **Gagal tertutup.** Config salah tidak merusak desktop (B5), IPC yang gagal validasi menolak, approval yang timeout berarti tolak.
- **Komentar menjelaskan kenapa**, bukan apa. Tidak ada TODO tanpa rujukan ke tugas atau issue: `TODO(<id>)`.
- **Satu perubahan, satu alasan.** Refactor dan fitur tidak digabung dalam satu PR.

## Bahasa

| Hal | Bahasa |
| --- | --- |
| Kode, identifier, komentar, commit, nama branch, log, pesan error, string CLI | Inggris |
| Dokumen di `docs/`, README, tugas, ADR | Indonesia; istilah teknis tidak diterjemahkan |
| String yang tampil di UI shell | Inggris di kode, dibungkus `qsTr()`; terjemahan Indonesia dan Inggris datang di 0.8.0 |

Nama produk selalu "either" huruf kecil, termasuk di awal kalimat (F1).

## Penamaan

| Hal | Aturan | Contoh |
| --- | --- | --- |
| Paket Cargo | `either-<nama>`; tidak ada paket bernama `either` saja | `either-ipc`, `either-cli` |
| Binary | `either` (CLI) dan `eitherd` (daemon) | `[[bin]] name = "either"` |
| Unit systemd | `either-<nama>.service`, daemon `eitherd.service` | `either-shell.service` |
| Paket pacman | `either-<nama>`; meta-package `either-desktop` | `either-core` |
| Variabel lingkungan | `EITHER_` + HURUF_BESAR | `EITHER_LOG`, `EITHER_RUNTIME_DIR` |
| Subcommand CLI | `either <kelompok> <aksi>`, huruf kecil | `either theme set`, `either ipc ping` |
| Method IPC | `<domain>.<aksi>` huruf kecil; kata ganda dengan `_` | `hypr.dispatch`, `state.snapshot` |
| Topik event | `<domain>.<objek>.<peristiwa>` huruf kecil | `hypr.workspace.focused` |
| Nama error IPC | snake_case | `version_mismatch` |
| Kunci config | snake_case | `modules_left` |
| Kode Rust | konvensi rustc: `snake_case`, `UpperCamelCase`, `SCREAMING_SNAKE_CASE` | |
| Berkas QML | tipe: `PascalCase.qml`; folder modul huruf kecil | `components/Button.qml`, `modules/bar/` |
| Token desain | kebab-case semantik seperti di `design/tokens/tokens.json`, bukan nilai | `surface-1`, `fg-muted`, bukan `gray-800` |
| Skrip dan dokumen | kebab-case | `check-docs.py` |
| ADR | `docs/adr/NNNN-judul-kebab.md` | `0001-threat-model.md` |
| Branch | `<type>/<topik-kebab>` | `feat/ipc-envelope` |

## Struktur kode

Peta folder ada di [02-arsitektur.md](../researchs/02-arsitektur.md#struktur-repo-dotfiles-dan-konfigurasi); setiap folder punya README singkat. Yang mudah terlewat:

- Tipe pesan IPC hanya didefinisikan di `core/crates/either-ipc`. Crate lain mengimpor, tidak menyalin.
- Logika penting adalah subcommand Rust (`either theme set`, `either doctor`), supaya teruji dan punya `--help`. `scripts/` hanya untuk glue pendek tanpa state.
- Keybind memanggil `either ...`, bukan skrip ad hoc (B14).
- Berkas hasil generate (`shell/theme/`, keluaran `either-theme`) tidak diedit tangan; ubah sumbernya di `design/tokens/` (pengecualian sementara untuk `Tokens.qml` ada di bagian QML). `tokens.json` tidak diedit demi satu komponen; mengubah nilai atau nama token butuh PR yang menjelaskan alasannya dan menjalankan `just tokens-check`.
- Default distribusi di `/usr/share/either/`, milik user di `~/.config/either/` (tidak pernah ditimpa), hasil generate di `~/.local/state/either/` (B5). Kode menghormati `XDG_*` dan `EITHER_RUNTIME_DIR` supaya tes bisa mengalihkannya.

## Rust

- Edisi, versi, dan lint diatur sekali di `core/Cargo.toml` (`[workspace.lints]`). Crate baru wajib `[lints] workspace = true` dan memakai `*.workspace = true` untuk metadata. CI memakai `clippy -D warnings`.
- `unsafe` dilarang. Pengecualian butuh `#[allow(unsafe_code)]`, komentar `// SAFETY:`, dan disebut di PR.
- Error bertipe di crate library. Tidak ada `unwrap()` di kode non-test; `expect("alasan invarian")` hanya untuk invarian yang benar-benar terjamin. Daemon tidak boleh panic karena input dari socket: validasi di batas.
- Logging dengan `tracing` ke journald, level lewat `EITHER_LOG`. Log tidak boleh berisi rahasia, isi prompt, atau payload mentah.
- Dependensi dipakai lebih dari satu crate dideklarasikan sekali di `[workspace.dependencies]`. `Cargo.lock` di-commit, build memakai `--locked`.
- Waktu dan sleep: logika yang bergantung waktu menerima `now` sebagai parameter. Tes tidak tidur sungguhan.
- Tes unit di berkas yang sama (`#[cfg(test)]`); integrasi di `tests/` crate terkait. Setiap bug yang diperbaiki meninggalkan satu tes.

## QML (shell)

- Semua warna, ukuran, spasi, radius, dan durasi berasal dari token (`Tokens.qml`), bukan literal (B12, ADR 0002). `just qml-check` menggagalkan hex, nama warna, durasi, ukuran font, radius, spacing, padding, dan margin literal di luar `shell/theme/`; pengecualian satu baris ditandai `// lint-ok: alasan`. Sebelum `either-theme` membangkitkan `Tokens.qml` (0.2.0), berkas itu ditulis tangan di `shell/theme/` dengan nama token persis dari `tokens.json`, supaya migrasinya hanya mengganti isi berkas.
- Aturan desain lengkap (warna, tipografi, motion, komponen, state, copy UI) ada di [design.md](design.md).
- API Quickshell hanya dipanggil dari `shell/services/` (mitigasi API 0.x yang berubah). `modules/` dan `components/` memakai service itu saja.
- State datang dari `eitherd` lewat service IPC; perintah selalu lewat `eitherd`. Satu-satunya pengecualian berlaku: modul Hyprland bawaan Quickshell boleh **membaca** workspace dan window langsung (B3). Pengecualian PipeWire (U4) belum berlaku.
- Setelah koneksi pulih, ambil `state.snapshot` ulang. Shell tidak boleh crash saat `eitherd` restart.
- Satu panel terbuka pada satu waktu; Escape dan klik di luar menutupnya (B13).
- Aksesibilitas: `Accessible.role` dan `Accessible.name` pada kontrol interaktif. Ini usulan U10: dianjurkan sekarang, wajib dan dilint setelah ADR menerimanya.
- Setiap komponen diperiksa di either Dusk dan either Dawn. `qmllint` bersih.

## C++ (`shell-native/`)

Hanya untuk plugin QML di jalur yang berat (F2). PR wajib melampirkan pengukuran yang menunjukkan QML tidak cukup. Tanpa logika bisnis. Build dengan CMake dan Ninja. Belum ada kode dan belum ada kebutuhan terukur.

## Shell script

POSIX `sh`, lolos `shellcheck`, hanya glue pendek tanpa state. Hook git di `scripts/hooks/` memakai bash dan juga lolos `shellcheck`.

## Config dan generator

- Satu `config.toml`, field `schema`. Struktur berubah berarti `schema` naik, ada migrasi otomatis, dan cadangan `config.toml.bak-<versi>` (B5).
- Key yang akan dihapus ditandai deprecated minimal satu rilis MINOR sebelumnya, dan `either config check` memperingatkannya.
- Format config Hyprland (hyprlang atau Lua) menunggu ADR (U1, I1). **Jangan menulis stub, generator keybind, atau file tema Hyprland sebelum ADR itu diterima.**

## Kontrak IPC

- Amplop terkunci sejak 0.1.0 (B2): NDJSON, JSON-RPC 2.0, `hello` pesan pertama, event `{v, seq, ts, topic, data}`, maksimal 1 MiB per baris.
- Dalam satu versi `proto`: hanya boleh menambah field opsional, method, atau topik. Client mengabaikan yang tidak dikenal. Menghapus atau mengganti nama berarti `proto` naik.
- Perubahan kontrak berlabel `ipc-change`, snapshot JSON Schema diperbarui di PR yang sama, dan dokumen IPC ikut diperbarui.
- `hypr.dispatch` memakai allowlist. Tidak ada `exec` lewat IPC; peluncuran aplikasi memakai method khusus yang menerima nama berkas `.desktop`.

## Keamanan dalam kode

- Satu uid bukan batas kepercayaan: `SO_PEERCRED` hanya menyaring uid lain. Method berbahaya dibatasi menurut identitas client.
- Socket di `$XDG_RUNTIME_DIR/either/` (direktori 0700), mode 0600.
- Proses eksternal dijalankan dengan daftar argumen, tidak lewat shell. Teks dari luar diberi `--` di depannya.
- Teks dari luar (judul window, nama koneksi, isi notifikasi) di-escape sebelum masuk markup (Pango, rich text). Nilai dari config yang masuk ke Lua atau hyprlang di-escape sebagai string.
- Rahasia hanya di Secret Service (`oo7`): tidak di `config.toml`, log, argumen, atau repo.
- Tidak ada `sudo` di daemon. Hak istimewa lewat polkit (B6).
- Tidak ada telemetri di 1.0. Satu-satunya koneksi jaringan bawaan adalah pemeriksaan update.

## Dokumentasi

- `docs/researchs/` adalah arsip ekspor: tidak diedit tangan.
- Perilaku yang berubah memperbarui dokumen yang menjelaskannya di PR yang sama. Dokumen untuk pengguna menjelaskan yang sudah diimplementasikan, bukan rencana.
- Dokumentasi keybind, config, dan IPC dibangkitkan dari sumbernya (target 0.9.0), bukan ditulis tangan.
- Keputusan arsitektur ditulis sebagai ADR (`docs/adr/`), bukan di komentar kode atau pesan commit.
