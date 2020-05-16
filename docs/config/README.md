# Configuration

::: tip

🔥 Configuration is currently being worked on.
Many new configuration options will be available in coming releases.

:::

To get started configuring starship, create the following file: `~/.config/starship.toml`.

```sh
$ mkdir -p ~/.config && touch ~/.config/starship.toml
```

All configuration for starship is done in this [TOML](https://github.com/toml-lang/toml) file:

```toml
# Don't print a new line at the start of the prompt
add_newline = false

# Replace the "❯" symbol in the prompt with "➜"
[character]      # The name of the module we are configuring is "character"
symbol = "➜"     # The "symbol" segment is being set to "➜"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

You can change default `starship.toml` file location with `STARSHIP_CONFIG` environment variable:
```sh
export STARSHIP_CONFIG=~/.starship
```

### Terminology

**Module**: A component in the prompt giving information based on contextual information from your OS. For example, the "nodejs" module shows the version of NodeJS that is currently installed on your computer, if your current directory is a NodeJS project.

**Variable**: Smaller sub-components that contains information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of NodeJS.

By convention, most modules have a prefix of default terminal color (e.g. `via ` in "nodejs") and an empty space as a suffix.

### Format Strings

Format strings are the format that a module prints all its variables with.
Most modules have an entry called `format` that configures the display format of the module.
You can use texts, variables and text groups in a format string.

#### Variable

A variable contains a `$` symbol followed by the name of the variable.
The name of a variable only contains letters, numbers and `_`.

For example:

- `$version` is a format string with a variable named `version`.
- `$git_branch$git_commit` is a format string with two variables named `git_branch` and `git_commit`.
- `$git_branch $git_commit` has the two variables separated with a space.

#### Text Group

A text group is made up of two different parts.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings).
You can add texts, variables, or even nested text groups in it.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). This can be used style the first part.

For example:

- `[on](red bold)` will print a string `on` with bold text colored red.
- `[⬢ $version](bold green)` will print a symbol `⬢ ` followed by the content of variable `version`, with bold text colored green.
- `[a [b](red) c](green)` will print `a b c` with `b` red, and `a` and `c` green.

#### Style Strings

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

#### Escapable characters

The following symbols have special usage in a format string.
If you want to print the following symbols, you have to escape them with a backslash (`\`).

- `$`
- `\`
- `[`
- `]`
- `(`
- `)`

Note that `toml` has [its own escape syntax](https://github.com/toml-lang/toml#user-content-string).
It is recommended to use a literal string (`''`) in your config.
If you want to use a basic string (`""`), pay attention to escape the backslash `\`.

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

### Options

| Variable       | Default                       | Description                                            |
| -------------- | ----------------------------- | ------------------------------------------------------ |
| `add_newline`  | `true`                        | Add a new line before the start of the prompt.         |
| `prompt_order` | [link](#default-prompt-order) | Configure the order in which the prompt module occurs. |
| `scan_timeout` | `30`                          | Timeout for starship to scan files (in milliseconds).  |

### Example

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
add_newline = false
# Overwrite a default_prompt_order and  use custom prompt_order
prompt_order=["rust","line_break","package","line_break","character"]
# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10
```

### Default Prompt Order

The default `prompt_order` is used to define the order in which modules are shown in the prompt, if empty or no `prompt_order` is provided. The default is as shown:

```toml
prompt_order = [
    "username",
    "hostname",
    "kubernetes",
    "directory",
    "git_branch",
    "git_commit",
    "git_state",
    "git_status",
    "hg_branch",
    "docker_context",
    "package",
    "dotnet",
    "elixir",
    "elm",
    "erlang",
    "golang",
    "haskell",
    "java",
    "julia",
    "nodejs",
    "php",
    "python",
    "ruby",
    "rust",
    "terraform",
    "nix_shell",
    "conda",
    "memory_usage",
    "aws",
    "env_var",
    "crystal",
    "cmd_duration",
    "custom",
    "line_break",
    "jobs",
    "battery",
    "time",
    "character",
]
```

## AWS

The `aws` module shows the current AWS region and profile. This is based on
`AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env var with
`~/.aws/config` file.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile
is read from the `AWS_VAULT` env var.

### Options

| Variable          | Default         | Description                                                                 |
| ----------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`          | `"☁️ "`         | The symbol used before displaying the current AWS profile.                  |
| `displayed_items` | `all`           | Choose which item to display. Possible values: [`all`, `profile`, `region`] |
| `region_aliases`  |                 | Table of region aliases to display in addition to the AWS name.             |
| `style`           | `"bold yellow"` | The style for the module.                                                   |
| `disabled`        | `false`         | Disables the `AWS` module.                                                  |

### Example

```toml
# ~/.config/starship.toml

[aws]
style = "bold blue"
symbol = "🅰 "
displayed_items = "region"
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

## Battery

The `battery` module shows how charged the device's battery is and its current charging status.
The module is only visible when the device's battery is below 10%.

### Options

| Option               | Default                           | Description                                       |
| -------------------- | --------------------------------- | ------------------------------------------------- |
| `full_symbol`        | `"•"`                             | The symbol shown when the battery is full.        |
| `charging_symbol`    | `"⇡"`                             | The symbol shown when the battery is charging.    |
| `discharging_symbol` | `"⇣"`                             | The symbol shown when the battery is discharging. |
| `format`             | `"[$symbol$percentage]($style) "` | The format for the module.                        |
| `display`            | [link](#battery-display)          | Display threshold and style for the module.       |
| `disabled`           | `false`                           | Disables the `battery` module.                    |

<details>
<summary>There are also options for some uncommon battery states.</summary>

| Variable         | Description                                         |
| ---------------- | --------------------------------------------------- |
| `unknown_symbol` | The symbol shown when the battery state is unknown. |
| `empty_symbol`   | The symbol shown when the battery state is empty.   |

Note: Battery indicator will be hidden if the status is `unknown` or `empty` unless you specify the option in the config.

</details>

### Example

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### Battery Display

The `display` configuration option is used to define when the battery indicator should be shown (threshold) and what it looks like (style).
If no `display` is provided. The default is as shown:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### Options

The `display` option is an array of the following table.

| Variable    | Description                                     |
| ----------- | ----------------------------------------------- |
| `threshold` | The upper bound for the display option.         |
| `style`     | The style used if the display option is in use. |

#### Example

```toml
[[battery.display]]  # "bold red" style when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" style when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"

# when capacity is over 30%, the battery indicator will not be displayed

```

## Character

The `character` module shows a character (usually an arrow) beside where the text
is entered in your terminal.

The character will tell you whether the last command was successful or not. It
can do this in two ways: by changing color (red/green) or by changing its shape
(❯/✖). The latter will only be done if `use_symbol_for_status` is set to `true`.

### Options

| Variable                | Default        | Description                                                                         |
| ----------------------- | -------------- | ----------------------------------------------------------------------------------- |
| `symbol`                | `"❯"`          | The symbol used before the text input in the prompt.                                |
| `error_symbol`          | `"✖"`          | The symbol used before text input if the previous command failed.                   |
| `use_symbol_for_status` | `false`        | Indicate error status by changing the symbol.                                       |
| `vicmd_symbol`          | `"❮"`          | The symbol used before the text input in the prompt if shell is in vim normal mode. |
| `style_success`         | `"bold green"` | The style used if the last command was successful.                                  |
| `style_failure`         | `"bold red"`   | The style used if the last command failed.                                          |
| `disabled`              | `false`        | Disables the `character` module.                                                    |

### Example

```toml
# ~/.config/starship.toml

[character]
symbol = "➜"
error_symbol = "✗"
use_symbol_for_status = true
```

## Command Duration

The `cmd_duration` module shows how long the last command took to execute.
The module will be shown only if the command took longer than two seconds, or
the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running
`eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use
[rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec).
Simply define the arrays `preexec_functions` and `precmd_functions` before
running `eval $(starship init $0)`, and then proceed as normal.

### Options

| Option              | Default                       | Description                                                |
| ------------------- | ----------------------------- | ---------------------------------------------------------- |
| `min_time`          | `2_000`                       | Shortest duration to show time for (in milliseconds).      |
| `show_milliseconds` | `false`                       | Show milliseconds in addition to seconds for the duration. |
| `format`            | `"took [$duration]($style) "` | The format for the module.                                 |
| `style`             | `"bold yellow"`               | The style for the module.                                  |
| `disabled`          | `false`                       | Disables the `cmd_duration` module.                        |

### Variables

| Variable | Example  | Description                             |
| -------- | -------- | --------------------------------------- |
| duration | `16m40s` | The time it took to execute the command |
| style\*  |          | Mirrors the value of option `style`     |

### Example

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

### Options

| Variable            | Default        | Description                                                                                                                                                                                                 |
| ------------------- | -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`            | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `"C "`         | The symbol used before the environment name.                                                                                                                                                                |
| `style`             | `"bold green"` | The style for the module.                                                                                                                                                                                   |
| `disabled`          | `false`        | Disables the `conda` module.                                                                                                                                                                                |

### Example

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## Crystal

The `crystal` module shows the currently installed version of Crystal.
The module will be shown if any of the following conditions are met:

- The current directory contains a `shard.yml` file
- The current directory contains a `.cr` file

### Options

| Variable   | Default      | Description                                               |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"🔮 "`      | The symbol used before displaying the version of crystal. |
| `style`    | `"bold red"` | The style for the module.                                 |
| `disabled` | `false`      | Disables the `crystal` module.                            |

### Example

```toml
# ~/.config/starship.toml

[crystal]
symbol = "✨ "
style = "bold blue"
```

## Directory

The `directory` module shows the path to your current directory, truncated to
three parent folders. Your directory will also be truncated to the root of the
git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is
truncated, you will see a shortened name of each directory based on the number
you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root,
and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before
it would have been `nixpkgs/pkgs`.

### Options

| Variable            | Default       | Description                                                                      |
| ------------------- | ------------- | -------------------------------------------------------------------------------- |
| `truncation_length` | `3`           | The number of parent folders that the current directory should be truncated to.  |
| `truncate_to_repo`  | `true`        | Whether or not to truncate to the root of the git repo that you're currently in. |
| `prefix`            | `"in "`       | Prefix to display immediately before the directory.                              |
| `style`             | `"bold cyan"` | The style for the module.                                                        |
| `disabled`          | `false`       | Disables the `directory` module.                                                 |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Variable                    | Default | Description                                                                              |
| --------------------------- | ------- | ---------------------------------------------------------------------------------------- |
| `fish_style_pwd_dir_length` | `0`     | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`  | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero,
the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path
`/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as
`/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with
a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Example

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker Context

The `docker_context` module shows the currently active
[Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to
`default`.

### Options

| Option            | Default                            | Description                                                                             |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | The format for the module.                                                              |
| `symbol`          | `"🐳 "`                            | The symbol used before displaying the Docker context.                                   |
| `style`           | `"blue bold"`                      | The style for the module.                                                               |
| `only_with_files` | `false`                            | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `disabled`        | `true`                             | Disables the `docker_context` module.                                                   |

### Variables

| Variable | Example        | Description                          |
| -------- | -------------- | ------------------------------------ |
| context  | `test_context` | The current docker context           |
| symbol   |                | Mirrors the value of option `symbol` |
| style\*  |                | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If
the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module
shows the latest installed version of the SDK.

This module will only be shown in your prompt when one of the following files are present in the
current directory: `global.json`, `project.json`, `*.sln`, `*.csproj`, `*.fsproj`, `*.xproj`. You'll
also need the .NET Core command-line tools installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast
as running `dotnet --version`, but it may show an incorrect version if your .NET project has an
unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by
setting `heuristic = false` in the module options.

### Options

| Variable    | Default       | Description                                              |
| ----------- | ------------- | -------------------------------------------------------- |
| `symbol`    | `"•NET "`     | The symbol used before displaying the version of dotnet. |
| `heuristic` | `true`        | Use faster version detection to keep starship snappy.    |
| `style`     | `"bold blue"` | The style for the module.                                |
| `disabled`  | `false`       | Disables the `dotnet` module.                            |

### Example

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP.
The module will be shown if any of the following conditions are met:

- The current directory contains a `mix.exs` file.

### Options

| Variable   | Default         | Description                                                     |
| ---------- | --------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`         | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"` | The style for the module.                                       |
| `disabled` | `false`         | Disables the `elixir` module.                                   |

### Example

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm.
The module will be shown if any of the following conditions are met:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### Options

| Variable   | Default       | Description                                           |
| ---------- | ------------- | ----------------------------------------------------- |
| `symbol`   | `"🌳 "`       | The symbol used before displaying the version of Elm. |
| `style`    | `"bold cyan"` | The style for the module.                             |
| `disabled` | `false`       | Disables the `elm` module.                            |


### Example

```toml
# ~/.config/starship.toml

[elm]
symbol = " "
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variable.
The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### Options

| Option     | Default                          | Description                                                                  |
| ---------- | -------------------------------- | ---------------------------------------------------------------------------- |
| `symbol`   |                                  | The symbol used before displaying the variable value.                        |
| `variable` |                                  | The environment variable to be displayed.                                    |
| `default`  |                                  | The default value to be displayed when the selected variable is not defined. |
| `format`   | `"with [${env_value}]($style) "` | The format for the module.                                                   |
| `disabled` | `false`                          | Disables the `env_var` module.                                               |

### Variables

| Variable  | Example                                     | Description                                |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if *variable* would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Mirrors the value of option `symbol`       |
| style     | `black bold dimmed`                         | Mirrors the value of option `style`        |

### Example

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP.
The module will be shown if any of the following conditions are met:

- The current directory contains a `rebar.config` file.
- The current directory contains a `erlang.mk` file.

### Options

| Variable   | Default | Description                                                     |
| ---------- | ------- | --------------------------------------------------------------- |
| `symbol`   | `"🖧 "` | The symbol used before displaying the version of Erlang. |
| `disabled` | `false` | Disables the `erlang` module.                                   |

### Example

```toml
# ~/.config/starship.toml

[erlang]
symbol = "e "
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Options

| Variable            | Default         | Description                                                                           |
| ------------------- | --------------- | ------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the branch name of the repo in your current directory.         |
| `truncation_length` | `2^63 - 1`      | Truncates a git branch to X graphemes                                                 |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated. You can use "" for no symbol |
| `style`             | `"bold purple"` | The style for the module.                                                             |
| `disabled`          | `false`         | Disables the `git_branch` module.                                                     |

### Example

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the current commit hash of the repo in your current directory.

### Options

| Option               | Default                    | Description                                           |
| -------------------- | -------------------------- | ----------------------------------------------------- |
| `commit_hash_length` | `7`                        | The length of the displayed git commit hash.          |
| `format`             | `"[\\($hash\\)]($style) "` | The format for the module.                            |
| `style`              | `"bold green"`             | The style for the module.                             |
| `only_detached`      | `true`                     | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`                    | Disables the `git_commit` module.                     |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| hash     | `b703eb3` | The current git commit hash          |
| style\*  |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
```

## Git State

The `git_state` module will show in directories which are part of a git
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc. If there is progress information (e.g., REBASING 3/10),
that information will be shown too.

### Options

| Variable           | Default            | Description                                                                                                      |
| ------------------ | ------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `rebase`           | `"REBASING"`       | The text displayed when a `rebase` is in progress.                                                               |
| `merge`            | `"MERGING"`        | The text displayed when a `merge` is in progress.                                                                |
| `revert`           | `"REVERTING"`      | The text displayed when a `revert` is in progress.                                                               |
| `cherry_pick`      | `"CHERRY-PICKING"` | The text displayed when a `cherry-pick` is in progress.                                                          |
| `bisect`           | `"BISECTING"`      | The text displayed when a `bisect` is in progress.                                                               |
| `am`               | `"AM"`             | The text displayed when an `apply-mailbox` (`git am`) is in progress.                                            |
| `am_or_rebase`     | `"AM/REBASE"`      | The text displayed when an ambiguous `apply-mailbox` or `rebase` is in progress.                                 |
| `progress_divider` | `"/"`              | The symbol or text which will separate the current and total progress amounts. (e.g., `" of "`, for `"3 of 10"`) |
| `style`            | `"bold yellow"`    | The style for the module.                                                                                        |
| `disabled`         | `false`            | Disables the `git_state` module.                                                                                 |

### Example

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your
current directory.

### Options

| Variable           | Default                    | Description                                             |
| ------------------ | -------------------------- | ------------------------------------------------------- |
| `conflicted`       | `"="`                      | This branch has merge conflicts.                        |
| `conflicted_count` | [link](#git-status-counts) | Show and style the number of conflicts.                 |
| `ahead`            | `"⇡"`                      | This branch is ahead of the branch being tracked.       |
| `behind`           | `"⇣"`                      | This branch is behind of the branch being tracked.      |
| `diverged`         | `"⇕"`                      | This branch has diverged from the branch being tracked. |
| `untracked`        | `"?"`                      | There are untracked files in the working directory.     |
| `untracked_count`  | [link](#git-status-counts) | Show and style the number of untracked files.           |
| `stashed`          | `"$"`                      | A stash exists for the local repository.                |
| `stashed_count`    | [link](#git-status-counts) | Show and style the number of stashes.                   |
| `modified`         | `"!"`                      | There are file modifications in the working directory.  |
| `modified_count`   | [link](#git-status-counts) | Show and style the number of modified files.            |
| `staged`           | `"+"`                      | A new file has been added to the staging area.          |
| `staged_count`     | [link](#git-status-counts) | Show and style the number of files staged files.        |
| `renamed`          | `"»"`                      | A renamed file has been added to the staging area.      |
| `renamed_count`    | [link](#git-status-counts) | Show and style the number of renamed files.             |
| `deleted`          | `"✘"`                      | A file's deletion has been added to the staging area.   |
| `deleted_count`    | [link](#git-status-counts) | Show and style the number of deleted files.             |
| `show_sync_count`  | `false`                    | Show ahead/behind count of the branch being tracked.    |
| `prefix`           | `[`                        | Prefix to display immediately before git status.        |
| `suffix`           | `]`                        | Suffix to display immediately after git status.         |
| `style`            | `"bold red"`               | The style for the module.                               |
| `disabled`         | `false`                    | Disables the `git_status` module.                       |

#### Git Status Counts

| Variable  | Default | Description                                            |
| --------- | ------- | ------------------------------------------------------ |
| `enabled` | `false` | Show the number of files                               |
| `style`   |         | Optionally style the count differently than the module |

### Example

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
staged.value = "++"
staged.style = "green"
staged_count.enabled = true
staged_count.style = "green"
renamed = "👅"
deleted = "🗑"
```

## Golang

The `golang` module shows the currently installed version of Golang.
The module will be shown if any of the following conditions are met:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Options

| Option     | Default                            | Description                                     |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🐹 "`                            | A format string representing the symbol of Go.  |
| `style`    | `"bold cyan"`                      | The style for the module.                       |
| `disabled` | `false`                            | Disables the `golang` module.                   |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v1.12.1` | The version of `go`                  |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version.
The module will be shown if any of the following conditions are met:

- The current directory contains a `stack.yaml` or `stack.yml` file
- The current directory contains a `package.yaml` or `package.yml` file

### Options

| Option     | Default                             | Description                                        |
| ---------- | ----------------------------------- | -------------------------------------------------- |
| `format`   | `"via [${symbol}${version}](${style}) "` | The format for the module.                         |
| `symbol`   | `"λ "`                               | A format string representing the symbol of Haskell |
| `style`    | `"bold red"`                        | The style for the module.                          |
| `disabled` | `false`                             | Disables the `haskell` module.                     |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v8.8.3` | The version of `ghc`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style    |          | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[haskell]
symbol = " "
```

## Hostname

The `hostname` module shows the system hostname.

### Options

| Variable   | Default               | Description                                                                                                                          |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                | Only show hostname when connected to an SSH session.                                                                                 |
| `prefix`   | `""`                  | Prefix to display immediately before the hostname.                                                                                   |
| `suffix`   | `""`                  | Suffix to display immediately after the hostname.                                                                                    |
| `trim_at`  | `"."`                 | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `style`    | `"bold dimmed green"` | The style for the module.                                                                                                            |
| `disabled` | `false`               | Disables the `hostname` module.                                                                                                      |

### Example

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
prefix = "⟪"
suffix = "⟫"
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of Java.
The module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### Options

| Option     | Default                             | Description                                     |
| ---------- | ----------------------------------- | ----------------------------------------------- |
| `format`   | `"via [${symbol}${version}]($style) "` | The format for the module.                      |
| `symbol`   | `"☕ "`                               | A format string representing the symbol of Java |
| `style`    | `"red dimmed"`                      | The style for the module.                       |
| `disabled` | `false`                             | Disables the `java` module.                     |

### Variables

| Variable | Example      | Description                          |
| -------- | ------------ | ------------------------------------ |
| version  | "v14"        | The version of `java`                |
| symbol   | "☕"          | Mirrors the value of option `symbol` |
| style    | "red dimmed" | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running.
The module will be shown only if there are background jobs running.
The module will show the number of jobs running if there is more than 1 job, or
more than the `threshold` config value, if it exists.

### Options

| Variable    | Default       | Description                                           |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦"`         | The symbol used before displaying the number of jobs. |
| `threshold` | `1`           | Show number of jobs if exceeded.                      |
| `style`     | `"bold blue"` | The style for the module.                             |
| `disabled`  | `false`       | Disables the `jobs` module.                           |

### Example

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia.
The module will be shown if any of the following conditions are met:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Options

| Variable   | Default       | Description                                              |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"ஃ "`       | The symbol used before displaying the version of Julia. |
| `style`    | `"bold purple"` | The style for the module.                                |
| `disabled` | `false`       | Disables the `julia` module.                            |

### Example

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file.
The namespace needs to be set in the kubeconfig file, this can be done via
`kubectl config set-context starship-cluster --namespace astronaut`.
If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default.
To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Option                  | Default                                             | Description                                                           |
| ----------------------- | --------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`                | `"☸ "`                                              | A format string representing the symbol displayed before the Cluster. |
| `format`                | `"on [$symbol$context( \\($namespace\\))]($style) "` | The format for the module.                                            |
| `style`                 | `"cyan bold"`                                       | The style for the module.                                             |
| `namespace_spaceholder` | `none`                                              | The value to display if no namespace was found.                       |
| `context_aliases`       |                                                     | Table of context aliases to display.                                  |
| `disabled`              | `true`                                              | Disables the `kubernetes` module.                                     |

### Variables

| Variable  | Example              | Description                              |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Mirrors the value of option `symbol`     |
| style\*   |                      | Mirrors the value of option `style`      |

\*: This variable can only be used as a part of a style string

### Example

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

### Options

| Variable   | Default | Description                                                        |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### Example

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default.
To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Option      | Default                                       | Description                                              |
| ----------- | --------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                          | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | The format for the module.                               |
| `symbol`    | `"🐏"`                                         | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                         | The style for the module.                                |
| `disabled`  | `true`                                        | Disables the `memory_usage` module.                      |

### Variables

| Variable    | Example       | Description                                                        |
| ----------- | ------------- | ------------------------------------------------------------------ |
| ram         | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct     | `48%`         | The percentage of the current system memory.                       |
| swap\**     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\** | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol      | `🐏`           | Mirrors the value of option `symbol`                               |
| style\*     |               | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string
\**: The SWAP file information is only displayed if detected on the current system

### Example

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

### Options

| Variable            | Default         | Description                                                                                  |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated.                                     |
| `style`             | `"bold purple"` | The style for the module.                                                                    |
| `disabled`          | `true`          | Disables the `hg_branch` module.                                                             |

### Example

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment.
The module will be shown when inside a nix-shell environment.

### Options

| Variable     | Default       | Description                                       |
| ------------ | ------------- | ------------------------------------------------- |
| `use_name`   | `false`       | Display the name of the nix-shell.                |
| `impure_msg` | `"impure"`    | Customize the "impure" msg.                       |
| `pure_msg`   | `"pure"`      | Customize the "pure" msg.                         |
| `symbol`     | `"❄️  "`       | The symbol used before displaying the shell name. |
| `style`      | `"bold blue"` | The style for the module.                         |
| `disabled`   | `false`       | Disables the `nix_shell` module.                  |

### Example

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
symbol = "☃️  "
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS.
The module will be shown if any of the following conditions are met:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js` extension

### Options

| Variable   | Default        | Description                                              |
| ---------- | -------------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`         | The symbol used before displaying the version of NodeJS. |
| `style`    | `"bold green"` | The style for the module.                                |
| `disabled` | `false`        | Disables the `nodejs` module.                            |

### Example

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## Package Version

The `package` module is shown when the current directory is the repository for a
package, and shows its current version. The module currently supports `npm`, `cargo`,
`poetry`, `composer`, `gradle`, `julia` and `mix` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present
  in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present
  in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present
  in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present
  in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present

> ⚠️ The version being shown is that of the package whose source code is in your
> current directory, not your package manager.

### Options

| Variable          | Default      | Description                                                |
| ----------------- | ------------ | ---------------------------------------------------------- |
| `symbol`          | `"📦 "`      | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"` | The style for the module.                                  |
| `display_private` | `false`      | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`      | Disables the `package` module.                             |

### Example

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## PHP

The `php` module shows the currently installed version of PHP.
The module will be shown if any of the following conditions are met:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### Options

| Option     | Default                            | Description                                           |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                            |
| `symbol`   | `"🐘 "`                            | The symbol used before displaying the version of PHP. |
| `style`    | `"147 bold"`                       | The style for the module.                             |
| `disabled` | `false`                            | Disables the `php` module.                            |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v7.3.8` | The version of `php`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## Python

The `python` module shows the currently installed version of Python and the
current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version
name. Otherwise, it will display the version number from `python --version`.

The module will be shown if any of the following conditions are met:

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- The current directory contains a `setup.py` file
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### Options

| Option               | Default                                                    | Description                                                                 |
| -------------------- | ---------------------------------------------------------- | --------------------------------------------------------------------------- |
| `format`             | `"via [${symbol}${version}( \\($virtualenv\\))]($style) "` | The format for the module.                                                  |
| `symbol`             | `"🐍 "`                                                     | A format string representing the symbol of Python                           |
| `style`              | `"yellow bold"`                                            | The style for the module.                                                   |
| `pyenv_version_name` | `false`                                                    | Use pyenv to get Python version                                             |
| `scan_for_pyfiles`   | `true`                                                     | If false, Python files in the current directory will not show this module.  |
| `disabled`           | `false`                                                    | Disables the `python` module.                                               |

### Variables

| Variable   | Example         | Description                          |
| ---------- | --------------- | ------------------------------------ |
| version    | `"v3.8.1"`      | The version of `python`              |
| symbol     | `"🐍 "`          | Mirrors the value of option `symbol` |
| style      | `"yellow bold"` | Mirrors the value of option `style`  |
| virtualenv | `"venv"`        | The current `virtualenv` name        |

### Example

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

The `ruby` module shows the currently installed version of Ruby.
The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Options

| Variable   | Default      | Description                                            |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"💎 "`      | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold red"` | The style for the module.                              |
| `disabled` | `false`      | Disables the `ruby` module.                            |

### Example

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust.
The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Options

| Option     | Default                            | Description                                     |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🦀 "`                            | A format string representing the symbol of Rust |
| `style`    | `"bold red"`                       | The style for the module.                       |
| `disabled` | `false`                            | Disables the `rust` module.                     |

### Variables

| Variable | Example           | Description                          |
| -------- | ----------------- | ------------------------------------ |
| version  | `v1.43.0-nightly` | The version of `rustc`               |
| symbol   |                   | Mirrors the value of option `symbol` |
| style\*  |                   | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container
and `$SINGULARITY_NAME` is set.

### Options

| Variable   | Default                          | Description                                      |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `"[$symbol\\[$env\\]]($style) "` | The format for the module.                       |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | The style for the module.                        |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variables

| Variable | Example      | Description                          |
| -------- | ------------ | ------------------------------------ |
| env      | `centos.img` | The current singularity image        |
| symbol   |              | Mirrors the value of option `symbol` |
| style\*  |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[singularity]
format = "[📦 \\[$env\\]]($style) "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version.
By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use.
The module will be shown if any of the following conditions are met:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### Options

| Variable       | Default      | Description                                                 |
| -------------- | ------------ | ----------------------------------------------------------- |
| `symbol`       | `"💠 "`      | The symbol used before displaying the terraform workspace.  |
| `show_version` | `false`      | Shows the terraform version. Very slow on large workspaces. |
| `style`        | `"bold 105"` | The style for the module.                                   |
| `disabled`     | `false`      | Disables the `terraform` module.                            |

### Example

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## Time

The `time` module shows the current **local** time.
The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default.
To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Variable          | Default         | Description                                                                                                            |
| ----------------- | --------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`         | Enables 12 hour formatting                                                                                             |
| `format`          | see below       | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.    |
| `style`           | `"bold yellow"` | The style for the module time                                                                                          |
| `utc_time_offset` | `"local"`       | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`          | Disables the `time` module.                                                                                            |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`.
Manually setting `format` will override the `use_12hr` setting.

### Example

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = "-5"
```

## Username

The `username` module shows active user's username.
The module will be shown if any of the following conditions are met:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

### Options

| Variable      | Default         | Description                           |
| ------------- | --------------- | ------------------------------------- |
| `style_root`  | `"bold red"`    | The style used when the user is root. |
| `style_user`  | `"bold yellow"` | The style used for non-root users.    |
| `show_always` | `false`         | Always shows the `username` module.   |
| `disabled`    | `false`         | Disables the `username` module.       |

### Example

```toml
# ~/.config/starship.toml

[username]
disabled = true
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

The order in which custom modules are shown can be individually set
by setting `custom.foo` in `prompt_order`. By default, the `custom` module
will simply show all custom modules in the order they were defined.

:::

### Options

| Variable      | Default             | Description                                                                  |
| ------------- | ------------------- | ---------------------------------------------------------------------------- |
| `command`     |                     | The command whose output should be printed.                                  |
| `when`        |                     | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                     | The path to the shell to use to execute the command. If unset, it will fallback to STARSHIP_SHELL and then to "sh". |
| `description` | `"<custom module>"` | The description of the module that is shown when running `starship explain`. |
| `files`       | `[]`                | The files that will be searched in the working directory for a match.        |
| `directories` | `[]`                | The directories that will be searched in the working directory for a match.  |
| `extensions`  | `[]`                | The extensions that will be searched in the working directory for a match.   |
| `symbol`      | `""`                | The symbol used before displaying the command output.                        |
| `style`       | `"bold green"`      | The style for the module.                                                    |
| `prefix`      | `""`                | Prefix to display immediately before the command output.                     |
| `suffix`      | `""`                | Suffix to display immediately after the command output.                      |
| `disabled`    | `false`             | Disables this `custom` module.                                               |

### Example

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
prefix = " transcending "
```
