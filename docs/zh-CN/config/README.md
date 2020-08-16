# 配置

::: tip

Starship 目前正在开发中。 很多新的配置选项将会在之后的版本中被公开。

:::

您需要创建配置文件 `~/.config/starship.toml` 以供 Starship 使用。

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Starship 的所有配置都在此 [TOML](https://github.com/toml-lang/toml) 配置文件中完成：

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

你可以设置环境变量 `STARSHIP_CONFIG` 来修改 starship 查找配置文件 `starship.toml` 时查找的位置：

```sh
export STARSHIP_CONFIG=~/.starship
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### 术语

**组件（Module）**：提示符的组成部分，通过来自系统的上下文信息向用户显示各种信息。 比如“nodejs”组件会在当前目录是一个 NodeJS 项目时显示您当前安装的 NodeJS 版本。

**Variable**: Smaller sub-components that contains information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of NodeJS.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Format Strings

Format strings are the format that a module prints all its variables with. Most modules have an entry called `format` that configures the display format of the module. You can use texts, variables and text groups in a format string.

#### 字段

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

#### 样式设定

Starship 中的大多数组件允许您为其设置显示样式。 显示样式可以通过一个字符串字段（一般是 `style`）来设置。 以下的例子给出了一些样式字符串并描述了它们的效果。 样式字符串的完整语法请查阅 [高级配置指南](/advanced-config/)。

- `"fg:green bg:blue"` 在蓝色背景上显示绿色文本
- `"bg:blue fg:bright-green"` 在蓝色背景上显示亮绿色文本
- `"bold fg:27"` 设置粗体字，用 27 号 [ANSI 标准色](https://i.stack.imgur.com/KTSQa.png)
- `"underline bg:#bf5700"` 在深橙色背景上显示带下划线文本
- `"bold italic fg:purple"` 设置文本为粗体、意大利体，颜色为紫色
- `""` 显式禁用所有样式

请注意，最终的显示样式将由您的终端模拟器控制。 例如，有的终端模拟器对于“bold”属性的文本是加亮颜色而不是加粗文字，有的颜色主题对“普通”和“明亮”两种属性的颜色使用相同的颜色值。 此外，要获得意大利体文本（一般设计为斜体），您的终端必须支持意大利体显示。

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

## 提示符

以下是关于提示符的配置项。

### 配置项

| Option         | 默认值                           | 描述                                  |
| -------------- | ----------------------------- | ----------------------------------- |
| `format`       | [见下文](#default-prompt-format) | Configure the format of the prompt. |
| `scan_timeout` | `30`                          | Starship 扫描文件的超时时间（单位：毫秒）。          |

### 示例

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

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. 默认设置如下：

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

`aws` 组件显示当前 AWS 主机所在区域与配置信息。 各组件基于 `AWS_REGION`，`AWS_DEFAULT_REGION` 和 `AWS_PROFILE` 环境变量与 `~/.aws/config` 文件。

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var.

### 配置项

| Option           | 默认值                                                  | 描述                         |
| ---------------- | ---------------------------------------------------- | -------------------------- |
| `format`         | `"on [$symbol$profile(\\($region\\))]($style) "` | The format for the module. |
| `symbol`         | `"☁️ "`                                              | 这个字段的内容会显示在当前 AWS 配置信息之前。  |
| `region_aliases` |                                                      | 地区缩写列表，用来显示在 AWS 主机名之后。    |
| `style`          | `"bold yellow"`                                      | 此组件的样式。                    |
| `disabled`       | `false`                                              | 禁用 `AWS` 组件。               |

### Variables

| 字段        | 示例               | 描述                                   |
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

## Battery

`battery` 组件显示电池充电情况和当前充电状态。 这个组件只会在当前电量低于 10% 时显示。

### 配置项

| Option               | 默认值                               | 描述                         |
| -------------------- | --------------------------------- | -------------------------- |
| `full_symbol`        | `"•"`                             | 显示于电池充满时。                  |
| `charging_symbol`    | `"⇡"`                             | 显示于正在充电时。                  |
| `discharging_symbol` | `"⇣"`                             | 显示于电池放电时。                  |
| `format`             | `"[$symbol$percentage]($style) "` | The format for the module. |
| `display`            | [见下文](#battery-display)           | 电量显示阈值和样式。                 |
| `disabled`           | `false`                           | 禁用 `battery` 组件。           |

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

`display` 选项用于定义电池指示器的显示阈值（threshold）和显示效果（style）。 如果 `display` 没有设置， 默认设置如下：

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

这个字符可以告诉您最后一个命令是否执行成功。 It can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change it's shape take a look at [this example](#with-custom-error-shape).

### 配置项

| Option           | 默认值                 | 描述                                                                               |
| ---------------- | ------------------- | -------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | The format string used before the text input.                                    |
| `success_symbol` | `"[❯](bold green)"` | The format string used before the text input if the previous command succeeded.  |
| `error_symbol`   | `"[❯](bold red)"`   | The format string used before the text input if the previous command failed.     |
| `vicmd_symbol`   | `"[❮](bold green)"` | The format string used before the text input if the shell is in vim normal mode. |
| `disabled`       | `false`             | 禁用 `character` 组件。                                                               |

### Variables

| 字段     | 示例 | 描述                                                                    |
| ------ | -- | --------------------------------------------------------------------- |
| symbol |    | A mirror of either `success_symbol`, `error_symbol` or `vicmd_symbol` |

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

### 配置项

| Option     | 默认值                                | 描述                                           |
| ---------- | ---------------------------------- | -------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                   |
| `symbol`   | `"🛆 "`                             | The symbol used before the version of cmake. |
| `style`    | `"bold blue"`                      | 此组件的样式。                                      |
| `disabled` | `false`                            | Disables the `cmake` module.                 |

### Variables

| 字段        | 示例        | 描述                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v3.17.3` | The version of cmake                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

## Command Duration

`cmd_duration` 组件显示上一个命令执行的时间。 此组件只在命令执行时间长于两秒时显示，或者当其 `min_time` 字段被设置时，按此值为执行时间的显示下限。

::: warning 不要在 Bash 里捕获 DEBUG 信号

如果您正在 `bash` 上使用 Starship，在运行 `eval $(starship)` 后，不要捕获 `DEBUG` 信号，否则此组件**将会**坏掉。

:::

需要在自动每一条命令前执行某些操作的 Bash 用户可以使用 [rcaloras 的 bash_preexec 框架](https://github.com/rcaloras/bash-preexec)。 只需要在执行 `eval $(starship init $0)` 前简单地定义 `preexec_functions` 和 `precmd_functions` 两个列表，就可以照常运行了。

### 配置项

| Option              | 默认值                           | 描述                         |
| ------------------- | ----------------------------- | -------------------------- |
| `min_time`          | `2_000`                       | 显示此组件所需的最短执行时长（单位：毫秒）。     |
| `show_milliseconds` | `false`                       | 除了秒数外在执行时长中额外显示毫秒。         |
| `format`            | `"took [$duration]($style) "` | The format for the module. |
| `style`             | `"bold yellow"`               | 此组件的样式。                    |
| `disabled`          | `false`                       | 禁用 `cmd_duration` 组件。      |

### Variables

| 字段        | 示例       | 描述                                      |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | The time it took to execute the command |
| style\* |          | Mirrors the value of option `style`     |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

`conda` 组件在 `$CONDA_DEFAULT_ENV` 被设置时显示当前 conda 环境。

::: tip

此组件没有禁用 conda 自带的提示符修改，您可能需要执行 `conda config --set changeps1 False`。

:::

### 配置项

| Option              | 默认值                                | 描述                                                                                                               |
| ------------------- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                | 如果这个 conda 环境是通过 `conda create -p [path]` 创建的，环境路径的目录深度应该被截断到此数量。 `0` 表示不用截断。 另请参阅 [`directory`](#directory) 组件。 |
| `symbol`            | `"🅒 "`                             | 在环境名之前显示的符号。                                                                                                     |
| `style`             | `"bold green"`                     | 此组件的样式。                                                                                                          |
| `format`            | `"[$symbol$environment]($style) "` | The format for the module.                                                                                       |
| `disabled`          | `false`                            | 禁用 `conda` 组件。                                                                                                   |

### Variables

| 字段          | 示例           | 描述                                   |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current conda environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*   |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. 此组件将在符合以下任意条件之一时显示：

- 当前目录包含一个 `shard.yml` 文件
- The current directory contains a `.cr` file

### 配置项

| Option     | 默认值                                | 描述                                                        |
| ---------- | ---------------------------------- | --------------------------------------------------------- |
| `symbol`   | `"🔮 "`                             | The symbol used before displaying the version of crystal. |
| `style`    | `"bold red"`                       | 此组件的样式。                                                   |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                                |
| `disabled` | `false`                            | Disables the `crystal` module.                            |

### Variables

| 字段        | 示例        | 描述                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v0.32.1` | The version of `crystal`             |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Directory

`directory` 组件显示当前目录的路径，显示的路径会截断到三个父目录以内。 如果您处于一个 git 仓库中，显示的路径则最多会截断到该仓库的根目录。

当使用 fish 风格的当前目录显示样式时，您会看到基于您的设置的每个上级目录的短名称，而不是隐藏被截断的上级目录。

例如，对于 `~/Dev/Nix/nixpkgs/pkgs`，其中 `nixpkgs` 是 git 仓库根目录，fish 风格相关选项设置为 `1`。 您将会看到 `~/D/N/nixpkgs/pkgs`，而在设置 fish 风格之前，当前路径将显示成 `nixpkgs/pkgs`。

### 配置项

| 字段                       | 默认值                                             | 描述                                                    |
| ------------------------ | ----------------------------------------------- | ----------------------------------------------------- |
| `truncation_length`      | `3`                                             | 当前目录路径被截断后最多保留的父目录数量。                                 |
| `truncate_to_repo`       | `true`                                          | 是否只截断到您当前处于的 git 仓库根目录下。                              |
| `format`                 | `"[$path]($style)[$lock_symbol]($lock_style) "` | The format for the module.                            |
| `style`                  | `"bold cyan"`                                   | 此组件的样式。                                               |
| `disabled`               | `false`                                         | 禁用 `directory` 组件。                                    |
| `read_only_symbol`       | `"🔒"`                                           | The symbol indicating current directory is read only. |
| `read_only_symbol_style` | `"red"`                                         | The style for the read only symbol.                   |

<details>
<summary>此组件有几个高级配置选项来控制当前目录路径的显示方式。</summary>

| Advanced Option             | 默认值    | 描述                                               |
| --------------------------- | ------ | ------------------------------------------------ |
| `substitutions`             |        | A table of substitutions to be made to the path. |
| `fish_style_pwd_dir_length` | `0`    | 使用 fish shell 当前目录路径逻辑时每个省略目录名使用的字符数。            |
| `use_logical_path`          | `true` | 显示由 shell 提供的逻辑路径（`PWD`）而不是 OS 提供的路径。            |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| 字段        | 示例                    | 描述                                  |
| --------- | --------------------- | ----------------------------------- |
| path      | `"D:/Projects"`       | The current directory path          |
| style\* | `"black bold dimmed"` | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### 配置项

| Option            | 默认值                                | 描述                                                                                      |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | The format for the module.                                                              |
| `symbol`          | `"🐳 "`                             | The symbol used before displaying the Docker context.                                   |
| `style`           | `"blue bold"`                      | 此组件的样式。                                                                                 |
| `only_with_files` | `false`                            | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `disabled`        | `true`                             | Disables the `docker_context` module.                                                   |

### Variables

| 字段        | 示例             | 描述                                   |
| --------- | -------------- | ------------------------------------ |
| context   | `test_context` | The current docker context           |
| symbol    |                | Mirrors the value of option `symbol` |
| style\* |                | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

`dotnet` 模块显示与当前目录下使用的 .NET Core SDK 相关联的版本。 如果当前目录已被绑定了一个版本的 SDK，则显示被帮定的版本。 否则此组件将显示最新安装的 SDK 版本。

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

在内部，此组件使用自己的版本检测机制。 一般来说此组件是直接执行 `dotnet --version` 的两倍快，但当你的 .NET 项目使用了不常见的目录布局时此组件可能显示一个错误的版本。 如果相比于速度您更需要正确的版本号，您可以在组件设置中设置 `heuristic = false` 来禁用该机制。

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### 配置项

| Option      | 默认值                                      | 描述                             |
| ----------- | ---------------------------------------- | ------------------------------ |
| `format`    | `"v[$symbol$version( 🎯 $tfm)]($style) "` | The format for the module.     |
| `symbol`    | `"•NET "`                                | 这个字段的内容会显示在当前 .NET 版本之前。       |
| `heuristic` | `true`                                   | 使用更快的版本探测机制以保证 starship 的运行速度。 |
| `style`     | `"bold blue"`                            | 此组件的样式。                        |
| `disabled`  | `false`                                  | 禁用 `dotnet` 组件。                |

### Variables

| 字段        | 示例               | 描述                                                                 |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Mirrors the value of option `symbol`                               |
| style\* |                  | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. 此组件将在符合以下任意条件之一时显示：

- 当前目录包含一个 `mix.exs` 文件.

### 配置项

| Option     | 默认值                                                           | 描述                                                              |
| ---------- | ------------------------------------------------------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`                                                        | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"`                                               | 此组件的样式。                                                         |
| `format`   | `"via [$symbol$version \\(OTP $otp_version\\)]($style) "` | The format for the module elixir.                               |
| `disabled` | `false`                                                       | Disables the `elixir` module.                                   |

### Variables

| 字段          | 示例      | 描述                                   |
| ----------- | ------- | ------------------------------------ |
| version     | `v1.10` | The version of `elixir`              |
| otp_version |         | The otp version of `elixir`          |
| symbol      |         | Mirrors the value of option `symbol` |
| style\*   |         | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. 此组件将在符合以下任意条件之一时显示：

- 当前目录包含一个 `elm.json` 文件
- 当前目录包含 `elm-package.json` 文件
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### 配置项

| Option     | 默认值                                | 描述                                              |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🌳 "`                             | A format string representing the symbol of Elm. |
| `style`    | `"cyan bold"`                      | 此组件的样式。                                         |
| `disabled` | `false`                            | Disables the `elm` module.                      |

### Variables

| 字段        | 示例        | 描述                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v0.19.1` | The version of `elm`                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Environment Variable

`env_var` 组件显示选定的环境变量的当前值。 此组件只有满足以下条件之一时才会被显示：

- 设置的 `variable` 是一个已存在的环境变量
- 未定义 `variable`，但定义了 `default`

### 配置项

| Option     | 默认值                            | 描述                         |
| ---------- | ------------------------------ | -------------------------- |
| `symbol`   |                                | 这个字段的内容会显示在环境变量值之前。        |
| `variable` |                                | 要显示的环境变量。                  |
| `default`  |                                | 所选变量未定义时显示的默认值。            |
| `format`   | `"with [$env_value]($style) "` | The format for the module. |
| `disabled` | `false`                        | 禁用 `env_var` 组件。           |

### Variables

| 字段        | 示例                                          | 描述                                         |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if *variable* would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Mirrors the value of option `symbol`       |
| style\* | `black bold dimmed`                         | Mirrors the value of option `style`        |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. 此组件将在符合以下任意条件之一时显示：

- 当前目录包含一个 `rebar.config` 文件.
- 当前目录包含一个 `erlang.mk` 文件.

### 配置项

| Option     | 默认值                                | 描述                                                       |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`                             | The symbol used before displaying the version of erlang. |
| `style`    | `"bold red"`                       | 此组件的样式。                                                  |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                               |
| `disabled` | `false`                            | Disables the `erlang` module.                            |

### Variables

| 字段        | 示例        | 描述                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v22.1.3` | The version of `erlang`              |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Git Branch

`git_branch` 组件显示当前目录的 git 仓库的活动分支。

### 配置项

| Option              | 默认值                              | 描述                                                                               |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------- |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.  Use `"$branch"` to refer to the current branch name. |
| `symbol`            | `" "`                           | A format string representing the symbol of git branch.                           |
| `style`             | `"bold purple"`                  | 此组件的样式。                                                                          |
| `truncation_length` | `2^63 - 1`                       | Truncates a git branch to X graphemes.                                           |
| `truncation_symbol` | `"…"`                            | 此字段的内容用来表示分支名称被截断。 You can use `""` for no symbol.                               |
| `disabled`          | `false`                          | 禁用 `git_branch` 组件。                                                              |

### Variables

| 字段        | 示例       | 描述                                                                                                   |
| --------- | -------- | ---------------------------------------------------------------------------------------------------- |
| branch    | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached HEAD). |
| symbol    |          | Mirrors the value of option `symbol`                                                                 |
| style\* |          | Mirrors the value of option `style`                                                                  |

\*: This variable can only be used as a part of a style string

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

### 配置项

| Option               | 默认值                            | 描述                                                    |
| -------------------- | ------------------------------ | ----------------------------------------------------- |
| `commit_hash_length` | `7`                            | 显示的 git 提交哈希值的长度。                                     |
| `format`             | `"[\\($hash\\)]($style) "` | The format for the module.                            |
| `style`              | `"bold green"`                 | 此组件的样式。                                               |
| `only_detached`      | `true`                         | Only show git commit hash when in detached HEAD state |
| `disabled`           | `false`                        | 禁用 `git_commit` 组件。                                   |

### Variables

| 字段        | 示例        | 描述                                  |
| --------- | --------- | ----------------------------------- |
| hash      | `b703eb3` | The current git commit hash         |
| style\* |           | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
```

## Git State

`git_state` 组件会显示当前目录在哪个 git 仓库中，以及正在进行的操作，例如：_REBASING_，_BISECTING_ 等。 进度信息（例如 REBASING 3/10）如果存在则也会被显示。

### 配置项

| Option         | 默认值                                                                 | 描述                                                                                      |
| -------------- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                        | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                         | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                       | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                                  | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                       | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                              | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                       | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                     | 此组件的样式。                                                                                 |
| `format`       | `"[\\($state( $progress_current/$progress_total)\\)]($style) "` | The format for the module.                                                              |
| `disabled`     | `false`                                                             | 禁用 `git_state` 模块                                                                       |

### Variables

| 字段               | 示例         | 描述                                  |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*        |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[git_state]
format = "[\\($state( $progress_current of $progress_total)\\)]($style) "
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Status

`git_status`组件通过相应的符号显示您当前目录中 git 仓库的状态。

### 配置项

| Option            | 默认值                                             | 描述                                  |
| ----------------- | ----------------------------------------------- | ----------------------------------- |
| `format`          | "([\[$all_status$ahead_behind\]]($style) )" | The default format for `git_status` |
| `conflicted`      | `"="`                                           | 这个分支有合并冲突。                          |
| `ahead`           | `"⇡"`                                           | The format of `ahead`               |
| `behind`          | `"⇣"`                                           | The format of `behind`              |
| `diverged`        | `"⇕"`                                           | The format of `diverged`            |
| `untracked`       | `"?"`                                           | The format of `untracked`           |
| `stashed`         | `"$"`                                           | The format of `stashed`             |
| `modified`        | `"!"`                                           | The format of `modified`            |
| `staged`          | `"+"`                                           | The format of `staged`              |
| `renamed`         | `"»"`                                           | The format of `renamed`             |
| `deleted`         | `"✘"`                                           | The format of `deleted`             |
| `show_sync_count` | `false`                                         | 显示领先/落后正在跟踪的分支的提交数。                 |
| `style`           | `"bold red"`                                    | 此组件的样式。                             |
| `disabled`        | `false`                                         | 禁用 `git_status` 组件。                 |

### Variables

The following variables can be used in `format`:

| 字段             | 描述                                                                                            |
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

| 字段             | 描述                                             |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| 字段      | 描述        |
| ------- | --------- |
| `count` | 显示相应的文件数量 |

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
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

## Golang

`golang` 组件显示当前安装的 Golang 版本。 此组件将在符合以下任意条件之一时显示：

- 当前目录包含 `go.mod` 文件
- 当前目录包含 `go.sum` 文件
- 当前目录包含 `glide.yaml` 文件
- 当前目录包含 `Gopkg.yml` 文件
- 当前目录包含 `Gopkg.lock` 文件
- The current directory contains a `.go-version` file
- 当前目录包含 `Godeps` 目录
- 当前目录包含一个使用 `.go` 扩展名的文件

### 配置项

| Option     | 默认值                                | 描述                                             |
| ---------- | ---------------------------------- | ---------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                     |
| `symbol`   | `"🐹 "`                             | A format string representing the symbol of Go. |
| `style`    | `"bold cyan"`                      | 此组件的样式。                                        |
| `disabled` | `false`                            | 禁用 `golang` 组件。                                |

### Variables

| 字段        | 示例        | 描述                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v1.12.1` | The version of `go`                  |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of Helm. 此组件将在符合以下任意条件之一时显示：

- 当前目录包含一个 `helmfile.yaml` 文件
- The current directory contains a `Chart.yaml` file

### 配置项

| Option     | 默认值                                | 描述                                               |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"⎈ "`                             | A format string representing the symbol of Helm. |
| `style`    | `"bold white"`                     | 此组件的样式。                                          |
| `disabled` | `false`                            | Disables the `helm` module.                      |

### Variables

| 字段        | 示例       | 描述                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v3.1.1` | The version of `helm`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

`hostname` 组件显示系统主机名。

### 配置项

| Option     | 默认值                         | 描述                                                                 |
| ---------- | --------------------------- | ------------------------------------------------------------------ |
| `ssh_only` | `true`                      | 仅在连接到 SSH 会话时显示主机名。                                                |
| `trim_at`  | `"."`                       | 当主机名过长被截断时，会截断成第一次匹配该字符串之前的主机名。 `"."` 会让主机名截断到第一个点处。 `""` 会禁用任何截断。 |
| `format`   | `"on [$hostname]($style) "` | The format for the module.                                         |
| `style`    | `"bold dimmed green"`       | 此组件的样式。                                                            |
| `disabled` | `false`                     | 禁用 `hostname` 组件。                                                  |

### Variables

| 字段        | 示例  | 描述                                   |
| --------- | --- | ------------------------------------ |
| number    | `1` | The number of jobs                   |
| symbol    |     | Mirrors the value of option `symbol` |
| style\* |     | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

`java` 组件显示当前安装的 Java 版本。 此组件将在符合以下任意条件之一时显示：

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- 当前目录包含一个扩展名为 `.java`，`.class`，`.gradle` 或 `.jar` 的文件

### 配置项

| Option     | 默认值                                    | 描述                                              |
| ---------- | -------------------------------------- | ----------------------------------------------- |
| `format`   | `"via [${symbol}${version}]($style) "` | The format for the module.                      |
| `symbol`   | `"☕ "`                                 | A format string representing the symbol of Java |
| `style`    | `"red dimmed"`                         | 此组件的样式。                                         |
| `disabled` | `false`                                | 禁用 `java` 组件。                                   |

### Variables

| 字段        | 示例    | 描述                                   |
| --------- | ----- | ------------------------------------ |
| version   | `v14` | The version of `java`                |
| symbol    |       | Mirrors the value of option `symbol` |
| style\* |       | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

`jobs` 组件显示当前正在运行的任务数量。 仅当有后台任务运行时，此组件才会显示。 如果有超过 1 个作业，模块将显示正在运行的作业数量，如果配置了 `threshold` 字段，则使用它作为显示作业数量的下限。

### 配置项

| Option      | 默认值                           | 描述                                               |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | 如果超过此字段的值，显示任务数量。                                |
| `format`    | `"[$symbol$number]($style) "` | The format for the module.                       |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | 此组件的样式。                                          |
| `disabled`  | `false`                       | 禁用 `jobs` 组件。                                    |

### Variables

| 字段        | 示例  | 描述                                   |
| --------- | --- | ------------------------------------ |
| number    | `1` | The number of jobs                   |
| symbol    |     | Mirrors the value of option `symbol` |
| style\* |     | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. 此组件将在符合以下任意条件之一时显示：

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### 配置项

| Option     | 默认值                                | 描述                                                |
| ---------- | ---------------------------------- | ------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                        |
| `symbol`   | `"ஃ "`                             | A format string representing the symbol of Julia. |
| `style`    | `"bold purple"`                    | 此组件的样式。                                           |
| `disabled` | `false`                            | Disables the `julia` module.                      |

### Variables

| 字段        | 示例       | 描述                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.4.0` | The version of `julia`               |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

此组件默认被禁用。 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

:::

### 配置项

| Option                  | 默认值                                                      | 描述                                                                    |
| ----------------------- | -------------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`                | `"☸ "`                                                   | A format string representing the symbol displayed before the Cluster. |
| `format`                | `"on [$symbol$context( \\($namespace\\))]($style) "` | The format for the module.                                            |
| `style`                 | `"cyan bold"`                                            | 此组件的样式。                                                               |
| `namespace_spaceholder` | `none`                                                   | The value to display if no namespace was found.                       |
| `context_aliases`       |                                                          | Table of context aliases to display.                                  |
| `disabled`              | `true`                                                   | Disables the `kubernetes` module.                                     |

### Variables

| 字段        | 示例                   | 描述                                       |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Mirrors the value of option `symbol`     |
| style\* |                      | Mirrors the value of option `style`      |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[kubernetes]
format = "on [⛵ $context \\($namespace\\)](dimmed green) "
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## Line Break

`line_break` 组件将提示分隔为两行。

### 配置项

| Option     | 默认值     | 描述                          |
| ---------- | ------- | --------------------------- |
| `disabled` | `false` | 禁用 `line_break` 组件，使提示成为单行。 |

### 示例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Memory Usage

`memory_usage` 组件显示当前系统内存和交换区使用情况。

默认情况下，如果系统交换区使用不为 0，则会显示交换区使用情况。

::: tip

此组件默认被禁用。 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

:::

### 配置项

| Option      | 默认值                                           | 描述                         |
| ----------- | --------------------------------------------- | -------------------------- |
| `threshold` | `75`                                          | 隐藏内存使用情况，除非它超过这个百分比。       |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | The format for the module. |
| `symbol`    | `"🐏"`                                         | 这个字段的内容会显示在当前内存使用情况之前。     |
| `style`     | `"bold dimmed white"`                         | 此组件的样式。                    |
| `disabled`  | `true`                                        | 禁用 `memory_usage` 模块       |

### Variables

| 字段            | 示例            | 描述                                                                 |
| ------------- | ------------- | ------------------------------------------------------------------ |
| ram           | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct       | `48%`         | The percentage of the current system memory.                       |
| swap\**     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\** | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol        | `🐏`           | Mirrors the value of option `symbol`                               |
| style\*     |               | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

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

`hg_branch` 组件显示当前目录的 hg 仓库的活动分支。

### 配置项

| Option              | 默认值                              | 描述                              |
| ------------------- | -------------------------------- | ------------------------------- |
| `symbol`            | `" "`                           | 该字段的内容显示于当前仓库的 hg 书签或活动分支名之前。   |
| `style`             | `"bold purple"`                  | 此组件的样式。                         |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.      |
| `truncation_length` | `2^63 - 1`                       | 将显示的 hg 分支名截断到该数量的字素（graphemes） |
| `truncation_symbol` | `"…"`                            | 此字段的内容用来表示分支名称被截断。              |
| `disabled`          | `true`                           | 禁用 `hg_branch` 组件。              |

### Variables

| 字段        | 示例       | 描述                                   |
| --------- | -------- | ------------------------------------ |
| branch    | `master` | The active mercurial branch          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of Nim. 此组件将在符合以下任意条件之一时显示：

- 当前目录包含一个 `nim.cfg` 文件
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### 配置项

| Option     | 默认值                                | 描述                                                    |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module                             |
| `symbol`   | `"👑 "`                             | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"`                    | 此组件的样式。                                               |
| `disabled` | `false`                            | Disables the `nim` module.                            |

### Variables

| 字段        | 示例       | 描述                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.2.0` | The version of `nimc`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

`nix_shell` 组件显示 nix-shell 环境。 当处于一个 nix-shell 环境中时，此组件会被显示。

### 配置项

| Option       | 默认值                                                | 描述                                                    |
| ------------ | -------------------------------------------------- | ----------------------------------------------------- |
| `format`     | `"via [$symbol$state( \\($name\\))]($style) "` | The format for the module.                            |
| `symbol`     | `"❄️  "`                                           | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                      | 此组件的样式。                                               |
| `impure_msg` | `"impure"`                                         | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                           | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                            | 禁用 `nix_shell` 组件。                                    |

### Variables

| 字段        | 示例      | 描述                                   |
| --------- | ------- | ------------------------------------ |
| state     | `pure`  | The state of the nix-shell           |
| name      | `lorri` | The name of the nix-shell            |
| symbol    |         | Mirrors the value of option `symbol` |
| style\* |         | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = "via [☃️ $state( \\($name\\))](bold blue) "
```

## NodeJS

`nodejs` 组件显示当前安装的 NodeJS 版本。 此组件将在符合以下任意条件之一时显示：

- 当前目录包含 `package.json` 文件
- The current directory contains a `.node-version` file
- 当前目录包含 `node_modules` 目录
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### 配置项

| Option     | 默认值                                | 描述                                                 |
| ---------- | ---------------------------------- | -------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                         |
| `symbol`   | `"⬢ "`                             | A format string representing the symbol of NodeJS. |
| `style`    | `"bold green"`                     | 此组件的样式。                                            |
| `disabled` | `false`                            | 禁用 `nodejs` 组件。                                    |

###  Variables

| 字段        | 示例         | 描述                                   |
| --------- | ---------- | ------------------------------------ |
| version   | `v13.12.0` | The version of `node`                |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## Package Version

当前目录是软件包的代码仓库时，将显示 `package` 组件，并显示软件包当前版本。 The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia` and `mix` packages.

- **npm** —— `npm` 软件包版本从当前目录下的 `package.json` 中得到
- **cargo** —— `cargo` 软件包的版本从当前目录下的 `Cargo.toml` 中得到
- **poetry** —— `poetry` 软件包版本从当前目录下的 `pyproject.toml` 中得到
- **composer** —— `composer` 软件包版本从当前目录下的 `composer.json` 中得到
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present

> ⚠ 此组件显示的是源代码在当前目录中的软件包的版本，而不是包管理器的版本。

### 配置项

| Option            | 默认值                                | 描述                                                        |
| ----------------- | ---------------------------------- | --------------------------------------------------------- |
| `format`          | `"via [$symbol$version]($style) "` | The format for the module.                                |
| `symbol`          | `"📦 "`                             | 这个字段的内容会显示在当前软件包版本之前。                                     |
| `style`           | `"bold 208"`                       | 此组件的样式。                                                   |
| `display_private` | `false`                            | Enable displaying version for packages marked as private. |
| `disabled`        | `false`                            | 禁用 `package` 组件。                                          |

### Variables

| 字段        | 示例       | 描述                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.0.0` | The version of your package          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. 此组件将在符合以下任意条件之一时显示：

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### 配置项

| Option     | 默认值                                | 描述                                                      |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format string for the module.                       |
| `symbol`   | `"🐫 "`                             | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"`                    | 此组件的样式。                                                 |
| `disabled` | `false`                            | Disables the `ocaml` module.                            |

### Variables

| 字段        | 示例        | 描述                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v4.10.0` | The version of `ocaml`               |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## PHP

`php` 组件显示当前安装的 PHP 版本。 此组件将在符合以下任意条件之一时显示：

- 当前目录包含一个 `composer.json` 文件
- The current directory contains a `.php-version` file
- 当前目录包含一个 `.php` 文件

### 配置项

| Option     | 默认值                                | 描述                         |
| ---------- | ---------------------------------- | -------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module. |
| `symbol`   | `"🐘 "`                             | 这个字段的内容会显示在当前 PHP 版本之前。    |
| `style`    | `"147 bold"`                       | 此组件的样式。                    |
| `disabled` | `false`                            | 禁用 `php` 组件。               |

### Variables

| 字段        | 示例       | 描述                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v7.3.8` | The version of `php`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

此组件将在符合以下任意条件之一时显示：

- 当前目录包含 `.python-version` 文件
- 当前目录包含 `requirements.txt` 文件
- 当前目录包含 `pyproject.toml` 文件
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- 当前目录包含 `Pipfile` 文件
- 当前目录包含一个 `tox.ini` 文件
- 当前目录包含一个 `setup.py` 文件
- The current directory contains a `__init__.py` file
- 当前处于一个活跃的 python 虚拟环境中

### 配置项

| Option               | 默认值                                                            | 描述                                                                         |
| -------------------- | -------------------------------------------------------------- | -------------------------------------------------------------------------- |
| `format`             | `"via [${symbol}${version}( \\($virtualenv\\))]($style) "` | The format for the module.                                                 |
| `symbol`             | `"🐍 "`                                                         | A format string representing the symbol of Python                          |
| `style`              | `"yellow bold"`                                                | 此组件的样式。                                                                    |
| `pyenv_version_name` | `false`                                                        | 使用 pyenv 获取 Python 版本                                                      |
| `scan_for_pyfiles`   | `true`                                                         | If false, Python files in the current directory will not show this module. |
| `disabled`           | `false`                                                        | 禁用 `python` 组件。                                                            |

### Variables

| 字段         | 示例              | 描述                                   |
| ---------- | --------------- | ------------------------------------ |
| version    | `"v3.8.1"`      | The version of `python`              |
| symbol     | `"🐍 "`          | Mirrors the value of option `symbol` |
| style      | `"yellow bold"` | Mirrors the value of option `style`  |
| virtualenv | `"venv"`        | The current `virtualenv` name        |

<details>
<summary>This module has some advanced configuration options.</summary>

| 字段              | 默认值      | 描述                                                                            |
| --------------- | -------- | ----------------------------------------------------------------------------- |
| `python_binary` | `python` | Configures the python binary that Starship executes when getting the version. |

The `python_binary` variable changes the binary that Starship executes to get the version of Python, it doesn't change the arguments that are used.

```toml
# ~/.config/starship.toml

[python]
python_binary = "python3"
```

</details>

### 示例

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

`ruby` 组件显示当前安装的 Ruby 版本。 此组件将在符合以下任意条件之一时显示：

- 当前目录包含 `Gemfile` 文件
- The current directory contains a `.ruby-version` file
- 当前目录包含 `.rb` 文件

### 配置项

| Option     | 默认值                                | 描述                                               |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"💎 "`                             | A format string representing the symbol of Ruby. |
| `style`    | `"bold red"`                       | 此组件的样式。                                          |
| `disabled` | `false`                            | 禁用 `ruby` 组件。                                    |

### Variables

| 字段        | 示例       | 描述                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v2.5.1` | The version of `ruby`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

`rust` 组件显示当前安装的 Rust 版本。 此组件将在符合以下任意条件之一时显示：

- 当前目录包含 `Cargo.toml` 文件
- 当前目录包含一个使用 `.rs` 扩展名的文件

### 配置项

| Option     | 默认值                                | 描述                                              |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🦀 "`                             | A format string representing the symbol of Rust |
| `style`    | `"bold red"`                       | 此组件的样式。                                         |
| `disabled` | `false`                            | 禁用 `rust` 组件。                                   |

### Variables

| 字段        | 示例                | 描述                                   |
| --------- | ----------------- | ------------------------------------ |
| version   | `v1.43.0-nightly` | The version of `rustc`               |
| symbol    |                   | Mirrors the value of option `symbol` |
| style\* |                   | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### 配置项

| Option     | 默认值                                  | 描述                                               |
| ---------- | ------------------------------------ | ------------------------------------------------ |
| `format`   | `"[$symbol\\[$env\\]]($style) "` | The format for the module.                       |
| `symbol`   | `""`                                 | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`                 | 此组件的样式。                                          |
| `disabled` | `false`                              | Disables the `singularity` module.               |

### Variables

| 字段        | 示例           | 描述                                   |
| --------- | ------------ | ------------------------------------ |
| env       | `centos.img` | The current singularity image        |
| symbol    |              | Mirrors the value of option `symbol` |
| style\* |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[singularity]
format = "[📦 \\[$env\\]]($style) "
```

## Terraform

`terraform` 组件显示当前选定的 terraform 工作区和版本。 默认情况下不会显示 terraform 版本，因为当使用大量插件时，当前版本 terraform 查询版本号很慢。 If you still want to enable it, [follow the example shown below](#with-version). 此组件将在符合以下任意条件之一时显示：

- 当前目录包含 `.terraform` 目录
- 当前目录包含一个使用 `.tf` 扩展名的文件

### 配置项

| Option     | 默认值                                  | 描述                                                    |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`   | `"💠 "`                               | A format string shown before the terraform workspace. |
| `style`    | `"bold 105"`                         | 此组件的样式。                                               |
| `disabled` | `false`                              | 禁用 `terraform` 组件。                                    |

### Variables

| 字段        | 示例         | 描述                                   |
| --------- | ---------- | ------------------------------------ |
| version   | `v0.12.24` | The version of `terraform`           |
| workspace | `default`  | The current terraform workspace      |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

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

`time` 组件显示当前的 **本地** 时间。 `format` 字段值会提供给 [`chrono`](https://crates.io/crates/chrono) crate 用来控制时间显示方式。 请参阅 [chrono strftime 文档](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) 以了解可用格式选项。

::: tip

此组件默认被禁用。 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

:::

### 配置项

| Option            | 默认值                     | 描述                                                                                                    |
| ----------------- | ----------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                     |
| `use_12hr`        | `false`                 | 启用 12 小时格式                                                                                            |
| `time_format`     | 见下文解释                   | 用来格式化时间显示的 [chrono 格式字符串](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)             |
| `style`           | `"bold yellow"`         | 显示时间的样式。                                                                                              |
| `utc_time_offset` | `"local"`               | 设置所用 UTC 偏移量。 Range from -24 &lt; x &lt; 24. 允许使用浮点数来得到 30/45 分钟的时区偏移。                    |
| `disabled`        | `true`                  | 禁用 `time` 组件。                                                                                         |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. 否则，其默认值为 `"%T"`。 Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| 字段        | 示例         | 描述                                  |
| --------- | ---------- | ----------------------------------- |
| time      | `13:08:10` | The current time.                   |
| style\* |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 示例

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

`username` 组件显示当前活跃的用户名。 此组件将在符合以下任意条件之一时显示：

- 当前用户是 root
- 当前用户与登录用户不相同
- 用户正通过 SSH 会话连接访问
- 字段 `show_always` 被设置为 true

### 配置项

| Option        | 默认值                      | 描述                         |
| ------------- | ------------------------ | -------------------------- |
| `style_root`  | `"bold red"`             | 当前用户为 root 时使用的样式。         |
| `style_user`  | `"bold yellow"`          | 非 root 用户使用的样式。            |
| `format`      | `"via [$user]($style) "` | The format for the module. |
| `show_always` | `false`                  | 总是显示 `username` 组件。        |
| `disabled`    | `false`                  | 禁用 `username` 组件。          |

### Variables

| 字段      | 示例           | 描述                                                                                          |
| ------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style` | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`  | `"matchai"`  | The currently logged-in user ID.                                                            |

### 示例

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

The `zig` module shows the currently installed version of Zig. 此组件将在符合以下任意条件之一时显示：

- The current directory contains a `.zig` file

### 配置项

| Option     | 默认值                                | 描述                                                    |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`                             | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"`                    | 此组件的样式。                                               |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                            |
| `disabled` | `false`                            | Disables the `zig` module.                            |

### Variables

| 字段        | 示例       | 描述                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v0.6.0` | The version of `zig`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

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

| Option        | 默认值                           | 描述                                                                                                                         |
| ------------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                               | The command whose output should be printed.                                                                                |
| `when`        |                               | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                               | [See below](#custom-command-shell)                                                                                         |
| `描述`          | `"<custom module>"`     | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                          | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                          | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                          | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                          | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                | 此组件的样式。                                                                                                                    |
| `format`      | `"[$symbol$output]($style) "` | The format for the module.                                                                                                 |
| `disabled`    | `false`                       | Disables this `custom` module.                                                                                             |

### Variables

| 字段        | 描述                                     |
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

### 示例

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

The `purescript` module shows the currently installed version of PureScript version. 此组件将在符合以下任意条件之一时显示：

- 当前目录包含一个 `spago.dhall` 文件
- The current directory contains a \*.purs files

### 配置项

| Option     | 默认值                                | 描述                                                           |
| ---------- | ---------------------------------- | ------------------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                                   |
| `symbol`   | `"<=> "`                     | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"`                     | 此组件的样式。                                                      |
| `disabled` | `false`                            | Disables the `purescript` module.                            |

### Variables

| 字段        | 示例       | 描述                                   |
| --------- | -------- | ------------------------------------ |
| version   | `0.13.5` | The version of `purescript`          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 示例

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```
