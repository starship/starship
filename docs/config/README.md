# Configuration

To get started configuring starship, create the following file: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

All configuration for starship is done in this [TOML](https://github.com/toml-lang/toml) file:

```toml
# Get editor completions based on the config schema
"$schema" = 'https://starship.rs/config-schema.json'

# Inserts a blank line between shell prompts
add_newline = true

# Replace the '❯' symbol in the prompt with '➜'
[character] # The name of the module we are configuring is 'character'
success_symbol = '[➜](bold green)' # The 'success_symbol' segment is being set to '➜' with the color 'bold green'

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

### Config File Location

You can change default configuration file location with `STARSHIP_CONFIG` environment variable:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Logging

By default starship logs warnings and errors into a file named `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, where the session key is corresponding to an instance of your terminal.
This, however can be changed using the `STARSHIP_CACHE` environment variable:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Terminology

**Module**: A component in the prompt giving information based on contextual information from your OS. For example, the "nodejs" module shows the version of Node.js that is currently installed on your computer, if your current directory is a Node.js project.

**Variable**: Smaller sub-components that contain information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of Node.js.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Strings

In TOML syntax, [text values](https://toml.io/en/v1.0.0#string) are declared with `'`, `"`, `'''`, or `"""`.

The following Starship syntax symbols have special usage in a format string and must be escaped to display as that character: `$ [ ] ( )`.

| Symbol | Type                      | Notes                                                  |
| ------ | ------------------------- | ------------------------------------------------------ |
| `'`    | literal string            | less escaping                                          |
| `"`    | string                    | more escaping                                          |
| `'''`  | multi-line literal string | less escaping                                          |
| `"""`  | multi-line string         | more escaping, newlines in declarations can be ignored |

For example:

```toml
# literal string
format = '☺\☻ '

# regular string
format = "☺\\☻ "

# escaping Starship symbols
format = '\[\$\] '
```

When using line breaks, multi-line declarations can be used.
For example, if you want to print a `$` symbol on a new line, the following values for `format` are equivalent:

```toml
# with literal string
format = '''

\$'''

# with multiline basic string
format = """

\\$"""

# with basic string
format = "\n\\$"
```

In multiline basic strings, newlines can be used for formatting without being present in the value by escaping them.

```toml
format = """
line1\
line1\
line1
line2\
line2\
line2
"""
```

### Format Strings

Format strings are the format that a module prints all its variables with.
Most modules have an entry called `format` that configures the display format of the module.
You can use texts, variables and text groups in a format string.

#### Variable

A variable contains a `$` symbol followed by the name of the variable.
The name of a variable can only contain letters, numbers and `_`.

For example:

- `'$version'` is a format string with a variable named `version`.
- `'$git_branch$git_commit'` is a format string with two variables named `git_branch` and `git_commit`.
- `'$git_branch $git_commit'` has the two variables separated with a space.

#### Text Group

A text group is made up of two different parts.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings).
You can add texts, variables, or even nested text groups in it.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). This can be used to style the first part.

For example:

- `'[on](red bold)'` will print a string `on` with bold text colored red.
- `'[⌘ $version](bold green)'` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `'[a [b](red) c](green)'` will print `a b c` with `b` red, and `a` and `c` green.

#### Style Strings

Most modules in starship allow you to configure their display styles. This is done with an entry (usually called `style`) which is a string specifying the configuration. Here are some examples of style strings along with what they do. For details on the full syntax, consult the [advanced config guide](../advanced-config/).

- `'fg:green bg:blue'` sets green text on a blue background
- `'bg:blue fg:bright-green'` sets bright green text on a blue background
- `'bold fg:27'` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `'underline bg:#bf5700'` sets underlined text on a burnt orange background
- `'bold italic fg:purple'` sets bold italic purple text
- `''` explicitly disables all styling

Note that what styling looks like will be controlled by your terminal emulator. For example, some terminal emulators will brighten the colors instead of bolding text, and some color themes use the same values for the normal and bright colors. Also, to get italic text, your terminal must support italics.

#### Conditional Format Strings

A conditional format string wrapped in `(` and `)` will not render if all variables inside are empty.

For example:

- `'(@$region)'` will show nothing if the variable `region` is `None` or empty string, otherwise `@` followed by the value of region.
- `'(some text)'` will always show nothing since there are no variables wrapped in the braces.
- When `$combined` is a shortcut for `\[$a$b\]`, `'($combined)'` will show nothing only if `$a` and `$b` are both `None`.
  This works the same as `'(\[$a$b\] )'`.

### Negative matching

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. These take
lists of strings to match or not match. "Negative" options, those which should not be matched, are
indicated with a leading '!' character. The presence of _any_ negative indicator in the directory
will result in the module not being matched.

Extensions are matched against both the characters after the last dot in a filename, and the
characters after the first dot in a filename. For example, `foo.bar.tar.gz` will be matched
against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Files whose name begins with a
dot are not considered to have extensions at all.

To see how this works in practice, you could match TypeScript but not MPEG Transport Stream files thus:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Prompt

This is the list of prompt-wide configuration options.

### Options

