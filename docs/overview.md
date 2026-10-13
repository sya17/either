# either: ringkasan project

Dokumen ini adalah titik awal untuk siapa pun yang baru membuka repo. Isinya ringkasan konteks, arsitektur, scope, dan status keputusan; detail teknis ada di dokumen lain yang ditunjuk di akhir. Bila ada perbedaan, urutan otoritasnya: keputusan final (F), lalu ADR berstatus `diterima` di [docs/adr/](adr/README.md), lalu baseline (B).

## Apa itu either

Sistem desktop Arch Linux di atas Hyprland, dibangun dari nol sebagai satu desktop utuh (bukan kumpulan dotfiles). Pengganti proyek uji coba sebelumnya yang fondasinya dinilai kurang matang. Nama produk selalu **either** huruf kecil.

Kebutuhan pemilik proyek: fondasi kuat dan stabil (K1); performa setara bahasa sistem (K2); tampilan cantik dan animasi mulus (K3); UI/UX sederhana tapi lengkap dan compact (K4); fitur dan window saling melengkapi (K5); seluruh tool gratis dan open source (K6); Hyprland tetap dasar dengan konsep tiling (K7); setelah base stabil, companion di dalam sistem dan CLI orkestrasi agent (K8).

**Urutan prioritas:** base dibangun dan dirilis stabil lebih dulu. Lapisan AI (companion, `either-agentd`, CLI agent) baru dikerjakan setelah 1.0.0; sebelum itu fondasinya hanya dijaga supaya bisa ditambahkan tanpa membongkar arsitektur.

## Keputusan final (F)

| ID | Keputusan |
| --- | --- |
| F1 | Nama **either**, selalu huruf kecil. Tema **either Dusk** (gelap) dan **either Dawn** (terang). |
| F2 | Stack: shell UI dengan Quickshell (Qt 6 + QML), daemon dan CLI dengan Rust, C++ hanya untuk plugin QML di jalur berat. Rust dan QML di proses terpisah. |
| F3 | Semua tool dan stack gratis dan open source. |
| F4 | Hyprland tetap dasar (tidak di-fork); konsep tiling dipertahankan. |
| F5 | Base dirilis stabil dulu; companion dan CLI orkestrasi agent sesudahnya. |
| F6 | Tiling: dwindle default, master dan scrolling per workspace; tampilan window dari token; smart gaps; dialog otomatis floating. |
| F7 | Palet brand: Periwinkle `#8DA2FF`, Soft Indigo `#A6B1E1`, Deep Indigo `#424874`, Coral `#FF9E7D` (aksen kecil), latar Dusk `#171826`, latar Dawn `#F7F3F0`, teks gelap `#2A2B3F`, teks terang `#E7E9F6`. |
| F8 | Logo: simbol Orbit (bingkai terbuka, inti, satelit), wordmark, dan lockup. |
| F9 | Font: Manrope untuk UI dan wordmark, JetBrains Mono untuk terminal dan CLI. |

Item bertanda † dan keputusan scope 1.0 adalah milik pemilik proyek.

## Arti status

| Status | Arti | Boleh diubah? |
| --- | --- | --- |
| **F** final | disetujui eksplisit pemilik proyek | hanya oleh pemilik proyek |
| **B** baseline | acuan kerja | lewat ADR |
| **A** asumsi | dianggap benar, belum diverifikasi | wajib dicek sebelum rilis yang membutuhkannya |
| **Q** pertanyaan terbuka | belum diputuskan | diputuskan sebelum rilis yang membutuhkannya; jangan diputuskan sendiri |
| **U** usulan | rekomendasi, menunggu Tech Lead | belum boleh diimplementasikan sebelum ADR menerimanya |
| **I** inkonsistensi | dua sumber berbeda | diselesaikan Tech Lead atau pemilik proyek; sampai itu, ikuti baseline |

Daftar yang masih menunggu keputusan dan efeknya pada kode: [development/readiness.md](development/readiness.md).

## Arsitektur

Lima lapisan. `eitherd` adalah satu-satunya pemilik state; semua komponen berbicara lewat satu kontrak IPC.

| Lapisan | Komponen | Bahasa |
| --- | --- | --- |
| L0 Kernel dan sistem | Linux (+ linux-lts cadangan), systemd, Btrfs + snapper | C (upstream) |
| L1 Sesi dan display | Hyprland, xdg-desktop-portal-hyprland, PipeWire + WirePlumber, uwsm | C++ (upstream) |
| L2 Core daemon | `eitherd` | Rust |
| L3 Shell UI | `either-shell` di atas Quickshell, plugin C++ untuk jalur berat | QML, C++ |
| L4 Intelligence | `either-agentd`, CLI agent, companion (setelah 1.0.0) | Rust, QML |

- **Proses:** setiap komponen adalah unit systemd user (bukan `exec-once`), sesi lewat uwsm. `eitherd.service` mulai setelah `graphical-session.target`; `either-shell.service` `Requires=eitherd.service`. Restart `eitherd` tidak boleh membuat shell crash; client menyambung ulang dan mengambil `state.snapshot`.
- **IPC (B2):** JSON-RPC 2.0 di atas NDJSON lewat Unix socket `$XDG_RUNTIME_DIR/either/core.sock` (0600, `SO_PEERCRED`). Pesan pertama wajib `hello {proto, client, version}`. Event berbentuk `{v, seq, ts, topic, data}`. Maksimal 1 MiB per baris. Dalam satu versi `proto` hanya boleh menambah field opsional, method, atau topik. Amplop terkunci sejak 0.1.0; `proto = 1` stabil di 0.9.0.
- **Pengecualian baca (B3):** modul Hyprland bawaan Quickshell boleh membaca state workspace dan window langsung. Semua perintah lewat `eitherd`.
- **Config (B5):** satu `config.toml` dengan empat lapisan: default kompilasi, default distribusi (`/usr/share/either/`), override user (`~/.config/either/`, tidak pernah ditimpa), hasil generate (`~/.local/state/either/`). Validasi skema, hot reload, migrasi otomatis lewat field `schema`; config yang salah tidak merusak desktop.
- **Keamanan (B6):** daemon tanpa root, polkit untuk hak istimewa, rahasia di Secret Service, `hypr.dispatch` lewat allowlist (tidak ada `exec`).
- **Distribusi (B7–B9):** repo pacman bertanda tangan, meta-package `either-desktop` sebagai satu-satunya daftar paket, `either setup` di Arch yang sudah ada, ISO archiso; Btrfs + snapper + snap-pac.
- **Integrasi (B17):** notification server dan polkit agent dipegang shell; portal hyprland + gtk; gnome-keyring.

