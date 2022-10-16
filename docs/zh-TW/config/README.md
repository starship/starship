# 設定

要開始設定 Starship，請先建立檔案： `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

所有關於 Starship 的設定都在這個 [TOML](https://github.com/toml-lang/toml) 檔案內：

```toml
# Get editor completions based on the config schema
"$schema" = 'https://starship.rs/config-schema.json'

# Inserts a blank line between shell prompts
add_newline = true

# Replace the "❯" symbol in the prompt with "➜"
[character] # The name of the module we are configuring is "character"
success_symbol = "[➜](bold green)" # The "success_symbol" segment is being set to "➜" with the color "bold green"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

你可以透過更改環境變數 `STARSHIP_CONFIG` 來變更設定檔存放的位置：

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

等同於在 PowersShell（Windows）的 `$PROFILE` 中添加下列文字：

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

等同於在 PowersShell（Windows）的 `$PROFILE` 中添加下列文字：

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### 術語

**模組 (Module)**： 提示字元中的一個元件，基於你的作業系統提供的背景資訊來提供訊息。 舉例來說，如果你現在的資料夾是一個 Node.js 專案，"nodejs" 模組會顯示出現在安裝在你的電腦上的 Node.js 版本。

變數 (**Variable**)：模組的子元件，主要是由模組提供的訊息。 舉例來說，在 "nodejs" 模組中提供的 "version" 變數代表著當下使用的 Node.js 版本。

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Format Strings

Format strings are the format that a module prints all its variables with. Most modules have an entry called `format` that configures the display format of the module. You can use texts, variables and text groups in a format string.

#### 變數

A variable contains a `$` symbol followed by the name of the variable. The name of a variable can only contain letters, numbers and `_`.

For example:

- `$version` is a format string with a variable named `version`.
- `$git_branch$git_commit` is a format string with two variables named `git_branch` and `git_commit`.
- `$git_branch $git_commit` has the two variables separated with a space.

#### Text Group

A text group is made up of two different parts.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings). You can add texts, variables, or even nested text groups in it.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). This can be used to style the first part.

For example:

- `[on](red bold)` will print a string `on` with bold text colored red.
- `[⌘ $version](bold green)` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `[a [b](red) c](green)` will print `a b c` with `b` red, and `a` and `c` green.

#### 風格字串

Starship 內大多數的模組允許你設定他們的顯示風格。 這要透過一個條目 (通常叫做 `style`)，這個條目使用一個字串來進行設定。 這裡給幾個風格字串的例子，以及這些字串的功用。 對於完整語法的詳細說明，請參照 [進階設定指南](/advanced-config/)。

