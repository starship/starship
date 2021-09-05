# Migrasi ke v0.45.0

Starship v0.45.0 merupakan rilisan dengan perubahan yang signifikan, untuk persiapan v1.0.0 yang besar. Kami membuat beberapa perubahan besar tentang bagaimana konfigurasi dilakukan pada prompt, hingga bagaimana kami mengizinkan tingkat kustomisasi yang lebih luas.

Petunjuk berikut memandu kamu ke perubahan besar kami.

## `prompt_order` kini digantikan dengan sebuah `format` root-level

Sebelum v0.45.0, `prompt_order` dapat menerima sebuah nama modul dengan urutan yang harusya di-render oleh Starship.

Starship v0.45.0 kini menerima nilai dari `format`, memungkinkan kustomisasi prompt di luar modul itu sendiri.

**Contoh konfigurasi pra-v0.45.0**

```toml
prompt_order = [
  "username",
  "hostname",
  "directory",
  "git_branch",
  "git_commit",
  "git_state",
  "git_status",
  "cmd_duration",
  "custom",
  "line_break",
  "jobs",
  "battery",
  "time",
  "character",
]
```

**Contoh konfigurasi v0.45.0**

```toml
format = """\
  $username\
  $hostname\
  $directory\
  $git_branch\
  $git_commit\
  $git_state\
  $git_status\
  $cmd_duration\
  $custom\
  $line_break\
  $jobs\
  $battery\
  $time\
  $character\
  """
```

## Modul `prefix` dan `suffix` kini digantikan dengan `format`

Sebelum v0.45.0, beberapa modul bisa menerima `prefix` dan/atau `suffix` untuk menata gayanya sesuai yang di-render modul.

Starship v0.45.0 menerima nilai `format` sebagai gantinya, memungkinkan bagaimana modul di-render untuk kustomisasi yang lebih jauh. Ketimbang membuat prefix dan suffix untuk varibel berbasis konteks, kini variabel dapat disubtitusikan dari sebuah format string, yang mana merepresantikan keluaran dari sebuah modul.

**Contoh konfigurasi pra-v0.45.0**

```toml
[cmd_duration]
prefix = "took "
```

**Contoh konfigurasi v0.45.0**

```toml
[cmd_duration]
# $duration – The command duration (e.g. "15s")
# $style    – The default style of the module (e.g. "bold yellow")
format = "took [$duration]($style) "
```

### Affected Modules

#### Karakter

| Removed Property        | Penggantinya     |
| ----------------------- | ---------------- |
| `symbol`                | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**Changes to the Default Configuration**

```diff
[character]
-- symbol = "❯"
-- error_symbol = "✖"
-- use_symbol_for_status = true
-- vicmd_symbol = "❮"
++ success_symbol = "[❯](bold green)"
++ error_symbol = "[❯](bold red)"
++ vicmd_symbol = "[❮](bold green)"
```

Previously, the `use_symbol_for_status` property was used to configure the prompt to show the `error_symbol` when the last command resulted in a non-zero status code.

With the release of v0.45.0, we now always use `error_symbol` after non-zero status codes, unifying `use_symbol_for_status` and `error_symbol` properties.

To configure the prompt to use the older `use_symbol_for_status = true` configuration, add the following to your config file:

```toml
[character]
error_symbol = "[✖](bold red)"
```

*Note:* The `character` element automatically adds a space after, so unlike the other `format` strings, we specifically do not add one in the above examples.

#### Durasi Perintah

| Removed Property | Penggantinya |
| ---------------- | ------------ |
| `prefix`         | `format`     |

**Changes to the Default Configuration**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### Directory

| Removed Property | Penggantinya |
| ---------------- | ------------ |
| `prefix`         | `format`     |

**Changes to the Default Configuration**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### Environment Variable

| Removed Property | Penggantinya |
| ---------------- | ------------ |
| `prefix`         | `format`     |
| `suffix`         | `format`     |

**Changes to the Default Configuration**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git Commit

| Removed Property | Penggantinya |
| ---------------- | ------------ |
| `prefix`         | `format`     |
| `suffix`         | `format`     |

**Changes to the Default Configuration**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\($hash\)]($style) '
```

#### Git Status

| Removed Property  | Penggantinya |
| ----------------- | ------------ |
| `prefix`          | `format`     |
| `suffix`          | `format`     |
| `show_sync_count` | `format`     |

**Changes to the Default Configuration**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\[$all_status$ahead_behind\]]($style) )'
```

Previously, the `show_sync_count` property was used to configure the prompt to show the number of commits the branch was ahead or behind the remote branch.

With the release of v0.45.0, this has been replaced with three separate properties, `ahead`, `behind`, and `diverged`.

To configure the prompt to use the older `show_sync_count = true` configuration, set the following to your config file:

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### Hostname

| Removed Property | Penggantinya |
| ---------------- | ------------ |
| `prefix`         | `format`     |
| `suffix`         | `format`     |

**Changes to the Default Configuration**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| Removed Property | Penggantinya |
| ---------------- | ------------ |
| `label`          | `format`     |
| `prefix`         | `format`     |
| `suffix`         | `format`     |

**Changes to the Default Configuration**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\[$env\]]($style) '
```

#### Time

| Removed Property | Penggantinya  |
| ---------------- | ------------- |
| `format`         | `time_format` |

**Changes to the Default Configuration**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### Custom Commands

| Removed Property | Penggantinya |
| ---------------- | ------------ |
| `prefix`         | `format`     |
| `suffix`         | `format`     |

**Changes to the Default Configuration**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
