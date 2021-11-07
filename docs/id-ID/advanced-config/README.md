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

## Change Window Title

Some shell prompts will automatically change the window title for you (e.g. to reflect your working directory). Fish even does it by default. Starship does not do this, but it's fairly straightforward to add this functionality to `bash` or `zsh`.

First, define a window title change function (identical in bash and zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

You can use variables to customize this title (`$USER`, `$HOSTNAME`, and `$PWD` are popular choices).

In `bash`, set this function to be the precmd starship function:

```bash
starship_precmd_user_func="set_win_title"
```

In `zsh`, add this to the `precmd_functions` array:

```bash
precmd_functions+=(set_win_title)
```

If you like the result, add these lines to your shell configuration file (`~/.bashrc` or `~/.zshrc`) to make it permanent.

For example, if you want to display your current directory in your terminal tab title, add the following snippet to your `~/.bashrc` or `~/.zshrc`:

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

`right_format` is currently supported for the following shells: elvish, fish, zsh.

### Contoh

```toml
# ~/.config/starship.toml

# A minimal left prompt
format = """$character"""

# move the rest of the prompt to the right
right_format = """$all"""
```

Produces a prompt like the following:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```


## Menata String

Style strings are a list of words, separated by whitespace. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - Warna terminal pada umumnya terdiri dari: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Untuk memperoleh varian warna yang lebih cerah, kamu dapat menggunakan token `bright-` (mis. `bright-white`).
 - Menuliskannya dengan menggunakan `#` dan diikuti oleh enam digit angka hexadesimal. Spesifikasi [kode heksadesimal pada warna RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Menggunakan bilangan antara 0-255. Spesifikasi [8-bit Kode Warna ANSI](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
