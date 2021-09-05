# Konfigurasi

Untuk memulai mengkonfigurasi starship, buatlah file berikut: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Seluruh konfigurasi starship dilakukan dalam file [TOML](https://github.com/toml-lang/toml) berikut:

```toml
# Menambahkan baris kosong antar prompt shell
add_newline = true

# Mengganti simbol "❯" pada prompt dengan simbol "➜"
[character]                            # Nama modul yang dikonfigurasi adalah "character"
success_symbol = "[➜](bold green)"     # Segmen "success_symbol" diganti menjadi "➜" dengan warna "bold green"

# Menonaktifkan paket modul, menyembunyikannya dari prompt seutuhnya
[package]
disabled = true
```

Kamu bisa mengganti lokasi file konfigurasi bawaan dengan menggunakan environment variable dari `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/.starship/config.toml
```

Ekuivalen dalam PowerShell (Windows), tambahkan baris berikut pada `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship\config.toml"
```

### Logging

Secara bawaan, starship mencatat peringatan dan eror ke dalam sebuah file bernama `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, yang mana session key file tersebut sama dengan instance terminalmu. Namun, hal tersebut dapat diubah dengan menggunakan environment variable dari `STARSHIP_CACHE` berikut:

```sh
export STARSHIP_CONFIG=~/.starship/cache
```

Pada PowerShell (Windows), tambahkan baris berikut pada `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### Terminologi

**Modul**: Sebuah komponen pada prompt yang memberikan informasi berdasarkan info kontekstual sistem operasimu. Sebagai contoh, modul "nodejs" menampilkan versi Node.js yang tengah terpasang pada komputermu, jika direktorimu sedang berada pada proyek Node.js.

**Variabel**: Sub-komponen yang terdiri dari informasi yang disediakan oleh modul. Sebagai contoh, variabel "version" dalam modul "nodejs" memuat versi terikini dari Node.js.

Secara konvensi, sebagian modul memiliki prefiks bawaan pada warna terminal (mis. `via` pada "nodejs") dan juga sebuah ruang kosong sebagai sufiks.

### Format String

Format string merupakan format yang sebuah modul gunakan untuk menampilkan semua variabelnya. Sebagian besar modul memiliki sebuah entri yang disebut `format` yang mengkonfigurasi format tampilan pada modul. Kamu bisa menggunakan teks, variabel, dan grup teks di dalam sebuah format string.

#### Variabel

Variabel memilki simbol `$` yang diikuti dengan nama variabelnya. Nama dari sebuah variabel hanya boleh berisikan huruf, angka, dan `_`.

Sebagai contoh:

- `$version` adalah format string dengan sebuah nama variabel `version`.
- `$git_branch$git_commit` merupakan sebuah format string dengan dua variabel bernama `git_branch` dan `git_commit`.
- `$git_branch $git_commit` memiliki dua variabel yang dipisahkan dengan sebuah spasi.

#### Grup Teks

Grup teks dibuat dengan dua bagian yang berbeda.

Bagian pertama, yang mana diapit dalam sebuah `[]`, merupakan sebuah [format string](#format-strings). Kamu bisa menambahkan teks, variabel, atau bahkan grup teks bercabang di dalamnya.

Pada bagian kedua, yang mana diapit dalam sebuah `()`, merupakan sebuah [penataan string](#style-strings). Bagian ini dapat digunakan untuk menata bagian pertama.

Sebagai contoh:

- `[on](red bold)` akan menampilkan string `on` dengan teks merah tebal.
- `[⌘ $version](bold green)` akan menampilkan simbol `⌘` yang diikuti oleh variabel yang berisikan `version`, dengan teks tebal berwarna hijau.
- `[a [b](red) c](green)` akan menampilkan `a b c` dengan `b` merah, dan `a` & `c` green.

#### Menata String

Sebagian besar modul starship memungkinkan kamu untuk mengkonfigurasi gaya tampilannya. Hal ini dilakukan dengan sebuah entri (biasanya `style`) yang konfigurasinya ditentukan oleh string. Berikut adalah beberapa contoh penataan pada string dan kegunaannya. Untuk detail sintaksis yang lebih lengkap, lihat [panduan konfigurasi lanjutan](/advanced-config/).

- `"fg:green bg:blue"` mengeset teks berwana hijau pada latar biru
- `"bg:blue fg:bright-green"` mengeset teks hijau terang pada latar biru
- `"bold fg:27"` mengeset tebal teks dengan [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` mengeset teks bergaris bawah pada latar oranye menyala
- `"bold italic fg:purple"` mengeset teks ungu miring tebal
- `""` secara eksplisit menonaktifkan semua penataan gaya

Perhatikan bagaimana nantinya penataanmu akan terlihat pada emulator terminalmu. Sebagai contoh, beberapa terminal emulator alih-alih membuat warnanya lebih terang, malah menebalkan teksnya, dan sebagian palet warna ada yang menggunakan nilai yang sama untuk warna normal dan terangnya. Dan juga, untuk memakai teks miring, terminalmu harus menunjang teks miring.

#### Format String Bersyarat (Conditional)

Sebuah string dengan format bersyarat dibungkus dengan `(` dan `)` tidak akan dijalankan jika variabel di dalamnya kosong.

Sebagai contoh:

- `(@$region)` tidak akan menampilkan apapun jika nilai variabel `region` adalah `None` atau berupa string kosong, jika tidak, `@` diikuti dengan nilai dari region.
- `(sembarang)` akan selalu tidak menampilkan apapun karena tidak ada variabel yang dibungkus dalam kurung kurawal.
- Tatkala `$all` digunakan sebagai shortcut untuk `\[$a$b\]`, `($all)` tidak akan menampilkan apapun jika nilai `$a` dan `$b` adalah `None`. Berlaku juga dengan `(\[$a$b\] )`.

#### Karakter Pelarian

Simbol-simbol berikut mempunyai kegunaan yang spesial dalam format srting. Jika kamu ingin menampilkan simbol-simbol berikut, kamu harus membebaskannya (escape) dengan garis miring terbalik (`\`).

- \$
- \\
- [
- ]
- (
- )

Perhatikan bahwa `toml` memiliki [sintaksi bebasnya sendiri](https://github.com/toml-lang/toml#user-content-string). Disarankan halnya untukmu menggunakan string literal (`''`) dalam konfigurasimu. Jika kamu mau memakai string standar (`""`), ingatlah untuk membebaskan garis miring terbalik `\`.

Sebagai contoh, ketika kamu ingin menampilkan simbol `$` pada sebuah baris baru, konfigurasi berikut sama halnya pada `format`:

```toml
# dengan string standar
format = "\n\\$"

# dengan string standar multibaris
format = """

\\$"""

# dengan string literal
format = '''

\$'''
```

## Prompt

Berikut adalah opsi konfigurasi dari list yang bersifat prompt-wide.

### Opsi

| Opsi              | Bawaan                         | Deskripsi                                                              |
| ----------------- | ------------------------------ | ---------------------------------------------------------------------- |
| `fromat`          | [link](#default-prompt-format) | Mengkonfigurasi format pada prompt.                                    |
| `scan_timeout`    | `30`                           | Batas waktu starpship untuk memindai file (dalam milidetik).           |
| `command_timeout` | `500`                          | Batas waktu untuk perintah yang dijalankan starship (dalam milidetik). |
| `add_newline`     | `true`                         | Memasukkan baris kosong antara prompt shell.                           |

### Contoh

```toml
# ~/.config/starship.toml

# Menggunakan format yang dikustomisasi
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Menunggu 10 milidetik untuk starship mencek file pada direktori terkini.
scan_timeout = 10

# Menonaktifkan baris kosong di awal prompt
add_newline = false
```

### Format Prompt Bawaan

Aturan `format` bawaan digunakan untuk mendefinisikan format pada prompt, apabila kosong atau tidak ada `format` yang diberikan. Aturannya seperti yang ditunjukkan:

```toml
format = "$all"

# yang mana ekuivalen dengan
format = """
$username\
$hostname\
$shlvl\
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
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
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
$env_var\
$crystal\
$custom\
$cmd_duration\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$shell\
$character"""
```

## AWS

Module `aws` menampilkan region dan profil AWS terkini. Diperoleh dari variabel environment `AWS_REGION`, `AWS_DEFAULT_REGION`, dan `AWS_PROFILE` pada file `~/.aws/config`. Modul ini juga menampilkan penghitung waktu mundur kedaluwarsa ketika menggunakan temporer kredensial.

Ketika menggunakan [aws-vault](https://github.com/99designs/aws-vault), profil dibaca dari variabel environment `AWS_VAULT` dan tanggal kedaluwarsanya dibaca dari variabel environment `AWS_SESSION_EXPIRATION`.

Ketika menggunakan [awsu](https://github.com/kreuzwerker/awsu) profil dibaca dari variabel environment `AWSU_PROFILE`.

Ketika menggunakan [AWSume](https://awsu.me) profil dibaca dari variabel environment `AWSUME_PROFILE` dan tanggal kedaluwarsanya dibaca dari variabel environment `AWSUME_EXPIRATION`.

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

## Baterai

Modul `battery` menampilkan seberapa penuh baterai perangkat terisi dan status pengisiannya. Modulnya hanya dapat terlihat ketika baterai perangkat di bawah 10%.

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

Opsi konfigurasi `display` digunakan untuk menentukan kapan indikator baterai harus ditampilkan (threshold), simbol mana yang akan digunakan (symbol), dan bagaimana seharusnya itu terlihat (style). Jika tidak ada `display` yang diberikan. Aturannya seperti yang ditunjukkan:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

Nilai bawaan untuk opsi `charging_symbol` dan `discharging_symbol` adalah nilai dari masing-masing opsi `charging_symbol` dan `discharging_symbol` dari nilai `battery`.

#### Opsi

Opsi dari `display` merupakan sebuah array dari tabel berikut.

| Opsi                 | Bawaan     | Deskripsi                                                                                                            |
| -------------------- | ---------- | -------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`       | The upper bound for the display option.                                                                              |
| `style`              | `bold red` | The style used if the display option is in use.                                                                      |
| `charging_symbol`    | `-`        | Simbol opsional ditampilan jika opsi tampilan sedang digunakan, bawaan untuk opsi `charging_symbol` dari baterai.    |
| `discharging_symbol` | `-`        | Simbol opsional ditampilan jika opsi tampilan sedang digunakan, bawaan untuk opsi `discharging_symbol` dari baterai. |

#### Contoh

```toml
[[battery.display]]  # "bold red" untuk corak gaya dan discharging_symbol ketika kapasitasnya berada di antara 0% dan 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" untuk corak gaya dan simbol 💦 ketika kapasitasnya berada di antara 10% dan 30%
threshold = 30
style = "bold yellow"
discharging_symbol = 💦

# ketika kapasitasnya di atas 30%, indikator baterai tidak akan ditampilkan

```

## Karakter

Modul `character` menampilkan sebuah karakter (biasanya anak panah) di samping teks pada terminalmu.

Karakter dapat memberitahu kamu apakah perintah terakhir berhasil atau tidak. Karakter dapat memberitahumu dengan dua cara ini:

- mengganti warna (`red`/`green`)
- mengganti bentuk (`❯`/`✖`)

Secara bawaan karakter hanya dapat mengganti warna. Jika kamu juga ingin mengganti bentuknya, perhatikan [contoh](#with-custom-error-shape) berikut.

::: peringatan

`error_symbol` tidak didukung pada elvish dan nu shell.

:::

::: peringatan

`vicmd_symbol` hanya didukung pada fish dan zsh.

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

Modul `cmake` menampilkan versi terkini dari [CMake](https://cmake.org/) yang terpasang. Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

## Durasi Perintah

Modul `cmd_duration` menampilkan seberapa lama perintah sebelumnya membutuhkan waktu untuk dilaksanakan. Modulnya hanya akan ditampilkan jika perintahnya membutuhkan waktu lebih dari dua detik, atau ada nilai dari konfigurasi `min_time`.

::: jangan lakukan hook DEBUG trap dalam Bash

Jia kamu menjalankan Starship di `bash`, jangan lakukan hook DEBUG trap setelah menjalankan `eval $(starship init $0)`, atau modulnya **akan** rusak.

:::

Pengguna Bash yang membutuhkan fungsi seperti preexec dapat menggunakan [kerangka kerja bash_preexec dari rcaloras](https://github.com/rcaloras/bash-preexec). Cukup dengan membuat array `preexec_functions` dan `precmd_functions` sebelum menjalankan `eval $(starship init $0)`, lalu lanjutkan seperti biasa.

### Opsi

| Opsi                 | Bawaan                        | Deskripsi                                                        |
| -------------------- | ----------------------------- | ---------------------------------------------------------------- |
| `min_tim`            | `2_000`                       | Durasi terpendek untuk menampilkan waktu (dalam milidetik).      |
| `show_milliseconds`  | `false`                       | Tampilkan milidetik sebagai ganti detik untuk durasinya.         |
| `format`             | `"took [$duration]($style) "` | Format dari modul.                                               |
| `style`              | `"bold yellow"`               | Gaya penataan untuk modul.                                       |
| `disabled`           | `false`                       | Menonaktifkan modul `cmd_duration`.                              |
| `show_notifications` | `false`                       | Menampilkan notifikasi layar ketika perintah selesai.            |
| `min_time_to_notify` | `45_000`                      | Durasi terpendek untuk menampilkan notifikasi (dalam milidetik). |

::: saran

Menampilkan notifikasi layar memerlukan starship dikembangkan dengan dukungan dari `rust-notify`. Periksa apakah starship kamu mendukung notifikasi dengan menjalankan `STARSHIP_LOG=debug starship module cmd_duration -d 60000` ketika `show_notifications` diatur menjadi `true`.

:::

### Variabel

| Variabel  | Contoh   | Deskripsi                                          |
| --------- | -------- | -------------------------------------------------- |
| duration  | `16m40s` | Waktu yang dibutuhkan untuk menyelesaikan perintah |
| style\* |          | Menyalin nilai dari opsi `style`                   |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

::: saran

Hal ini tidak menahan pengubah (modifier) prompt dari conda sendiri, kamu mungkin bisa menjalankan `conda config --set changeps1 False`.

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

Modul `crystal` menampilkan versi terkini dari [Crystal](https://crystal-lang.org/) yang terpasang. Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `shard.yml`
- Direktori terkini yang berisikan sebuah file `.cr`

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `symbol`            | `"🔮 "`                               | Simbol yang digunakan sebelum menampilkan versi crystal terkini.                    |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | Gaya penataan untuk modul.                                                          |
| `detect_extensions` | `["cr"]`                             | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["shard.yml"]`                      | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `disabled`          | `false`                              | Menonaktifkan modul `crystal`.                                                      |

### Variabel

| Variabel  | Contoh    | Deskripsi                         |
| --------- | --------- | --------------------------------- |
| version   | `v0.32.1` | Versi dari `crystal`              |
| symbol    |           | Menyalin nilai dari opsi `symbol` |
| style\* |           | Menyalin nilai dari opsi `style`  |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

Modul `dart` menampilkan versi terkini dari [Dart](https://dart.dev/) yang terpasang. Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file berekstensi `.dart`
- Direktori terkini yang berisikan sebuah direktori `dart_tool`
- Direktori terkini yang berisikan sebuah file `pubspec.yaml`, `pubspec.yml` atau `pubspec.lock`

### Opsi

| Opsi                | Bawaan                                            | Deskripsi                                                                           |
| ------------------- | ------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                       | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🎯 "`                                            | Sebuah format string yang melambangkan simbol Dart                                  |
| `detect_extensions` | `["dart"]`                                        | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[".dart_tool"]`                                  | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold blue"`                                     | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                           | Menonaktifkan modul `dart`.                                                         |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.8.4` | Versi dari `dart`                 |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

Modul `deno` menampilkan versi terkini dari [Deno](https://deno.land/) yang terpasang. Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:
- Direktori terkini yang berisikan sebuah file `mod.ts`, `mod.js`, `deps.ts` atau `deps.js`

### Opsi

| Opsi                | Bawaan                                       | Deskripsi                                                                           |
| ------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                  | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `🦕 "`                                        | Sebuah format string yang melambangkan simbol Deno                                  |
| `detect_extensions` | `[]`                                         | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["mod.ts", "mod.js", "deps.ts", "deps.js"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                         | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"green bold"`                               | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                      | Menonaktifkan modul `deno`.                                                         |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.8.3` | Versi dari `deno`                 |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

### Contoh

```toml
# ~/.config/starship.toml

[deno]
format = "via [🦕 $version](green bold) "
```

## Direktori

Modul `directory` menampilkan arah ke direkori terkinimu, disingkat ke tiga folder induk. Direkrotimu juga akan disingkat ke root dari git repo di tempatmu berada saat ini.

Ketika kamu menggunakan penataan pwd option fish, alih-alih menyembunyikan jalur yang disingkat, kamu akan melihat nama yang disingkat untuk tiap-tiap direktori berdasarkan dari jumlah nomor yang kamu aktifkan untuk opsi tersebut.

Sebagai contoh, untuk `~/Dev/Nix/nixpkgs/pkgs` dimana `nixpkgs` merupakan root repo-nya, dan lalu opsinya diset menjadi `1`. Kamu akan melihat `~/D/N/nixpkgs/pkgs`, sedangkan sebelumnya direktori tersebut harusnya `nixpkgs/pkgs`.

### Opsi

| Opsi                | Bawaan                                             | Deskripsi                                                                 |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                | Jumlah dari folder induk yang harusnya disingkat oleh direktori saat ini. |
| `truncate_to_repo`  | `true`                                             | Apakah harus menyingkat root dari git repo tempatmu berada saat ini.      |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | Format dari modul.                                                        |
| `style`             | `"bold cyan"`                                      | Gaya penataan untuk modul.                                                |
| `disabled`          | `false`                                            | Menonaktifkan modul `directory`.                                          |
| `read_only`         | `"🔒"`                                              | Simbol yang mengindikasikan direktori saat ini bersifat read only.        |
| `read_only_style`   | `"red"`                                            | Corak gaya untuk simbol read only.                                        |
| `truncation_symbol` | `""`                                               | Simbol untuk awalan jalur yang disingkat. misalnya: ".../"                |
| `home_symbol`       | `"~"`                                              | Simbol yang mengindikasikan direktori home.                               |

<details>
<summary>Modul ini memilki beberapa opsi konfigurasi lanjutan yang mengontrol bagaimana direktori ditampilkan.</summary>

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
| path      | `"D:/Projects"`       | Direktori terkini                |
| style\* | `"black bold dimmed"` | Menyalin nilai dari opsi `style` |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

Modul `docker_context` menampilkan [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) yang kini aktif jika tidak diset ke `default` atau jika variabel environment `DOCKER_HOST` atau `DOCKER_CONTEXT` telah diatur (yang seharusnya diatur untuk menimpa konteks yang digunakan).

### Opsi

| Opsi                | Bawaan                                                        | Deskripsi                                                                                 |
| ------------------- | ------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | Format dari modul.                                                                        |
| `symbol`            | `"🐳 "`                                                        | Simbol yang digunakan sebelum menampilkan Docker context.                                 |
| `only_with_files`   | `true`                                                        | Hanya ditampilkan jika terdapat kecocokan                                                 |
| `detect_extensions` | `[]`                                                          | Extensions mana yang harusnya memicu modul (butuh `only_with_files` untuk diset true).    |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | filenames mana yang harusnya memicu modul ini (butuh `only_with_files` untuk diset true). |
| `detect_folders`    | `[]`                                                          | Folder mana yang harusnya memicu modul (butuh `only_with_files` untuk diset true).        |
| `style`             | `"blue bold"`                                                 | Gaya penataan untuk modul.                                                                |
| `disabled`          | `false`                                                       | Menonaktifkan module `docket_context`.                                                    |

### Variabel

| Variabel  | Contoh         | Deskripsi                         |
| --------- | -------------- | --------------------------------- |
| context   | `test_context` | Docker context terkini            |
| symbol    |                | Menyalin nilai dari opsi `symbol` |
| style\* |                | Menyalin nilai dari opsi `style`  |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

Modul `dotnet` menampilkan informasi terkait versi dari [.NET Core SDK](https://dotnet.microsoft.com/) pada direktori terkini. Apabila SDK telah disematkan pada direktori terkni, maka veri yang telah disematkan tersebutlah yang ditampilkan. Jika tidak, maka modul akan menampilkan versi SDK terkini.

Secara bawaan, modul ini hanya akan ditampilkan ke prompt kamu ketika teradapat satu atau lebih file berikut di dalam direktorimu saat ini:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

Kamu juga perlu memasang .NET Core SDK untuk menggunakannya dengan baik.

Secara internal, modul ini menggunakan mekenasimenya sendiri untuk melakukan pendeteksian versi. Biasanya, hal ini dua kali lebih cepat seperti untuk menjalankan `dotnet --version`, tetapi ada kemungkinan hal ini akan menampilkan versi yang salah jika proyek .NET milikmu memiliki tata letak direktori yang tidak biasa. Jika menurutmu akurasi lebih penting dari kecepatan, kamu dapat menonaktifkan mekanisme tersebut dengan mengatur `heuristic = false` di dalam opsi modul.

Modul ini juga akan menampilkan Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) ketika terdapat sebuah file csproj di dalam direktori terkini.

### Opsi

| Opsi                | Bawaan                                                                                                  | Deskripsi                                                                           |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                                                                             | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | Simbol yang digunakan sebelum menampilkan versi dotnet terkini.                     |
| `heuristic`         | `true`                                                                                                  | Menggunakan versi yang lebih cepat untuk membuat starship tetap trendi.             |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                                                                                    | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold blue"`                                                                                           | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                                                                                 | Menonaktifkan modul `dotnet`.                                                       |

### Variabel

| Variabel  | Contoh           | Deskripsi                                              |
| --------- | ---------------- | ------------------------------------------------------ |
| version   | `v3.1.201`       | Versi dari sdk `dotnet`                                |
| tfm       | `netstandard2.0` | Target Framework Moniket yang tengah ditarget starship |
| symbol    |                  | Menyalin nilai dari opsi `symbol`                      |
| style\* |                  | Menyalin nilai dari opsi `style`                       |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

Modul `elixir` menampilkan versi terkini dari [Elixir](https://elixir-lang.org/) dan [Erlang/OTP](https://erlang.org/doc/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- Direktori terkini yang berisikan sebuah file `mix.exs`.

### Opsi

| Opsi                | Bawaan                                                      | Deskripsi                                                                           |
| ------------------- | ----------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | Format dari modul elixir.                                                           |
| `version_format`    | `"v${raw}"`                                                 | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | Simbol yang digunakan sebelum menampilkan versi Elixir/Erlang terkini.              |
| `detect_extensions` | `[]`                                                        | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["mix.exs"]`                                               | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                                        | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold purple"`                                             | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                                     | Menonaktifkan modul `elixir`.                                                       |

### Variabel

| Variabel    | Contoh  | Deskripsi                         |
| ----------- | ------- | --------------------------------- |
| version     | `v1.10` | Versi dari `elixir`               |
| otp_version |         | Versi otp dari `elixir`           |
| symbol      |         | Menyalin nilai dari opsi `symbol` |
| style\*   |         | Menyalin nilai dari opsi `style`  |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

Modul `elixir` menampilkan versi terkini dari [Elm](https://elm-lang.org/) yang terpasang. Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### Opsi

| Opsi                | Bawaan                                             | Deskripsi                                                                           |
| ------------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`               | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                                        | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | Sebuah format string yang melambangkan simbol Elm.                                  |
| `detect_extensions` | `["elm"]`                                          | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `["elm-stuff"]`                                    | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"cyan bold"`                                      | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                                            | Disables the `elm` module.                                                          |

### Variabel

| Variabel  | Contoh    | Deskripsi                         |
| --------- | --------- | --------------------------------- |
| version   | `v0.19.1` | Versi dari `elm`                  |
| symbol    |           | Menyalin nilai dari opsi `symbol` |
| style\* |           | Menyalin nilai dari opsi `style`  |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Variabel Environment

The `env_var` module displays the current value of a selected environment variables. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is


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
| `symbol`   |                                | The symbol used before displaying the variable value.                        |
| `variabel` |                                | The environment variable to be displayed.                                    |
| `bawaan`   |                                | The default value to be displayed when the selected variable is not defined. |
| `format`   | `"with [$env_value]($style) "` | Format dari modul.                                                           |
| `disabled` | `false`                        | Disables the `env_var` module.                                               |

### Variabel

| Variabel  | Contoh                                      | Deskripsi                                  |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Menyalin nilai dari opsi `symbol`          |
| style\* | `black bold dimmed`                         | Menyalin nilai dari opsi `style`           |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

- The current directory contains a `rebar.config` file.
- The current directory contains a `erlang.mk` file.

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
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
| active    | `bawaan`      | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |               | Menyalin nilai dari opsi `symbol`                                  |
| style\* |               | Menyalin nilai dari opsi `style`                                   |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in the current git repository.

::: saran

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

| Opsi         | Bawaan                                          | Deskripsi                           |
| ------------ | ----------------------------------------------- | ----------------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status` |
| `conflicted` | `"="`                                           | This branch has merge conflicts.    |
| `ahead`      | `"⇡"`                                           | The format of `ahead`               |
| `behind`     | `"⇣"`                                           | The format of `behind`              |
| `diverged`   | `"⇕"`                                           | The format of `diverged`            |
| `up_to_date` | `""`                                            | The format of `up_to_date`          |
| `untracked`  | `"?"`                                           | The format of `untracked`           |
| `stashed`    | `"$"`                                           | The format of `stashed`             |
| `modified`   | `"!"`                                           | The format of `modified`            |
| `staged`     | `"+"`                                           | The format of `staged`              |
| `renamed`    | `"»"`                                           | The format of `renamed`             |
| `deleted`    | `"✘"`                                           | The format of `deleted`             |
| `style`      | `"bold red"`                                    | Gaya penataan untuk modul.          |
| `disabled`   | `false`                                         | Disables the `git_status` module.   |

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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
untracked = "🤷‍"
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

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `helmfile.yaml` file
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

| Variabel  | Contoh | Deskripsi                         |
| --------- | ------ | --------------------------------- |
| symbol    |        | Menyalin nilai dari opsi `symbol` |
| style\* |        | Menyalin nilai dari opsi `style`  |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there are at least 2 jobs, or more than the `number_threshold` config value, if it exists. The module will show a symbol if there is at least 1 job, or more than the `symbol_threshold` config value, if it exists. You can set both values to 0 in order to *always* show the symbol and number of jobs, even if there are 0 jobs running.

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
| `threshold`\*    | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `"[$symbol$number]($style) "` | Format dari modul.                                                       |
| `symbol`           | `"✦"`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `"bold blue"`                 | Gaya penataan untuk modul.                                               |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |
 \*: This option is deprecated, please use the 

`number_threshold` and `symbol_threshold` options instead.

### Variabel

| Variabel  | Contoh | Deskripsi                         |
| --------- | ------ | --------------------------------- |
| number    | `1`    | The number of jobs                |
| symbol    |        | Menyalin nilai dari opsi `symbol` |
| style\* |        | Menyalin nilai dari opsi `style`  |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

::: saran

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: saran

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

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul                                                                   |
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

- The current directory contains a `package.json` file
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
| `symbol`            | `" "`                               | A format string representing the symbol of Node.js.                                                   |
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

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
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

- The current directory contains a `composer.json` file
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `spago.dhall` file
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
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

::: saran

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | Format dari modul.                                                                  |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                                    |
| `detect_extensions` | `["rb"]`                             | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[]`                                 | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold red"`                         | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `ruby` module.                                                         |

### Variabel

| Variabel  | Contoh   | Deskripsi                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.5.1` | The version of `ruby`             |
| symbol    |          | Menyalin nilai dari opsi `symbol` |
| style\* |          | Menyalin nilai dari opsi `style`  |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: saran

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opsi

| Opsi                   | Bawaan       | Deskripsi                                                    |
| ---------------------- | ------------ | ------------------------------------------------------------ |
| `bash_indicator`       | `bsh`        | A format string used to represent bash.                      |
| `fish_indicator`       | `fsh`        | A format string used to represent fish.                      |
| `zsh_indicator`        | `zsh`        | A format string used to represent zsh.                       |
| `powershell_indicator` | `psh`        | A format string used to represent powershell.                |
| `ion_indicator`        | `ion`        | A format string used to represent ion.                       |
| `elvish_indicator`     | `esh`        | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `tsh`        | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `xsh`        | A format string used to represent xonsh.                     |
| `unknown_indicator`    |              | The default value to be displayed when the shell is unknown. |
| `format`               | `$indicator` | Format dari modul.                                           |
| `disabled`             | `true`       | Disables the `shell` module.                                 |

### Variabel

| Variabel  | Bawaan | Deskripsi                                                  |
| --------- | ------ | ---------------------------------------------------------- |
| indicator |        | Mirrors the value of `indicator` for currently used shell. |

### Contoh

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "mystery shell"
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: saran

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

::: warning This module is not supported on elvish and nu shell. :::

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
| `pipestatus_separator`  | `|`                                                                                  | The symbol that separate in pipe program exit codes     |
| `pipestatus_format`     | `\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)` | The format of the module when the command is a pipeline |
| `disabled`              | `true`                                                                               | Disables the `status` module.                           |

### Variabel

| Variabel       | Contoh  | Deskripsi                                                                                   |
| -------------- | ------- | ------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                           |
| int            | `127`   | The exit code of the last command                                                           |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                         |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                             |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                        |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                |
| pipestatus     |         | Rendering of in pipeline programs's exit codes, this is only available in pipestatus_format |
| symbol         |         | Menyalin nilai dari opsi `symbol`                                                           |
| style\*      |         | Menyalin nilai dari opsi `style`                                                            |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.

::: saran

By default the Terraform version is not shown, since this is slow for current versions of Terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-terraform-version).

:::

Secara bawaan, modul akan aktif jika beberapa syarat berikut telah terpenuhi:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` or `.hcl` extensions

### Opsi

| Opsi                | Bawaan                               | Deskripsi                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                                                   |
| `version_format`    | `"v${raw}"`                          | Format dari versi. Variabel yang tersedia adalah `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.                               |
| `detect_extensions` | `["tf", "hcl"]`                      | Ekstensi mana yang sebaiknya memicu modul ini.                                      |
| `detect_files`      | `[]`                                 | filenames mana yang sebaiknya memicu modul ini.                                     |
| `detect_folders`    | `[".terraform"]`                     | Folder mana yang sebaiknya memicul modul ini.                                       |
| `style`             | `"bold 105"`                         | Gaya penataan untuk modul.                                                          |
| `disabled`          | `false`                              | Disables the `terraform` module.                                                    |

### Variabel

| Variabel  | Contoh     | Deskripsi                         |
| --------- | ---------- | --------------------------------- |
| version   | `v0.12.24` | The version of `terraform`        |
| workspace | `bawaan`   | The current Terraform workspace   |
| symbol    |            | Menyalin nilai dari opsi `symbol` |
| style\* |            | Menyalin nilai dari opsi `style`  |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

::: saran

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

::: saran

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

### Contoh

```toml
# ~/.config/starship.toml

[vcsh]
format = "via [✨ $repo](bold blue) "
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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

::: saran

Multiple custom modules can be defined by using a `.`.

:::

::: saran

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: saran

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

### Opsi

| Opsi          | Bawaan                          | Deskripsi                                                                                                                                                                     |
| ------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                                 | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                 |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code.                                                    |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                                                                            |
| `deskripsi`   | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                                                                                  |
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

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

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
command = "echo foo"  # affiche la sortie de la commande
files = ["foo"]       # ajoute un filtre
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
