# core

Cargo workspace untuk komponen Rust (B4). Semua perintah cargo dijalankan dari folder ini; `just` sudah melakukannya.

| Paket | Isi | Mulai diisi |
| --- | --- | --- |
| `crates/either-ipc` | kontrak protokol IPC: pesan, event, versi, JSON Schema | 0.1.0 |
| `crates/either-hypr` | client socket Hyprland (request dan event) | 0.1.0 |
| `crates/either-dbus` | bridge D-Bus (zbus) | klaim nama bus di 0.1.0 bila ADR Q7 mengharuskan; MPRIS di 0.3.0 |
| `crates/either-config` | skema `config.toml` dan validasi | 0.2.0 |
| `crates/either-theme` | token → QML, Hyprland, GTK, Qt, terminal | 0.2.0 |
| `eitherd` | daemon inti | 0.1.0 |
| `either-cli` | paket untuk binary `either` (`[[bin]] name = "either"`) | 0.1.0 |

- Tidak ada paket bernama `either` saja: nama itu sudah dipakai crate populer.
- `either-agentd` tidak dibuat sebelum 1.0.0 (B16).
- Isi crate sekarang hanya kerangka supaya workspace bisa dikompilasi; kodenya diisi pada rilis di kolom kanan.
- `Cargo.lock` wajib di-commit dan build memakai `--locked`. Lockfile dibuat dengan `cargo generate-lockfile` dan ikut di-commit.
- Aturan kode ada di [docs/development/rules.md](../docs/development/rules.md).
