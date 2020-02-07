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

</details>

### 示例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Dotnet

`dotnet` 模块显示与当前目录下使用的 .NET Core SDK 相关联的版本。 如果当前目录已被绑定了一个版本的 SDK，则显示被帮定的版本。 否则此组件将显示最新安装的 SDK 版本。

此组件只会在以下文件之一出现在当前目录中时显示：`global.json`，`project.json`，`*.sln`，`*.csproj`，`*.fsproj`，`*.xproj`。 为了正确使用此组件，您还需要安装 .NET Core 命令行工具。

在内部，此组件使用自己的版本检测机制。 一般来说此组件是直接执行 `dotnet --version` 的两倍快，但当你的 .NET 项目使用了不常见的目录布局时此组件可能显示一个错误的版本。 如果相比于速度您更需要正确的版本号，您可以在组件设置中设置 `heuristic = false` 来禁用该机制。

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

`env_var` 组件显示选定的环境变量的当前值。 此组件只有满足以下条件之一时才会被显示：

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

## Git Branch

`git_branch` 组件显示当前目录的 git 仓库的活动分支。

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

`git_commit` 组件显示当前目录的 git 仓库的当前提交的哈希值。

::: tip

此组件默认被禁用。 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

:::

### 配置项

| 字段                   | 默认值            | 描述                   |
| -------------------- | -------------- | -------------------- |
| `commit_hash_length` | `7`            | 显示的 git 提交哈希值的长度。    |
| `prefix`             | `"("`          | 直接在 git 提交哈希值前显示的前缀。 |
| `suffix`             | `")"`          | 直接在 git 提交哈希值后显示的后缀。 |
| `style`              | `"bold green"` | 此组件的样式。              |
| `disabled`           | `true`         | 禁用 `git_commit` 组件。  |

### 示例

```toml
# ~/.config/starship.toml

[git_commit]
disabled = false
commit_hash_length = 4
```

## Git State

`git_state` 组件会显示当前目录在哪个 git 仓库中，以及正在进行的操作，例如：_REBASING_，_BISECTING_ 等。 进度信息（例如 REBASING 3/10）如果存在则也会被显示。

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

`git_status`组件通过相应的符号显示您当前目录中 git 仓库的状态。

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

`golang` 组件显示当前安装的 Golang 版本。 此组件只有满足以下条件之一时才会被显示：

- 当前目录包含 `go.mod` 文件
- 当前目录包含 `go.sum` 文件
- 当前目录包含 `glide.yaml` 文件
- 当前目录包含 `Gopkg.yml` 文件
- 当前目录包含 `Gopkg.lock` 文件
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

`haskell` 组件显示当前安装的 Haskell Stack 版本。 此组件只有满足以下条件之一时才会被显示：

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

## Mercurial Branch

`hg_branch` 组件显示当前目录的 hg 仓库的活动分支。

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

## Hostname

`hostname` 组件显示系统主机名。

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

## Jobs

`jobs` 组件显示当前正在运行的任务数量。 仅当有后台任务运行时，此组件才会显示。 如果有超过 1 个作业，模块将显示正在运行的作业数量，如果配置了 `threshold` 字段，则使用它作为显示作业数量的下限。

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

## Kubernetes

显示当前的 Kubernetes 上下文名以及，如果有相关设置，则显示来自 kubeconig 文件的命名空间。 命名空间需要在 kubeconfig 文件中设置，这可以通过 `kubectl config set-context starship-cluster --namespace astronaut` 完成。 如果设置了环境变量 `$KUBECONFIG`，此组件将使用该值，否则会使用 `~/.kube/config`。

::: tip

此组件默认被禁用。 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

:::

### 配置项

| 字段         | 默认值           | 描述                   |
| ---------- | ------------- | -------------------- |
| `symbol`   | `"☸ "`        | 这个字段的内容会显示在当前集群信息之前。 |
| `style`    | `"bold blue"` | 此组件的样式。              |
| `disabled` | `true`        | 禁用 `kubernetes` 组件。  |

