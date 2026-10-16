# Konfigurasi Lanjutan

While Starship is a versatile shell, sometimes you need to do more than edit
`starship.toml` to get it to do certain things. Halaman ini merincikan beberapa teknik konfigurasi lanjutan yang digunakan starship.

> [!WARNING] The configurations in this section are subject to change in future releases of Starship.

## TransientPrompt in PowerShell

It is possible to replace the previous-printed prompt with a custom string. This
is useful in cases where all the prompt information is not always needed. To enable
this, run `Enable-TransientPrompt` in the shell session. To make it permanent, put
this statement in your `$PROFILE`. Transience can be disabled on-the-fly with
`Disable-TransientPrompt`.

By default, the left side of input gets replaced with `>`. To customize this,
define a new function called `Invoke-Starship-TransientFunction`. For example, to
display Starship's `character` module here, you would do

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt and TransientRightPrompt in Cmd

Clink allows you to replace the previous-printed prompt with custom strings. This
is useful in cases where all the prompt information is not always needed. To enable
this, run `clink set prompt.transient <value>` where \<value\> can be one of:

- `always`: always replace the previous prompt
- `same_dir`: replace the previous prompt only if the working directory is same
- `off`: do not replace the prompt (i.e. turn off transience)

You need to do this only once. Make the following changes to your `starship.lua`
to customize what gets displayed on the left and on the right:

- By default, the left side of input gets replaced with `>`. To customize this,
  define a new function called `starship_transient_prompt_func`. This function
  receives the current prompt as a string that you can utilize. For example, to
  display Starship's `character` module here, you would do

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- By default, the right side of input is empty. To customize this, define a new
  function called `starship_transient_rprompt_func`. This function receives the
  current prompt as a string that you can utilize. For example, to display
  the time at which the last command was started here, you would do

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt and TransientRightPrompt in Fish

It is possible to replace the previous-printed prompt with a custom string. This
is useful in cases where all the prompt information is not always needed. To enable
this, run `enable_transience` in the shell session. To make it permanent, put
this statement in your `~/.config/fish/config.fish`. Transience can be disabled on-the-fly with
`disable_transience`.

Note that in case of Fish, the transient prompt is only printed if the commandline is non-empty, and syntactically correct.

- By default, the left side of input gets replaced with a bold-green `❯`. To customize this,
  define a new function called `starship_transient_prompt_func`. For example, to
  display Starship's `character` module here, you would do

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- By default, the right side of input is empty. To customize this, define a new
  function called `starship_transient_rprompt_func`. For example, to display
  the time at which the last command was started here, you would do

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt and TransientRightPrompt in Bash

The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework at v0.4 or higher allows you to replace
the previous-printed prompt with custom strings. This is useful in cases where all
the prompt information is not always needed. To enable this, put this in `~/.bashrc`
`bleopt prompt_ps1_transient=<value>`:

The \<value\> here is a colon-separated list of `always`, `same-dir` and `trim`.
When `prompt_ps1_final` is empty and the option `prompt_ps1_transient` has a non-empty \<value\>,
the prompt specified by `PS1` is erased on leaving the current command line.
If \<value\> contains a field `trim`, only the last line of multiline `PS1` is
preserved and the other lines are erased. Otherwise, the command line will be
redrawn as if `PS1=` is specified. When a field `same-dir` is contained in
\<value\> and the current working directory is different from the final directory of
the previous command line, this option `prompt_ps1_transient` is ignored.

Make the following changes to your `~/.blerc` (or in `~/.config/blesh/init.sh`) to customize what gets displayed on the left and on the right:

- To customize what the left side of input gets replaced with, configure the
  `prompt_ps1_final` Ble.sh option. For example, to display Starship's `character`
  module here, you would do

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- To customize what the right side of input gets replaced with, configure the
  `prompt_rps1_final` Ble.sh option. For example, to display
  the time at which the last command was started here, you would do

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Kustomisasi Perintah pre-prompt dan pre-execution Pada Cmd

Clink menyediakan APIs yang sangat fleksibel untuk menjalankan perintah pre-prompt dan pre-exec di Cmd shell. Caranya sangat mudah dengan Starship. Make the following changes
to your `starship.lua` file as per your requirements:

- To run a custom function right before the prompt is drawn, define a new
  function called `starship_preprompt_user_func`. This function receives
  the current prompt as a string that you can utilize. For example, to
  draw a rocket before the prompt, you would do

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- To run a custom function right before a command is executed, define a new
  function called `starship_precmd_user_func`. This function receives
  the current commandline as a string that you can utilize. For example, to
  print the command that's about to be executed, you would do

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Kustomisasi Perintah pre-prompt dan pre-execution Pada Bash

