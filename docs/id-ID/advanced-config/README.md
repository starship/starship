# Konfigurasi Lanjutan

Meskipun Starship tergolong sebagai shell yang serbaguna, terkadang kita butuh upaya yang lebih dari sekadar mengedit `starship.toml` untuk membuatnya menjalankan beberapa hal tertentu. Halaman ini merincikan beberapa konfigurasi lebih lanjut yang digunakan oleh starship.

::: peringatan

Konfigurasi pada bagian ini dapat berubah saat pembaruan Starship rilis di kemudian hari.

:::

## Perintah pre-prompt dan pre-execution pada Bash

Bash tidak memiliki framework preexec/precmd yang baku seperti kebanyakan shell pada umumnya. Oleh karena itu, sulit halnya untuk membuat hook yang dapat dengan sepenuhnya disesuaikan dalam `bash`. Namun, Starship memberikan beberapa cara supaya kamu bisa memasukkan functions kamu sendiri ke dalam prosedur prompt-rendering:

- Untuk menampilkan fungsi yang dibuat tepat sebelum prompt, buatlah sebuah fungsi baru lalu berikan nama `starship_precmd_user_func` ke fungsi tersebut. Sebagai contoh, untuk menampilkan gambar roket sebelum prompt, kamu bisa melakukannya dengan cara

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Untuk menjalankan fungsi yang diatur tepat sebelum commands berjalan, anda dapat menggunakan [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Akan tetapi, anda **harus** melakukan trap pada DEBUG signal *sebelum* menginisiasikan Starship! Starship dapat menyimpan nilai dari DEBUG trap, tapi apabila trap ditimpa setelah starship berjalan, beberapa fungsionalitas akan rusak.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *sebelum* menjalankan starship
eval $(starship init bash)
```

## Mengubah Judul Window

Beberapa prompts shell akan mengubah dengan otomatis judul window-nya untukmu (mis. menampilan lokasi derektorimu yang sedang bekerja). Fish bahkan mengaturnya sebagai bawaan. Starship tidak, tapi mudah halnya untuk menambahkan fungsionalitas seperti ini ke dalam `bash` ataupun `zsh`.

Pertama, buatlah fungsi untuk mengubah judul window (bekerja pada bash dan zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; JUDUL_WINDOW_MU \007"
}
```

Kamu bisa menggunakan beberapa variabel untuk mengatur judul window-nya (`$USER`, `$HOSTNAME`, dan `$PWD` adalah pilihan yang paling banyak digemari).

Di dalam `bash`, aturlah function ini sebagai function precmd untuk starship:

```bash
starship_precmd_user_func="set_win_title"
```

Untuk `zsh`, tambahkan `precmd_functions` ke dalam array:

```bash
precmd_functions+=(set_win_title)
```

Kalau kamu suka dengan hasilnya, tambahkan baris (`~/.bashrc` or `~/.zshrc`) ke dalam file konfigurasi shell-mu untuk membuat hasilnya jadi permanen.

Sebagai contoh, kalau kamu mau menampilkan lokasi direktori terkinimu pada judul label terminal, tambahkan snipper berikut ke dalam `~/.bashrc` atau `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

## Penataan Strings

Penataan strings merupakan kumpulan kata, dipisahkan oleh whistespace. Sifat kata pada string bukanlah case sensitive (mis. `tebal` dan `TeBaL` dianggap sebagai string yang sama). Kata-kata yang dapat digunakan adalah salah satu dari daftar berikut:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

yang mana `<color>` adalah spesifikasi warna (dibahas di bawah). `fg:<color>` dan `<color>` untuk sementara memiliki fungsi yang sama, namun hal ini dapat berubah di kemudian hari. `inverted` menukar warna pada background dan foreground. Urutan kata pada string tidaklah menjadi masalah.

Token `none` mampu melakukan overrides pada token lainnya di dalam string jika Ia tidak termaksud ke dalam specifier `bg:` jadi, misalnya, `fg:red none fg:blue` akan tetap menghasilkan string tanpa ada penataan apapun. `bg:none` mengatur warna background menjadi default. Jadi, `fg:red bg:none` sama dengan `red` ataupun `fg:red` dan `bg:green fg:red bg:none` juga sama dengan `fg:red` ataupun `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
 - A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - A number between 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
