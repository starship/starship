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

## Prompt

This is the list of prompt-wide configuration options.

### Options

| Variable      | Default | Description                                    |
| ------------- | ------- | ---------------------------------------------- |
| `add_newline` | `true`  | Add a new line before the start of the prompt. |

### Example

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
add_newline = false
```

## Battery

The `battery` module shows how charged the device's battery is and its current charging status.
The module is only visible when the device's battery is below 10%.

### Options

| Variable             | Default | Description                                       |
| -------------------- | ------- | ------------------------------------------------- |
| `full_symbol`        | `"•"`   | The symbol shown when the battery is full.        |
| `charging_symbol`    | `"⇡"`   | The symbol shown when the battery is charging.    |
| `discharging_symbol` | `"⇣"`   | The symbol shown when the battery is discharging. |
| `disabled`           | `false` | Disables the `battery` module.                    |

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

| Variable                | Default | Description                                                                       |
| ----------------------- | ------- | --------------------------------------------------------------------------------- |
| `symbol`                | `"❯"`   | The symbol used before the text input in the prompt.                              |
| `error_symbol`          | `"✖"`   | The symbol used before text input if the previous command failed.                 |
| `use_symbol_for_status` | `false` | Indicate error status by changing the symbol.                                     |
| `vicmd_symbol`          | `"❮"`   | The symbol used before the text input in the prompt if zsh is in vim normal mode. |
| `disabled`              | `false` | Disables the `character` module.                                                  |

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

| Variable   | Default | Description                         |
| ---------- | ------- | ----------------------------------- |
| `min_time` | `2`     | Shortest duration to show time for. |
| `disabled` | `false` | Disables the `cmd_duration` module. |

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

### Options

| Variable            | Default | Description                                                                      |
| ------------------- | ------- | -------------------------------------------------------------------------------- |
| `truncation_length` | `3`     | The number of parent folders that the current directory should be truncated to.  |
| `truncate_to_repo`  | `true`  | Whether or not to truncate to the root of the git repo that you're currently in. |
| `disabled`          | `false` | Disables the `directory` module.                                                 |

### Example

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Options

| Variable   | Default | Description                                                                   |
| ---------- | ------- | ----------------------------------------------------------------------------- |
| `symbol`   | `" "`  | The symbol used before the branch name of the repo in your current directory. |
| `disabled` | `false` | Disables the `git_branch` module.                                             |

### Example

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your
current directory.

### Options

| Variable     | Default | Description                                             |
| ------------ | ------- | ------------------------------------------------------- |
| `conflicted` | `"="`   | This branch has merge conflicts.                        |
| `ahead`      | `"⇡"`   | This branch is ahead of the branch being tracked.       |
| `behind`     | `"⇣"`   | This branch is behind of the branch being tracked.      |
| `diverged`   | `"⇕"`   | This branch has diverged from the branch being tracked. |
| `untracked`  | `"?"`   | There are untracked files in the working directory.     |
| `stashed`    | `"$"`   | A stash exists for the local repository.                |
| `modified`   | `"!"`   | There are file modifications in the working directory.  |
| `added`      | `"+"`   | A new file has been added to the staging area.          |
| `renamed`    | `"»"`   | A renamed file has been added to the staging area.      |
| `deleted`    | `"✘"`   | A file's deletion has been added to the staging area.   |
| `disabled`   | `false` | Disables the `git_status` module.                       |

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
added = "➕"
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

| Variable   | Default | Description                                              |
| ---------- | ------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "` | The symbol used before displaying the version of Golang. |
| `disabled` | `false` | Disables the `golang` module.                            |

### Example

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```

## Jobs

The `jobs` module shows the current number of jobs running.
The module will be shown only if there are background jobs running.
The module will show the number of jobs running if there is more than 1 job, or
more than the `threshold` config value, if it exists.

### Options

| Variable    | Default | Description                      |
| ----------- | ------- | -------------------------------- |
| `threshold` | `1`     | Show number of jobs if exceeded. |
| `disabled`  | `false` | Disables the `jobs` module.      |

### Example

```toml
# ~/.config/starship.toml

[jobs]
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

## Ruby

The `ruby` module shows the currently installed version of Ruby.
The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.rb` file

### Options

| Variable   | Default | Description                 |
| ---------- | ------- | --------------------------- |
| `disabled` | `false` | Disables the `ruby` module. |

### Example

```toml
# ~/.config/starship.toml

[ruby]
disabled = false
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS.
The module will be shown if any of the following conditions are met:

- The current directory contains a `package.json` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js` extension

### Options

| Variable   | Default | Description                                              |
| ---------- | ------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`  | The symbol used before displaying the version of NodeJS. |
| `disabled` | `false` | Disables the `nodejs` module.                            |

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

| Variable   | Default | Description                                                |
| ---------- | ------- | ---------------------------------------------------------- |
| `symbol`   | `"📦 "` | The symbol used before displaying the version the package. |
| `disabled` | `false` | Disables the `package` module.                             |

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

### Options

| Variable             | Default    | Description                                                                 |
| -------------------- | ---------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`    | The symbol used before displaying the version of Python.                    |
| `disabled`           | `false`    | Disables the `python` module.                                               |
| `pyenv_version_name` | `false`    | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "` | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |

### Example

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Rust

The `rust` module shows the currently installed version of Rust.
The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Options

| Variable   | Default | Description                                              |
| ---------- | ------- | -------------------------------------------------------- |
| `symbol`   | `"🦀 "` | The symbol used before displaying the version of Python. |
| `disabled` | `false` | Disables the `rust` module.                              |

### Example

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Nim

The `nim` module shows the currently installed version of Nim.
The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nimble` extension

### Options

| Variable   | Default | Description                                              |
| ---------- | ------- | -------------------------------------------------------- |
| `symbol`   | `"👑 "` | The symbol used before displaying the version of Nim.    |
| `disabled` | `false` | Disables the `nim` module.                               |

### Example

```toml
# ~/.config/starship.toml

[nim]
symbol = "👑 "
```

## Username

The `username` module shows active user's username.
The module will be shown if any of the following conditions are met:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session

### Options

| Variable   | Default | Description                     |
| ---------- | ------- | ------------------------------- |
| `disabled` | `false` | Disables the `username` module. |

### Example

```toml
# ~/.config/starship.toml

[username]
disabled = true
```
