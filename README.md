# omar
Boilerplate Rust untuk API Server dengan database MySQL

Ini adalah contoh aplikasi menggunakan bahasa pemrograman Rust. Sesuai deskripsinya, aplikasi ini adalah backend sebuah aplikasi web. Ia melakukan fungsi-fungsi berikut:

- Mengambil data dari database MySQL
- Mengirimkan data via HTTP sebagai file JSON
- Berjalan sebagai server di Port 3000

Aplikasi ini saya buat untuk teman-teman yang ingin mempelajari Rust. Meskipun kekuatan Rust bukanlah sebagai backend web service, tapi saya rasa kasus sederhana seperti ini akan membantu mempelajari Rust lebih lanjut.

TODO:
Melengkapi dokumentasi

## Konfigurasi Awal

Secara default, aplikasi ini menggunakan konfigurasi sebagai berikut:

- MySQL
  + host: localhost
  + port: 8889
  + username: root
  + password: root
  + database: omar_db
  + table: produk
- Server:
  + host: localhost
  + port: 3000

Sebelum menjalankan, pastikan bahwa MySQL telah diatur untuk berjalan mengikuti konfigurasi di atas

## Menjalankan Aplikasi

1. Download rust di [rust-lang.org](http://www.rust-lang.org)
2. Download/clone omar dari repo ini
3. Buka terminal, lalu pergi ke direktori tempat omar didownload
4. Jalankan perintah `cargo run`
5. Aplikasi akan berjalan di `localhost:3000`