Bash tidak memiliki framework preexec/precmd yang tetap seperti kebanyakan shell pada umumnya.
Because of this, it is difficult to provide fully customizable hooks in `bash`.
Namun, Starship memberimu sedikit kemampuan untuk bisa menambahkan function milikmu ke dalam prosedur prompt-rendering:

- To run a custom function right before the prompt is drawn, define a new
  function and then assign its name to `starship_precmd_user_func`. For example,
  to draw a rocket before the prompt, you would do

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- To run a custom function right before a command runs, you can use the
  [`DEBUG` trap mechanism](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/).
  However, you **must** trap the DEBUG signal _before_ initializing Starship!
  Starship can preserve the value of the DEBUG trap, but if the trap is overwritten
  after starship starts up, some functionality will break.

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

PowerShell tidak memiliki framework preecex/precmd seperti kebanyak shells pada umumnya.
Because of this, it is difficult to provide fully customizable hooks in `powershell`.
Namun, Starship memberimu sedikit kemampuan untuk bisa menambahkan function milikmu ke dalam prosedur prompt-rendering:

Buatlah sebuah funciton dengan nama `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Mengubah Judul Window

Beberapa prompt shell dengan otomatis akan mengubah judul window-nya untukmu (mis. untuk merefleksikan direktori kerjamu). Fish bahkan mengaturnya sebagai bawaan.
Starship does not do this, but it's fairly straightforward to add this
functionality to `bash`, `zsh`, `cmd` or `powershell`.

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
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Mengaktifkan Right Prompt

Sebagian shells mendukung right prompt yang mana dirender di baris yang sama sesuai dengan masukannya. Starship can
set the content of the right prompt using the `right_format` option. Any module that can be used
in `format` is also supported in `right_format`. The `$all` variable will only contain modules
not explicitly used in either `format` or `right_format`.

Catatan: Right propmt merupakan sebuah baris yang mengikuti lokasi baris inputan. To right align modules above
the input line in a multi-line prompt, see the [`fill` module](../config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Note: The [Ble.sh](https://github.com/akinomyoga/ble.sh) framework v0.4 or higher should be installed in order to use right prompt in bash.

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

When using `zsh` (v5.0.5+), the shell adds a default trailing space to the right prompt. This can cause alignment issues specifically when using the Starship `$fill` module. To remove this gap, add the following to your `.zshrc`:

```zsh
ZLE_RPROMPT_INDENT=0
```

## Prompt Berkelanjutan

Beberapa shells mendukung continuation prompt bersamaan dengan prompt biasa. Prompt tersebutlah yang akan dirender daripada prompt biasa ketika pengguna memasukkan perintah yang kurang lengkap (seperti tanda kurung atau tanda kutipan tunggal).

Starship can set the continuation prompt using the `continuation_prompt` option. The default prompt is `'[∙](bright-black) '`.

Note: `continuation_prompt` should be set to a literal string without any variables.

Catatan: Continuation prompts hanya tersedia pada beberapa shells berikut:

- `bash`
- `zsh`
- `PowerShell`

### Contoh

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled-in arrows
continuation_prompt = '▶▶ '
```

## Statusline for Claude Code

Starship supports displaying a custom statusline when running inside Claude Code, Anthropic's CLI tool for interactive coding with Claude. This statusline provides real-time information about your Claude session, including the model being used, context window usage, and session costs.

