# 配置

为 Starship 创建配置文件 `~/.config/starship.toml`。

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Starship 的所有配置都在此  [TOML](https://github.com/toml-lang/toml) 文件内：

```toml
# 根据 schema 提供自动补全
"$schema" = 'https://starship.rs/config-schema.json'

# 在提示符之间插入空行
add_newline = true

# 将提示符中的 '❯' 替换为 '➜'
[character] # 此组件名称为 'character'
success_symbol = '[➜](bold green)' # 将 'success_symbol' 字段设置成颜色为 'bold green' 的 '➜'  

# 禁用 'package' 组件，将其隐藏
[package]
disabled = true
```

### 配置文件路径

您可以使用 `STARSHIP_CONFIG` 环境变量更改默认配置文件的位置：

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

在 PowerShell (Windows) 中，在 `$PROFILE` 中添加下面的代码行能达到同样的效果：

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

或者在 Cmd (Windows) 中，将以下内容添加到您的 `starship.lua`：

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### 日志

默认情况下，Starship 会将警告和错误日志记录到文件 `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`. 其中 session key 与您的终端实例相对应。 不过，也可以使用 `STARSHIP_CACHE` 环境变量来修改：

```sh
export STARSHIP_CACHE=~/.starship/cache
```

在 PowerShell (Windows) 中，在 `$PROFILE` 中添加下面的代码行能达到同样的效果：

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

或者在 Cmd (Windows) 中，将下面的代码添加到 `starship.lua`：

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### 术语

**组件（Module）**：提示符中显示信息的部分，基于系统上下文信息。 例如，如果您当前的目录是 Node.js 项目，“nodejs” 组件会显示当前安装在您电脑上的 Node.js 的版本。

**字段（Variable）**：组件中显示信息的部分。 例如，“nodejs”组件里的“version”字段包含了现有 Node.js 的版本。

依惯例，大多组件都有一个带有默认终端颜色的前缀（例如“nodejs”的 `via`），以及一个空格作为后缀。

### 字符串

在 TOML 语法中，[字符串](https://toml.io/en/v1.0.0#string) 以 `'`、`"`、`'''`、或 `"""` 来表示。

以下 Starship 语法符号在样式字符串中具有特殊用途，必须转义才能以原字符显示： `$ [ ] ( )`。

| 符号    | 类型      | 备注        |
| ----- | ------- | --------- |
| `'`   | 字面字符串   | 较少转义      |
| `"`   | 字符串     | 较多转义      |
| `'''` | 多行字面字符串 | 较少转义      |
| `"""` | 多行字符串   | 较多转义，无视换行 |

例如：

```toml
# 字面字符串
format = '☺\☻ '

# 常规字符串
format = "☺\\☻ "

# 转义 Starship 符号
format = '\[\$\] '
```

需要换行时，请使用多行字符串。 例如，以下 `format` 字段的值都会下一行输出 `$` 符号：

```toml
# 使用字面字符串
format = '''

\$'''

# 使用多行字符串
format = """

\\$"""

# 使用基本字符串
format = "\n\\$"
```

使用多行字符串时，可以转义换行符以忽略换行，从而实现格式化。

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

### 格式字符串

组件将根据格式字符串输出其中字段。 大多数组件可以通过 `format` 项来配置它的显示格式。 格式设定内可以包含文本、字段以及文本组。

#### 字段

字段由 `$` 和名称组成。 名称仅支持字母、数字和 `_`。

例如：

- 格式字符串 `'$version'` 仅包含字段 `version`。
- 格式字符串 `'$git_branch$git_commit'` 包含两个字段，`git_branch` 和 `git_commit`。
- 格式字符串 `'$git_branch $git_commit'` 包含两个字段，以一个空格分隔。

#### 文本组

一个文本组以两个括号部分组成：

第一个部分包含在 `[]` 中，内容是一个[格式字符串](#format-strings)， 你可以在当中添加文本、字段，甚至嵌套其他文本组。

第二个部分包含在 `()`中，内容是一个[样式字符串](#style-strings)， 用于指定第一部分括号中内容的样式。

例如：

- `'[on](red bold)'` 会以红色粗体输出字符串 `on`。
- `'[⌘ $version](bold green)'` 会以绿色粗体输出符号 `⌘` 以及字段 `version` 的内容。
- `'[a [b](red) c](green)'` 会输出 `a b c`，其中 `b` 为红色，`a` 与 `c` 为绿色。

#### 样式字符串

Starship 中的大多数组件允许您为其设置显示样式。 显示样式可以通过一个字符串字段（一般是 `style`）来设置。 以下的例子给出了一些样式字符串并描述了它们的效果。 样式字符串的完整语法请查阅 [高级配置指南](../advanced-config/)。

- `'fg:green bg:blue'` 在蓝色背景上显示绿色文本
- `'bg:blue fg:bright-green'` 在蓝色背景上显示亮绿色文本
- `'bold fg:27'` 用 [ANSI颜色](https://i.stack.imgur.com/KTSQa.png) 27 设置粗体文本
- `'underline bg:#bf5700'` 在深橙色背景上显示带下划线文本
- `'bold italic fg:purple'` 设置粗体、斜体、紫色文本
- `''` 显式禁用所有样式

请注意，最终显示样式将由您的终端模拟器控制。 例如，有的终端模拟器对于“bold”属性的文本是加亮颜色而不是加粗文字，有的颜色主题对“普通”和“明亮”两种属性的颜色使用相同的颜色值。 此外，要显示斜体文本，也需要您的终端的支持。

#### 条件格式字符串

由 `(``)` 包裹的内容是条件格式字符串： 当其中包含的字段全部为空时，该字符串不会渲染。

例如：

- `'(@$region)'`：如果字段 `region` 为 `None` 或空字符串，什么都不显示；否则显示 `@` 和 region 字段的值。
- `'(some text)'`：永远不会显示，因为括号里没有任何字段。
- 如果 `$combined` 是 `\[$a$b\]` 的简写，则 `'($combined)'` 当切仅当 `$a` 和 `$b` 都为 `None` 时什么都不显示。 这和 `'(\[$a$b\] )'` 的效果相同。

### 反向匹配

许多组件都有  `detect_extensions`、`detect_files` 和  `detect_folders` 变量。 这些变量接受用于匹配或不匹配的字符串。 反向匹配指用于指定不匹配的字符串，以字符 '!' 开头。 目录中出现反向匹配的_任何_文件会导致组件不被匹配。

对于文件扩展名，文件名中第一个点后的内容和最后一个点后的内容会被分别匹配一次。 例如，`foo.bar.tar.gz` 在 `detect_extensions` 字段中既会匹配 `bar.tar.gz`，也会匹配 `gz`。 文件名以点开头的文件则被视为没有扩展名。

实践中的例子，你可以这样来匹配 TypeScript，同时排除 MPEG 传输流文件：

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## 提示符

以下是关于提示符的配置项。

### 配置项

| 选项                | 默认值                           | 描述                                                                                                                                                                                 |
| ----------------- | ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [见下文](#default-prompt-format) | 配置提示符的格式。                                                                                                                                                                          |
| `right_format`    | `''`                          | 参见[启用右侧提示](../advanced-config/#enable-right-prompt)                                                                                                                                |
| `scan_timeout`    | `30`                          | Starship 扫描文件的超时时间（单位：毫秒）。                                                                                                                                                         |
| `command_timeout` | `500`                         | Starship 执行命令的超时时间（单位：毫秒）。                                                                                                                                                         |
| `add_newline`     | `true`                        | 在 shell 提示符之间插入空行。                                                                                                                                                                 |
| `palette`         | `''`                          | Sets which color palette from `palettes` to use.                                                                                                                                   |
| `palettes`        | `{}`                          | Collection of color palettes that assign [colors](../advanced-config/#style-strings) to user-defined names. Note that color palettes cannot reference their own color definitions. |
| `follow_symlinks` | `true`                        | 跟随系统链接检查他们是否是一个路径。在诸如 Git 等组件中使用。                                                                                                                                                  |

> [!TIP] If you have symlinks to networked filesystems, consider setting `follow_symlinks` to `false`.

### 示例

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

### 默认提示符格式

如果没有提供`format`字段或者它的值是空的，将会使用默认的`format`配置来指定提示符的格式。 默认配置如下：

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
$nats\
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
$bun\
$c\
$cmake\
$cobol\
$cpp\
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
$gradle\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$lua\
$maven\
$mojo\
$nim\
$nodejs\
$ocaml\
$odin\
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
$xmake\
$zig\
$buf\
$guix_shell\
$nix_shell\
$conda\
$pixi\
$meson\
$spack\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
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
$container\
$netns\
$os\
$shell\
$character"""
```

如果你只是想扩展默认的格式，你可以使用 `$all`；你另外添加到格式中的组件不会重复出现。 例如：

```toml
# Move the directory to the second line
format = '$all$directory$character'
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials. The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process`, `sso_start_url`, or `sso_session` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice. If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var and the credentials expiration date is read from the `AWS_SESSION_EXPIRATION` env var.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile is read from the `AWSUME_PROFILE` env var and the credentials expiration date is read from the `AWSUME_EXPIRATION` env var.

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials` falls back to the `x_security_token_expires` key.

When using [aws-sso-cli](https://github.com/synfinatic/aws-sso-cli) the profile is read from the `AWS_SSO_PROFILE` env var.

### 配置项

| 选项                  | 默认值                                                                   | 描述                                                                                                          |
| ------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | 组件格式化模板。                                                                                                    |
| `symbol`            | `'☁️ '`                                                               | 这个字段的内容会显示在当前 AWS 配置信息之前。                                                                                   |
| `region_aliases`    | `{}`                                                                  | 地区缩写列表，用来显示在 AWS 主机名之后。                                                                                     |
| `profile_aliases`   | `{}`                                                                  | Table of profile aliases to display in addition to the AWS name.                                            |
| `style`             | `'bold yellow'`                                                       | 此组件的样式。                                                                                                     |
| `expiration_symbol` | `'X'`                                                                 | The symbol displayed when the temporary credentials have expired.                                           |
| `disabled`          | `false`                                                               | 禁用 `AWS` 组件。                                                                                                |
| `force_display`     | `false`                                                               | If `true` displays info even if `credentials`, `credential_process` or `sso_start_url` have not been setup. |

### 变量

| 字段        | 示例               | 描述                                          |
| --------- | ---------------- | ------------------------------------------- |
| region    | `ap-northeast-1` | The current AWS region                      |
| profile   | `astronauts`     | The current AWS profile                     |
| duration  | `2h27m20s`       | The temporary credentials validity duration |
| symbol    |                  | `symbol`对应值                                 |
| style\* |                  | `style`对应值                                  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

#### 显示所有内容

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

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 字段                     | 默认值                                      | 描述                                                                                    |
| ---------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------- |
| `format`               | `'on [$symbol($subscription)]($style) '` | The format for the Azure module to render.                                            |
| `symbol`               | `'󰠅 '`                                   | 格式中使用的符号                                                                              |
| `style`                | `'blue bold'`                            | The style used in the format.                                                         |
| `disabled`             | `true`                                   | 禁用 `azure` 组件。                                                                        |
| `subscription_aliases` | `{}`                                     | Table of subscription name aliases to display in addition to Azure subscription name. |

### 示例

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

## 电池

`battery` 组件显示电池充电情况和当前充电状态。 这个组件只会在当前电量低于 10% 时显示。

### 配置项

| 选项                   | 默认值                               | 描述               |
| -------------------- | --------------------------------- | ---------------- |
| `full_symbol`        | `'󰁹 '`                            | 显示于电池充满时。        |
| `charging_symbol`    | `'󰂄 '`                            | 显示于正在充电时。        |
| `discharging_symbol` | `'󰂃 '`                            | 显示于电池放电时。        |
| `unknown_symbol`     | `'󰂑 '`                            | 显示于电池状态未知时       |
| `empty_symbol`       | `'󰂎 '`                            | 显示于电池状态为空时       |
| `format`             | `'[$symbol$percentage]($style) '` | 组件格式化模板。         |
| `display`            | [见下文](#battery-display)           | 电量显示阈值和样式。       |
| `disabled`           | `false`                           | 禁用 `battery` 组件。 |

### 示例

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### Battery 组件的显示

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style). 如果 `display` 没有设置， 默认配置如下：

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### 配置项

`display` 字段的子字段如下：

| 选项                   | 默认值          | 描述                                                                                                        |
| -------------------- | ------------ | --------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | 当前显示设置的电量上限（见示例）                                                                                          |
| `style`              | `'red bold'` | 若组件被显示，则使用此样式                                                                                             |
| `charging_symbol`    |              | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.    |
| `discharging_symbol` |              | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option. |

#### 示例

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

### 配置项

| 选项                  | 默认值                                             | 描述                                                    |
| ------------------- | ----------------------------------------------- | ----------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | The format for the `buf` module.                      |
| `version_format`    | `'v${raw}'`                                     | 版本格式                                                  |
| `symbol`            | `'🐃 '`                                          | The symbol used before displaying the version of Buf. |
| `detect_extensions` | `[]`                                            | 触发此组件的扩展名                                             |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | 触发此组件的文件名                                             |
| `detect_folders`    | `[]`                                            | 触发此组件的文件夹                                             |
| `style`             | `'bold blue'`                                   | 此组件的样式。                                               |
| `disabled`          | `false`                                         | Disables the `elixir` module.                         |

### 变量

| 字段        | 示例       | 描述                   |
| --------- | -------- | -------------------- |
| `version` | `v1.0.0` | The version of `buf` |
| `symbol`  |          | `symbol`对应值          |
| `style`*  |          | `style`对应值           |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

`bun` 组件显示当前安装的 [bun](https://bun.sh) JavaScript 运行时版本。 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含 `bun.lock` 文件
- 当前目录包含一个 `bun.lockb` 文件
- 当前目录包含 `bunfig.toml` 文件

### 配置项

| 选项                  | 默认值                                        | 描述                                             |
| ------------------- | ------------------------------------------ | ---------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`       | 组件格式化模板。                                       |
| `version_format`    | `'v${raw}'`                                | 版本格式。 可用的字段有 `raw`, `major`, `minor` 和 `patch` |
| `symbol`            | `'🥟 '`                                     | 用于表示 Bun 的格式化字符串                               |
| `detect_extensions` | `[]`                                       | 触发此组件的扩展名                                      |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | 触发此组件的文件名                                      |
| `detect_folders`    | `[]`                                       | 触发此组件的文件夹                                      |
| `style`             | `'bold red'`                               | 此组件的样式。                                        |
| `disabled`          | `false`                                    | 禁用`bun`组件                                      |

### 变量

| 字段        | 示例       | 描述          |
| --------- | -------- | ----------- |
| version   | `v0.1.4` | `bun` 版本    |
| symbol    |          | `symbol`对应值 |
| style\* |          | `style`对应值  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

#### 自定义格式

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

The `c` module shows some information about your C compiler. By default the module will be shown if the current directory contains a `.c` or `.h` file.

### 配置项

| 选项                  | 默认值                                                                           | 描述                                                     |
| ------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | The format string for the module.                      |
| `version_format`    | `'v${raw}'`                                                                   | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`          |
| `symbol`            | `'C '`                                                                        | The symbol used before displaying the compiler details |
| `detect_extensions` | `['c', 'h']`                                                                  | 触发此组件的扩展名                                              |
| `detect_files`      | `[]`                                                                          | 触发此组件的文件名                                              |
| `detect_folders`    | `[]`                                                                          | 触发此组件的文件夹                                              |
| `commands`          | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | How to detect what the compiler is                     |
| `style`             | `'bold 149'`                                                                  | 此组件的样式。                                                |
| `disabled`          | `false`                                                                       | Disables the `c` module.                               |

### 变量

| 字段      | 示例     | 描述          |
| ------- | ------ | ----------- |
| name    | clang  | 编译器的名称      |
| version | 13.0.0 | 编译器的版本      |
| symbol  |        | `symbol`对应值 |
| style   |        | `style`对应值  |

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship will try executing each command until it gets a result on STDOUT.

If a C compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### 示例

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## CPP

The `cpp` module shows some information about your `C++` compiler. By default, the module will be shown if the current directory contains a `.cpp`, `.hpp`, or other `C++`-related files.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                  | 默认值                                                                              | 描述                                                     |
| ------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                     | The format string for the module.                      |
| `version_format`    | `'v${raw}'`                                                                      | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`          |
| `symbol`            | `'C++ '`                                                                         | The symbol used before displaying the compiler details |
| `detect_extensions` | `['cpp', 'cc', 'cxx', 'c++', 'hpp', 'hh', 'hxx', 'h++', 'tcc']`                  | 触发此组件的扩展名                                              |
| `detect_files`      | `[]`                                                                             | 触发此组件的文件名                                              |
| `detect_folders`    | `[]`                                                                             | 触发此组件的文件夹                                              |
| `commands`          | `[ [ 'c++', '--version' ], [ 'g++', '--version' ], [ 'clang++', '--version' ] ]` | How to detect what the compiler is                     |
| `style`             | `'bold 149'`                                                                     | 此组件的样式。                                                |
| `disabled`          | `true`                                                                           | Disables the `cpp` module.                             |

### 变量

| 字段      | 示例      | 描述          |
| ------- | ------- | ----------- |
| name    | clang++ | 编译器的名称      |
| version | 13.0.0  | 编译器的版本      |
| symbol  |         | `symbol`对应值 |
| style   |         | `style`对应值  |

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycpp', '--version']`. Starship will try executing each command until it gets a result on STDOUT.

If a C++ compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### 示例

```toml
# ~/.config/starship.toml

[cpp]
disabled = false
format = 'via [$name $version]($style)'
```

## 字符

`character` 组件用于在您输入终端的文本旁显示一个字符（通常是一个箭头）。

这个字符可以告诉您上一个命令是否执行成功。 有两种方式：

- 更换颜色（`red`/`green`）
- 更换字符（`❯`/`✖`）

默认情况下只会更改颜色。 如果您也想更换字符，请查看 [此示例](#with-custom-error-shape)。

> [!WARNING] `vimcmd_symbol` is only supported in cmd, fish and zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol` are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

### 配置项

| 选项                          | 默认值                  | 描述                                                                                      |
| --------------------------- | -------------------- | --------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | 用在输入文本之前的格式字符串。                                                                         |
| `success_symbol`            | `'[❯](bold green)'`  | 当前一条命令执行成功时，用在输入文本之前的格式字符串。                                                             |
| `error_symbol`              | `'[❯](bold red)'`    | 当前一条命令执行失败时，用在输入文本之前的格式字符串。                                                             |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | The format string used before the text input if the shell is in vim normal mode.        |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim `replace_one` mode. |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim replace mode.       |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | The format string used before the text input if the shell is in vim visual mode.        |
| `disabled`                  | `false`              | 禁用 `character` 组件。                                                                      |

### 变量

| 字段     | 示例 | 描述                                                                                                       |
| ------ | -- | -------------------------------------------------------------------------------------------------------- |
| symbol |    | A mirror of either `success_symbol`, `error_symbol`, `vimcmd_symbol` or `vimcmd_replace_one_symbol` etc. |

### 示例

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

`cmake` 组件显示当前安装的 [CMake](https://cmake.org/) 版本。 默认情况下，此组件将在满足以下任意条件时启用：

- 当前目录包含 `CMakeLists.txt` 文件
- 当前目录包含 `CMakeCache.txt` 文件

### 配置项

| 选项                  | 默认值                                    | 描述                                            |
| ------------------- | -------------------------------------- | --------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`   | 组件格式化模板。                                      |
| `version_format`    | `'v${raw}'`                            | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch` |
| `symbol`            | `'△ '`                                 | The symbol used before the version of cmake.  |
| `detect_extensions` | `[]`                                   | Which extensions should trigger this module   |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | Which filenames should trigger this module    |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module      |
| `style`             | `'bold blue'`                          | 此组件的样式。                                       |
| `disabled`          | `false`                                | Disables the `cmake` module.                  |

### 变量

| 字段        | 示例        | 描述          |
| --------- | --------- | ----------- |
| version   | `v3.17.3` | cmake 版本    |
| symbol    |           | `symbol`对应值 |
| style\* |           | `style`对应值  |

*: 此变量只能作为样式字符串的一部分使用

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL. By default, the module will be shown if any of the following conditions are met:

- The current directory contains any files ending in `.cob` or `.COB`
- The current directory contains any files ending in `.cbl` or `.CBL`

### 配置项

| 选项                  | 默认值                                  | 描述                                                      |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| `symbol`            | `'⚙️ '`                              | The symbol used before displaying the version of COBOL. |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                                |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`           |
| `style`             | `'bold blue'`                        | 此组件的样式。                                                 |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | 触发此组件的扩展名                                               |
| `detect_files`      | `[]`                                 | 触发此组件的文件名                                               |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                               |
| `disabled`          | `false`                              | Disables the `cobol` module.                            |

### 变量

| 字段        | 示例         | 描述          |
| --------- | ---------- | ----------- |
| version   | `v3.1.2.0` | `cobol` 版本  |
| symbol    |            | `symbol`对应值 |
| style\* |            | `style`对应值  |

*: 此变量只能作为样式字符串的一部分使用

## Command Duration 命令执行时长

`cmd_duration` 组件显示上一个命令执行的用时。 此组件只在命令执行时间长于两秒时显示，或者当其 `min_time` 字段被设置时，按此值为执行时间的显示下限。

> [!WARNING] Do not hook the DEBUG trap in Bash
> 
> If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). 只需要在执行 `eval $(starship init $0)` 前简单地定义 `preexec_functions` 和 `precmd_functions` 两个列表，就可以照常运行了。

### 配置项

| 选项                     | 默认值                           | 描述                                                                                                                                                                |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | 显示此组件所需的最短执行时长（单位：毫秒）。                                                                                                                                            |
| `show_milliseconds`    | `false`                       | 除了秒数外在执行时长中额外显示毫秒。                                                                                                                                                |
| `format`               | `'took [$duration]($style) '` | 组件格式化模板。                                                                                                                                                          |
| `style`                | `'bold yellow'`               | 此组件的样式。                                                                                                                                                           |
| `disabled`             | `false`                       | 禁用 `cmd_duration` 组件。                                                                                                                                             |
| `show_notifications`   | `false`                       | 当命令完成时显示桌面通知。                                                                                                                                                     |
| `min_time_to_notify`   | `45_000`                      | 要显示通知的最短执行时长（单位：毫秒）。                                                                                                                                              |
| `notification_timeout` |                               | Duration to show notification for (in milliseconds). If unset, notification timeout will be determined by daemon. Not all notification daemons honor this option. |

### 变量

| 字段        | 示例       | 描述                                      |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | The time it took to execute the command |
| style\* |          | `style`对应值                              |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

> [!TIP] This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`. If you use [pixi](https://pixi.sh), you can disable pixi's prompt modifier by running `pixi config set shell.change-ps1 false`.

### 配置项

| 选项                  | 默认值                                    | 描述                                                                                                                                   |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length` | `1`                                    | 如果这个 conda 环境是通过 `conda create -p [path]` 创建的，环境路径的目录深度应该被截断到此数量。 `0` 表示不用截断。 另请参阅 [`directory`](#directory) 组件。                     |
| `symbol`            | `'🅒 '`                                 | 在环境名之前显示的符号。                                                                                                                         |
| `style`             | `'bold green'`                         | 此组件的样式。                                                                                                                              |
| `format`            | `'via [$symbol$environment]($style) '` | 组件格式化模板。                                                                                                                             |
| `ignore_base`       | `true`                                 | 激活时忽略 `base` 环境。                                                                                                                     |
| `detect_env_vars`   | `["!PIXI_ENVIRONMENT_NAME"]`           | Which environment variable(s) should trigger this module. If it's a pixi environment, this module is not being triggered by default. |
| `disabled`          | `false`                                | 禁用 `conda` 组件。                                                                                                                       |

### 变量

| 字段          | 示例           | 描述          |
| ----------- | ------------ | ----------- |
| environment | `astronauts` | 当前 conda 环境 |
| symbol      |              | `symbol`对应值 |
| style\*   |              | `style`对应值  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## Container

The `container` module displays a symbol and container name, if inside a container.

### 配置项

| 选项         | 默认值                                | 描述                                        |
| ---------- | ---------------------------------- | ----------------------------------------- |
| `symbol`   | `'⬢'`                              | The symbol shown, when inside a container |
| `style`    | `'bold red dimmed'`                | 此组件的样式。                                   |
| `format`   | `'[$symbol \[$name\]]($style) '` | 组件格式化模板。                                  |
| `disabled` | `false`                            | Disables the `container` module.          |

### 变量

| 字段        | 示例                  | 描述                        |
| --------- | ------------------- | ------------------------- |
| name      | `fedora-toolbox:35` | The name of the container |
| symbol    |                     | `symbol`对应值               |
| style\* |                     | `style`对应值                |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含一个 `shard.yml` 文件
- 当前目录包含一个 `.cr` 文件

### 配置项

| 选项                  | 默认值                                  | 描述                                                        |
| ------------------- | ------------------------------------ | --------------------------------------------------------- |
| `symbol`            | `'🔮 '`                               | The symbol used before displaying the version of crystal. |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                                  |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`             |
| `style`             | `'bold red'`                         | 此组件的样式。                                                   |
| `detect_extensions` | `['cr']`                             | 触发此组件的扩展名                                                 |
| `detect_files`      | `['shard.yml']`                      | 触发此组件的文件名                                                 |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                                 |
| `disabled`          | `false`                              | Disables the `crystal` module.                            |

### 变量

| 字段        | 示例        | 描述                       |
| --------- | --------- | ------------------------ |
| version   | `v0.32.1` | The version of `crystal` |
| symbol    |           | `symbol`对应值              |
| style\* |           | `style`对应值               |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers) SDK version when you are in the root directory of your Daml project. The `sdk-version` in the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION` environment variable. 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含一个 `daml.yaml` 文件

### 配置项

| 选项                  | 默认值                                  | 描述                                              |
| ------------------- | ------------------------------------ | ----------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                        |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`   |
| `symbol`            | `'Λ '`                               | A format string representing the symbol of Daml |
| `style`             | `'bold cyan'`                        | 此组件的样式。                                         |
| `detect_extensions` | `[]`                                 | 触发此组件的扩展名                                       |
| `detect_files`      | `['daml.yaml']`                      | 触发此组件的文件名                                       |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                       |
| `disabled`          | `false`                              | Disables the `daml` module.                     |

### 变量

| 字段        | 示例       | 描述                    |
| --------- | -------- | --------------------- |
| version   | `v2.2.0` | The version of `daml` |
| symbol    |          | `symbol`对应值           |
| style\* |          | `style`对应值            |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- 当前目录包含 `pubspec.yaml`，`pubspec.yml` 或 `pubspec.lock` 文件

### 配置项

| 选项                  | 默认值                                               | 描述                                              |
| ------------------- | ------------------------------------------------- | ----------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`              | 组件格式化模板。                                        |
| `version_format`    | `'v${raw}'`                                       | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`   |
| `symbol`            | `'🎯 '`                                            | A format string representing the symbol of Dart |
| `detect_extensions` | `['dart']`                                        | 触发此组件的扩展名                                       |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | 触发此组件的文件名                                       |
| `detect_folders`    | `['.dart_tool']`                                  | 触发此组件的文件夹                                       |
| `style`             | `'bold blue'`                                     | 此组件的样式。                                         |
| `disabled`          | `false`                                           | Disables the `dart` module.                     |

### 变量

| 字段        | 示例       | 描述                    |
| --------- | -------- | --------------------- |
| version   | `v2.8.4` | The version of `dart` |
| symbol    |          | `symbol`对应值           |
| style\* |          | `style`对应值            |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

`deno` 组件显示当前安装的 [Deno](https://deno.land/) 版本。 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### 配置项

| 选项                  | 默认值                                                                                  | 描述                                            |
| ------------------- | ------------------------------------------------------------------------------------ | --------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | 组件格式化模板。                                      |
| `version_format`    | `'v${raw}'`                                                                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch` |
| `symbol`            | `'🦕 '`                                                                               | 用于表示 Deno 的格式化字符串                             |
| `detect_extensions` | `[]`                                                                                 | 触发此组件的扩展名                                     |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | 触发此组件的文件名                                     |
| `detect_folders`    | `[]`                                                                                 | 触发此组件的文件夹                                     |
| `style`             | `'green bold'`                                                                       | 此组件的样式。                                       |
| `disabled`          | `false`                                                                              | 禁用 `deno` 组件                                  |

### 变量

| 字段        | 示例       | 描述          |
| --------- | -------- | ----------- |
| version   | `v1.8.3` | `deno` 版本   |
| symbol    |          | `symbol`对应值 |
| style\* |          | `style`对应值  |

### 示例

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Directory 目录

`directory` 组件显示当前目录的路径，显示的路径会截断到三个父目录以内。 如果您处于一个 git 仓库中，显示的路径则最多会截断到该仓库的根目录。

当使用 `fish_style_pwd_dir_length` 选项时，不会隐藏截断的路径，而是会根据该选项值简写后的每个路径。

例如，对于 `~/Dev/Nix/nixpkgs/pkgs`，其中 `nixpkgs` 是 git 仓库根目录，fish 风格相关选项设置为 `1`。 您将会看到 `~/D/N/nixpkgs/pkgs`，而在设置 fish 风格之前，当前路径将显示成 `nixpkgs/pkgs`。

### 配置项

| 选项                       | 默认值                                                                                                                          | 描述                                                                                                         |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | 当前目录路径被截断后最多保留的父目录数量。                                                                                      |
| `truncate_to_repo`       | `true`                                                                                                                       | 是否只截断到您当前处于的 git 仓库根目录下。                                                                                   |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | 组件格式化模板。                                                                                                   |
| `style`                  | `'bold cyan'`                                                                                                                | 此组件的样式。                                                                                                    |
| `disabled`               | `false`                                                                                                                      | 禁用 `directory` 组件。                                                                                         |
| `read_only`              | `'🔒'`                                                                                                                        | 表明当前目录只读的符号。                                                                                               |
| `read_only_style`        | `'red'`                                                                                                                      | 只读符号的样式。                                                                                                   |
| `truncation_symbol`      | `''`                                                                                                                         | The symbol to prefix to truncated paths. eg: '…/'                                                          |
| `before_repo_root_style` |                                                                                                                              | The style for the path segment above the root of the git repo. The default value is equivalent to `style`. |
| `repo_root_style`        |                                                                                                                              | The style for the root of the git repo. The default value is equivalent to `style`.                        |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | The format of a git repo when `before_repo_root_style` and `repo_root_style` is defined.                   |
| `home_symbol`            | `'~'`                                                                                                                        | The symbol indicating home directory.                                                                      |
| `use_os_path_sep`        | `true`                                                                                                                       | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows)                    |

<details>
<summary>此组件有几个高级配置选项来控制当前目录路径的显示方式。</summary>

| Advanced Option             | 默认值    | 描述                                                                                                                                                                     |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | An Array or table of substitutions to be made to the path.                                                                                                             |
| `fish_style_pwd_dir_length` | `0`    | 使用 fish shell 当前目录路径逻辑时每个省略目录名使用的字符数。                                                                                                                                  |
| `use_logical_path`          | `true` | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories of Java. Note that this will disable the fish style PWD. It takes an array of the following key/value pairs:

| Value   | 类型      | 描述                                       |
| ------- | ------- | ---------------------------------------- |
| `from`  | String  | The value to substitute                  |
| `to`    | String  | The replacement for that value, if found |
| `regex` | Boolean | (Optional) Whether `from` is a regex     |

By using `regex = true`, you can use [Rust's regular expressions](https://docs.rs/regex/latest/regex/#syntax) in `from`. For instance you can replace every slash except the first with the following:

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

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### 变量

| 字段        | 示例                    | 描述         |
| --------- | --------------------- | ---------- |
| path      | `'D:/Projects'`       | 当前目录的路径    |
| style\* | `'black bold dimmed'` | `style`对应值 |

*: 此变量只能作为样式字符串的一部分使用

<details>
<summary>The git repos have additional variables.</summary>

Let us consider the path `/path/to/home/git_repo/src/lib`

| 字段                 | 示例                    | 描述                                      |
| ------------------ | --------------------- | --------------------------------------- |
| before_root_path | `'/path/to/home/'`    | The path before git root directory path |
| repo_root          | `'git_repo'`          | The git root directory name             |
| path               | `'/src/lib'`          | The remaining path                      |
| style              | `'black bold dimmed'` | `style`对应值                              |
| repo_root_style  | `'underline white'`   | Style for git root directory name       |

</details>

### 示例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Direnv

The `direnv` module shows the status of the current rc file if one is present. The status includes the path to the rc file, whether it is loaded, and whether it has been allowed by `direnv`.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                  | 默认值                                    | 描述                                                      |
| ------------------- | -------------------------------------- | ------------------------------------------------------- |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | 组件格式化模板。                                                |
| `symbol`            | `'direnv '`                            | The symbol used before displaying the direnv context.   |
| `style`             | `'bold orange'`                        | 此组件的样式。                                                 |
| `disabled`          | `true`                                 | Disables the `direnv` module.                           |
| `detect_extensions` | `[]`                                   | 触发此组件的扩展名                                               |
| `detect_files`      | `['.envrc']`                           | 触发此组件的文件名                                               |
| `detect_folders`    | `[]`                                   | 触发此组件的文件夹                                               |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | Which environment variables should trigger this module. |
| `allowed_msg`       | `'allowed'`                            | The message displayed when an rc file is allowed.       |
| `not_allowed_msg`   | `'not allowed'`                        | The message displayed when an rc file is not_allowed.   |
| `denied_msg`        | `'denied'`                             | The message displayed when an rc file is denied.        |
| `loaded_msg`        | `'loaded'`                             | The message displayed when an rc file is loaded.        |
| `unloaded_msg`      | `'not loaded'`                         | The message displayed when an rc file is not loaded.    |

### 变量

| 字段        | 示例                  | 描述                                      |
| --------- | ------------------- | --------------------------------------- |
| loaded    | `loaded`            | Whether the current rc file is loaded.  |
| allowed   | `denied`            | Whether the current rc file is allowed. |
| rc_path   | `/home/test/.envrc` | The current rc file path.               |
| symbol    |                     | `symbol`对应值.                            |
| style\* | `red bold`          | `style`对应值.                             |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default` or `desktop-linux`, or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or `DOCKER_CONTEXT` environment variables are set (as they are meant to override the context in use).

### 配置项

| 选项                  | 默认值                                                                                          | 描述                                                                                |
| ------------------- | -------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$context]($style) '`                                                           | 组件格式化模板。                                                                          |
| `symbol`            | `'🐳 '`                                                                                       | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                                                       | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                                                         | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `['compose.yml', 'compose.yaml', 'docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                                                         | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `'blue bold'`                                                                                | 此组件的样式。                                                                           |
| `disabled`          | `false`                                                                                      | Disables the `docker_context` module.                                             |

### 变量

| 字段        | 示例             | 描述                         |
| --------- | -------------- | -------------------------- |
| context   | `test_context` | The current docker context |
| symbol    |                | `symbol`对应值                |
| style\* |                | `style`对应值                 |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. 如果当前目录已被绑定了一个版本的 SDK，则显示被帮定的版本。 否则此组件将显示最新安装的 SDK 版本。

By default this module will only be shown in your prompt when one or more of the following files are present in the current directory:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

You'll also need the .NET Core SDK installed in order to use it correctly.

在内部，此组件使用自己的版本检测机制。 一般来说此组件是直接执行 `dotnet --version` 的两倍快，但当你的 .NET 项目使用了不常见的目录布局时此组件可能显示一个错误的版本。 如果相比于速度您更需要正确的版本号，您可以在组件设置中设置 `heuristic = false` 来禁用该机制。

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>) when there is a `.csproj` file in the current directory.

### 配置项

| 选项                  | 默认值                                                                                                     | 描述                                            |
| ------------------- | ------------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                           | 组件格式化模板。                                      |
| `version_format`    | `'v${raw}'`                                                                                             | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch` |
| `symbol`            | `'.NET '`                                                                                               | 这个字段的内容会显示在当前 .NET 版本之前。                      |
| `heuristic`         | `true`                                                                                                  | 使用更快的版本探测机制以保证 starship 的运行速度。                |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | 触发此组件的扩展名                                     |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | 触发此组件的文件名                                     |
| `detect_folders`    | `[]`                                                                                                    | 触发此组件的文件夹                                     |
| `style`             | `'bold blue'`                                                                                           | 此组件的样式。                                       |
| `disabled`          | `false`                                                                                                 | 禁用 `dotnet` 组件。                               |

### 变量

| 字段        | 示例               | 描述                                                                 |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | `symbol`对应值                                                        |
| style\* |                  | `style`对应值                                                         |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/). 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含一个 `mix.exs` 文件.

### 配置项

| 选项                  | 默认值                                                         | 描述                                                              |
| ------------------- | ----------------------------------------------------------- | --------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                               |
| `version_format`    | `'v${raw}'`                                                 | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`                   |
| `symbol`            | `'💧 '`                                                      | The symbol used before displaying the version of Elixir/Erlang. |
| `detect_extensions` | `[]`                                                        | 触发此组件的扩展名                                                       |
| `detect_files`      | `['mix.exs']`                                               | 触发此组件的文件名                                                       |
| `detect_folders`    | `[]`                                                        | 触发此组件的文件夹                                                       |
| `style`             | `'bold purple'`                                             | 此组件的样式。                                                         |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                   |

### 变量

| 字段          | 示例      | 描述                          |
| ----------- | ------- | --------------------------- |
| version     | `v1.10` | The version of `elixir`     |
| otp_version |         | The otp version of `elixir` |
| symbol      |         | `symbol`对应值                 |
| style\*   |         | `style`对应值                  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含一个 `elm.json` 文件
- 当前目录包含一个 `elm-package.json` 文件
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains `*.elm` files

### 配置项

| 选项                  | 默认值                                                | 描述                                              |
| ------------------- | -------------------------------------------------- | ----------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`               | 组件格式化模板。                                        |
| `version_format`    | `'v${raw}'`                                        | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`   |
| `symbol`            | `'🌳 '`                                             | A format string representing the symbol of Elm. |
| `detect_extensions` | `['elm']`                                          | 触发此组件的扩展名                                       |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | 触发此组件的文件名                                       |
| `detect_folders`    | `['elm-stuff']`                                    | 触发此组件的文件夹                                       |
| `style`             | `'cyan bold'`                                      | 此组件的样式。                                         |
| `disabled`          | `false`                                            | Disables the `elm` module.                      |

### 变量

| 字段        | 示例        | 描述                   |
| --------- | --------- | -------------------- |
| version   | `v0.19.1` | The version of `elm` |
| symbol    |           | `symbol`对应值          |
| style\* |           | `style`对应值           |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variables. 此组件只有满足以下条件之一时才会被显示：

- 设置的 `variable` 是一个已存在的环境变量
- 未定义 `variable`，但定义了 `default`

> [!TIP] The order in which env_var modules are shown can be individually set by including `${env_var.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `env_var` module will simply show all env_var modules in the order they were defined.

> [!TIP] Multiple environmental variables can be displayed by using a `.`. (see example) If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.
> 
> Example: following configuration will display value of USER environment variable
> 
> ```toml
> 
> # ~/.config/starship.toml
> 
> [env_var.USER] default = 'unknown user' ```

### 配置项

| 选项         | 默认值                                   | 描述                                                                           |
| ---------- | ------------------------------------- | ---------------------------------------------------------------------------- |
| `symbol`   | `""`                                  | 这个字段的内容会显示在环境变量值之前。                                                          |
| `variable` |                                       | 要显示的环境变量。                                                                    |
| `default`  |                                       | 所选变量未定义时显示的默认值。                                                              |
| `format`   | `"with [$symbol$env_value]($style) "` | 组件格式化模板。                                                                     |
| `描述`       | `"<env_var module>"`            | The description of the module that is shown when running `starship explain`. |
| `disabled` | `false`                               | 禁用 `env_var` 组件。                                                             |
| `style`    | `"black bold dimmed"`                 | 此组件的样式。                                                                      |

### 变量

| 字段        | 示例                                          | 描述                                         |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | `symbol`对应值                                |
| style\* |                                             | `style`对应值                                 |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/). 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含一个 `rebar.config` 文件.
- 当前目录包含一个 `erlang.mk` 文件.

### 配置项

| 选项                  | 默认值                                  | 描述                                                       |
| ------------------- | ------------------------------------ | -------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                                 |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`            |
| `symbol`            | `' '`                               | The symbol used before displaying the version of erlang. |
| `style`             | `'bold red'`                         | 此组件的样式。                                                  |
| `detect_extensions` | `[]`                                 | 触发此组件的扩展名                                                |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | 触发此组件的文件名                                                |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                                |
| `disabled`          | `false`                              | Disables the `erlang` module.                            |

### 变量

| 字段        | 示例        | 描述                      |
| --------- | --------- | ----------------------- |
| version   | `v22.1.3` | The version of `erlang` |
| symbol    |           | `symbol`对应值             |
| style\* |           | `style`对应值              |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Fennel

The `fennel` module shows the currently installed version of [Fennel](https://fennel-lang.org). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a file with the `.fnl` extension

### 配置项

| 选项                  | 默认值                                  | 描述                                                       |
| ------------------- | ------------------------------------ | -------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                                 |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`            |
| `symbol`            | `'🧅 '`                               | The symbol used before displaying the version of fennel. |
| `style`             | `'bold green'`                       | 此组件的样式。                                                  |
| `detect_extensions` | `['fnl']`                            | 触发此组件的扩展名                                                |
| `detect_files`      | `[]`                                 | 触发此组件的文件名                                                |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                                |
| `disabled`          | `false`                              | Disables the `fennel` module.                            |

### 变量

| 字段        | 示例       | 描述                      |
| --------- | -------- | ----------------------- |
| version   | `v1.2.1` | The version of `fennel` |
| symbol    |          | `symbol`对应值             |
| style\* |          | `style`对应值              |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```

## Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are present in a line they will split the space evenly between them. This is useful for aligning other modules.

### 配置项

| 选项         | 默认值            | 描述                                |
| ---------- | -------------- | --------------------------------- |
| `symbol`   | `'.'`          | The symbol used to fill the line. |
| `style`    | `'bold black'` | 此组件的样式。                           |
| `disabled` | `false`        | Disables the `fill` module        |

### 示例

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

### 配置项

| 选项                  | 默认值                                                                                                                         | 描述                                                        |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| `symbol`            | `' '`                                                                                                                      | The symbol used before displaying the version of Fortran. |
| `format`            | `'via [$symbol($version )]($style)'`                                                                                        | 组件格式化模板。                                                  |
| `version_format`    | `'${raw}'`                                                                                                                  | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`             |
| `style`             | `'bold purple'`                                                                                                             | 此组件的样式。                                                   |
| `detect_extensions` | `['f', 'F', 'for', 'FOR', 'ftn', 'FTN', 'f77', 'F77', 'f90', 'F90', 'f95', 'F95','f03', 'F03', 'f08', 'F08', 'f18', 'F18']` | 触发此组件的扩展名                                                 |
| `detect_files`      | `['fpm.toml']`                                                                                                              | 触发此组件的文件名                                                 |
| `detect_folders`    | `[]`                                                                                                                        | 触发此组件的文件夹                                                 |
| `commands`          | `[ [ 'gfortran', '--version' ], [ 'flang', '--version' ], [ 'flang-new', '--version' ] ]`                                   | How to detect what the compiler is                        |
| `disabled`          | `false`                                                                                                                     | Disables the `fortran` module.                            |

### 变量

| 字段        | 示例       | 描述                                  |
| --------- | -------- | ----------------------------------- |
| name      | gfortran | 编译器的名称                              |
| version   | `14.2.0` | The version of the Fortran compiler |
| symbol    |          | `symbol`对应值                         |
| style\* |          | `style`对应值                          |

*: 此变量只能作为样式字符串的一部分使用

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['myfortran', '--version']`. Starship will try executing each command until it gets a result on STDOUT.

If a Fortran compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/).

## Fossil Branch

The `fossil_branch` module shows the name of the active branch of the check-out in your current directory.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                  | 默认值                              | 描述                                                                                 |
| ------------------- | -------------------------------- | ---------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | 组件格式化模板。 Use `'$branch'` to refer to the current branch name.                      |
| `symbol`            | `' '`                           | The symbol used before the branch name of the check-out in your current directory. |
| `style`             | `'bold purple'`                  | 此组件的样式。                                                                            |
| `truncation_length` | `2^63 - 1`                       | Truncates a Fossil branch name to `N` graphemes                                    |
| `truncation_symbol` | `'…'`                            | 此字段的内容用来表示分支名称被截断。 You can use `''` for no symbol.                                 |
| `disabled`          | `true`                           | Disables the `fossil_branch` module.                                               |

### 变量

| 字段        | 示例      | 描述                       |
| --------- | ------- | ------------------------ |
| branch    | `trunk` | The active Fossil branch |
| symbol    |         | `symbol`对应值              |
| style\* |         | `style`对应值               |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```

## Fossil Metrics

The `fossil_metrics` module will show the number of added and deleted lines in the check-out in your current directory. At least v2.14 (2021-01-20) of Fossil is required.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                   | 默认值                                                          | 描述                                    |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | 组件格式化模板。                              |
| `added_style`        | `'bold green'`                                               | The style for the added count.        |
| `deleted_style`      | `'bold red'`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `disabled`           | `true`                                                       | Disables the `fossil_metrics` module. |

### 变量

| 字段                | 示例  | 描述                                          |
| ----------------- | --- | ------------------------------------------- |
| added             | `1` | The current number of added lines           |
| deleted           | `2` | The current number of deleted lines         |
| added_style\*   |     | Mirrors the value of option `added_style`   |
| deleted_style\* |     | Mirrors the value of option `deleted_style` |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[fossil_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

When the module is enabled it will always be active, unless `detect_env_vars` has been set in which case the module will only be active when one of the environment variables has been set.

### 配置项

| 选项                | 默认值                                                        | 描述                                                               |
| ----------------- | ---------------------------------------------------------- | ---------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | 组件格式化模板。                                                         |
| `symbol`          | `'☁️  '`                                                   | The symbol used before displaying the current GCP profile.       |
| `region_aliases`  | `{}`                                                       | Table of region aliases to display in addition to the GCP name.  |
| `project_aliases` | `{}`                                                       | Table of project aliases to display in addition to the GCP name. |
| `detect_env_vars` | `[]`                                                       | Which environmental variables should trigger this module         |
| `style`           | `'bold blue'`                                              | 此组件的样式。                                                          |
| `disabled`        | `false`                                                    | Disables the `gcloud` module.                                    |

### 变量

| 字段        | 示例            | 描述                                                                 |
| --------- | ------------- | ------------------------------------------------------------------ |
| region    | `us-central1` | The current GCP region                                             |
| account   | `foo`         | The current GCP profile                                            |
| domain    | `example.com` | The current GCP profile domain                                     |
| project   |               | The current GCP project                                            |
| active    | `default`     | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |               | `symbol`对应值                                                        |
| style\* |               | `style`对应值                                                         |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

`git_branch` 组件显示当前目录的 git 仓库的活动分支。

### 配置项

| 选项                   | 默认值                                               | 描述                                                                                   |
| -------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `always_show_remote` | `false`                                           | Shows the remote tracking branch name, even if it is equal to the local branch name. |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | 组件格式化模板。 Use `'$branch'` to refer to the current branch name.                        |
| `symbol`             | `' '`                                            | A format string representing the symbol of git branch.                               |
| `style`              | `'bold purple'`                                   | 此组件的样式。                                                                              |
| `truncation_length`  | `2^63 - 1`                                        | Truncates a git branch to `N` graphemes.                                             |
| `truncation_symbol`  | `'…'`                                             | 此字段的内容用来表示分支名称被截断。 You can use `''` for no symbol.                                   |
| `only_attached`      | `false`                                           | Only show the branch name when not in a detached `HEAD` state.                       |
| `ignore_branches`    | `[]`                                              | A list of names to avoid displaying. Useful for 'master' or 'main'.                  |
| `ignore_bare_repo`   | `false`                                           | Do not show when in a bare repo.                                                     |
| `disabled`           | `false`                                           | 禁用 `git_branch` 组件。                                                                  |

### 变量

| 字段            | 示例       | 描述                                                                                                     |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | `symbol`对应值                                                                                            |
| style\*     |          | `style`对应值                                                                                             |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

### 配置项

| 选项                   | 默认值                            | 描述                                                                                   |
| -------------------- | ------------------------------ | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                            | 显示的 git 提交哈希值的长度。                                                                    |
| `format`             | `'[\($hash$tag\)]($style) '` | 组件格式化模板。                                                                             |
| `style`              | `'bold green'`                 | 此组件的样式。                                                                              |
| `only_detached`      | `true`                         | Only show git commit hash when in detached `HEAD` state                              |
| `tag_disabled`       | `true`                         | Disables showing tag info in `git_commit` module.                                    |
| `tag_max_candidates` | `0`                            | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `' 🏷  '`                       | Tag symbol prefixing the info shown                                                  |
| `disabled`           | `false`                        | 禁用 `git_commit` 组件。                                                                  |

### 变量

| 字段        | 示例        | 描述                                           |
| --------- | --------- | -------------------------------------------- |
| hash      | `b703eb3` | The current git commit hash                  |
| tag       | `v1.0.0`  | The tag name if showing tag info is enabled. |
| style\* |           | `style`对应值                                   |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## Git State

`git_state` 组件会显示当前目录在哪个 git 仓库中，以及正在进行的操作，例如：_REBASING_，_BISECTING_ 等。 进度信息（例如 REBASING 3/10）如果存在则也会被显示。

### 配置项

| 选项             | 默认值                                                             | 描述                                                                                      |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `'MERGING'`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `'REVERTING'`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `'BISECTING'`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `'AM'`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `'AM/REBASE'`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `'bold yellow'`                                                 | 此组件的样式。                                                                                 |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | 组件格式化模板。                                                                                |
| `disabled`     | `false`                                                         | 禁用 `git_state` 组件。                                                                      |

### 变量

| 字段               | 示例         | 描述                             |
| ---------------- | ---------- | ------------------------------ |
| state            | `REBASING` | The current state of the repo  |
| progress_current | `1`        | The current operation progress |
| progress_total   | `2`        | The total operation progress   |
| style\*        |            | `style`对应值                     |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in the current git repository.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                   | 默认值                                                          | 描述                                    |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `'bold green'`                                               | The style for the added count.        |
| `deleted_style`      | `'bold red'`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | 组件格式化模板。                              |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.    |
| `忽略子组件`              | `false`                                                      | 忽略子模块的更改                              |

### 变量

| 字段                | 示例  | 描述                                          |
| ----------------- | --- | ------------------------------------------- |
| added             | `1` | The current number of added lines           |
| deleted           | `2` | The current number of deleted lines         |
| added_style\*   |     | Mirrors the value of option `added_style`   |
| deleted_style\* |     | Mirrors the value of option `deleted_style` |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git Status

`git_status`组件通过相应的符号显示您当前目录中 git 仓库的状态。

> [!TIP] The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment. You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

### 配置项

| 选项                     | 默认值                                             | 描述                                                                                                          |
| ---------------------- | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`               | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status`                                                                         |
| `conflicted`           | `'='`                                           | The format shown when this branch has merge conflicts.                                                      |
| `ahead`                | `'⇡'`                                           | The format shown when this branch is ahead of the branch being tracked.                                     |
| `behind`               | `'⇣'`                                           | The format shown when this branch is behind the branch being tracked.                                       |
| `diverged`             | `'⇕'`                                           | The format shown when this branch has diverged from the branch being tracked.                               |
| `up_to_date`           | `''`                                            | The format shown when this branch is up to date with the branch being tracked.                              |
| `untracked`            | `'?'`                                           | The format shown when there are untracked files in the working directory.                                   |
| `stashed`              | `'\$'`                                         | The format shown when a stash exists for the local repository.                                              |
| `modified`             | `'!'`                                           | The format shown when there are file modifications in the working directory.                                |
| `staged`               | `'+'`                                           | The format shown when a new file has been added to the staging area.                                        |
| `renamed`              | `'»'`                                           | The format shown when a renamed file has been added to the staging area.                                    |
| `deleted`              | `'✘'`                                           | The format shown when a file's deletion has been added to the staging area.                                 |
| `typechanged`          | `""`                                            | The format shown when a file's type has been changed in the staging area.                                   |
| `style`                | `'bold red'`                                    | 此组件的样式。                                                                                                     |
| `ignore_submodules`    | `false`                                         | 忽略子模块的更改.                                                                                                   |
| `worktree_added`       | `""`                                            | The format shown when a new file has been added in the working directory.                                   |
| `worktree_deleted`     | `""`                                            | The format shown when a file has been deleted in the working directory.                                     |
| `worktree_modified`    | `""`                                            | The format shown when a file has been modified in the working directory.                                    |
| `worktree_typechanged` | `""`                                            | The format shown when a file's type has been changed in the working directory.                              |
| `index_added`          | `""`                                            | The format shown when a new file has been added to the staging area.                                        |
| `index_deleted`        | `""`                                            | The format shown when a file has been deleted from the staging area.                                        |
| `index_modified`       | `""`                                            | The format shown when a file has been modified in the staging area.                                         |
| `index_typechanged`    | `""`                                            | The format shown when a file's type has been changed in the staging area.                                   |
| `disabled`             | `false`                                         | 禁用 `git_status` 组件。                                                                                         |
| `windows_starship`     |                                                 | Use this (Linux) path to a Windows Starship executable to render `git_status` when on Windows paths in WSL. |
| `use_git_executable`   | `false`                                         | Do not use `gitoxide` for computing the status, but use the `git` executable instead.                       |

### 变量

The following variables can be used in `format`:

| 字段                     | 描述                                                                                                            |
| ---------------------- | ------------------------------------------------------------------------------------------------------------- |
| `all_status`           | Shortcut for `$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked`.                     |
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
| `index_deleted`        | Displays `index_deleted` when a file has been deleted from the staging area.                                  |
| `index_modified`       | Displays `index_modified` when a file has been modified in the staging area.                                  |
| `index_typechanged`    | Displays `index_typechanged` when a file's type has been changed in the staging area.                         |
| style\*              | `style`对应值                                                                                                    |

*: 此变量只能作为样式字符串的一部分使用

The following variables can be used in `diverged`:

| 字段             | 描述                                             |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed`, `deleted`, `typechanged`, `worktree_added`, `worktree_deleted`, `worktree_modified`, `worktree_typechanged`, `index_added`, `index_deleted`, `index_modified`, and `index_typechanged`:

| 字段      | 描述        |
| ------- | --------- |
| `count` | 显示相应的文件数量 |

### 示例

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

The `gleam` module shows the currently installed version of [Gleam](https://gleam.run/). 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含 `gleam.toml` 文件
- The current directory contains a file with the `.gleam` extension

### 配置项

| 选项                  | 默认值                                  | 描述                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                          |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`     |
| `symbol`            | `'⭐ '`                               | A format string representing the symbol of Gleam. |
| `detect_extensions` | `['gleam']`                          | 触发此组件的扩展名                                         |
| `detect_files`      | `['gleam.toml']`                     | 触发此组件的文件名                                         |
| `style`             | `'bold #FFAFF3'`                     | 此组件的样式。                                           |
| `disabled`          | `false`                              | Disables the `gleam` module.                      |

### 变量

| 字段        | 示例       | 描述                     |
| --------- | -------- | ---------------------- |
| version   | `v1.0.0` | The version of `gleam` |
| symbol    |          | `symbol`对应值            |
| style\* |          | `style`对应值             |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[gleam]
format = 'via [⭐ $version](bold red) '
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含 `go.mod` 文件
- 当前目录包含 `go.sum` 文件
- 当前目录包含 `go.work` 文件
- 当前目录包含 `glide.yaml` 文件
- 当前目录包含 `Gopkg.yml` 文件
- 当前目录包含 `Gopkg.lock` 文件
- 当前目录包含 `.go-version` 文件
- 当前目录包含 `Godeps` 目录
- 当前目录包含使用 `.go` 扩展名的文件

### 配置项

| 选项                  | 默认值                                                                                       | 描述                                                                                                         |
| ------------------- | ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | 组件格式化模板。                                                                                                   |
| `version_format`    | `'v${raw}'`                                                                               | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`                                                              |
| `symbol`            | `'🐹 '`                                                                                    | A format string representing the symbol of Go.                                                             |
| `detect_extensions` | `['go']`                                                                                  | 触发此组件的扩展名                                                                                                  |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | 触发此组件的文件名                                                                                                  |
| `detect_folders`    | `['Godeps']`                                                                              | 触发此组件的文件夹                                                                                                  |
| `style`             | `'bold cyan'`                                                                             | 此组件的样式。                                                                                                    |
| `not_capable_style` | `'bold red'`                                                                              | The style for the module when the go directive in the go.mod file does not match the installed Go version. |
| `disabled`          | `false`                                                                                   | 禁用 `golang` 组件。                                                                                            |

### 变量

| 字段          | 示例        | 描述                                                                                                                                          |
| ----------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| version     | `v1.12.1` | The version of `go`                                                                                                                         |
| mod_version | `1.16`    | `go` version requirement as set in the go directive of `go.mod`. Will only show if the version requirement does not match the `go` version. |
| symbol      |           | `symbol`对应值                                                                                                                                 |
| style\*   |           | `style`对应值                                                                                                                                  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment. The module will be shown when inside a guix-shell environment.

### 配置项

| 选项         | 默认值                        | 描述                                                     |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `'via [$symbol]($style) '` | 组件格式化模板。                                               |
| `symbol`   | `'🐃 '`                     | A format string representing the symbol of guix-shell. |
| `style`    | `'yellow bold'`            | 此组件的样式。                                                |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### 变量

| 字段        | 示例 | 描述          |
| --------- | -- | ----------- |
| symbol    |    | `symbol`对应值 |
| style\* |    | `style`对应值  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

The `gradle` module shows the version of the [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) currently used in the project directory.

默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `gradle/wrapper/gradle-wrapper.properties` directory.
- The current directory contains a file ending with `.gradle` or `.gradle.kts`.

The `gradle` module is only able to read your Gradle Wrapper version from your config file, we don't execute your wrapper, because of the security concerns.

### 配置项

| 选项                  | 默认值                                  | 描述                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                              |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`         |
| `symbol`            | `'🅶 '`                               | A format string representing the symbol of Gradle.    |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | 触发此组件的扩展名                                             |
| `detect_files`      | `[]`                                 | 触发此组件的文件名                                             |
| `detect_folders`    | `['gradle']`                         | 触发此组件的文件夹                                             |
| `style`             | `'bold bright-cyan'`                 | 此组件的样式。                                               |
| `disabled`          | `false`                              | Disables the `gradle` module.                         |
| `recursive`         | `false`                              | Enables recursive finding for the `gradle` directory. |

### 变量

| 字段      | 示例       | 描述                      |
| ------- | -------- | ----------------------- |
| version | `v7.5.1` | The version of `gradle` |
| symbol  |          | `symbol`对应值             |
| style*  |          | `style`对应值              |

*: 此变量只能作为样式字符串的一部分使用

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含 `stack.yaml` 文件
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### 配置项

| 选项                  | 默认值                                  | 描述                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                           |
| `symbol`            | `'λ '`                               | A format string representing the symbol of Haskell |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | 触发此组件的扩展名                                          |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | 触发此组件的文件名                                          |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                          |
| `style`             | `'bold purple'`                      | 此组件的样式。                                            |
| `disabled`          | `false`                              | 禁用 `haskell` 组件。                                   |

### 变量

| 字段             | 示例          | 描述                                                                                      |
| -------------- | ----------- | --------------------------------------------------------------------------------------- |
| version        |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot       | `lts-18.12` | Currently selected Stack snapshot                                                       |
| ghc\_version | `9.2.1`     | Currently installed GHC version                                                         |
| symbol         |             | `symbol`对应值                                                                             |
| style\*      |             | `style`对应值                                                                              |

*: 此变量只能作为样式字符串的一部分使用

## Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- The current directory contains a `.haxelib` or a `haxe_libraries` directory
- The current directory contains a file with the `.hx` or `.hxml` extension

### 配置项

| 选项                  | 默认值                                                                                             | 描述                                               |
| ------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                            | 组件格式化模板。                                         |
| `version_format`    | `'v${raw}'`                                                                                     | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`    |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | 触发此组件的扩展名                                        |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | 触发此组件的文件名                                        |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | 触发此组件的文件夹                                        |
| `symbol`            | `'⌘ '`                                                                                          | A format string representing the symbol of Haxe. |
| `style`             | `'bold fg:202'`                                                                                 | 此组件的样式。                                          |
| `disabled`          | `false`                                                                                         | Disables the `haxe` module.                      |

### 变量

| 字段        | 示例       | 描述                    |
| --------- | -------- | --------------------- |
| version   | `v4.2.5` | The version of `haxe` |
| symbol    |          | `symbol`对应值           |
| style\* |          | `style`对应值            |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含一个 `helmfile.yaml` 文件
- The current directory contains a `Chart.yaml` file

### 配置项

| 选项                  | 默认值                                  | 描述                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                         |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`    |
| `detect_extensions` | `[]`                                 | 触发此组件的扩展名                                        |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | 触发此组件的文件名                                        |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                        |
| `symbol`            | `'⎈ '`                               | A format string representing the symbol of Helm. |
| `style`             | `'bold white'`                       | 此组件的样式。                                          |
| `disabled`          | `false`                              | Disables the `helm` module.                      |

### 变量

| 字段        | 示例       | 描述                    |
| --------- | -------- | --------------------- |
| version   | `v3.1.1` | The version of `helm` |
| symbol    |          | `symbol`对应值           |
| style\* |          | `style`对应值            |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## Hostname

`hostname` 组件显示系统主机名。

### 配置项

| 选项                | 默认值                                    | 描述                                                                                                                         |
| ----------------- | -------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`        | `true`                                 | 仅在连接到 SSH 会话时显示主机名。                                                                                                        |
| `ssh_symbol`      | `'🌐 '`                                 | A format string representing the symbol when connected to SSH session.                                                     |
| `trim_at`         | `'.'`                                  | 当主机名过长被截断时，会截断成第一次匹配该字符串之前的主机名。 `'.'` will stop after the first dot. `''` will disable any truncation.                     |
| `detect_env_vars` | `[]`                                   | Which environment variable(s) should trigger this module.                                                                  |
| `format`          | `'[$ssh_symbol$hostname]($style) in '` | 组件格式化模板。                                                                                                                   |
| `style`           | `'bold dimmed green'`                  | 此组件的样式。                                                                                                                    |
| `disabled`        | `false`                                | 禁用 `hostname` 组件。                                                                                                          |
| `aliases`         | `{}`                                   | Translate system hostnames to something else. If `trim_at` is specified, only the first part will be matched and replaced. |

### 变量

| 字段         | 示例         | 描述                                                    |
| ---------- | ---------- | ----------------------------------------------------- |
| hostname   | `computer` | The hostname of the computer                          |
| style\*  |            | `style`对应值                                            |
| ssh_symbol | `'🌏 '`     | The symbol to represent when connected to SSH session |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

`java` 组件显示当前安装的 [Java](https://www.oracle.com/java/) 版本。 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot`, or `.sdkmanrc` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### 配置项

| 选项                  | 默认值                                                                                                                   | 描述                                              |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | 组件格式化模板。                                        |
| `version_format`    | `'v${raw}'`                                                                                                           | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`   |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | 触发此组件的扩展名                                       |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | 触发此组件的文件名                                       |
| `detect_folders`    | `[]`                                                                                                                  | 触发此组件的文件夹                                       |
| `symbol`            | `'☕ '`                                                                                                                | A format string representing the symbol of Java |
| `style`             | `'red dimmed'`                                                                                                        | 此组件的样式。                                         |
| `disabled`          | `false`                                                                                                               | 禁用 `java` 组件。                                   |

### 变量

| 字段        | 示例    | 描述                    |
| --------- | ----- | --------------------- |
| version   | `v14` | The version of `java` |
| symbol    |       | `symbol`对应值           |
| style\* |       | `style`对应值            |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## Jobs

`jobs` 组件显示当前正在运行的任务数量。 仅当有后台任务运行时，此组件才会显示。 The module will show the number of jobs running if there are at least 2 jobs, or more than the `number_threshold` config value, if it exists. The module will show a symbol if there is at least 1 job, or more than the `symbol_threshold` config value, if it exists. You can set both values to 0 in order to _always_ show the symbol and number of jobs, even if there are 0 jobs running.

The default functionality is:

- 0 jobs -> Nothing is shown.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

> [!WARNING] This module is not supported on tcsh.

> [!WARNING] The `threshold` option is deprecated, but if you want to use it, the module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

### 配置项

| 选项                 | 默认值                           | 描述                                                                       |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | 如果超过此字段的值，显示任务数量。                                                        |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | 组件格式化模板。                                                                 |
| `symbol`           | `'✦'`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `'bold blue'`                 | 此组件的样式。                                                                  |
| `disabled`         | `false`                       | 禁用 `jobs` 组件。                                                            |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### 变量

| 字段        | 示例  | 描述                 |
| --------- | --- | ------------------ |
| number    | `1` | The number of jobs |
| symbol    |     | `symbol`对应值        |
| style\* |     | `style`对应值         |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### 配置项

| 选项                  | 默认值                                  | 描述                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                          |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`     |
| `detect_extensions` | `['jl']`                             | 触发此组件的扩展名                                         |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | 触发此组件的文件名                                         |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                         |
| `symbol`            | `'ஃ '`                               | A format string representing the symbol of Julia. |
| `style`             | `'bold purple'`                      | 此组件的样式。                                           |
| `disabled`          | `false`                              | Disables the `julia` module.                      |

### 变量

| 字段        | 示例       | 描述                     |
| --------- | -------- | ---------------------- |
| version   | `v1.4.0` | The version of `julia` |
| symbol    |          | `symbol`对应值            |
| style\* |          | `style`对应值             |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `.kt` or a `.kts` file

### 配置项

| 选项                  | 默认值                                  | 描述                                                                            |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                                                      |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`                                 |
| `detect_extensions` | `['kt', 'kts']`                      | 触发此组件的扩展名                                                                     |
| `detect_files`      | `[]`                                 | 触发此组件的文件名                                                                     |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                                                     |
| `symbol`            | `'🅺 '`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `'bold blue'`                        | 此组件的样式。                                                                       |
| `kotlin_binary`     | `'kotlin'`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### 变量

| 字段        | 示例        | 描述                      |
| --------- | --------- | ----------------------- |
| version   | `v1.4.21` | The version of `kotlin` |
| symbol    |           | `symbol`对应值             |
| style\* |           | `style`对应值              |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-context --namespace astronaut`. Similarly, the user and cluster can be set with `kubectl config set-context starship-context --user starship-user` and `kubectl config set-context starship-context --cluster starship-cluster`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。
> 
> When the module is enabled it will always be active, unless any of `detect_env_vars`, `detect_extensions`, `detect_files` or `detect_folders` have been set in which case the module will only be active in directories that match those conditions or one of the environmental variables has been set.

### 配置项

> [!WARNING] The `context_aliases` and `user_aliases` options are deprecated. Use `contexts` and the corresponding `context_alias` and `user_alias` options instead.

| 选项                  | 默认值                                                  | 描述                                                                    |
| ------------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`            | `'☸ '`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | 组件格式化模板。                                                              |
| `style`             | `'cyan bold'`                                        | 此组件的样式。                                                               |
| `context_aliases`*  | `{}`                                                 | Table of context aliases to display.                                  |
| `user_aliases`*     | `{}`                                                 | Table of user aliases to display.                                     |
| `detect_extensions` | `[]`                                                 | 触发此组件的扩展名                                                             |
| `detect_files`      | `[]`                                                 | 触发此组件的文件名                                                             |
| `detect_folders`    | `[]`                                                 | 触发此组件的文件夹                                                             |
| `detect_env_vars`   | `[]`                                                 | Which environmental variables should trigger this module              |
| `contexts`          | `[]`                                                 | Customized styles and symbols for specific contexts.                  |
| `disabled`          | `true`                                               | Disables the `kubernetes` module.                                     |

*: This option is deprecated, please add `contexts` with the corresponding `context_alias` and `user_alias` options instead.

To customize the style of the module for specific environments, use the following configuration as part of the `contexts` list:

| 字段                | 描述                                                                                       |
| ----------------- | ---------------------------------------------------------------------------------------- |
| `context_pattern` | **Required** Regular expression to match current Kubernetes context name.                |
| `user_pattern`    | Regular expression to match current Kubernetes user name.                                |
| `context_alias`   | Context alias to display instead of the full context name.                               |
| `user_alias`      | User alias to display instead of the full user name.                                     |
| `style`           | The style for the module when using this context. If not set, will use module's style.   |
| `symbol`          | The symbol for the module when using this context. If not set, will use module's symbol. |

Note that all regular expression are anchored with `^<pattern>$` and so must match the whole string. The `*_pattern` regular expressions may contain capture groups, which can be referenced in the corresponding alias via `$name` and `$N` (see example below and the [rust Regex::replace() documentation](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)).

### 变量

| 字段        | 示例                   | 描述                                       |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbol    |                      | `symbol`对应值                              |
| style\* |                      | `style`对应值                               |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

The `contexts` configuration option is used to customise what the current Kubernetes context name looks like (style and symbol) if the name matches the defined regular expression.

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

`line_break` 组件将提示分隔为两行。

### 配置项

| 选项         | 默认值     | 描述                          |
| ---------- | ------- | --------------------------- |
| `disabled` | `false` | 禁用 `line_break` 组件，使提示成为单行。 |

### 示例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Local IP

The `localip` module shows the IPv4 address of the primary network interface.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项         | 默认值                       | 描述                                                     |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `'[$localipv4]($style) '` | 组件格式化模板。                                               |
| `style`    | `'bold yellow'`           | 此组件的样式。                                                |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### 变量

| 字段        | 示例           | 描述                                |
| --------- | ------------ | --------------------------------- |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address |
| style\* |              | `style`对应值                        |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### 配置项

| 选项                  | 默认值                                  | 描述                                                                         |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                                                   |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`                              |
| `symbol`            | `'🌙 '`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `['lua']`                            | 触发此组件的扩展名                                                                  |
| `detect_files`      | `['.lua-version']`                   | 触发此组件的文件名                                                                  |
| `detect_folders`    | `['lua']`                            | 触发此组件的文件夹                                                                  |
| `style`             | `'bold blue'`                        | 此组件的样式。                                                                    |
| `lua_binary`        | `'lua'`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### 变量

| 字段        | 示例       | 描述                   |
| --------- | -------- | -------------------- |
| version   | `v5.4.0` | The version of `lua` |
| symbol    |          | `symbol`对应值          |
| style\* |          | `style`对应值           |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## Maven

The `maven` module indicates the presence of a Maven project in the current directory. If the [Maven Wrapper](https://maven.apache.org/wrapper/) is enabled, the Maven version will be parsed from `.mvn/wrapper/maven-wrapper.properties` and shown.

默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含 `pom.xml` 文件.
- The current directory contains a `.mvn/wrapper/maven-wrapper.properties` file.

If you use an alternate POM syntax (for example `pom.hocon`), add its filename to `detect_files`.

### 配置项

| 选项                  | 默认值                                  | 描述                                                  |
| ------------------- | ------------------------------------ | --------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                            |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`       |
| `symbol`            | `'🅼 '`                               | A format string representing the symbol of Maven.   |
| `detect_extensions` | `[]`                                 | 触发此组件的扩展名                                           |
| `detect_files`      | `['pom.xml']`                        | 触发此组件的文件名                                           |
| `detect_folders`    | `['.mvn']`                           | 触发此组件的文件夹                                           |
| `style`             | `'bold bright-cyan'`                 | 此组件的样式。                                             |
| `disabled`          | `false`                              | Disables the `maven` module.                        |
| `recursive`         | `false`                              | Enables recursive finding for the `.mvn` directory. |

### 变量

| 字段      | 示例       | 描述                     |
| ------- | -------- | ---------------------- |
| version | `v3.2.0` | The version of `maven` |
| symbol  |          | `symbol`对应值            |
| style*  |          | `style`对应值             |

*: 此变量只能作为样式字符串的一部分使用

## Memory Usage

`memory_usage` 组件显示当前系统内存和交换区使用情况。

默认情况下，如果系统交换区使用不为 0，则会显示交换区使用情况。

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项          | 默认值                                             | 描述                     |
| ----------- | ----------------------------------------------- | ---------------------- |
| `threshold` | `75`                                            | 隐藏内存使用情况，除非它超过这个百分比。   |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | 组件格式化模板。               |
| `symbol`    | `'🐏'`                                           | 这个字段的内容会显示在当前内存使用情况之前。 |
| `style`     | `'bold dimmed white'`                           | 此组件的样式。                |
| `disabled`  | `true`                                          | 禁用 `memory_usage` 组件。  |

### 变量

| 字段               | 示例            | 描述                                                                 |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | `symbol`对应值                                                        |
| style\*        |               | `style`对应值                                                         |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

### 示例

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

### 配置项

| 选项                  | 默认值                                | 描述                                                                                        |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | 组件格式化模板。                                                                                  |
| `symbol`            | `'⬢ '`                             | The symbol used before displaying the project name.                                       |
| `style`             | `'blue bold'`                      | 此组件的样式。                                                                                   |
| `disabled`          | `false`                            | Disables the `meson` module.                                                              |

### 变量

| 字段        | 示例         | 描述                             |
| --------- | ---------- | ------------------------------ |
| project   | `starship` | The current Meson project name |
| symbol    | `🐏`        | `symbol`对应值                    |
| style\* |            | `style`对应值                     |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                  | 默认值                                       | 描述                                                    |
| ------------------- | ----------------------------------------- | ----------------------------------------------------- |
| `symbol`            | `' '`                                    | 该字段的内容显示于当前仓库的 hg 书签或活动分支名之前。                         |
| `style`             | `'bold purple'`                           | 此组件的样式。                                               |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | 组件格式化模板。                                              |
| `truncation_length` | `2^63 - 1`                                | Truncates the hg branch / topic name to `N` graphemes |
| `truncation_symbol` | `'…'`                                     | 此字段的内容用来表示分支名称被截断。                                    |
| `disabled`          | `true`                                    | 禁用 `hg_branch` 组件。                                    |

### 变量

| 字段        | 示例        | 描述                          |
| --------- | --------- | --------------------------- |
| branch    | `master`  | The active mercurial branch |
| topic     | `feature` | The active mercurial topic  |
| symbol    |           | `symbol`对应值                 |
| style\* |           | `style`对应值                  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Mercurial State

The `hg_state` module will show in directories which are part of a mercurial repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项           | 默认值                         | 描述                                                            |
| ------------ | --------------------------- | ------------------------------------------------------------- |
| `merge`      | `'MERGING'`                 | A format string displayed when a `merge` is in progress.      |
| `rebase`     | `'REBASING'`                | A format string displayed when a `rebase` is in progress.     |
| `update`     | `'UPDATING'`                | A format string displayed when a `update` is in progress.     |
| `bisect`     | `'BISECTING'`               | A format string displayed when a `bisect` is in progress.     |
| `shelve`     | `'SHELVING'`                | A format string displayed when a `shelve` is in progress.     |
| `graft`      | `'GRAFTING'`                | A format string displayed when a `graft` is in progress.      |
| `transplant` | `'TRANSPLANTING'`           | A format string displayed when a `transplant` is in progress. |
| `histedit`   | `'HISTEDITING'`             | A format string displayed when a `histedit` is in progress.   |
| `style`      | `'bold yellow'`             | 此组件的样式。                                                       |
| `format`     | `'\([$state]($style)\) '` | 组件格式化模板。                                                      |
| `disabled`   | `true`                      | Disables the `hg_state` module.                               |

### 变量

| 字段               | 示例         | 描述                             |
| ---------------- | ---------- | ------------------------------ |
| state            | `REBASING` | The current state of the repo  |
| progress_current | `1`        | The current operation progress |
| progress_total   | `2`        | The total operation progress   |
| style\*        |            | `style`对应值                     |

*: 此变量只能作为样式字符串的一部分使用

## Mise

The `mise` module shows the current mise health as reported by running `mise doctor`.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                  | 默认值                                                                  | 描述                                               |
| ------------------- | -------------------------------------------------------------------- | ------------------------------------------------ |
| `symbol`            | `'mise '`                                                            | The symbol used before displaying _mise_ health. |
| `style`             | `'bold purple'`                                                      | 此组件的样式。                                          |
| `format`            | `'on [$symbol$health]($style) '`                                     | 组件格式化模板。                                         |
| `detect_extensions` | `[]`                                                                 | 触发此组件的扩展名                                        |
| `detect_files`      | `['mise.toml', 'mise.local.toml', '.mise.toml', '.mise.local.toml']` | 触发此组件的文件名                                        |
| `detect_folders`    | `['.mise']`                                                          | 触发此组件的文件夹                                        |
| `healthy_symbol`    | `healthy`                                                            | The message displayed when _mise_ is healthy.    |
| `unhealthy_symbol`  | `unhealthy`                                                          | The message displayed when _mise_ is unhealthy.  |
| `disabled`          | `true`                                                               | Disables the `mise` module.                      |

### 变量

| 字段        | 示例        | 描述                   |
| --------- | --------- | -------------------- |
| health    | `healthy` | The health of _mise_ |
| symbol    |           | `symbol`对应值          |
| style\* |           | `style`对应值           |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[mise]
health = 'ready'
```

## Mojo

The `mojo` module shows the current version of [Mojo programming language](https://www.modular.com/mojo) installed

### 配置项

| 选项                  | 默认值                                   | 描述                                                     |
| ------------------- | ------------------------------------- | ------------------------------------------------------ |
| `format`            | `'with [$symbol($version )]($style)'` | 组件格式化模板。                                               |
| `symbol`            | `'🔥 '`                                | The symbol used before displaying the version of Mojo. |
| `style`             | `'bold 208'`                          | 此组件的样式。                                                |
| `disabled`          | `false`                               | Disables the `mojo` module.                            |
| `detect_extensions` | `['mojo', '🔥']`                       | 触发此组件的扩展名                                              |
| `detect_files`      | `[]`                                  | 触发此组件的文件名                                              |
| `detect_folders`    | `[]`                                  | 触发此组件的文件夹                                              |

### 变量

| 字段        | 示例       | 描述                    |
| --------- | -------- | --------------------- |
| version   | `24.4.0` | The version of `mojo` |
| symbol    |          | `symbol`对应值           |
| style\* |          | `style`对应值            |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[mojo]
format = 'via [mojo ($version )($hash )]($style)'
```

## NATS

The `nats` module shows the name of the current [NATS](https://nats.io) context.

### 配置项

| 选项         | 默认值                        | 描述                                                           |
| ---------- | -------------------------- | ------------------------------------------------------------ |
| `symbol`   | `'✉️ '`                    | The symbol used before the NATS context (defaults to empty). |
| `style`    | `'bold purple'`            | 此组件的样式。                                                      |
| `format`   | `'[$symbol$name]($style)'` | 组件格式化模板。                                                     |
| `disabled` | `false`                    | Disables the `nats` module.                                  |

### 变量

| 字段        | 示例          | 描述                           |
| --------- | ----------- | ---------------------------- |
| name      | `localhost` | The name of the NATS context |
| symbol    |             | `symbol`对应值                  |
| style\* |             | `style`对应值                   |

### 示例

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```

## Network Namespace

The `netns` module shows the current network namespace. This uses `ip netns identify` to get the network namespace, so only network namespaces mounted at `/var/run/netns` will be detected.

### 配置项

| 选项         | 默认值                               | 描述                                                                |
| ---------- | --------------------------------- | ----------------------------------------------------------------- |
| `format`   | `'[$symbol \[$name\]]($style)'` | 组件格式化模板。                                                          |
| `symbol`   | `'🛜 '`                            | The symbol used before the network namespace (defaults to empty). |
| `style`    | `'blue bold dimmed'`              | 此组件的样式。                                                           |
| `disabled` | `false`                           | Disables the `netns` module.                                      |

### 变量

| 字段        | 示例         | 描述                                        |
| --------- | ---------- | ----------------------------------------- |
| name      | `my-netns` | The name of the current network namespace |
| symbol    |            | `symbol`对应值                               |
| style\* |            | `style`对应值                                |

### 示例

```toml
# ~/.config/starship.toml

[netns]
style = 'bold yellow'
symbol = '🌐 '
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含一个 `nim.cfg` 文件
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### 配置项

| 选项                  | 默认值                                  | 描述                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module                             |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`         |
| `symbol`            | `'👑 '`                               | The symbol used before displaying the version of Nim. |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | 触发此组件的扩展名                                             |
| `detect_files`      | `['nim.cfg']`                        | 触发此组件的文件名                                             |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                             |
| `style`             | `'bold yellow'`                      | 此组件的样式。                                               |
| `disabled`          | `false`                              | Disables the `nim` module.                            |

### 变量

| 字段        | 示例       | 描述                    |
| --------- | -------- | --------------------- |
| version   | `v1.2.0` | The version of `nimc` |
| symbol    |          | `symbol`对应值           |
| style\* |          | `style`对应值            |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. 当处于一个 nix-shell 环境中时，此组件会被显示。

### 配置项

| 选项            | 默认值                                            | 描述                                                                    |
| ------------- | ---------------------------------------------- | --------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | 组件格式化模板。                                                              |
| `symbol`      | `'❄️ '`                                        | A format string representing the symbol of nix-shell.                 |
| `style`       | `'bold blue'`                                  | 此组件的样式。                                                               |
| `impure_msg`  | `'impure'`                                     | A format string shown when the shell is impure.                       |
| `pure_msg`    | `'纯色'`                                         | A format string shown when the shell is pure.                         |
| `unknown_msg` | `''`                                           | A format string shown when it is unknown if the shell is pure/impure. |
| `disabled`    | `false`                                        | 禁用 `nix_shell` 组件。                                                    |
| `heuristic`   | `false`                                        | Attempts to detect new `nix shell`-style shells with a heuristic.     |

### 变量

| 字段        | 示例      | 描述                         |
| --------- | ------- | -------------------------- |
| state     | `纯色`    | The state of the nix-shell |
| name      | `lorri` | The name of the nix-shell  |
| symbol    |         | `symbol`对应值                |
| style\* |         | `style`对应值                 |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

`nodejs` 组件显示当前安装的 [Node.js](https://nodejs.org/) 版本。 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含 `package.json` 文件
- 当前目录包含 `.node-version` 文件
- 当前目录包含 `.nvmrc` 文件
- 当前目录包含 `node_modules` 目录
- 当前目录包含扩展名为 `.js`、`.mjs` 或 `.cjs` 的文件
- 当前目录包含扩展名为 `.ts`、`.mts` 或 `.cts` 的文件

此外，在默认情况下，如果目录中包含了 `bunfig.toml`、`bun.lock` 或 `bun.lockb` 文件，此组件将被隐藏，覆盖以上条件。

### 配置项

| 选项                  | 默认值                                           | 描述                                                                                                    |
| ------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | 组件格式化模板。                                                                                              |
| `version_format`    | `'v${raw}'`                                   | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`                                                         |
| `symbol`            | `' '`                                        | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | 触发此组件的扩展名                                                                                             |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | 触发此组件的文件名                                                                                             |
| `detect_folders`    | `['node_modules']`                            | 触发此组件的文件夹                                                                                             |
| `style`             | `'bold green'`                                | 此组件的样式。                                                                                               |
| `disabled`          | `false`                                       | 禁用 `nodejs` 组件。                                                                                       |
| `not_capable_style` | `'bold red'`                                  | The style for the module when an engines property in package.json does not match the Node.js version. |

### 变量

| 字段              | 示例            | 描述                                                                                                                                                        |
| --------------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version         | `v13.12.0`    | The version of `node`                                                                                                                                     |
| engines_version | `>=12.0.0` | `node` version requirement as set in the engines property of `package.json`. Will only show if the version requirement does not match the `node` version. |
| symbol          |               | `symbol`对应值                                                                                                                                               |
| style\*       |               | `style`对应值                                                                                                                                                |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### 配置项

| 选项                        | 默认值                                                                        | 描述                                                      |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------- |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | The format string for the module.                       |
| `version_format`          | `'v${raw}'`                                                                | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`           |
| `symbol`                  | `'🐫 '`                                                                     | The symbol used before displaying the version of OCaml. |
| `global_switch_indicator` | `''`                                                                       | The format string used to represent global OPAM switch. |
| `local_switch_indicator`  | `'*'`                                                                      | The format string used to represent local OPAM switch.  |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                       | 触发此组件的扩展名                                               |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | 触发此组件的文件名                                               |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                    | 触发此组件的文件夹                                               |
| `style`                   | `'bold yellow'`                                                            | 此组件的样式。                                                 |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                            |

### 变量

| 字段               | 示例           | 描述                                                                |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | `symbol`对应值                                                       |
| style\*        |              | `style`对应值                                                        |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Odin

The `odin` module shows the currently installed version of [Odin](https://odin-lang.org/). By default the module will be shown if the current directory contains a `.odin` file.

### 配置项

| 选项                  | 默认值                                  | 描述                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                               |
| `show_commit`       | `false`                              | Shows the commit as part of the version.               |
| `symbol`            | `'Ø '`                               | The symbol used before displaying the version of Odin. |
| `style`             | `'bold bright-blue'`                 | 此组件的样式。                                                |
| `disabled`          | `false`                              | Disables the `odin` module.                            |
| `detect_extensions` | `['odin']`                           | 触发此组件的扩展名                                              |
| `detect_files`      | `[]`                                 | 触发此组件的文件名                                              |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                              |

### 变量

| 字段        | 示例            | 描述                    |
| --------- | ------------- | --------------------- |
| version   | `dev-2024-03` | The version of `odin` |
| symbol    |               | `symbol`对应值           |
| style\* |               | `style`对应值            |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[odin]
format = 'via [󰹩 ($version )]($style)'
show_commit = true
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool. By default the module will be shown if the current directory contains a `.rego` file.

### 配置项

| 选项                  | 默认值                                  | 描述                                              |
| ------------------- | ------------------------------------ | ----------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                        |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`   |
| `symbol`            | `'🪖  '`                              | A format string representing the symbol of OPA. |
| `detect_extensions` | `['rego']`                           | 触发此组件的扩展名                                       |
| `detect_files`      | `[]`                                 | 触发此组件的文件名                                       |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                       |
| `style`             | `'bold blue'`                        | 此组件的样式。                                         |
| `disabled`          | `false`                              | Disables the `opa` module.                      |

### 变量

| 字段        | 示例        | 描述                   |
| --------- | --------- | -------------------- |
| version   | `v0.44.0` | The version of `opa` |
| symbol    |           | `symbol`对应值          |
| style\* |           | `style`对应值           |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### 配置项

| 选项         | 默认值                                             | 描述                                                             |
| ---------- | ----------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | 组件格式化模板。                                                       |
| `symbol`   | `'☁️ '`                                         | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `'bold yellow'`                                 | 此组件的样式。                                                        |
| `disabled` | `false`                                         | Disables the `openstack` module.                               |

### 变量

| 字段        | 示例     | 描述                            |
| --------- | ------ | ----------------------------- |
| cloud     | `corp` | The current OpenStack cloud   |
| project   | `dev`  | The current OpenStack project |
| symbol    |        | `symbol`对应值                   |
| style\* |        | `style`对应值                    |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```

## OS

The `os` module shows the current operating system. OS information is detected via the [os_info](https://lib.rs/crates/os_info) crate.

> [!WARNING] The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项         | 默认值                   | 描述                                                     |
| ---------- | --------------------- | ------------------------------------------------------ |
| `format`   | `'[$symbol]($style)'` | 组件格式化模板。                                               |
| `style`    | `'bold white'`        | 此组件的样式。                                                |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type. Operating system types not defined by your configuration use the default symbols table below. All operating systems currently supported by the module are listed below. If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

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

### 变量

| 字段        | 示例           | 描述                                                                 |
| --------- | ------------ | ------------------------------------------------------------------ |
| symbol    | `🎗️`         | The current operating system symbol from advanced option `symbols` |
| name      | `Arch Linux` | The current operating system name                                  |
| type      | `Arch`       | The current operating system type                                  |
| codename  |              | The current operating system codename, if applicable               |
| edition   |              | The current operating system edition, if applicable                |
| version   |              | The current operating system version, if applicable                |
| style\* |              | `style`对应值                                                         |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

## Package Version 包版本

当前目录是软件包的代码仓库时，将显示 `package` 组件，并显示软件包当前版本。 The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `galaxy`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – `npm` 包版本提取自当前目录中的 `package.json` 文件
- [**JSR**](https://jsr.io/) – The `jsr` package version is extracted from the `jsr.json`/`jsr.jsonc` or `deno.json`/`deno.jsonc` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from a [PEP 621](https://peps.python.org/pep-0621/) compliant `pyproject.toml` or a `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
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

> ⚠️ 此组件显示的是源代码在当前目录中的软件包的版本，而不是包管理器的版本。

### 配置项

| 选项                | 默认值                               | 描述                                                        |
| ----------------- | --------------------------------- | --------------------------------------------------------- |
| `format`          | `'is [$symbol$version]($style) '` | 组件格式化模板。                                                  |
| `symbol`          | `'📦 '`                            | 这个字段的内容会显示在当前软件包版本之前。                                     |
| `version_format`  | `'v${raw}'`                       | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`             |
| `style`           | `'bold 208'`                      | 此组件的样式。                                                   |
| `display_private` | `false`                           | Enable displaying version for packages marked as private. |
| `disabled`        | `false`                           | 禁用 `package` 组件。                                          |

### 变量

| 字段        | 示例       | 描述                          |
| --------- | -------- | --------------------------- |
| version   | `v1.0.0` | The version of your package |
| symbol    |          | `symbol`对应值                 |
| style\* |          | `style`对应值                  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[package]
format = 'via [🎁 $version](208 bold) '
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### 配置项

| 选项                  | 默认值                                                                                                      | 描述                                                    |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | The format string for the module.                     |
| `version_format`    | `'v${raw}'`                                                                                              | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`         |
| `symbol`            | `'🐪 '`                                                                                                   | The symbol used before displaying the version of Perl |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | 触发此组件的扩展名                                             |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | 触发此组件的文件名                                             |
| `detect_folders`    | `[]`                                                                                                     | 触发此组件的文件夹                                             |
| `style`             | `'bold 149'`                                                                                             | 此组件的样式。                                               |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                           |

### 变量

| 字段        | 示例        | 描述                    |
| --------- | --------- | --------------------- |
| version   | `v5.26.1` | The version of `perl` |
| symbol    |           | `symbol`对应值           |
| style\* |           | `style`对应值            |

### 示例

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含一个 `composer.json` 文件
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### 配置项

| 选项                  | 默认值                                  | 描述                                            |
| ------------------- | ------------------------------------ | --------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                      |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch` |
| `symbol`            | `'🐘 '`                               | 这个字段的内容会显示在当前 PHP 版本之前。                       |
| `detect_extensions` | `['php']`                            | 触发此组件的扩展名                                     |
| `detect_files`      | `['composer.json', '.php-version']`  | 触发此组件的文件名                                     |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                     |
| `style`             | `'147 bold'`                         | 此组件的样式。                                       |
| `disabled`          | `false`                              | 禁用 `php` 组件。                                  |

### 变量

| 字段        | 示例       | 描述                   |
| --------- | -------- | -------------------- |
| version   | `v7.3.8` | The version of `php` |
| symbol    |          | `symbol`对应值          |
| style\* |          | `style`对应值           |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Pijul Channel

The `pijul_channel` module shows the active channel of the repo in your current directory.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                  | 默认值                               | 描述                                                                                   |
| ------------------- | --------------------------------- | ------------------------------------------------------------------------------------ |
| `symbol`            | `' '`                            | The symbol used before the pijul channel name of the repo in your current directory. |
| `style`             | `'bold purple'`                   | 此组件的样式。                                                                              |
| `format`            | `'on [$symbol$channel]($style) '` | 组件格式化模板。                                                                             |
| `truncation_length` | `2^63 - 1`                        | Truncates the pijul channel name to `N` graphemes                                    |
| `truncation_symbol` | `'…'`                             | 此字段的内容用来表示分支名称被截断。                                                                   |
| `disabled`          | `true`                            | Disables the `pijul` module.                                                         |

## Pixi

The `pixi` module shows the installed [pixi](https://pixi.sh) version as well as the activated environment, if `$PIXI_ENVIRONMENT_NAME` is set.

> [!TIP] This does not suppress pixi's own prompt modifier, you may want to run `pixi config set shell.change-ps1 false`.

### 配置项

| 选项                         | 默认值                                                       | 描述                                                                                |
| -------------------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`                   | `'via [$symbol($version )(\($environment\) )]($style)'` | 组件格式化模板。                                                                          |
| `version_format`           | `'v${raw}'`                                               | 版本格式 Available vars are `raw`, `major`, `minor`, & `patch`.                       |
| `symbol`                   | `'🧚 '`                                                    | 在环境名之前显示的符号。                                                                      |
| `style`                    | `'yellow bold'`                                           | 此组件的样式。                                                                           |
| `show_default_environment` | `true`                                                    | Whether to indicate that the `default` environment of your project is activated.  |
| `pixi_binary`              | `['pixi']`                                                | Configures the pixi binary that Starship should execute when getting the version. |
| `detect_extensions`        | `[]`                                                      | 触发此组件的扩展名                                                                         |
| `detect_files`             | `['pixi.toml']`                                           | 触发此组件的文件名                                                                         |
| `detect_folders`           | `[]`                                                      | 触发此组件的文件夹                                                                         |
| `disabled`                 | `false`                                                   | Disables the `pixi` module.                                                       |

### 变量

| 字段          | 示例        | 描述                           |
| ----------- | --------- | ---------------------------- |
| version     | `v0.33.0` | The version of `pixi`        |
| environment | `py311`   | The current pixi environment |
| symbol      |           | `symbol`对应值                  |
| style       |           | `style`对应值                   |

### 示例

```toml
# ~/.config/starship.toml

[pixi]
format = '[$symbol$environment](yellow) '
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

> [!TIP] By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms). If you still want to enable it, [follow the example shown below](#with-pulumi-version).

默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### 配置项

| 选项               | 默认值                                          | 描述                                                             |
| ---------------- | -------------------------------------------- | -------------------------------------------------------------- |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | The format string for the module.                              |
| `version_format` | `'v${raw}'`                                  | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`                  |
| `symbol`         | `' '`                                       | A format string shown before the Pulumi stack.                 |
| `style`          | `'bold 5'`                                   | 此组件的样式。                                                        |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories. |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                  |

### 变量

| 字段        | 示例         | 描述                          |
| --------- | ---------- | --------------------------- |
| version   | `v0.12.24` | The version of `pulumi`     |
| stack     | `dev`      | The current Pulumi stack    |
| 用户名       | `alice`    | The current Pulumi username |
| symbol    |            | `symbol`对应值                 |
| style\* |            | `style`对应值                  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. 默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含一个 `spago.dhall` 文件
- 当前目录包含一个 `spago.yaml` 文件
- 当前目录包含一个 `spago.lock` 文件
- The current directory contains a file with the `.purs` extension

### 配置项

| 选项                  | 默认值                                           | 描述                                                           |
| ------------------- | --------------------------------------------- | ------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`          | 组件格式化模板。                                                     |
| `version_format`    | `'v${raw}'`                                   | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`                |
| `symbol`            | `'<=> '`                                | The symbol used before displaying the version of PureScript. |
| `detect_extensions` | `['purs']`                                    | 触发此组件的扩展名                                                    |
| `detect_files`      | `['spago.dhall', 'spago.yaml', 'spago.lock']` | 触发此组件的文件名                                                    |
| `detect_folders`    | `[]`                                          | 触发此组件的文件夹                                                    |
| `style`             | `'bold white'`                                | 此组件的样式。                                                      |
| `disabled`          | `false`                                       | Disables the `purescript` module.                            |

### 变量

| 字段        | 示例       | 描述                          |
| --------- | -------- | --------------------------- |
| version   | `0.13.5` | The version of `purescript` |
| symbol    |          | `symbol`对应值                 |
| style\* |          | `style`对应值                  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

如果`pyenv_version_name`被设置为`true`, 本组件将会展示pyenv版本名。 否则则显示通过`python --version`获得的版本号

By default, the module will be shown if any of the following conditions are met:

- 当前目录包含 `.python-version` 文件
- 当前目录包含 `Pipfile` 文件
- 当前目录包含一个名为`__init__.py`的文件
- 当前目录包含 `pyproject.toml` 文件
- 当前目录包含 `requirements.txt` 文件
- 当前目录包含一个 `setup.py` 文件
- 当前目录包含一个 `tox.ini` 文件
- 当前目录包含一个使用 `.py` 扩展名的文件.
- The current directory contains a file with the `.ipynb` extension.
- 当前处于一个活跃的 python 虚拟环境中

### 配置项

| 选项                   | 默认值                                                                                                          | 描述                                                                                    |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | 组件格式化模板。                                                                              |
| `version_format`     | `'v${raw}'`                                                                                                  | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`                                         |
| `symbol`             | `'🐍 '`                                                                                                       | 用于表示Python的格式化字符串。                                                                    |
| `style`              | `'yellow bold'`                                                                                              | 此组件的样式。                                                                               |
| `pyenv_version_name` | `false`                                                                                                      | 使用 pyenv 获取 Python 版本                                                                 |
| `pyenv_prefix`       | `'pyenv'`                                                                                                    | Prefix before pyenv version display, only used if pyenv is used                       |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configures the python binaries that Starship should execute when getting the version. |
| `detect_extensions`  | `['py', 'ipynb']`                                                                                            | Which extensions should trigger this module                                           |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | Which filenames should trigger this module                                            |
| `detect_folders`     | `[]`                                                                                                         | Which folders should trigger this module                                              |
| `generic_venv_names` | `[]`                                                                                                         | Which venv names should be replaced with the parent directory name.                   |
| `disabled`           | `false`                                                                                                      | 禁用 `python` 组件。                                                                       |

> [!TIP] The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.
> 
> The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `'python3'` will hide any Python version 2, see example below.

### 变量

| 字段           | 示例              | 描述                                                                          |
| ------------ | --------------- | --------------------------------------------------------------------------- |
| version      | `'v3.8.1'`      | `python`版本                                                                  |
| symbol       | `'🐍 '`          | `symbol`对应值                                                                 |
| style        | `'yellow bold'` | `style`对应值                                                                  |
| pyenv_prefix | `'pyenv '`      | Mirrors the value of option `pyenv_prefix`                                  |
| virtualenv   | `'venv'`        | The current `virtualenv` name or the parent if matches `generic_venv_names` |

### 示例

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

### 配置项

| 选项                  | 默认值                                  | 描述                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                          |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`     |
| `symbol`            | `'⨁ '`                               | A format string representing the symbol of Quarto |
| `style`             | `'bold #75AADB'`                     | 此组件的样式。                                           |
| `detect_extensions` | `['.qmd']`                           | 触发此组件的扩展名                                         |
| `detect_files`      | `['_quarto.yml']`                    | 触发此组件的文件名                                         |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                         |
| `disabled`          | `false`                              | Disables the `quarto` module.                     |

### 变量

| 字段        | 示例        | 描述                      |
| --------- | --------- | ----------------------- |
| version   | `1.4.549` | The version of `quarto` |
| symbol    |           | `symbol`对应值             |
| style\* |           | `style`对应值              |

*: 此变量只能作为样式字符串的一部分使用

## R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### 配置项

| 选项                  | 默认值                                  | 描述                                            |
| ------------------- | ------------------------------------ | --------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                      |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch` |
| `symbol`            | `'📐'`                                | A format string representing the symbol of R. |
| `style`             | `'blue bold'`                        | 此组件的样式。                                       |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Which extensions should trigger this module   |
| `detect_files`      | `['.Rprofile']`                      | Which filenames should trigger this module    |
| `detect_folders`    | `['.Rproj.user']`                    | Which folders should trigger this module      |
| `disabled`          | `false`                              | Disables the `r` module.                      |

### 变量

| 字段      | 示例            | 描述                 |
| ------- | ------------- | ------------------ |
| version | `v4.0.5`      | The version of `R` |
| symbol  |               | `symbol`对应值        |
| style   | `'blue bold'` | `style`对应值         |

### 示例

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### 配置项

| 选项                  | 默认值                                              | 描述                                                    |
| ------------------- | ------------------------------------------------ | ----------------------------------------------------- |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | The format string for the module.                     |
| `version_format`    | `'v${raw}'`                                      | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`         |
| `symbol`            | `'🦋 '`                                           | The symbol used before displaying the version of Raku |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | 触发此组件的扩展名                                             |
| `detect_files`      | `['META6.json']`                                 | 触发此组件的文件名                                             |
| `detect_folders`    | `[]`                                             | 触发此组件的文件夹                                             |
| `style`             | `'bold 149'`                                     | 此组件的样式。                                               |
| `disabled`          | `false`                                          | Disables the `raku` module.                           |

### 变量

| 字段         | 示例     | 描述                                   |
| ---------- | ------ | ------------------------------------ |
| version    | `v6.d` | The version of `raku`                |
| vm_version | `moar` | The version of VM `raku` is built on |
| symbol     |        | `symbol`对应值                          |
| style\*  |        | `style`对应值                           |

### 示例

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). 此组件将在符合以下任意条件时显示：

- The current directory contains a file with `.red` or `.reds` extension

### 配置项

| 选项                  | 默认值                                  | 描述                                              |
| ------------------- | ------------------------------------ | ----------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                        |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`   |
| `symbol`            | `'🔺 '`                               | A format string representing the symbol of Red. |
| `detect_extensions` | `['red']`                            | 触发此组件的扩展名                                       |
| `detect_files`      | `[]`                                 | 触发此组件的文件名                                       |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                       |
| `style`             | `'red bold'`                         | 此组件的样式。                                         |
| `disabled`          | `false`                              | Disables the `red` module.                      |

### 变量

| 字段        | 示例       | 描述                   |
| --------- | -------- | -------------------- |
| version   | `v2.5.1` | The version of `red` |
| symbol    |          | `symbol`对应值          |
| style\* |          | `style`对应值           |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). 此组件将在符合以下任意条件时显示：

- 当前目录包含 `Gemfile` 文件
- The current directory contains a `.ruby-version` file
- 当前目录包含 `.rb` 文件
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### 配置项

| 选项                  | 默认值                                  | 描述                                                      |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                                |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`           |
| `symbol`            | `'💎 '`                               | A format string representing the symbol of Ruby.        |
| `detect_extensions` | `['rb']`                             | 触发此组件的扩展名                                               |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | 触发此组件的文件名                                               |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                               |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Which environment variables should trigger this module. |
| `style`             | `'bold red'`                         | 此组件的样式。                                                 |
| `disabled`          | `false`                              | 禁用 `ruby` 组件。                                           |

### 变量

| 字段        | 示例       | 描述                                          |
| --------- | -------- | ------------------------------------------- |
| version   | `v2.5.1` | The version of `ruby`                       |
| symbol    |          | `symbol`对应值                                 |
| style\* |          | `style`对应值                                  |
| gemset    | `test`   | Optional, gets the current RVM gemset name. |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). 此组件将在符合以下任意条件时显示：

- 当前目录包含 `Cargo.toml` 文件
- 当前目录包含一个使用 `.rs` 扩展名的文件

### 配置项

| 选项                  | 默认值                                  | 描述                                              |
| ------------------- | ------------------------------------ | ----------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                        |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`   |
| `symbol`            | `'🦀 '`                               | A format string representing the symbol of Rust |
| `detect_extensions` | `['rs']`                             | 触发此组件的扩展名                                       |
| `detect_files`      | `['Cargo.toml']`                     | 触发此组件的文件名                                       |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                       |
| `style`             | `'bold red'`                         | 此组件的样式。                                         |
| `disabled`          | `false`                              | 禁用 `rust` 组件。                                   |

### 变量

| 字段        | 示例                | 描述                                           |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbol    |                   | `symbol`对应值                                  |
| style\* |                   | `style`对应值                                   |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### 配置项

| 选项                  | 默认值                                      | 描述                                                |
| ------------------- | ---------------------------------------- | ------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'` | 组件格式化模板。                                          |
| `version_format`    | `'v${raw}'`                              | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`     |
| `detect_extensions` | `['sbt', 'scala']`                       | 触发此组件的扩展名                                         |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | 触发此组件的文件名                                         |
| `detect_folders`    | `['.metals']`                            | 触发此组件的文件夹                                         |
| `symbol`            | `'🆂 '`                                   | A format string representing the symbol of Scala. |
| `style`             | `'red dimmed'`                           | 此组件的样式。                                           |
| `disabled`          | `false`                                  | Disables the `scala` module.                      |

### 变量

| 字段        | 示例       | 描述                     |
| --------- | -------- | ---------------------- |
| version   | `2.13.5` | The version of `scala` |
| symbol    |          | `symbol`对应值            |
| style\* |          | `style`对应值             |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```

## Shell

The `shell` module shows an indicator for currently used shell.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                     | 默认值                       | 描述                                                                                                     |
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
| `format`               | `'[$indicator]($style) '` | 组件格式化模板。                                                                                               |
| `style`                | `'white bold'`            | 此组件的样式。                                                                                                |
| `disabled`             | `true`                    | Disables the `shell` module.                                                                           |

### 变量

| 字段        | 默认值 | 描述                                                         |
| --------- | --- | ---------------------------------------------------------- |
| indicator |     | Mirrors the value of `indicator` for currently used shell. |
| style\* |     | `style`对应值.                                                |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is set to a number and meets or exceeds the specified threshold.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项              | 默认值                          | 描述                                                                  |
| --------------- | ---------------------------- | ------------------------------------------------------------------- |
| `threshold`     | `2`                          | Display threshold.                                                  |
| `format`        | `'[$symbol$shlvl]($style) '` | 组件格式化模板。                                                            |
| `symbol`        | `'↕️  '`                     | The symbol used to represent the `SHLVL`.                           |
| `repeat`        | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount.       |
| `repeat_offset` | `0`                          | Decrements number of times `symbol` is repeated by the offset value |
| `style`         | `'bold yellow'`              | 此组件的样式。                                                             |
| `disabled`      | `true`                       | Disables the `shlvl` module.                                        |

### 变量

| 字段        | 示例  | 描述                           |
| --------- | --- | ---------------------------- |
| shlvl     | `3` | The current value of `SHLVL` |
| symbol    |     | `symbol`对应值                  |
| style\* |     | `style`对应值                   |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

Using `repeat` and `repeat_offset` along with `character` module, one can get prompt like `❯❯❯` where last character is colored appropriately for return status code and preceding characters are provided by `shlvl`.

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

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### 配置项

| 选项         | 默认值                              | 描述                                               |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | 组件格式化模板。                                         |
| `symbol`   | `''`                             | A format string displayed before the image name. |
| `style`    | `'bold dimmed blue'`             | 此组件的样式。                                          |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### 变量

| 字段        | 示例           | 描述                            |
| --------- | ------------ | ----------------------------- |
| env       | `centos.img` | The current Singularity image |
| symbol    |              | `symbol`对应值                   |
| style\* |              | `style`对应值                    |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Solidity

The `solidity` module shows the currently installed version of [Solidity](https://soliditylang.org/) The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.sol` extension

### 配置项

| 选项                  | 默认值                                  | 描述                                                  |
| ------------------- | ------------------------------------ | --------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                            |
| `version_format`    | `'v${major}.${minor}.${patch}'`      | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`       |
| `symbol`            | `'S '`                               | A format string representing the symbol of Solidity |
| `compiler          | ['solc']                             | The default compiler for Solidity.                  |
| `detect_extensions` | `['sol']`                            | 触发此组件的扩展名                                           |
| `detect_files`      | `[]`                                 | 触发此组件的文件名                                           |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                           |
| `style`             | `'bold blue'`                        | 此组件的样式。                                             |
| `disabled`          | `false`                              | Disables this module.                               |

### 变量

| 字段        | 示例       | 描述                        |
| --------- | -------- | ------------------------- |
| version   | `v0.8.1` | The version of `solidity` |
| symbol    |          | `symbol`对应值               |
| style\* |          | `style`对应值                |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml
[solidity]
format = "via [S $version](blue bold)"
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### 配置项

| 选项                  | 默认值                                    | 描述                                                                                                                    |
| ------------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to. `0` 表示不用截断。 另请参阅 [`directory`](#directory) 组件。 |
| `symbol`            | `'🅢  '`                                | 在环境名之前显示的符号。                                                                                                          |
| `style`             | `'bold blue'`                          | 此组件的样式。                                                                                                               |
| `format`            | `'via [$symbol$environment]($style) '` | 组件格式化模板。                                                                                                              |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                          |

### 变量

| 字段          | 示例           | 描述                            |
| ----------- | ------------ | ----------------------------- |
| environment | `astronauts` | The current spack environment |
| symbol      |              | `symbol`对应值                   |
| style\*   |              | `style`对应值                    |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Status

The `status` module displays the exit code of the previous command. If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`. The status code will cast to a signed 32-bit integer.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                          | 默认值                                                                                 | 描述                                                                    |
| --------------------------- | ----------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                       | The format of the module                                              |
| `symbol`                    | `'❌'`                                                                               | The symbol displayed on program error                                 |
| `success_symbol`            | `''`                                                                                | The symbol displayed on program success                               |
| `not_executable_symbol`     | `'🚫'`                                                                               | The symbol displayed when file isn't executable                       |
| `not_found_symbol`          | `'🔍'`                                                                               | The symbol displayed when the command can't be found                  |
| `sigint_symbol`             | `'🧱'`                                                                               | The symbol displayed on SIGINT (Ctrl + c)                             |
| `signal_symbol`             | `'⚡'`                                                                               | The symbol displayed on any signal                                    |
| `style`                     | `'bold red'`                                                                        | 此组件的样式。                                                               |
| `success_style`             |                                                                                     | The style used on program success (defaults to `style` if unset).     |
| `failure_style`             |                                                                                     | The style used on program failure (defaults to `style` if unset).     |
| `recognize_signal_code`     | `true`                                                                              | Enable signal mapping from exit code                                  |
| `map_symbol`                | `false`                                                                             | Enable symbols mapping from exit code                                 |
| `pipestatus`                | `false`                                                                             | Enable pipestatus reporting                                           |
| `pipestatus_separator`      | <code>&vert;</code>                                                           | The symbol used to separate pipestatus segments (supports formatting) |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style) '` | The format of the module when the command is a pipeline               |
| `pipestatus_segment_format` |                                                                                     | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                              | Disables the `status` module.                                         |

### 变量

| 字段             | 示例      | 描述                                                                                           |
| -------------- | ------- | -------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                            |
| hex_status     | `0x7F`  | The exit code of the last command in hex                                                     |
| int            | `127`   | The exit code of the last command                                                            |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                          |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                              |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                         |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                 |
| pipestatus     |         | Rendering of in pipeline programs' exit codes, this is only available in pipestatus_format   |
| symbol         |         | `symbol`对应值                                                                                  |
| style\*      |         | Mirrors the value of option `success_style` on program success and `failure_style` otherwise |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

The `sudo` module displays if sudo credentials are currently cached. The module will only be shown if credentials are cached.

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项              | 默认值                      | 描述                                                      |
| --------------- | ------------------------ | ------------------------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | The format of the module                                |
| `symbol`        | `'🧙 '`                   | The symbol displayed when credentials are cached        |
| `style`         | `'bold blue'`            | 此组件的样式。                                                 |
| `allow_windows` | `false`                  | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                   | Disables the `sudo` module.                             |

### 变量

| 字段        | 示例 | 描述          |
| --------- | -- | ----------- |
| symbol    |    | `symbol`对应值 |
| style\* |    | `style`对应值  |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[sudo]
style = 'bold green'
symbol = '👩‍💻 '
disabled = false
```

```toml
# 在 Windows 上
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). 此组件将在符合以下任意条件时显示：

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### 配置项

| 选项                  | 默认值                                  | 描述                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                         |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`    |
| `symbol`            | `'🐦 '`                               | A format string representing the symbol of Swift |
| `detect_extensions` | `['swift']`                          | 触发此组件的扩展名                                        |
| `detect_files`      | `['Package.swift']`                  | 触发此组件的文件名                                        |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                        |
| `style`             | `'bold 202'`                         | 此组件的样式。                                          |
| `disabled`          | `false`                              | Disables the `swift` module.                     |

### 变量

| 字段        | 示例       | 描述                     |
| --------- | -------- | ---------------------- |
| version   | `v5.2.4` | The version of `swift` |
| symbol    |          | `symbol`对应值            |
| style\* |          | `style`对应值             |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[swift]
format = 'via [🏎  $version](red bold)'
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version. It supports both Hashicorp Terraform and OpenTofu for version detection.

> [!TIP] By default the Terraform/OpenTofu version is not shown, since this is slow for current versions when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-terraform-version).

默认情况下，此组件将在满足以下任意条件时显示：

- 当前目录包含 `.terraform` 目录
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### 配置项

| 选项                  | 默认值                                                     | 描述                                                    |
| ------------------- | ------------------------------------------------------- | ----------------------------------------------------- |
| `format`            | `'via [$symbol$workspace]($style) '`                    | The format string for the module.                     |
| `version_format`    | `'v${raw}'`                                             | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`         |
| `symbol`            | `'💠'`                                                   | A format string shown before the terraform workspace. |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`                           | 触发此组件的扩展名                                             |
| `detect_files`      | `[]`                                                    | 触发此组件的文件名                                             |
| `detect_folders`    | `['.terraform']`                                        | 触发此组件的文件夹                                             |
| `style`             | `'bold 105'`                                            | 此组件的样式。                                               |
| `disabled`          | `false`                                                 | 禁用 `terraform` 组件。                                    |
| `commands`          | `[ [ 'terraform', 'version' ], [ 'tofu', 'version' ] ]` | How to detect what the Terraform version is.          |

### 变量

| 字段        | 示例         | 描述                              |
| --------- | ---------- | ------------------------------- |
| version   | `v0.12.24` | The version of `terraform`      |
| workspace | `default`  | The current Terraform workspace |
| symbol    |            | `symbol`对应值                     |
| style\* |            | `style`对应值                      |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

## 时间

`time` 组件显示当前的 **本地** 时间。 `format` 字段值会提供给 [`chrono`](https://crates.io/crates/chrono) crate 用来控制时间显示方式。 请参阅 [chrono strftime 文档](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) 以了解可用格式选项。

> [!TIP] This module is disabled by default. 若要启用此组件，请在配置文件中设置 `disable` 字段为 `false`。

### 配置项

| 选项                | 默认值                     | 描述                                                                                                    |
| ----------------- | ----------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | The format string for the module.                                                                     |
| `use_12hr`        | `false`                 | 启用 12 小时格式                                                                                            |
| `time_format`     | 见下文解释                   | 用来格式化时间显示的 [chrono 格式字符串](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)             |
| `style`           | `'bold yellow'`         | 显示时间的样式。                                                                                              |
| `utc_time_offset` | `'local'`               | 设置所用 UTC 偏移量。 Range from -24 &lt; x &lt; 24. 允许使用浮点数来得到 30/45 分钟的时区偏移。                    |
| `disabled`        | `true`                  | 禁用 `time` 组件。                                                                                         |
| `time_range`      | `'-'`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`. Manually setting `time_format` will override the `use_12hr` setting.

### 变量

| 字段        | 示例         | 描述         |
| --------- | ---------- | ---------- |
| time      | `13:08:10` | 当前时间       |
| style\* |            | `style`对应值 |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

- 当前目录包含一个 `template.typ` 文件
- The current directory contains any `*.typ` file

### 配置项

| 选项                  | 默认值                                  | 描述                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                         |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`    |
| `symbol`            | `'t '`                               | A format string representing the symbol of Typst |
| `style`             | `'bold #0093A7'`                     | 此组件的样式。                                          |
| `detect_extensions` | `['.typ']`                           | 触发此组件的扩展名                                        |
| `detect_files`      | `['template.typ']`                   | 触发此组件的文件名                                        |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                        |
| `disabled`          | `false`                              | Disables the `typst` module.                     |

### 变量

| 字段            | 示例        | 描述                                              |
| ------------- | --------- | ----------------------------------------------- |
| version       | `v0.9.0`  | The version of `typst`, alias for typst_version |
| typst_version | `default` | The current Typst version                       |
| symbol        |           | `symbol`对应值                                     |
| style\*     |           | `style`对应值                                      |

*: 此变量只能作为样式字符串的一部分使用

## Username 用户名

`username` 组件显示当前活跃的用户名。 此组件将在符合以下任意条件时显示：

- 当前用户是 root/admin
- 当前用户与登录用户不相同
- 用户正通过 SSH 会话连接访问
- 字段 `show_always` 被设置为 true
- The array `detect_env_vars` contains at least the name of one environment variable, that is set

> [!TIP] SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

### 配置项

| 选项                | 默认值                     | 描述                                                        |
| ----------------- | ----------------------- | --------------------------------------------------------- |
| `style_root`      | `'bold red'`            | The style used when the user is root/admin.               |
| `style_user`      | `'bold yellow'`         | 非 root 用户使用的样式。                                           |
| `detect_env_vars` | `[]`                    | Which environment variable(s) should trigger this module. |
| `format`          | `'[$user]($style) in '` | 组件格式化模板。                                                  |
| `show_always`     | `false`                 | 总是显示 `username` 组件。                                       |
| `disabled`        | `false`                 | 禁用 `username` 组件。                                         |
| `aliases`         | `{}`                    | Translate system usernames to something else.             |

### 变量

| 字段      | 示例           | 描述                                                                                          |
| ------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style` | `'red bold'` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`  | `'matchai'`  | The currently logged-in user ID.                                                            |

### 示例

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

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a `Vagrantfile` file

### 配置项

| 选项                  | 默认值                                  | 描述                                                  |
| ------------------- | ------------------------------------ | --------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                            |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`       |
| `symbol`            | `'⍱ '`                               | A format string representing the symbol of Vagrant. |
| `detect_extensions` | `[]`                                 | 触发此组件的扩展名                                           |
| `detect_files`      | `['Vagrantfile']`                    | 触发此组件的文件名                                           |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                           |
| `style`             | `'cyan bold'`                        | 此组件的样式。                                             |
| `disabled`          | `false`                              | Disables the `vagrant` module.                      |

### 变量

| 字段        | 示例               | 描述                       |
| --------- | ---------------- | ------------------------ |
| version   | `Vagrant 2.2.10` | The version of `Vagrant` |
| symbol    |                  | `symbol`对应值              |
| style\* |                  | `style`对应值               |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/). 默认情况下，此组件将在满足以下任意条件时显示：

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### 配置项

| 选项                  | 默认值                                          | 描述                                            |
| ------------------- | -------------------------------------------- | --------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`         | 组件格式化模板。                                      |
| `version_format`    | `'v${raw}'`                                  | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch` |
| `symbol`            | `'V '`                                       | A format string representing the symbol of V  |
| `detect_extensions` | `['v']`                                      | 触发此组件的扩展名                                     |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | 触发此组件的文件名                                     |
| `detect_folders`    | `[]`                                         | 触发此组件的文件夹                                     |
| `style`             | `'blue bold'`                                | 此组件的样式。                                       |
| `disabled`          | `false`                                      | Disables the `vlang` module.                  |

### 变量

| 字段        | 示例     | 描述                 |
| --------- | ------ | ------------------ |
| version   | `v0.2` | The version of `v` |
| symbol    |        | `symbol`对应值        |
| style\* |        | `style`对应值         |

### 示例

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCS

> Note the module is enabled by default but **not** included in the default list because that would be a breaking change. Additionally, the exact format of the module may change in the future, for example to handle right-aligned prompt.

The `vcs` module displays the current active Version Control System (VCS). The module will be shown only if a configured VCS is currently in use.

### 配置项

| 选项               | 默认值                                                         | 描述                                                    |
| ---------------- | ----------------------------------------------------------- | ----------------------------------------------------- |
| `order`          | `["git", "hg", "pijul", "fossil"]`                          | The order in which to search VCSes.                   |
| `fossil_modules` | `"$fossil_branch$fossil_metrics"`                           | Modules to show when a Fossil repository is found.    |
| `git_modules`    | `"$git_branch$git_commit$git_state$git_metrics$git_status"` | Modules to show when a Git repository is found.       |
| `hg_modules`     | `"$hg_branch$hg_state"`                                     | Modules to show when a Mercurial repository is found. |
| `pijul_modules`  | `"$pijul_channel"`                                          | Modules to show when a Pijul repository is found.     |
| `disabled`       | `false`                                                     | Disables the `vcs` module.                            |

### 示例

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

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### 配置项

| 选项         | 默认值                              | 描述                                                     |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   | `''`                             | The symbol used before displaying the repository name. |
| `style`    | `'bold yellow'`                  | 此组件的样式。                                                |
| `format`   | `'vcsh [$symbol$repo]($style) '` | 组件格式化模板。                                               |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### 变量

| 字段        | 示例                                          | 描述                         |
| --------- | ------------------------------------------- | -------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name |
| symbol    |                                             | `symbol`对应值                |
| style\* | `black bold dimmed`                         | `style`对应值                 |

*: 此变量只能作为样式字符串的一部分使用

### 示例

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## XMake

The `xmake` module shows the currently installed version of [XMake](https://xmake.io/). 默认情况下，此组件将在满足以下任意条件时启用：

- 当前目录包含 `xmake.lua` 文件

### 配置项

| 选项                  | 默认值                                  | 描述                                            |
| ------------------- | ------------------------------------ | --------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                      |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch` |
| `symbol`            | `'△ '`                               | The symbol used before the version of cmake.  |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module   |
| `detect_files`      | `['xmake.lua']`                      | Which filenames should trigger this module    |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module      |
| `style`             | `'bold green'`                       | 此组件的样式。                                       |
| `disabled`          | `false`                              | Disables the `xmake` module.                  |

### 变量

| 字段        | 示例       | 描述                   |
| --------- | -------- | -------------------- |
| version   | `v2.9.5` | The version of xmake |
| symbol    |          | `symbol`对应值          |
| style\* |          | `style`对应值           |

*: 此变量只能作为样式字符串的一部分使用

## Zig

By default the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). 此组件将在符合以下任意条件时显示：

- The current directory contains a `.zig` file

### 配置项

| 选项                  | 默认值                                  | 描述                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 组件格式化模板。                                              |
| `version_format`    | `'v${raw}'`                          | 版本格式 可用的字段有 `raw`, `major`, `minor` 和 `patch`         |
| `symbol`            | `'↯ '`                               | The symbol used before displaying the version of Zig. |
| `style`             | `'bold yellow'`                      | 此组件的样式。                                               |
| `disabled`          | `false`                              | Disables the `zig` module.                            |
| `detect_extensions` | `['zig']`                            | 触发此组件的扩展名                                             |
| `detect_files`      | `[]`                                 | 触发此组件的文件名                                             |
| `detect_folders`    | `[]`                                 | 触发此组件的文件夹                                             |

### 变量

| 字段        | 示例       | 描述                   |
| --------- | -------- | -------------------- |
| version   | `v0.6.0` | The version of `zig` |
| symbol    |          | `symbol`对应值          |
| style\* |          | `style`对应值           |

*: 此变量只能作为样式字符串的一部分使用

### 示例

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

> [!TIP] Multiple custom modules can be defined by using a `.`.

> [!TIP] The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

> [!TIP] [Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

> [!WARNING] If `unsafe_no_escape` is enabled or prior to starship v1.20 command output is printed unescaped to the prompt.
> 
> Whatever output the command generates is printed unmodified in the prompt. This means if the output contains shell-specific interpretable sequences, they could be interpreted on display. Depending on the shell, this can mean that e.g. strings enclosed by backticks are executed by the shell. Such sequences are usually shell specific, e.g. you can write a command module that writes bash sequences, e.g. `\h`, but this module will not work in a fish or zsh shell.
> 
> Format strings can also contain shell specific prompt sequences, e.g. [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

### 配置项

| 选项                  | 默认值                             | 描述                                                                                                                                                                                                                                                                                            |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                                                                                                                                 |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the `shell` returns a `0` status code from executing it.                                                        |
| `require_repo`      | `false`                         | If `true`, the module will only be shown in paths containing a (git) repository. This option alone is not sufficient display condition in absence of other options.                                                                                                                           |
| `shell`             |                                 | [See below](#custom-command-shell)                                                                                                                                                                                                                                                            |
| `description`       | `'<custom module>'`       | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                  |
| `unsafe_no_escape`  | `false`                         | When set, command output is not escaped of characters that could be interpreted by the shell.                                                                                                                                                                                                 |
| `detect_files`      | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                                                                                                                                         |
| `detect_folders`    | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                                                                                                                                   |
| `detect_extensions` | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                                                                                                                                    |
| `symbol`            | `''`                            | The symbol used before displaying the command output.                                                                                                                                                                                                                                         |
| `style`             | `'bold green'`                  | 此组件的样式。                                                                                                                                                                                                                                                                                       |
| `format`            | `'[$symbol($output )]($style)'` | 组件格式化模板。                                                                                                                                                                                                                                                                                      |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                |
| `os`                |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                 |
| `use_stdin`         |                                 | An optional boolean value that overrides whether commands should be forwarded to the shell via the standard input or as an argument. If unset standard input is used by default, unless the shell does not support it (cmd, nushell). Setting this disables shell-specific argument handling. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                     |

### 变量

| 字段        | 描述                                     |
| --------- | -------------------------------------- |
| output    | The output of `command` run in `shell` |
| symbol    | `symbol`对应值                            |
| style\* | `style`对应值                             |

*: 此变量只能作为样式字符串的一部分使用

#### 自定义命令 shell

`shell` 接受一个非空字符串列表：

- 第一个字符串是用于执行命令的 shell 路径。
- 剩下的将作为参数传递给 shell。

如果未设置，它将回退到 STARSHIP_SHELL，然后回退到 Linux 上的 "sh" 命令，在 Windows 上则是 "cmd /C"。

The `command` (and `when`, if applicable) will be passed in on stdin.

如果 `shell` 未设置或仅包含一个元素，并且 Starship 检测到正在使用 PowerShell，则会自动添加 `-NoProfile -Command -` 参数。 如果 `shell` 未设置或仅包含一个元素，并且 Starship 检测到正在使用 Cmd，则会自动添加 `/C` 参数，并且将 `stdin` 设置为 `false`。 如果 `shell` 未设置或仅包含一个元素，并且 Starship 检测到正在使用 Cmd，则会自动添加 `-C` 参数，并且将 `stdin` 设置为 `false`。 可以通过将参数显式传递给 shell 来避免这种行为，例如：

```toml
shell = ['pwsh', '-Command', '-']
```

> [!WARNING] Make sure your custom shell configuration exits gracefully
> 
> If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).
> 
> For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.
> 
> Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.
> 
> Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

### 示例

```toml
# ~/.config/starship.toml

[custom.foo]
command = 'echo foo' # 显示命令输出
detect_files = ['foo'] # 支持过滤器，但不支持通配符
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'

[custom.time]
command = 'time /T'
detect_extensions = ['pst'] # 识别 *.pst 文件
shell = ['pwsh.exe', '-NoProfile', '-Command', '-']

[custom.time-as-arg]
command = 'time /T'
detect_extensions = ['pst'] # 识别 *.pst 文件
shell = ['pwsh.exe', '-NoProfile', '-Command']
use_stdin = false
```
