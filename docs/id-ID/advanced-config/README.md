# Konfigurasi Lanjutan

Meskipun Starship tergolong sebagai shell yang serbaguna, terkadang kita butuh upaya yang lebih dari sekadar mengedit `starship.toml` untuk membuatnya menjalankan beberapa hal tertentu. Halaman ini merincikan beberapa teknik konfigurasi lanjutan yang digunakan oleh starship.

::: peringatan

Konfigurasi pada bagian ini dapat berubah saat pembaruan Starship rilis di kemudian hari nanti.

:::

## Kostumisasi Perintah pre-prompt dan pre-execution Pada Bash

Bash tidak memiliki framework preexec/precmd yang tetap seperti kebanyakan shell pada umumnya. Oleh karena itu, sulit halnya untuk membuat hook yang dapat dikustomisasi sepenuhnya di dalam `bash`. Namun, Starship memberikan beberapa cara supaya kamu bisa memasukkan fungsimu sendiri ke dalam prosedur prompt-rendering:

- Untuk menjalankan fungsi yang dikustomisasi tepat sebelum prompt, buatlah sebuah fungsi baru lalu berikan nama `starship_precmd_user_func` ke fungsi tersebut. Sebagai contoh, untuk menampilkan gambar roket sebelum prompt, kamu bisa melakukannya dengan cara

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Untuk menjalankan fungsi buatan tepat sebelum commands berjalan, kamu bisa menggunakan [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Akan tetapi, kamu **harus** melakukan proses trap pada DEBUG signal *sebelum* menjalankan Starship! Starship bisa menyimpan nilai dari DEBUG trap, tapi jika trap diganti setelah starship berjalan, beberapa fungsi akan rusak.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *sebelum* menjalankan starship
eval $(starship init bash)
```

## Mengubah Judul Window

Beberapa prompts shell dengan otomatis akan mengubah judul window-nya untukmu (mis. untuk merefleksikan direktori kerjamu). Fish bahkan mengaturnya sebagai bawaan. Starship tidak, tapi mudah saja untuk menambahkan fungsi tersebut ke dalam `bash` ataupun `zsh`.

Pertama, buatlah fungsi untuk mengubah judul window (bekerja pada bash dan zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; JUDUL_WINDOW_MU \007"
}
```

Kamu bisa menggunakan variabel untuk mengkustomisasi judulnya (`$USER`, `$HOSTNAME`, dan `$PWD` adalah opsi yang populer).

Di dalam `bash`, atur fungsi berikut menjadi fungsi precmd untuk starship:

```bash
starship_precmd_user_func="set_win_title"
```

Dalam `zsh`, pada array `precmd_functions`, tambahkan:

```bash
precmd_functions+=(set_win_title)
```

Kalau kamu suka hasilnya, tambahkan baris (`~/.bashrc` or `~/.zshrc`) ke dalam file konfigurasi shell-mu untuk membuatnya permanen.

Sebagai contoh, kalau kamu mau menampilkan lokasi direktori pada judul label terminalmu, tambahkan snippet berikut ke dalam `~/.bashrc` atau `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

## Menata String

Tataan pada strings merupakan sebuah kumpulan kata yang dipisahkan dengan spasi kosong. Kata-katanya tidak bersifat case sensitive (mis. `tebal` dan `TeBaL` dianggap sebagai string yang sama). Tiap-tiap kata berikut adalah opsinya:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

yang mana `<color>` merupakan sebuah penentu warna (dibahas di bawah). `fg:<color>` dan `<color>` untuk saat ini memiliki fungsi yang sama, meskipun bisa berubah di kemudian hari. `inverted` menggantikan warna pada latar depan dan belakang. Urutan kata pada string tidak jadi masalah.

`none` bisa menimpa nilai token lainnya di dalam string jika Ia tidak termaksud dalam penentu warna pada `bg:` sebagai contoh, `fg:red none fg:blue` akan tetap menjadi string yang tidak memiliki penataan. `bg:none` menjadikan warna pada latar belakang sebagai warna bawaan. Jadi, nilai `fg:red bg:none` sama dengan `red` atau `fg:red` dan nilai `bg:green fg:red bg:none` juga sama dengan `fg:red` ataupun `red`. Mungkin akan jadi masalah untuk menggunakan `none` dengan token lainnya di kemudian hari.

Penentuan warna bisa dilakukan dengan salah satu cara berikut:

 - Warna terminal pada umumnya terdiri dari: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Untuk memperoleh varian warna yang lebih cerah, kamu dapat menggunakan token `bright-` (mis. `bright-white`).
 - Menuliskannya dengan menggunakan `#` dan diikuti oleh enam digit angka hexadesimal. Spesifikasi [kode heksadesimal pada warna RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Menggunakan bilangan antara 0-255. Spesifikasi [8-bit Kode Warna ANSI](https://i.stack.imgur.com/KTSQa.png).

Jika warna yang dipakai pada latar depan/latar belakang banyak, maka warna yang terbaru pada string yang akan diprioritaskan.
