# Konfigurasi Lanjutan

Walaupun Starship adalah shell yang serbaguna, terkadang kita butuh upaya lebih dari sekadar mengedit `starship.toml` untuk membuatnya menjalankan beberapa hal tertentu. Halaman ini merincikan beberapa teknik konfigurasi lanjutan yang digunakan starship.

::: warning

Konfigurasi pada bagian ini dapat berubah saat Starship terbaru rilis di kemudian hari nanti.

:::

## TransientPrompt in PowerShell

It is possible to replace the previous-printed prompt with a custom string. This is useful in cases where all the prompt information is not always needed. To enable this, run `Enable-TransientPrompt` in the shell session. To make it permanent, put this statement in your `$PROFILE`. Transience can be disabled on-the-fly with `Disable-TransientPrompt`.

By default, the left side of input gets replaced with `>`. To customize this, define a new function called `Invoke-Starship-TransientFunction`. For example, to display Starship's `character` module here, you would do

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt and TransientRightPrompt in Cmd

Clink allows you to replace the previous-printed prompt with custom strings. This is useful in cases where all the prompt information is not always needed. To enable this, run `clink set prompt.transient <value>` where \<value\> can be one of:

- `always`: always replace the previous prompt
- `same_dir`: replace the previous prompt only if the working directory is same
- `off`: do not replace the prompt (i.e. turn off transience)

You need to do this only once. Make the following changes to your `starship.lua` to customize what gets displayed on the left and on the right:

- By default, the left side of input gets replaced with `>`. To customize this, define a new function called `starship_transient_prompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display Starship's `character` module here, you would do

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display the time at which the last command was started here, you would do

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt and TransientRightPrompt in Fish

It is possible to replace the previous-printed prompt with a custom string. This is useful in cases where all the prompt information is not always needed. To enable this, run `enable_transience` in the shell session. To make it permanent, put this statement in your `~/.config/fish/config.fish`. Transience can be disabled on-the-fly with `disable_transience`.

Note that in case of Fish, the transient prompt is only printed if the commandline is non-empty, and syntactically correct.

- By default, the left side of input gets replaced with a bold-green `❯`. To customize this, define a new function called `starship_transient_prompt_func`. For example, to display Starship's `character` module here, you would do

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. For example, to display the time at which the last command was started here, you would do

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## Kustomisasi Perintah pre-prompt dan pre-execution Pada Cmd

Clink menyediakan APIs yang sangat fleksibel untuk menjalankan perintah pre-prompt dan pre-exec di Cmd shell. Caranya sangat mudah dengan Starship. Ubahlah file `starship.lua` sesuai kebutuhanmu:

- Untuk menjalankan custom function sebelum prompt muncul, definisikan function baru dengan nama `starship_preprompt_user_func`. Function ini menerima prompt yang berjalan sebagai string yang mampu kamu gunakan. Sebagai contoh, untuk menampilkan roket sebelum prompt, kamu bisa

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- Untuk menjalankan custom function tepat sebelum perintah dieksekusi, definisikan function baru dengan nama `starship_precmd_user_func`. Function ini menerima prompt yang berjalan sebagai string yang mampu kamu gunakan. Sebagai contoh, untuk menampilkan perintah yang akan dieksekusi, kamu bisa

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Kustomisasi Perintah pre-prompt dan pre-execution Pada Bash

Bash tidak memiliki framework preexec/precmd yang tetap seperti kebanyakan shell pada umumnya. Oleh karena itu, sulit halnya untuk membuat hook yang dapat dikustomisasi sepenuhnya di dalam `bash`. Namun, Starship memberimu sedikit kemampuan untuk bisa menambahkan function milikmu ke dalam prosedur prompt-rendering:

- Untuk menjalankan custom function tepat sebelum prompt, buatlah sebuah fungsi baru lalu berikan nama `starship_precmd_user_func` ke fungsi tersebut. Sebagai contoh, untuk menampilkan roket sebelum prompt, kamu bisa

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Untuk menjalankan custom function tepat sebelum perintah berjalan, kamu bisa menggunakan [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). Akan tetapi, kamu **harus** melakukan proses trap pada DEBUG signal _sebelum_ menjalankan Starship! Starship bisa menyimpan nilai dari DEBUG trap, tapi jika trap diganti setelah starship berjalan, beberapa fungsi akan rusak.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Perintah Custom pre-promt dan pre-execution di PowerShell

PowerShell tidak memiliki framework preecex/precmd seperti kebanyak shells pada umumnya. Karena itu, sulit halnya untuk membuat hook yang dapat dikustomisasi sepenuhnya di dalam `powershell`. Namun, Starship memberimu sedikit kemampuan untuk bisa menambahkan function milikmu ke dalam prosedur prompt-rendering:

Buatlah sebuah funciton dengan nama `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Mengubah Judul Window

Beberapa prompt shell dengan otomatis akan mengubah judul window-nya untukmu (mis. untuk merefleksikan direktori kerjamu). Fish bahkan mengaturnya sebagai bawaan. Di dalam Starship tidak bisa, namun mudah halnya untuk menambahkan fungsionalitas tersebut ke dalam `bash`, `zsh`, `cmd` ataupun `powershell`.

