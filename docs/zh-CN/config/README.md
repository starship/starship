# 配置

::: tip

Starship 目前正在开发中。 很多新的配置选项将会在之后的版本中被公开。

:::

您需要创建配置文件 `~/.config/starship.toml` 以供 Starship 使用。

```sh
$ mkdir -p ~/.config && touch ~/.config/starship.toml
```

Starship 的所有配置都在此 [TOML](https://github.com/toml-lang/toml) 配置文件中完成：

```toml
# 不用在提示符的开始换行
add_newline = false

# 将提示符标志由“❯”换成“➜”
[character]      # 正在配置的组件名称是“character”
symbol = "➜"     # “symbol”字段被设置为 "➜"

# 禁用 package 组件，它不会在提示符中被显示出来
[package]
disabled = true
```

你可以设置环境变量 `STARSHIP_CONFIG` 来修改 starship 查找配置文件 `starship.toml` 时查找的位置：
```sh
export STARSHIP_CONFIG=~/.starship
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:
```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### 术语

**Module**: A component in the prompt giving information based on contextual information from your OS. For example, the "nodejs" module shows the version of NodeJS that is currently installed on your computer, if your current directory is a NodeJS project.

**Segment**: Smaller sub-components that compose a module. For example, the "symbol" segment in the "nodejs" module contains the character that is shown before the version number (⬢ by default).

Here is the representation of the node module. In the following example, "symbol" and "version" are segments within it. Every module also has a prefix and suffix that are the default terminal color.

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### 样式字符串

Most modules in starship allow you to configure their display styles. This is done with an entry (usually called `style`) which is a string specifying the configuration. Here are some examples of style strings along with what they do. For details on the full syntax, consult the [advanced config guide](/advanced-config/).

- `"fg:green bg:blue"` 在蓝色背景上显示绿色文本
- `"bg:blue fg:bright-green"` 在蓝色背景上显示亮绿色文本
- `"bold fg:27"` 设置粗体字，用 27 号 [ANSI 标准色](https://i.stack.imgur.com/KTSQa.png)
- `"underline bg:#bf5700"` 在深橙色背景上显示带下划线文本
- `"bold italic fg:purple"` 设置文本为粗体、意大利体，颜色为紫色
- `""` 显式禁用所有样式

Note that what styling looks like will be controlled by your terminal emulator. For example, some terminal emulators will brighten the colors instead of bolding text, and some color themes use the same values for the normal and bright colors. Also, to get italic text, your terminal must support italics.

## 提示符

This is the list of prompt-wide configuration options.

### 配置项

| 字段             | 默认值                          | 描述                         |
| -------------- | ---------------------------- | -------------------------- |
| `add_newline`  | `true`                       | 在提示符与提示信息间换行。              |
| `prompt_order` | [见下文](#default-prompt-order) | 配置各组件在提示信息中出现的顺序。          |
| `scan_timeout` | `30`                         | Starship 扫描文件的超时时间（单位：毫秒）。 |

### 示例

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
add_newline = false
# Overwrite a default_prompt_order and  use custom prompt_order
prompt_order=["rust","line_break","package","line_break","character"]
# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10
```

### 默认的组件顺序

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
    "ocaml",
    "php",
    "purescript",
    "python",
    "ruby",
    "rust",
    "terraform",
    "zig",
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

The `aws` module shows the current AWS region and profile. This is based on `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env var with `~/.aws/config` file.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var.

### 配置项

| 字段                | 默认值             | 描述                                            |
| ----------------- | --------------- | --------------------------------------------- |
| `symbol`          | `"☁️ "`         | 这个字段的内容会显示在当前 AWS 配置信息之前。                     |
| `displayed_items` | `all`           | 选择要显示的字段。 可能的值有： [`all`, `profile`, `region`] |
| `region_aliases`  |                 | 地区缩写列表，用来显示在 AWS 主机名之后。                       |
| `style`           | `"bold yellow"` | 此组件的样式。                                       |
| `disabled`        | `false`         | 禁用 `AWS` 组件。                                  |

### 示例

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

The `battery` module shows how charged the device's battery is and its current charging status. The module is only visible when the device's battery is below 10%.

### 配置项

| 字段                   | 默认值                     | 描述               |
| -------------------- | ----------------------- | ---------------- |
| `full_symbol`        | `"•"`                   | 显示于电池充满时。        |
| `charging_symbol`    | `"⇡"`                   | 显示于正在充电时。        |
| `discharging_symbol` | `"⇣"`                   | 显示于电池放电时。        |
| `display`            | [见下文](#battery-display) | 电量显示阈值和样式。       |
| `disabled`           | `false`                 | 禁用 `battery` 组件。 |

<details>
<summary>There are also options for some uncommon battery states.</summary>

| 字段               | 描述         |
| ---------------- | ---------- |
| `unknown_symbol` | 显示于电池状态未知时 |
| `empty_symbol`   | 显示于电池状态为空时 |

Note: Battery indicator will be hidden if the status is `unknown` or `empty` unless you specify the option in the config.

</details>

### 示例

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### Battery 组件的显示

The `display` configuration option is used to define when the battery indicator should be shown (threshold) and what it looks like (style). If no `display` is provided. The default is as shown:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### 配置项

The `display` option is an array of the following table.

| 字段          | 描述               |
| ----------- | ---------------- |
| `threshold` | 当前显示设置的电量上限（见示例） |
| `style`     | 若组件被显示，则使用此样式    |

#### 示例

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

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways: by changing color (red/green) or by changing its shape (❯/✖). The latter will only be done if `use_symbol_for_status` is set to `true`.

### 配置项

| 字段                      | 默认值            | 描述                                            |
| ----------------------- | -------------- | --------------------------------------------- |
| `symbol`                | `"❯"`          | 提示符中在输入文本之前显示的符号。                             |
| `error_symbol`          | `"✖"`          | 如果上一条命令失败，提示符中在输入文本之前显示的符号。                   |
| `use_symbol_for_status` | `false`        | 指示是否改变显示符号来指出错误状态。                            |
| `vicmd_symbol`          | `"❮"`          | 如果 shell 处于 vim 的 normal 模式，提示符中在输入文本之前显示的符号。 |
| `style_success`         | `"bold green"` | 上次命令成功时使用的样式。                                 |
| `style_failure`         | `"bold red"`   | 上次命令失败时使用的样式。                                 |
| `disabled`              | `false`        | 禁用 `character` 组件。                            |

### 示例

```toml
# ~/.config/starship.toml

[character]
symbol = "➜"
error_symbol = "✗"
use_symbol_for_status = true
```

## Command Duration

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### 配置项

| 字段                  | 默认值             | 描述                     |
| ------------------- | --------------- | ---------------------- |
| `min_time`          | `2_000`         | 显示此组件所需的最短执行时长（单位：毫秒）。 |
| `show_milliseconds` | `false`         | 除了秒数外在执行时长中额外显示毫秒。     |
| `prefix`            | `took`          | 直接在执行时长前显示的前缀。         |
| `style`             | `"bold yellow"` | 此组件的样式。                |
| `disabled`          | `false`         | 禁用 `cmd_duration` 组件。  |

### 示例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
prefix = "underwent "
```

## Conda

The `conda` module shows the current conda environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### 配置项

| 字段                  | 默认值            | 描述                                                                                                               |
| ------------------- | -------------- | ---------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`            | 如果这个 conda 环境是通过 `conda create -p [path]` 创建的，环境路径的目录深度应该被截断到此数量。 `0` 表示不用截断。 另请参阅 [`directory`](#directory) 组件。 |
| `symbol`            | `"C "`         | 在环境名之前显示的符号。                                                                                                     |
| `style`             | `"bold green"` | 此组件的样式。                                                                                                          |
| `disabled`          | `false`        | 禁用 `conda` 组件。                                                                                                   |

### 示例

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. 此组件将在符合以下任意条件时显示：

- 当前目录包含一个 `shard.yml` 文件
- The current directory contains a `.cr` file

### 配置项

| 字段         | 默认值          | 描述                                                        |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"🔮 "`       | The symbol used before displaying the version of crystal. |
| `style`    | `"bold red"` | 此组件的样式。                                                   |
| `disabled` | `false`      | Disables the `crystal` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[crystal]
symbol = "✨ "
style = "bold blue"
```

## Directory

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### 配置项

| 字段                  | 默认值           | 描述                       |
| ------------------- | ------------- | ------------------------ |
| `truncation_length` | `3`           | 当前目录路径被截断后最多保留的父目录数量。    |
| `truncate_to_repo`  | `true`        | 是否只截断到您当前处于的 git 仓库根目录下。 |
| `prefix`            | `"in "`       | 直接在显示路径前显示的前缀。           |
| `style`             | `"bold cyan"` | 此组件的样式。                  |
| `disabled`          | `false`       | 禁用 `directory` 组件。       |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| 字段                          | 默认值    | 描述                                    |
| --------------------------- | ------ | ------------------------------------- |
| `fish_style_pwd_dir_length` | `0`    | 使用 fish shell 当前目录路径逻辑时每个省略目录名使用的字符数。 |
| `use_logical_path`          | `true` | 显示由 shell 提供的逻辑路径（`PWD`）而不是 OS 提供的路径。 |

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### 示例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### 配置项

| 字段                | 默认值           | 描述                                                                                      |
| ----------------- | ------------- | --------------------------------------------------------------------------------------- |
| `symbol`          | `"🐳 "`        | The symbol used before displaying the Docker context .                                  |
| `only_with_files` | `false`       | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `style`           | `"bold blue"` | 此组件的样式。                                                                                 |
| `disabled`        | `true`        | Disables the `docker_context` module.                                                   |

### 示例

```toml
# ~/.config/starship.toml

[docker_context]
symbol = "🐋 "
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one of the following files are present in the current directory: `global.json`, `project.json`, `*.sln`, `*.csproj`, `*.fsproj`, `*.xproj`. You'll also need the .NET Core command-line tools installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

### 配置项

| 字段          | 默认值           | 描述                             |
| ----------- | ------------- | ------------------------------ |
| `symbol`    | `"•NET "`     | 这个字段的内容会显示在当前 .NET 版本之前。       |
| `heuristic` | `true`        | 使用更快的版本探测机制以保证 starship 的运行速度。 |
| `style`     | `"bold blue"` | 此组件的样式。                        |
| `disabled`  | `false`       | 禁用 `dotnet` 组件。                |

### 示例

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. 此组件将在符合以下任意条件时显示：

- 当前目录包含一个 `mix.exs` 文件.

### 配置项

| 字段         | 默认值             | 描述                                                              |
| ---------- | --------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`          | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"` | 此组件的样式。                                                         |
| `disabled` | `false`         | Disables the `elixir` module.                                   |

### 示例

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. 此组件将在符合以下任意条件时显示：

- 当前目录包含一个 `elm.json` 文件
- 当前目录包含 `elm-package.json` 文件
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### 配置项

| 字段         | 默认值           | 描述                                                    |
| ---------- | ------------- | ----------------------------------------------------- |
| `symbol`   | `"🌳 "`        | The symbol used before displaying the version of Elm. |
| `style`    | `"bold cyan"` | 此组件的样式。                                               |
| `disabled` | `false`       | Disables the `elm` module.                            |


### 示例

```toml
# ~/.config/starship.toml

[elm]
symbol = " "
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- 设置的 `variable` 是一个已存在的环境变量
- 未定义 `variable`，但定义了 `default`

### 配置项

| 字段         | 默认值                   | 描述                  |
| ---------- | --------------------- | ------------------- |
| `symbol`   |                       | 这个字段的内容会显示在环境变量值之前。 |
| `variable` |                       | 要显示的环境变量。           |
| `default`  |                       | 所选变量未定义时显示的默认值。     |
| `prefix`   | `""`                  | 直接在显示环境变量值前显示的前缀。   |
| `suffix`   | `""`                  | 直接在显示环境变量值后显示的后缀。   |
| `style`    | `"dimmed bold black"` | 此组件的样式。             |
| `disabled` | `false`               | 禁用 `env_var` 组件。    |

### 示例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. 此组件将在符合以下任意条件时显示：

- 当前目录包含一个 `rebar.config` 文件.
- 当前目录包含一个 `erlang.mk` 文件.

### 配置项

| 字段         | 默认值        | 描述                                                       |
| ---------- | ---------- | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`     | The symbol used before displaying the version of Erlang. |
| `style`    | `bold red` | The style for this module.                               |
| `disabled` | `false`    | Disables the `erlang` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[erlang]
symbol = "e "
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### 配置项

| 字段                  | 默认值             | 描述                                    |
| ------------------- | --------------- | ------------------------------------- |
| `symbol`            | `" "`          | 该字段的内容显示于当前仓库活动分支名之前。                 |
| `truncation_length` | `2^63 - 1`      | 将显示的分支名截断到该数量的字素（graphemes）           |
| `truncation_symbol` | `"…"`           | 此字段的内容用来表示分支名称被截断。 您可以使用 "" 以不显示任何符号。 |
| `style`             | `"bold purple"` | 此组件的样式。                               |
| `disabled`          | `false`         | 禁用 `git_branch` 组件。                   |

### 示例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the current commit hash of the repo in your current directory.

### 配置项

| 字段                   | 默认值            | 描述                                                    |
| -------------------- | -------------- | ----------------------------------------------------- |
| `commit_hash_length` | `7`            | 显示的 git 提交哈希值的长度。                                     |
| `prefix`             | `"("`          | 直接在 git 提交哈希值前显示的前缀。                                  |
| `suffix`             | `")"`          | 直接在 git 提交哈希值后显示的后缀。                                  |
| `style`              | `"bold green"` | 此组件的样式。                                               |
| `only_detached`      | `true`         | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`        | 禁用 `git_commit` 组件。                                   |

### 示例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### 配置项

| 字段                 | 默认值                | 描述                                                    |
| ------------------ | ------------------ | ----------------------------------------------------- |
| `rebase`           | `"REBASING"`       | `rebase` 时显示的文本。                                      |
| `merge`            | `"MERGING"`        | `merge` 时显示的文本。                                       |
| `revert`           | `"REVERTING"`      | `revert` 时显示的文本。                                      |
| `cherry_pick`      | `"CHERRY-PICKING"` | `cherry-pick` 时显示的文本。                                 |
| `bisect`           | `"BISECTING"`      | `bisect` 时显示的文本。                                      |
| `am`               | `"AM"`             | 正在执行 `apply-mailbox`（`git am`）时显示的文本。                 |
| `am_or_rebase`     | `"AM/REBASE"`      | 当无法分辨正在执行的是 `apply-mailbox` 还是 `rebase` 时显示的文本。       |
| `progress_divider` | `"/"`              | 将当前进度与总进度分开的符号或文本。 （例如，设置为 `" of "` 时效果是 `"3 of 10"`） |
| `style`            | `"bold yellow"`    | 此组件的样式。                                               |
| `disabled`         | `false`            | 禁用 `git_state` 模块                                     |

### 示例

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### 配置项

| 字段                 | 默认值                       | 描述                           |
| ------------------ | ------------------------- | ---------------------------- |
| `conflicted`       | `"="`                     | 这个分支有合并冲突。                   |
| `conflicted_count` | [见下文](#git-status-counts) | 显示冲突数量，设置冲突数量的显示样式。          |
| `ahead`            | `"⇡"`                     | 这个分支领先于正在跟踪的分支。              |
| `behind`           | `"⇣"`                     | 这个分支落后于正在跟踪的分支。              |
| `diverged`         | `"⇕"`                     | 这个分支与正在跟踪的分支有分歧。             |
| `untracked`        | `"?"`                     | 工作目录中有未跟踪的文件。                |
| `untracked_count`  | [见下文](#git-status-counts) | 显示未跟踪文件的数量，设置该数量的显示样式。       |
| `stashed`          | `"$"`                     | 本地 git 仓库中存在一个 stash 快照。     |
| `stashed_count`    | [见下文](#git-status-counts) | 显示 stash 快照数量，设置快照数量的显示样式。   |
| `modified`         | `"!"`                     | 工作目录中有文件修改。                  |
| `modified_count`   | [见下文](#git-status-counts) | 显示修改文件的数量，设置该数量的显示样式。        |
| `staged`           | `"+"`                     | 一个新文件被添加到了暂存区（staging area）。 |
| `staged_count`     | [见下文](#git-status-counts) | 显示暂存区中文件数量，设置该数量的显示样式。       |
| `renamed`          | `"»"`                     | 一个重命名的文件被添加到了暂存区。            |
| `renamed_count`    | [见下文](#git-status-counts) | 显示重命名文件的数量，设置该数量的显示样式。       |
| `deleted`          | `"✘"`                     | 一个文件的删除记录被添加到了暂存区。           |
| `deleted_count`    | [见下文](#git-status-counts) | 显示文件删除记录的数量，设置该数量的显示样式。      |
| `show_sync_count`  | `false`                   | 显示领先/落后正在跟踪的分支的提交数。          |
| `prefix`           | `[`                       | 直接在 git 状态前显示的前缀。            |
| `suffix`           | `]`                       | 直接在 git 状态后显示的后缀。            |
| `style`            | `"bold red"`              | 此组件的样式。                      |
| `disabled`         | `false`                   | 禁用 `git_status` 组件。          |

#### Git Status 中的计数值

| 字段        | 默认值     | 描述                              |
| --------- | ------- | ------------------------------- |
| `enabled` | `false` | 显示相应的文件数量                       |
| `style`   |         | 可选字段，使计数值的显示风格不同于 git_status 组件 |

### 示例

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

The `golang` module shows the currently installed version of Golang. 此组件将在符合以下任意条件之一时显示：

- 当前目录包含 `go.mod` 文件
- 当前目录包含 `go.sum` 文件
- 当前目录包含 `glide.yaml` 文件
- 当前目录包含 `Gopkg.yml` 文件
- 当前目录包含 `Gopkg.lock` 文件
- The current directory contains a `.go-version` file
- 当前目录包含 `Godeps` 目录
- 当前目录包含一个使用 `.go` 扩展名的文件

### 配置项

| 字段         | 默认值           | 描述                         |
| ---------- | ------------- | -------------------------- |
| `symbol`   | `"🐹 "`        | 这个字段的内容会显示在当前 Golang 版本之前。 |
| `style`    | `"bold cyan"` | 此组件的样式。                    |
| `disabled` | `false`       | 禁用 `golang` 组件。            |

### 示例

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```
## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version. 此组件将在符合以下任意条件时显示：

- 当前目录包含 `stack.yaml` 文件

### 配置项

| 字段         | 默认值          | 描述                          |
| ---------- | ------------ | --------------------------- |
| `symbol`   | `"λ "`       | 这个字段的内容会显示在当前 Haskell 版本之前。 |
| `style`    | `"bold red"` | 此组件的样式。                     |
| `disabled` | `false`      | 禁用 `haskell` 组件。            |


### 示例

```toml
# ~/.config/starship.toml

[haskell]
symbol = " "
```

## Hostname

The `hostname` module shows the system hostname.

### 配置项

| 字段         | 默认值                   | 描述                                                                 |
| ---------- | --------------------- | ------------------------------------------------------------------ |
| `ssh_only` | `true`                | 仅在连接到 SSH 会话时显示主机名。                                                |
| `prefix`   | `""`                  | 直接在主机名前显示的前缀。                                                      |
| `suffix`   | `""`                  | 直接在主机名后显示的后缀。                                                      |
| `trim_at`  | `"."`                 | 当主机名过长被截断时，会截断成第一次匹配该字符串之前的主机名。 `"."` 会让主机名截断到第一个点处。 `""` 会禁用任何截断。 |
| `style`    | `"bold dimmed green"` | 此组件的样式。                                                            |
| `disabled` | `false`               | 禁用 `hostname` 组件。                                                  |

### 示例

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

The `java` module shows the currently installed version of Java. 此组件将在符合以下任意条件时显示：

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- 当前目录包含一个扩展名为 `.java`，`.class`，`.gradle` 或 `.jar` 的文件

### 配置项

| 字段         | 默认值            | 描述                       |
| ---------- | -------------- | ------------------------ |
| `symbol`   | `"☕ "`         | 这个字段的内容会显示在当前 Java 版本之前。 |
| `style`    | `"dimmed red"` | 此组件的样式。                  |
| `disabled` | `false`        | 禁用 `java` 组件。            |

### 示例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### 配置项

| 字段          | 默认值           | 描述                   |
| ----------- | ------------- | -------------------- |
| `symbol`    | `"✦"`         | 这个字段的内容会显示在当前作业数量之前。 |
| `threshold` | `1`           | 如果超过此字段的值，显示任务数量。    |
| `style`     | `"bold blue"` | 此组件的样式。              |
| `disabled`  | `false`       | 禁用 `jobs` 组件。        |

### 示例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. 此组件将在符合以下任意条件时显示：

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### 配置项

| 字段         | 默认值             | 描述                                                      |
| ---------- | --------------- | ------------------------------------------------------- |
| `symbol`   | `"ஃ "`          | The symbol used before displaying the version of Julia. |
| `style`    | `"bold purple"` | 此组件的样式。                                                 |
| `disabled` | `false`         | Disables the `julia` module.                            |

### 示例

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

### 配置项

| 字段                | 默认值           | 描述                                  |
| ----------------- | ------------- | ----------------------------------- |
| `symbol`          | `"☸ "`        | 这个字段的内容会显示在当前集群信息之前。                |
| `context_aliases` |               | Table of context aliases to display |
| `style`           | `"bold blue"` | 此组件的样式。                             |
| `disabled`        | `true`        | 禁用 `kubernetes` 组件。                 |

### 示例

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dimmed green"
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Line Break

The `line_break` module separates the prompt into two lines.

### 配置项

| 字段         | 默认值     | 描述                          |
| ---------- | ------- | --------------------------- |
| `disabled` | `false` | 禁用 `line_break` 组件，使提示成为单行。 |

### 示例

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

### 配置项

| 字段                | 默认值                   | 描述                         |
| ----------------- | --------------------- | -------------------------- |
| `show_percentage` | `false`               | 用可用内存的百分比来显示内存使用情况。        |
| `show_swap`       | `true`                | 如果总交换区使用量为非零，则显示交换区使用情况。   |
| `threshold`       | `75`                  | 隐藏内存使用情况，除非它超过这个百分比。       |
| `symbol`          | `"🐏 "`                | 这个字段的内容会显示在当前内存使用情况之前。     |
| `separator`       | `" | "`               | 此字段所设置的符号或文本会分隔内存和交换区使用情况。 |
| `style`           | `"bold dimmed white"` | 此组件的样式。                    |
| `disabled`        | `true`                | 禁用 `memory_usage` 模块       |

### 示例

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

### 配置项

| 字段                  | 默认值             | 描述                              |
| ------------------- | --------------- | ------------------------------- |
| `symbol`            | `" "`          | 该字段的内容显示于当前仓库的 hg 书签或活动分支名之前。   |
| `truncation_length` | `2^63 - 1`      | 将显示的 hg 分支名截断到该数量的字素（graphemes） |
| `truncation_symbol` | `"…"`           | 此字段的内容用来表示分支名称被截断。              |
| `style`             | `"bold purple"` | 此组件的样式。                         |
| `disabled`          | `true`          | 禁用 `hg_branch` 组件。              |

### 示例

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### 配置项

| 字段           | 默认值           | 描述                                                |
| ------------ | ------------- | ------------------------------------------------- |
| `use_name`   | `false`       | 显示 nix-shell 的名称。                                 |
| `impure_msg` | `"impure"`    | 自定义“impure”消息。                                    |
| `pure_msg`   | `"pure"`      | 自定义“pure”消息。                                      |
| `symbol`     | `"❄️  "`      | The symbol used before displaying the shell name. |
| `style`      | `"bold blue"` | 此组件的样式。                                           |
| `disabled`   | `false`       | 禁用 `nix_shell` 组件。                                |

### 示例

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

The `nodejs` module shows the currently installed version of NodeJS. 此组件将在符合以下任意条件时显示：

- 当前目录包含 `package.json` 文件
- The current directory contains a `.node-version` file
- 当前目录包含 `node_modules` 目录
- 当前目录包含一个使用 `.js` 扩展名的文件

### 配置项

| 字段         | 默认值            | 描述                         |
| ---------- | -------------- | -------------------------- |
| `symbol`   | `"⬢ "`         | 这个字段的内容会显示在当前 NodeJS 版本之前。 |
| `style`    | `"bold green"` | 此组件的样式。                    |
| `disabled` | `false`        | 禁用 `nodejs` 组件。            |

### 示例

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia` and `mix` packages.

- **npm** —— `npm` 软件包版本从当前目录下的 `package.json` 中得到
- **cargo** —— `cargo` 软件包的版本从当前目录下的 `Cargo.toml` 中得到
- **poetry** —— `poetry` 软件包版本从当前目录下的 `pyproject.toml` 中得到
- **composer** —— `composer` 软件包版本从当前目录下的 `composer.json` 中得到
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present

> ⚠ 此组件显示的是源代码在当前目录中的软件包的版本，而不是包管理器的版本。

### 配置项

| 字段                | 默认值          | 描述                                                        |
| ----------------- | ------------ | --------------------------------------------------------- |
| `symbol`          | `"📦 "`       | 这个字段的内容会显示在当前软件包版本之前。                                     |
| `style`           | `"bold 208"` | 此组件的样式。                                                   |
| `display_private` | `false`      | Enable displaying version for packages marked as private. |
| `disabled`        | `false`      | 禁用 `package` 组件。                                          |

### 示例

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. 此组件将在符合以下任意条件时显示：

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### 配置项

| 字段         | 默认值             | 描述                                                      |
| ---------- | --------------- | ------------------------------------------------------- |
| `symbol`   | `"🐫 "`          | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"` | 此组件的样式。                                                 |
| `disabled` | `false`         | Disables the `ocaml` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[ocaml]
symbol = "🐪 "
```

## PHP

The `php` module shows the currently installed version of PHP. 此组件将在符合以下任意条件时显示：

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### 配置项

| 字段         | 默认值          | 描述                                                    |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold 147"` | 此组件的样式。                                               |
| `disabled` | `false`      | Disables the `php` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

此组件将在符合以下任意条件时显示：

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- The current directory contains a `setup.py` file
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### 配置项

| 字段                   | 默认值             | 描述                                                                          |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
| `scan_for_pyfiles`   | `true`          | If false, Python files in the current directory will not show this module.  |
| `style`              | `"bold yellow"` | 此组件的样式。                                                                     |
| `disabled`           | `false`         | Disables the `python` module.                                               |

### 示例

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

The `ruby` module shows the currently installed version of Ruby. 此组件将在符合以下任意条件时显示：

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### 配置项

| 字段         | 默认值          | 描述                                                     |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"💎 "`       | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold red"` | 此组件的样式。                                                |
| `disabled` | `false`      | Disables the `ruby` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust. 此组件将在符合以下任意条件时显示：

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### 配置项

| 字段         | 默认值          | 描述                                                     |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`       | The symbol used before displaying the version of Rust. |
| `style`    | `"bold red"` | 此组件的样式。                                                |
| `disabled` | `false`      | Disables the `rust` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### 配置项

| 字段         | 默认值                  | 描述                                               |
| ---------- | -------------------- | ------------------------------------------------ |
| `label`    | `""`                 | Prefix before the image name display.            |
| `prefix`   | `"["`                | Prefix to display immediately before image name. |
| `suffix`   | `"]"`                | Suffix to display immediately after image name.  |
| `symbol`   | `""`                 | The symbol used before the image name.           |
| `style`    | `"bold dimmed blue"` | 此组件的样式。                                          |
| `disabled` | `false`              | Disables the `singularity` module.               |

### 示例

```toml
# ~/.config/starship.toml

[singularity]
symbol = "📦 "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. 此组件将在符合以下任意条件时显示：

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### 配置项

| 字段             | 默认值          | 描述                                                          |
| -------------- | ------------ | ----------------------------------------------------------- |
| `symbol`       | `"💠 "`       | The symbol used before displaying the terraform workspace.  |
| `show_version` | `false`      | Shows the terraform version. Very slow on large workspaces. |
| `style`        | `"bold 105"` | 此组件的样式。                                                     |
| `disabled`     | `false`      | Disables the `terraform` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### 配置项

| 字段                | 默认值             | 描述                                                                                                                  |
| ----------------- | --------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`         | Enables 12 hour formatting                                                                                          |
| `format`          | see below       | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `"bold yellow"` | The style for the module time                                                                                       |
| `utc_time_offset` | `"local"`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |
| `disabled`        | `true`          | Disables the `time` module.                                                                                         |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `use_12hr` setting.

### 示例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = "-5"
```

## Username

The `username` module shows active user's username. 此组件将在符合以下任意条件时显示：

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

### 配置项

| 字段            | 默认值             | 描述                                    |
| ------------- | --------------- | ------------------------------------- |
| `style_root`  | `"bold red"`    | The style used when the user is root. |
| `style_user`  | `"bold yellow"` | The style used for non-root users.    |
| `show_always` | `false`         | Always shows the `username` module.   |
| `disabled`    | `false`         | Disables the `username` module.       |

### 示例

```toml
# ~/.config/starship.toml

[username]
disabled = true
```


## Zig

The `zig` module shows the currently installed version of Zig. 此组件将在符合以下任意条件时显示：

- The current directory contains a `.zig` file

### 配置项

| 字段         | 默认值             | 描述                                                    |
| ---------- | --------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`          | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"` | 此组件的样式。                                               |
| `disabled` | `false`         | Disables the `zig` module.                            |

### 示例

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

### 配置项

| 字段            | 默认值                       | 描述                                                                                                                         |
| ------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                           | The command whose output should be printed.                                                                                |
| `when`        |                           | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                           | The path to the shell to use to execute the command. If unset, it will fallback to STARSHIP_SHELL and then to "sh".        |
| `description` | `"<custom module>"` | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                      | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                      | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                      | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                      | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`            | 此组件的样式。                                                                                                                    |
| `prefix`      | `""`                      | Prefix to display immediately before the command output.                                                                   |
| `suffix`      | `""`                      | Suffix to display immediately after the command output.                                                                    |
| `disabled`    | `false`                   | Disables this `custom` module.                                                                                             |

### 示例

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
prefix = " transcending "
```

## PureScript

The `purescript` module shows the currently installed version of PureScript version. 此组件将在符合以下任意条件时显示：

- The current directory contains a `spago.dhall` file
- The current directory contains a \*.purs files

### 配置项

| 字段         | 默认值            | 描述                                                           |
| ---------- | -------------- | ------------------------------------------------------------ |
| `symbol`   | `"<=> "` | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"` | 此组件的样式。                                                      |
| `disabled` | `false`        | Disables the `purescript` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[purescript]
symbol = "<=> "
```