- `"fg:green bg:blue"` 在一個藍色背景上設定綠色文字
- `"bg:blue fg:bright-green"` 在一個藍色背景上設定亮綠色文字
- `"bold fg:27"` 設定具有 [ANSI 顏色](https://i.stack.imgur.com/KTSQa.png) 27 號的粗體文字
- `"underline bg:#bf5700"` 在一個燒橙色背景上設定有底線的文字
- `"bold italic fg:purple"` 設定粗體、斜體且紫色的文字
- `""` 明確地關閉所有風格

注意風格產出的樣子取決於你的終端機模擬器。 例如，有些終端機模擬器會提升顏色的亮度而不是讓文字變粗體，而且有些色彩主題對一般與加亮顏色使用的是相同色碼。 除此之外，為了要有斜體字，你的終端機一定要支援斜體。

#### Conditional Format Strings

A conditional format string wrapped in `(` and `)` will not render if all variables inside are empty.

For example:

- `(@$region)` will show nothing if the variable `region` is `None` or empty string, otherwise `@` followed by the value of region.
- `(some text)` will always show nothing since there are no variables wrapped in the braces.
- When `$all` is a shortcut for `\[$a$b\]`, `($all)` will show nothing only if `$a` and `$b` are both `None`. This works the same as `(\[$a$b\] )`.

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

### Negative matching

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. These take lists of strings to match or not match. "Negative" options, those which should not be matched, are indicated with a leading "!" character. The presence of _any_ negative indicator in the directory will result in the module not being matched.

Extensions are matched against both the characters after the last dot in a filename, and the characters after the first dot in a filename. For example, `foo.bar.tar.gz` will be matched against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Files whose name begins with a dot are not considered to have extensions at all.

To see how this works in practice, you could match TypeScript but not MPEG Transport Stream files thus:

```toml
detect_extensions = ["ts", "!video.ts", "!audio.ts"]
```

## 提示字元

以下是針對提示字元內容的設定。

### 選項

| Option            | 預設                           | 說明                                                                                                                                                                               |
| ----------------- | ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [連結](#default-prompt-format) | Configure the format of the prompt.                                                                                                                                              |
| `right_format`    | `""`                         | See [Enable Right Prompt](/advanced-config/#enable-right-prompt)                                                                                                                 |
| `scan_timeout`    | `30`                         | Timeout for starship to scan files (in milliseconds).                                                                                                                            |
| `command_timeout` | `500`                        | Timeout for commands executed by starship (in milliseconds).                                                                                                                     |
| `add_newline`     | `true`                       | Inserts blank line between shell prompts.                                                                                                                                        |
| `palette`         | `""`                         | Sets which color palette from `palettes` to use.                                                                                                                                 |
| `palettes`        | `{}`                         | Collection of color palettes that assign [colors](/advanced-config/#style-strings) to user-defined names. Note that color palettes cannot reference their own color definitions. |

### 範例

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

# Set "foo" as custom color palette
palette = "foo"

# Define custom colors
[palettes.foo]
# Overwrite existing color
blue = "21"
# Define new color
mustard = "#af8700"
```

### Default Prompt Format

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. 預設如下：

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
$c\
$cmake\
$cobol\
$daml\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$haskell\
$helm\
$java\
$julia\
$kotlin\
$lua\
$nim\
$nodejs\
$ocaml\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$raku\
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
$buf\
$nix_shell\
$conda\
$meson\
$spack\
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
$container\
$shell\
$character"""
```

If you just want to extend the default format, you can use `$all`; modules you explicitly add to the format will not be duplicated. Eg.

```toml
# Move the directory to the second line
format = "$all$directory$character"
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials. The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process` or `sso_start_url` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice. If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var and the credentials expiration date is read from the `AWS_SESSION_EXPIRATION` env var.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile is read from the `AWSUME_PROFILE` env var and the credentials expiration date is read from the `AWSUME_EXPIRATION` env var.

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials` falls back to the `x_security_token_expires` key.

### 選項

| Option              | 預設                                                                    | 說明                                                                                                          |
| ------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | The format for the module.                                                                                  |
| `symbol`            | `"☁️ "`                                                               | 顯示在目前 AWS 配置之前的符號。                                                                                          |
| `region_aliases`    |                                                                       | 除了AWS名稱外，顯示區域別名表                                                                                            |
| `profile_aliases`   |                                                                       | Table of profile aliases to display in addition to the AWS name.                                            |
| `style`             | `"bold yellow"`                                                       | 這個模組的風格。                                                                                                    |
| `expiration_symbol` | `X`                                                                   | The symbol displayed when the temporary credentials have expired.                                           |
| `disabled`          | `false`                                                               | 停用 `AWS` 模組。                                                                                                |
| `force_display`     | `false`                                                               | If `true` displays info even if `credentials`, `credential_process` or `sso_start_url` have not been setup. |

### Variables

| 變數        | 範例               | 說明                                          |
| --------- | ---------------- | ------------------------------------------- |
| region    | `ap-northeast-1` | The current AWS region                      |
| profile   | `astronauts`     | The current AWS profile                     |
| duration  | `2h27m20s`       | The temporary credentials validity duration |
| symbol    |                  | Mirrors the value of option `symbol`        |
| style\* |                  | Mirrors the value of option `style`         |

*: This variable can only be used as a part of a style string

### Examples

#### Display everything

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
[aws.profile_aliases]
CompanyGroupFrobozzOnCallAccess = 'Frobozz'
```

#### Display region

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

#### Display profile

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.profile_aliases]
Enterprise_Naming_Scheme-voidstars = 'void**'
```

## Azure

The `azure` module shows the current Azure Subscription. This is based on showing the name of the default subscription, as defined in the `~/.azure/azureProfile.json` file.

### 選項

| 變數         | 預設                                       | 說明                                         |
| ---------- | ---------------------------------------- | ------------------------------------------ |
| `format`   | `"on [$symbol($subscription)]($style) "` | The format for the Azure module to render. |
| `symbol`   | `"ﴃ "`                                   | The symbol used in the format.             |
| `style`    | `"blue bold"`                            | The style used in the format.              |
| `disabled` | `true`                                   | Disables the `azure` module.               |

### 範例

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($subscription)]($style) "
symbol = "ﴃ "
style = "blue bold"
```

## 電池

`battery` 模組顯示電池的電量以及現在的充電狀態。 這個模組只會在裝置的電量低於 10% 的時候看見。

### 選項

| Option               | 預設                                | 說明                         |
| -------------------- | --------------------------------- | -------------------------- |
| `full_symbol`        | `" "`                            | 當電池充飽時顯示的符號。               |
| `charging_symbol`    | `" "`                            | 當電池正在充電時顯示的符號。             |
| `discharging_symbol` | `" "`                            | 當電池正在放電時顯示的符號。             |
| `unknown_symbol`     | `" "`                            | 當電池狀態不明時顯示的符號。             |
| `empty_symbol`       | `" "`                            | 當電池沒電時顯示的符號。               |
| `format`             | `"[$symbol$percentage]($style) "` | The format for the module. |
| `display`            | [連結](#battery-display)            | 顯示的門檻與模組的風格。               |
| `disabled`           | `false`                           | 停用 `battery` 模組。           |

### 範例

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "
```

### 電池顯示

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style). 如果沒有提供 `display`。 預設如下：

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### 選項

`display` 選項是一個下列表格的陣列。

| Option               | 預設           | 說明                                                                                                        |
| -------------------- | ------------ | --------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | 顯示選項的上界。                                                                                                  |
| `style`              | `"red bold"` | 顯示選項使用時的風格。                                                                                               |
| `charging_symbol`    |              | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.    |
| `discharging_symbol` |              | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option. |

#### 範例

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

## Buf

The `buf` module shows the currently installed version of [Buf](https://buf.build). By default, the module is shown if all of the following conditions are met:

- The [`buf`](https://github.com/bufbuild/buf) CLI is installed.
- The current directory contains a [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), or [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) configuration file.

### 選項

| Option              | 預設                                              | 說明                                                    |
| ------------------- | ----------------------------------------------- | ----------------------------------------------------- |
| `format`            | `"with [$symbol($version )]($style)"`           | The format for the `buf` module.                      |
| `version_format`    | `"v${raw}"`                                     | The version format.                                   |
| `symbol`            | `"🦬 "`                                          | The symbol used before displaying the version of Buf. |
| `detect_extensions` | `[]`                                            | Which extensions should trigger this module.          |
| `detect_files`      | `["buf.yaml", "buf.gen.yaml", "buf.work.yaml"]` | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                            | Which folders should trigger this modules.            |
| `style`             | `"bold blue"`                                   | 這個模組的風格。                                              |
| `disabled`          | `false`                                         | Disables the `elixir` module.                         |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| `version` | `v1.0.0` | The version of `buf`                 |
| `symbol`  |          | Mirrors the value of option `symbol` |
| `style`*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[buf]
symbol = "🦬 "
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `bun.lockb` 檔案
- 現在資料夾中含有一個 `bunfig.toml` 檔案

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🍞 "`                               | A format string representing the symbol of Node.js.                       |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["bun.lockb", "bunfig.toml"]`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold red"`                         | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `bun` module.                                                |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v0.1.4` | The version of `bun`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[bun]
format = "via [🍔 $version](bold green) "
```

## C

The `c` module shows some information about your C compiler. By default the module will be shown if the current directory contains a `.c` or `.h` file.

### 選項

| Option              | 預設                                                                          | 說明                                                                        |
| ------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version(-$name) )]($style)"`                                | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                                                                 | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"C "`                                                                      | The symbol used before displaying the compiler details                    |
| `detect_extensions` | `["c", "h"]`                                                                | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                                                        | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                        | Which folders should trigger this module.                                 |
| `commands`          | [ [ "cc", "--version" ], [ "gcc", "--version" ], [ "clang", "--version" ] ] | How to detect what the compiler is                                        |
| `style`             | `"bold 149"`                                                                | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                                                     | Disables the `c` module.                                                  |

### Variables

| 變數      | 範例     | 說明                                   |
| ------- | ------ | ------------------------------------ |
| name    | clang  | The name of the compiler             |
| version | 13.0.0 | The version of the compiler          |
| symbol  |        | Mirrors the value of option `symbol` |
| style   |        | Mirrors the value of option `style`  |

NB that `version` is not in the default format.

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `["mycc", "--version"]`. Starship will try executing each command until it gets a result on STDOUT.

If a C compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/).

### 範例

```toml
# ~/.config/starship.toml

[c]
format = "via [$name $version]($style)"
```

## 字元

`character` 模組在你的文字輸入處旁顯示一個字元 (通常是箭頭)。

這個字元會告訴你最後的指令是成功還是失敗。 It can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change its shape take a look at [this example](#with-custom-error-shape).

::: warning

`vimcmd_symbol` is only supported in cmd, fish and zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol` are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

:::

### 選項

| Option                      | 預設                   | 說明                                                                                      |
| --------------------------- | -------------------- | --------------------------------------------------------------------------------------- |
| `format`                    | `"$symbol "`         | The format string used before the text input.                                           |
| `success_symbol`            | `"[❯](bold green)"`  | The format string used before the text input if the previous command succeeded.         |
| `error_symbol`              | `"[❯](bold red)"`    | The format string used before the text input if the previous command failed.            |
| `vimcmd_symbol`             | `"[❮](bold green)"`  | The format string used before the text input if the shell is in vim normal mode.        |
| `vimcmd_replace_one_symbol` | `"[❮](bold purple)"` | The format string used before the text input if the shell is in vim `replace_one` mode. |
| `vimcmd_replace_symbol`     | `"[❮](bold purple)"` | The format string used before the text input if the shell is in vim replace mode.       |
| `vimcmd_visual_symbol`      | `"[❮](bold yellow)"` | The format string used before the text input if the shell is in vim replace mode.       |
| `disabled`                  | `false`              | 停用 `character` 模組。                                                                      |

### Variables

| 變數     | 範例 | 說明                                                                                                       |
| ------ | -- | -------------------------------------------------------------------------------------------------------- |
| symbol |    | A mirror of either `success_symbol`, `error_symbol`, `vimcmd_symbol` or `vimcmd_replace_one_symbol` etc. |

### Examples

#### With custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Without custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### With custom vim shape

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). By default the module will be activated if any of the following conditions are met:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### 選項

| Option              | 預設                                     | 說明                                                                        |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`   | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                            | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"△ "`                                 | The symbol used before the version of cmake.                              |
| `detect_extensions` | `[]`                                   | Which extensions should trigger this module                               |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Which filenames should trigger this module                                |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module                                  |
| `style`             | `"bold blue"`                          | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                | Disables the `cmake` module.                                              |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v3.17.3` | The version of cmake                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL. By default, the module will be shown if any of the following conditions are met:

- The current directory contains any files ending in `.cob` or `.COB`
- The current directory contains any files ending in `.cbl` or `.CBL`

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `symbol`            | `"⚙️ "`                              | The symbol used before displaying the version of COBOL.                   |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold blue"`                        | 這個模組的風格。                                                                  |
| `detect_extensions` | `["cbl", "cob", "CBL", "COB"]`       | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `cobol` module.                                              |

### Variables

| 變數        | 範例         | 說明                                   |
| --------- | ---------- | ------------------------------------ |
| version   | `v3.1.2.0` | The version of `cobol`               |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

## 指令持續時間

`cmd_duration` 模組顯示最後一個指令執行所花費的時間。 這個模組只會在指令花費超過兩秒或是有設定 `min_time` 時，超過設定值時出現。

::: warning 不要在 Bash 中設置 DEBUG trap

如果你在 `bash` 中使用 Starship，不要在執行 `eval $(starship init $0)` 之後設置 `DEBUG` trap，不然這個模組**會**壞掉。

:::

想使用類似 preexec 功能的 Bash 使用者可以 [rcaloras 的 bash_preexec 框架](https://github.com/rcaloras/bash-preexec)。 只要在 `eval $(starship init $0)` 之前簡單地定義 `preexec_functions` 與 `precmd_functions` 兩個陣列，然後就可以照常進行。

### 選項

| Option                 | 預設                            | 說明                                                                                                                                                                |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Shortest duration to show time for (in milliseconds).                                                                                                             |
| `show_milliseconds`    | `false`                       | 顯示時間除了以秒為單位外，亦以毫秒顯示                                                                                                                                               |
| `format`               | `"took [$duration]($style) "` | The format for the module.                                                                                                                                        |
| `style`                | `"bold yellow"`               | 這個模組的風格。                                                                                                                                                          |
| `disabled`             | `false`                       | 停用 `cmd_duration` 模組。                                                                                                                                             |
| `show_notifications`   | `false`                       | Show desktop notifications when command completes.                                                                                                                |
| `min_time_to_notify`   | `45_000`                      | Shortest duration for notification (in milliseconds).                                                                                                             |
| `notification_timeout` |                               | Duration to show notification for (in milliseconds). If unset, notification timeout will be determined by daemon. Not all notification daemons honor this option. |

### Variables

| 變數        | 範例       | 說明                                      |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | The time it took to execute the command |
| style\* |          | Mirrors the value of option `style`     |

*: This variable can only be used as a part of a style string

### 範例

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

### 選項

| Option              | 預設                                     | 說明                                                                                              |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | 如果環境變數由所`conda create -p [path]`產生時，環境變數的資料夾需要截斷的數目。 `0` 表示不截斷 也請參考 [`directory`](#directory)模組 |
| `symbol`            | `"🅒 "`                                 | 環境名稱前使用的符號。                                                                                     |
| `style`             | `"bold green"`                         | 這個模組的風格。                                                                                        |
| `format`            | `"via [$symbol$environment]($style) "` | The format for the module.                                                                      |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                      |
| `disabled`          | `false`                                | 停用 `conda` 模組。                                                                                  |

### Variables

| 變數          | 範例           | 說明                                   |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current conda environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*   |              | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Container

The `container` module displays a symbol and container name, if inside a container.

### 選項

| Option     | 預設                                 | 說明                                        |
| ---------- | ---------------------------------- | ----------------------------------------- |
| `symbol`   | `"⬢"`                              | The symbol shown, when inside a container |
| `style`    | `"bold red dimmed"`                | 這個模組的風格。                                  |
| `format`   | `'[$symbol \[$name\]]($style) '` | The format for the module.                |
| `disabled` | `false`                            | Disables the `container` module.          |

### Variables

| 變數        | 範例                  | 說明                                   |
| --------- | ------------------- | ------------------------------------ |
| name      | `fedora-toolbox:35` | The name of the container            |
| symbol    |                     | Mirrors the value of option `symbol` |
| style\* |                     | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/). By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `shard.yml` 檔案
- 現在資料夾中含有一個`.cr`檔案

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `symbol`            | `"🔮 "`                               | The symbol used before displaying the version of crystal.                 |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | 這個模組的風格。                                                                  |
| `detect_extensions` | `["cr"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["shard.yml"]`                      | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `crystal` module.                                            |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v0.32.1` | The version of `crystal`             |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers) SDK version when you are in the root directory of your Daml project. The `sdk-version` in the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION` environment variable. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `daml.yaml` 檔案

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"Λ "`                               | A format string representing the symbol of Daml                           |
| `style`             | `"bold cyan"`                        | 這個模組的風格。                                                                  |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["daml.yaml"]`                      | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `daml` module.                                               |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v2.2.0` | The version of `daml`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[daml]
format = "via [D $version](bold bright-green) "
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- 現在資料夾中包含一個 `pubspec.yaml`、`pubspec.yml` 或 `pubspec.lock` 檔案

### 選項

| Option              | 預設                                                | 說明                                                                        |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                                       | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🎯 "`                                            | A format string representing the symbol of Dart                           |
| `detect_extensions` | `["dart"]`                                        | Which extensions should trigger this module.                              |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".dart_tool"]`                                  | Which folders should trigger this module.                                 |
| `style`             | `"bold blue"`                                     | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                           | Disables the `dart` module.                                               |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v2.8.4` | The version of `dart`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `deno.json`, `deno.jsonc`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### 選項

| Option              | 預設                                                                      | 說明                                                                        |
| ------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                    | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                                                  | A format string representing the symbol of Deno                           |
| `detect_extensions` | `[]`                                                                    | Which extensions should trigger this module.                              |
| `detect_files`      | `["deno.json", "deno.jsonc", "mod.ts", "mod.js", "deps.ts", "deps.js"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                    | Which folders should trigger this module.                                 |
| `style`             | `"green bold"`                                                          | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                                                 | Disables the `deno` module.                                               |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.8.3` | The version of `deno`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

### 範例

```toml
# ~/.config/starship.toml

[deno]
format = "via [🦕 $version](green bold) "
```

## 資料夾

`directory` 模組顯示到現在資料夾的路徑，並裁減到前三層資料夾。 你的資料夾也會被裁減到你所在的 git 儲存庫的根目錄。

如果正在使用 fish 風格的 pwd 選項，將不會隱藏被裁減的資料夾，而是會根據你在選項中設定的數字看到每一層資料夾的縮寫。

例如，給定一個右列的路徑 `~/Dev/Nix/nixpkgs/pkgs` 其中 `nixpkgs` 是儲存庫的根目錄，而且該選項被設定為 `1`。 你會看到 `~/D/N/nixpkgs/pkgs`，而在這個設定之前則是 `nixpkgs/pkgs`。

### 選項

| Option              | 預設                                                                                                          | 說明                                                                                      |
| ------------------- | ----------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                                                                         | 到達現在資料夾的路徑中，要被裁減掉的資料夾數目。                                                                |
| `truncate_to_repo`  | `true`                                                                                                      | 是否要裁減到你現在所在的 git 儲存庫的根目錄。                                                               |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "`                                                          | The format for the module.                                                              |
| `style`             | `"bold cyan"`                                                                                               | 這個模組的風格。                                                                                |
| `disabled`          | `false`                                                                                                     | 停用 `directory` 模組。                                                                      |
| `read_only`         | `"🔒"`                                                                                                       | The symbol indicating current directory is read only.                                   |
| `read_only_style`   | `"red"`                                                                                                     | The style for the read only symbol.                                                     |
| `truncation_symbol` | `""`                                                                                                        | The symbol to prefix to truncated paths. eg: "…/"                                       |
| `repo_root_style`   |                                                                                                             | The style for the root of the git repo. The default value is equivalent to `style`.     |
| `repo_root_format`  | `"[$before_root_path]($style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) "` | The format of a git repo when `repo_root_style` is defined.                             |
| `home_symbol`       | `"~"`                                                                                                       | The symbol indicating home directory.                                                   |
| `use_os_path_sep`   | `true`                                                                                                      | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows) |

<details>
<summary>這個模組有些進階設定選項可以控制顯示資料夾。</summary>

| Advanced Option             | 預設     | 說明                                                                                                                                                                     |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`    | 當使用 fish shell 的 pwd 路徑邏輯時使用的字元數量。                                                                                                                                     |
| `use_logical_path`          | `true` | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| 變數        | 範例                    | 說明                                  |
| --------- | --------------------- | ----------------------------------- |
| path      | `"D:/Projects"`       | The current directory path          |
| style\* | `"black bold dimmed"` | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

<details>
<summary>The git repos have additional variables.</summary>

Let us consider the path `/path/to/home/git_repo/src/lib`

| 變數                 | 範例                    | 說明                                      |
| ------------------ | --------------------- | --------------------------------------- |
| before_root_path | `"/path/to/home/"`    | The path before git root directory path |
| repo_root          | `"git_repo"`          | The git root directory name             |
| path               | `"/src/lib"`          | The remaining path                      |
| style              | `"black bold dimmed"` | Mirrors the value of option `style`     |
| repo_root_style  | `"underline white"`   | Style for git root directory name       |

</details>

### 範例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default` or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or `DOCKER_CONTEXT` environment variables are set (as they are meant to override the context in use).

### 選項

| Option              | 預設                                                            | 說明                                                                                |
| ------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | The format for the module.                                                        |
| `symbol`            | `"🐳 "`                                                        | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                        | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                          | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                          | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `"blue bold"`                                                 | 這個模組的風格。                                                                          |
| `disabled`          | `false`                                                       | Disables the `docker_context` module.                                             |

### Variables

| 變數        | 範例             | 說明                                   |
| --------- | -------------- | ------------------------------------ |
| context   | `test_context` | The current docker context           |
| symbol    |                | Mirrors the value of option `symbol` |
| style\* |                | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. 如果這個資料夾已經選定一個 SDK，則顯示這個 SDK 的版本。 如果沒有的話，則顯示最新安裝的 SDK 版本。

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

這個模組內部是使用它自己的機制來偵測版本。 一般來說這個模組有 `dotnet --version` 的兩倍快，但是它可能會在你的 .NET 專案有不尋常的資料夾結構時顯示不正確的版本。 如果精確度比速度更重要的話，你可以藉由設定模組中的 `heuristic = false` 選項來停用這個功能。

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>) when there is a `.csproj` file in the current directory.