| Option            | Default                        | Description                                                                                                                                                                        |
| ----------------- | ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | Configure the format of the prompt.                                                                                                                                                |
| `right_format`    | `''`                           | See [Enable Right Prompt](../advanced-config/#enable-right-prompt)                                                                                                                 |
| `scan_timeout`    | `30`                           | Timeout for starship to scan files (in milliseconds).                                                                                                                              |
| `command_timeout` | `500`                          | Timeout for commands executed by starship (in milliseconds).                                                                                                                       |
| `add_newline`     | `true`                         | Inserts blank line between shell prompts.                                                                                                                                          |
| `palette`         | `''`                           | Sets which color palette from `palettes` to use.                                                                                                                                   |
| `palettes`        | `{}`                           | Collection of color palettes that assign [colors](../advanced-config/#style-strings) to user-defined names. Note that color palettes cannot reference their own color definitions. |
| `follow_symlinks` | `true`                         | Follows symlinks to check if they're directories; used in modules such as git.                                                                                                     |

> [!TIP]
> If you have symlinks to networked filesystems, consider setting
> `follow_symlinks` to `false`.

### Example

```toml
# ~/.config/starship.toml

# Use custom format
format = '''
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) '''

# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10

# Disable the blank line at the start of the prompt
add_newline = false

# Set 'foo' as custom color palette
palette = 'foo'

# Define custom colors
[palettes.foo]
# Overwrite existing color
blue = '21'
# Define new color
mustard = '#af8700'
```

### Default Prompt Format

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. The default is as shown:

```toml
format = '$all'

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
$fossil_branch\
$fossil_metrics\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$hg_state\
$pijul_channel\
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
$fennel\
$fortran\
$gleam\
$golang\
$guix_shell\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$gradle\
$lua\
$maven\
$nim\
$nodejs\
$ocaml\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$quarto\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$solidity\
$swift\
$terraform\
$typst\
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
$nats\
$direnv\
$env_var\
$mise\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$os\
$container\
$netns\
$shell\
$character"""
```

If you just want to extend the default format, you can use `$all`;
modules you explicitly add to the format will not be duplicated. Eg.

```toml
# Move the directory to the second line
format = '$all$directory$character'
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials.
The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process`, `sso_start_url`, or `sso_session` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice.
If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile
is read from the `AWS_VAULT` env var and the credentials expiration date
is read from the `AWS_SESSION_EXPIRATION` env var.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile
is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile
is read from the `AWSUME_PROFILE` env var and the credentials expiration
date is read from the `AWSUME_EXPIRATION` env var.

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials`
falls back to the `x_security_token_expires` key.

When using [aws-sso-cli](https://github.com/synfinatic/aws-sso-cli) the profile
is read from the `AWS_SSO_PROFILE` env var.

### Options

| Option              | Default                                                           | Description                                                                                                 |
| ------------------- | ----------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | The format for the module.                                                                                  |
| `symbol`            | `'☁️ '`                                                            | The symbol used before displaying the current AWS profile.                                                  |
| `region_aliases`    | `{}`                                                              | Table of region aliases to display in addition to the AWS name.                                             |
| `profile_aliases`   | `{}`                                                              | Table of profile aliases to display in addition to the AWS name.                                            |
| `style`             | `'bold yellow'`                                                   | The style for the module.                                                                                   |
| `expiration_symbol` | `'X'`                                                             | The symbol displayed when the temporary credentials have expired.                                           |
| `disabled`          | `false`                                                           | Disables the `AWS` module.                                                                                  |
| `force_display`     | `false`                                                           | If `true` displays info even if `credentials`, `credential_process` or `sso_start_url` have not been setup. |

### Variables

| Variable | Example          | Description                                 |
| -------- | ---------------- | ------------------------------------------- |
| region   | `ap-northeast-1` | The current AWS region                      |
| profile  | `astronauts`     | The current AWS profile                     |
| duration | `2h27m20s`       | The temporary credentials validity duration |
| symbol   |                  | Mirrors the value of option `symbol`        |
| style\*  |                  | Mirrors the value of option `style`         |

*: This variable can only be used as a part of a style string

### Examples

#### Display everything

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
[aws.profile_aliases]
CompanyGroupFrobozzOnCallAccess = 'Frobozz'
```

#### Display region

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$region]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
```

#### Display profile

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$profile]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.profile_aliases]
Enterprise_Naming_Scheme-voidstars = 'void**'
```

## Azure

The `azure` module shows the current Azure Subscription. This is based on showing the name of the default subscription or the username, as defined in the `~/.azure/azureProfile.json` file.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Variable               | Default                                  | Description                                                                           |
| ---------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------- |
| `format`               | `'on [$symbol($subscription)]($style) '` | The format for the Azure module to render.                                            |
| `symbol`               | `'󰠅 '`                                   | The symbol used in the format.                                                        |
| `style`                | `'blue bold'`                            | The style used in the format.                                                         |
| `disabled`             | `true`                                   | Disables the `azure` module.                                                          |
| `subscription_aliases` | `{}`                                     | Table of subscription name aliases to display in addition to Azure subscription name. |

### Examples

#### Display Subscription Name

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = '󰠅 '
style = 'blue bold'
```

#### Display Username

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "󰠅 "
style = "blue bold"
```

#### Display Subscription Name Alias

```toml
# ~/.config/starship.toml

[azure.subscription_aliases]
very-long-subscription-name = 'vlsn'
```

## Battery

The `battery` module shows how charged the device's battery is and its current charging status.
The module is only visible when the device's battery is below 10%.

### Options

| Option               | Default                           | Description                                         |
| -------------------- | --------------------------------- | --------------------------------------------------- |
| `full_symbol`        | `'󰁹 '`                            | The symbol shown when the battery is full.          |
| `charging_symbol`    | `'󰂄 '`                            | The symbol shown when the battery is charging.      |
| `discharging_symbol` | `'󰂃 '`                            | The symbol shown when the battery is discharging.   |
| `unknown_symbol`     | `'󰁽 '`                            | The symbol shown when the battery state is unknown. |
| `empty_symbol`       | `'󰂎 '`                            | The symbol shown when the battery state is empty.   |
| `format`             | `'[$symbol$percentage]($style) '` | The format for the module.                          |
| `display`            | [link](#battery-display)          | Display threshold and style for the module.         |
| `disabled`           | `false`                           | Disables the `battery` module.                      |

### Example

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### Battery Display

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style).
If no `display` is provided. The default is as shown:

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Options

The `display` option is an array of the following table.

| Option               | Default      | Description                                                                                               |
| -------------------- | ------------ | --------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | The upper bound for the display option.                                                                   |
| `style`              | `'red bold'` | The style used if the display option is in use.                                                           |
| `charging_symbol`    |              | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.    |
| `discharging_symbol` |              | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option. |

#### Example

```toml
[[battery.display]] # 'bold red' style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = 'bold red'

[[battery.display]] # 'bold yellow' style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦 '

# when capacity is over 30%, the battery indicator will not be displayed
```

## Buf

The `buf` module shows the currently installed version of [Buf](https://buf.build). By default, the module is shown if the current directory contains a [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), or [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) configuration file.

### Options

| Option              | Default                                         | Description                                           |
| ------------------- | ----------------------------------------------- | ----------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | The format for the `buf` module.                      |
| `version_format`    | `'v${raw}'`                                     | The version format.                                   |
| `symbol`            | `'🐃 '`                                         | The symbol used before displaying the version of Buf. |
| `detect_extensions` | `[]`                                            | Which extensions should trigger this module.          |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                            | Which folders should trigger this modules.            |
| `style`             | `'bold blue'`                                   | The style for the module.                             |
| `disabled`          | `false`                                         | Disables the `elixir` module.                         |

### Variables

| Variable  | Example  | Description                          |
| --------- | -------- | ------------------------------------ |
| `version` | `v1.0.0` | The version of `buf`                 |
| `symbol`  |          | Mirrors the value of option `symbol` |
| `style`*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime.
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `bun.lock` file
- The current directory contains a `bun.lockb` file
- The current directory contains a `bunfig.toml` file

### Options

| Option              | Default                                    | Description                                                               |
| ------------------- | ------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`       | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🥟 '`                                    | A format string representing the symbol of Bun.                           |
| `detect_extensions` | `[]`                                       | Which extensions should trigger this module.                              |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                       | Which folders should trigger this module.                                 |
| `style`             | `'bold red'`                               | The style for the module.                                                 |
| `disabled`          | `false`                                    | Disables the `bun` module.                                                |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v0.1.4` | The version of `bun`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

#### Customize the format

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

The `c` module shows some information about your C compiler. By default
the module will be shown if the current directory contains a `.c` or `.h`
file.

### Options

| Option              | Default                                                                       | Description                                                               |
| ------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | The format string for the module.                                         |
| `version_format`    | `'v${raw}'`                                                                   | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'C '`                                                                        | The symbol used before displaying the compiler details                    |
| `detect_extensions` | `['c', 'h']`                                                                  | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                                                          | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                          | Which folders should trigger this module.                                 |
| `commands`          | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | How to detect what the compiler is                                        |
| `style`             | `'bold 149'`                                                                  | The style for the module.                                                 |
| `disabled`          | `false`                                                                       | Disables the `c` module.                                                  |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| name     | clang   | The name of the compiler             |
| version  | 13.0.0  | The version of the compiler          |
| symbol   |         | Mirrors the value of option `symbol` |
| style    |         | Mirrors the value of option `style`  |

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship will try executing each command until it gets a result on STDOUT.

If a C compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Example

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## CPP

The `cpp` module shows some information about your `C++` compiler. By default,
the module will be shown if the current directory contains a `.cpp`, `.hpp`, or other `C++`-related files.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                                                                          | Description                                                               |
| ------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                     | The format string for the module.                                         |
| `version_format`    | `'v${raw}'`                                                                      | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'C++ '`                                                                         | The symbol used before displaying the compiler details                    |
| `detect_extensions` | `['cpp', 'cc', 'cxx', 'c++', 'hpp', 'hh', 'hxx', 'h++', 'tcc']`                  | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                                                             | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                             | Which folders should trigger this module.                                 |
| `commands`          | `[ [ 'c++', '--version' ], [ 'g++', '--version' ], [ 'clang++', '--version' ] ]` | How to detect what the compiler is                                        |
| `style`             | `'bold 149'`                                                                     | The style for the module.                                                 |
| `disabled`          | `true`                                                                           | Disables the `cpp` module.                                                |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| name     | clang++ | The name of the compiler             |
| version  | 13.0.0  | The version of the compiler          |
| symbol   |         | Mirrors the value of option `symbol` |
| style    |         | Mirrors the value of option `style`  |

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycpp', '--version']`. Starship will try executing each command until it gets a result on STDOUT.

If a C++ compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Example

```toml
# ~/.config/starship.toml

[cpp]
disabled = false
format = 'via [$name $version]($style)'
```

## Character

The `character` module shows a character (usually an arrow) beside where the text
is entered in your terminal.

The character will tell you whether the last command was successful or not. It
can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change its shape take a
look at [this example](#with-custom-error-shape).

> [!WARNING]
> `vimcmd_symbol` is only supported in cmd, fish and zsh.
> `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol`
> are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

### Options

| Option                      | Default              | Description                                                                             |
| --------------------------- | -------------------- | --------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | The format string used before the text input.                                           |
| `success_symbol`            | `'[❯](bold green)'`  | The format string used before the text input if the previous command succeeded.         |
| `error_symbol`              | `'[❯](bold red)'`    | The format string used before the text input if the previous command failed.            |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | The format string used before the text input if the shell is in vim normal mode.        |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim `replace_one` mode. |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim replace mode.       |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | The format string used before the text input if the shell is in vim visual mode.        |
| `disabled`                  | `false`              | Disables the `character` module.                                                        |

### Variables

| Variable | Example | Description                                                                                              |
| -------- | ------- | -------------------------------------------------------------------------------------------------------- |
| symbol   |         | A mirror of either `success_symbol`, `error_symbol`, `vimcmd_symbol` or `vimcmd_replace_one_symbol` etc. |

### Examples

#### With custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[✗](bold red) '
```

#### Without custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[➜](bold red) '
```

#### With custom vim shape

```toml
# ~/.config/starship.toml

[character]
vimcmd_symbol = '[V](bold green) '
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). By default
the module will be activated if any of the following conditions are met:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### Options

| Option              | Default                                | Description                                                               |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`   | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                            | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'△ '`                                 | The symbol used before the version of cmake.                              |
| `detect_extensions` | `[]`                                   | Which extensions should trigger this module                               |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | Which filenames should trigger this module                                |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module                                  |
| `style`             | `'bold blue'`                          | The style for the module.                                                 |
| `disabled`          | `false`                                | Disables the `cmake` module.                                              |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v3.17.3` | The version of cmake                 |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL.
By default, the module will be shown if any of the following conditions are met:

- The current directory contains any files ending in `.cob` or `.COB`
- The current directory contains any files ending in `.cbl` or `.CBL`

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `symbol`            | `'⚙️ '`                               | The symbol used before displaying the version of COBOL.                   |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold blue'`                        | The style for the module.                                                 |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `cobol` module.                                              |

### Variables

| Variable | Example    | Description                          |
| -------- | ---------- | ------------------------------------ |
| version  | `v3.1.2.0` | The version of `cobol`               |
| symbol   |            | Mirrors the value of option `symbol` |
| style\*  |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

## Command Duration

The `cmd_duration` module shows how long the last command took to execute.
The module will be shown only if the command took longer than two seconds, or
the `min_time` config value, if it exists.

> [!WARNING]
> Do not hook the DEBUG trap in Bash
>
> If you are running Starship in `bash`, do not hook the `DEBUG` trap after running
> `eval $(starship init $0)`, or this module **will** break.

Bash users who need preexec-like functionality can use
[rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec).
Simply define the arrays `preexec_functions` and `precmd_functions` before
running `eval $(starship init $0)`, and then proceed as normal.

### Options

| Option                 | Default                       | Description                                                                                                                                                       |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Shortest duration to show time for (in milliseconds).                                                                                                             |
| `show_milliseconds`    | `false`                       | Show milliseconds in addition to seconds for the duration.                                                                                                        |
| `format`               | `'took [$duration]($style) '` | The format for the module.                                                                                                                                        |
| `style`                | `'bold yellow'`               | The style for the module.                                                                                                                                         |
| `disabled`             | `false`                       | Disables the `cmd_duration` module.                                                                                                                               |
| `show_notifications`   | `false`                       | Show desktop notifications when command completes.                                                                                                                |
| `min_time_to_notify`   | `45_000`                      | Shortest duration for notification (in milliseconds).                                                                                                             |
| `notification_timeout` |                               | Duration to show notification for (in milliseconds). If unset, notification timeout will be determined by daemon. Not all notification daemons honor this option. |

### Variables

| Variable | Example  | Description                             |
| -------- | -------- | --------------------------------------- |
| duration | `16m40s` | The time it took to execute the command |
| style\*  |          | Mirrors the value of option `style`     |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

> [!TIP]
> This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.
> If you use [pixi](https://pixi.sh), you can disable pixi's prompt modifier by running `pixi config set shell.change-ps1 false`.

### Options

| Option              | Default                                | Description                                                                                                                                                                                                 |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `'🅒 '`                                 | The symbol used before the environment name.                                                                                                                                                                |
| `style`             | `'bold green'`                         | The style for the module.                                                                                                                                                                                   |
| `format`            | `'via [$symbol$environment]($style) '` | The format for the module.                                                                                                                                                                                  |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                                                                                                                  |
| `detect_env_vars`   | `["!PIXI_ENVIRONMENT_NAME"]`           | Which environment variable(s) should trigger this module. If it's a pixi environment, this module is not being triggered by default.                                                                        |
| `disabled`          | `false`                                | Disables the `conda` module.                                                                                                                                                                                |

### Variables

| Variable    | Example      | Description                          |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current conda environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*     |              | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## Container

The `container` module displays a symbol and container name, if inside a container.

### Options

| Option     | Default                          | Description                               |
| ---------- | -------------------------------- | ----------------------------------------- |
| `symbol`   | `'⬢'`                            | The symbol shown, when inside a container |
| `style`    | `'bold red dimmed'`              | The style for the module.                 |
| `format`   | `'[$symbol \[$name\]]($style) '` | The format for the module.                |
| `disabled` | `false`                          | Disables the `container` module.          |

### Variables

| Variable | Example             | Description                          |
| -------- | ------------------- | ------------------------------------ |
| name     | `fedora-toolbox:35` | The name of the container            |
| symbol   |                     | Mirrors the value of option `symbol` |
| style\*  |                     | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `shard.yml` file
- The current directory contains a `.cr` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `symbol`            | `'🔮 '`                              | The symbol used before displaying the version of crystal.                 |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | The style for the module.                                                 |
| `detect_extensions` | `['cr']`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `['shard.yml']`                      | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `crystal` module.                                            |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v0.32.1` | The version of `crystal`             |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers)
SDK version when you are in the root directory of your Daml project. The `sdk-version` in
the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION`
environment variable.
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `daml.yaml` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'Λ '`                               | A format string representing the symbol of Daml                           |
| `style`             | `'bold cyan'`                        | The style for the module.                                                 |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['daml.yaml']`                      | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `daml` module.                                               |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v2.2.0` | The version of `daml`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml`, `pubspec.yml` or `pubspec.lock` file

### Options

| Option              | Default                                           | Description                                                               |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`              | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                       | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🎯 '`                                           | A format string representing the symbol of Dart                           |
| `detect_extensions` | `['dart']`                                        | Which extensions should trigger this module.                              |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `['.dart_tool']`                                  | Which folders should trigger this module.                                 |
| `style`             | `'bold blue'`                                     | The style for the module.                                                 |
| `disabled`          | `false`                                           | Disables the `dart` module.                                               |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v2.8.4` | The version of `dart`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### Options

| Option              | Default                                                                              | Description                                                               |
| ------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                                                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦕 '`                                                                              | A format string representing the symbol of Deno                           |
| `detect_extensions` | `[]`                                                                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                 | Which folders should trigger this module.                                 |
| `style`             | `'green bold'`                                                                       | The style for the module.                                                 |
| `disabled`          | `false`                                                                              | Disables the `deno` module.                                               |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v1.8.3` | The version of `deno`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Directory

The `directory` module shows the path to your current directory, truncated to
three parent folders. Your directory will also be truncated to the root of the
git repo that you're currently in.

When using the `fish_style_pwd_dir_length` option, instead of hiding the path that is
truncated, you will see a shortened name of each directory based on the number
you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root,
and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before
it would have been `nixpkgs/pkgs`.

### Options

| Option                   | Default                                                                                                                      | Description                                                                                                |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | The number of parent folders that the current directory should be truncated to.                            |
| `truncate_to_repo`       | `true`                                                                                                                       | Whether or not to truncate to the root of the git repo that you're currently in.                           |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | The format for the module.                                                                                 |
| `style`                  | `'bold cyan'`                                                                                                                | The style for the module.                                                                                  |
| `disabled`               | `false`                                                                                                                      | Disables the `directory` module.                                                                           |
| `read_only`              | `'🔒'`                                                                                                                       | The symbol indicating current directory is read only.                                                      |
| `read_only_style`        | `'red'`                                                                                                                      | The style for the read only symbol.                                                                        |
| `truncation_symbol`      | `''`                                                                                                                         | The symbol to prefix to truncated paths. eg: '…/'                                                          |
| `before_repo_root_style` |                                                                                                                              | The style for the path segment above the root of the git repo. The default value is equivalent to `style`. |
| `repo_root_style`        |                                                                                                                              | The style for the root of the git repo. The default value is equivalent to `style`.                        |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | The format of a git repo when `before_repo_root_style` and `repo_root_style` is defined.                   |
| `home_symbol`            | `'~'`                                                                                                                        | The symbol indicating home directory.                                                                      |
| `use_os_path_sep`        | `true`                                                                                                                       | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows)                       |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | Default | Description                                                                                                                                                            |
| --------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |         | An Array or table of substitutions to be made to the path.                                                                                                             |
| `fish_style_pwd_dir_length` | `0`     | The number of characters to use when applying fish shell pwd path logic.                                                                                               |
| `use_logical_path`          | `true`  | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network
prefixes or development directories of Java. Note that this will disable the fish style PWD. It takes an array of the following
key/value pairs:

| Value   | Type    | Description                              |
| ------- | ------- | ---------------------------------------- |
| `from`  | String  | The value to substitute                  |
| `to`    | String  | The replacement for that value, if found |
| `regex` | Boolean | (Optional) Whether `from` is a regex     |

By using `regex = true`, you can use [Rust's regular expressions](https://docs.rs/regex/latest/regex/#syntax) in `from`.
For instance you can replace every slash except the first with the following:

```toml
substitutions = [
  { from = "^/", to = "<root>/", regex = true },
  { from = "/", to = " | " },
  { from = "^<root>", to = "/", regex = true },
]
```

This will replace `/var/log` to `/ | var | log`.

The old syntax still works, although it doesn't support regular expressions:

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero,
the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path
`/built/this/city/on/rock/and/roll`, which would normally be displayed as `rock/and/roll`, would be displayed as
`/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with
a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| Variable | Example               | Description                         |
| -------- | --------------------- | ----------------------------------- |
| path     | `'D:/Projects'`       | The current directory path          |
| style\*  | `'black bold dimmed'` | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

<details>
<summary>The git repos have additional variables.</summary>

Let us consider the path `/path/to/home/git_repo/src/lib`

| Variable         | Example               | Description                             |
| ---------------- | --------------------- | --------------------------------------- |
| before_root_path | `'/path/to/home/'`    | The path before git root directory path |
| repo_root        | `'git_repo'`          | The git root directory name             |
| path             | `'/src/lib'`          | The remaining path                      |
| style            | `'black bold dimmed'` | Mirrors the value of option `style`     |
| repo_root_style  | `'underline white'`   | Style for git root directory name       |

</details>

### Example

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Direnv

The `direnv` module shows the status of the current rc file if one is present. The status includes the path to the rc file, whether it is loaded, and whether it has been allowed by `direnv`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                                | Description                                             |
| ------------------- | -------------------------------------- | ------------------------------------------------------- |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | The format for the module.                              |
| `symbol`            | `'direnv '`                            | The symbol used before displaying the direnv context.   |
| `style`             | `'bold orange'`                        | The style for the module.                               |
| `disabled`          | `true`                                 | Disables the `direnv` module.                           |
| `detect_extensions` | `[]`                                   | Which extensions should trigger this module.            |
| `detect_files`      | `['.envrc']`                           | Which filenames should trigger this module.             |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module.               |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | Which environment variables should trigger this module. |
| `allowed_msg`       | `'allowed'`                            | The message displayed when an rc file is allowed.       |
| `not_allowed_msg`   | `'not allowed'`                        | The message displayed when an rc file is not_allowed.   |
| `denied_msg`        | `'denied'`                             | The message displayed when an rc file is denied.        |
| `loaded_msg`        | `'loaded'`                             | The message displayed when an rc file is loaded.        |
| `unloaded_msg`      | `'not loaded'`                         | The message displayed when an rc file is not loaded.    |

### Variables

| Variable | Example             | Description                             |
| -------- | ------------------- | --------------------------------------- |
| loaded   | `loaded`            | Whether the current rc file is loaded.  |
| allowed  | `denied`            | Whether the current rc file is allowed. |
| rc_path  | `/home/test/.envrc` | The current rc file path.               |
| symbol   |                     | Mirrors the value of option `symbol`.   |
| style\*  | `red bold`          | Mirrors the value of option `style`.    |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```

## Docker Context

The `docker_context` module shows the currently active
[Docker context](https://docs.docker.com/engine/context/working-with-contexts/)
if it's not set to `default` or `desktop-linux`, or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or
`DOCKER_CONTEXT` environment variables are set (as they are meant to override
the context in use).

### Options

| Option              | Default                                                                                      | Description                                                                       |
| ------------------- | -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$context]($style) '`                                                           | The format for the module.                                                        |
| `symbol`            | `'🐳 '`                                                                                      | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                                                       | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                                                         | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `['compose.yml', 'compose.yaml', 'docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                                                         | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `'blue bold'`                                                                                | The style for the module.                                                         |
| `disabled`          | `false`                                                                                      | Disables the `docker_context` module.                                             |

### Variables

| Variable | Example        | Description                          |
| -------- | -------------- | ------------------------------------ |
| context  | `test_context` | The current docker context           |
| symbol   |                | Mirrors the value of option `symbol` |
| style\*  |                | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. If
the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module
shows the latest installed version of the SDK.

By default this module will only be shown in your prompt when one or more of
the following files are present in the current directory:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

You'll also need the .NET Core SDK installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast
as running `dotnet --version`, but it may show an incorrect version if your .NET project has an
unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by
setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker
(<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>)
when there is a `.csproj` file in the current directory.

### Options

| Option              | Default                                                                                                 | Description                                                               |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                          | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                                                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'.NET '`                                                                                               | The symbol used before displaying the version of dotnet.                  |
| `heuristic`         | `true`                                                                                                  | Use faster version detection to keep starship snappy.                     |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Which extensions should trigger this module.                              |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                    | Which folders should trigger this modules.                                |
| `style`             | `'bold blue'`                                                                                           | The style for the module.                                                 |
| `disabled`          | `false`                                                                                                 | Disables the `dotnet` module.                                             |

### Variables

| Variable | Example          | Description                                                        |
| -------- | ---------------- | ------------------------------------------------------------------ |
| version  | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm      | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol   |                  | Mirrors the value of option `symbol`                               |
| style\*  |                  | Mirrors the value of option `style`                                |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `mix.exs` file.

### Options

| Option              | Default                                                   | Description                                                               |
| ------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                                         |
| `version_format`    | `'v${raw}'`                                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💧 '`                                                   | The symbol used before displaying the version of Elixir/Erlang.           |
| `detect_extensions` | `[]`                                                      | Which extensions should trigger this module.                              |
| `detect_files`      | `['mix.exs']`                                             | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                      | Which folders should trigger this modules.                                |
| `style`             | `'bold purple'`                                           | The style for the module.                                                 |
| `disabled`          | `false`                                                   | Disables the `elixir` module.                                             |

### Variables

| Variable    | Example | Description                          |
| ----------- | ------- | ------------------------------------ |
| version     | `v1.10` | The version of `elixir`              |
| otp_version |         | The otp version of `elixir`          |
| symbol      |         | Mirrors the value of option `symbol` |
| style\*     |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains `*.elm` files

### Options

| Option              | Default                                            | Description                                                               |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`               | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                        | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🌳 '`                                            | A format string representing the symbol of Elm.                           |
| `detect_extensions` | `['elm']`                                          | Which extensions should trigger this module.                              |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `['elm-stuff']`                                    | Which folders should trigger this modules.                                |
| `style`             | `'cyan bold'`                                      | The style for the module.                                                 |
| `disabled`          | `false`                                            | Disables the `elm` module.                                                |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v0.19.1` | The version of `elm`                 |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variables.
The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

> [!TIP]
> The order in which env_var modules are shown can be individually set by including
> `${env_var.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`).
> By default, the `env_var` module will simply show all env_var modules in the order they were defined.

> [!TIP]
> Multiple environmental variables can be displayed by using a `.`. (see example)
> If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.
>
> Example: following configuration will display value of USER environment variable
>
> ```toml
> # ~/.config/starship.toml
>
> [env_var.USER]
> default = 'unknown user'
> ```

### Options

| Option        | Default                        | Description                                                                  |
| ------------- | ------------------------------ | ---------------------------------------------------------------------------- |
| `symbol`      | `""`                           | The symbol used before displaying the variable value.                        |
| `variable`    |                                | The environment variable to be displayed.                                    |
| `default`     |                                | The default value to be displayed when the selected variable is not defined. |
| `format`      | `"with [$env_value]($style) "` | The format for the module.                                                   |
| `description` | `"<env_var module>"`           | The description of the module that is shown when running `starship explain`. |
| `disabled`    | `false`                        | Disables the `env_var` module.                                               |
| `style`       | `"black bold dimmed"`          | The style for the module.                                                    |

### Variables

| Variable  | Example                                     | Description                                |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Mirrors the value of option `symbol`       |
| style\*   |                                             | Mirrors the value of option `style`        |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[env_var]
variable = 'SHELL'
default = 'unknown shell'
```

Displaying multiple environmental variables:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = 'SHELL'
default = 'unknown shell'
[env_var.USER]
default = 'unknown user'
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `rebar.config` file.
- The current directory contains a `erlang.mk` file.

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `' '`                               | The symbol used before displaying the version of erlang.                  |
| `style`             | `'bold red'`                         | The style for the module.                                                 |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `disabled`          | `false`                              | Disables the `erlang` module.                                             |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v22.1.3` | The version of `erlang`              |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Fennel

The `fennel` module shows the currently installed version of [Fennel](https://fennel-lang.org).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.fnl` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🧅 '`                              | The symbol used before displaying the version of fennel.                  |
| `style`             | `'bold green'`                       | The style for the module.                                                 |
| `detect_extensions` | `['fnl']`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `disabled`          | `false`                              | Disables the `fennel` module.                                             |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v1.2.1` | The version of `fennel`              |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```

## Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are
present in a line they will split the space evenly between them. This is useful for aligning
other modules.

### Options

| Option     | Default        | Description                       |
| ---------- | -------------- | --------------------------------- |
| `symbol`   | `'.'`          | The symbol used to fill the line. |
| `style`    | `'bold black'` | The style for the module.         |
| `disabled` | `false`        | Disables the `fill` module        |

### Example

```toml
# ~/.config/starship.toml
format = 'AA $fill BB $fill CC'

[fill]
symbol = '-'
style = 'bold green'
```

Produces a prompt that looks like:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Fortran

The `fortran` module shows the current compiler version of Fortran.

### Options

| Option              | Default                                                                                                                     | Description                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `symbol`            | `' '`                                                                                                                      | The symbol used before displaying the version of Fortran.                 |
| `format`            | `'via [$symbol($version )]($style)'`                                                                                        | The format for the module.                                                |
| `version_format`    | `'${raw}'`                                                                                                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold purple'`                                                                                                             | The style for the module.                                                 |
| `detect_extensions` | `['f', 'F', 'for', 'FOR', 'ftn', 'FTN', 'f77', 'F77', 'f90', 'F90', 'f95', 'F95','f03', 'F03', 'f08', 'F08', 'f18', 'F18']` | Which extensions should trigger this module.                              |
| `detect_files`      | `['fpm.toml']`                                                                                                              | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                                        | Which folders should trigger this module.                                 |
| `commands`          | `[ [ 'gfortran', '--version' ], [ 'flang', '--version' ], [ 'flang-new', '--version' ] ]`                                   | How to detect what the compiler is                                        |
| `disabled`          | `false`                                                                                                                     | Disables the `fortran` module.                                            |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| name     | gfortran | The name of the compiler             |
| version  | `14.2.0` | The version of the Fortran compiler  |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['myfortran', '--version']`. Starship will try executing each command until it gets a result on STDOUT.

If a Fortran compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/).

## Fossil Branch

The `fossil_branch` module shows the name of the active branch of the check-out in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                          | Description                                                                              |
| ------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | The format for the module. Use `'$branch'` to refer to the current branch name.          |
| `symbol`            | `' '`                           | The symbol used before the branch name of the check-out in your current directory.       |
| `style`             | `'bold purple'`                  | The style for the module.                                                                |
| `truncation_length` | `2^63 - 1`                       | Truncates a Fossil branch name to `N` graphemes                                          |
| `truncation_symbol` | `'…'`                            | The symbol used to indicate a branch name was truncated. You can use `''` for no symbol. |
| `disabled`          | `true`                           | Disables the `fossil_branch` module.                                                     |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| branch   | `trunk` | The active Fossil branch             |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```

## Fossil Metrics

The `fossil_metrics` module will show the number of added and deleted lines in the check-out in your current directory. At least v2.14 (2021-01-20) of Fossil is required.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option               | Default                                                      | Description                           |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | The format for the module.            |
| `added_style`        | `'bold green'`                                               | The style for the added count.        |
| `deleted_style`      | `'bold red'`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `disabled`           | `true`                                                       | Disables the `fossil_metrics` module. |

### Variables

| Variable        | Example | Description                                 |
| --------------- | ------- | ------------------------------------------- |
| added           | `1`     | The current number of added lines           |
| deleted         | `2`     | The current number of deleted lines         |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[fossil_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI.
This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

When the module is enabled it will always be active, unless `detect_env_vars` has
been set in which case the module will only be active when one of the
environment variables has been set.

### Options

| Option            | Default                                                  | Description                                                      |
| ----------------- | -------------------------------------------------------- | ---------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | The format for the module.                                       |
| `symbol`          | `'☁️  '`                                                  | The symbol used before displaying the current GCP profile.       |
| `region_aliases`  | `{}`                                                     | Table of region aliases to display in addition to the GCP name.  |
| `project_aliases` | `{}`                                                     | Table of project aliases to display in addition to the GCP name. |
| `detect_env_vars` | `[]`                                                     | Which environmental variables should trigger this module         |
| `style`           | `'bold blue'`                                            | The style for the module.                                        |
| `disabled`        | `false`                                                  | Disables the `gcloud` module.                                    |

### Variables

| Variable | Example       | Description                                                        |
| -------- | ------------- | ------------------------------------------------------------------ |
| region   | `us-central1` | The current GCP region                                             |
| account  | `foo`         | The current GCP profile                                            |
| domain   | `example.com` | The current GCP profile domain                                     |
| project  |               | The current GCP project                                            |
| active   | `default`     | The active config name written in `~/.config/gcloud/active_config` |
| symbol   |               | Mirrors the value of option `symbol`                               |
| style\*  |               | Mirrors the value of option `style`                                |

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
format = '[$symbol$active]($style) '
style = 'bold yellow'
```

#### Display account and aliased region

```toml
# ~/.config/starship.toml

[gcloud]
symbol = '️🇬️ '
[gcloud.region_aliases]
us-central1 = 'uc1'
asia-northeast1 = 'an1'
```

#### Display account and aliased project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = 'vlpn'
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Options

| Option               | Default                                           | Description                                                                              |
| -------------------- | ------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Shows the remote tracking branch name, even if it is equal to the local branch name.     |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | The format for the module. Use `'$branch'` to refer to the current branch name.          |
| `symbol`             | `' '`                                            | A format string representing the symbol of git branch.                                   |
| `style`              | `'bold purple'`                                   | The style for the module.                                                                |
| `truncation_length`  | `2^63 - 1`                                        | Truncates a git branch to `N` graphemes.                                                 |
| `truncation_symbol`  | `'…'`                                             | The symbol used to indicate a branch name was truncated. You can use `''` for no symbol. |
| `only_attached`      | `false`                                           | Only show the branch name when not in a detached `HEAD` state.                           |
| `ignore_branches`    | `[]`                                              | A list of names to avoid displaying. Useful for 'master' or 'main'.                      |
| `ignore_bare_repo`   | `false`                                           | Do not show when in a bare repo.                                                         |
| `disabled`           | `false`                                           | Disables the `git_branch` module.                                                        |

### Variables

| Variable      | Example  | Description                                                                                            |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | Mirrors the value of option `symbol`                                                                   |
| style\*       |          | Mirrors the value of option `style`                                                                    |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[git_branch]
symbol = '🌱 '
truncation_length = 4
truncation_symbol = ''
ignore_branches = ['master', 'main']
```

## Git Commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Options

| Option               | Default                      | Description                                                                          |
| -------------------- | ---------------------------- | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                          | The length of the displayed git commit hash.                                         |
| `format`             | `'[\($hash$tag\)]($style) '` | The format for the module.                                                           |
| `style`              | `'bold green'`               | The style for the module.                                                            |
| `only_detached`      | `true`                       | Only show git commit hash when in detached `HEAD` state                              |
| `tag_disabled`       | `true`                       | Disables showing tag info in `git_commit` module.                                    |
| `tag_max_candidates` | `0`                          | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `' 🏷  '`                     | Tag symbol prefixing the info shown                                                  |
| `disabled`           | `false`                      | Disables the `git_commit` module.                                                    |

### Variables

| Variable | Example   | Description                                  |
| -------- | --------- | -------------------------------------------- |
| hash     | `b703eb3` | The current git commit hash                  |
| tag      | `v1.0.0`  | The tag name if showing tag info is enabled. |
| style\*  |           | Mirrors the value of option `style`          |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## Git State

The `git_state` module will show in directories which are part of a git
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc. If there is progress information (e.g., REBASING 3/10),
that information will be shown too.

### Options

| Option         | Default                                                       | Description                                                                             |
| -------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                  | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `'MERGING'`                                                   | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `'REVERTING'`                                                 | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `'CHERRY-PICKING'`                                            | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `'BISECTING'`                                                 | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `'AM'`                                                        | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `'AM/REBASE'`                                                 | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `'bold yellow'`                                               | The style for the module.                                                               |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | The format for the module.                                                              |
| `disabled`     | `false`                                                       | Disables the `git_state` module.                                                        |

### Variables

| Variable         | Example    | Description                         |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*          |            | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in
the current git repository.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option               | Default                                                      | Description                           |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `'bold green'`                                               | The style for the added count.        |
| `deleted_style`      | `'bold red'`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | The format for the module.            |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.    |
| `ignore_submodules`  | `false`                                                      | Ignore changes to submodules          |

### Variables

| Variable        | Example | Description                                 |
| --------------- | ------- | ------------------------------------------- |
| added           | `1`     | The current number of added lines           |
| deleted         | `2`     | The current number of deleted lines         |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your
current directory.

> [!TIP]
> The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment.
> You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

### Options

| Option                 | Default                                       | Description                                                                                                 |
| ---------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`               | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status`                                                                         |
| `conflicted`           | `'='`                                         | This branch has merge conflicts.                                                                            |
| `ahead`                | `'⇡'`                                         | The format of `ahead`                                                                                       |
| `behind`               | `'⇣'`                                         | The format of `behind`                                                                                      |
| `diverged`             | `'⇕'`                                         | The format of `diverged`                                                                                    |
| `up_to_date`           | `''`                                          | The format of `up_to_date`                                                                                  |
| `untracked`            | `'?'`                                         | The format of `untracked`                                                                                   |
| `stashed`              | `'\$'`                                        | The format of `stashed`                                                                                     |
| `modified`             | `'!'`                                         | The format of `modified`                                                                                    |
| `staged`               | `'+'`                                         | The format of `staged`                                                                                      |
| `renamed`              | `'»'`                                         | The format of `renamed`                                                                                     |
| `deleted`              | `'✘'`                                         | The format of `deleted`                                                                                     |
| `typechanged`          | `""`                                          | The format of `typechanged`                                                                                 |
| `style`                | `'bold red'`                                  | The style for the module.                                                                                   |
| `ignore_submodules`    | `false`                                       | Ignore changes to submodules.                                                                               |
| `worktree_added`       | `""`                                          | The format of `worktree_added`                                                                              |
| `worktree_deleted`     | `""`                                          | The format of `worktree_deleted`                                                                            |
| `worktree_modified`    | `""`                                          | The format of `worktree_modified`                                                                           |
| `worktree_typechanged` | `""`                                          | The format of `worktree_typechanged`                                                                        |
| `index_added`          | `""`                                          | The format of `index_added`                                                                                 |
| `index_deleted`        | `""`                                          | The format of `index_deleted`                                                                               |
| `index_modified`       | `""`                                          | The format of `index_modified`                                                                              |
| `index_typechanged`    | `""`                                          | The format of `index_typechanged`                                                                           |
| `disabled`             | `false`                                       | Disables the `git_status` module.                                                                           |
| `windows_starship`     |                                               | Use this (Linux) path to a Windows Starship executable to render `git_status` when on Windows paths in WSL. |
| `use_git_executable`   | `false`                                       | Do not use `gitoxide` for computing the status, but use the `git` executable instead.                       |

### Variables

The following variables can be used in `format`:

| Variable               | Description                                                                                                   |
| ---------------------- | ------------------------------------------------------------------------------------------------------------- |
| `all_status`           | Shortcut for`$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked`                       |
| `ahead_behind`         | Displays `diverged`, `ahead`, `behind` or `up_to_date` format string based on the current status of the repo. |
| `conflicted`           | Displays `conflicted` when this branch has merge conflicts.                                                   |
| `untracked`            | Displays `untracked` when there are untracked files in the working directory.                                 |
| `stashed`              | Displays `stashed` when a stash exists for the local repository.                                              |
| `modified`             | Displays `modified` when there are file modifications in the working directory.                               |
| `staged`               | Displays `staged` when a new file has been added to the staging area.                                         |
| `renamed`              | Displays `renamed` when a renamed file has been added to the staging area.                                    |
| `deleted`              | Displays `deleted` when a file's deletion has been added to the staging area.                                 |
| `typechanged`          | Displays `typechanged` when a file's type has been changed in the staging area.                               |
| `worktree_added`       | Displays `worktree_added` when a new file has been added in the working directory.                            |
| `worktree_deleted`     | Displays `worktree_deleted` when a file's been deleted in the working directory.                              |
| `worktree_modified`    | Displays `worktree_modified` when a file's been modified in the working directory.                            |
| `worktree_typechanged` | Displays `worktree_typechanged` when a file's type has been changed in the working directory.                 |
| `index_added`          | Displays `index_added` when a new file has been added to the staging area.                                    |
| `index_deleted`        | Displays `index_deleted` when a file's been deleted to the staging area.                                      |
| `index_modified`       | Displays `index_modified` when a file's been modified to the staging area.                                    |
| `index_typechanged`    | Displays `index_typechanged` when a file's type has been changed to the staging area.                         |
| style\*                | Mirrors the value of option `style`                                                                           |

*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| Variable       | Description                                    |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed`, `deleted`, `typechanged`, `worktree_added`, `worktree_deleted`, `worktree_modified`, `worktree_typechanged`, `index_added`, `index_deleted`, `index_modified`, and `index_typechanged`:

| Variable | Description              |
| -------- | ------------------------ |
| `count`  | Show the number of files |

### Example

```toml
# ~/.config/starship.toml

[git_status]
conflicted = '🏳'
ahead = '🏎💨'
behind = '😰'
diverged = '😵'
up_to_date = '✓'
untracked = '🤷'
stashed = '📦'
modified = '📝'
staged = '[++\($count\)](green)'
renamed = '👅'
deleted = '🗑'
```

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = '⇡${count}'
diverged = '⇕⇡${ahead_count}⇣${behind_count}'
behind = '⇣${count}'
```

Use Windows Starship executable on Windows paths in WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/username/scoop/apps/starship/current/starship.exe'
```

## Gleam

The `gleam` module shows the currently installed version of [Gleam](https://gleam.run/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `gleam.toml` file
- The current directory contains a file with the `.gleam` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⭐ '`                              | A format string representing the symbol of Gleam.                         |
| `detect_extensions` | `['gleam']`                          | Which extensions should trigger this module.                              |
| `detect_files`      | `['gleam.toml']`                     | Which filenames should trigger this module.                               |
| `style`             | `'bold #FFAFF3'`                     | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `gleam` module.                                              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v1.0.0` | The version of `gleam`               |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[gleam]
format = 'via [⭐ $version](bold red) '
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `go.work` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Options

| Option              | Default                                                                                   | Description                                                                                                |
| ------------------- | ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | The format for the module.                                                                                 |
| `version_format`    | `'v${raw}'`                                                                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch`                                  |
| `symbol`            | `'🐹 '`                                                                                   | A format string representing the symbol of Go.                                                             |
| `detect_extensions` | `['go']`                                                                                  | Which extensions should trigger this module.                                                               |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Which filenames should trigger this module.                                                                |
| `detect_folders`    | `['Godeps']`                                                                              | Which folders should trigger this module.                                                                  |
| `style`             | `'bold cyan'`                                                                             | The style for the module.                                                                                  |
| `not_capable_style` | `'bold red'`                                                                              | The style for the module when the go directive in the go.mod file does not match the installed Go version. |
| `disabled`          | `false`                                                                                   | Disables the `golang` module.                                                                              |

### Variables

| Variable    | Example   | Description                                                                                                                                 |
| ----------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| version     | `v1.12.1` | The version of `go`                                                                                                                         |
| mod_version | `1.16`    | `go` version requirement as set in the go directive of `go.mod`. Will only show if the version requirement does not match the `go` version. |
| symbol      |           | Mirrors the value of option `symbol`                                                                                                        |
| style\*     |           | Mirrors the value of option `style`                                                                                                         |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

### Using `mod_version`

```toml
# ~/.config/starship.toml

[golang]
format = 'via [$symbol($version )($mod_version )]($style)'
```

## Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment.
The module will be shown when inside a guix-shell environment.

### Options

| Option     | Default                    | Description                                            |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `'via [$symbol]($style) '` | The format for the module.                             |
| `symbol`   | `'🐃 '`                    | A format string representing the symbol of guix-shell. |
| `style`    | `'yellow bold'`            | The style for the module.                              |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

The `gradle` module shows the version of the [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html)
currently used in the project directory.

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `gradle/wrapper/gradle-wrapper.properties` directory.
- The current directory contains a file ending with `.gradle` or `.gradle.kts`.

The `gradle` module is only able to read your Gradle Wrapper version from your config file, we don't execute your wrapper, because of the security concerns.

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🅶 '`                               | A format string representing the symbol of Gradle.                        |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `['gradle']`                         | Which folders should trigger this module.                                 |
| `style`             | `'bold bright-cyan'`                 | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `gradle` module.                                             |
| `recursive`         | `false`                              | Enables recursive finding for the `gradle` directory.                     |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v7.5.1` | The version of `gradle`              |
| symbol   |          | Mirrors the value of option `symbol` |
| style*   |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `stack.yaml` file
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### Options

| Option              | Default                              | Description                                        |
| ------------------- | ------------------------------------ | -------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                         |
| `symbol`            | `'λ '`                               | A format string representing the symbol of Haskell |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Which extensions should trigger this module.       |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | Which filenames should trigger this module.        |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.          |
| `style`             | `'bold purple'`                      | The style for the module.                          |
| `disabled`          | `false`                              | Disables the `haskell` module.                     |

### Variables

| Variable     | Example     | Description                                                                             |
| ------------ | ----------- | --------------------------------------------------------------------------------------- |
| version      |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot     | `lts-18.12` | Currently selected Stack snapshot                                                       |
| ghc\_version | `9.2.1`     | Currently installed GHC version                                                         |
| symbol       |             | Mirrors the value of option `symbol`                                                    |
| style\*      |             | Mirrors the value of option `style`                                                     |

*: This variable can only be used as a part of a style string

## Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- The current directory contains a `.haxelib` or a `haxe_libraries` directory
- The current directory contains a file with the `.hx` or `.hxml` extension

### Options

| Option              | Default                                                                                         | Description                                                               |
| ------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                            | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                                                                     | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | Which extensions should trigger this module.                              |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | Which folders should trigger this modules.                                |
| `symbol`            | `'⌘ '`                                                                                          | A format string representing the symbol of Haxe.                          |
| `style`             | `'bold fg:202'`                                                                                 | The style for the module.                                                 |
| `disabled`          | `false`                                                                                         | Disables the `haxe` module.                                               |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v4.2.5` | The version of `haxe`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `helmfile.yaml` file
- The current directory contains a `Chart.yaml` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `symbol`            | `'⎈ '`                               | A format string representing the symbol of Helm.                          |
| `style`             | `'bold white'`                       | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `helm` module.                                               |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v3.1.1` | The version of `helm`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## Hostname

The `hostname` module shows the system hostname.

### Options

| Option            | Default                                | Description                                                                                                                           |
| ----------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`        | `true`                                 | Only show hostname when connected to an SSH session.                                                                                  |
| `ssh_symbol`      | `'🌐 '`                                | A format string representing the symbol when connected to SSH session.                                                                |
| `trim_at`         | `'.'`                                  | String that the hostname is cut off at, after the first match. `'.'` will stop after the first dot. `''` will disable any truncation. |
| `detect_env_vars` | `[]`                                   | Which environment variable(s) should trigger this module.                                                                             |
| `format`          | `'[$ssh_symbol$hostname]($style) in '` | The format for the module.                                                                                                            |
| `style`           | `'bold dimmed green'`                  | The style for the module.                                                                                                             |
| `disabled`        | `false`                                | Disables the `hostname` module.                                                                                                       |
| `aliases`         | `{}`                                   | Translate system hostnames to something else. If `trim_at` is specified, only the first part will be matched and replaced.            |

### Variables

| Variable   | Example    | Description                                           |
| ---------- | ---------- | ----------------------------------------------------- |
| hostname   | `computer` | The hostname of the computer                          |
| style\*    |            | Mirrors the value of option `style`                   |
| ssh_symbol | `'🌏 '`    | The symbol to represent when connected to SSH session |

*: This variable can only be used as a part of a style string

### Examples

#### Always show the hostname

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

#### Hide the hostname in remote tmux sessions

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
detect_env_vars = ['!TMUX', 'SSH_CONNECTION']
disabled = false
```

#### Replace the hostname with a nickname

```toml
# ~/.config/starship.toml
[hostname]
aliases = { "Max's MacBook Pro" = "home" }
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot`, or `.sdkmanrc` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Options

| Option              | Default                                                                                                               | Description                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                                                                                           | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | Which extensions should trigger this module.                              |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                                  | Which folders should trigger this modules.                                |
| `symbol`            | `'☕ '`                                                                                                               | A format string representing the symbol of Java                           |
| `style`             | `'red dimmed'`                                                                                                        | The style for the module.                                                 |
| `disabled`          | `false`                                                                                                               | Disables the `java` module.                                               |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| version  | `v14`   | The version of `java`                |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## Jobs

The `jobs` module shows the current number of jobs running.
The module will be shown only if there are background jobs running.
The module will show the number of jobs running if there are at least
2 jobs, or more than the `number_threshold` config value, if it exists.
The module will show a symbol if there is at least 1 job, or more than the
`symbol_threshold` config value, if it exists. You can set both values
to 0 in order to _always_ show the symbol and number of jobs, even if there are
0 jobs running.

The default functionality is:

- 0 jobs -> Nothing is shown.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

> [!WARNING]
> This module is not supported on tcsh.

> [!WARNING]
> The `threshold` option is deprecated, but if you want to use it,
> the module will show the number of jobs running if there is more than 1 job, or
> more than the `threshold` config value, if it exists. If `threshold` is set to 0,
> then the module will also show when there are 0 jobs running.

### Options

| Option             | Default                       | Description                                                              |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | The format for the module.                                               |
| `symbol`           | `'✦'`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `'bold blue'`                 | The style for the module.                                                |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| number   | `1`     | The number of jobs                   |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Examples

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
symbol_threshold = 0
```

#### Changing process grouping behavior in fish

When using the Fish shell, Starship counts **job groups** instead of individual process IDs by default. This prevents overcounting when a pipeline has multiple processes but only one suspended group. To revert to the legacy PID-based counting, please add the following to your shell config:

```fish
set -g __starship_fish_use_job_groups "false"
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['jl']`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `symbol`            | `'ஃ '`                               | A format string representing the symbol of Julia.                         |
| `style`             | `'bold purple'`                      | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `julia` module.                                              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v1.4.0` | The version of `julia`               |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.kt` or a `.kts` file

### Options

| Option              | Default                              | Description                                                                   |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                    |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`     |
| `detect_extensions` | `['kt', 'kts']`                      | Which extensions should trigger this module.                                  |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                                   |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                    |
| `symbol`            | `'🅺 '`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `'bold blue'`                        | The style for the module.                                                     |
| `kotlin_binary`     | `'kotlin'`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v1.4.21` | The version of `kotlin`              |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[kotlin]
symbol = '🅺 '
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = 'kotlinc'
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file.
The namespace needs to be set in the kubeconfig file, this can be done via
`kubectl config set-context starship-context --namespace astronaut`.
Similarly, the user and cluster can be set with `kubectl config set-context starship-context --user starship-user`
and `kubectl config set-context starship-context --cluster starship-cluster`.
If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.
>
> When the module is enabled it will always be active, unless any of
> `detect_env_vars`, `detect_extensions`, `detect_files` or `detect_folders` have
> been set in which case the module will only be active in directories that match
> those conditions or one of the environmental variables has been set.

### Options

> [!WARNING]
> The `context_aliases` and `user_aliases` options are deprecated. Use `contexts` and the corresponding `context_alias`
> and `user_alias` options instead.

| Option              | Default                                            | Description                                                           |
| ------------------- | -------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`            | `'☸ '`                                             | A format string representing the symbol displayed before the Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | The format for the module.                                            |
| `style`             | `'cyan bold'`                                      | The style for the module.                                             |
| `context_aliases`*  | `{}`                                               | Table of context aliases to display.                                  |
| `user_aliases`*     | `{}`                                               | Table of user aliases to display.                                     |
| `detect_extensions` | `[]`                                               | Which extensions should trigger this module.                          |
| `detect_files`      | `[]`                                               | Which filenames should trigger this module.                           |
| `detect_folders`    | `[]`                                               | Which folders should trigger this modules.                            |
| `detect_env_vars`   | `[]`                                               | Which environmental variables should trigger this module              |
| `contexts`          | `[]`                                               | Customized styles and symbols for specific contexts.                  |
| `disabled`          | `true`                                             | Disables the `kubernetes` module.                                     |

*: This option is deprecated, please add `contexts` with the corresponding `context_alias` and `user_alias` options instead.

To customize the style of the module for specific environments, use the following configuration as
part of the `contexts` list:

| Variable          | Description                                                                              |
| ----------------- | ---------------------------------------------------------------------------------------- |
| `context_pattern` | **Required** Regular expression to match current Kubernetes context name.                |
| `user_pattern`    | Regular expression to match current Kubernetes user name.                                |
| `context_alias`   | Context alias to display instead of the full context name.                               |
| `user_alias`      | User alias to display instead of the full user name.                                     |
| `style`           | The style for the module when using this context. If not set, will use module's style.   |
| `symbol`          | The symbol for the module when using this context. If not set, will use module's symbol. |

Note that all regular expression are anchored with `^<pattern>$` and so must match the whole string. The `*_pattern`
regular expressions may contain capture groups, which can be referenced in the corresponding alias via `$name` and `$N`
(see example below and the
[rust Regex::replace() documentation](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)).

### Variables

| Variable  | Example              | Description                              |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbol    |                      | Mirrors the value of option `symbol`     |
| style\*   |                      | Mirrors the value of option `style`      |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
contexts = [
  { context_pattern = "dev.local.cluster.k8s", style = "green", symbol = "💔 " },
]
```

Only show the module in directories that contain a `k8s` file.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Kubernetes Context specific config

The `contexts` configuration option is used to customise what the current Kubernetes context name looks
like (style and symbol) if the name matches the defined regular expression.

```toml
# ~/.config/starship.toml

[[kubernetes.contexts]]
# "bold red" style + default symbol when Kubernetes current context name equals "production" *and* the current user
# equals "admin_user"
context_pattern = "production"
user_pattern = "admin_user"
style = "bold red"
context_alias = "prod"
user_alias = "admin"

[[kubernetes.contexts]]
# "green" style + a different symbol when Kubernetes current context name contains openshift
context_pattern = ".*openshift.*"
style = "green"
symbol = "💔 "
context_alias = "openshift"

[[kubernetes.contexts]]
# Using capture groups
# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
context_pattern = "gke_.*_(?P<cluster>[\\w-]+)"
context_alias = "gke-$cluster"
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Options

| Option     | Default | Description                                                        |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### Example

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Local IP

The `localip` module shows the IPv4 address of the primary network interface.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option     | Default                   | Description                                            |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `'[$localipv4]($style) '` | The format for the module.                             |
| `style`    | `'bold yellow'`           | The style for the module.                              |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### Variables

| Variable  | Example      | Description                         |
| --------- | ------------ | ----------------------------------- |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address   |
| style\*   |              | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Options

| Option              | Default                              | Description                                                                |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                 |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`  |
| `symbol`            | `'🌙 '`                              | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `['lua']`                            | Which extensions should trigger this module.                               |
| `detect_files`      | `['.lua-version']`                   | Which filenames should trigger this module.                                |
| `detect_folders`    | `['lua']`                            | Which folders should trigger this module.                                  |
| `style`             | `'bold blue'`                        | The style for the module.                                                  |
| `lua_binary`        | `'lua'`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v5.4.0` | The version of `lua`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## Maven

The `maven` module indicates the presence of a Maven project in the current directory. If the [Maven Wrapper](https://maven.apache.org/wrapper/) is enabled, the Maven version will be parsed from `.mvn/wrapper/maven-wrapper.properties` and shown.

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml` file.
- The current directory contains a `.mvn/wrapper/maven-wrapper.properties` file.

If you use an alternate POM syntax (for example `pom.hocon`), add its filename to `detect_files`.

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🅼 '`                               | A format string representing the symbol of Maven.                         |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['pom.xml']`                        | Which filenames should trigger this module.                               |
| `detect_folders`    | `['.mvn']`                           | Which folders should trigger this module.                                 |
| `style`             | `'bold bright-cyan'`                 | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `maven` module.                                              |
| `recursive`         | `false`                              | Enables recursive finding for the `.mvn` directory.                       |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v3.2.0` | The version of `maven`               |
| symbol   |          | Mirrors the value of option `symbol` |
| style*   |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option      | Default                                        | Description                                              |
| ----------- | ---------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                           | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | The format for the module.                               |
| `symbol`    | `'🐏'`                                         | The symbol used before displaying the memory usage.      |
| `style`     | `'bold dimmed white'`                          | The style for the module.                                |
| `disabled`  | `true`                                         | Disables the `memory_usage` module.                      |

### Variables

| Variable     | Example       | Description                                                        |
| ------------ | ------------- | ------------------------------------------------------------------ |
| ram          | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct      | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol       | `🐏`          | Mirrors the value of option `symbol`                               |
| style\*      |               | Mirrors the value of option `style`                                |

*: This variable can only be used as a part of a style string
*\*: The SWAP file information is only displayed if detected on the current system

### Example

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = ' '
style = 'bold dimmed green'
```

## Meson

The `meson` module shows the current Meson developer environment status.

By default the Meson project name is displayed, if `$MESON_DEVENV` is set.

### Options

| Option              | Default                            | Description                                                                               |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | The format for the module.                                                                |
| `symbol`            | `'⬢ '`                             | The symbol used before displaying the project name.                                       |
| `style`             | `'blue bold'`                      | The style for the module.                                                                 |
| `disabled`          | `false`                            | Disables the `meson` module.                                                              |

### Variables

| Variable | Example    | Description                          |
| -------- | ---------- | ------------------------------------ |
| project  | `starship` | The current Meson project name       |
| symbol   | `🐏`       | Mirrors the value of option `symbol` |
| style\*  |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = '--'
symbol = ' '
style = 'bold dimmed green'
```

## Mercurial Branch

The `hg_branch` module shows the active branch and topic of the repo in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                                   | Description                                                                                  |
| ------------------- | ----------------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `' '`                                    | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `'bold purple'`                           | The style for the module.                                                                    |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | The format for the module.                                                                   |
| `truncation_length` | `2^63 - 1`                                | Truncates the hg branch / topic name to `N` graphemes                                        |
| `truncation_symbol` | `'…'`                                     | The symbol used to indicate a branch name was truncated.                                     |
| `disabled`          | `true`                                    | Disables the `hg_branch` module.                                                             |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| branch   | `master`  | The active mercurial branch          |
| topic    | `feature` | The active mercurial topic           |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Mercurial State

The `hg_state` module will show in directories which are part of a mercurial
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option       | Default                   | Description                                                   |
| ------------ | ------------------------- | ------------------------------------------------------------- |
| `merge`      | `'MERGING'`               | A format string displayed when a `merge` is in progress.      |
| `rebase`     | `'REBASING'`              | A format string displayed when a `rebase` is in progress.     |
| `update`     | `'UPDATING'`              | A format string displayed when a `update` is in progress.     |
| `bisect`     | `'BISECTING'`             | A format string displayed when a `bisect` is in progress.     |
| `shelve`     | `'SHELVING'`              | A format string displayed when a `shelve` is in progress.     |
| `graft`      | `'GRAFTING'`              | A format string displayed when a `graft` is in progress.      |
| `transplant` | `'TRANSPLANTING'`         | A format string displayed when a `transplant` is in progress. |
| `histedit`   | `'HISTEDITING'`           | A format string displayed when a `histedit` is in progress.   |
| `style`      | `'bold yellow'`           | The style for the module.                                     |
| `format`     | `'\([$state]($style)\) '` | The format for the module.                                    |
| `disabled`   | `true`                    | Disables the `hg_state` module.                               |

### Variables

| Variable         | Example    | Description                         |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*          |            | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

## Mise

The `mise` module shows the current mise health as reported by running `mise doctor`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                                                              | Description                                      |
| ------------------- | -------------------------------------------------------------------- | ------------------------------------------------ |
| `symbol`            | `'mise '`                                                            | The symbol used before displaying _mise_ health. |
| `style`             | `'bold purple'`                                                      | The style for the module.                        |
| `format`            | `'on [$symbol$health]($style) '`                                     | The format for the module.                       |
| `detect_extensions` | `[]`                                                                 | Which extensions should trigger this module.     |
| `detect_files`      | `['mise.toml', 'mise.local.toml', '.mise.toml', '.mise.local.toml']` | Which filenames should trigger this module.      |
| `detect_folders`    | `['.mise']`                                                          | Which folders should trigger this module.        |
| `healthy_symbol`    | `healthy`                                                            | The message displayed when _mise_ is healthy.    |
| `unhealthy_symbol`  | `unhealthy`                                                          | The message displayed when _mise_ is unhealthy.  |
| `disabled`          | `true`                                                               | Disables the `mise` module.                      |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| health   | `healthy` | The health of _mise_                 |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[mise]
health = 'ready'
```

## Mojo

The `mojo` module shows the current version of [Mojo programming language](https://www.modular.com/mojo) installed

### Options

| Option              | Default                               | Description                                            |
| ------------------- | ------------------------------------- | ------------------------------------------------------ |
| `format`            | `'with [$symbol($version )]($style)'` | The format for the module.                             |
| `symbol`            | `'🔥 '`                               | The symbol used before displaying the version of Mojo. |
| `style`             | `'bold 208'`                          | The style for the module.                              |
| `disabled`          | `false`                               | Disables the `mojo` module.                            |
| `detect_extensions` | `['mojo', '🔥']`                      | Which extensions should trigger this module.           |
| `detect_files`      | `[]`                                  | Which filenames should trigger this module.            |
| `detect_folders`    | `[]`                                  | Which folders should trigger this module.              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `24.4.0` | The version of `mojo`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[mojo]
format = 'via [mojo ($version )($hash )]($style)'
```

## NATS

The `nats` module shows the name of the current [NATS](https://nats.io) context.

### Options

| Option     | Default                    | Description                                                  |
| ---------- | -------------------------- | ------------------------------------------------------------ |
| `symbol`   | `'✉️ '`                     | The symbol used before the NATS context (defaults to empty). |
| `style`    | `'bold purple'`            | The style for the module.                                    |
| `format`   | `'[$symbol$name]($style)'` | The format for the module.                                   |
| `disabled` | `false`                    | Disables the `nats` module.                                  |

### Variables

| Variable | Example     | Description                          |
| -------- | ----------- | ------------------------------------ |
| name     | `localhost` | The name of the NATS context         |
| symbol   |             | Mirrors the value of option `symbol` |
| style\*  |             | Mirrors the value of option `style`  |

### Example

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```

## Network Namespace

The `netns` module shows the current network namespace.
This uses `ip netns identify` to get the network namespace, so only network namespaces mounted at `/var/run/netns` will be detected.

### Options

| Option     | Default                         | Description                                                       |
| ---------- | ------------------------------- | ----------------------------------------------------------------- |
| `format`   | `'[$symbol \[$name\]]($style)'` | The format for the module.                                        |
| `symbol`   | `'🛜 '`                         | The symbol used before the network namespace (defaults to empty). |
| `style`    | `'blue bold dimmed'`            | The style for the module.                                         |
| `disabled` | `false`                         | Disables the `netns` module.                                      |

### Variables

| Variable | Example    | Description                               |
| -------- | ---------- | ----------------------------------------- |
| name     | `my-netns` | The name of the current network namespace |
| symbol   |            | Mirrors the value of option `symbol`      |
| style\*  |            | Mirrors the value of option `style`       |

### Example

```toml
# ~/.config/starship.toml

[netns]
style = 'bold yellow'
symbol = '🌐 '
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module                                                 |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'👑 '`                              | The symbol used before displaying the version of Nim.                     |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | Which extensions should trigger this module.                              |
| `detect_files`      | `['nim.cfg']`                        | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'bold yellow'`                      | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `nim` module.                                                |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v1.2.0` | The version of `nimc`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment.
The module will be shown when inside a nix-shell environment.

### Options

| Option        | Default                                      | Description                                                           |
| ------------- | -------------------------------------------- | --------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | The format for the module.                                            |
| `symbol`      | `'❄️ '`                                       | A format string representing the symbol of nix-shell.                 |
| `style`       | `'bold blue'`                                | The style for the module.                                             |
| `impure_msg`  | `'impure'`                                   | A format string shown when the shell is impure.                       |
| `pure_msg`    | `'pure'`                                     | A format string shown when the shell is pure.                         |
| `unknown_msg` | `''`                                         | A format string shown when it is unknown if the shell is pure/impure. |
| `disabled`    | `false`                                      | Disables the `nix_shell` module.                                      |
| `heuristic`   | `false`                                      | Attempts to detect new `nix shell`-style shells with a heuristic.     |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| state    | `pure`  | The state of the nix-shell           |
| name     | `lorri` | The name of the nix-shell            |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = '[impure shell](bold red)'
pure_msg = '[pure shell](bold green)'
unknown_msg = '[unknown shell](bold yellow)'
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts`, `.mts` or `.cts` extension

Additionally, the module will be hidden by default if the directory contains a `bunfig.toml`, `bun.lock`, or `bun.lockb` file, overriding the above conditions.

### Options

| Option              | Default                                       | Description                                                                                           |
| ------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | The format for the module.                                                                            |
| `version_format`    | `'v${raw}'`                                   | The version format. Available vars are `raw`, `major`, `minor`, & `patch`                             |
| `symbol`            | `' '`                                        | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | Which extensions should trigger this module.                                                          |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | Which filenames should trigger this module.                                                           |
| `detect_folders`    | `['node_modules']`                            | Which folders should trigger this module.                                                             |
| `style`             | `'bold green'`                                | The style for the module.                                                                             |
| `disabled`          | `false`                                       | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `'bold red'`                                  | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variables

| Variable        | Example    | Description                                                                                                                                               |
| --------------- | ---------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version         | `v13.12.0` | The version of `node`                                                                                                                                     |
| engines_version | `>=12.0.0` | `node` version requirement as set in the engines property of `package.json`. Will only show if the version requirement does not match the `node` version. |
| symbol          |            | Mirrors the value of option `symbol`                                                                                                                      |
| style\*         |            | Mirrors the value of option `style`                                                                                                                       |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Options

| Option                    | Default                                                                  | Description                                                               |
| ------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | The format string for the module.                                         |
| `version_format`          | `'v${raw}'`                                                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `'🐫 '`                                                                  | The symbol used before displaying the version of OCaml.                   |
| `global_switch_indicator` | `''`                                                                     | The format string used to represent global OPAM switch.                   |
| `local_switch_indicator`  | `'*'`                                                                    | The format string used to represent local OPAM switch.                    |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                     | Which extensions should trigger this module.                              |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`         | Which filenames should trigger this module.                               |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                  | Which folders should trigger this module.                                 |
| `style`                   | `'bold yellow'`                                                          | The style for the module.                                                 |
| `disabled`                | `false`                                                                  | Disables the `ocaml` module.                                              |

### Variables

| Variable         | Example      | Description                                                       |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Mirrors the value of option `symbol`                              |
| style\*          |              | Mirrors the value of option `style`                               |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Odin

The `odin` module shows the currently installed version of [Odin](https://odin-lang.org/). By default the module will be shown if the current directory contains a `.odin` file.

### Options

| Option              | Default                              | Description                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                             |
| `show_commit`       | `false`                              | Shows the commit as part of the version.               |
| `symbol`            | `'Ø '`                               | The symbol used before displaying the version of Odin. |
| `style`             | `'bold bright-blue'`                 | The style for the module.                              |
| `disabled`          | `false`                              | Disables the `odin` module.                            |
| `detect_extensions` | `['odin']`                           | Which extensions should trigger this module.           |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.            |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.              |

### Variables

| Variable | Example       | Description                          |
| -------- | ------------- | ------------------------------------ |
| version  | `dev-2024-03` | The version of `odin`                |
| symbol   |               | Mirrors the value of option `symbol` |
| style\*  |               | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[odin]
format = 'via [󰹩 ($version )]($style)'
show_commit = true
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool.
By default the module will be shown if the current directory contains a `.rego` file.

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🪖  '`                             | A format string representing the symbol of OPA.                           |
| `detect_extensions` | `['rego']`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'bold blue'`                        | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `opa` module.                                                |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v0.44.0` | The version of `opa`                 |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module
only active when the `OS_CLOUD` env var is set, in which case it will read
`clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files).
to fetch the current project in use.

### Options

| Option     | Default                                       | Description                                                    |
| ---------- | --------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | The format for the module.                                     |
| `symbol`   | `'☁️ '`                                        | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `'bold yellow'`                               | The style for the module.                                      |
| `disabled` | `false`                                       | Disables the `openstack` module.                               |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| cloud    | `corp`  | The current OpenStack cloud          |
| project  | `dev`   | The current OpenStack project        |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```

## OS

The `os` module shows the current operating system.
OS information is detected via the [os_info](https://lib.rs/crates/os_info) crate.

> [!WARNING]
> The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option     | Default               | Description                                            |
| ---------- | --------------------- | ------------------------------------------------------ |
| `format`   | `'[$symbol]($style)'` | The format for the module.                             |
| `style`    | `'bold white'`        | The style for the module.                              |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type.
Operating system types not defined by your configuration use the default symbols table below.
All operating systems currently supported by the module are listed below.
If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# This is the default symbols table.
[os.symbols]
AIX = "➿ "
Alpaquita = "🔔 "
AlmaLinux = "💠 "
Alpine = "🏔️ "
ALTLinux = "Ⓐ "
Amazon = "🙂 "
Android = "🤖 "
AOSC = "🐱 "
Arch = "🎗️ "
Artix = "🎗️ "
Bluefin = "🐟 "
CachyOS = "🎗️ "
CentOS = "💠 "
Debian = "🌀 "
DragonFly = "🐉 "
Elementary = "🍏 "
Emscripten = "🔗 "
EndeavourOS = "🚀 "
Fedora = "🎩 "
FreeBSD = "😈 "
Garuda = "🦅 "
Gentoo = "🗜️ "
HardenedBSD = "🛡️ "
Illumos = "🐦 "
Ios = "📱 "
InstantOS = "⏲️ "
Kali = "🐉 "
Linux = "🐧 "
Mabox = "📦 "
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
Nobara = "🎩 "
OpenBSD = "🐡 "
OpenCloudOS = "☁️ "
openEuler = "🦉 "
openSUSE = "🦎 "
OracleLinux = "🦴 "
PikaOS = "🐤 "
Pop = "🍭 "
Raspbian = "🍓 "
Redhat = "🎩 "
RedHatEnterprise = "🎩 "
RockyLinux = "💠 "
Redox = "🧪 "
Solus = "⛵ "
SUSE = "🦎 "
Ubuntu = "🎯 "
Ultramarine = "🔷 "
Unknown = "❓ "
Uos = "🐲 "
Void = " "
Windows = "🪟 "
Zorin = "🔹 "
```

### Variables

| Variable | Example      | Description                                                        |
| -------- | ------------ | ------------------------------------------------------------------ |
| symbol   | `🎗️`          | The current operating system symbol from advanced option `symbols` |
| name     | `Arch Linux` | The current operating system name                                  |
| type     | `Arch`       | The current operating system type                                  |
| codename |              | The current operating system codename, if applicable               |
| edition  |              | The current operating system edition, if applicable                |
| version  |              | The current operating system version, if applicable                |
| style\*  |              | Mirrors the value of option `style`                                |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[os]
format = "on [($name )]($style)"
style = "bold blue"
disabled = false

[os.symbols]
Windows = " "
Arch = "Arch is the best! "
```

## Package Version

The `package` module is shown when the current directory is the repository for a
package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`,
`poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `galaxy`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present
  in the current directory
- [**JSR**](https://jsr.io/) – The `jsr` package version is extracted from the `jsr.json`/`jsr.jsonc` or `deno.json`/`deno.jsonc` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present
  in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from a [PEP 621](https://peps.python.org/pep-0621/) compliant `pyproject.toml` or a `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present
  in the current directory
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present in the current directory
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present in the current directory
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present in the current directory
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present in the current directory
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present in the current directory
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present in the current directory
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present in the current directory
- [**Galaxy**](https://galaxy.ansible.com/) - The `galaxy` package version is extracted from the `galaxy.yml` present in the current directory
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present in the current directory
- [**SBT**](https://scala-sbt.org) - The `sbt` package version is extracted from the `build.sbt` present in the current directory
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) - The `dart` package version is extracted from the `pubspec.yaml` present in the current directory

> ⚠️ The version being shown is that of the package whose source code is in your
> current directory, not your package manager.

### Options

| Option            | Default                           | Description                                                               |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------- |
| `format`          | `'is [$symbol$version]($style) '` | The format for the module.                                                |
| `symbol`          | `'📦 '`                           | The symbol used before displaying the version the package.                |
| `version_format`  | `'v${raw}'`                       | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`           | `'bold 208'`                      | The style for the module.                                                 |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.                 |
| `disabled`        | `false`                           | Disables the `package` module.                                            |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v1.0.0` | The version of your package          |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[package]
format = 'via [🎁 $version](208 bold) '
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Options

| Option              | Default                                                                                                  | Description                                                               |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | The format string for the module.                                         |
| `version_format`    | `'v${raw}'`                                                                                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐪 '`                                                                                                  | The symbol used before displaying the version of Perl                     |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | Which extensions should trigger this module.                              |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                     | Which folders should trigger this module.                                 |
| `style`             | `'bold 149'`                                                                                             | The style for the module.                                                 |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                               |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v5.26.1` | The version of `perl`                |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐘 '`                              | The symbol used before displaying the version of PHP.                     |
| `detect_extensions` | `['php']`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `['composer.json', '.php-version']`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'147 bold'`                         | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `php` module.                                                |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v7.3.8` | The version of `php`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Pijul Channel

The `pijul_channel` module shows the active channel of the repo in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                           | Description                                                                          |
| ------------------- | --------------------------------- | ------------------------------------------------------------------------------------ |
| `symbol`            | `' '`                            | The symbol used before the pijul channel name of the repo in your current directory. |
| `style`             | `'bold purple'`                   | The style for the module.                                                            |
| `format`            | `'on [$symbol$channel]($style) '` | The format for the module.                                                           |
| `truncation_length` | `2^63 - 1`                        | Truncates the pijul channel name to `N` graphemes                                    |
| `truncation_symbol` | `'…'`                             | The symbol used to indicate a branch name was truncated.                             |
| `disabled`          | `true`                            | Disables the `pijul` module.                                                         |

## Pixi

The `pixi` module shows the installed [pixi](https://pixi.sh) version as well as the activated environment, if `$PIXI_ENVIRONMENT_NAME` is set.

> [!TIP]
> This does not suppress pixi's own prompt modifier, you may want to run `pixi config set shell.change-ps1 false`.

### Options

| Option                     | Default                                                 | Description                                                                       |
| -------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`                   | `'via [$symbol($version )(\($environment\) )]($style)'` | The format for the module.                                                        |
| `version_format`           | `'v${raw}'`                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch`.        |
| `symbol`                   | `'🧚 '`                                                 | The symbol used before the environment name.                                      |
| `style`                    | `'yellow bold'`                                         | The style for the module.                                                         |
| `show_default_environment` | `true`                                                  | Whether to indicate that the `default` environment of your project is activated.  |
| `pixi_binary`              | `['pixi']`                                              | Configures the pixi binary that Starship should execute when getting the version. |
| `detect_extensions`        | `[]`                                                    | Which extensions should trigger this module.                                      |
| `detect_files`             | `['pixi.toml']`                                         | Which filenames should trigger this module.                                       |
| `detect_folders`           | `[]`                                                    | Which folders should trigger this module.                                         |
| `disabled`                 | `false`                                                 | Disables the `pixi` module.                                                       |

### Variables

| Variable    | Example   | Description                          |
| ----------- | --------- | ------------------------------------ |
| version     | `v0.33.0` | The version of `pixi`                |
| environment | `py311`   | The current pixi environment         |
| symbol      |           | Mirrors the value of option `symbol` |
| style       |           | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[pixi]
format = '[$symbol$environment](yellow) '
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

> [!TIP]
> By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms).
> If you still want to enable it, [follow the example shown below](#with-pulumi-version).

By default the module will be shown if any of the following conditions are met:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Options

| Option           | Default                                      | Description                                                               |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | The format string for the module.                                         |
| `version_format` | `'v${raw}'`                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `' '`                                       | A format string shown before the Pulumi stack.                            |
| `style`          | `'bold 5'`                                   | The style for the module.                                                 |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.            |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                             |

### Variables

| Variable | Example    | Description                          |
| -------- | ---------- | ------------------------------------ |
| version  | `v0.12.24` | The version of `pulumi`              |
| stack    | `dev`      | The current Pulumi stack             |
| username | `alice`    | The current Pulumi username          |
| symbol   |            | Mirrors the value of option `symbol` |
| style\*  |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

#### With Pulumi Version

```toml
# ~/.config/starship.toml

[pulumi]
format = '[🛥 ($version )$stack]($style) '
```

#### Without Pulumi version

```toml
# ~/.config/starship.toml
[pulumi]
symbol = '🛥 '
format = '[$symbol$stack]($style) '
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version.
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `spago.dhall` file
- The current directory contains a `spago.yaml` file
- The current directory contains a `spago.lock` file
- The current directory contains a file with the `.purs` extension

### Options

| Option              | Default                                       | Description                                                               |
| ------------------- | --------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                   | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'<=> '`                                      | The symbol used before displaying the version of PureScript.              |
| `detect_extensions` | `['purs']`                                    | Which extensions should trigger this module.                              |
| `detect_files`      | `['spago.dhall', 'spago.yaml', 'spago.lock']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                          | Which folders should trigger this module.                                 |
| `style`             | `'bold white'`                                | The style for the module.                                                 |
| `disabled`          | `false`                                       | Disables the `purescript` module.                                         |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `0.13.5` | The version of `purescript`          |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the
current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version
name. Otherwise, it will display the version number from `python --version`.

By default, the module will be shown if any of the following conditions are met:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
- The current directory contains a file with the `.py` extension.
- The current directory contains a file with the `.ipynb` extension.
- A virtual environment is currently activated

### Options

| Option               | Default                                                                                                      | Description                                                                           |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                    | The format for the module.                                                            |
| `version_format`     | `'v${raw}'`                                                                                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch`             |
| `symbol`             | `'🐍 '`                                                                                                      | A format string representing the symbol of Python                                     |
| `style`              | `'yellow bold'`                                                                                              | The style for the module.                                                             |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                       |
| `pyenv_prefix`       | `'pyenv'`                                                                                                    | Prefix before pyenv version display, only used if pyenv is used                       |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configures the python binaries that Starship should execute when getting the version. |
| `detect_extensions`  | `['py', 'ipynb']`                                                                                            | Which extensions should trigger this module                                           |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | Which filenames should trigger this module                                            |
| `detect_folders`     | `[]`                                                                                                         | Which folders should trigger this module                                              |
| `generic_venv_names` | `[]`                                                                                                         | Which venv names should be replaced with the parent directory name.                   |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                         |

> [!TIP]
> The `python_binary` variable accepts either a string or a list of strings.
> Starship will try executing each binary until it gets a result. Note you can
> only change the binary that Starship executes to get the version of Python not
> the arguments that are used.
>
> The default values and order for `python_binary` was chosen to first identify
> the Python version in a virtualenv/conda environments (which currently still
> add a `python`, no matter if it points to `python3` or `python2`). This has the
> side effect that if you still have a system Python 2 installed, it may be
> picked up before any Python 3 (at least on Linux Distros that always symlink
> `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but
> cannot remove the system Python 2, changing this to `'python3'` will hide any
> Python version 2, see example below.

### Variables

| Variable     | Example         | Description                                                                 |
| ------------ | --------------- | --------------------------------------------------------------------------- |
| version      | `'v3.8.1'`      | The version of `python`                                                     |
| symbol       | `'🐍 '`         | Mirrors the value of option `symbol`                                        |
| style        | `'yellow bold'` | Mirrors the value of option `style`                                         |
| pyenv_prefix | `'pyenv '`      | Mirrors the value of option `pyenv_prefix`                                  |
| virtualenv   | `'venv'`        | The current `virtualenv` name or the parent if matches `generic_venv_names` |

### Example

```toml
# ~/.config/starship.toml

[python]
symbol = '👾 '
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = 'python3'
```

```toml
# ~/.config/starship.toml

[python]
# Don't trigger for files with the py extension
detect_extensions = []
```

## Quarto

The `quarto` module shows the current installed version of Quarto used in a project.

By default, the module will be shown if any of the following conditions are met:

- The current directory contains a `_quarto.yml` file
- The current directory contains any `*.qmd` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⨁ '`                               | A format string representing the symbol of Quarto                         |
| `style`             | `'bold #75AADB'`                     | The style for the module.                                                 |
| `detect_extensions` | `['.qmd']`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `['_quarto.yml']`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `quarto` module.                                             |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `1.4.549` | The version of `quarto`              |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

## R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). The module will be shown if
any of the following conditions are met:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'📐'`                               | A format string representing the symbol of R.                             |
| `style`             | `'blue bold'`                        | The style for the module.                                                 |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Which extensions should trigger this module                               |
| `detect_files`      | `['.Rprofile']`                      | Which filenames should trigger this module                                |
| `detect_folders`    | `['.Rproj.user']`                    | Which folders should trigger this module                                  |
| `disabled`          | `false`                              | Disables the `r` module.                                                  |

### Variables

| Variable | Example       | Description                          |
| -------- | ------------- | ------------------------------------ |
| version  | `v4.0.5`      | The version of `R`                   |
| symbol   |               | Mirrors the value of option `symbol` |
| style    | `'blue bold'` | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Options

| Option              | Default                                          | Description                                                               |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | The format string for the module.                                         |
| `version_format`    | `'v${raw}'`                                      | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦋 '`                                          | The symbol used before displaying the version of Raku                     |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Which extensions should trigger this module.                              |
| `detect_files`      | `['META6.json']`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                             | Which folders should trigger this module.                                 |
| `style`             | `'bold 149'`                                     | The style for the module.                                                 |
| `disabled`          | `false`                                          | Disables the `raku` module.                                               |

### Variables

| Variable   | Example | Description                          |
| ---------- | ------- | ------------------------------------ |
| version    | `v6.d`  | The version of `raku`                |
| vm_version | `moar`  | The version of VM `raku` is built on |
| symbol     |         | Mirrors the value of option `symbol` |
| style\*    |         | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/).
The module will be shown if any of the following conditions are met:

- The current directory contains a file with `.red` or `.reds` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🔺 '`                              | A format string representing the symbol of Red.                           |
| `detect_extensions` | `['red']`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'red bold'`                         | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `red` module.                                                |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v2.5.1` | The version of `red`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/).
The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💎 '`                              | A format string representing the symbol of Ruby.                          |
| `detect_extensions` | `['rb']`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Which environment variables should trigger this module.                   |
| `style`             | `'bold red'`                         | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `ruby` module.                                               |

### Variables

| Variable | Example  | Description                                 |
| -------- | -------- | ------------------------------------------- |
| version  | `v2.5.1` | The version of `ruby`                       |
| symbol   |          | Mirrors the value of option `symbol`        |
| style\*  |          | Mirrors the value of option `style`         |
| gemset   | `test`   | Optional, gets the current RVM gemset name. |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/).
The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦀 '`                              | A format string representing the symbol of Rust                           |
| `detect_extensions` | `['rs']`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `['Cargo.toml']`                     | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'bold red'`                         | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `rust` module.                                               |

### Variables

| Variable  | Example           | Description                                  |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbol    |                   | Mirrors the value of option `symbol`         |
| style\*   |                   | Mirrors the value of option `style`          |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Options

| Option              | Default                                  | Description                                                               |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['sbt', 'scala']`                       | Which extensions should trigger this module.                              |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `['.metals']`                            | Which folders should trigger this modules.                                |
| `symbol`            | `'🆂 '`                                   | A format string representing the symbol of Scala.                         |
| `style`             | `'red dimmed'`                           | The style for the module.                                                 |
| `disabled`          | `false`                                  | Disables the `scala` module.                                              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `2.13.5` | The version of `scala`               |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```

## Shell

The `shell` module shows an indicator for currently used shell.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option                 | Default                   | Description                                                                                            |
| ---------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------ |
| `bash_indicator`       | `'bsh'`                   | A format string used to represent bash.                                                                |
| `fish_indicator`       | `'fsh'`                   | A format string used to represent fish.                                                                |
| `zsh_indicator`        | `'zsh'`                   | A format string used to represent zsh.                                                                 |
| `powershell_indicator` | `'psh'`                   | A format string used to represent powershell.                                                          |
| `pwsh_indicator`       |                           | A format string used to represent pwsh. The default value mirrors the value of `powershell_indicator`. |
| `ion_indicator`        | `'ion'`                   | A format string used to represent ion.                                                                 |
| `elvish_indicator`     | `'esh'`                   | A format string used to represent elvish.                                                              |
| `tcsh_indicator`       | `'tsh'`                   | A format string used to represent tcsh.                                                                |
| `xonsh_indicator`      | `'xsh'`                   | A format string used to represent xonsh.                                                               |
| `cmd_indicator`        | `'cmd'`                   | A format string used to represent cmd.                                                                 |
| `nu_indicator`         | `'nu'`                    | A format string used to represent nu.                                                                  |
| `unknown_indicator`    | `''`                      | The default value to be displayed when the shell is unknown.                                           |
| `format`               | `'[$indicator]($style) '` | The format for the module.                                                                             |
| `style`                | `'white bold'`            | The style for the module.                                                                              |
| `disabled`             | `true`                    | Disables the `shell` module.                                                                           |

### Variables

| Variable  | Default | Description                                                |
| --------- | ------- | ---------------------------------------------------------- |
| indicator |         | Mirrors the value of `indicator` for currently used shell. |
| style\*   |         | Mirrors the value of option `style`.                       |

*: This variable can only be used as a part of a style string

### Examples

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = '󰈺 '
powershell_indicator = '_'
unknown_indicator = 'mystery shell'
style = 'cyan bold'
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is
set to a number and meets or exceeds the specified threshold.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option          | Default                      | Description                                                         |
| --------------- | ---------------------------- | ------------------------------------------------------------------- |
| `threshold`     | `2`                          | Display threshold.                                                  |
| `format`        | `'[$symbol$shlvl]($style) '` | The format for the module.                                          |
| `symbol`        | `'↕️  '`                      | The symbol used to represent the `SHLVL`.                           |
| `repeat`        | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount.       |
| `repeat_offset` | `0`                          | Decrements number of times `symbol` is repeated by the offset value |
| `style`         | `'bold yellow'`              | The style for the module.                                           |
| `disabled`      | `true`                       | Disables the `shlvl` module.                                        |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| shlvl    | `3`     | The current value of `SHLVL`         |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

Using `repeat` and `repeat_offset` along with `character` module, one can get
prompt like `❯❯❯` where last character is colored appropriately for return
status code and preceding characters are provided by `shlvl`.

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '[$symbol]($style)'
repeat = true
symbol = '❯'
repeat_offset = 1
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container
and `$SINGULARITY_NAME` is set.

### Options

| Option     | Default                        | Description                                      |
| ---------- | ------------------------------ | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | The format for the module.                       |
| `symbol`   | `''`                           | A format string displayed before the image name. |
| `style`    | `'bold dimmed blue'`           | The style for the module.                        |
| `disabled` | `false`                        | Disables the `singularity` module.               |

### Variables

| Variable | Example      | Description                          |
| -------- | ------------ | ------------------------------------ |
| env      | `centos.img` | The current Singularity image        |
| symbol   |              | Mirrors the value of option `symbol` |
| style\*  |              | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Solidity

The `solidity` module shows the currently installed version of [Solidity](https://soliditylang.org/)
The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.sol` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${major}.${minor}.${patch}'`      | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'S '`                               | A format string representing the symbol of Solidity                       |
| `compiler           | ['solc']                             | The default compiler for Solidity.                                        |
| `detect_extensions` | `['sol']`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'bold blue'`                        | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables this module.                                                     |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v0.8.1` | The version of `solidity`            |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml
[solidity]
format = "via [S $version](blue bold)"
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### Options

| Option              | Default                                | Description                                                                                                                                    |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `'🅢  '`                                | The symbol used before the environment name.                                                                                                   |
| `style`             | `'bold blue'`                          | The style for the module.                                                                                                                      |
| `format`            | `'via [$symbol$environment]($style) '` | The format for the module.                                                                                                                     |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                                   |

### Variables

| Variable    | Example      | Description                          |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current spack environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*     |              | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Status

The `status` module displays the exit code of the previous command.
If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`.
The status code will cast to a signed 32-bit integer.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option                      | Default                                                                        | Description                                                           |
| --------------------------- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                  | The format of the module                                              |
| `symbol`                    | `'❌'`                                                                         | The symbol displayed on program error                                 |
| `success_symbol`            | `''`                                                                           | The symbol displayed on program success                               |
| `not_executable_symbol`     | `'🚫'`                                                                         | The symbol displayed when file isn't executable                       |
| `not_found_symbol`          | `'🔍'`                                                                         | The symbol displayed when the command can't be found                  |
| `sigint_symbol`             | `'🧱'`                                                                         | The symbol displayed on SIGINT (Ctrl + c)                             |
| `signal_symbol`             | `'⚡'`                                                                         | The symbol displayed on any signal                                    |
| `style`                     | `'bold red'`                                                                   | The style for the module.                                             |
| `success_style`             |                                                                                | The style used on program success (defaults to `style` if unset).     |
| `failure_style`             |                                                                                | The style used on program failure (defaults to `style` if unset).     |
| `recognize_signal_code`     | `true`                                                                         | Enable signal mapping from exit code                                  |
| `map_symbol`                | `false`                                                                        | Enable symbols mapping from exit code                                 |
| `pipestatus`                | `false`                                                                        | Enable pipestatus reporting                                           |
| `pipestatus_separator`      | <code>&vert;</code>                                                            | The symbol used to separate pipestatus segments (supports formatting) |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style) '` | The format of the module when the command is a pipeline               |
| `pipestatus_segment_format` |                                                                                | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                         | Disables the `status` module.                                         |

### Variables

| Variable       | Example | Description                                                                                  |
| -------------- | ------- | -------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                            |
| hex_status     | `0x7F`  | The exit code of the last command in hex                                                     |
| int            | `127`   | The exit code of the last command                                                            |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                          |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                              |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                         |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                 |
| pipestatus     |         | Rendering of in pipeline programs' exit codes, this is only available in pipestatus_format   |
| symbol         |         | Mirrors the value of option `symbol`                                                         |
| style\*        |         | Mirrors the value of option `success_style` on program success and `failure_style` otherwise |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[status]
style = 'bg:blue'
symbol = '🔴 '
success_symbol = '🟢 SUCCESS'
format = '[\[$symbol$common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

The `sudo` module displays if sudo credentials are currently cached.
The module will only be shown if credentials are cached.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option          | Default                  | Description                                             |
| --------------- | ------------------------ | ------------------------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | The format of the module                                |
| `symbol`        | `'🧙 '`                  | The symbol displayed when credentials are cached        |
| `style`         | `'bold blue'`            | The style for the module.                               |
| `allow_windows` | `false`                  | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                   | Disables the `sudo` module.                             |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[sudo]
style = 'bold green'
symbol = '👩‍💻 '
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

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/).
The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐦 '`                              | A format string representing the symbol of Swift                          |
| `detect_extensions` | `['swift']`                          | Which extensions should trigger this module.                              |
| `detect_files`      | `['Package.swift']`                  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'bold 202'`                         | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `swift` module.                                              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v5.2.4` | The version of `swift`               |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[swift]
format = 'via [🏎  $version](red bold)'
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.
It supports both Hashicorp Terraform and OpenTofu for version detection.

> [!TIP]
> By default the Terraform/OpenTofu version is not shown, since this is slow for current versions when a lot of plugins are in use.
> If you still want to enable it, [follow the example shown below](#with-terraform-version).

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Options

| Option              | Default                                                 | Description                                                               |
| ------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$workspace]($style) '`                    | The format string for the module.                                         |
| `version_format`    | `'v${raw}'`                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💠'`                                                  | A format string shown before the terraform workspace.                     |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `['.terraform']`                                        | Which folders should trigger this module.                                 |
| `style`             | `'bold 105'`                                            | The style for the module.                                                 |
| `disabled`          | `false`                                                 | Disables the `terraform` module.                                          |
| `commands`          | `[ [ 'terraform', 'version' ], [ 'tofu', 'version' ] ]` | How to detect what the Terraform version is.                              |

### Variables

| Variable  | Example    | Description                          |
| --------- | ---------- | ------------------------------------ |
| version   | `v0.12.24` | The version of `terraform`           |
| workspace | `default`  | The current Terraform workspace      |
| symbol    |            | Mirrors the value of option `symbol` |
| style\*   |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

#### With Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$version $workspace]($style) '
```

#### Without Terraform version

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$workspace]($style) '
```

## Time

The `time` module shows the current **local** time.
The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option            | Default                 | Description                                                                                                            |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | The format string for the module.                                                                                      |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                             |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.    |
| `style`           | `'bold yellow'`         | The style for the module time                                                                                          |
| `utc_time_offset` | `'local'`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                            |
| `time_range`      | `'-'`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                  |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`.
Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable | Example    | Description                         |
| -------- | ---------- | ----------------------------------- |
| time     | `13:08:10` | The current time.                   |
| style\*  |            | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = '%T'
utc_time_offset = '-5'
time_range = '10:00:00-14:00:00'
```

## Typst

The `typst` module shows the current installed version of Typst used in a project.

By default, the module will be shown if any of the following conditions are met:

- The current directory contains a `template.typ` file
- The current directory contains any `*.typ` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'t '`                               | A format string representing the symbol of Typst                          |
| `style`             | `'bold #0093A7'`                     | The style for the module.                                                 |
| `detect_extensions` | `['.typ']`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `['template.typ']`                   | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `typst` module.                                              |

### Variables

| Variable      | Example   | Description                                     |
| ------------- | --------- | ----------------------------------------------- |
| version       | `v0.9.0`  | The version of `typst`, alias for typst_version |
| typst_version | `default` | The current Typst version                       |
| symbol        |           | Mirrors the value of option `symbol`            |
| style\*       |           | Mirrors the value of option `style`             |

*: This variable can only be used as a part of a style string

## Username

The `username` module shows active user's username.
The module will be shown if any of the following conditions are met:

- The current user is root/admin
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true
- The array `detect_env_vars` contains at least the name of one environment variable, that is set

> [!TIP]
> SSH connection is detected by checking environment variables
> `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up
> these variables, one workaround is to set one of them with a dummy value.

### Options

| Option            | Default                 | Description                                               |
| ----------------- | ----------------------- | --------------------------------------------------------- |
| `style_root`      | `'bold red'`            | The style used when the user is root/admin.               |
| `style_user`      | `'bold yellow'`         | The style used for non-root users.                        |
| `detect_env_vars` | `[]`                    | Which environment variable(s) should trigger this module. |
| `format`          | `'[$user]($style) in '` | The format for the module.                                |
| `show_always`     | `false`                 | Always shows the `username` module.                       |
| `disabled`        | `false`                 | Disables the `username` module.                           |
| `aliases`         | `{}`                    | Translate system usernames to something else.             |

### Variables

| Variable | Example      | Description                                                                                 |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `'red bold'` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `'matchai'`  | The currently logged-in user ID.                                                            |

### Example

#### Always show the username

```toml
# ~/.config/starship.toml

[username]
style_user = 'white bold'
style_root = 'black bold'
format = 'user: [$user]($style) '
disabled = false
show_always = true
aliases = { "corpuser034g" = "matchai" }
```

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Vagrantfile` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⍱ '`                               | A format string representing the symbol of Vagrant.                       |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['Vagrantfile']`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'cyan bold'`                        | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                            |

### Variables

| Variable | Example          | Description                          |
| -------- | ---------------- | ------------------------------------ |
| version  | `Vagrant 2.2.10` | The version of `Vagrant`             |
| symbol   |                  | Mirrors the value of option `symbol` |
| style\*  |                  | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Options

| Option              | Default                                      | Description                                                               |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`         | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'V '`                                       | A format string representing the symbol of V                              |
| `detect_extensions` | `['v']`                                      | Which extensions should trigger this module.                              |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                         | Which folders should trigger this module.                                 |
| `style`             | `'blue bold'`                                | The style for the module.                                                 |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                              |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| version  | `v0.2`  | The version of `v`                   |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCS

> Note the module is enabled by default but **not** included in the default list because that would be a breaking change.
> Additionally, the exact format of the module may change in the future, for example to handle right-aligned prompt.

The `vcs` module displays the current active Version Control System (VCS).
The module will be shown only if a configured VCS is currently in use.

### Options

| Option           | Default                                                     | Description                                           |
| ---------------- | ----------------------------------------------------------- | ----------------------------------------------------- |
| `order`          | `["git", "hg", "pijul", "fossil"]`                          | The order in which to search VCSes.                   |
| `fossil_modules` | `"$fossil_branch$fossil_metrics"`                           | Modules to show when a Fossil repository is found.    |
| `git_modules`    | `"$git_branch$git_commit$git_state$git_metrics$git_status"` | Modules to show when a Git repository is found.       |
| `hg_modules`     | `"$hg_branch$hg_state"`                                     | Modules to show when a Mercurial repository is found. |
| `pijul_modules`  | `"$pijul_channel"`                                          | Modules to show when a Pijul repository is found.     |
| `disabled`       | `false`                                                     | Disables the `vcs` module.                            |

### Example

```toml
# ~/.config/starship.toml

[vcs]
# Will look for Git then Pijul if not found but not for other VCSes at all
order = [
  "git",
  "pijul",
]
# Any module (except `$vcs` itself to avoid infinite loops) can be included here
git_modules = "$git_branch${custom.foo}"

# See documentation for custom modules
[custom.foo]
command = 'echo foo'
detect_files = ['foo']
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository.
The module will be shown only if a repository is currently in use.

### Options

| Option     | Default                          | Description                                            |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   | `''`                             | The symbol used before displaying the repository name. |
| `style`    | `'bold yellow'`                  | The style for the module.                              |
| `format`   | `'vcsh [$symbol$repo]($style) '` | The format for the module.                             |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variables

| Variable | Example                                     | Description                          |
| -------- | ------------------------------------------- | ------------------------------------ |
| repo     | `dotfiles` if in a VCSH repo named dotfiles | The active repository name           |
| symbol   |                                             | Mirrors the value of option `symbol` |
| style\*  | `black bold dimmed`                         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## XMake

The `xmake` module shows the currently installed version of [XMake](https://xmake.io/). By default
the module will be activated if any of the following conditions are met:

- The current directory contains a `xmake.lua` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'△ '`                               | The symbol used before the version of cmake.                              |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module                               |
| `detect_files`      | `['xmake.lua']`                      | Which filenames should trigger this module                                |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module                                  |
| `style`             | `'bold green'`                       | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `xmake` module.                                              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v2.9.5` | The version of xmake                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

## Zig

By default the `zig` module shows the currently installed version of [Zig](https://ziglang.org/).
The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'↯ '`                               | The symbol used before displaying the version of Zig.                     |
| `style`             | `'bold yellow'`                      | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `zig` module.                                                |
| `detect_extensions` | `['zig']`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v0.6.0` | The version of `zig`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `detect_files`
- The current directory contains a directory whose name is in `detect_folders`
- The current directory contains a file whose extension is in `detect_extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matches with `os` field if defined.

> [!TIP]
> Multiple custom modules can be defined by using a `.`.

> [!TIP]
> The order in which custom modules are shown can be individually set by including
> `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`).
> By default, the `custom` module will simply show all custom modules in the order they were defined.

> [!TIP]
> [Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules.
> If you have an interesting example not covered there, feel free to share it there!

> [!WARNING]
> If `unsafe_no_escape` is enabled or prior to starship v1.20 command output is printed unescaped to the prompt.
>
> Whatever output the command generates is printed unmodified in the prompt. This means if the output
> contains shell-specific interpretable sequences, they could be interpreted on display.
> Depending on the shell, this can mean that e.g. strings enclosed by backticks are executed by the shell.
> Such sequences are usually shell specific, e.g. you can write a command module that writes bash sequences,
> e.g. `\h`, but this module will not work in a fish or zsh shell.
>
> Format strings can also contain shell specific prompt sequences, e.g.
> [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html),
> [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

### Options

| Option              | Default                         | Description                                                                                                                                                                                                                                                                                   |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                                                                                                                                 |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the `shell` returns a `0` status code from executing it.                                                        |
| `require_repo`      | `false`                         | If `true`, the module will only be shown in paths containing a (git) repository. This option alone is not sufficient display condition in absence of other options.                                                                                                                           |
| `shell`             |                                 | [See below](#custom-command-shell)                                                                                                                                                                                                                                                            |
| `description`       | `'<custom module>'`             | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                  |
| `unsafe_no_escape`  | `false`                         | When set, command output is not escaped of characters that could be interpreted by the shell.                                                                                                                                                                                                 |
| `detect_files`      | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                                                                                                                                         |
| `detect_folders`    | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                                                                                                                                   |
| `detect_extensions` | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                                                                                                                                    |
| `symbol`            | `''`                            | The symbol used before displaying the command output.                                                                                                                                                                                                                                         |
| `style`             | `'bold green'`                  | The style for the module.                                                                                                                                                                                                                                                                     |
| `format`            | `'[$symbol($output )]($style)'` | The format for the module.                                                                                                                                                                                                                                                                    |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                |
| `os`                |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                 |
| `use_stdin`         |                                 | An optional boolean value that overrides whether commands should be forwarded to the shell via the standard input or as an argument. If unset standard input is used by default, unless the shell does not support it (cmd, nushell). Setting this disables shell-specific argument handling. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                     |

### Variables

| Variable | Description                            |
| -------- | -------------------------------------- |
| output   | The output of `command` run in `shell` |
| symbol   | Mirrors the value of option `symbol`   |
| style\*  | Mirrors the value of option `style`    |

*: This variable can only be used as a part of a style string

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to 'sh' on Linux, and 'cmd /C' on Windows.

The `command` (and `when`, if applicable) will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used,
the following arguments will automatically be added: `-NoProfile -Command -`.
If `shell` is not given or only contains one element and Starship detects Cmd will be used,
the following argument will automatically be added: `/C` and `stdin` will be set to `false`.
If `shell` is not given or only contains one element and Starship detects Nushell will be used,
the following arguments will automatically be added: `-c` and `stdin` will be set to `false`.
This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ['pwsh', '-Command', '-']
```

> [!WARNING]
> Make sure your custom shell configuration exits gracefully
>
> If you set a custom command, make sure that the default Shell used by starship
> will properly execute the command with a graceful exit (via the `shell`
> option).
>
> For example, PowerShell requires the `-Command` parameter to execute a one
> liner. Omitting this parameter might throw starship into a recursive loop
> where the shell might try to load a full profile environment with starship
> itself again and hence re-execute the custom command, getting into a never
> ending loop.
>
> Parameters similar to `-NoProfile` in PowerShell are recommended for other
> shells as well to avoid extra loading time of a custom profile on every
> starship invocation.
>
> Automatic detection of shells and proper parameters addition are currently
> implemented, but it's possible that not all shells are covered.
> [Please open an issue](https://github.com/starship/starship/issues/new/choose)
> with shell details and starship configuration if you hit such scenario.

### Example

```toml
# ~/.config/starship.toml

[custom.foo]
command = 'echo foo' # shows output of command
detect_files = ['foo'] # can specify filters but wildcards are not supported
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'

[custom.time]
command = 'time /T'
detect_extensions = ['pst'] # filters *.pst files
shell = ['pwsh.exe', '-NoProfile', '-Command', '-']

[custom.time-as-arg]
command = 'time /T'
detect_extensions = ['pst'] # filters *.pst files
shell = ['pwsh.exe', '-NoProfile', '-Command']
use_stdin = false
```