For more information about the Claude Code statusline feature, see the [Claude Code statusline documentation](https://code.claude.com/docs/en/statusline).

### Setup

To use Starship as your Claude Code statusline:

1. Run `/statusline` in Claude Code and ask it to configure Starship, or manually add the following to your `.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "starship statusline claude-code"
  }
}
```

2. Customize the statusline appearance in your `~/.config/starship.toml` (see [Configuration](#configuration) below)

### Overview

When invoked with `starship statusline claude-code`, Starship receives Claude Code session data via stdin and renders a statusline using a dedicated profile named `claude-code`.

The profile includes three specialized modules:

- `claude_model`: Displays the current Claude model being used
- `claude_context`: Shows context window usage with a visual gauge
- `claude_cost`: Displays session cost and statistics

The default profile format is:

```toml
[profiles]
claude-code = "$claude_model$git_branch$claude_context$claude_cost"
```

### Konfigurasi

You can customize the Claude Code statusline by modifying the `claude-code` profile and individual module configurations in your `~/.config/starship.toml`:

```toml
# ~/.config/starship.toml

# Customize the claude-code profile
[profiles]
claude-code = "$claude_model$claude_context$claude_cost"

# Configure individual modules
[claude_model]
format = "[$symbol$model]($style) "
symbol = "🤖 "
style = "bold blue"

[claude_context]
format = "[$gauge $percentage]($style) "
gauge_width = 10

[claude_cost]
format = "[$symbol$cost]($style) "
symbol = "💰 "
```

### Claude Model

The `claude_model` module displays the current Claude model being used in the session.

#### Variabel

| Variabel        | Contoh                       | Deskripsi                                                                                                                 |
| --------------- | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `format`        | `'[$symbol$model]($style) '` | Format dari modul.                                                                                        |
| `symbol`        | `'🤖 '`                      | The symbol shown before the model name.                                                                   |
| `style`         | `'bold blue'`                | Menyalin nilai dari opsi `symbol`                                                                                         |
| `model_aliases` | `{}`                         | Map of model IDs or display names to shorter aliases. Checks ID first, then display name. |
| `disabled`      | `false`                      | Disables the `claude_model` module.                                                                       |

#### Variabel

| Variabel                      | Contoh              | Deskripsi                             |
| ----------------------------- | ------------------- | ------------------------------------- |
| model                         | `Claude 3.5 Sonnet` | The display name of the current model |
| model_id | `claude-3-5-sonnet` | The model ID                          |
| symbol                        |                     | Menyalin nilai dari opsi `style`      |
| style\*                       |                     | Menyalin nilai dari opsi `style`      |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

#### Contoh

```toml
# ~/.config/starship.toml

# Basic customization
[claude_model]
format = "on [$symbol$model]($style) "
symbol = "🧠 "
style = "bold cyan"

# Using model aliases for vendor-specific model names
# You can alias by model ID or display name
[claude_model.model_aliases]
# Alias by vendor model ID (e.g. AWS Bedrock)
"global.anthropic.claude-sonnet-4-5-20250929-v1:0" = "Sonnet 4.5"
# Alias by display name
"Claude Sonnet 4.5 (Vendor Proxy)" = "Sonnet"
```

### Claude Context

The `claude_context` module displays context window usage as a percentage and visual gauge. The style automatically changes based on configurable thresholds.

#### Opsi

| Opsi                   | Bawaan                            | Deskripsi                                                          |
| ---------------------- | --------------------------------- | ------------------------------------------------------------------ |
| `format`               | `'[$gauge $percentage]($style) '` | Format dari modul.                                 |
| `symbol`               | `''`                              | The symbol shown before the gauge.                 |
| `gauge_width`          | `5`                               | The width of the gauge in characters.              |
| `gauge_full_symbol`    | `'█'`                             | The symbol used for filled segments of the gauge.  |
| `gauge_partial_symbol` | `'▒'`                             | The symbol used for partial segments of the gauge. |
| `gauge_empty_symbol`   | `'░'`                             | The symbol used for empty segments of the gauge.   |
| `display`              | [see below](#display)             | Threshold and style configurations.                |
| `disabled`             | `false`                           | Disables the `claude_context` module.              |

##### Display

The `display` option is an array of objects that define thresholds and styles for different usage levels. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Opsi        | Bawaan       | Deskripsi                                                                |
| ----------- | ------------ | ------------------------------------------------------------------------ |
| `threshold` | `0.0`        | The minimum context windows usage percentage to match this configuration |
| `style`     | `bold green` | The value of `style` if this display configuration is matched            |
| `hidden`    | `false`      | Hide this module if this the configuration is matched.   |

```toml
[[claude_context.display]]
threshold = 0
hidden = true

[[claude_context.display]]
threshold = 30
style = "bold green"

[[claude_context.display]]
threshold = 60
style = "bold yellow"

[[claude_context.display]]
threshold = 80
style = "bold red"
```

#### Variabel

| Variabel                                                                                  | Contoh  | Deskripsi                                             |
| ----------------------------------------------------------------------------------------- | ------- | ----------------------------------------------------- |
| gauge                                                                                     | `██▒░░` | Visual representation of context usage                |
| percentage                                                                                | `65%`   | Context usage as a percentage                         |
| input_tokens                                                         | `45.2k` | Total input tokens in conversation                    |
| output_tokens                                                        | `12.3k` | Total output tokens in conversation                   |
| curr_input_tokens                               | `5.1k`  | Input tokens from most recent API call                |
| curr_output_tokens                              | `1.2k`  | Output tokens from most recent API call               |
| curr_cache_creation_tokens | `1.5k`  | Cache creation tokens from most recent API call       |
| curr_cache_read_tokens     | `23.4k` | Cache read tokens from most recent API call           |
| total_tokens                                                         | `200k`  | Total context window size                             |
| symbol                                                                                    |         | Menyalin nilai dari opsi `style`                      |
| style\*                                                                                   |         | Mirrors the style from the matching display threshold |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

#### Contoh

**Minimal gauge-only display**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$gauge]($style) "
gauge_width = 10
```

**Detailed token information**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$percentage ($input_tokens in / $output_tokens out)]($style) "
```

**Custom gauge symbols**

```toml
# ~/.config/starship.toml

[claude_context]
gauge_full_symbol = "▰"
gauge_partial_symbol = ""
gauge_empty_symbol = "▱"
gauge_width = 10
format = "[$gauge]($style) "
```

**Custom thresholds**

```toml
# ~/.config/starship.toml

[[claude_context.display]]
threshold = 0
style = "bold green"

[[claude_context.display]]
threshold = 50
style = "bold yellow"

[[claude_context.display]]
threshold = 75
style = "bold orange"

[[claude_context.display]]
threshold = 90
style = "bold red"
```

### Claude Cost

The `claude_cost` module displays the total cost of the current Claude Code session in USD. Like `claude_context`, it supports threshold-based styling.

#### Opsi

| Opsi       | Bawaan                             | Deskripsi                                           |
| ---------- | ---------------------------------- | --------------------------------------------------- |
| `format`   | `'[$symbol(\\$$cost)]($style) '` | Format dari modul.                  |
| `symbol`   | `'💰 '`                            | The symbol shown before the cost.   |
| `display`  | [see below](#display-1)            | Threshold and style configurations. |
| `disabled` | `false`                            | Disables the `claude_cost` module.  |

##### Display

The `display` option is an array of objects that define cost thresholds and styles. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Opsi        | Bawaan       | Deskripsi                                                          |
| ----------- | ------------ | ------------------------------------------------------------------ |
| `threshold` | `0.0`        | The minimum cost in USD to match this configuration                |
| `style`     | `bold green` | The value of `style` if this display configuration is matched      |
| `hidden`    | `false`      | Hide this module if this configuration is matched. |

**Default configuration:**

```toml
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 1.0
style = "bold yellow"

[[claude_cost.display]]
threshold = 5.0
style = "bold red"
```

#### Variabel

| Variabel                           | Contoh   | Deskripsi                                                              |
| ---------------------------------- | -------- | ---------------------------------------------------------------------- |
| cost                               | `1.23`   | Total session cost in USD (formatted to 2 decimals) |
| duration                           | `1m 30s` | Total session duration                                                 |
| api_duration  | `45s`    | Total API call duration                                                |
| lines_added   | `1.2k`   | Total lines of code added                                              |
| lines_removed | `500`    | Total lines of code removed                                            |
| symbol                             |          | Menyalin nilai dari opsi `style`                                       |
| style\*                            |          | Mirrors the style from the matching display threshold                  |

\*: Variabel tersebut hanya dapat digunakan sebagai bagian dari penataan string

#### Penataan String

```toml
# ~/.config/starship.toml

# Cost with code change statistics
[claude_cost]
format = "[$symbol$cost (+$lines_added -$lines_removed)]($style) "

# Hide module until cost exceeds $0.10
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 0.10
style = "bold yellow"

[[claude_cost.display]]
threshold = 2.0
style = "bold red"

# Show duration information
[claude_cost]
format = "[$symbol$cost ($duration)]($style) "
```

## Penataan String

Penataan string adalah kumpulan kata-kata, yang dipisahkan oleh ruang kosong. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Tiap-tiap kata berikut adalah opsinya:

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

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future.
`<color>` can also be set to `prev_fg` or `prev_bg` which evaluates to the previous item's foreground or background color respectively if available or `none` otherwise.
`inverted` swaps the background and foreground colors. Urutan kata pada string tidak jadi masalah.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

Jika warna yang dipakai pada latar depan/latar belakang banyak, maka warna yang terbaru pada string yang akan diprioritaskan.

- One of the standard terminal colors: `black`, `red`, `green`, `blue`,
  `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these
  with `bright-` to get the bright version (e.g. `bright-white`).
- A `#` followed by a six-digit hexadecimal number. This specifies an
  [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Menggunakan bilangan antara 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

Jika warna yang dipakai pada latar depan/latar belakang banyak, maka warna yang terbaru pada string yang akan diprioritaskan.

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default.
- `hidden` is [not supported on iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app.
