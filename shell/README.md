# shell

Konfigurasi Quickshell (QML). Subfolder baseline: `components/` (library komponen), `modules/` (bar, launcher, notifikasi, lock, OSD), `services/` (singleton QML: client IPC dan pembungkus API Quickshell), `theme/` (`Tokens.qml` hasil generate, jangan diedit tangan).

Isi pertama: bar minimal di 0.1.0. Semua nilai visual dari token; `just qml-check` menegakkannya (`theme/` dikecualikan). API Quickshell hanya dipanggil dari `services/`. Pemecahan menjadi modul bersama `either.ui`, `either.theme`, `either.ipc` menunggu ADR (U18).
