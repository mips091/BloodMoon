# BloodMoon di Linux dengan Horizon Launcher resmi

Panduan ini memakai **Horizon-Bedrock sebagai basis BloodMoon** dan launcher resmi Horizon sebagai pengelola versi, akun, serta file game. Jangan jalankan `horizon-client` hasil build secara langsung untuk pemakaian normal: daftarkan build itu ke launcher terlebih dahulu.

## Yang dibutuhkan

- CachyOS / Arch Linux 64-bit.
- Git dan Rust melalui rustup.
- Paket build Linux yang dibutuhkan workspace. Di Arch/CachyOS, mulai dengan:
  `sudo pacman -S --needed base-devel git pkgconf alsa-lib systemd-libs`
- Horizon Launcher resmi sudah terpasang.
- Di Horizon Launcher resmi, install dan **siapkan** satu versi Nightly/release 26.3 sampai statusnya siap dimainkan.
- Akun Microsoft yang memiliki Minecraft: Java Edition.

## 1. Build BloodMoon

Dari terminal:

```sh
git clone https://github.com/mips091/BloodMoon.git
cd BloodMoon
cargo build --release -p horizon-client
```

File hasil build:

```
target/release/horizon-client
```

Jika kamu sudah punya clone BloodMoon, masuk ke folder itu dan jalankan dua perintah build terakhir saja.

## 2. Impor build ke Horizon Launcher resmi

Pastikan launcher resmi sudah memiliki versi Horizon 26.3 yang terpasang dan **prepared**. Pilih versi resmi tersebut sebagai versi aktif/terpilih.

Lalu jalankan:

```sh
horizon-launcher import-build "$PWD/target/release/horizon-client" --trust-unsigned
```

Jika perintah `horizon-launcher` tidak ditemukan, buka Horizon Launcher resmi dan gunakan **Versions → Import a game build…**, lalu pilih file absolut `target/release/horizon-client`. Setujui peringatan build unsigned karena itu adalah build BloodMoon yang kamu kompilasi sendiri.

Launcher akan membuat versi `imported-...` dari versi resmi yang sedang dipilih. Folder versi itu membawa file game dan resource dari versi dasar; executable game-nya diganti dengan build BloodMoon. Jadi kamu tidak perlu menyalin isi folder `.local` secara manual.

Jika versi dasar belum prepared, siapkan dahulu melalui launcher resmi. CLI-nya:

```sh
horizon-launcher prepare
```

## 3. Pasang Actions & Stuff (.mcpack)

Salin file `.mcpack` milikmu ke folder `addons` pada versi `imported-...` yang baru dibuat. Lokasi data launcher biasanya berada di:

```
~/.local/share/horizon-launcher/versions/
```

Cari folder bernama `imported-...`, lalu buat folder `addons` jika belum ada. Jangan letakkan pack di folder versi Nightly dasar karena pack harus berada di versi hasil impor.

Buka Horizon Launcher resmi, pastikan versi `Imported` terpilih, lalu tekan **Play**. Di dalam game buka **Options → Add-ons** dan aktifkan pack.

## 4. Memperbarui BloodMoon

Setelah mengubah source code:

```sh
cargo build --release -p horizon-client
horizon-launcher import-build "$PWD/target/release/horizon-client" --trust-unsigned
```

Impor ulang akan membuat versi imported untuk build baru. Pilih versi imported terbaru di launcher. Jangan menghapus versi imported lama sampai build baru berhasil dijalankan.

## Catatan penting

- Jangan commit file game Minecraft, resource hasil preparation, akun, atau pack berhak cipta ke repositori.
- Jangan menjalankan binary langsung sebagai cara bermain sehari-hari; gunakan launcher agar versi dasar, resource, dan sesi akun dikelola dengan benar.
- Jika popup masih muncul saat menekan **Play** dari launcher resmi, catat teks popup persis dan lampirkan `logs/latest.log` dari folder versi imported. Jangan hapus folder versi dasar atau data launcher untuk mencoba memperbaikinya.
