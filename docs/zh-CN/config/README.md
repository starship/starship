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

### 术语

**组件（Module）**：提示符的组成部分，通过来自系统的上下文信息向用户显示各种信息。 比如“nodejs”组件会在当前目录是一个 NodeJS 项目时显示您当前安装的 NodeJS 版本。

**字段（Segment）**：组成组件的下级单位。 例如，“nodejs”组件中的“symbol”字段包含了在版本号之前显示的字符（默认为 ⬢）。

以下定义了整个 node 组件的显示格式。 在下面这个例子里，“symbol”和“version”是其中的字段。 每一个组件都有一个以终端默认颜色显示的前缀（prefix）和后缀（suffix）。

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### 样式字符串

Starship 中的大多数组件允许您为其设置显示样式。 显示样式可以通过一个字符串字段（一般是 `style`）来设置。 以下的例子给出了一些样式字符串并描述了它们的效果。 样式字符串的完整语法请查阅 [高级配置指南](/advanced-config/)。

- `"fg:green bg:blue"` 在蓝色背景上显示绿色文本
- `"bg:blue fg:bright-green"` 在蓝色背景上显示亮绿色文本
- `"bold fg:27"` 设置粗体字，用 27 号 [ANSI 标准色](https://i.stack.imgur.com/KTSQa.png)
- `"underline bg:#bf5700"` 在深橙色背景上显示带下划线文本
- `"bold italic fg:purple"` 设置文本为粗体、意大利体，颜色为紫色
- `""` 显式禁用所有样式

请注意，最终的显示样式将由您的终端模拟器控制。 例如，有的终端模拟器对于“bold”属性的文本是加亮颜色而不是加粗文字，有的颜色主题对“普通”和“明亮”两种属性的颜色使用相同的颜色值。 此外，要获得意大利体文本（一般设计为斜体），您的终端必须支持意大利体显示。

## 提示符

以下是关于提示符的配置项。

### 配置项

| 字段             | 默认值                          | 描述                         |
| -------------- | ---------------------------- | -------------------------- |
| `add_newline`  | `true`                       | 在提示符与提示信息间换行。              |
| `prompt_order` | [见下文](#default-prompt-order) | 配置各组件在提示信息中出现的顺序。          |
| `scan_timeout` | `30`                         | Starship 扫描文件的超时时间（单位：毫秒）。 |

### 示例

```toml
# ~/.config/starship.toml

# 不用在提示符前换行
add_newline = false
# 使用自定义的组件顺序替换默认组件顺序
prompt_order=["rust","line_break","package","line_break","character"]
# 当 starship 扫描当前目录下的文件和文件夹时，最多使用 10 毫秒
scan_timeout = 10
```

### 默认的组件顺序

如果 `prompt_order` 是空值或未设置，提示信息中组件的显示顺序会设置为 `prompt_order` 的默认值。 默认设置如下：

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
    "package",
    "dotnet",
    "elixir",
    "elm",
    "golang",
    "haskell",
    "java",
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
    "line_break",
    "jobs",
    "battery",
    "time",
    "character",
]
```

## AWS

`aws` 组件显示当前 AWS 主机所在区域与配置信息。 各组件基于 `AWS_REGION`，`AWS_DEFAULT_REGION` 和 `AWS_PROFILE` 环境变量与 `~/.aws/config` 文件。

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

`battery` 组件显示电池充电情况和当前充电状态。 这个组件只会在当前电量低于 10% 时显示。

### 配置项

| 字段                   | 默认值                     | 描述               |
| -------------------- | ----------------------- | ---------------- |
| `full_symbol`        | `"•"`                   | 显示于电池充满时。        |
| `charging_symbol`    | `"⇡"`                   | 显示于正在充电时。        |
| `discharging_symbol` | `"⇣"`                   | 显示于电池放电时。        |
| `display`            | [见下文](#battery-display) | 电量显示阈值和样式。       |
| `disabled`           | `false`                 | 禁用 `battery` 组件。 |

<details>
<summary>也有一些给不常见的电源状态设立的字段。</summary>

| 字段               | 描述         |
| ---------------- | ---------- |
| `unknown_symbol` | 显示于电池状态未知时 |
| `empty_symbol`   | 显示于电池状态为空时 |

注意：如果状态为 `unknown` 或 `empty`，电池指示器将被隐藏，除非您在配置中指定相关选项。

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

`display` 选项用于定义电池指示器的显示阈值（threshold）和显示效果（style）。 如果 `display` 没有设置， 默认配置如下所示：

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### 配置项

`display` 字段的子字段如下：

| 字段          | 描述               |
| ----------- | ---------------- |
| `threshold` | 当前显示设置的电量上限（见示例） |
| `style`     | 若组件被显示，则使用此样式    |

#### 示例

```toml
[[battery.display]]  # 当电量在 0% 到 10% 时以 "bold red" 样式显示
threshold = 10
style = "bold red"

[[battery.display]]  # 当电量在 10% 到 30% 时以 "bold yellow" 样式显示
threshold = 30
style = "bold yellow"

# 当电量在 30% 时以上时，电池指示器组件将不会显示出来

```

## Character

`character` 组件用于在您输入终端的文本旁显示一个字符（通常是一个箭头）。

这个字符可以告诉您最后一个命令是否执行成功。 它可以用两种方式完成这一功能：改变字符颜色（红色/绿色）或者改变其形状（❯/✖）。 后者仅在 `use_symbol_for_status` 设置为 `true` 时才能实现。

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

`cmd_duration` 组件显示上一个命令执行的时间。 此组件只在命令执行时间长于两秒时显示，或者当其 `min_time` 字段被设置时，按此值为执行时间的显示下限。

::: warning 不要在 Bash 里捕获 DEBUG 信号

如果您正在 `bash` 上使用 Starship，在运行 `eval $(starship)` 后，不要捕获 `DEBUG` 信号，否则此组件**将会**坏掉。

:::

需要在自动每一条命令前执行某些操作的 Bash 用户可以使用 [rcaloras 的 bash_preexec 框架](https://github.com/rcaloras/bash-preexec)。 只需要在执行 `eval $(starship init $0)` 前简单地定义 `preexec_functions` 和 `precmd_functions` 两个列表，就可以照常运行了。

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

`conda` 组件在 `$CONDA_DEFAULT_ENV` 被设置时显示当前 conda 环境。

::: tip

此组件没有禁用 conda 自带的提示符修改，您可能需要执行 `conda config --set changeps1 False`。

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

## Directory

`directory` 组件显示当前目录的路径，显示的路径会截断到三个父目录以内。 如果您处于一个 git 仓库中，显示的路径则最多会截断到该仓库的根目录。

当使用 fish 风格的当前目录显示样式时，您会看到基于您的设置的每个上级目录的短名称，而不是隐藏被截断的上级目录。

例如，对于 `~/Dev/Nix/nixpkgs/pkgs`，其中 `nixpkgs` 是 git 仓库根目录，fish 风格相关选项设置为 `1`。 您将会看到 `~/D/N/nixpkgs/pkgs`，而在设置 fish 风格之前，当前路径将显示成 `nixpkgs/pkgs`。

### 配置项

| 字段                  | 默认值           | 描述                       |
| ------------------- | ------------- | ------------------------ |
| `truncation_length` | `3`           | 当前目录路径被截断后最多保留的父目录数量。    |
| `truncate_to_repo`  | `true`        | 是否只截断到您当前处于的 git 仓库根目录下。 |
| `prefix`            | `"in "`       | 直接在显示路径前显示的前缀。           |
| `style`             | `"bold cyan"` | 此组件的样式。                  |
| `disabled`          | `false`       | 禁用 `directory` 组件。       |

<details>
<summary>此组件有几个高级配置选项来控制当前目录路径的显示方式。</summary>

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

## Elm

The `elm` module shows the currently installed version of Elm. 此组件只有满足以下条件之一时才会被显示：

- 当前目录包含 `elm.json` 文件
- 当前目录包含 `elm-package.json` 文件
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

| 字段         | 默认值              | 描述                  |
| ---------- | ---------------- | ------------------- |
| `symbol`   |                  | 这个字段的内容会显示在环境变量值之前。 |
| `variable` |                  | 要显示的环境变量。           |
| `default`  |                  | 所选变量未定义时显示的默认值。     |
| `prefix`   | `""`             | 直接在显示环境变量值前显示的前缀。   |
| `suffix`   | `""`             | 直接在显示环境变量值后显示的后缀。   |
| `style`    | `"dimmed black"` | 此组件的样式。             |
| `disabled` | `false`          | 禁用 `env_var` 组件。    |

### 示例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. 此组件只有满足以下条件之一时才会被显示：

- The current directory contains a `mix.exs` file.

### 配置项

| 字段         | 默认值     | 描述                                                              |
| ---------- | ------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`  | The symbol used before displaying the version of Elixir/Erlang. |
| `disabled` | `false` | Disables the `elixir` module.                                   |

### 示例

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### 配置项

| 字段                  | 默认值             | 描述                                                                                    |
| ------------------- | --------------- | ------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the branch name of the repo in your current directory.         |
| `truncation_length` | `2^63 - 1`      | Truncates a git branch to X graphemes                                                 |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated. You can use "" for no symbol |
| `style`             | `"bold purple"` | 此组件的样式。                                                                               |
| `disabled`          | `false`         | Disables the `git_branch` module.                                                     |

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
| `commit_hash_length` | `7`            | The length of the displayed git commit hash.          |
| `prefix`             | `"("`          | Prefix to display immediately before git commit.      |
| `suffix`             | `")"`          | Suffix to display immediately after git commit.       |
| `style`              | `"bold green"` | 此组件的样式。                                               |
| `only_detached`      | `true`         | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`        | Disables the `git_commit` module.                     |

### 示例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### 配置项

| 字段                 | 默认值                | 描述                                                                                                               |
| ------------------ | ------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `rebase`           | `"REBASING"`       | The text displayed when a `rebase` is in progress.                                                               |
| `merge`            | `"MERGING"`        | The text displayed when a `merge` is in progress.                                                                |
| `revert`           | `"REVERTING"`      | The text displayed when a `revert` is in progress.                                                               |
| `cherry_pick`      | `"CHERRY-PICKING"` | The text displayed when a `cherry-pick` is in progress.                                                          |
| `bisect`           | `"BISECTING"`      | The text displayed when a `bisect` is in progress.                                                               |
| `am`               | `"AM"`             | The text displayed when an `apply-mailbox` (`git am`) is in progress.                                            |
| `am_or_rebase`     | `"AM/REBASE"`      | The text displayed when an ambiguous `apply-mailbox` or `rebase` is in progress.                                 |
| `progress_divider` | `"/"`              | The symbol or text which will separate the current and total progress amounts. (e.g., `" of "`, for `"3 of 10"`) |
| `style`            | `"bold yellow"`    | 此组件的样式。                                                                                                          |
| `disabled`         | `false`            | Disables the `git_state` module.                                                                                 |

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

| 字段                 | 默认值                       | 描述                                                      |
| ------------------ | ------------------------- | ------------------------------------------------------- |
| `conflicted`       | `"="`                     | This branch has merge conflicts.                        |
| `conflicted_count` | [见下文](#git-status-counts) | Show and style the number of conflicts.                 |
| `ahead`            | `"⇡"`                     | This branch is ahead of the branch being tracked.       |
| `behind`           | `"⇣"`                     | This branch is behind of the branch being tracked.      |
| `diverged`         | `"⇕"`                     | This branch has diverged from the branch being tracked. |
| `untracked`        | `"?"`                     | There are untracked files in the working directory.     |
| `untracked_count`  | [见下文](#git-status-counts) | Show and style the number of untracked files.           |
| `stashed`          | `"$"`                     | A stash exists for the local repository.                |
| `stashed_count`    | [见下文](#git-status-counts) | Show and style the number of stashes.                   |
| `modified`         | `"!"`                     | There are file modifications in the working directory.  |
| `modified_count`   | [见下文](#git-status-counts) | Show and style the number of modified files.            |
| `staged`           | `"+"`                     | A new file has been added to the staging area.          |
| `staged_count`     | [见下文](#git-status-counts) | Show and style the number of files staged files.        |
| `renamed`          | `"»"`                     | A renamed file has been added to the staging area.      |
| `renamed_count`    | [见下文](#git-status-counts) | Show and style the number of renamed files.             |
| `deleted`          | `"✘"`                     | A file's deletion has been added to the staging area.   |
| `deleted_count`    | [见下文](#git-status-counts) | Show and style the number of deleted files.             |
| `show_sync_count`  | `false`                   | Show ahead/behind count of the branch being tracked.    |
| `prefix`           | `[`                       | Prefix to display immediately before git status.        |
| `suffix`           | `]`                       | Suffix to display immediately after git status.         |
| `style`            | `"bold red"`              | 此组件的样式。                                                 |
| `disabled`         | `false`                   | Disables the `git_status` module.                       |

#### Git Status 中的计数值

| 字段        | 默认值     | 描述                                                     |
| --------- | ------- | ------------------------------------------------------ |
| `enabled` | `false` | Show the number of files                               |
| `style`   |         | Optionally style the count differently than the module |

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

The `golang` module shows the currently installed version of Golang. 此组件只有满足以下条件之一时才会被显示：

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### 配置项

| 字段         | 默认值           | 描述                                                       |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`        | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | 此组件的样式。                                                  |
| `disabled` | `false`       | Disables the `golang` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```
## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version. 此组件只有满足以下条件之一时才会被显示：

- The current directory contains a `stack.yaml` file

### 配置项

| 字段         | 默认值          | 描述                                                        |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"λ "`       | The symbol used before displaying the version of Haskell. |
| `style`    | `"bold red"` | 此组件的样式。                                                   |
| `disabled` | `false`      | Disables the `haskell` module.                            |


### 示例

```toml
# ~/.config/starship.toml

[haskell]
symbol = " "
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### 配置项

| 字段                  | 默认值             | 描述                                                                                           |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated.                                     |
| `style`             | `"bold purple"` | 此组件的样式。                                                                                      |
| `disabled`          | `true`          | Disables the `hg_branch` module.                                                             |

### 示例

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Hostname

The `hostname` module shows the system hostname.

### 配置项

| 字段         | 默认值                   | 描述                                                                                                                                   |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                | Only show hostname when connected to an SSH session.                                                                                 |
| `prefix`   | `""`                  | Prefix to display immediately before the hostname.                                                                                   |
| `suffix`   | `""`                  | Suffix to display immediately after the hostname.                                                                                    |
| `trim_at`  | `"."`                 | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `style`    | `"bold dimmed green"` | 此组件的样式。                                                                                                                              |
| `disabled` | `false`               | Disables the `hostname` module.                                                                                                      |

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

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### 配置项

| 字段          | 默认值           | 描述                                                    |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦"`         | The symbol used before displaying the number of jobs. |
| `threshold` | `1`           | Show number of jobs if exceeded.                      |
| `style`     | `"bold blue"` | 此组件的样式。                                               |
| `disabled`  | `false`       | Disables the `jobs` module.                           |

### 示例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### 配置项

| 字段         | 默认值           | 描述                                                  |
| ---------- | ------------- | --------------------------------------------------- |
| `symbol`   | `"☸ "`        | The symbol used before displaying the Cluster info. |
| `style`    | `"bold blue"` | 此组件的样式。                                             |
| `disabled` | `true`        | Disables the `kubernetes` module                    |

### 示例

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dimmed green"
disabled = false
```

## Line Break

The `line_break` module separates the prompt into two lines.

### 配置项

| 字段         | 默认值     | 描述                                                                 |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### 示例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### 配置项

| 字段           | 默认值          | 描述                                 |
| ------------ | ------------ | ---------------------------------- |
| `use_name`   | `false`      | Display the name of the nix-shell. |
| `impure_msg` | `"impure"`   | Customize the "impure" msg.        |
| `pure_msg`   | `"pure"`     | Customize the "pure" msg.          |
| `style`      | `"bold red"` | 此组件的样式。                            |
| `disabled`   | `false`      | Disables the `nix_shell` module.   |

### 示例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
```

## Java

The `java` module shows the currently installed version of Java. 此组件只有满足以下条件之一时才会被显示：

- The current directory contains a `pom.xml`, `build.gradle.kts` or `build.sbt` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### 配置项

| 字段         | 默认值            | 描述                                                     |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"☕ "`         | The symbol used before displaying the version of Java. |
| `style`    | `"dimmed red"` | 此组件的样式。                                                |
| `disabled` | `false`        | Disables the `java` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### 配置项

| 字段                | 默认值                   | 描述                                                            |
| ----------------- | --------------------- | ------------------------------------------------------------- |
| `show_percentage` | `false`               | Display memory usage as a percentage of the available memory. |
| `show_swap`       | `true`                | Display swap usage if total swap is non-zero.                 |
| `threshold`       | `75`                  | Hide the memory usage unless it exceeds this percentage.      |
| `symbol`          | `"🐏 "`                | The symbol used before displaying the memory usage.           |
| `separator`       | `" | "`               | The symbol or text that will seperate the ram and swap usage. |
| `style`           | `"bold dimmed white"` | 此组件的样式。                                                       |
| `disabled`        | `true`                | Disables the `memory_usage` module.                           |

### 示例

```toml
# ~/.config/starship.toml

[memory_usage]
show_percentage = true
show_swap = true
threshold = -1
symbol = " "
separator = "/"
style = "bold dimmed green"
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. 此组件只有满足以下条件之一时才会被显示：

- The current directory contains a `shard.yml` file
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

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. 此组件只有满足以下条件之一时才会被显示：

- The current directory contains a `package.json` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js` extension

### 配置项

| 字段         | 默认值            | 描述                                                       |
| ---------- | -------------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`         | The symbol used before displaying the version of NodeJS. |
| `style`    | `"bold green"` | 此组件的样式。                                                  |
| `disabled` | `false`        | Disables the `nodejs` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, and `gradle` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present

> ⚠ 此组件显示的是源代码在当前目录中的软件包的版本，而不是包管理器的版本。

### 配置项

| 字段         | 默认值          | 描述                                                         |
| ---------- | ------------ | ---------------------------------------------------------- |
| `symbol`   | `"📦 "`       | The symbol used before displaying the version the package. |
| `style`    | `"bold red"` | 此组件的样式。                                                    |
| `disabled` | `false`      | Disables the `package` module.                             |

### 示例

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## PHP

The `php` module shows the currently installed version of PHP. 此组件只有满足以下条件之一时才会被显示：

- The current directory contains a `composer.json` file
- The current directory contains a `.php` file

### 配置项

| 字段         | 默认值          | 描述                                                    |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold red"` | 此组件的样式。                                               |
| `disabled` | `false`      | Disables the `php` module.                            |

### 示例

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

The `python` module shows the currently installed version of Python.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name.

Otherwise, it will display the version number from `python --version` and show the current Python virtual environment if one is activated.

此组件只有满足以下条件之一时才会被显示：

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- A virtual environment is currently activated

### 配置项

| 字段                   | 默认值             | 描述                                                                          |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
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

The `ruby` module shows the currently installed version of Ruby. 此组件只有满足以下条件之一时才会被显示：

- The current directory contains a `Gemfile` file
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

The `rust` module shows the currently installed version of Rust. 此组件只有满足以下条件之一时才会被显示：

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

:::

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

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. 此组件只有满足以下条件之一时才会被显示：

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

The `username` module shows active user's username. 此组件只有满足以下条件之一时才会被显示：

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