### 選項

| Option              | 預設                                                                                                      | 說明                                                                        |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                                                                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | 在顯示 dotnet 版本之前用的符號。                                                      |
| `heuristic`         | `true`                                                                                                  | 使用更快速的版本偵測法來保持 starship 的速度。                                              |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | Which extensions should trigger this module.                              |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                    | Which folders should trigger this modules.                                |
| `style`             | `"bold blue"`                                                                                           | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                                                                                 | 停用 `dotnet` 模組。                                                           |

### Variables

| 變數        | 範例               | 說明                                                                 |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Mirrors the value of option `symbol`                               |
| style\* |                  | Mirrors the value of option `style`                                |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/). By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `mix.exs` 檔案.

### 選項

| Option              | 預設                                                          | 說明                                                                        |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                                         |
| `version_format`    | `"v${raw}"`                                                 | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | The symbol used before displaying the version of Elixir/Erlang.           |
| `detect_extensions` | `[]`                                                        | Which extensions should trigger this module.                              |
| `detect_files`      | `["mix.exs"]`                                               | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                        | Which folders should trigger this modules.                                |
| `style`             | `"bold purple"`                                             | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                             |

### Variables

| 變數          | 範例      | 說明                                   |
| ----------- | ------- | ------------------------------------ |
| version     | `v1.10` | The version of `elixir`              |
| otp_version |         | The otp version of `elixir`          |
| symbol      |         | Mirrors the value of option `symbol` |
| style\*   |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/). By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `elm.json` 檔案
- 現在資料夾中包含一個 `elm-package.json` 檔案
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains `*.elm` files