Peta repo:

```text
core/            Cargo workspace (Rust): crates/either-{ipc,hypr,dbus,config,theme}, eitherd, either-cli
shell/           Quickshell (QML): components/, modules/, services/, theme/
shell-native/    plugin C++ untuk QML (hanya bila terukur perlu)
design/          tokens/ (sumber tunggal token), assets/ (logo, ikon)
dotfiles/        konfigurasi default (hypr, gtk, qt, kitty, fish)
scripts/         glue kecil dan pemeriksa; hooks/ untuk git
packaging/       PKGBUILD, meta-package, repo pacman
iso/, installer/ profil archiso dan archinstall
tests/           uji integrasi (VM QEMU, kontrak IPC)
docs/            dokumen ini, development/, adr/
```

## Scope sampai 1.0.0

**Termasuk:** bar dengan island, launcher, notifikasi dan pusat notifikasi, OSD, media, cheatsheet keybind, control center, clipboard dan emoji, capture, menu power dan menu either, lock screen dan idle, greeter, wallpaper, aplikasi Settings, wizard first-run, distribusi (repo bertanda tangan, `either setup`, ISO), dan tema Dusk dan Dawn untuk seluruh sistem.

**Di luar scope:** lapisan AI (hanya fondasinya: socket agent, topik `agent.*`, shortcut `Super + A` dicadangkan), API plugin publik (B16), overview dengan pratinjau live, dock, widget desktop, tema Firefox dan VS Code, HDR dan screen reader penuh.

Komponen baru hanya masuk 1.0 bila ada yang dikeluarkan (B15); penambahan atau pengurangan adalah keputusan pemilik proyek.

## Rencana rilis

Rilis memakai exit criteria, bukan tanggal. Dogfooding sejak 0.2.0. Feature freeze di 0.9.0.

| Rilis | Tema |
| --- | --- |
| 0.1.0 | Walking skeleton: monorepo, CI, amplop IPC, `eitherd` + bridge Hyprland, bar minimal, `either ipc ping`, PKGBUILD lokal |
| 0.2.0 | Tema dan config: token, `either-theme`, Dusk/Dawn, `config.toml`, wallpaper |
| 0.3.0 | Shell inti: launcher, notifikasi, OSD, media, keybind dan cheatsheet |
| 0.4.0 | Integrasi sistem: control center, clipboard, capture, menu, polkit agent, aplikasi bawaan |
| 0.5.0 | Sesi: lock screen, idle, greeter, profil daya, suspend |
| 0.6.0 | Distribusi: repo, `either-desktop`, `either setup`, `either update`, snapshot |
| 0.7.0 | Installer: ISO, Btrfs, bootloader, LUKS2 opsional, Plymouth |
| 0.8.0 | Polish: matriks hardware, Settings, wizard, aksesibilitas, bahasa, baseline performa |
| 0.9.0 | Beta: freeze, `proto` dan `schema` dibekukan, regresi menggagalkan CI, situs docs |
| 1.0.0 | Stabil: RC terakhir tanpa perubahan kode, artefak terbit |

Selama `0.x`, rilis MINOR boleh merusak asal tercatat di changelog dan disertai migrasi config.

## Istilah

| Istilah | Arti |
| --- | --- |
| `eitherd` | daemon inti (Rust): pemilik state, config, tema, event bus |
| `either` | CLI untuk mengontrol sistem; semua keybind memanggilnya |
| `either-shell` | shell UI berbasis Quickshell |
| `proto` | versi mayor kontrak IPC, terpisah dari versi paket |
| Island | elemen di tengah bar yang membesar menjadi media, OSD, indikator rekam, launcher |
| Token | nilai desain bernama (warna, spasi, radius, motion); satu-satunya sumber gaya |
| ADR | catatan satu keputusan beserta alasannya di `docs/adr/` |
| Walking skeleton | rilis pertama yang tipis tapi melewati semua lapisan ujung ke ujung |
| Dwindle, master, scrolling | tiga layout tiling Hyprland yang didukung |
| Layer-shell | protokol Wayland untuk bar, panel, dan overlay |

## Dokumen lain

| Kebutuhan | Dokumen |
| --- | --- |
| Setup mesin, build, debug | [development/setup.md](development/setup.md) |
| Aturan kode dan penamaan | [development/rules.md](development/rules.md) |
| Design dan standar UI | [development/design.md](development/design.md) |
| Alur kerja, commit, PR, rilis | [development/workflow.md](development/workflow.md) |
| Test, lint, CI, keamanan | [development/quality-security.md](development/quality-security.md) |
| Yang menunggu keputusan | [development/readiness.md](development/readiness.md) |
| Keputusan arsitektur | [adr/README.md](adr/README.md) |
| Kontribusi | [../CONTRIBUTING.md](../CONTRIBUTING.md) |
