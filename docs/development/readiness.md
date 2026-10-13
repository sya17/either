# Kesiapan: yang sudah diterapkan dan yang menunggu keputusan

Dibuat 2026-10-13 dari tinjauan [01-keputusan-status.md](../researchs/01-keputusan-status.md). Perbarui tabel kedua setiap kali sebuah ADR diterima: hapus barisnya dan tulis berkas yang dibuat.

## Sudah diterapkan

Semua ini mengikuti baseline atau default yang netral terhadap keputusan terbuka.

| Area | Berkas | Catatan |
| --- | --- | --- |
| Struktur monorepo | folder sesuai B4 dengan README singkat per folder | `either-agentd` sengaja tidak ada (B16) |
| Workspace Rust | `core/Cargo.toml`, 7 crate kerangka, `clippy.toml`, `rust-toolchain.toml`, `Cargo.lock` | `just fmt-check lint test` hijau (cargo 1.99.0); toolchain dipin 1.99.0 |
| Dependensi | `core/deny.toml` (allowlist B10) | `just deny` hijau pada workspace kosong |
| Perintah | `justfile` | semua recipe dijalankan; `just check` hijau |
| Hook git | `scripts/hooks/` (`commit-msg`, `pre-commit`, `pre-push`, `install.sh`) dan `scripts/check-clean.sh` | diuji: format commit, daftar kata lokal, private key, `qmllint`, `shellcheck` |
| Aturan dan workflow | `docs/development/`, `CONTRIBUTING.md` | |
| Keputusan | `docs/adr/` (template, pelacak gerbang 0.1.0, ADR 0002) | ADR 0002 diterima pemilik proyek 2026-10-13 |
| Konteks dan design | `docs/overview.md`, `docs/development/design.md`, `design/tokens/tokens.json` | token dan 20 SVG aset diperiksa `just tokens-check`; lisensi aset di `design/LICENSES.md` (CC BY-SA 4.0, keputusan pemilik proyek); teks lisensi di `LICENSES/CC-BY-SA-4.0.txt` |
| Editor dan git | `.editorconfig`, `.gitignore` | |

## Menunggu keputusan

Diputuskan saat pekerjaan yang bergantung padanya tiba; prosedur dan batas waktunya di [docs/adr/README.md](../adr/README.md).

| Keputusan | Pemutus | Berkas yang menunggu | Sementara |
| --- | --- | --- | --- |
| Platform CI dan hosting (Q4) | Tech Lead | remote git sudah ada di GitHub (`sya17/either`, publik); sisanya: `.github/workflows/` atau konfigurasi Forgejo/Woodpecker; template PR dan issue; branch protection; `cargo-audit` terjadwal; `SECURITY.md`; pelaporan privat | gerbang sudah ada di `just check`; CI cukup memanggilnya |
| Lisensi (Q5) | **pemilik proyek** | `LICENSE`, header SPDX, bidang `license` final | `GPL-3.0-or-later` (B11) dipakai sementara di `core/Cargo.toml`; remote GitHub berstatus publik, jadi tanpa `LICENSE` kodenya belum berlisensi |
| Format config Hyprland (U1, I1, A4) | Tech Lead | `dotfiles/hypr/`, generator di `either-hypr` dan `either-theme`, target generator keybind, stub user | tidak ada kode yang menulis config Hyprland |
| Strategi versi Hyprland, Quickshell, Qt (I8, U16, U17) | Tech Lead | PKGBUILD, `packaging/repo/`, image CI, kanal rilis | PKGBUILD lokal untuk uji, tanpa repo |
| Nama D-Bus (Q7) | Tech Lead | konstanta nama bus di `eitherd` | `eitherd` tidak mengklaim nama bus |
| Spike Quickshell (A1) | developer | bar minimal | spike di 0.1.0 |
| Hyprland nested (A6) | developer, di mesin pemilik | bagian "Hyprland bersarang" di `setup.md` | verifikasi di 0.1.0 |
| Lokasi config Quickshell (Q14, U21) | Tech Lead | path di unit systemd dan PKGBUILD | `quickshell -p shell` saat dev |
| Settings terpisah dan modul QML bersama (U18) | Tech Lead | pecahan `shell/` (`either.ui`, `either.theme`, `either.ipc`) | hanya subfolder baseline: `components`, `modules`, `services`, `theme` |
| Kalibrasi nilai token dan ukuran permukaan (Q9, sisa setelah ADR 0002) | Tech Lead dan desainer | angka final tinggi bar, lebar panel, motion lain, sumber set ikon | nilai dari `tokens.json` dan B12 dipakai sebagai titik awal; kalibrasi di 0.2.0 |
| Bootloader, disk, enkripsi (Q3, U3, U23, I10, I11, I18, I19) | Tech Lead | `installer/`, `iso/` | belum disentuh |
| Model plugin (I13, U26) | Tech Lead; scope: **pemilik proyek** | `widget.sock`, `plugin.toml`, folder plugin | tidak ada API plugin (B16) |
| Aksesibilitas wajib (U10) | Tech Lead | lint `Accessible.*` | dianjurkan di `rules.md`, belum dilint |
| Kunci repo, SBOM, SLSA (U27) | Tech Lead | penandatanganan rilis, `either-keyring`, commit bertanda tangan | tidak ada artefak yang diterbitkan |
| DCO, SPDX, kebijakan trademark (U30) | Tech Lead | pengaturan DCO di CONTRIBUTING dan hook | belum diwajibkan |
| Threat model (U22) | Tech Lead | ADR 0001 | |
| Scope 1.0: firewall, hotspot, night light, game mode, dan kandidat lain | **pemilik proyek** | fitur terkait | tidak dikerjakan |

## Default setup awal

Tidak ada di dokumen riset; dibuat supaya workflow jalan. Boleh diubah lewat PR, tidak perlu ADR.

| Default | Alternatif |
| --- | --- |
| Bahasa: kode, commit, string CLI dalam Inggris; dokumen dalam Indonesia | semua Indonesia, semua Inggris |
| Semua perubahan lewat PR, squash merge, riwayat linear | commit langsung ke `main` untuk perubahan kecil |
| Pesan commit dicek hook lokal; tipe: `feat fix perf refactor docs style test build ci chore revert` | tambahkan `ipc` sebagai tipe seperti di contoh dokumen riset (bukan standar Conventional Commits; dipakai sebagai scope di sini) |
| Tugas dilacak di luar repo sampai platform issue dipilih (Q4); DoR dan DoD ada di `workflow.md` | issue tracker forge setelah Q4 diputuskan |
| `unsafe` dilarang, `unwrap` memberi peringatan | lebih longgar |
| Keputusan Q, U, dan I dianalisis dan diputuskan saat pekerjaan yang bergantung padanya tiba, dengan batas keras dari dokumen riset (prosedur di `docs/adr/README.md`) | semua diputuskan di muka sebelum 0.1.0 |
| Pesan commit hanya boleh berfooter `Refs`, `Fixes`, `Closes`, `BREAKING CHANGE` (ditambah `Signed-off-by` bila DCO diputuskan, U30) | bebas |