### 選項

| Option              | 預設                                                 | 說明                                                                        |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`               | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                                        | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | A format string representing the symbol of Elm.                           |
| `detect_extensions` | `["elm"]`                                          | Which extensions should trigger this module.                              |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `["elm-stuff"]`                                    | Which folders should trigger this modules.                                |
| `style`             | `"cyan bold"`                                      | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                            | Disables the `elm` module.                                                |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v0.19.1` | The version of `elm`                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## 環境變數

The `env_var` module displays the current value of a selected environment variables. 這個模組只在下列條件其中之一達到時顯示：

- `variable` 設定選項符合一個存在的環境變數。
- 沒有設定 `variable` 選項，但是有設定 `default` 選項。

::: tip

Multiple environmental variables can be displayed by using a `.`. (see example) If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.

Example: following configuration will display value of USER environment variable

```toml
# ~/.config/starship.toml

[env_var.USER]
default = "unknown user"
```

:::

### 選項

| Option     | 預設                             | 說明                         |
| ---------- | ------------------------------ | -------------------------- |
| `symbol`   | `""`                           | 顯示在變數數值之前的符號。              |
| `variable` |                                | 要顯示的環境變數。                  |
| `default`  |                                | 在選擇的變數值沒有定義時，顯示的預設值。       |
| `format`   | `"with [$env_value]($style) "` | The format for the module. |
| `disabled` | `false`                        | 停用 `env_var` 模組。           |

### Variables

| 變數        | 範例                                          | 說明                                         |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Mirrors the value of option `symbol`       |
| style\* | `black bold dimmed`                         | Mirrors the value of option `style`        |

*: This variable can only be used as a part of a style string

### 範例

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

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/). By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `rebar.config` 檔案.
- 現在資料夾中含有一個 `erlang.mk` 檔案.

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | The symbol used before displaying the version of erlang.                  |
| `style`             | `"bold red"`                         | 這個模組的風格。                                                                  |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `disabled`          | `false`                              | Disables the `erlang` module.                                             |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v22.1.3` | The version of `erlang`              |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are present in a line they will split the space evenly between them. This is useful for aligning other modules.

### 選項

| Option     | 預設             | 說明                                |
| ---------- | -------------- | --------------------------------- |
| `symbol`   | `"."`          | The symbol used to fill the line. |
| `style`    | `"bold black"` | 這個模組的風格。                          |
| `disabled` | `false`        | Disables the `fill` module        |

### 範例

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

### 選項

| Option            | 預設                                                         | 說明                                                               |
| ----------------- | ---------------------------------------------------------- | ---------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | The format for the module.                                       |
| `symbol`          | `"☁️  "`                                                   | The symbol used before displaying the current GCP profile.       |
| `region_aliases`  |                                                            | Table of region aliases to display in addition to the GCP name.  |
| `project_aliases` |                                                            | Table of project aliases to display in addition to the GCP name. |
| `style`           | `"bold blue"`                                              | 這個模組的風格。                                                         |
| `disabled`        | `false`                                                    | Disables the `gcloud` module.                                    |

### Variables

| 變數        | 範例            | 說明                                                                 |
| --------- | ------------- | ------------------------------------------------------------------ |
| region    | `us-central1` | The current GCP region                                             |
| account   | `foo`         | The current GCP profile                                            |
| domain    | `example.com` | The current GCP profile domain                                     |
| project   |               | The current GCP project                                            |
| active    | `default`     | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |               | Mirrors the value of option `symbol`                               |
| style\* |               | Mirrors the value of option `style`                                |

*: This variable can only be used as a part of a style string

### Examples

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

#### Display account and aliased project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = "vlpn"
```

## Git 分支

`git_branch` 模組顯示現在的資料夾中使用中的儲存庫的分支。

### 選項

| Option               | 預設                                                | 說明                                                                                   |
| -------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `always_show_remote` | `false`                                           | Shows the remote tracking branch name, even if it is equal to the local branch name. |
| `format`             | `"on [$symbol$branch(:$remote_branch)]($style) "` | The format for the module. Use `"$branch"` to refer to the current branch name.      |
| `symbol`             | `" "`                                            | A format string representing the symbol of git branch.                               |
| `style`              | `"bold purple"`                                   | 這個模組的風格。                                                                             |
| `truncation_length`  | `2^63 - 1`                                        | Truncates a git branch to `N` graphemes.                                             |
| `truncation_symbol`  | `"…"`                                             | 用來指示分支名稱被縮減的符號。 You can use `""` for no symbol.                                      |
| `only_attached`      | `false`                                           | Only show the branch name when not in a detached `HEAD` state.                       |
| `ignore_branches`    | `[]`                                              | A list of names to avoid displaying. Useful for "master" or "main".                  |
| `disabled`           | `false`                                           | 停用 `git_branch` 模組。                                                                  |

### Variables

| 變數            | 範例       | 說明                                                                                                     |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | Mirrors the value of option `symbol`                                                                   |
| style\*     |          | Mirrors the value of option `style`                                                                    |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
ignore_branches = ["master", "main"]
```

## Git Commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### 選項

| Option               | 預設                             | 說明                                                                                   |
| -------------------- | ------------------------------ | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                            | The length of the displayed git commit hash.                                         |
| `format`             | `'[\($hash$tag\)]($style) '` | The format for the module.                                                           |
| `style`              | `"bold green"`                 | 這個模組的風格。                                                                             |
| `only_detached`      | `true`                         | Only show git commit hash when in detached `HEAD` state                              |
| `tag_disabled`       | `true`                         | Disables showing tag info in `git_commit` module.                                    |
| `tag_max_candidates` | `0`                            | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `" 🏷 "`                        | Tag symbol prefixing the info shown                                                  |
| `disabled`           | `false`                        | Disables the `git_commit` module.                                                    |

### Variables

| 變數        | 範例        | 說明                                  |
| --------- | --------- | ----------------------------------- |
| hash      | `b703eb3` | The current git commit hash         |
| style\* |           | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

`git_state` 模組會顯示在 git 儲存庫中的資料夾內，以及會在有作業正在進行時顯示，像是：_REBASING_、_BISECTING_ 等等。 如果有進展的資訊 (像是 REBASING 3/10)，也會一併顯示出來。

### 選項

| Option         | 預設                                                              | 說明                                                                                      |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                 | 這個模組的風格。                                                                                |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | The format for the module.                                                              |
| `disabled`     | `false`                                                         | 停用 `git_state` 模組。                                                                      |

### Variables

| 變數               | 範例         | 說明                                  |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*        |            | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in the current git repository.

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| Option               | 預設                                                           | 說明                                    |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `"bold green"`                                               | The style for the added count.        |
| `deleted_style`      | `"bold red"`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `"([+$added]($added_style) )([-$deleted]($deleted_style) )"` | The format for the module.            |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.    |

### Variables

| 變數                | 範例  | 說明                                          |
| ----------------- | --- | ------------------------------------------- |
| added             | `1` | The current number of added lines           |
| deleted           | `2` | The current number of deleted lines         |
| added_style\*   |     | Mirrors the value of option `added_style`   |
| deleted_style\* |     | Mirrors the value of option `deleted_style` |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = "bold blue"
format = "[+$added]($added_style)/[-$deleted]($deleted_style) "
```

## Git Status

`git_status` 模組顯示用來表示現在資料夾之中儲存庫狀態的符號。

::: tip

The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment. You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

:::

### 選項

