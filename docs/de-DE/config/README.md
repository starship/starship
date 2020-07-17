# Konfiguration

::: tip

🔥 Die Konfiguration befindet sich derzeit noch in der Entwicklung. Viele neue Konfigurationsoptionen werden in den kommenden Versionen verfügbar sein.

:::

Um mit der Konfiguration von Starship zu beginnen, musst du die folgende Datei erstellen: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Die gesamte Konfiguration von Starship wird über diese [TOML](https://github.com/toml-lang/toml)-Datei durchgeführt:

```toml
# Don't print a new line at the start of the prompt
add_newline = false

# Replace the "❯" symbol in the prompt with "➜"
[character]                            # The name of the module we are configuring is "character"
success_symbol = "[➜](bold green)"     # The "success_symbol" segment is being set to "➜" with the color "bold green"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

Sie können den Pfad zur `starship.toml` mit der `STARSHIP_CONFIG` Umgebungsvariable ändern:

```sh
export STARSHIP_CONFIG=~/.starship
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### Terminologie

**Module**: Eine Komponente in der Konsole, die auf kontextualisierte Informationen des OS basiert. Zum Beispiel zeigt das Modul "nodejs" die Version von NodeJS, die derzeit auf Ihrem Computer installiert ist, wenn Ihr aktuelles Verzeichnis ein NodeJS-Projekt ist.

**Variable**: Smaller sub-components that contains information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of NodeJS.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Format Strings

Format strings are the format that a module prints all its variables with. Most modules have an entry called `format` that configures the display format of the module. You can use texts, variables and text groups in a format string.

#### Variable

A variable contains a `$` symbol followed by the name of the variable. The name of a variable only contains letters, numbers and `_`.

For example:

- `$version` is a format string with a variable named `version`.
- `$git_branch$git_commit` is a format string with two variables named `git_branch` and `git_commit`.
- `$git_branch $git_commit` has the two variables separated with a space.

#### Text Group

A text group is made up of two different parts.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings). You can add texts, variables, or even nested text groups in it.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). This can be used style the first part.

For example:

- `[on](red bold)` will print a string `on` with bold text colored red.
- `[⬢ $version](bold green)` will print a symbol `⬢` followed by the content of variable `version`, with bold text colored green.
- `[a [b](red) c](green)` will print `a b c` with `b` red, and `a` and `c` green.

#### Style-Strings

Most modules in starship allow you to configure their display styles. This is done with an entry (usually called `style`) which is a string specifying the configuration. Here are some examples of style strings along with what they do. For details on the full syntax, consult the [advanced config guide](/advanced-config/).

- `"fg:green bg:blue"` sets green text on a blue background
- `"bg:blue fg:bright-green"` sets bright green text on a blue background
- `"bold fg:27"` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` sets underlined text on a burnt orange background
- `"bold italic fg:purple"` sets bold italic purple text
- `""` explicitly disables all styling

Note that what styling looks like will be controlled by your terminal emulator. For example, some terminal emulators will brighten the colors instead of bolding text, and some color themes use the same values for the normal and bright colors. Also, to get italic text, your terminal must support italics.

#### Conditional Format Strings

A conditional format string wrapped in `(` and `)` will not render if all variables inside are empty.

For example:

- `(@$region)` will show nothing if the variable `region` is `None`, otherwise `@` followed by the value of region.
- `(some text)` will always show nothing since there are no variables wrapped in the braces.
- When `$all` is a shortcut for `\[$a$b\]`, `($all)` will show nothing only if `$a` and `$b` are both `None`. This works the same as `(\[$a$b\] )`.

#### Escapable characters

The following symbols have special usage in a format string. If you want to print the following symbols, you have to escape them with a backslash (`\`).

- $
- \\
- [
- ]
- (
- )

Note that `toml` has [its own escape syntax](https://github.com/toml-lang/toml#user-content-string). It is recommended to use a literal string (`''`) in your config. If you want to use a basic string (`""`), pay attention to escape the backslash `\`.

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

### Optionen

| Option         | Standardwert                   | Beschreibung                                          |
| -------------- | ------------------------------ | ----------------------------------------------------- |
| `format`       | [link](#default-prompt-format) | Configure the format of the prompt.                   |
| `scan_timeout` | `30`                           | Timeout for starship to scan files (in milliseconds). |

### Beispiel

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
format = "$all"

# Use custom format
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10
```

### Default Prompt Format

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. The default is as shown:

```toml
format = "\n$all"

# Which is equivalent to
format = """

$username\
$hostname\
$kubernetes\
$directory\
$git_branch\
$git_commit\
$git_state\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$nim\
$nodejs\
$ocaml\
$php\
$purescript\
$python\
$ruby\
$rust\
$terraform\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$env_var\
$crystal\
$cmd_duration\
$custom\
$line_break\
$jobs\
$battery\
$time\
$character"""
```

## AWS

The `aws` module shows the current AWS region and profile. This is based on `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env var with `~/.aws/config` file.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var.

### Optionen

| Option           | Standardwert                                         | Beschreibung                                                    |
| ---------------- | ---------------------------------------------------- | --------------------------------------------------------------- |
| `format`         | `"on [$symbol$profile(\\($region\\))]($style) "` | The format for the module.                                      |
| `symbol`         | `"☁️ "`                                              | The symbol used before displaying the current AWS profile.      |
| `region_aliases` |                                                      | Table of region aliases to display in addition to the AWS name. |
| `style`          | `"bold yellow"`                                      | Stil für dieses Modul.                                          |
| `disabled`       | `false`                                              | Deaktiviert das `aws`-Modul.                                    |

### Variables

| Variable  | Beispiel         | Beschreibung                         |
| --------- | ---------------- | ------------------------------------ |
| region    | `ap-northeast-1` | The current AWS region               |
| profile   | `astronauts`     | The current AWS profile              |
| symbol    |                  | Mirrors the value of option `symbol` |
| style\* |                  | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Examples

#### Display everything

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile(\\($region\\))]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
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
```

## Akkustand

The `battery` module shows how charged the device's battery is and its current charging status. The module is only visible when the device's battery is below 10%.

### Optionen

