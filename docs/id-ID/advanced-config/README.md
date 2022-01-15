# Konfigurasi Lanjutan

Meskipun Starship tergolong sebagai shell yang serbaguna, terkadang kita butuh upaya yang lebih dari sekadar mengedit `starship.toml` untuk membuatnya menjalankan beberapa hal tertentu. Halaman ini merincikan beberapa teknik konfigurasi lanjutan yang digunakan oleh starship.

::: peringatan

Konfigurasi pada bagian ini dapat berubah saat pembaruan Starship rilis di kemudian hari nanti.

:::

## Kustomisasi Perintah pre-prompt dan pre-execution Pada Bash

Bash tidak memiliki framework preexec/precmd yang tetap seperti kebanyakan shell pada umumnya. Oleh karena itu, sulit halnya untuk membuat hook yang dapat dikustomisasi sepenuhnya di dalam `bash`. Namun, Starship memberikan beberapa cara supaya kamu bisa memasukkan fungsimu sendiri ke dalam prosedur prompt-rendering:

- Untuk menjalankan fungsi yang dikustomisasi tepat sebelum prompt, buatlah sebuah fungsi baru lalu berikan nama `starship_precmd_user_func` ke fungsi tersebut. Sebagai contoh, untuk menampilkan gambar roket sebelum prompt, kamu bisa melakukannya dengan cara

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Untuk menjalankan fungsi yang dikustomisasi tepat sebelum commands berjalan, kamu bisa menggunakan [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Akan tetapi, kamu **harus** melakukan proses trap pada DEBUG signal *sebelum* menjalankan Starship! Starship bisa menyimpan nilai dari DEBUG trap, tapi jika trap diganti setelah starship berjalan, beberapa fungsi akan rusak.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *sebelum* menjalankan starship
eval $(starship init bash)
```

## Custom pre-prompt and pre-execution Commands in PowerShell

PowerShell does not have a formal preexec/precmd framework like most other shells. Because of this, it is difficult to provide fully customizable hooks in `powershell`. Namun, Starship memberikan beberapa cara supaya kamu bisa memasukkan fungsimu sendiri ke dalam prosedur prompt-rendering:

Create a function named `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Mengubah Judul Window

Beberapa prompt shell dengan otomatis akan mengubah judul window-nya untukmu (mis. untuk merefleksikan direktori kerjamu). Fish bahkan mengaturnya sebagai bawaan. Starship tidak, tapi mudah saja untuk menambahkan fungsi tersebut ke dalam `bash` ataupun `zsh`.

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

Kalau kamu suka hasilnya, tambahkan baris (`~/.bashrc` or `~/.zshrc`) ke dalam file konfigurasi shell milikmu untuk membuatnya permanen.

Sebagai contoh, kalau kamu mau menampilkan lokasi direktori pada judul label terminalmu, tambahkan bagian berikut ke dalam `~/.bashrc` atau `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

You can also set a similar output with PowerShell by creating a function named `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (&starship init powershell)
```

## Enable Right Prompt

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` saat ini hanya tersedia pada shell: elvish, fish, zsh.

### Contoh

```toml
# ~/.config/starship.toml

# A minimal left prompt
format = """$character"""

# move the rest of the prompt to the right
right_format = """$all"""
```

Menghasilkan prompt seperti berikut:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```


## Menata String

Style strings are a list of words, separated by whitespace. Kumpulan katanya tidak bersifat case sensitive (mis. `tebal` dan `TeBaL` dianggap sebagai string yang sama). Tiap-tiap kata berikut adalah opsinya:

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