| Option              | 預設                                              | 說明                                                                                                          |
| ------------------- | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status`                                                                         |
| `conflicted`        | `"="`                                           | 這個分支有合併衝突。                                                                                                  |
| `ahead`             | `"⇡"`                                           | The format of `ahead`                                                                                       |
| `behind`            | `"⇣"`                                           | The format of `behind`                                                                                      |
| `diverged`          | `"⇕"`                                           | The format of `diverged`                                                                                    |
| `up_to_date`        | `""`                                            | The format of `up_to_date`                                                                                  |
| `untracked`         | `"?"`                                           | The format of `untracked`                                                                                   |
| `stashed`           | `"$"`                                           | The format of `stashed`                                                                                     |
| `modified`          | `"!"`                                           | The format of `modified`                                                                                    |
| `staged`            | `"+"`                                           | The format of `staged`                                                                                      |
| `renamed`           | `"»"`                                           | The format of `renamed`                                                                                     |
| `deleted`           | `"✘"`                                           | The format of `deleted`                                                                                     |
| `style`             | `"bold red"`                                    | 這個模組的風格。                                                                                                    |
| `ignore_submodules` | `false`                                         | Ignore changes to submodules.                                                                               |
| `disabled`          | `false`                                         | 停用 `git_status` 模組。                                                                                         |
| `windows_starship`  |                                                 | Use this (Linux) path to a Windows Starship executable to render `git_status` when on Windows paths in WSL. |

### Variables

The following variables can be used in `format`:

| 變數             | 說明                                                                                                            |
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
| style\*      | Mirrors the value of option `style`                                                                           |

*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| 變數             | 說明                                             |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| 變數      | 說明                       |
| ------- | ------------------------ |
| `count` | Show the number of files |

### 範例

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

Use Windows Starship executable on Windows paths in WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = "/mnt/c/Users/username/scoop/apps/starship/current/starship.exe"
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/). By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `go.mod` 檔案
- 現在資料夾中含有一個 `go.sum` 檔案
- 現在資料夾中含有一個 `go.work` 檔案
- 現在資料夾中含有一個 `glide.yaml` 檔案
- 現在資料夾中含有一個 `Gopkg.yml` 檔案
- 現在資料夾中含有一個 `Gopkg.lock` 檔案
- The current directory contains a `.go-version` file
- 現在資料夾中含有一個 `Godeps` 資料夾
- 現在資料夾中含有一個檔案具有 `.go` 副檔名

### 選項

| Option              | 預設                                                                                        | 說明                                                                        |
| ------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                      | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                                                                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                                    | A format string representing the symbol of Go.                            |
| `detect_extensions` | `["go"]`                                                                                  | Which extensions should trigger this module.                              |
| `detect_files`      | `["go.mod", "go.sum", "go.work", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `["Godeps"]`                                                                              | Which folders should trigger this module.                                 |
| `style`             | `"bold cyan"`                                                                             | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                                                                   | 停用 `golang` 模組。                                                           |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v1.12.1` | The version of `go`                  |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `stack.yaml` 檔案
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### 選項

| Option              | 預設                                   | 說明                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                         |
| `symbol`            | `"λ "`                               | A format string representing the symbol of Haskell |
| `detect_extensions` | `["hs", "cabal", "hs-boot"]`         | Which extensions should trigger this module.       |
| `detect_files`      | `["stack.yaml", "cabal.project"]`    | Which filenames should trigger this module.        |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.          |
| `style`             | `"bold purple"`                      | 這個模組的風格。                                           |
| `disabled`          | `false`                              | Disables the `haskell` module.                     |

### Variables

| 變數             | 範例          | 說明                                                                                      |
| -------------- | ----------- | --------------------------------------------------------------------------------------- |
| version        |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot       | `lts-18.12` | Currently selected Stack snapshot                                                       |
| ghc\_version | `9.2.1`     | Currently installed GHC version                                                         |
| symbol         |             | Mirrors the value of option `symbol`                                                    |
| style\*      |             | Mirrors the value of option `style`                                                     |

*: This variable can only be used as a part of a style string

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `helmfile.yaml` 檔案
- The current directory contains a `Chart.yaml` file

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm.                          |
| `style`             | `"bold white"`                       | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `helm` module.                                               |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v3.1.1` | The version of `helm`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## 主機名稱

`hostname` 模組顯示系統的主機名稱。

### 選項

| Option       | 預設                                     | 說明                                                                     |
| ------------ | -------------------------------------- | ---------------------------------------------------------------------- |
| `ssh_only`   | `true`                                 | 只在連接到一個 SSH session 時顯示主機名稱。                                           |
| `ssh_symbol` | `"🌐 "`                                 | A format string representing the symbol when connected to SSH session. |
| `trim_at`    | `"."`                                  | 擷取出主機名稱的斷點，以第一個符合的為準。 `"."` 會讓它停在第一個點的符號。 `""` 會停用任何的截斷功能。             |
| `format`     | `"[$ssh_symbol$hostname]($style) in "` | The format for the module.                                             |
| `style`      | `"bold dimmed green"`                  | 這個模組的風格。                                                               |
| `disabled`   | `false`                                | 停用 `hostname` 模組。                                                      |

### Variables

| 變數         | 範例         | 說明                                                    |
| ---------- | ---------- | ----------------------------------------------------- |
| 主機名稱       | `computer` | The hostname of the computer                          |
| style\*  |            | Mirrors the value of option `style`                   |
| ssh_symbol | `"🌏 "`     | The symbol to represent when connected to SSH session |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = "[$ssh_symbol](bold blue) on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### 選項

| Option              | 預設                                                                                                       | 說明                                                                        |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                 | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                                                                                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                     | Which extensions should trigger this module.                              |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", "deps.edn", "project.clj", "build.boot"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                     | Which folders should trigger this modules.                                |
| `symbol`            | `"☕ "`                                                                                                   | A format string representing the symbol of Java                           |
| `style`             | `"red dimmed"`                                                                                           | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                                                                                  | 停用 `java` 模組。                                                             |

### Variables

| 變數        | 範例    | 說明                                   |
| --------- | ----- | ------------------------------------ |
| version   | `v14` | The version of `java`                |
| symbol    |       | Mirrors the value of option `symbol` |
| style\* |       | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## 工作

`jobs` 模組顯示現在正在執行中的工作。 這個模組只會在有背景工作正在執行時顯示。 The module will show the number of jobs running if there are at least 2 jobs, or more than the `number_threshold` config value, if it exists. The module will show a symbol if there is at least 1 job, or more than the `symbol_threshold` config value, if it exists. You can set both values to 0 in order to _always_ show the symbol and number of jobs, even if there are 0 jobs running.

The default functionality is:

- 0 jobs -> Nothing is shown.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

::: warning

This module is not supported on tcsh and nu.

:::

::: warning

The `threshold` option is deprecated, but if you want to use it, the module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

:::

### 選項

| Option             | 預設                            | 說明                                                                       |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | 在超過指定值時顯示工作數量。                                                           |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `"[$symbol$number]($style) "` | The format for the module.                                               |
| `symbol`           | `"✦"`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `"bold blue"`                 | 這個模組的風格。                                                                 |
| `disabled`         | `false`                       | 停用 `jobs` 模組。                                                            |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variables

| 變數        | 範例  | 說明                                   |
| --------- | --- | ------------------------------------ |
| number    | `1` | The number of jobs                   |
| symbol    |     | Mirrors the value of option `symbol` |
| style\* |     | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
number_threshold = 4
symbol_threshold = 0
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia.                         |
| `style`             | `"bold purple"`                      | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `julia` module.                                              |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.4.0` | The version of `julia`               |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.kt` or a `.kts` file

### 選項

| Option              | 預設                                   | 說明                                                                            |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                    |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`     |
| `detect_extensions` | `["kt", "kts"]`                      | Which extensions should trigger this module.                                  |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                                   |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                    |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `"bold blue"`                        | 這個模組的風格。                                                                      |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v1.4.21` | The version of `kotlin`              |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

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

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-context --namespace astronaut`. Similarly the user and cluster can be set with `kubectl config set-context starship-context --user starship-user` and `kubectl config set-context starship-context --cluster starship-cluster`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

When the module is enabled it will always be active, unless any of `detect_extensions`, `detect_files` or `detect_folders` have been st in which case the module will only be active in directories that match those conditions.

:::

### 選項

| Option              | 預設                                                   | 說明                                                                    |
| ------------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`            | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | The format for the module.                                            |
| `style`             | `"cyan bold"`                                        | 這個模組的風格。                                                              |
| `context_aliases`   |                                                      | Table of context aliases to display.                                  |
| `user_aliases`      |                                                      | Table of user aliases to display.                                     |
| `detect_extensions` | `[]`                                                 | Which extensions should trigger this module.                          |
| `detect_files`      | `[]`                                                 | Which filenames should trigger this module.                           |
| `detect_folders`    | `[]`                                                 | Which folders should trigger this modules.                            |
| `disabled`          | `true`                                               | Disables the `kubernetes` module.                                     |

### Variables

| 變數        | 範例                   | 說明                                       |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbol    |                      | Mirrors the value of option `symbol`     |
| style\* |                      | Mirrors the value of option `style`      |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
".*/openshift-cluster/.*" = "openshift"
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
[kubernetes.user_aliases]
"dev.local.cluster.k8s" = "dev"
"root/.*" = "root"
```

Only show the module in directories that contain a `k8s` file.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Regex Matching

Additional to simple aliasing, `context_aliases` and `user_aliases` also supports extended matching and renaming using regular expressions.

The regular expression must match on the entire kube context, capture groups can be referenced using `$name` and `$N` in the replacement. This is more explained in the [regex crate](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace) documentation.

Long and automatically generated cluster names can be identified and shortened using regular expressions:

```toml
[kubernetes.context_aliases]
# OpenShift contexts carry the namespace and user in the kube context: `namespace/name/user`:
".*/openshift-cluster/.*" = "openshift"
# Or better, to rename every OpenShift cluster at once:
".*/(?P<var_cluster>[\\w-]+)/.*" = "$var_cluster"

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
```

## 換行

`line_break` 模組將提示字元分成兩行。

### 選項

| Option     | 預設      | 說明                            |
| ---------- | ------- | ----------------------------- |
| `disabled` | `false` | 停用 `line_break` 模組，讓提示字元變成一行。 |

### 範例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Local IP

The `localip` module shows the IPv4 address of the primary network interface.

### 選項

| Option     | 預設                        | 說明                                                     |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `"[$localipv4]($style) "` | The format for the module.                             |
| `style`    | `"bold yellow"`           | 這個模組的風格。                                               |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### Variables

| 變數        | 範例           | 說明                                  |
| --------- | ------------ | ----------------------------------- |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address   |
| style\* |              | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = "@[$localipv4](bold red) "
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### 選項

| Option              | 預設                                   | 說明                                                                         |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`  |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `["lua"]`                            | Which extensions should trigger this module.                               |
| `detect_files`      | `[".lua-version"]`                   | Which filenames should trigger this module.                                |
| `detect_folders`    | `["lua"]`                            | Which folders should trigger this module.                                  |
| `style`             | `"bold blue"`                        | 這個模組的風格。                                                                   |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v5.4.0` | The version of `lua`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## 記憶體使用量

`memory_usage` 模組顯示現在系統記憶體與 swap 的使用量。

預設 swap 使用量會在系統總 swap 使用量不為 0 時顯示出來。

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| Option      | 預設                                              | 說明                         |
| ----------- | ----------------------------------------------- | -------------------------- |
| `threshold` | `75`                                            | 將記憶體使用量隱藏，除非使用量超過指定值。      |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | The format for the module. |
| `symbol`    | `"🐏"`                                           | 顯示在記憶體使用量之前的符號。            |
| `style`     | `"bold dimmed white"`                           | 這個模組的風格。                   |
| `disabled`  | `true`                                          | 停用 `memory_usage` 模組。      |

### Variables

| 變數               | 範例            | 說明                                                                 |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Mirrors the value of option `symbol`                               |
| style\*        |               | Mirrors the value of option `style`                                |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

### 範例

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Meson

The `meson` module shows the current Meson developer environment status.

By default the Meson project name is displayed, if `$MESON_DEVENV` is set.

### 選項

| Option              | 預設                                 | 說明                                                                                        |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                |
| `truncation_symbol` | `"…"`                              | The symbol used to indicate a project name was truncated. You can use `""` for no symbol. |
| `format`            | `"via [$symbol$project]($style) "` | The format for the module.                                                                |
| `symbol`            | `"⬢ "`                             | The symbol used before displaying the project name.                                       |
| `style`             | `"blue bold"`                      | 這個模組的風格。                                                                                  |
| `disabled`          | `false`                            | Disables the `meson` module.                                                              |

### Variables

| 變數        | 範例         | 說明                                   |
| --------- | ---------- | ------------------------------------ |
| project   | `starship` | The current Meson project name       |
| symbol    | `🐏`        | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = "--"
symbol = " "
style = "bold dimmed green"
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### 選項

| Option              | 預設                               | 說明                                                                                           |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | 這個模組的風格。                                                                                     |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.                                                                   |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `"…"`                            | 用來指示分支名稱被縮減的符號。                                                                              |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| branch    | `master` | The active mercurial branch          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `nim.cfg` 檔案
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module                                                 |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim.                     |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Which extensions should trigger this module.                              |
| `detect_files`      | `["nim.cfg"]`                        | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold yellow"`                      | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `nim` module.                                                |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.2.0` | The version of `nimc`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. 這個模組會在 nix-shell 環境中顯示。

### 選項

| Option       | 預設                                             | 說明                                                    |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | The format for the module.                            |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | 這個模組的風格。                                              |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | 停用 `nix_shell` 模組。                                    |

### Variables

| 變數        | 範例      | 說明                                   |
| --------- | ------- | ------------------------------------ |
| state     | `pure`  | The state of the nix-shell           |
| name      | `lorri` | The name of the nix-shell            |
| symbol    |         | Mirrors the value of option `symbol` |
| style\* |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). By default the module will be shown if any of the following conditions are met:

- 現在資料夾中包含一個 `package.json` 檔案
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- 現在資料夾中包含一個 `node_modules` 資料夾
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts`, `.mts` or `.cts` extension

### 選項

| Option              | 預設                                         | 說明                                                                                                    |
| ------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`       | The format for the module.                                                                            |
| `version_format`    | `"v${raw}"`                                | The version format. Available vars are `raw`, `major`, `minor`, & `patch`                             |
| `symbol`            | `" "`                                     | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts", "mts", "cts"]` | Which extensions should trigger this module.                                                          |
| `detect_files`      | `["package.json", ".node-version"]`        | Which filenames should trigger this module.                                                           |
| `detect_folders`    | `["node_modules"]`                         | Which folders should trigger this module.                                                             |
| `style`             | `"bold green"`                             | 這個模組的風格。                                                                                              |
| `disabled`          | `false`                                    | 停用 `nodejs` 模組。                                                                                       |
| `not_capable_style` | `bold red`                                 | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variables

| 變數        | 範例         | 說明                                   |
| --------- | ---------- | ------------------------------------ |
| version   | `v13.12.0` | The version of `node`                |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### 選項

| Option                    | 預設                                                                         | 說明                                                                        |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | The format string for the module.                                         |
| `version_format`          | `"v${raw}"`                                                                | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml.                   |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch.                   |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.                    |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Which extensions should trigger this module.                              |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Which filenames should trigger this module.                               |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Which folders should trigger this module.                                 |
| `style`                   | `"bold yellow"`                                                            | 這個模組的風格。                                                                  |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                              |

### Variables

| 變數               | 範例           | 說明                                                                |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Mirrors the value of option `symbol`                              |
| style\*        |              | Mirrors the value of option `style`                               |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool. By default the module will be shown if the current directory contains a `.rego` file.

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🪖  "`                              | A format string representing the symbol of OPA.                           |
| `detect_extensions` | `["rego"]`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold blue"`                        | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `opa` module.                                                |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v0.44.0` | The version of `opa`                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[opa]
format = "via [⛑️  $version](bold red) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### 選項

| Option     | 預設                                              | 說明                                                             |
| ---------- | ----------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | The format for the module.                                     |
| `symbol`   | `"☁️ "`                                         | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                 | 這個模組的風格。                                                       |
| `disabled` | `false`                                         | Disables the `openstack` module.                               |

### Variables

| 變數        | 範例     | 說明                                   |
| --------- | ------ | ------------------------------------ |
| cloud     | `corp` | The current OpenStack cloud          |
| project   | `dev`  | The current OpenStack project        |
| symbol    |        | Mirrors the value of option `symbol` |
| style\* |        | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = "bold yellow"
symbol = "☁️ "
```

## 套件版本

The `package` 模組在現在資料夾是一個套件的儲藏庫時出現，並顯示他的現在版本。 The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from a [PEP 621](https://peps.python.org/pep-0621/) compliant `pyproject.toml` or a `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present in the current directory
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present in the current directory
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present in the current directory
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present in the current directory
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present in the current directory
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present in the current directory
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present in the current directory
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present in the current directory
- [**SBT**](https://scala-sbt.org) - The `sbt` package version is extracted from the `build.sbt` present in the current directory
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) - The `dart` package version is extracted from the `pubspec.yaml` present in the current directory

> ⚠️ 顯示出來的版本是從你的現在資料夾之中擷取出來的，並非從套件管理員取得。

### 選項

| Option            | 預設                                | 說明                                                                        |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | The format for the module.                                                |
| `symbol`          | `"📦 "`                            | 顯示在套件的版本之前的符號。                                                            |
| `version_format`  | `"v${raw}"`                       | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`           | `"bold 208"`                      | 這個模組的風格。                                                                  |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.                 |
| `disabled`        | `false`                           | 停用 `package` 模組。                                                          |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.0.0` | The version of your package          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### 選項

| Option              | 預設                                                                                                       | 說明                                                                        |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                                                                                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl                     |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Which extensions should trigger this module.                              |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                     | Which folders should trigger this module.                                 |
| `style`             | `"bold 149"`                                                                                             | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                               |

### Variables

| 變數        | 範例        | 說明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v5.26.1` | The version of `perl`                |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

### 範例

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `composer.json` 檔案
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | The symbol used before displaying the version of PHP.                     |
| `detect_extensions` | `["php"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `["composer.json", ".php-version"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"147 bold"`                         | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `php` module.                                                |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v7.3.8` | The version of `php`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

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

By default the module will be shown if any of the following conditions are met:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### 選項

| Option           | 預設                                           | 說明                                                                        |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`         | `"via [$symbol($username@)$stack]($style) "` | The format string for the module.                                         |
| `version_format` | `"v${raw}"`                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `" "`                                       | A format string shown before the Pulumi stack.                            |
| `style`          | `"bold 5"`                                   | 這個模組的風格。                                                                  |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.            |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                             |

### Variables

| 變數        | 範例         | 說明                                   |
| --------- | ---------- | ------------------------------------ |
| version   | `v0.12.24` | The version of `pulumi`              |
| stack     | `dev`      | The current Pulumi stack             |
| 使用者名稱     | `alice`    | The current Pulumi username          |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

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

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. By default the module will be shown if any of the following conditions are met:

- 現在資料夾中含有一個 `spago.dhall` 檔案
- The current directory contains a file with the `.purs` extension

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript.              |
| `detect_extensions` | `["purs"]`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `["spago.dhall"]`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold white"`                       | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `purescript` module.                                         |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `0.13.5` | The version of `purescript`          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

By default the module will be shown if any of the following conditions are met:

- 目前資料夾中有一個 `.python-version` 檔案
- 目前資料夾中有一個 `Pipfile` 檔案
- The current directory contains a `__init__.py` file
- 目前資料夾中有一個 `pyproject.toml` 檔案
- 目前資料夾中有一個 `requirements.txt` 檔案
- 現在資料夾中含有一個 `setup.py` 檔案
- 目前資料夾中有一個 `tox.ini` 檔案
- 目前資料夾中有一個 `.py` 副檔名的檔案.
- A virtual environment is currently activated

### 選項

| Option               | 預設                                                                                                           | 說明                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | The format for the module.                                                             |
| `version_format`     | `"v${raw}"`                                                                                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch`              |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | 這個模組的風格。                                                                               |
| `pyenv_version_name` | `false`                                                                                                      | 使用 pyenv 取得 Python 的版本。                                                                |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3", "python2"]`                                                                           | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `["py"]`                                                                                                     | Which extensions should trigger this module                                            |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Which filenames should trigger this module                                             |
| `detect_folders`     | `[]`                                                                                                         | Which folders should trigger this module                                               |
| `disabled`           | `false`                                                                                                      | 停用 `python` 模組。                                                                        |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Variables

| 變數           | 範例              | 說明                                         |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Mirrors the value of option `symbol`       |
| style        | `"yellow bold"` | Mirrors the value of option `style`        |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |

### 範例

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

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"📐"`                                | A format string representing the symbol of R.                             |
| `style`             | `"blue bold"`                        | 這個模組的風格。                                                                  |
| `detect_extensions` | `["R", "Rd", "Rmd", "Rproj", "Rsx"]` | Which extensions should trigger this module                               |
| `detect_files`      | `[".Rprofile"]`                      | Which filenames should trigger this module                                |
| `detect_folders`    | `[".Rproj.user"]`                    | Which folders should trigger this module                                  |
| `disabled`          | `false`                              | Disables the `r` module.                                                  |

### Variables

| 變數      | 範例            | 說明                                   |
| ------- | ------------- | ------------------------------------ |
| version | `v4.0.5`      | The version of `R`                   |
| symbol  |               | Mirrors the value of option `symbol` |
| style   | `"blue bold"` | Mirrors the value of option `style`  |

### 範例

```toml
# ~/.config/starship.toml

[rlang]
format = "with [📐 $version](blue bold) "
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### 選項

| Option              | 預設                                               | 說明                                                                        |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version-$vm_version )]($style)"` | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                                      | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦋 "`                                           | The symbol used before displaying the version of Raku                     |
| `detect_extensions` | `["p6", "pm6", "pod6", "raku", "rakumod"]`       | Which extensions should trigger this module.                              |
| `detect_files`      | `["META6.json"]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                             | Which folders should trigger this module.                                 |
| `style`             | `"bold 149"`                                     | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                          | Disables the `raku` module.                                               |