Pertama, buatlah function untuk mengubah judul window (pada bash dan zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; JUDUL_WINDOW_MU \007"
}
```

Kamu bisa menggunakan variabel untuk mengkustomisasi judulnya (`$USER`, `$HOSTNAME`, dan `$PWD` adalah opsi yang populer).

Di dalam `bash`, atur function berikut menjadi function precmd untuk starship:

```bash
starship_precmd_user_func="set_win_title"
```

Dalam `zsh`, pada array `precmd_functions`, tambahkan:

```bash
precmd_functions+=(set_win_title)
```

Kalau kamu suka dengan hasilnya, tambahkan baris (`~/.bashrc` or `~/.zshrc`) ke dalam file konfigurasi shell milikmu untuk membuatnya menjadi tetap.

Sebagai contoh, kalau kamu mau menampilkan lokasi direktori pada judul label terminalmu, tambahkan bagian berikut ke dalam `~/.bashrc` atau `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

Untuk Cmd, kamu dapat mengubah judul window-mu dengan menggunakan function `starship_preprompt_user_func`.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

Kamu juga dapat mengatur keluaran yang sama dengan PowerShell dengan membuat sebuah function bernama `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (&starship init powershell)
```

## Mengaktifkan Right Prompt

Sebagian shells mendukung right prompt yang mana dirender di baris yang sama sesuai dengan masukannya. Starship mampu mengatur konten right prompt dengan menggunakan opsi `right_format`. Semua modul yang bisa digunakan di dalam `format` juga dapat digunakan di dalam `right_format`. Variabel `$all` hanya akan memuat modul yang tidak digunakan secara eksplisit di dalam `format` ataupun `right_format`.

Catatan: Right propmt merupakan sebuah baris yang mengikuti lokasi baris inputan. Untuk membuat modul rata ke kanan di atas baris masukan di dalam multi-line prompt, lihat [`fill` module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell.

Note: Nushell 0.71.0 or later is required

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
starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## Prompt Berkelanjutan

Beberapa shells mendukung continuation prompt bersamaan dengan prompt biasa. Prompt tersebutlah yang akan dirender daripada prompt biasa ketika pengguna memasukkan perintah yang kurang lengkap (seperti tanda kurung atau tanda kutipan tunggal).

Starship dapat mengatur continuation prompt dengan opsi `continuation_prompt`. Prompt bawaannya adalah `"[∙](bright-black) "`.

Catatan: `continuation_prompt` harus diubah menjadi string literal tanpa variabel apapun.

Catatan: Continuation prompts hanya tersedia pada beberapa shells berikut:

- `bash`
- `zsh`
- `PowerShell`

### Contoh

```toml
# ~/.config/starship.toml

# Continuation prompt yang menampilkan dua panah solid
continuation_prompt = "▶▶"
```

## Penataan String

Penataan string adalah kumpulan kata-kata, yang dipisahkan oleh ruang kosong. Kumpulannya tidak bersifat case sensitive (mis. `tebal` dan `TeBaL` dianggap sebagai string yang sama). Tiap-tiap kata berikut adalah opsinya:

- `bold`
- `italic`
- `underline`
- `dimmed`
- `inverted`
- `blink`
- `hidden`
- `strikethrough`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

yang mana `<color>` merupakan sebuah penentu warna (dibahas di bawah). `fg:<color>` dan `<color>` untuk saat ini memiliki fungsi yang sama, meskipun bisa berubah di kemudian hari. `inverted` menggantikan warna pada latar depan dan belakang. Urutan kata pada string tidak jadi masalah.

`none` bisa menimpa nilai token lainnya di dalam string jika Ia tidak termaksud dalam penentu warna pada `bg:` sebagai contoh, `fg:red none fg:blue` akan tetap menjadi string yang tidak memiliki penataan. `bg:none` menjadikan warna pada latar belakang sebagai warna bawaan. Jadi, nilai `fg:red bg:none` sama dengan `red` atau `fg:red` dan nilai `bg:green fg:red bg:none` juga sama dengan `fg:red` ataupun `red`. Mungkin akan jadi masalah untuk menggunakan `none` dengan token lainnya di kemudian hari.

Penentuan warna bisa dilakukan dengan salah satu cara berikut:

- Warna terminal pada umumnya terdiri dari: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Secar opsional kamu bisa menambahkannya dengan `bright-` untuk mendapatkan versi yang lebih terang (mis. `bright-white`).
- Menuliskannya dengan menggunakan `#` dan diikuti oleh enam digit angka hexadesimal. Spesifikasi [kode heksadesimal pada warna RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Menggunakan bilangan antara 0-255. Spesifikasi [8-bit Kode Warna ANSI](https://i.stack.imgur.com/KTSQa.png).

Jika warna yang dipakai pada latar depan/latar belakang banyak, maka warna yang terbaru pada string yang akan diprioritaskan.

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default
- `hidden` is [not supported on iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app
