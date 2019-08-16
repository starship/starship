# Configuration

::: tip
🔥 Configuration is currently being worked on.
Many new configuration options will be available in coming releases.
:::

To get started configuring starship, create the following file: `~/.config/starship.toml`.

```shell
$ touch ~/.config/starship.toml
```

All configuration for starship is done in this [TOML](https://github.com/toml-lang/toml) file:

```toml
# Don't print a new line at the start of the prompt
add_newline = false

# Replace the "❯" symbol in the prompt with "➜"
[character]      # The name of the module we are confguring is "character"
symbol = "➜"     # The "symbol" segment is being set to "➜"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

### Terminology

**Module**: A component in the prompt giving information based on contextual information from your OS. For example, the "nodejs" module shows the version of NodeJS that is currently installed on your computer, if your current directory is a NodeJS project.

**Segment**: Smaller sub-components that compose a module. For example, the "symbol" segment in the "nodejs" module contains the character that is shown before the version number (⬢ by default).

Here is the representation of the node module. In the following example, "symbol" and "version"
are segments within it. Every module also has a prefix and suffix that are the default terminal color.

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### Style Strings

Most modules in starship allow you to configure their display styles. This is done with an entry (usually called `style`) which is a string specifying the configuration. Here are some examples of style strings along with what they do. For details on the full syntax, consult the [advanced config guide](/advanced-config/).

- `"fg:green bg:blue"` sets green text on a blue background
- `"bg:blue fg:bright-green"` sets bright green text on a blue background
- `"bold fg:27"` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` sets underlined text on a burnt orange background
- `""` explicitly disables all styling

Note that what styling looks like will be controlled by your terminal emulator. For example, some terminal emulators will brighten the colors instead of bolding text, and some color themes use the same values for the normal and bright colors.

## Prompt

This is the list of prompt-wide configuration options.

### Options

| Variable       | Default                       | Description                                            |
| -------------- | ----------------------------- | ------------------------------------------------------ |
| `add_newline`  | `true`                        | Add a new line before the start of the prompt.         |
| `prompt_order` | [link](#default-prompt-order) | Configure the order in which the prompt module occurs. |

### Example

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
add_newline = false
# Overwrite a default_prompt_order and  use custom prompt_order
prompt_order=["rust","line_break","package","line_break","character"]
```

### Default prompt order

The `default_prompt_order` configuration option is used to define the order in which modules are shown in the prompt, if empty or no `prompt_order` is provided. The default is as shown:

```
default_prompt_order = [
    "username",
    "hostname",
    "directory",
    "git_branch",
    "git_state",
    "git_status",
    "package",
    "nodejs",
    "ruby",
    "rust",
    "python",
    "golang",
    "nix_shell",
    "cmd_duration",
    "line_break",
    "jobs",
    "battery",
    "character",
]
```

## Battery

The `battery` module shows how charged the device's battery is and its current charging status.
The module is only visible when the device's battery is below 10%.

### Options

| Variable             | Default      | Description                                       |
| -------------------- | ------------ | ------------------------------------------------- |
| `full_symbol`        | `"•"`        | The symbol shown when the battery is full.        |
| `charging_symbol`    | `"⇡"`        | The symbol shown when the battery is charging.    |
| `discharging_symbol` | `"⇣"`        | The symbol shown when the battery is discharging. |
| `style`              | `"bold red"` | The style for the module.                         |
| `disabled`           | `false`      | Disables the `battery` module.                    |

### Example

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
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

| Variable   | Default         | Description                         |
| ---------- | --------------- | ----------------------------------- |
| `min_time` | `2`             | Shortest duration to show time for. |
| `style`    | `"bold yellow"` | The style for the module.           |
| `disabled` | `false`         | Disables the `cmd_duration` module. |

### Example

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 4
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

| Variable                    | Default       | Description                                                                      |
| --------------------------- | ------------- | -------------------------------------------------------------------------------- |
| `truncation_length`         | `3`           | The number of parent folders that the current directory should be truncated to.  |
| `truncate_to_repo`          | `true`        | Whether or not to truncate to the root of the git repo that you're currently in. |
| `fish_style_pwd_dir_length` | `0`           | The number of characters to use when applying fish shell pwd path logic.         |
| `style`                     | `"bold cyan"` | The style for the module.                                                        |
| `disabled`                  | `false`       | Disables the `directory` module.                                                 |

### Example

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
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
truncation_length = "4"
truncation_symbol = ""
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

| Variable          | Default      | Description                                             |
| ----------------- | ------------ | ------------------------------------------------------- |
| `conflicted`      | `"="`        | This branch has merge conflicts.                        |
| `ahead`           | `"⇡"`        | This branch is ahead of the branch being tracked.       |
| `behind`          | `"⇣"`        | This branch is behind of the branch being tracked.      |
| `diverged`        | `"⇕"`        | This branch has diverged from the branch being tracked. |
| `untracked`       | `"?"`        | There are untracked files in the working directory.     |
| `stashed`         | `"$"`        | A stash exists for the local repository.                |
| `modified`        | `"!"`        | There are file modifications in the working directory.  |
| `staged`          | `"+"`        | A new file has been added to the staging area.          |
| `renamed`         | `"»"`        | A renamed file has been added to the staging area.      |
| `deleted`         | `"✘"`        | A file's deletion has been added to the staging area.   |
| `show_sync_count` | `false`      | Show ahead/behind count of the branch being tracked.    |
| `style`           | `"bold red"` | The style for the module.                               |
| `disabled`        | `false`      | Disables the `git_status` module.                       |

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
staged = "➕"
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
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Options

| Variable   | Default       | Description                                              |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`       | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | The style for the module.                                |
| `disabled` | `false`       | Disables the `golang` module.                            |

### Example

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```


## Hostname

The `hostname` module shows the system hostname.

### Options

| Variable   | Default               | Description                                          |
| ---------- | --------------------- | ---------------------------------------------------- |
| `ssh_only` | `true`                | Only show hostname when connected to an SSH session. |
| `prefix`   | `""`                  | Prefix to display immediately before the hostname.   |
| `suffix`   | `""`                  | Suffix to display immediately after the hostname.    |
| `style`    | `"bold dimmed green"` | The style for the module.                            |
| `disabled` | `false`               | Disables the `hostname` module.                      |

### Example

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
prefix = "⟪"
suffix = "⟫"
disabled = false
```


## Jobs

The `jobs` module shows the current number of jobs running.
The module will be shown only if there are background jobs running.
The module will show the number of jobs running if there is more than 1 job, or
more than the `threshold` config value, if it exists.

### Options

| Variable    | Default       | Description                                           |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦ "`        | The symbol used before displaying the number of jobs. |
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


## Nix-shell

The `nix_shell` module shows the nix-shell environment.
The module will be shown when inside a nix-shell environment.

### Options

| Variable     | Default      | Description                        |
| ------------ | ------------ | ---------------------------------- |
| `use_name`   | `false`      | Display the name of the nix-shell. |
| `impure_msg` | `impure`     | Customize the "impure" msg.        |
| `pure_msg`   | `pure`       | Customize the "pure" msg.          |
| `style`      | `"bold red"` | The style for the module.          |
| `disabled`   | `false`      | Disables the `nix_shell` module.   |

### Example

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
```


## NodeJS

The `nodejs` module shows the currently installed version of NodeJS.
The module will be shown if any of the following conditions are met:

- The current directory contains a `package.json` file
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
and `poetry` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present
  in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present
  in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present
  in the current directory

> ⚠️ The version being shown is that of the package whose source code is in your
> current directory, not your package manager.

### Options

| Variable   | Default      | Description                                                |
| ---------- | ------------ | ---------------------------------------------------------- |
| `symbol`   | `"📦 "`      | The symbol used before displaying the version the package. |
| `style`    | `"bold red"` | The style for the module.                                  |
| `disabled` | `false`      | Disables the `package` module.                             |

### Example

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## Python

The `python` module shows the currently installed version of Python.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name.

Otherwise, it will display the version number from `python --version`
and show the current Python virtual environment if one is
activated.

The module will be shown if any of the following conditions are met:

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension
- The current directory contains a `Pipfile` file

### Options

| Variable             | Default         | Description                                                                 |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`         | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
| `style`              | `"bold yellow"` | The style for the module.                                                   |
| `disabled`           | `false`         | Disables the `python` module.                                               |

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

| Variable   | Default      | Description                                            |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`      | The symbol used before displaying the version of Rust. |
| `style`    | `"bold red"` | The style for the module.                              |
| `disabled` | `false`      | Disables the `rust` module.                            |

### Example

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Time

The `time` module shows the current **local** time.

::: tip
For the `format` configuration value, take a look [here](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what you can pass in.
:::

### Options

| Variable   | Default | Description                                  |
| ---------- | ------- | -------------------------------------------- |
| `12hr`     | `false` | Enables 12 hour formatting                   |
| `format`   | `''`    | Accepts a [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to customize the time formatting |
| `disabled` | `true`  | Disables the `time` module.                  |

### Example

```toml
# ~/.config/starship.toml

[time]
disabled = true
```

## Username

The `username` module shows active user's username.
The module will be shown if any of the following conditions are met:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session

### Options

| Variable     | Default         | Description                           |
| ------------ | --------------- | ------------------------------------- |
| `style_root` | `"bold red"`    | The style used when the user is root. |
| `style_user` | `"bold yellow"` | The style used for non-root users.    |
| `disabled`   | `false`         | Disables the `username` module.       |

### Example

```toml
# ~/.config/starship.toml

[username]
disabled = true
```