### Variables

| 變數         | 範例     | 說明                                   |
| ---------- | ------ | ------------------------------------ |
| version    | `v6.d` | The version of `raku`                |
| vm_version | `moar` | The version of VM `raku` is built on |
| symbol     |        | Mirrors the value of option `symbol` |
| style\*  |        | Mirrors the value of option `style`  |

### 範例

```toml
# ~/.config/starship.toml

[raku]
format = "via [🦪 $version]($style) "
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a file with `.red` or `.reds` extension

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.                           |
| `detect_extensions` | `["red"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"red bold"`                         | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `red` module.                                                |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v2.5.1` | The version of `red`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). 這個模組在下列其中一個條件達成時顯示：

- 目前資料夾中有一個 `Gemfile` 檔案
- The current directory contains a `.ruby-version` file
- 目前資料夾中有一個 `.rb` 檔案
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                          |
| `detect_extensions` | `["rb"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `detect_variables`  | `["RUBY_VERSION", "RBENV_VERSION"]`  | Which environment variables should trigger this module.                   |
| `style`             | `"bold red"`                         | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | 停用 `ruby` 模組。                                                             |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v2.5.1` | The version of `ruby`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). 這個模組在下列其中一個條件達成時顯示：

- 目前資料夾中有一個 `Cargo.toml` 檔案
- 現在資料夾中包含一個檔案具有 `.rs` 副檔名

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust                           |
| `detect_extensions` | `["rs"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Cargo.toml"]`                     | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold red"`                         | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | 停用 `rust` 模組。                                                             |

