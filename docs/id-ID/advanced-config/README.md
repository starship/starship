# Konfigurasi Lanjutan

Meskipun Starship tergolong sebagai shell yang serbaguna, terkadang kita butuh upaya yang lebih dari sekadar mengedit `starship.toml` untuk membuatnya menjalankan beberapa hal tertentu. Halaman ini merincikan beberapa teknik konfigurasi lanjutan yang digunakan oleh starship.

::: peringatan

Konfigurasi pada bagian ini dapat berubah saat pembaruan Starship rilis di kemudian hari.

:::

## Perintah pre-prompt dan pre-execution pada Bash

Bash tidak memiliki framework preexec/precmd yang baku seperti kebanyakan shell pada umumnya. Oleh karena itu, sulit halnya untuk membuat hook dalam `bash` yang dapat dengan mudah disesuaikan sesuka hati. Namun, Starship memberikan beberapa cara supaya kamu bisa memasukkan functions kamu sendiri ke dalam prosedur prompt-rendering:

- Untuk menampilkan fungsi buatan untuk berjalan tepat sebelum prompt, buatlah sebuah fungsi baru lalu berikan nama `starship_precmd_user_func` ke fungsi tersebut. Sebagai contoh, untuk menampilkan gambar roket sebelum prompt, kamu bisa melakukannya dengan cara

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Untuk menjalankan fungsi buatan tepat sebelum commands berjalan, kamu bisa menggunakan [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Akan tetapi, kamu **harus** melakukan proses trap pada DEBUG signal *sebelum* menginisiasikan Starship! Starship bisa menyimpan nilai dari DEBUG trap, tapi jika trap ditimpa setelah starship berjalan, beberapa fungsionalitas bisa rusak.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *sebelum* menjalankan starship
eval $(starship init bash)
```

## Mengubah Judul Window

Ada beberapa prompts shell yang dengan otomatis akan mengubah judul window-nya untukmu (mis. menampilan lokasi derektorimu yang sedang bekerja). Fish bahkan menjadikannya sebagai aturan bawaan. Starship tidak, tapi mudah halnya untuk menambahkan fungsionalitas seperti ini ke dalam `bash` ataupun `zsh`.

Pertama, buatlah fungsi untuk mengubah judul window (bekerja pada bash dan zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; JUDUL_WINDOW_MU \007"
}
```

Kamu bisa menggunakan beberapa variabel untuk mengatur judul window-nya (`$USER`, `$HOSTNAME`, dan `$PWD` adalah pilihan yang paling banyak digemari).

Di dalam `bash`, buatlah function berikut sebagai function precmd untuk starship:

```bash
starship_precmd_user_func="set_win_title"
```

Untuk `zsh`, tambahkan `precmd_functions` ke dalam array:

```bash
precmd_functions+=(set_win_title)
```

Kalau kamu suka dengan hasilnya, tambahkan baris (`~/.bashrc` or `~/.zshrc`) ke dalam file konfigurasi shell-mu untuk membuatnya permanen.

Sebagai contoh, kalau kamu mau menampilkan lokasi direktori terkinimu pada judul label terminal, tambahkan snippet berikut ke dalam `~/.bashrc` atau `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

## Penataan Strings

Penataan strings merupakan kumpulan kata, dan dipisahkan oleh whistespace. Sifat kata pada string bukanlah case sensitive (mis. `tebal` dan `TeBaL` dianggap sebagai string yang sama). Kata-kata yang dapat digunakan adalah salah satu dari daftar berikut:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

yang mana `<color>` adalah spesifikasi warna (dibahas di bawah). `fg:<color>` dan `<color>` untuk sementara memiliki fungsi yang sama, namun hal ini bisa saja berubah di kemudian hari. `inverted` menukar warna pada background dan foreground. Urutan kata pada string tidaklah menjadi masalah.

Token `none` mampu melakukan overrides pada token lainnya di dalam string jika Ia tidak termaksud ke dalam specifier `bg:` Jadi, sebagai contoh, `fg:red none fg:blue` akan tetap menghasilkan string namun tanpa penataan apapun. `bg:none` mengatur warna background menjadi default. Jadi, `fg:red bg:none` sama dengan `red` ataupun `fg:red` dan `bg:green fg:red bg:none` juga sama dengan `fg:red` ataupun `red`. Penggunakan `none` bersama dengan token lainnya bisa saja menjadi masalah di kemudian hari.

Berikut ini adalah contoh specifier warna:

 - Beberapa warna terminal pada umumnya terdiri dari: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Untuk memperoleh varian warna yang lebih cerah, kamu dapat menggunakan `bright-` (mis. `bright-white`).
 - Dituliskan dengan menggunakan `#` dan diikuti oleh enam digit angka hexadesimal. Tautan ini menspesifikasikan [warna code hexadesimal](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Bilangan antara 0-255. Tautan ini menspesifikasikan [8-bit Kode Warna ANSI](https://i.stack.imgur.com/KTSQa.png).

Jika beberapa warna latar depan/latar belakang diberikan, yang terbaru dalam string akan didahulukan.
