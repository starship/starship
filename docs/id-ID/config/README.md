# Konfigurasi

Untuk memulai mengkonfigurasi starship, buatlah file berikut: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Seluruh konfigurasi starship dilakukan dalam file [TOML](https://github.com/toml-lang/toml) berikut:

```toml
# Inserts a blank line between shell prompts
add_newline = true

# Replace the "❯" symbol in the prompt with "➜"
[character] # The name of the module we are configuring is "character"
success_symbol = "[➜](bold green)" # The "success_symbol" segment is being set to "➜" with the color "bold green"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

Kamu bisa mengganti lokasi file konfigurasi bawaan dengan menggunakan environment variable dari `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

Ekuivalen dalam PowerShell (Windows), tambahkan baris berikut pada `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Logging

By default starship logs warnings and errors into a file named `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, where the session key is corresponding to a instance of your terminal. This, however can be changed using the `STARSHIP_CACHE` environment variable:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Ekuivalen dalam PowerShell (Windows), tambahkan baris berikut pada `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Terminologi

**Module**: A component in the prompt giving information based on contextual information from your OS. For example, the "nodejs" module shows the version of Node.js that is currently installed on your computer, if your current directory is a Node.js project.

**Variable**: Smaller sub-components that contain information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of Node.js.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Format String

Format strings are the format that a module prints all its variables with. Most modules have an entry called `format` that configures the display format of the module. You can use texts, variables and text groups in a format string.

#### Variabel

A variable contains a `$` symbol followed by the name of the variable. The name of a variable can only contain letters, numbers and `_`.

For example:

- `$version` adalah format string dengan sebuah nama variabel `version`.
- `$git_branch$git_commit` merupakan sebuah format string dengan dua variabel bernama `git_branch` dan `git_commit`.
- `$git_branch $git_commit` memiliki dua variabel yang dipisahkan dengan sebuah spasi.

#### Grup Teks

A text group is made up of two different parts.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings). You can add texts, variables, or even nested text groups in it.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). This can be used to style the first part.

For example:

- `[on](red bold)` akan menampilkan string `on` dengan teks merah tebal.
- `[⌘ $version](bold green)` akan menampilkan simbol `⌘` yang diikuti oleh variabel yang berisikan `version`, dengan teks tebal berwarna hijau.
- `[a [b](red) c](green)` akan menampilkan `a b c` dengan `b` merah, dan `a` & `c` green.

#### Menata String

Most modules in starship allow you to configure their display styles. This is done with an entry (usually called `style`) which is a string specifying the configuration. Here are some examples of style strings along with what they do. For details on the full syntax, consult the [advanced config guide](/advanced-config/).

- `"fg:green bg:blue"` mengeset teks berwana hijau pada latar biru
- `"bg:blue fg:bright-green"` mengeset teks hijau terang pada latar biru
- `"bold fg:27"` mengeset tebal teks dengan [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` mengeset teks bergaris bawah pada latar oranye menyala
- `"bold italic fg:purple"` mengeset teks ungu miring tebal
- `""` secara eksplisit menonaktifkan semua penataan gaya

Note that what styling looks like will be controlled by your terminal emulator. For example, some terminal emulators will brighten the colors instead of bolding text, and some color themes use the same values for the normal and bright colors. Also, to get italic text, your terminal must support italics.

#### Format String Bersyarat (Conditional)

A conditional format string wrapped in `(` and `)` will not render if all variables inside are empty.

For example:

- `(@$region)` tidak akan menampilkan apapun jika nilai variabel `region` adalah `None` atau berupa string kosong, jika tidak, `@` diikuti dengan nilai dari region.
- `(sembarang)` akan selalu tidak menampilkan apapun karena tidak ada variabel yang dibungkus dalam kurung kurawal.
- Tatkala `$all` digunakan sebagai shortcut untuk `\[$a$b\]`, `($all)` tidak akan menampilkan apapun jika nilai `$a` dan `$b` adalah `None`. Berlaku juga dengan `(\[$a$b\] )`.

#### Special characters

The following symbols have special usage in a format string and must be escaped: `$ \ [ ] ( )`.

Note that TOML has [both basic strings and literal strings](https://toml.io/en/v1.0.0#string). It is recommended to use a literal string (surrounded by single quotes) in your config. If you want to use a basic string (surrounded by double quotes), you must escape the backslash itself (i.e. use `\\`).

For example, when you want to print a `$` symbol on a new line, the following configs for `format` are equivalent:

```toml
# with basic string
format = "\n\\$"

# with multiline basic string
format = """

\\$"""

# with literal string
format = '''

\$'''
```

## Prompt

This is the list of prompt-wide configuration options.

### Opsi

| Opsi              | Bawaan                         | Deskripsi                                                              |
| ----------------- | ------------------------------ | ---------------------------------------------------------------------- |
| `fromat`          | [link](#default-prompt-format) | Mengkonfigurasi format pada prompt.                                    |
| `right_format`    | `""`                           | See [Enable Right Prompt](/advanced-config/#enable-right-prompt)       |
| `scan_timeout`    | `30`                           | Batas waktu starpship untuk memindai file (dalam milidetik).           |
| `command_timeout` | `500`                          | Batas waktu untuk perintah yang dijalankan starship (dalam milidetik). |
| `add_newline`     | `true`                         | Memasukkan baris kosong antara prompt shell.                           |

### Contoh

```toml
# ~/.config/starship.toml

# Use custom format
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10

# Disable the blank line at the start of the prompt
add_newline = false
```

### Format Prompt Bawaan

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. The default is as shown:

```toml
format = "$all"

# Which is equivalent to
format = """
$username\
$hostname\
$localip\
$shlvl\
$singularity\
$kubernetes\
$directory\
$vcsh\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$cobol\
$container\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$kotlin\
$lua\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$pulumi\
$purescript\
$python\
$rlang\
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vlang\
$vagrant\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
$env_var\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$shell\
$character"""
```

If you just want to extend the default format, you can use `$all`; modules you explicitly add to the format will not be duplicated. Eg.

```toml
# Move the directory to the second line
format = "$all$directory$character"
```

## AWS

The `aws` module shows the current AWS region and profile. This is based on `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env var with `~/.aws/config` file. This module also shows an expiration timer when using temporary credentials.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var and the credentials expiration date is read from the `AWS_SESSION_EXPIRATION` env var.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile is read from the `AWSUME_PROFILE` env var and the credentials expiration date is read from the `AWSUME_EXPIRATION` env var.

### Opsi

| Opsi                | Bawaan                                                               | Deskripsi                                                        |
| ------------------- | -------------------------------------------------------------------- | ---------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\])]($style)'` | Format dari modul.                                               |
| `symbol`            | `"☁️ "`                                                              | Simbol yang digunakan sebelum menampilkan profil AWS terkini.    |
| `region_aliases`    |                                                                      | Tabel alias dari region yang ditampilan selain nama AWS.         |
| `style`             | `"bold yellow"`                                                      | Gaya penataan untuk modul.                                       |
| `expiration_symbol` | `X`                                                                  | Simbol ditampilkan ketika temporer kredensial telah kedaluwarsa. |
| `disabled`          | `false`                                                              | Menonaktifkan modul `AWS`.                                       |

### Variabel

| Variabel  | Contoh           | Deskripsi                         |
| --------- | ---------------- | --------------------------------- |
| region    | `ap-northeast-1` | Region AWS terkini                |
| profile   | `astronauts`     | Profil AWS terkini                |
| duration  | `2h27m20s`       | Durasi temporer kredensial        |
| symbol    |                  | Menyalin nilai dari opsi `symbol` |
| style\* |                  | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

#### Menampilkan semuanya

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Menampilkan region

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$region]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Menampilkan profil

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
```

## Azure

The `azure` module shows the current Azure Subscription. This is based on showing the name of the default subscription, as defined in the `~/.azure/azureProfile.json` file.

### Opsi

| Variabel   | Bawaan                                   | Deskripsi                                  |
| ---------- | ---------------------------------------- | ------------------------------------------ |
| `format`   | `"on [$symbol($subscription)]($style) "` | The format for the Azure module to render. |
| `symbol`   | `"ﴃ "`                                   | The symbol used in the format.             |
| `style`    | `"blue bold"`                            | The style used in the format.              |
| `disabled` | `true`                                   | Disables the `azure` module.               |

### Contoh

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($subscription)]($style) "
symbol = "ﴃ "
style = "blue bold"
```

## Baterai

The `battery` module shows how charged the device's battery is and its current charging status. The module is only visible when the device's battery is below 10%.

### Opsi

| Opsi                 | Bawaan                            | Deskripsi                                                 |
| -------------------- | --------------------------------- | --------------------------------------------------------- |
| `full_symbol`        | `" "`                            | Simbol dimunculkan ketika baterai penuh.                  |
| `charging_symbol`    | `" "`                            | Simbol dimunculkan ketika baterai mengisi.                |
| `discharging_symbol` | `" "`                            | Simbol dimunculkan ketika baterai terpakai.               |
| `unknown_symbol`     | `" "`                            | Simbol dimunculkan ketika keadaan baterai tidak dikenali. |
| `empty_symbol`       | `" "`                            | Simbol dimunculkan ketika keadaan baterai kosong.         |
| `format`             | `"[$symbol$percentage]($style) "` | Format dari modul.                                        |
| `display`            | [link](#battery-display)          | Menampilkan ambang dan gaya dari modul.                   |
| `disabled`           | `false`                           | Menonaktifkan modul `baterai`.                            |

### Contoh

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "
```

### Tampilan Baterai

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style). If no `display` is provided. The default is as shown:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Opsi

The `display` option is an array of the following table.

| Opsi                 | Bawaan     | Deskripsi                                                                                                            |
| -------------------- | ---------- | -------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`       | The upper bound for the display option.                                                                              |
| `style`              | `bold red` | The style used if the display option is in use.                                                                      |
| `charging_symbol`    | `-`        | Simbol opsional ditampilan jika opsi tampilan sedang digunakan, bawaan untuk opsi `charging_symbol` dari baterai.    |
| `discharging_symbol` | `-`        | Simbol opsional ditampilan jika opsi tampilan sedang digunakan, bawaan untuk opsi `discharging_symbol` dari baterai. |

#### Contoh

```toml
[[battery.display]] # "bold red" style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]] # "bold yellow" style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"
discharging_symbol = "💦"

# when capacity is over 30%, the battery indicator will not be displayed
```

## Karakter

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways:

- mengganti warna (`red`/`green`)
- mengganti bentuk (`❯`/`✖`)

By default it only changes color. If you also want to change its shape take a look at [this example](#with-custom-error-shape).

::: peringatan

`error_symbol` is not supported on nu shell.

:::

::: peringatan

`vicmd_symbol` is only supported in cmd, fish and zsh.

:::

### Opsi

| Opsi             | Bawaan              | Deskripsi                                                                                         |
| ---------------- | ------------------- | ------------------------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | Format string yang digunakan sebelum masukan teks.                                                |
| `success_symbol` | `"[❯](bold green)"` | Format string yang digunakan sebelum masukan teks jika perintah sebelumnya berhasil.              |
| `error_symbol`   | `"[❯](bold red)"`   | Format string yang digunakan sebelum masukan teks jika perintah sebelumnya gagal.                 |
| `vicmd_symbol`   | `"[❮](bold green)"` | Format string yang digunakan sebelum masukan teks jika shell sedang dalam vim dengan mode normal. |
| `disabled`       | `false`             | Menonaktifkan module `character`.                                                                 |

### Variabel

| Variabel | Contoh | Deskripsi                                                              |
| -------- | ------ | ---------------------------------------------------------------------- |
| symbol   |        | Representasi dari `success_symbol`, `error_symbol` atau `vicmd_symbol` |

### Contoh

#### Dengan menggunakan corak eror yang dikustomisasi

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Tanpa menggunakan corak eror yang dikustomisasi

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### Dengan menggunakan corak vim yang dikustomisasi

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). By default the module will be activated if any of the following conditions are met:

- Direktori terkini yang berisikan sebuah file `CMakeLists.txt`
- Direktori terkini yang berisikan sebuah file `CMakeCache.txt`

### Opsi

| Opsi                | Bawaan                                 | Deskripsi                                                                           |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`   | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                            | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"△ "`                                 | Simbol yang digunakan sebelum versi cmake.                                          |
| `detect_extensions` | `[]`                                   | Ekstensi mana yang sebaiknya memicu modul ini                                       |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | filenames mana yang sebaiknya memicu modul ini                                      |
| `detect_folders`    | `[]`                                   | Folder mana yang sebaiknya memicul modul ini                                        |
| `style`             | `"bold blue"`                          | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                | Menonaktifkan modul `cmake`.                                                        |

### Variabel

| Variabel  | Contoh    | Deskripsi                         |
| --------- | --------- | --------------------------------- |
| version   | `v3.17.3` | Versi dari cmake                  |
| symbol    |           | Menyalin nilai dari opsi `symbol` |
| style\* |           | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL. By default, the module will be shown if any of the following conditions are met:

- The current directory contains any files ending in `.cob` or `.COB`
- The current directory contains any files ending in `.cbl` or `.CBL`

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `symbol`            | `"⚙️ "`                              | The symbol used before displaying the version of COBOL.                             |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold blue"`                        | Gaya penataan untuk modul.                                                          |
| `detect_extensions` | `["cbl", "cob", "CBL", "COB"]`       | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `[]`                                 | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `disabled`          | `false`                              | Disables the `cobol` module.                                                        |

### Variabel

| Variabel  | Contoh     | Deskripsi                         |
| --------- | ---------- | --------------------------------- |
| version   | `v3.1.2.0` | The version of `cobol`            |
| symbol    |            | Menyalin nilai dari opsi `symbol` |
| style\* |            | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

## Durasi Perintah

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### Opsi

| Opsi                   | Bawaan                        | Deskripsi                                                                                                                                                         |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_tim`              | `2_000`                       | Durasi terpendek untuk menampilkan waktu (dalam milidetik).                                                                                                       |
| `show_milliseconds`    | `false`                       | Tampilkan milidetik sebagai ganti detik untuk durasinya.                                                                                                          |
| `format`               | `"took [$duration]($style) "` | Format dari modul.                                                                                                                                                |
| `style`                | `"bold yellow"`               | Gaya penataan untuk modul.                                                                                                                                        |
| `disabled`             | `false`                       | Menonaktifkan modul `cmd_duration`.                                                                                                                               |
| `show_notifications`   | `false`                       | Menampilkan notifikasi layar ketika perintah selesai.                                                                                                             |
| `min_time_to_notify`   | `45_000`                      | Durasi terpendek untuk menampilkan notifikasi (dalam milidetik).                                                                                                  |
| `notification_timeout` |                               | Duration to show notification for (in milliseconds). If unset, notification timeout will be determined by daemon. Not all notification daemons honor this option. |

:::

### Variabel

| Variabel  | Contoh   | Deskripsi                                          |
| --------- | -------- | -------------------------------------------------- |
| duration  | `16m40s` | Waktu yang dibutuhkan untuk menyelesaikan perintah |
| style\* |          | Menyalin nilai dari opsi `style`                   |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### Opsi

| Opsi                | Bawaan                                 | Deskripsi                                                                                                                                                                                       |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | Jumlah direktori yang dipotong oleh environment path, jika environment-nya dibuat melalui `conda create -p [path]`. `0` artinya tidak ada potongan. Lihat juga modul [`directory`](#directory). |
| `symbol`            | `"🅒 "`                                 | Simbol yang digunakan sebelum nama environment.                                                                                                                                                 |
| `style`             | `"bold green"`                         | Gaya penataan untuk modul.                                                                                                                                                                      |
| `format`            | `"via [$symbol$environment]($style) "` | Format dari modul.                                                                                                                                                                              |
| `ignore_base`       | `true`                                 | Mengabaikan `base` environment saat aktif.                                                                                                                                                      |
| `disabled`          | `false`                                | Menonaktifkan modul `conda`.                                                                                                                                                                    |

### Variabel

| Variabel    | Contoh       | Deskripsi                         |
| ----------- | ------------ | --------------------------------- |
| environment | `astronauts` | Environment conda saat ini        |
| symbol      |              | Menyalin nilai dari opsi `symbol` |
| style\*   |              | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Container

The `container` module displays a symbol and container name, if inside a container.

### Opsi

| Opsi       | Bawaan                               | Deskripsi                                 |
| ---------- | ------------------------------------ | ----------------------------------------- |
| `symbol`   | `"⬢"`                                | The symbol shown, when inside a container |
| `style`    | `"bold red dimmed"`                  | Gaya penataan untuk modul.                |
| `format`   | "[$symbol \\[$name\\]]($style) " | Format dari modul.                        |
| `disabled` | `false`                              | Disables the `container` module.          |

### Variabel

| Variabel  | Contoh              | Deskripsi                         |
| --------- | ------------------- | --------------------------------- |
| name      | `fedora-toolbox:35` | The name of the container         |
| symbol    |                     | Menyalin nilai dari opsi `symbol` |
| style\* |                     | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[container]
format = "[$symbol \\[$name\\]]($style) "
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `shard.yml`
- Direktori terkini yang berisikan sebuah file `.cr`

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `symbol`            | `"🔮 "`                               | The symbol used before displaying the version of crystal.                           |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | Gaya penataan untuk modul.                                                          |
| `detect_extensions` | `["cr"]`                             | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["shard.yml"]`                      | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `disabled`          | `false`                              | Disables the `crystal` module.                                                      |

### Variabel

| Variabel  | Contoh    | Deskripsi                         |
| --------- | --------- | --------------------------------- |
| version   | `v0.32.1` | The version of `crystal`          |
| symbol    |           | Menyalin nilai dari opsi `symbol` |
| style\* |           | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file berekstensi `.dart`
- Direktori terkini yang berisikan sebuah direktori `dart_tool`
- Direktori terkini yang berisikan sebuah file `pubspec.yaml`, `pubspec.yml` atau `pubspec.lock`

### Opsi

| Opsi                | Bawaan                                            | Deskripsi                                                                           |
| ------------------- | ------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                       | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🎯 "`                                            | A format string representing the symbol of Dart                                     |
| `detect_extensions` | `["dart"]`                                        | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[".dart_tool"]`                                  | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold blue"`                                     | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                           | Disables the `dart` module.                                                         |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.8.4` | The version of `dart`             |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `deno.json`, `deno.jsonc`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### Opsi

| Opsi                | Bawaan                                                                  | Deskripsi                                                                           |
| ------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                    | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                                             | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                                                  | A format string representing the symbol of Deno                                     |
| `detect_extensions` | `[]`                                                                    | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["deno.json", "deno.jsonc", "mod.ts", "mod.js", "deps.ts", "deps.js"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                                                    | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"green bold"`                                                          | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                                                 | Disables the `deno` module.                                                         |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.8.3` | The version of `deno`             |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

### Contoh

```toml
# ~/.config/starship.toml

[deno]
format = "via [🦕 $version](green bold) "
```

## Direktori

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Opsi

| Opsi                | Bawaan                                             | Deskripsi                                                                               |
| ------------------- | -------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                | The number of parent folders that the current directory should be truncated to.         |
| `truncate_to_repo`  | `true`                                             | Whether or not to truncate to the root of the git repo that you're currently in.        |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | Format dari modul.                                                                      |
| `style`             | `"bold cyan"`                                      | Gaya penataan untuk modul.                                                              |
| `disabled`          | `false`                                            | Disables the `directory` module.                                                        |
| `read_only`         | `"🔒"`                                              | The symbol indicating current directory is read only.                                   |
| `read_only_style`   | `"red"`                                            | The style for the read only symbol.                                                     |
| `truncation_symbol` | `""`                                               | The symbol to prefix to truncated paths. eg: "…/"                                       |
| `repo_root_style`   | `None`                                             | The style for the root of the git repo when `truncate_to_repo` option is set to false.  |
| `home_symbol`       | `"~"`                                              | The symbol indicating home directory.                                                   |
| `use_os_path_sep`   | `true`                                             | Use the OS specific path seperator instead of always using `/` (e.g. `\` on Windows) |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | Bawaan | Deskripsi                                                                                                                                                              |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`    | The number of characters to use when applying fish shell pwd path logic.                                                                                               |
| `use_logical_path`          | `true` | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variabel

| Variabel  | Contoh                | Deskripsi                        |
| --------- | --------------------- | -------------------------------- |
| path      | `"D:/Projects"`       | The current directory path       |
| style\* | `"black bold dimmed"` | Menyalin nilai dari opsi `style` |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default` or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or `DOCKER_CONTEXT` environment variables are set (as they are meant to override the context in use).

### Opsi

| Opsi                | Bawaan                                                        | Deskripsi                                                                         |
| ------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | Format dari modul.                                                                |
| `symbol`            | `"🐳 "`                                                        | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                        | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                          | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                          | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `"blue bold"`                                                 | Gaya penataan untuk modul.                                                        |
| `disabled`          | `false`                                                       | Disables the `docker_context` module.                                             |

### Variabel

| Variabel  | Contoh         | Deskripsi                         |
| --------- | -------------- | --------------------------------- |
| context   | `test_context` | The current docker context        |
| symbol    |                | Menyalin nilai dari opsi `symbol` |
| style\* |                | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

By default this module will only be shown in your prompt when one or more of the following files are present in the current directory:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

You'll also need the .NET Core SDK installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### Opsi

| Opsi                | Bawaan                                                                                                  | Deskripsi                                                                           |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                                                                             | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | The symbol used before displaying the version of dotnet.                            |
| `heuristic`         | `true`                                                                                                  | Use faster version detection to keep starship snappy.                               |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                                                                                    | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold blue"`                                                                                           | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                                                                                 | Disables the `dotnet` module.                                                       |

### Variabel

| Variabel  | Contoh           | Deskripsi                                                          |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Menyalin nilai dari opsi `symbol`                                  |
| style\* |                  | Menyalin nilai dari opsi `style`                                   |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `mix.exs`.

### Opsi

| Opsi                | Bawaan                                                      | Deskripsi                                                                           |
| ------------------- | ----------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                                                   |
| `version_format`    | `"v${raw}"`                                                 | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | The symbol used before displaying the version of Elixir/Erlang.                     |
| `detect_extensions` | `[]`                                                        | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["mix.exs"]`                                               | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                                        | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold purple"`                                             | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                                       |

### Variabel

| Variabel    | Contoh  | Deskripsi                         |
| ----------- | ------- | --------------------------------- |
| version     | `v1.10` | The version of `elixir`           |
| otp_version |         | The otp version of `elixir`       |
| symbol      |         | Menyalin nilai dari opsi `symbol` |
| style\*   |         | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `elm.json`
- Direktori terkini yang berisikan sebuah file `elm-package.json`
- Direktori terkini yang berisikan sebuah file `elm-version`
- Direktori terkini yang berisikan sebuah folder`elm-stuff`
- Direktori terkini yang berisikan sebuah file `*.elm`

### Opsi

| Opsi                | Bawaan                                             | Deskripsi                                                                           |
| ------------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`               | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                        | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | A format string representing the symbol of Elm.                                     |
| `detect_extensions` | `["elm"]`                                          | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `["elm-stuff"]`                                    | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"cyan bold"`                                      | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                            | Disables the `elm` module.                                                          |

### Variabel

| Variabel  | Contoh    | Deskripsi                         |
| --------- | --------- | --------------------------------- |
| version   | `v0.19.1` | The version of `elm`              |
| symbol    |           | Menyalin nilai dari opsi `symbol` |
| style\* |           | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Variabel Environment

The `env_var` module displays the current value of a selected environment variables. The module will be shown only if any of the following conditions are met:

- Opsi konfigurasi `variable` cocok dengan variabel environment yang ada
- Opsi konfigurasi `variable` tidak didefinisikan, tapi opsi konfigurasi `default` yang didefinisikan

::: tip Multiple environmental variables can be displayed by using a `.`. (see example) If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.

Example: following configuration will display value of USER environment variable

```toml
# ~/.config/starship.toml

[env_var.USER]
default = "unknown user"
```

:::

### Opsi

| Opsi       | Bawaan                         | Deskripsi                                                                    |
| ---------- | ------------------------------ | ---------------------------------------------------------------------------- |
| `symbol`   | `""`                           | The symbol used before displaying the variable value.                        |
| `variable` |                                | The environment variable to be displayed.                                    |
| `default`  |                                | The default value to be displayed when the selected variable is not defined. |
| `format`   | `"with [$env_value]($style) "` | Format dari modul.                                                           |
| `disabled` | `false`                        | Disables the `env_var` module.                                               |

### Variabel

| Variabel  | Contoh                                      | Deskripsi                                  |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Menyalin nilai dari opsi `symbol`          |
| style\* | `black bold dimmed`                         | Menyalin nilai dari opsi `style`           |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

Displaying multiple environmental variables:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = "SHELL"
default = "unknown shell"
[env_var.USER]
default = "unknown user"
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `rebar.config`.
- Direktori terkini yang berisikan sebuah file `erlang.mk`.

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | The symbol used before displaying the version of erlang.                            |
| `style`             | `"bold red"`                         | Gaya penataan untuk modul.                                                          |
| `detect_extensions` | `[]`                                 | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `disabled`          | `false`                              | Disables the `erlang` module.                                                       |

### Variabel

| Variabel  | Contoh    | Deskripsi                         |
| --------- | --------- | --------------------------------- |
| version   | `v22.1.3` | The version of `erlang`           |
| symbol    |           | Menyalin nilai dari opsi `symbol` |
| style\* |           | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are present in a line they will split the space evenly between them. This is useful for aligning other modules.

### Opsi

| Opsi       | Bawaan         | Deskripsi                         |
| ---------- | -------------- | --------------------------------- |
| `symbol`   | `"."`          | The symbol used to fill the line. |
| `style`    | `"bold black"` | Gaya penataan untuk modul.        |
| `disabled` | `false`        | Disables the `fill` module        |

### Contoh

```toml
# ~/.config/starship.toml
format = "AA $fill BB $fill CC"

[fill]
symbol = "-"
style = "bold green"
```

Produces a prompt that looks like:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Opsi

| Opsi             | Bawaan                                                     | Deskripsi                                                       |
| ---------------- | ---------------------------------------------------------- | --------------------------------------------------------------- |
| `format`         | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | Format dari modul.                                              |
| `symbol`         | `"☁️  "`                                                   | The symbol used before displaying the current GCP profile.      |
| `region_aliases` |                                                            | Table of region aliases to display in addition to the GCP name. |
| `style`          | `"bold blue"`                                              | Gaya penataan untuk modul.                                      |
| `disabled`       | `false`                                                    | Disables the `gcloud` module.                                   |

### Variabel

| Variabel  | Contoh        | Deskripsi                                                          |
| --------- | ------------- | ------------------------------------------------------------------ |
| region    | `us-central1` | The current GCP region                                             |
| account   | `foo`         | The current GCP profile                                            |
| domain    | `example.com` | The current GCP profile domain                                     |
| project   |               | The current GCP project                                            |
| active    | `default`     | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |               | Menyalin nilai dari opsi `symbol`                                  |
| style\* |               | Menyalin nilai dari opsi `style`                                   |

*: This variable can only be used as a part of a style string

### Contoh

#### Display account and project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
```

#### Display active config name only

```toml
# ~/.config/starship.toml

[gcloud]
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### Display account and aliased region

```toml
# ~/.config/starship.toml

[gcloud]
symbol = "️🇬️ "
[gcloud.region_aliases]
us-central1 = "uc1"
asia-northeast1 = "an1"
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Opsi

| Opsi                 | Bawaan                           | Deskripsi                                                                                |
| -------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Shows the remote tracking branch name, even if it is equal to the local branch name.     |
| `format`             | `"on [$symbol$branch]($style) "` | Format dari modul. Use `"$branch"` to refer to the current branch name.                  |
| `symbol`             | `" "`                           | A format string representing the symbol of git branch.                                   |
| `style`              | `"bold purple"`                  | Gaya penataan untuk modul.                                                               |
| `truncation_length`  | `2^63 - 1`                       | Truncates a git branch to `N` graphemes.                                                 |
| `truncation_symbol`  | `"…"`                            | The symbol used to indicate a branch name was truncated. You can use `""` for no symbol. |
| `only_attached`      | `false`                          | Only show the branch name when not in a detached `HEAD` state.                           |
| `disabled`           | `false`                          | Disables the `git_branch` module.                                                        |

### Variabel

| Variabel      | Contoh   | Deskripsi                                                                                              |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | Menyalin nilai dari opsi `symbol`                                                                      |
| style\*     |          | Menyalin nilai dari opsi `style`                                                                       |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Opsi

| Opsi                 | Bawaan                             | Deskripsi                                               |
| -------------------- | ---------------------------------- | ------------------------------------------------------- |
| `commit_hash_length` | `7`                                | The length of the displayed git commit hash.            |
| `format`             | `"[\\($hash$tag\\)]($style) "` | Format dari modul.                                      |
| `style`              | `"bold green"`                     | Gaya penataan untuk modul.                              |
| `only_detached`      | `true`                             | Only show git commit hash when in detached `HEAD` state |
| `tag_disabled`       | `true`                             | Disables showing tag info in `git_commit` module.       |
| `tag_symbol`         | `" 🏷 "`                            | Tag symbol prefixing the info shown                     |
| `disabled`           | `false`                            | Disables the `git_commit` module.                       |

### Variabel

| Variabel  | Contoh    | Deskripsi                        |
| --------- | --------- | -------------------------------- |
| hash      | `b703eb3` | The current git commit hash      |
| style\* |           | Menyalin nilai dari opsi `style` |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Opsi

| Opsi           | Bawaan                                                          | Deskripsi                                                                               |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                 | Gaya penataan untuk modul.                                                              |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | Format dari modul.                                                                      |
| `disabled`     | `false`                                                         | Disables the `git_state` module.                                                        |

### Variabel

| Variabel         | Contoh     | Deskripsi                        |
| ---------------- | ---------- | -------------------------------- |
| state            | `REBASING` | The current state of the repo    |
| progress_current | `1`        | The current operation progress   |
| progress_total   | `2`        | The total operation progress     |
| style\*        |            | Menyalin nilai dari opsi `style` |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in the current git repository.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opsi

| Opsi                 | Bawaan                                                       | Deskripsi                             |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `"bold green"`                                               | The style for the added count.        |
| `deleted_style`      | `"bold red"`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | Format dari modul.                    |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.    |

### Variabel

| Variabel          | Contoh | Deskripsi                                   |
| ----------------- | ------ | ------------------------------------------- |
| added             | `1`    | The current number of added lines           |
| deleted           | `2`    | The current number of deleted lines         |
| added_style\*   |        | Mirrors the value of option `added_style`   |
| deleted_style\* |        | Mirrors the value of option `deleted_style` |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = "bold blue"
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Opsi

| Opsi                | Bawaan                                          | Deskripsi                           |
| ------------------- | ----------------------------------------------- | ----------------------------------- |
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status` |
| `conflicted`        | `"="`                                           | This branch has merge conflicts.    |
| `ahead`             | `"⇡"`                                           | The format of `ahead`               |
| `behind`            | `"⇣"`                                           | The format of `behind`              |
| `diverged`          | `"⇕"`                                           | The format of `diverged`            |
| `up_to_date`        | `""`                                            | The format of `up_to_date`          |
| `untracked`         | `"?"`                                           | The format of `untracked`           |
| `stashed`           | `"$"`                                           | The format of `stashed`             |
| `modified`          | `"!"`                                           | The format of `modified`            |
| `staged`            | `"+"`                                           | The format of `staged`              |
| `renamed`           | `"»"`                                           | The format of `renamed`             |
| `deleted`           | `"✘"`                                           | The format of `deleted`             |
| `style`             | `"bold red"`                                    | Gaya penataan untuk modul.          |
| `ignore_submodules` | `false`                                         | Ignore changes to submodules.       |
| `disabled`          | `false`                                         | Disables the `git_status` module.   |

### Variabel

The following variables can be used in `format`:

| Variabel       | Deskripsi                                                                                                     |
| -------------- | ------------------------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                                   |
| `ahead_behind` | Displays `diverged`, `ahead`, `behind` or `up_to_date` format string based on the current status of the repo. |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                                   |
| `untracked`    | Displays `untracked` when there are untracked files in the working directory.                                 |
| `stashed`      | Displays `stashed` when a stash exists for the local repository.                                              |
| `modified`     | Displays `modified` when there are file modifications in the working directory.                               |
| `staged`       | Displays `staged` when a new file has been added to the staging area.                                         |
| `renamed`      | Displays `renamed` when a renamed file has been added to the staging area.                                    |
| `deleted`      | Displays `deleted` when a file's deletion has been added to the staging area.                                 |
| style\*      | Menyalin nilai dari opsi `style`                                                                              |

*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| Variabel       | Deskripsi                                      |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Variabel | Deskripsi                |
| -------- | ------------------------ |
| `count`  | Show the number of files |

### Contoh

```toml
# ~/.config/starship.toml

[git_status]
conflicted = "🏳"
ahead = "🏎💨"
behind = "😰"
diverged = "😵"
up_to_date = "✓"
untracked = "🤷"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `go.mod`
- Direktori terkini yang berisikan sebuah file `go.sum`
- Direktori terkini yang berisikan sebuah file `glide.yaml`
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Opsi

| Opsi                | Bawaan                                                                         | Deskripsi                                                                           |
| ------------------- | ------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                           | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                                                    | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                         | A format string representing the symbol of Go.                                      |
| `detect_extensions` | `["go"]`                                                                       | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `["Godeps"]`                                                                   | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold cyan"`                                                                  | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                                                        | Disables the `golang` module.                                                       |

### Variabel

| Variabel  | Contoh    | Deskripsi                         |
| --------- | --------- | --------------------------------- |
| version   | `v1.12.1` | The version of `go`               |
| symbol    |           | Menyalin nilai dari opsi `symbol` |
| style\* |           | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `helmfile.yaml`
- The current directory contains a `Chart.yaml` file

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm.                                    |
| `style`             | `"bold white"`                       | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `helm` module.                                                         |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v3.1.1` | The version of `helm`             |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

The `hostname` module shows the system hostname.

### Opsi

| Opsi       | Bawaan                      | Deskripsi                                                                                                                            |
| ---------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                      | Only show hostname when connected to an SSH session.                                                                                 |
| `trim_at`  | `"."`                       | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `format`   | `"[$hostname]($style) in "` | Format dari modul.                                                                                                                   |
| `style`    | `"bold dimmed green"`       | Gaya penataan untuk modul.                                                                                                           |
| `disabled` | `false`                     | Disables the `hostname` module.                                                                                                      |

### Variabel

| Variabel  | Contoh     | Deskripsi                        |
| --------- | ---------- | -------------------------------- |
| hostname  | `computer` | The hostname of the computer     |
| style\* |            | Menyalin nilai dari opsi `style` |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Opsi

| Opsi                | Bawaan                                                                                                    | Deskripsi                                                                           |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                                                                               | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                                                                                      | Folder mana yang sebaiknya memicul modul ini.                                       |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java                                     |
| `style`             | `"red dimmed"`                                                                                            | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                                                                                   | Disables the `java` module.                                                         |

### Variabel

| Variabel  | Contoh | Deskripsi                         |
| --------- | ------ | --------------------------------- |
| version   | `v14`  | The version of `java`             |
| symbol    |        | Menyalin nilai dari opsi `symbol` |
| style\* |        | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there are at least 2 jobs, or more than the `number_threshold` config value, if it exists. The module will show a symbol if there is at least 1 job, or more than the `symbol_threshold` config value, if it exists. You can set both values to 0 in order to _always_ show the symbol and number of jobs, even if there are 0 jobs running.

The default functionality is:

- 0 jobs -> Nothing is shown.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

::: peringatan

This module is not supported on tcsh and nu.

:::

::: peringatan

The `threshold` option is deprecated, but if you want to use it, the module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

:::

### Opsi

| Opsi               | Bawaan                        | Deskripsi                                                                |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `"[$symbol$number]($style) "` | Format dari modul.                                                       |
| `symbol`           | `"✦"`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `"bold blue"`                 | Gaya penataan untuk modul.                                               |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variabel

| Variabel  | Contoh | Deskripsi                         |
| --------- | ------ | --------------------------------- |
| number    | `1`    | The number of jobs                |
| symbol    |        | Menyalin nilai dari opsi `symbol` |
| style\* |        | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
number_threshold = 4
symbol_threshold = 0
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia.                                   |
| `style`             | `"bold purple"`                      | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `julia` module.                                                        |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.4.0` | The version of `julia`            |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## localip

The `localip` module shows the IPv4 address of the primary network interface.

### Opsi

| Opsi       | Bawaan                    | Deskripsi                                              |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `"[$localipv4]($style) "` | Format dari modul.                                     |
| `style`    | `"bold yellow"`           | Gaya penataan untuk modul.                             |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### Variabel

| Variabel  | Contoh       | Deskripsi                         |
| --------- | ------------ | --------------------------------- |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address |
| style\* |              | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = "@[$localipv4](bold red) "
disabled = false
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `.kt` or a `.kts` file

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["kt", "kts"]`                      | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `[]`                                 | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                                  |
| `style`             | `"bold blue"`                        | Gaya penataan untuk modul.                                                          |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version.       |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                       |

### Variabel

| Variabel  | Contoh    | Deskripsi                         |
| --------- | --------- | --------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`           |
| symbol    |           | Menyalin nilai dari opsi `symbol` |
| style\* |           | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = "kotlinc"
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opsi

| Opsi              | Bawaan                                               | Deskripsi                                                             |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | Format dari modul.                                                    |
| `style`           | `"cyan bold"`                                        | Gaya penataan untuk modul.                                            |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Disables the `kubernetes` module.                                     |

### Variabel

| Variabel  | Contoh               | Deskripsi                                |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Menyalin nilai dari opsi `symbol`        |
| style\* |                      | Menyalin nilai dari opsi `style`         |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
".*/openshift-cluster/.*" = "openshift"
"gke_.*_(?P<cluster>[\\w-]+)" = "gke-$cluster"
```

#### Regex Matching

Additional to simple aliasing, `context_aliases` also supports extended matching and renaming using regular expressions.

The regular expression must match on the entire kube context, capture groups can be referenced using `$name` and `$N` in the replacement. This is more explained in the [regex crate](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace) documentation.

Long and automatically generated cluster names can be identified and shortened using regular expressions:

```toml
[kubernetes.context_aliases]
# OpenShift contexts carry the namespace and user in the kube context: `namespace/name/user`:
".*/openshift-cluster/.*" = "openshift"
# Or better, to rename every OpenShift cluster at once:
".*/(?P<cluster>[\\w-]+)/.*" = "$cluster"

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
"gke_.*_(?P<cluster>[\\w-]+)" = "gke-$cluster"
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Opsi

| Opsi       | Bawaan  | Deskripsi                                                          |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### Contoh

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                                     |
| `detect_extensions` | `["lua"]`                            | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `[".lua-version"]`                   | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `["lua"]`                            | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold blue"`                        | Gaya penataan untuk modul.                                                          |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version.          |
| `disabled`          | `false`                              | Disables the `lua` module.                                                          |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v5.4.0` | The version of `lua`              |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opsi

| Opsi        | Bawaan                                          | Deskripsi                                                |
| ----------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | Format dari modul.                                       |
| `symbol`    | `"🐏"`                                           | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                           | Gaya penataan untuk modul.                               |
| `disabled`  | `true`                                          | Disables the `memory_usage` module.                      |

### Variabel

| Variabel         | Contoh        | Deskripsi                                                          |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Menyalin nilai dari opsi `symbol`                                  |
| style\*        |               | Menyalin nilai dari opsi `style`                                   |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

### Contoh

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Opsi

| Opsi                | Bawaan                           | Deskripsi                                                                                    |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | Gaya penataan untuk modul.                                                                   |
| `format`            | `"on [$symbol$branch]($style) "` | Format dari modul.                                                                           |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `"…"`                            | The symbol used to indicate a branch name was truncated.                                     |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| branch    | `master` | The active mercurial branch       |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `nim.cfg`
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module                                                           |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim.                               |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["nim.cfg"]`                        | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold yellow"`                      | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `nim` module.                                                          |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.2.0` | The version of `nimc`             |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. The module will be shown when inside a nix-shell environment.

### Opsi

| Opsi         | Bawaan                                         | Deskripsi                                             |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | Format dari modul.                                    |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | Gaya penataan untuk modul.                            |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Disables the `nix_shell` module.                      |

### Variabel

| Variabel  | Contoh  | Deskripsi                         |
| --------- | ------- | --------------------------------- |
| state     | `pure`  | The state of the nix-shell        |
| name      | `lorri` | The name of the nix-shell         |
| symbol    |         | Menyalin nilai dari opsi `symbol` |
| style\* |         | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `package.json`
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                                             |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                                    |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch`                   |
| `symbol`            | `" "`                               | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Ekstensi mana yang sebaiknya memicu modul ini.                                                        |
| `detect_files`      | `["package.json", ".node-version"]`  | filenames mana yang sebaiknya memicu modul ini.                                                       |
| `detect_folders`    | `["node_modules"]`                   | Folder mana yang sebaiknya memicul modul ini.                                                         |
| `style`             | `"bold green"`                       | Gaya penataan untuk modul.                                                                            |
| `disabled`          | `false`                              | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variabel

| Variabel  | Contoh     | Deskripsi                         |
| --------- | ---------- | --------------------------------- |
| version   | `v13.12.0` | The version of `node`             |
| symbol    |            | Menyalin nilai dari opsi `symbol` |
| style\* |            | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Opsi

| Opsi                      | Bawaan                                                                     | Deskripsi                                                                           |
| ------------------------- | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | The format string for the module.                                                   |
| `version_format`          | `"v${raw}"`                                                                | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml.                             |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch.                             |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.                              |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`                   | `"bold yellow"`                                                            | Gaya penataan untuk modul.                                                          |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                                        |

### Variabel

| Variabel         | Contoh       | Deskripsi                                                         |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Menyalin nilai dari opsi `symbol`                                 |
| style\*        |              | Menyalin nilai dari opsi `style`                                  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Opsi

| Opsi       | Bawaan                                              | Deskripsi                                                      |
| ---------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | Format dari modul.                                             |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | Gaya penataan untuk modul.                                     |
| `disabled` | `false`                                             | Disables the `openstack` module.                               |

### Variabel

| Variabel  | Contoh | Deskripsi                         |
| --------- | ------ | --------------------------------- |
| cloud     | `corp` | The current OpenStack cloud       |
| project   | `dev`  | The current OpenStack project     |
| symbol    |        | Menyalin nilai dari opsi `symbol` |
| style\* |        | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from the `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present
- [**SBT**](https://scala-sbt.org) - The `sbt` package version is extracted from the `build.sbt` present in the current directory
- [**Dart**](https://pub.dev/) - The `dart` package version is extracted from the `pubspec.yaml` present in the current directory

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Opsi

| Opsi              | Bawaan                            | Deskripsi                                                                           |
| ----------------- | --------------------------------- | ----------------------------------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | Format dari modul.                                                                  |
| `symbol`          | `"📦 "`                            | The symbol used before displaying the version the package.                          |
| `version_format`  | `"v${raw}"`                       | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `style`           | `"bold 208"`                      | Gaya penataan untuk modul.                                                          |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.                           |
| `disabled`        | `false`                           | Disables the `package` module.                                                      |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.0.0` | The version of your package       |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Opsi

| Opsi                | Bawaan                                                                                                   | Deskripsi                                                                           |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | The format string for the module.                                                   |
| `version_format`    | `"v${raw}"`                                                                                              | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl                               |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                                                                                     | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold 149"`                                                                                             | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                                         |

### Variabel

| Variabel  | Contoh    | Deskripsi                         |
| --------- | --------- | --------------------------------- |
| version   | `v5.26.1` | The version of `perl`             |
| symbol    |           | Menyalin nilai dari opsi `symbol` |
| style\* |           | Menyalin nilai dari opsi `style`  |

### Contoh

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `composer.json`
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | The symbol used before displaying the version of PHP.                               |
| `detect_extensions` | `["php"]`                            | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["composer.json", ".php-version"]`  | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"147 bold"`                         | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `php` module.                                                          |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v7.3.8` | The version of `php`              |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

::: tip

By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms). If you still want to enable it, [follow the example shown below](#with-pulumi-version).

:::

Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml`

### Opsi

| Opsi             | Bawaan                                       | Deskripsi                                                                           |
| ---------------- | -------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`         | `"via [$symbol($username@)$stack]($style) "` | The format string for the module.                                                   |
| `version_format` | `"v${raw}"`                                  | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `" "`                                       | A format string shown before the Pulumi stack.                                      |
| `style`          | `"bold 5"`                                   | Gaya penataan untuk modul.                                                          |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                                       |

### Variabel

| Variabel  | Contoh     | Deskripsi                         |
| --------- | ---------- | --------------------------------- |
| version   | `v0.12.24` | The version of `pulumi`           |
| stack     | `dev`      | The current Pulumi stack          |
| username  | `alice`    | The current Pulumi username       |
| symbol    |            | Menyalin nilai dari opsi `symbol` |
| style\* |            | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

#### With Pulumi Version

```toml
# ~/.config/starship.toml

[pulumi]
format = "[🛥 ($version )$stack]($style) "
```

#### Without Pulumi version

```toml
# ~/.config/starship.toml
[pulumi]
symbol = "🛥 "
format = "[$symbol$stack]($style) "
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `spago.dhall`
- The current directory contains a file with the `.purs` extension

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript.                        |
| `detect_extensions` | `["purs"]`                           | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["spago.dhall"]`                    | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold white"`                       | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `purescript` module.                                                   |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `0.13.5` | The version of `purescript`       |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- Direktori terkini yang berisikan sebuah file `pyproject.toml`
- Direktori terkini yang berisikan sebuah file `requirements.txt`
- Direktori terkini yang berisikan sebuah file `setup.py`
- Direktori terkini yang berisikan sebuah file `tox.ini`
- The current directory contains a file with the `.py` extension.
- A virtual environment is currently activated

### Opsi

| Opsi                 | Bawaan                                                                                                       | Deskripsi                                                                              |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | Format dari modul.                                                                     |
| `version_format`     | `"v${raw}"`                                                                                                  | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch`    |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | Gaya penataan untuk modul.                                                             |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                        |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3", "python2"]`                                                                           | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `["py"]`                                                                                                     | Ekstensi mana yang sebaiknya memicu modul ini                                          |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | filenames mana yang sebaiknya memicu modul ini                                         |
| `detect_folders`     | `[]`                                                                                                         | Folder mana yang sebaiknya memicul modul ini                                           |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                          |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Variabel

| Variabel     | Contoh          | Deskripsi                                  |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Menyalin nilai dari opsi `symbol`          |
| style        | `"yellow bold"` | Menyalin nilai dari opsi `style`           |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |

### Contoh

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = "python3"
```

```toml
# ~/.config/starship.toml

[python]
# Don't trigger for files with the py extension
detect_extensions = []
```

```toml
# ~/.config/starship.toml

[python]
# Display the version of python from inside a local venv.
#
# Note this will only work when the venv is inside the project and it will only
# work in the directory that contains the venv dir but maybe this is ok?
python_binary = ["./venv/bin/python", "python", "python3", "python2"]
```

## R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"📐"`                                | A format string representing the symbol of R.                                       |
| `style`             | `"blue bold"`                        | Gaya penataan untuk modul.                                                          |
| `detect_extensions` | `["R", "Rd", "Rmd", "Rproj", "Rsx"]` | Ekstensi mana yang sebaiknya memicu modul ini                                       |
| `detect_files`      | `[".Rprofile"]`                      | filenames mana yang sebaiknya memicu modul ini                                      |
| `detect_folders`    | `[".Rproj.user"]`                    | Folder mana yang sebaiknya memicul modul ini                                        |
| `disabled`          | `false`                              | Disables the `r` module.                                                            |

### Variabel

| Variabel | Contoh        | Deskripsi                         |
| -------- | ------------- | --------------------------------- |
| version  | `v4.0.5`      | The version of `R`                |
| symbol   |               | Menyalin nilai dari opsi `symbol` |
| style    | `"blue bold"` | Menyalin nilai dari opsi `style`  |

### Contoh

```toml
# ~/.config/starship.toml

[rlang]
format = "with [📐 $version](blue bold) "
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a file with `.red` or `.reds` extension

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.                                     |
| `detect_extensions` | `["red"]`                            | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `[]`                                 | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"red bold"`                         | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `red` module.                                                          |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.5.1` | The version of `red`              |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                                    |
| `detect_extensions` | `["rb"]`                             | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `detect_variables`  | `["RUBY_VERSION", "RBENV_VERSION"]`  | Which environment variables should trigger this module.                             |
| `style`             | `"bold red"`                         | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `ruby` module.                                                         |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.5.1` | The version of `ruby`             |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust                                     |
| `detect_extensions` | `["rs"]`                             | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["Cargo.toml"]`                     | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold red"`                         | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `rust` module.                                                         |

### Variabel

| Variabel  | Contoh            | Deskripsi                         |
| --------- | ----------------- | --------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`            |
| symbol    |                   | Menyalin nilai dari opsi `symbol` |
| style\* |                   | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Opsi

| Opsi                | Bawaan                                   | Deskripsi                                                                           |
| ------------------- | ---------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                              | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[".metals"]`                            | Folder mana yang sebaiknya memicul modul ini.                                       |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.                                   |
| `style`             | `"red dimmed"`                           | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                  | Disables the `scala` module.                                                        |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `2.13.5` | The version of `scala`            |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opsi

| Opsi                   | Bawaan                    | Deskripsi                                                    |
| ---------------------- | ------------------------- | ------------------------------------------------------------ |
| `bash_indicator`       | `bsh`                     | A format string used to represent bash.                      |
| `fish_indicator`       | `fsh`                     | A format string used to represent fish.                      |
| `zsh_indicator`        | `zsh`                     | A format string used to represent zsh.                       |
| `powershell_indicator` | `psh`                     | A format string used to represent powershell.                |
| `ion_indicator`        | `ion`                     | A format string used to represent ion.                       |
| `elvish_indicator`     | `esh`                     | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `tsh`                     | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `xsh`                     | A format string used to represent xonsh.                     |
| `cmd_indicator`        | `cmd`                     | A format string used to represent cmd.                       |
| `nu_indicator`         | `nu`                      | A format string used to represent nu.                        |
| `unknown_indicator`    |                           | The default value to be displayed when the shell is unknown. |
| `format`               | `"[$indicator]($style) "` | Format dari modul.                                           |
| `style`                | `"white bold"`            | Gaya penataan untuk modul.                                   |
| `disabled`             | `true`                    | Disables the `shell` module.                                 |

### Variabel

| Variabel  | Bawaan | Deskripsi                                                  |
| --------- | ------ | ---------------------------------------------------------- |
| indicator |        | Mirrors the value of `indicator` for currently used shell. |
| style\* |        | Mirrors the value of option `style`.                       |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "mystery shell"
style = "cyan bold"
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Opsi

| Opsi        | Bawaan                       | Deskripsi                                                     |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | Format dari modul.                                            |
| `symbol`    | `"↕️  "`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | Gaya penataan untuk modul.                                    |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Variabel

| Variabel  | Contoh | Deskripsi                         |
| --------- | ------ | --------------------------------- |
| shlvl     | `3`    | The current value of `SHLVL`      |
| symbol    |        | Menyalin nilai dari opsi `symbol` |
| style\* |        | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### Opsi

| Opsi       | Bawaan                           | Deskripsi                                        |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | Format dari modul.                               |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | Gaya penataan untuk modul.                       |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variabel

| Variabel  | Contoh       | Deskripsi                         |
| --------- | ------------ | --------------------------------- |
| env       | `centos.img` | The current Singularity image     |
| symbol    |              | Menyalin nilai dari opsi `symbol` |
| style\* |              | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

::: warning This module is not supported on nu shell. :::

### Opsi

| Opsi                    | Bawaan                                                                               | Deskripsi                                               |
| ----------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "`                                                        | The format of the module                                |
| `symbol`                | `"✖"`                                                                                | The symbol displayed on program error                   |
| `success_symbol`        | `"✔️"`                                                                               | The symbol displayed on program success                 |
| `not_executable_symbol` | `"🚫"`                                                                                | The symbol displayed when file isn't executable         |
| `not_found_symbol`      | `"🔍"`                                                                                | The symbol displayed when the command can't be found    |
| `sigint_symbol`         | `"🧱"`                                                                                | The symbol displayed on SIGINT (Ctrl + c)               |
| `signal_symbol`         | `"⚡"`                                                                                | The symbol displayed on any signal                      |
| `style`                 | `"bold red"`                                                                         | Gaya penataan untuk modul.                              |
| `recognize_signal_code` | `true`                                                                               | Enable signal mapping from exit code                    |
| `map_symbol`            | `false`                                                                              | Enable symbols mapping from exit code                   |
| `pipestatus`            | `false`                                                                              | Enable pipestatus reporting                             |
| `pipestatus_separator`  | `|`                                                                                  |                                                         |
| `pipestatus_format`     | `\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)` | The format of the module when the command is a pipeline |
| `disabled`              | `true`                                                                               | Disables the `status` module.                           |

### Variabel

| Variabel       | Contoh  | Deskripsi                                                                                   |
| -------------- | ------- | ------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                           |
| hex_status     | `0x7F`  | The exit code of the last command in hex                                                    |
| int            | `127`   | The exit code of the last command                                                           |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                         |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                             |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                        |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                |
| pipestatus     |         | Rendering of in pipeline programs's exit codes, this is only available in pipestatus_format |
| symbol         |         | Menyalin nilai dari opsi `symbol`                                                           |
| style\*      |         | Menyalin nilai dari opsi `style`                                                            |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴"
format = '[\[$symbol $common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

The `sudo` module displays if sudo credentials are currently cached. The module will only be shown if credentials are cached.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opsi

| Opsi            | Bawaan                  | Deskripsi                                               |
| --------------- | ----------------------- | ------------------------------------------------------- |
| `format`        | `[as $symbol]($style)"` | The format of the module                                |
| `symbol`        | `"🧙 "`                  | The symbol displayed when credentials are cached        |
| `style`         | `"bold blue"`           | Gaya penataan untuk modul.                              |
| `allow_windows` | `false`                 | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                  | Disables the `sudo` module.                             |

### Variabel

| Variabel  | Contoh | Deskripsi                         |
| --------- | ------ | --------------------------------- |
| symbol    |        | Menyalin nilai dari opsi `symbol` |
| style\* |        | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[sudo]
style = "bold green"
symbol = "👩‍💻 "
disabled = false
```

```toml
# On windows
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift                                    |
| `detect_extensions` | `["swift"]`                          | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["Package.swift"]`                  | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold 202"`                         | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `swift` module.                                                        |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v5.2.4` | The version of `swift`            |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.

::: tip

By default the Terraform version is not shown, since this is slow for current versions of Terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-terraform-version).

:::

Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                                                   |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.                               |
| `detect_extensions` | `["tf", "tfplan", "tfstate"]`        | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `[]`                                 | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[".terraform"]`                     | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold 105"`                         | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `terraform` module.                                                    |

### Variabel

| Variabel  | Contoh     | Deskripsi                         |
| --------- | ---------- | --------------------------------- |
| version   | `v0.12.24` | The version of `terraform`        |
| workspace | `default`  | The current Terraform workspace   |
| symbol    |            | Menyalin nilai dari opsi `symbol` |
| style\* |            | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

#### With Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Without Terraform version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Waktu

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opsi

| Opsi              | Bawaan                  | Deskripsi                                                                                                                          |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                  |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `"bold yellow"`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `"local"`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Variabel

| Variabel  | Contoh     | Deskripsi                        |
| --------- | ---------- | -------------------------------- |
| time      | `13:08:10` | The current time.                |
| style\* |            | Menyalin nilai dari opsi `style` |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. The module will be shown if any of the following conditions are met:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Opsi

| Opsi          | Bawaan                  | Deskripsi                             |
| ------------- | ----------------------- | ------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root. |
| `style_user`  | `"bold yellow"`         | The style used for non-root users.    |
| `format`      | `"[$user]($style) in "` | Format dari modul.                    |
| `show_always` | `false`                 | Always shows the `username` module.   |
| `disabled`    | `false`                 | Disables the `username` module.       |

### Variabel

| Variabel | Contoh       | Deskripsi                                                                                   |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `"matchai"`  | The currently logged-in user ID.                                                            |

### Contoh

```toml
# ~/.config/starship.toml

[username]
style_user = "white bold"
style_root = "black bold"
format = "user: [$user]($style) "
disabled = false
show_always = true
```

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `Vagrantfile` file

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.                                 |
| `detect_extensions` | `[]`                                 | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["Vagrantfile"]`                    | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"cyan bold"`                        | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                                      |

### Variabel

| Variabel  | Contoh           | Deskripsi                         |
| --------- | ---------------- | --------------------------------- |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`          |
| symbol    |                  | Menyalin nilai dari opsi `symbol` |
| style\* |                  | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Opsi

| Opsi                | Bawaan                                       | Deskripsi                                                                           |
| ------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                  | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"V "`                                       | A format string representing the symbol of V                                        |
| `detect_extensions` | `["v"]`                                      | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                         | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"blue bold"`                                | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                                        |

### Variabel

| Variabel  | Contoh | Deskripsi                         |
| --------- | ------ | --------------------------------- |
| version   | `v0.2` | The version of `v`                |
| symbol    |        | Menyalin nilai dari opsi `symbol` |
| style\* |        | Menyalin nilai dari opsi `style`  |

### Contoh

```toml
# ~/.config/starship.toml
[vlang]
format = "via [V $version](blue bold) "
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### Opsi

| Opsi       | Bawaan                           | Deskripsi                                              |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   |                                  | The symbol used before displaying the repository name. |
| `style`    | `"bold yellow"`                  | Gaya penataan untuk modul.                             |
| `format`   | `"vcsh [$symbol$repo]($style) "` | Format dari modul.                                     |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variabel

| Variabel  | Contoh                                      | Deskripsi                         |
| --------- | ------------------------------------------- | --------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name        |
| symbol    |                                             | Menyalin nilai dari opsi `symbol` |
| style\* | `black bold dimmed`                         | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig.                               |
| `style`             | `"bold yellow"`                      | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `zig` module.                                                          |
| `detect_extensions` | `["zig"]`                            | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `[]`                                 | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v0.6.0` | The version of `zig`              |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

*: This variable can only be used as a part of a style string

### Contoh

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `files`
- The current directory contains a directory whose name is in `directories`
- The current directory contains a file whose extension is in `extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matchs with `os` field if defined.

::: tip

Multiple custom modules can be defined by using a `.`.

:::

::: tip

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

::: warning Command output is printed unescaped to the prompt

Whatever output the command generates is printed unmodified in the prompt. This means if the output contains special sequences that are interpreted by your shell they will be expanded when displayed. These special sequences are shell specific, e.g. you can write a command module that writes bash sequences, e.g. `\h`, but this module will not work in a fish or zsh shell.

Format strings can also contain shell specific prompt sequences, e.g. [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

:::

### Opsi

| Opsi          | Bawaan                          | Deskripsi                                                                                                                                                                     |
| ------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | `""`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                 |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code.                                                    |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                                                                            |
| `description` | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                                                                                  |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                         |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                   |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                    |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                                                                         |
| `style`       | `"bold green"`                  | Gaya penataan untuk modul.                                                                                                                                                    |
| `format`      | `"[$symbol($output )]($style)"` | Format dari modul.                                                                                                                                                            |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                                                                                |
| `os`          |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html). |

### Variabel

| Variabel  | Deskripsi                              |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Menyalin nilai dari opsi `symbol`      |
| style\* | Menyalin nilai dari opsi `style`       |

*: This variable can only be used as a part of a style string

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ["pwsh", "-Command", "-"]
```

::: warning Make sure your custom shell configuration exits gracefully

If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).

For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.

Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.

Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

:::

### Contoh

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo" # shows output of command
files = ["foo"] # can specify filters but wildcards are not supported
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
extensions = ["pst"] # filters *.pst files
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