### Variables

| 變數        | 範例                | 說明                                           |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbol    |                   | Mirrors the value of option `symbol`         |
| style\* |                   | Mirrors the value of option `style`          |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### 選項

| Option              | 預設                                       | 說明                                                                        |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Which extensions should trigger this module.                              |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".metals"]`                            | Which folders should trigger this modules.                                |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.                         |
| `style`             | `"red dimmed"`                           | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                  | Disables the `scala` module.                                              |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `2.13.5` | The version of `scala`               |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| Option                 | 預設                        | 說明                                                           |
| ---------------------- | ------------------------- | ------------------------------------------------------------ |
| `bash_indicator`       | `"bsh"`                   | A format string used to represent bash.                      |
| `fish_indicator`       | `"fsh"`                   | A format string used to represent fish.                      |
| `zsh_indicator`        | `"zsh"`                   | A format string used to represent zsh.                       |
| `powershell_indicator` | `"psh"`                   | A format string used to represent powershell.                |
| `ion_indicator`        | `"ion"`                   | A format string used to represent ion.                       |
| `elvish_indicator`     | `"esh"`                   | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `"tsh"`                   | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `"xsh"`                   | A format string used to represent xonsh.                     |
| `cmd_indicator`        | `"cmd"`                   | A format string used to represent cmd.                       |
| `nu_indicator`         | `"nu"`                    | A format string used to represent nu.                        |
| `unknown_indicator`    |                           | The default value to be displayed when the shell is unknown. |
| `format`               | `"[$indicator]($style) "` | The format for the module.                                   |
| `style`                | `"white bold"`            | 這個模組的風格。                                                     |
| `disabled`             | `true`                    | Disables the `shell` module.                                 |

### Variables

| 變數        | 預設 | 說明                                                         |
| --------- | -- | ---------------------------------------------------------- |
| indicator |    | Mirrors the value of `indicator` for currently used shell. |
| style\* |    | Mirrors the value of option `style`.                       |

*: This variable can only be used as a part of a style string

### Examples

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

### 選項

| Option      | 預設                           | 說明                                                            |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | The format for the module.                                    |
| `symbol`    | `"↕️  "`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | 這個模組的風格。                                                      |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Variables

| 變數        | 範例  | 說明                                   |
| --------- | --- | ------------------------------------ |
| shlvl     | `3` | The current value of `SHLVL`         |
| symbol    |     | Mirrors the value of option `symbol` |
| style\* |     | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### 選項

| Option     | 預設                               | 說明                                               |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | The format for the module.                       |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | 這個模組的風格。                                         |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variables

| 變數        | 範例           | 說明                                   |
| --------- | ------------ | ------------------------------------ |
| env       | `centos.img` | The current Singularity image        |
| symbol    |              | Mirrors the value of option `symbol` |
| style\* |              | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### 選項

| Option              | 預設                                     | 說明                                                                                                                |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to. `0` 表示不截斷 也請參考 [`directory`](#directory)模組 |
| `symbol`            | `"🅢  "`                                | 環境名稱前使用的符號。                                                                                                       |
| `style`             | `"bold blue"`                          | 這個模組的風格。                                                                                                          |
| `format`            | `"via [$symbol$environment]($style) "` | The format for the module.                                                                                        |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                      |

### Variables

| 變數          | 範例           | 說明                                   |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current spack environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*   |              | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[spack]
format = "[$symbol$environment](dimmed blue) "
```

## Status

The `status` module displays the exit code of the previous command. If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`. The status code will cast to a signed 32-bit integer.

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| Option                      | 預設                                                                                 | 說明                                                                    |
| --------------------------- | ---------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `format`                    | `"[$symbol$status]($style) "`                                                      | The format of the module                                              |
| `symbol`                    | `"❌"`                                                                              | The symbol displayed on program error                                 |
| `success_symbol`            | `""`                                                                               | The symbol displayed on program success                               |
| `not_executable_symbol`     | `"🚫"`                                                                              | The symbol displayed when file isn't executable                       |
| `not_found_symbol`          | `"🔍"`                                                                              | The symbol displayed when the command can't be found                  |
| `sigint_symbol`             | `"🧱"`                                                                              | The symbol displayed on SIGINT (Ctrl + c)                             |
| `signal_symbol`             | `"⚡"`                                                                              | The symbol displayed on any signal                                    |
| `style`                     | `"bold red"`                                                                       | 這個模組的風格。                                                              |
| `recognize_signal_code`     | `true`                                                                             | Enable signal mapping from exit code                                  |
| `map_symbol`                | `false`                                                                            | Enable symbols mapping from exit code                                 |
| `pipestatus`                | `false`                                                                            | Enable pipestatus reporting                                           |
| `pipestatus_separator`      | <code>&vert;</code>                                                          | The symbol used to separate pipestatus segments (supports formatting) |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style)'` | The format of the module when the command is a pipeline               |
| `pipestatus_segment_format` |                                                                                    | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                             | Disables the `status` module.                                         |

