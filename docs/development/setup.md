# Setup, build, dan debug

Target: Arch Linux dengan sesi Hyprland. Bagian bertanda **belum diverifikasi** adalah rencana dari dokumen riset yang belum pernah dijalankan di repo ini; verifikasinya adalah tugas tersendiri.

## 1. Prasyarat

```bash
sudo pacman -S --needed base-devel git rustup just shellcheck cargo-deny qt6-declarative
```

`qt6-declarative` menyediakan `qmllint`. Untuk bekerja di shell, tambahkan `quickshell hyprland uwsm`. Opsional: `cargo-audit`, `git-cliff`, `socat` (debug socket), `qemu-desktop` dan `devtools` (uji VM dan build paket di chroot bersih, mulai 0.1.0 untuk paket lokal dan 0.7.0 untuk ISO).

Toolchain Rust dipasang rustup dari `core/rust-toolchain.toml`. Rustup tidak memasangnya otomatis:

```bash
cd core && rustup toolchain install   # tanpa argumen: memasang toolchain dari rust-toolchain.toml
```

## 2. Clone dan hook

```bash
git clone https://github.com/sya17/either.git either && cd either   # CI dan pelaporan kerentanan menunggu keputusan (Q4)
just setup                                  # mengaktifkan hook commit-msg dan pre-commit
just check                                  # semua gerbang PR
```

`core/Cargo.lock` sudah dibuat dan harus ikut di-commit. Setiap perubahan dependensi memperbaruinya di PR yang sama; build dan tes memakai `--locked`.

## 3. Perintah sehari-hari

| Perintah | Fungsi |
| --- | --- |
| `just` | daftar semua perintah |
| `just check` | semua gerbang PR: format, lint, tes, deny, dokumen, token, literal QML |
| `just fmt` | format kode Rust |
| `just test` | tes workspace |

Perintah `just dev`, `just pkg`, `just iso`, `just vm` ada di rencana (lihat [03-build-rilis-distribusi.md](../researchs/03-build-rilis-distribusi.md#lingkungan-development)) dan ditambahkan saat dibutuhkan.

## 4. Menjalankan project

Belum ada yang bisa dijalankan. Urutan yang akan berlaku di 0.1.0:

| Setelah | Perintah |
| --- | --- |
| `eitherd` ada | `cd core && EITHER_RUNTIME_DIR=/tmp/either-dev EITHER_LOG=debug cargo run -p eitherd` |
| `either ipc ping` ada | `cd core && EITHER_RUNTIME_DIR=/tmp/either-dev cargo run -p either-cli -- ipc ping` |
| bar minimal ada | `quickshell -p shell` |
| PKGBUILD lokal ada | `cd packaging/pkgbuilds/either-core && makepkg -si` |

`EITHER_RUNTIME_DIR` memisahkan socket dev dari `eitherd` asli milik sesi, supaya keduanya tidak bentrok. Variabel ini kontrak dari dokumen riset dan harus diimplementasikan di `eitherd` (0.1.0).

### Hyprland bersarang (nested)

Rencana dokumen riset: mengembangkan shell di dalam Hyprland yang berjalan sebagai window di sesi yang ada, supaya sesi harian tidak rusak. **Belum diverifikasi (A6).** Sampai terverifikasi:

- jangan menjalankan Hyprland bersarang atau headless dari skrip; pemilik mesin yang mencobanya dan mencatat hasilnya di sini;
- setelah terverifikasi, perintah dan batasannya ditulis di sini.

## 5. Debug

| Kebutuhan | Cara |
| --- | --- |
| Log daemon | `journalctl --user -u eitherd -f`; level lewat `EITHER_LOG` (`trace`, `debug`, `info`) |
| Bicara langsung ke core socket | `socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/either/core.sock`, lalu kirim satu baris JSON, misalnya `{"jsonrpc":"2.0","id":1,"method":"hello","params":{"proto":1,"client":"debug","version":"0.1.0"}}` |
| State Hyprland | `hyprctl`; log: `$XDG_RUNTIME_DIR/hypr/<instance>/hyprland.log` |
| Crash | `coredumpctl`; crash report Hyprland: `~/.cache/hyprland/hyprlandCrashReport<PID>.txt` |
| Unit gagal | `systemctl --user --failed` |
| Kontrak IPC | JSON Schema dibangkitkan dari struct di `either-ipc`; snapshot-nya di folder tes crate |

## 6. Menjaga sesi harian tetap aman

Mesin development biasanya juga desktop harian, jadi uji di salinan sementara:

- jalankan `eitherd` dan shell dev dengan `EITHER_RUNTIME_DIR` terpisah, jangan memakai socket sesi asli;
- arahkan `XDG_CONFIG_HOME` dan `XDG_STATE_HOME` ke folder sementara (`mktemp -d`); jangan menimpa `~/.config/hypr`, `~/.config/either`, atau `~/.local/state/either`;
- jangan menguji lock screen di sesi yang sedang dipakai; gagal membuka kunci mengunci pengguna di luar sesinya. Pakai VM atau Hyprland bersarang;
- kembalikan state yang diubah saat uji (tema, DND, profil daya).

## 7. Berkontribusi

Baca [CONTRIBUTING.md](../../CONTRIBUTING.md), lalu ambil tugas dari backlog rilis berjalan.