### 示例

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dim green"
disabled = false
```

## Line Break

`line_break` 组件将提示分隔为两行。

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

## Nix-shell

`nix_shell` 组件显示 nix-shell 环境。 当处于一个 nix-shell 环境中时，此组件会被显示。

### 配置项

| 字段           | 默认值          | 描述                 |
| ------------ | ------------ | ------------------ |
| `use_name`   | `false`      | 显示 nix-shell 的名称。  |
| `impure_msg` | `"impure"`   | 自定义“impure”消息。     |
| `pure_msg`   | `"pure"`     | 自定义“pure”消息。       |
| `style`      | `"bold red"` | 此组件的样式。            |
| `disabled`   | `false`      | 禁用 `nix_shell` 组件。 |

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

`java` 组件显示当前安装的 Java 版本。 此组件只有满足以下条件之一时才会被显示：

- 当前目录包含 `pom.xml`，`build.gradle.kts` 或 `build.sbt` 文件
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

## Memory Usage

`memory_usage` 组件显示当前系统内存和交换区使用情况。

默认情况下，如果系统交换区使用不为 0，则会显示交换区使用情况。

::: tip

此组件默认被禁用。 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

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
show_percentage = true
show_swap = true
threshold = -1
symbol = " "
separator = "/"
style = "bold dimmed green"
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. 此组件只有满足以下条件之一时才会被显示：

- 当前目录包含 `shard.yml` 文件
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

`nodejs` 组件显示当前安装的 NodeJS 版本。 此组件将在符合以下任意条件之一时显示：

- 当前目录包含 `package.json` 文件
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

当前目录是软件包的代码仓库时，将显示 `package` 组件，并显示软件包当前版本。 此组件目前支持`npm`，`cargo` 和 `poetry` 软件包。

- **npm** —— `npm` 软件包版本从当前目录下的 `package.json` 中得到
- **cargo** —— `cargo` 软件包的版本从当前目录下的 `Cargo.toml` 中得到
- **poetry** —— `poetry` 软件包版本从当前目录下的 `pyproject.toml` 中得到
- **composer** —— `composer` 软件包版本从当前目录下的 `composer.json` 中得到

> ⚠ 此组件显示的是源代码在当前目录中的软件包的版本，而不是包管理器的版本。

### 配置项

| 字段         | 默认值          | 描述                    |
| ---------- | ------------ | --------------------- |
| `symbol`   | `"📦 "`       | 这个字段的内容会显示在当前软件包版本之前。 |
| `style`    | `"bold red"` | 此组件的样式。               |
| `disabled` | `false`      | 禁用 `package` 组件。      |

### 示例

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## PHP

`php` 组件显示当前安装的 PHP 版本。 此组件只有满足以下条件之一时才会被显示：

- 当前目录包含一个 `composer.json` 文件
- 当前目录包含一个 `.php` 文件

### 配置项

| 字段         | 默认值          | 描述                      |
| ---------- | ------------ | ----------------------- |
| `symbol`   | `"🐘 "`       | 这个字段的内容会显示在当前 PHP 版本之前。 |
| `style`    | `"bold red"` | 此组件的样式。                 |
| `disabled` | `false`      | 禁用 `php` 组件。            |

### 示例

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

`python` 组件显示当前安装的 Python 版本。

如果 `pyenv_version_name` 设置为 `true`，则将显示 pyenv 版本名称。

否则，它将显示来自 `python --version` 的版本号，并显示当前的 Python 虚拟环境，如果激活了的话。

此组件只有满足以下条件之一时才会被显示：

- 当前目录包含 `.python-version` 文件
- 当前目录包含 `requirements.txt` 文件
- 当前目录包含 `pyproject.toml` 文件
- 当前目录包含一个使用 `.py` 扩展名的文件
- 当前目录包含 `Pipfile` 文件
- 当前目录包含一个 `tox.ini` 文件
- 当前处于一个活跃的 python 虚拟环境中

### 配置项

| 字段                   | 默认值             | 描述                                        |
| -------------------- | --------------- | ----------------------------------------- |
| `symbol`             | `"🐍 "`          | 这个字段的内容会显示在当前 Python 版本之前。                |
| `pyenv_version_name` | `false`         | 使用 pyenv 获取 Python 版本                     |
| `pyenv_prefix`       | `"pyenv "`      | 在 pyenv 版本前显示的前缀（默认显示 `pyenv MY_VERSION`） |
| `style`              | `"bold yellow"` | 此组件的样式。                                   |
| `disabled`           | `false`         | 禁用 `python` 组件。                           |

### 示例

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

`ruby` 组件显示当前安装的 Ruby 版本。 此组件只有满足以下条件之一时才会被显示：

- 当前目录包含 `Gemfile` 文件
- 当前目录包含 `.rb` 文件

### 配置项

| 字段         | 默认值          | 描述                                                     |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"💎 "`       | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold red"` | 此组件的样式。                                                |
| `disabled` | `false`      | 禁用 `ruby` 组件。                                          |