### Variables

| 變數             | 範例      | 說明                                                                                          |
| -------------- | ------- | ------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                           |
| hex_status     | `0x7F`  | The exit code of the last command in hex                                                    |
| int            | `127`   | The exit code of the last command                                                           |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                         |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                             |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                        |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                |
| pipestatus     |         | Rendering of in pipeline programs's exit codes, this is only available in pipestatus_format |
| symbol         |         | Mirrors the value of option `symbol`                                                        |
| style\*      |         | Mirrors the value of option `style`                                                         |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴 "
success_symbol = "🟢 SUCCESS"
format = '[\[$symbol$common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

The `sudo` module displays if sudo credentials are currently cached. The module will only be shown if credentials are cached.

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| Option          | 預設                       | 說明                                                      |
| --------------- | ------------------------ | ------------------------------------------------------- |
| `format`        | `"[as $symbol]($style)"` | The format of the module                                |
| `symbol`        | `"🧙 "`                   | The symbol displayed when credentials are cached        |
| `style`         | `"bold blue"`            | 這個模組的風格。                                                |
| `allow_windows` | `false`                  | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                   | Disables the `sudo` module.                             |

### Variables

| 變數        | 範例 | 說明                                   |
| --------- | -- | ------------------------------------ |
| symbol    |    | Mirrors the value of option `symbol` |
| style\* |    | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

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

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift                          |
| `detect_extensions` | `["swift"]`                          | Which extensions should trigger this module.                              |
| `detect_files`      | `["Package.swift"]`                  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold 202"`                         | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `swift` module.                                              |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v5.2.4` | The version of `swift`               |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

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

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.                     |
| `detect_extensions` | `["tf", "tfplan", "tfstate"]`        | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".terraform"]`                     | Which folders should trigger this module.                                 |
| `style`             | `"bold 105"`                         | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `terraform` module.                                          |

### Variables

| 變數        | 範例         | 說明                                   |
| --------- | ---------- | ------------------------------------ |
| version   | `v0.12.24` | The version of `terraform`           |
| workspace | `default`  | The current Terraform workspace      |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

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

## 時間

`time` 模組顯示目前的**當地**時間. `format` 設定值被 [`chrono`](https://crates.io/crates/chrono) crate 用來控制時間如何顯示。 請看 [chrono 的 strftime 文件](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)來了解有那些選項可以使用。

::: tip

這個模組預設是停用的。 想要啟用它的話，請在設定檔中將 `disabled` 設定為 `false`。

:::

### 選項

| Option            | 預設                      | 說明                                                                                                    |
| ----------------- | ----------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                     |
| `use_12hr`        | `false`                 | 啟用 12 小時格式。                                                                                           |
| `time_format`     | 請看下列                    | 用來顯示時間的 [chrono 格式字串](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)。                |
| `style`           | `"bold yellow"`         | 這個模組的時間的風格。                                                                                           |
| `utc_time_offset` | `"local"`               | 設定相對於 UTC 的時差。 Range from -24 &lt; x &lt; 24. 允許使用浮點數來表示 30/45 分鐘時差的時區。                   |
| `disabled`        | `true`                  | 停用 `time` 模組。                                                                                         |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. 不然的話，它會被預設為 `"%T"`。 Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| 變數        | 範例         | 說明                                  |
| --------- | ---------- | ----------------------------------- |
| 時間        | `13:08:10` | The current time.                   |
| style\* |            | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## 使用者名稱

`username` 模組顯示現在使用中的使用者名稱。 這個模組在下列其中一個條件達成時顯示：

- The current user is root/admin
- 目前使用者並非登入時的使用者
- 使用者透過 SSH session 進行連線
- 變數 `show_always` 被設為 true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### 選項

| Option        | 預設                      | 說明                                          |
| ------------- | ----------------------- | ------------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root/admin. |
| `style_user`  | `"bold yellow"`         | 非 root 使用者時使用的風格。                           |
| `format`      | `"[$user]($style) in "` | The format for the module.                  |
| `show_always` | `false`                 | 總是顯示 `username` 模組。                         |
| `disabled`    | `false`                 | 停用 `username` 模組。                           |

### Variables

| 變數      | 範例           | 說明                                                                                          |
| ------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style` | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`  | `"matchai"`  | The currently logged-in user ID.                                                            |

### 範例

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

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Vagrantfile` file

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.                       |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["Vagrantfile"]`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"cyan bold"`                        | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                            |

### Variables

| 變數        | 範例               | 說明                                   |
| --------- | ---------------- | ------------------------------------ |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`             |
| symbol    |                  | Mirrors the value of option `symbol` |
| style\* |                  | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/). By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### 選項

| Option              | 預設                                           | 說明                                                                        |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"V "`                                       | A format string representing the symbol of V                              |
| `detect_extensions` | `["v"]`                                      | Which extensions should trigger this module.                              |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                         | Which folders should trigger this module.                                 |
| `style`             | `"blue bold"`                                | 這個模組的風格。                                                                  |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                              |

### Variables

| 變數        | 範例     | 說明                                   |
| --------- | ------ | ------------------------------------ |
| version   | `v0.2` | The version of `v`                   |
| symbol    |        | Mirrors the value of option `symbol` |
| style\* |        | Mirrors the value of option `style`  |

### 範例

```toml
# ~/.config/starship.toml
[vlang]
format = "via [V $version](blue bold) "
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### 選項

| Option     | 預設                               | 說明                                                     |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   |                                  | The symbol used before displaying the repository name. |
| `style`    | `"bold yellow"`                  | 這個模組的風格。                                               |
| `format`   | `"vcsh [$symbol$repo]($style) "` | The format for the module.                             |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variables

| 變數        | 範例                                          | 說明                                   |
| --------- | ------------------------------------------- | ------------------------------------ |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name           |
| symbol    |                                             | Mirrors the value of option `symbol` |
| style\* | `black bold dimmed`                         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). 這個模組在下列其中一個條件達成時顯示：

- The current directory contains a `.zig` file

### 選項

| Option              | 預設                                   | 說明                                                                        |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module.                                                |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig.                     |
| `style`             | `"bold yellow"`                      | 這個模組的風格。                                                                  |
| `disabled`          | `false`                              | Disables the `zig` module.                                                |
| `detect_extensions` | `["zig"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |

### Variables

| 變數        | 範例       | 說明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v0.6.0` | The version of `zig`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### 範例

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `detect_files`
- The current directory contains a directory whose name is in `detect_folders`
- The current directory contains a file whose extension is in `detect_extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matches with `os` field if defined.

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

### 選項

| Option              | 預設                              | 說明                                                                                                                                                                                                                                                                                            |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `""`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                                                                                                                                 |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the command returns a `0` status code.                                                                          |
| `shell`             |                                 | [See below](#custom-command-shell)                                                                                                                                                                                                                                                            |
| `說明`                | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                  |
| `detect_files`      | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                                                                                                                                         |
| `detect_folders`    | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                                                                                                                                   |
| `detect_extensions` | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                                                                                                                                    |
| `symbol`            | `""`                            | The symbol used before displaying the command output.                                                                                                                                                                                                                                         |
| `style`             | `"bold green"`                  | 這個模組的風格。                                                                                                                                                                                                                                                                                      |
| `format`            | `"[$symbol($output )]($style)"` | The format for the module.                                                                                                                                                                                                                                                                    |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                |
| `os`                |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                 |
| `use_stdin`         |                                 | An optional boolean value that overrides whether commands should be forwarded to the shell via the standard input or as an argument. If unset standard input is used by default, unless the shell does not support it (cmd, nushell). Setting this disables shell-specific argument handling. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                     |

### Variables

| 變數        | 說明                                     |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Mirrors the value of option `symbol`   |
| style\* | Mirrors the value of option `style`    |

*: This variable can only be used as a part of a style string

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. If `shell` is not given or only contains one element and Starship detects Cmd will be used, the following argument will automatically be added: `/C` and `stdin` will be set to `false`. If `shell` is not given or only contains one element and Starship detects Nushell will be used, the following arguments will automatically be added: `-c` and `stdin` will be set to `false`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ["pwsh", "-Command", "-"]
```

::: warning Make sure your custom shell configuration exits gracefully

If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).

For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.

Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.

Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

:::

### 範例

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo" # shows output of command
detect_files = ["foo"] # can specify filters but wildcards are not supported
when = """ test "$HOME" = "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
detect_extensions = ["pst"] # filters *.pst files
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]

[custom.time-as-arg]
command = "time /T"
detect_extensions = ["pst"] # filters *.pst files
shell = ["pwsh.exe", "-NoProfile", "-Command"]
use_stdin = false
```