| Option               | Standardwert                      | Beschreibung                                      |
| -------------------- | --------------------------------- | ------------------------------------------------- |
| `full_symbol`        | `"•"`                             | The symbol shown when the battery is full.        |
| `charging_symbol`    | `"⇡"`                             | The symbol shown when the battery is charging.    |
| `discharging_symbol` | `"⇣"`                             | The symbol shown when the battery is discharging. |
| `format`             | `"[$symbol$percentage]($style) "` | The format for the module.                        |
| `display`            | [link](#battery-display)          | Display threshold and style for the module.       |
| `disabled`           | `false`                           | Disables the `battery` module.                    |

<details>
<summary>There are also options for some uncommon battery states.</summary>

| Variable         | Beschreibung                                        |
| ---------------- | --------------------------------------------------- |
| `unknown_symbol` | The symbol shown when the battery state is unknown. |
| `empty_symbol`   | The symbol shown when the battery state is empty.   |

Note: Battery indicator will be hidden if the status is `unknown` or `empty` unless you specify the option in the config.

</details>

### Beispiel

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### Battery Display

The `display` configuration option is used to define when the battery indicator should be shown (threshold) and what it looks like (style). If no `display` is provided. The default is as shown:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Optionen

The `display` option is an array of the following table.

| Variable    | Beschreibung                                    |
| ----------- | ----------------------------------------------- |
| `threshold` | The upper bound for the display option.         |
| `style`     | The style used if the display option is in use. |

#### Beispiel

```toml
[[battery.display]]  # "bold red" style when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" style when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"

# when capacity is over 30%, the battery indicator will not be displayed

```

## Zeichen

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change it's shape take a look at [this example](#with-custom-error-shape).

### Optionen

| Option           | Standardwert        | Beschreibung                                                                     |
| ---------------- | ------------------- | -------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | The format string used before the text input.                                    |
| `success_symbol` | `"[❯](bold green)"` | The format string used before the text input if the previous command succeeded.  |
| `error_symbol`   | `"[❯](bold red)"`   | The format string used before the text input if the previous command failed.     |
| `vicmd_symbol`   | `"[❮](bold green)"` | The format string used before the text input if the shell is in vim normal mode. |
| `disabled`       | `false`             | Disables the `character` module.                                                 |

### Variables

| Variable | Beispiel | Beschreibung                                                          |
| -------- | -------- | --------------------------------------------------------------------- |
| symbol   |          | A mirror of either `success_symbol`, `error_symbol` or `vicmd_symbol` |

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

The `cmake` module shows the currently installed version of CMake if:

- The current directory contains a `CMakeLists.txt` file

### Optionen

| Option     | Standardwert                       | Beschreibung                                 |
| ---------- | ---------------------------------- | -------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                   |
| `symbol`   | `"🛆 "`                             | The symbol used before the version of cmake. |
| `style`    | `"bold blue"`                      | Stil für dieses Modul.                       |
| `disabled` | `false`                            | Disables the `cmake` module.                 |

### Variables

| Variable  | Beispiel  | Beschreibung                         |
| --------- | --------- | ------------------------------------ |
| version   | `v3.17.3` | The version of cmake                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

## Command Duration

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### Optionen

| Option              | Standardwert                  | Beschreibung                                               |
| ------------------- | ----------------------------- | ---------------------------------------------------------- |
| `min_time`          | `2_000`                       | Shortest duration to show time for (in milliseconds).      |
| `show_milliseconds` | `false`                       | Show milliseconds in addition to seconds for the duration. |
| `format`            | `"took [$duration]($style) "` | The format for the module.                                 |
| `style`             | `"bold yellow"`               | Stil für dieses Modul.                                     |
| `disabled`          | `false`                       | Disables the `cmd_duration` module.                        |

### Variables

| Variable  | Beispiel | Beschreibung                            |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | The time it took to execute the command |
| style\* |          | Mirrors the value of option `style`     |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

The `conda` module shows the current conda environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### Optionen

| Option              | Standardwert                       | Beschreibung                                                                                                                                                                                                |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `"🅒 "`                             | The symbol used before the environment name.                                                                                                                                                                |
| `style`             | `"bold green"`                     | Stil für dieses Modul.                                                                                                                                                                                      |
| `format`            | `"[$symbol$environment]($style) "` | The format for the module.                                                                                                                                                                                  |
| `disabled`          | `false`                            | Disables the `conda` module.                                                                                                                                                                                |

### Variables

| Variable    | Beispiel     | Beschreibung                         |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current conda environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*   |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `shard.yml` file
- The current directory contains a `.cr` file

### Optionen

| Option     | Standardwert                       | Beschreibung                                              |
| ---------- | ---------------------------------- | --------------------------------------------------------- |
| `symbol`   | `"🔮 "`                             | The symbol used before displaying the version of crystal. |
| `style`    | `"bold red"`                       | Stil für dieses Modul.                                    |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                                |
| `disabled` | `false`                            | Disables the `crystal` module.                            |

### Variables

| Variable  | Beispiel  | Beschreibung                         |
| --------- | --------- | ------------------------------------ |
| version   | `v0.32.1` | The version of `crystal`             |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Directory

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Optionen

| Variable                 | Standardwert                                    | Beschreibung                                                                     |
| ------------------------ | ----------------------------------------------- | -------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                             | The number of parent folders that the current directory should be truncated to.  |
| `truncate_to_repo`       | `true`                                          | Whether or not to truncate to the root of the git repo that you're currently in. |
| `format`                 | `"[$path]($style)[$lock_symbol]($lock_style) "` | The format for the module.                                                       |
| `style`                  | `"bold cyan"`                                   | Stil für dieses Modul.                                                           |
| `disabled`               | `false`                                         | Disables the `directory` module.                                                 |
| `read_only_symbol`       | `"🔒"`                                           | The symbol indicating current directory is read only.                            |
| `read_only_symbol_style` | `"red"`                                         | The style for the read only symbol.                                              |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | Standardwert | Beschreibung                                                                             |
| --------------------------- | ------------ | ---------------------------------------------------------------------------------------- |
| `substitutions`             |              | A table of substitutions to be made to the path.                                         |
| `fish_style_pwd_dir_length` | `0`          | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`       | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable  | Beispiel              | Beschreibung                        |
| --------- | --------------------- | ----------------------------------- |
| path      | `"D:/Projects"`       | The current directory path          |
| style\* | `"black bold dimmed"` | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### Optionen

| Option            | Standardwert                       | Beschreibung                                                                            |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | The format for the module.                                                              |
| `symbol`          | `"🐳 "`                             | The symbol used before displaying the Docker context.                                   |
| `style`           | `"blue bold"`                      | Stil für dieses Modul.                                                                  |
| `only_with_files` | `false`                            | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `disabled`        | `true`                             | Disables the `docker_context` module.                                                   |

### Variables

| Variable  | Beispiel       | Beschreibung                         |
| --------- | -------------- | ------------------------------------ |
| context   | `test_context` | The current docker context           |
| symbol    |                | Mirrors the value of option `symbol` |
| style\* |                | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one or more of the following files are present in the current directory:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.sln`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

You'll also need the .NET Core SDK installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### Optionen

| Option      | Standardwert                             | Beschreibung                                             |
| ----------- | ---------------------------------------- | -------------------------------------------------------- |
| `format`    | `"v[$symbol$version( 🎯 $tfm)]($style) "` | The format for the module.                               |
| `symbol`    | `"•NET "`                                | The symbol used before displaying the version of dotnet. |
| `heuristic` | `true`                                   | Use faster version detection to keep starship snappy.    |
| `style`     | `"bold blue"`                            | Stil für dieses Modul.                                   |
| `disabled`  | `false`                                  | Disables the `dotnet` module.                            |

### Variables

| Variable  | Beispiel         | Beschreibung                                                       |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Mirrors the value of option `symbol`                               |
| style\* |                  | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `mix.exs` file.

### Optionen

| Option     | Standardwert                                                  | Beschreibung                                                    |
| ---------- | ------------------------------------------------------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`                                                        | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"`                                               | Stil für dieses Modul.                                          |
| `format`   | `"via [$symbol$version \\(OTP $otp_version\\)]($style) "` | The format for the module elixir.                               |
| `disabled` | `false`                                                       | Disables the `elixir` module.                                   |

### Variables

| Variable    | Beispiel | Beschreibung                         |
| ----------- | -------- | ------------------------------------ |
| version     | `v1.10`  | The version of `elixir`              |
| otp_version |          | The otp version of `elixir`          |
| symbol      |          | Mirrors the value of option `symbol` |
| style\*   |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### Optionen

| Option     | Standardwert                       | Beschreibung                                    |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🌳 "`                             | A format string representing the symbol of Elm. |
| `style`    | `"cyan bold"`                      | Stil für dieses Modul.                          |
| `disabled` | `false`                            | Disables the `elm` module.                      |

### Variables

| Variable  | Beispiel  | Beschreibung                         |
| --------- | --------- | ------------------------------------ |
| version   | `v0.19.1` | The version of `elm`                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### Optionen

| Option     | Standardwert                   | Beschreibung                                                                 |
| ---------- | ------------------------------ | ---------------------------------------------------------------------------- |
| `symbol`   |                                | The symbol used before displaying the variable value.                        |
| `variable` |                                | The environment variable to be displayed.                                    |
| `default`  |                                | The default value to be displayed when the selected variable is not defined. |
| `format`   | `"with [$env_value]($style) "` | The format for the module.                                                   |
| `disabled` | `false`                        | Disables the `env_var` module.                                               |

### Variables

| Variable  | Beispiel                                    | Beschreibung                               |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if *variable* would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Mirrors the value of option `symbol`       |
| style\* | `black bold dimmed`                         | Mirrors the value of option `style`        |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `rebar.config` file.
- The current directory contains a `erlang.mk` file.

### Optionen

| Option     | Standardwert                       | Beschreibung                                             |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`                             | The symbol used before displaying the version of erlang. |
| `style`    | `"bold red"`                       | Stil für dieses Modul.                                   |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                               |
| `disabled` | `false`                            | Disables the `erlang` module.                            |

### Variables

| Variable  | Beispiel  | Beschreibung                         |
| --------- | --------- | ------------------------------------ |
| version   | `v22.1.3` | The version of `erlang`              |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Optionen

| Option              | Standardwert                     | Beschreibung                                                                             |
| ------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.  Use `"$branch"` to refer to the current branch name.         |
| `symbol`            | `" "`                           | A format string representing the symbol of git branch.                                   |
| `style`             | `"bold purple"`                  | Stil für dieses Modul.                                                                   |
| `truncation_length` | `2^63 - 1`                       | Truncates a git branch to X graphemes.                                                   |
| `truncation_symbol` | `"…"`                            | The symbol used to indicate a branch name was truncated. You can use `""` for no symbol. |
| `disabled`          | `false`                          | Disables the `git_branch` module.                                                        |

### Variables

| Variable  | Beispiel | Beschreibung                                                                                         |
| --------- | -------- | ---------------------------------------------------------------------------------------------------- |
| branch    | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached HEAD). |
| symbol    |          | Mirrors the value of option `symbol`                                                                 |
| style\* |          | Mirrors the value of option `style`                                                                  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the current commit hash of the repo in your current directory.

### Optionen

| Option               | Standardwert                   | Beschreibung                                          |
| -------------------- | ------------------------------ | ----------------------------------------------------- |
| `commit_hash_length` | `7`                            | The length of the displayed git commit hash.          |
| `format`             | `"[\\($hash\\)]($style) "` | The format for the module.                            |
| `style`              | `"bold green"`                 | Stil für dieses Modul.                                |
| `only_detached`      | `true`                         | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`                        | Disables the `git_commit` module.                     |

### Variables

| Variable  | Beispiel  | Beschreibung                        |
| --------- | --------- | ----------------------------------- |
| hash      | `b703eb3` | The current git commit hash         |
| style\* |           | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Optionen

| Option         | Standardwert                                                        | Beschreibung                                                                            |
| -------------- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                        | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                         | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                       | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                                  | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                       | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                              | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                       | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                     | Stil für dieses Modul.                                                                  |
| `format`       | `"[\\($state( $progress_current/$progress_total)\\)]($style) "` | The format for the module.                                                              |
| `disabled`     | `false`                                                             | Disables the `git_state` module.                                                        |

### Variables

| Variable         | Beispiel   | Beschreibung                        |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*        |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[git_state]
format = "[\\($state( $progress_current of $progress_total)\\)]($style) "
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Optionen

| Option            | Standardwert                                    | Beschreibung                                         |
| ----------------- | ----------------------------------------------- | ---------------------------------------------------- |
| `format`          | "([\[$all_status$ahead_behind\]]($style) )" | The default format for `git_status`                  |
| `conflicted`      | `"="`                                           | This branch has merge conflicts.                     |
| `ahead`           | `"⇡"`                                           | The format of `ahead`                                |
| `behind`          | `"⇣"`                                           | The format of `behind`                               |
| `diverged`        | `"⇕"`                                           | The format of `diverged`                             |
| `untracked`       | `"?"`                                           | The format of `untracked`                            |
| `stashed`         | `"$"`                                           | The format of `stashed`                              |
| `modified`        | `"!"`                                           | The format of `modified`                             |
| `staged`          | `"+"`                                           | The format of `staged`                               |
| `renamed`         | `"»"`                                           | The format of `renamed`                              |
| `deleted`         | `"✘"`                                           | The format of `deleted`                              |
| `show_sync_count` | `false`                                         | Show ahead/behind count of the branch being tracked. |
| `style`           | `"bold red"`                                    | Stil für dieses Modul.                               |
| `disabled`        | `false`                                         | Disables the `git_status` module.                    |

### Variables

The following variables can be used in `format`:

| Variable       | Beschreibung                                                                                  |
| -------------- | --------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                   |
| `ahead_behind` | Displays `diverged` `ahead` or `behind` format string based on the current status of the repo |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                   |
| `untracked`    | Displays `untracked`  when there are untracked files in the working directory.                |
| `stashed`      | Displays `stashed`    when a stash exists for the local repository.                           |
| `modified`     | Displays `modified`   when there are file modifications in the working directory.             |
| `staged`       | Displays `staged`     when a new file has been added to the staging area.                     |
| `renamed`      | Displays `renamed`    when a renamed file has been added to the staging area.                 |
| `deleted`      | Displays `deleted`    when a file's deletion has been added to the staging area.              |
| style\*      | Mirrors the value of option `style`                                                           |

\*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| Variable       | Beschreibung                                   |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Variable | Beschreibung             |
| -------- | ------------------------ |
| `count`  | Show the number of files |

### Beispiel

```toml
# ~/.config/starship.toml

[git_status]
conflicted = "🏳"
ahead = "🏎💨"
behind = "😰"
diverged = "😵"
untracked = "🤷‍"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

## Golang

The `golang` module shows the currently installed version of Golang. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Optionen

| Option     | Standardwert                       | Beschreibung                                   |
| ---------- | ---------------------------------- | ---------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                     |
| `symbol`   | `"🐹 "`                             | A format string representing the symbol of Go. |
| `style`    | `"bold cyan"`                      | Stil für dieses Modul.                         |
| `disabled` | `false`                            | Disables the `golang` module.                  |

### Variables

| Variable  | Beispiel  | Beschreibung                         |
| --------- | --------- | ------------------------------------ |
| version   | `v1.12.1` | The version of `go`                  |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of Helm. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `helmfile.yaml` file
- The current directory contains a `Chart.yaml` file

### Optionen

| Option     | Standartwert                       | Beschreibung                                     |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"⎈ "`                             | A format string representing the symbol of Helm. |
| `style`    | `"bold white"`                     | Stil für dieses Modul.                           |
| `disabled` | `false`                            | Disables the `helm` module.                      |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| version   | `v3.1.1` | The version of `helm`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

The `hostname` module shows the system hostname.

### Optionen

| Option     | Standartwert                | Beschreibung                                                                                                                         |
| ---------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                      | Only show hostname when connected to an SSH session.                                                                                 |
| `trim_at`  | `"."`                       | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `format`   | `"on [$hostname]($style) "` | The format for the module.                                                                                                           |
| `style`    | `"bold dimmed green"`       | Stil für dieses Modul.                                                                                                               |
| `disabled` | `false`                     | Disables the `hostname` module.                                                                                                      |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| number    | `1`      | The number of jobs                   |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of Java. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### Optionen

| Option     | Standartwert                           | Beschreibung                                    |
| ---------- | -------------------------------------- | ----------------------------------------------- |
| `format`   | `"via [${symbol}${version}]($style) "` | The format for the module.                      |
| `symbol`   | `"☕ "`                                 | A format string representing the symbol of Java |
| `style`    | `"red dimmed"`                         | Stil für dieses Modul.                          |
| `disabled` | `false`                                | Disables the `java` module.                     |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| version   | `v14`    | The version of `java`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### Optionen

| Option      | Standardwert                  | Beschreibung                                     |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | Show number of jobs if exceeded.                 |
| `format`    | `"[$symbol$number]($style) "` | The format for the module.                       |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | Stil für dieses Modul.                           |
| `disabled`  | `false`                       | Disables the `jobs` module.                      |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| number    | `1`      | The number of jobs                   |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Optionen

| Option     | Standardwert                       | Beschreibung                                      |
| ---------- | ---------------------------------- | ------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                        |
| `symbol`   | `"ஃ "`                             | A format string representing the symbol of Julia. |
| `style`    | `"bold purple"`                    | Stil für dieses Modul.                            |
| `disabled` | `false`                            | Disables the `julia` module.                      |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| version   | `v1.4.0` | The version of `julia`               |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Optionen

| Option                  | Standardwert                                             | Beschreibung                                                          |
| ----------------------- | -------------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`                | `"☸ "`                                                   | A format string representing the symbol displayed before the Cluster. |
| `format`                | `"on [$symbol$context( \\($namespace\\))]($style) "` | The format for the module.                                            |
| `style`                 | `"cyan bold"`                                            | Stil für dieses Modul.                                                |
| `namespace_spaceholder` | `none`                                                   | The value to display if no namespace was found.                       |
| `context_aliases`       |                                                          | Table of context aliases to display.                                  |
| `disabled`              | `true`                                                   | Disables the `kubernetes` module.                                     |

### Variables

| Variable  | Beispiel             | Beschreibung                             |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Mirrors the value of option `symbol`     |
| style\* |                      | Mirrors the value of option `style`      |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[kubernetes]
format = "on [⛵ $context \\($namespace\\)](dimmed green) "
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Optionen

| Option     | Standardwert | Beschreibung                                                       |
| ---------- | ------------ | ------------------------------------------------------------------ |
| `disabled` | `false`      | Disables the `line_break` module, making the prompt a single line. |

### Beispiel

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Optionen

| Option      | Standardwert                                  | Beschreibung                                             |
| ----------- | --------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                          | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | The format for the module.                               |
| `symbol`    | `"🐏"`                                         | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                         | Stil für dieses Modul.                                   |
| `disabled`  | `true`                                        | Disables the `memory_usage` module.                      |

### Variables

| Variable      | Beispiel      | Beschreibung                                                       |
| ------------- | ------------- | ------------------------------------------------------------------ |
| ram           | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct       | `48%`         | The percentage of the current system memory.                       |
| swap\**     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\** | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol        | `🐏`           | Mirrors the value of option `symbol`                               |
| style\*     |               | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

### Beispiel

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
show_percentage = true
show_swap = true
threshold = -1
symbol = " "
separator = "/"
style = "bold dimmed green"
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Optionen

| Option              | Standardwert                     | Beschreibung                                                                                 |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | Stil für dieses Modul.                                                                       |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.                                                                   |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`                            | The symbol used to indicate a branch name was truncated.                                     |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| branch    | `master` | The active mercurial branch          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of Nim. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Optionen

| Option     | Standardwert                       | Beschreibung                                          |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module                             |
| `symbol`   | `"👑 "`                             | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"`                    | Stil für dieses Modul.                                |
| `disabled` | `false`                            | Disables the `nim` module.                            |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| version   | `v1.2.0` | The version of `nimc`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### Optionen

| Option       | Standardwert                                       | Beschreibung                                          |
| ------------ | -------------------------------------------------- | ----------------------------------------------------- |
| `format`     | `"via [$symbol$state( \\($name\\))]($style) "` | The format for the module.                            |
| `symbol`     | `"❄️  "`                                           | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                      | Stil für dieses Modul.                                |
| `impure_msg` | `"impure"`                                         | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                           | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                            | Disables the `nix_shell` module.                      |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| state     | `pure`   | The state of the nix-shell           |
| name      | `lorri`  | The name of the nix-shell            |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = "via [☃️ $state( \\($name\\))](bold blue) "
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Optionen

| Option     | Standardwert                       | Beschreibung                                       |
| ---------- | ---------------------------------- | -------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                         |
| `symbol`   | `"⬢ "`                             | A format string representing the symbol of NodeJS. |
| `style`    | `"bold green"`                     | Stil für dieses Modul.                             |
| `disabled` | `false`                            | Disables the `nodejs` module.                      |

###  Variables

| Variable  | Beispiel   | Beschreibung                         |
| --------- | ---------- | ------------------------------------ |
| version   | `v13.12.0` | The version of `node`                |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia` and `mix` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present

> ⚠️ Die angezeigte Version ist die des Pakets, dessen Quellcode im Verzeichnis liegt, nicht die des Paketmanagers.

### Optionen

| Option            | Standardwert                       | Beschreibung                                               |
| ----------------- | ---------------------------------- | ---------------------------------------------------------- |
| `format`          | `"via [$symbol$version]($style) "` | The format for the module.                                 |
| `symbol`          | `"📦 "`                             | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"`                       | Stil für dieses Modul.                                     |
| `display_private` | `false`                            | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`                            | Disables the `package` module.                             |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| version   | `v1.0.0` | The version of your package          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Optionen

| Option     | Standardwert                       | Beschreibung                                            |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format string for the module.                       |
| `symbol`   | `"🐫 "`                             | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"`                    | Stil für dieses Modul.                                  |
| `disabled` | `false`                            | Disables the `ocaml` module.                            |

### Variables

| Variable  | Beispiel  | Beschreibung                         |
| --------- | --------- | ------------------------------------ |
| version   | `v4.10.0` | The version of `ocaml`               |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of PHP. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### Optionen

| Option     | Standardwert                       | Beschreibung                                          |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                            |
| `symbol`   | `"🐘 "`                             | The symbol used before displaying the version of PHP. |
| `style`    | `"147 bold"`                       | Stil für dieses Modul.                                |
| `disabled` | `false`                            | Disables the `php` module.                            |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| version   | `v7.3.8` | The version of `php`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- The current directory contains a `setup.py` file
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### Optionen

| Option               | Standardwert                                                   | Beschreibung                                                               |
| -------------------- | -------------------------------------------------------------- | -------------------------------------------------------------------------- |
| `format`             | `"via [${symbol}${version}( \\($virtualenv\\))]($style) "` | The format for the module.                                                 |
| `symbol`             | `"🐍 "`                                                         | A format string representing the symbol of Python                          |
| `style`              | `"yellow bold"`                                                | Stil für dieses Modul.                                                     |
| `pyenv_version_name` | `false`                                                        | Use pyenv to get Python version                                            |
| `scan_for_pyfiles`   | `true`                                                         | If false, Python files in the current directory will not show this module. |
| `disabled`           | `false`                                                        | Disables the `python` module.                                              |

### Variables

| Variable   | Beispiel        | Beschreibung                         |
| ---------- | --------------- | ------------------------------------ |
| version    | `"v3.8.1"`      | The version of `python`              |
| symbol     | `"🐍 "`          | Mirrors the value of option `symbol` |
| style      | `"yellow bold"` | Mirrors the value of option `style`  |
| virtualenv | `"venv"`        | The current `virtualenv` name        |

<details>
<summary>This module has some advanced configuration options.</summary>

| Variable        | Standardwert | Beschreibung                                                                  |
| --------------- | ------------ | ----------------------------------------------------------------------------- |
| `python_binary` | `python`     | Configures the python binary that Starship executes when getting the version. |

The `python_binary` variable changes the binary that Starship executes to get the version of Python, it doesn't change the arguments that are used.

```toml
# ~/.config/starship.toml

[python]
python_binary = "python3"
```

</details>

### Beispiel

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

The `ruby` module shows the currently installed version of Ruby. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Optionen

| Option     | Standardwert                       | Beschreibung                                     |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"💎 "`                             | A format string representing the symbol of Ruby. |
| `style`    | `"bold red"`                       | Stil für dieses Modul.                           |
| `disabled` | `false`                            | Disables the `ruby` module.                      |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| version   | `v2.5.1` | The version of `ruby`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Optionen

| Option     | Standardwert                       | Beschreibung                                    |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🦀 "`                             | A format string representing the symbol of Rust |
| `style`    | `"bold red"`                       | Stil für dieses Modul.                          |
| `disabled` | `false`                            | Disables the `rust` module.                     |

### Variables

| Variable  | Beispiel          | Beschreibung                         |
| --------- | ----------------- | ------------------------------------ |
| version   | `v1.43.0-nightly` | The version of `rustc`               |
| symbol    |                   | Mirrors the value of option `symbol` |
| style\* |                   | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### Optionen

| Option     | Standardwert                         | Beschreibung                                     |
| ---------- | ------------------------------------ | ------------------------------------------------ |
| `format`   | `"[$symbol\\[$env\\]]($style) "` | The format for the module.                       |
| `symbol`   | `""`                                 | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`                 | Stil für dieses Modul.                           |
| `disabled` | `false`                              | Disables the `singularity` module.               |

### Variables

| Variable  | Beispiel     | Beschreibung                         |
| --------- | ------------ | ------------------------------------ |
| env       | `centos.img` | The current singularity image        |
| symbol    |              | Mirrors the value of option `symbol` |
| style\* |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[singularity]
format = "[📦 \\[$env\\]]($style) "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-version). Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### Optionen

| Option     | Standardwert                         | Beschreibung                                          |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`   | `"💠 "`                               | A format string shown before the terraform workspace. |
| `style`    | `"bold 105"`                         | Stil für dieses Modul.                                |
| `disabled` | `false`                              | Disables the `terraform` module.                      |

### Variables

| Variable  | Beispiel   | Beschreibung                         |
| --------- | ---------- | ------------------------------------ |
| version   | `v0.12.24` | The version of `terraform`           |
| workspace | `default`  | The current terraform workspace      |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

#### With Version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Without version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Optionen

| Option            | Standardwert            | Beschreibung                                                                                                                       |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                  |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `"bold yellow"`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `"local"`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable  | Beispiel   | Beschreibung                        |
| --------- | ---------- | ----------------------------------- |
| time      | `13:08:10` | The current time.                   |
| style\* |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[\\[ $time \\]]($style) "
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

### Optionen

| Option        | Standardwert             | Beschreibung                          |
| ------------- | ------------------------ | ------------------------------------- |
| `style_root`  | `"bold red"`             | The style used when the user is root. |
| `style_user`  | `"bold yellow"`          | The style used for non-root users.    |
| `format`      | `"via [$user]($style) "` | The format for the module.            |
| `show_always` | `false`                  | Always shows the `username` module.   |
| `disabled`    | `false`                  | Disables the `username` module.       |

### Variables

| Variable | Beispiel     | Beschreibung                                                                                |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `"matchai"`  | The currently logged-in user ID.                                                            |

### Beispiel

```toml
# ~/.config/starship.toml

[username]
style_user = "white bold"
style_root = "black bold"
format = "user: [$user]($style) "
disabled = false
show_always = true
```

## Zig

The `zig` module shows the currently installed version of Zig. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `.zig` file

### Optionen

| Option     | Standardwert                       | Beschreibung                                          |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`                             | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"`                    | Stil für dieses Modul.                                |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                            |
| `disabled` | `false`                            | Disables the `zig` module.                            |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| version   | `v0.6.0` | The version of `zig`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

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

::: tip

Multiple custom modules can be defined by using a `.`.

:::

::: tip

The order in which custom modules are shown can be individually set by setting `custom.foo` in `prompt_order`. By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

### Optionen

| Option        | Standardwert                  | Beschreibung                                                                                                               |
| ------------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                               | The command whose output should be printed.                                                                                |
| `when`        |                               | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                               | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"`     | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                          | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                          | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                          | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                          | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                | Stil für dieses Modul.                                                                                                     |
| `format`      | `"[$symbol$output]($style) "` | The format for the module.                                                                                                 |
| `disabled`    | `false`                       | Disables this `custom` module.                                                                                             |

### Variables

| Variable  | Beschreibung                           |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Mirrors the value of option `symbol`   |
| style\* | Mirrors the value of option `style`    |

\*: This variable can only be used as a part of a style string

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

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

### Beispiel

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
prefix = " transcending "

[custom.time]
command = "time /T"
files = ["*.pst"]
prefix = "transcending "
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```

## PureScript

The `purescript` module shows the currently installed version of PureScript version. Das Modul wird gezeigt, wenn mindestens einer der folgenden Punkte erfüllt ist:

- The current directory contains a `spago.dhall` file
- The current directory contains a \*.purs files

### Optionen

| Option     | Standardwert                       | Beschreibung                                                 |
| ---------- | ---------------------------------- | ------------------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                                   |
| `symbol`   | `"<=> "`                     | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"`                     | Stil für dieses Modul.                                       |
| `disabled` | `false`                            | Disables the `purescript` module.                            |

### Variables

| Variable  | Beispiel | Beschreibung                         |
| --------- | -------- | ------------------------------------ |
| version   | `0.13.5` | The version of `purescript`          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Beispiel

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```