### 示例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

`rust` 组件显示当前安装的 Rust 版本。 此组件只有满足以下条件之一时才会被显示：

- 当前目录包含 `Cargo.toml` 文件
- 当前目录包含一个使用 `.rs` 扩展名的文件

### 配置项

| 字段         | 默认值          | 描述                       |
| ---------- | ------------ | ------------------------ |
| `symbol`   | `"🦀 "`       | 这个字段的内容会显示在当前 Rust 版本之前。 |
| `style`    | `"bold red"` | 此组件的样式。                  |
| `disabled` | `false`      | 禁用 `rust` 组件。            |

### 示例

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Terraform

`terraform` 组件显示当前选定的 terraform 工作区和版本。 默认情况下不会显示 terraform 版本，因为当使用大量插件时，当前版本 terraform 查询版本号很慢。 此组件只有满足以下条件之一时才会被显示：

- 当前目录包含 `.terraform` 目录
- 当前目录包含一个使用 `.tf` 扩展名的文件

### 配置项

| 字段             | 默认值          | 描述                               |
| -------------- | ------------ | -------------------------------- |
| `symbol`       | `"💠 "`       | 这个字段的内容会显示在当前 terraform 工作区之前。   |
| `show_version` | `false`      | 显示 terraform 版本信息。 在大型工作空间中非常缓慢。 |
| `style`        | `"bold 105"` | 此组件的样式。                          |
| `disabled`     | `false`      | 禁用 `terraform` 组件。               |

### 示例

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## Time

`time` 组件显示当前的 **本地** 时间。 `format` 字段值会提供给 [`chrono`](https://crates.io/crates/chrono) crate 用来控制时间显示方式。 请参阅 [chrono strftime 文档](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) 以了解可用格式选项。

::: tip

此组件默认被禁用。 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

:::

### 配置项

| 字段                | 默认值           | 描述                                                                                        |
| ----------------- | ------------- | ----------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`       | 启用 12 小时格式                                                                                |
| `format`          | 见下文解释         | 用来格式化时间显示的 [chrono 格式字符串](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) |
| `style`           | `bold yellow` | 显示时间的样式。                                                                                  |
| `utc_time_offset` | `local`       | 设置所用 UTC 偏移量。 范围是 -24 < x < 24。 允许使用浮点数来得到 30/45 分钟的时区偏移。                                 |
| `disabled`        | `true`        | 禁用 `time` 组件。                                                                             |

如果 `use_12hr` 为 `true`，则`format` 默认值为 `"%r"`。 否则，其默认值为 `"%T"`。 手动设置 `format` 将使 `use_12hr` 被忽略。

### 示例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = -5
```

## Username

`username` 组件显示当前活跃的用户名。 此组件只有满足以下条件之一时才会被显示：

- 当前用户是 root
- 当前用户与登录用户不相同
- 用户正通过 SSH 会话连接访问
- 字段 `show_always` 被设置为 true

### 配置项

| 字段            | 默认值             | 描述                  |
| ------------- | --------------- | ------------------- |
| `style_root`  | `"bold red"`    | 当前用户为 root 时使用的样式。  |
| `style_user`  | `"bold yellow"` | 非 root 用户使用的样式。     |
| `show_always` | `false`         | 总是显示 `username` 组件。 |
| `disabled`    | `false`         | 禁用 `username` 组件。   |

### 示例

```toml
# ~/.config/starship.toml

[username]
disabled = true
```
