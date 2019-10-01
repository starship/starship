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
[character]      # The name of the module we are configuring is "character"
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
- `"bold italic fg:purple"` sets bold italic purple text
- `""` explicitly disables all styling

Note that what styling looks like will be controlled by your terminal emulator. For example, some terminal emulators will brighten the colors instead of bolding text, and some color themes use the same values for the normal and bright colors. Also, to get italic text, your terminal must support italics.

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

### Default Prompt Order

The default `prompt_order` is used to define the order in which modules are shown in the prompt, if empty or no `prompt_order` is provided. The default is as shown:

```toml
prompt_order = [
    "username",
    "hostname",
    "kubernetes",
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
    "java",
    "nix_shell",
    "memory_usage",
    "aws",
    "env_var",
    "cmd_duration",
    "line_break",
    "jobs",
    "battery",
    "time",
    "character",
]
```

## AWS

The `aws` module shows the current AWS profile. This is based on the
`AWS_PROFILE` env var.

### Options

| Variable   | Default         | Description                                                |
| ---------- | --------------- | ---------------------------------------------------------- |
| `symbol`   | `"☁️ "`         | The symbol used before displaying the current AWS profile. |
| `style`    | `"bold yellow"` | The style for the module.                                  |
| `disabled` | `false`         | Disables the `AWS` module.                                 |

### Example

```toml
# ~/.config/starship.toml

[aws]
style = "bold blue"
symbol = "🅰 "
```

## Battery

The `battery` module shows how charged the device's battery is and its current charging status.
The module is only visible when the device's battery is below 10%.

### Options

| Variable             | Default                  | Description                                       |
| -------------------- | ------------------------ | ------------------------------------------------- |
| `full_symbol`        | `"•"`                    | The symbol shown when the battery is full.        |
| `charging_symbol`    | `"⇡"`                    | The symbol shown when the battery is charging.    |
| `discharging_symbol` | `"⇣"`                    | The symbol shown when the battery is discharging. |
| `display`            | [link](#battery-display) | Display threshold and style for the module.       |
| `disabled`           | `false`                  | Disables the `battery` module.                    |

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

| Variable   | Default         | Description                                                |
| ---------- | --------------- | ---------------------------------------------------------- |
| `min_time` | `2`             | Shortest duration to show time for.                        |
| `prefix`   | `took`          | Prefix to display immediately before the command duration. |
| `style`    | `"bold yellow"` | The style for the module.                                  |
| `disabled` | `false`         | Disables the `cmd_duration` module.                        |

### Example

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 4
prefix = "underwent "
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
| `style`             | `"bold cyan"` | The style for the module.                                                        |
| `disabled`          | `false`       | Disables the `directory` module.                                                 |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Variable                    | Default | Description                                                                              |
| --------------------------- | ------- | ---------------------------------------------------------------------------------------- |
| `fish_style_pwd_dir_length` | `0`     | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true`  | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

</details>

### Example

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variable.
The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### Options

| Variable   | Default          | Description                                                                  |
| ---------- | ---------------- | ---------------------------------------------------------------------------- |
| `symbol`   |                  | The symbol used before displaying the variable value.                        |
| `variable` |                  | The environment variable to be displayed.                                    |
| `default`  |                  | The default value to be displayed when the selected variable is not defined. |
| `prefix`   | `""`             | Prefix to display immediately before the variable value.                     |
| `suffix`   | `""`             | Suffix to display immediately after the variable value.                      |
| `style`    | `"dimmed black"` | The style for the module.                                                    |
| `disabled` | `false`          | Disables the `env_var` module.                                               |

### Example

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
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
| `prefix`          | `[`          | Prefix to display immediately before git status.        |
| `suffix`          | `]`          | Suffix to display immediately after git status.         |
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


## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from
the kubeconfig file. The namespace needs to be set in the kubeconfig file, this
can be done via `kubectl config set-context starship-cluster --namespace
astronaut`. If the `$KUBECONFIG` env var is set the module will use that if
not it will use the `~/.kube/config`.

### Options

| Variable   | Default       | Description                                         |
| ---------- | ------------- | --------------------------------------------------- |
| `symbol`   | `"☸ "`       | The symbol used before displaying the Cluster info. |
| `style`    | `"bold blue"` | The style for the module.                           |
| `disabled` | `false`       | Disables the `kubernetes` module                    |

### Example

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dim green"
disabled = true
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

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default.
To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Variable          | Default                  | Description                                                   |
| ----------------- | ------------------------ | ------------------------------------------------------------- |
| `show_percentage` | `false`                  | Display memory usage as a percentage of the available memory. |
| `show_swap`       | when total swap non-zero | Display swap usage.                                           |
| `threshold`       | `75`                     | Hide the memory usage unless it exceeds this percentage.      |
| `symbol`          | `"🐏 "`                  | The symbol used before displaying the memory usage.           |
| `style`           | `"bold dimmed white"`    | The style for the module.                                     |
| `disabled`        | `true`                   | Disables the `memory_usage` module.                           |

### Example

```toml
# ~/.config/starship.toml

[memory_usage]
show_percentage = true
show_swap = true
threshold = -1
icon = " "
style = "bold dimmed green"
```

## Java

The `java` module shows the currently installed version of Java.
The module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml` or `build.gradle` file
- The current directory contains a file with the `.java`, `.class` or `.jar` extension

### Options

| Variable   | Default        | Description                                            |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"☕ "`        | The symbol used before displaying the version of Java. |
| `style`    | `"dimmed red"` | The style for the module.                              |
| `disabled` | `false`        | Disables the `java` module.                            |

### Example

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
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
- The current directory contains a `tox.ini` file

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
The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default.
To enable it, set `disabled` to `false` in your configuration file.

:::

### Options

| Variable   | Default       | Description                                                                                                         |
| ---------- | ------------- | ------------------------------------------------------------------------------------------------------------------- |
| `12hr`     | `false`       | Enables 12 hour formatting                                                                                          |
| `format`   | see below     | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`    | `bold yellow` | The style for the module time                                                                                       |
| `disabled` | `true`        | Disables the `time` module.                                                                                         |

If `12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`.
Manually setting `format` will override the `12hr` setting.

### Example

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
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
