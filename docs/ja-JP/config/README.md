# 設定

::: tip

🔥Starshipの開発は現在も進んでいます。 多くの新しいオプションが今後のリリースで利用可能になります。

:::

Starshipの設定を開始するには、`~/.config/starship.toml` ファイルを作成します。

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Starshipのすべての設定は、この[TOML](https://github.com/toml-lang/toml)ファイルで行われます。

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

`STARSHIP_CONFIG` 環境変数を使用して、デフォルトの`starship.toml` ファイルの場所を変更できます。

```sh
export STARSHIP_CONFIG=~/.starship
```

PowerShell (Windows) で同様に `$PROFILE`にこの行を追加します。

```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### 用語

**モジュール**: OSのコンテキスト情報に基づいて情報を提供するプロンプト内のコンポーネントです。 たとえば、現在のディレクトリがNodeJSプロジェクトである場合、「nodejs」モジュールは、現在コンピューターにインストールされているNodeJSのバージョンを表示します。

**Variable**: Smaller sub-components that contains information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of NodeJS.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Format Strings

Format strings are the format that a module prints all its variables with. Most modules have an entry called `format` that configures the display format of the module. You can use texts, variables and text groups in a format string.

#### 変数

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

#### スタイルの設定

Starshipのほとんどのモジュールでは、表示スタイルを設定できます。 これは、設定を指定する文字列であるエントリ（`style`）で行われます。 スタイル文字列の例とその機能を次に示します。 完全な構文の詳細については、詳細は [高度な設定](/advanced-config/)を参照してください 。

- `"fg:green bg:blue"` は、青色の背景に緑色のテキストを設定します
- `"bg:blue fg:bright-green"` は、青色の背景に明るい緑色のテキストを設定します
- `"bold fg:27"` は、 [ANSIカラー](https://i.stack.imgur.com/KTSQa.png) 27の太字テキストを設定します
- `"underline bg:#bf5700"` は、焦げたオレンジ色の背景に下線付きのテキストを設定します
- `"bold italic fg:purple"`は、紫色の太字斜体のテキストを設定します
- `""` はすべてのスタイルを明示的に無効にします

スタイリングがどのように見えるかは、端末エミュレータによって制御されることに注意してください。 たとえば、一部の端末エミュレータはテキストを太字にする代わりに色を明るくします。また、一部のカラーテーマは通常の色と明るい色と同じ値を使用します。 また、斜体のテキストを取得するには、端末で斜体をサポートする必要があります。

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

## プロンプト

これは、プロンプト全体のオプションのリストです。

### オプション

| Option         | デフォルト                          | 説明                                       |
| -------------- | ------------------------------ | ---------------------------------------- |
| `format`       | [link](#default-prompt-format) | Configure the format of the prompt.      |
| `scan_timeout` | `30`                           | ファイルをスキャンする際のタイムアウト時間 (milliseconds) です。 |

### 設定例

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

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. デフォルトは次のとおりです。

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

`aws` モジュールは現在のAWSプロファイルが表示されます。 これは `~/.aws/config` に記述されている `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` 環境変数に基づいています。

[aws-vault](https://github.com/99designs/aws-vault)を使用する場合、プロファイル は`AWS_VAULT`env varから読み取られます。

### オプション

| Option           | デフォルト                                                | 説明                            |
| ---------------- | ---------------------------------------------------- | ----------------------------- |
| `format`         | `"on [$symbol$profile(\\($region\\))]($style) "` | The format for the module.    |
| `symbol`         | `"☁️ "`                                              | 現在のAWSプロファイルを表示する前に表示される記号です。 |
| `region_aliases` |                                                      | AWS名に加えて表示するリージョンのエイリアスです。    |
| `style`          | `"bold yellow"`                                      | モジュールのスタイルです。                 |
| `disabled`       | `false`                                              | `aws`モジュールを無効にします。            |

### Variables

| 変数        | 設定例              | 説明                                   |
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

## バッテリー

`battery`モジュールは、デバイスのバッテリー残量と現在の充電状態を示します。 モジュールは、デバイスのバッテリー残量が10％未満の場合にのみ表示されます。

### オプション

| Option               | デフォルト                             | 説明                         |
| -------------------- | --------------------------------- | -------------------------- |
| `full_symbol`        | `"•"`                             | バッテリーが満タンのときに表示される記号です。    |
| `charging_symbol`    | `"⇡"`                             | バッテリーの充電中に表示される記号です。       |
| `discharging_symbol` | `"⇣"`                             | バッテリーが放電しているときに表示される記号です。  |
| `format`             | `"[$symbol$percentage]($style) "` | The format for the module. |
| `display`            | [link](#battery-display)          | モジュールの閾値とスタイルを表示します。       |
| `disabled`           | `false`                           | `battery`モジュールを無効にします。     |

<details>
<summary>いくつかのまれなバッテリー状態のオプションもあります。</summary>

| 変数               | 説明                       |
| ---------------- | ------------------------ |
| `unknown_symbol` | バッテリー状態が不明なときに表示される記号です。 |
| `empty_symbol`   | バッテリーが空のときに表示される記号です。    |

オプションを指定しない限り、バッテリーの状態が`unknown`もしくは`empty`になった場合にインジケーターは非表示になります。

</details>

### 設定例

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### バッテリーの表示

`display` オプションを使用して、バッテリーインジケーターを表示するタイミング（閾値）と外観（スタイル）を定義します。 `display` が提供されない場合、 デフォルトは次のとおりです。

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### オプション

`display`オプションは、次の表の通りです。

| 変数          | 説明                             |
| ----------- | ------------------------------ |
| `threshold` | バッテリーが表示される上限です。               |
| `style`     | displayオプションが使用されている場合のスタイルです。 |

#### 設定例

```toml
[[battery.display]]  # バッテリー残量が0％〜10％の間は「太字の赤色」スタイルを利用する
threshold = 10
style = "bold red"

[[battery.display]]  # バッテリー残量が10％〜30％の間は「太字の黄色」スタイルを利用する
threshold = 30
style = "bold yellow"

# 容量が30％を超えると、バッテリーインジケーターは表示されません

```

## 文字

`character`モジュールは、端末でテキストが入力される場所の横に文字（通常は矢印）を表示します。

文字は、最後のコマンドが成功したかどうかを示します。 It can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change it's shape take a look at [this example](#with-custom-error-shape).

### オプション

| Option           | デフォルト               | 説明                                                                               |
| ---------------- | ------------------- | -------------------------------------------------------------------------------- |
| `format`         | `"$symbol "`        | The format string used before the text input.                                    |
| `success_symbol` | `"[❯](bold green)"` | The format string used before the text input if the previous command succeeded.  |
| `error_symbol`   | `"[❯](bold red)"`   | The format string used before the text input if the previous command failed.     |
| `vicmd_symbol`   | `"[❮](bold green)"` | The format string used before the text input if the shell is in vim normal mode. |
| `disabled`       | `false`             | `character`モジュールを無効にします。                                                         |

### Variables

| 変数     | 設定例 | 説明                                                                    |
| ------ | --- | --------------------------------------------------------------------- |
| symbol |     | A mirror of either `success_symbol`, `error_symbol` or `vicmd_symbol` |

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

### オプション

| Option     | デフォルト                              | 説明                                           |
| ---------- | ---------------------------------- | -------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                   |
| `symbol`   | `"🛆 "`                             | The symbol used before the version of cmake. |
| `style`    | `"bold blue"`                      | モジュールのスタイルです。                                |
| `disabled` | `false`                            | Disables the `cmake` module.                 |

### Variables

| 変数        | 設定例       | 説明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v3.17.3` | The version of cmake                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

## コマンド実行時間

`cmd_duration`モジュールは、最後のコマンドの実行にかかった時間を示します。 モジュールが表示されるのは、コマンドが2秒以上かかった場合、または`min_time`値が存在する場合のみです。

::: warning BashでDEBUGトラップをhookしない

`bash`でStarshipを実行している場合、 `eval $(starship init $0)`実行した後に`DEBUG`トラップをフックしないでください。そうしないと、このモジュールが**おそらくですが**壊れます。

:::

preexecのような機能を必要とするBashユーザーは、 [rcalorasのbash_preexecフレームワーク](https://github.com/rcaloras/bash-preexec)を使用できます。 `eval $(starship init $0)` を実行する前に、`preexec_functions`、および`precmd_functions`定義するだけで、通常どおり続行します。

### オプション

| Option              | デフォルト                         | 説明                          |
| ------------------- | ----------------------------- | --------------------------- |
| `min_time`          | `2_000`                       | 実行時間を表示する最短期間（ミリ秒単位）です。     |
| `show_milliseconds` | `false`                       | 実行時間の秒に加えてミリ秒を表示します。        |
| `format`            | `"took [$duration]($style) "` | The format for the module.  |
| `style`             | `"bold yellow"`               | モジュールのスタイルです。               |
| `disabled`          | `false`                       | `cmd_duration`モジュールを無効にします。 |

### Variables

| 変数        | 設定例      | 説明                                      |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | The time it took to execute the command |
| style\* |          | Mirrors the value of option `style`     |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

`$CONDA_DEFAULT_ENV`が設定されている場合、`conda`モジュールは現在のcondaの環境を表示します。

::: tip

Note: これはconda自身の プロンプト修飾子 を抑制しません。`conda config --set changeps1 False` で実行することができます。

:::

### オプション

| Option              | デフォルト                              | 説明                                                                                                               |
| ------------------- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                | 環境が`conda create -p [path]`で作成された場合、環境パスが切り捨てられるディレクトリ数。 `0`は切り捨てがないことを意味します。  [`directory`](#directory)もご覧ください。 |
| `symbol`            | `"🅒 "`                             | 環境名の直前に使用されるシンボルです。                                                                                              |
| `style`             | `"bold green"`                     | モジュールのスタイルです。                                                                                                    |
| `format`            | `"[$symbol$environment]($style) "` | The format for the module.                                                                                       |
| `disabled`          | `false`                            | `conda`モジュールを無効にします。                                                                                             |

### Variables

| 変数          | 設定例          | 説明                                   |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current conda environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*   |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

`crystal`モジュールには、現在インストールされているCrystalのバージョンが表示されます。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`shard.yml`ファイルが含まれている
- カレントディレクトリに`.cr`の拡張子のファイルが含まれている

### オプション

| Option     | デフォルト                              | 説明                             |
| ---------- | ---------------------------------- | ------------------------------ |
| `symbol`   | `"🔮 "`                             | Crystalのバージョンを表示する前に使用される記号です。 |
| `style`    | `"bold red"`                       | モジュールのスタイルです。                  |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.     |
| `disabled` | `false`                            | `crystal`モジュールを無効にします。         |

### Variables

| 変数        | 設定例       | 説明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v0.32.1` | The version of `crystal`             |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## ディレクトリ

`directory`モジュールには、現在のディレクトリへのパスが表示され、3つの親フォルダは切り捨てられます。 ディレクトリは、現在のgitリポジトリであるとルートとなります。

fishスタイルのpwdオプションを使用すると、切り捨てられたパスを非表示にする代わりに、オプションで有効にした番号に基づいて各ディレクトリの短縮名が表示されます。

例として、`~/Dev/Nix/nixpkgs/pkgs`で、`nixpkgs`がリポジトリルートであり、オプションが`1`に設定されている場合を挙げます。 以前は`nixpkgs/pkgs`でしたが、`~/D/N/nixpkgs/pkgs`が表示されます。

### オプション

| 変数                       | デフォルト                                           | 説明                                                    |
| ------------------------ | ----------------------------------------------- | ----------------------------------------------------- |
| `truncation_length`      | `3`                                             | 現在のディレクトリを切り捨てる親フォルダーの数です。                            |
| `truncate_to_repo`       | `true`                                          | 現在いるgitリポジトリのルートに切り捨てるかどうかです。                         |
| `format`                 | `"[$path]($style)[$lock_symbol]($lock_style) "` | The format for the module.                            |
| `style`                  | `"bold cyan"`                                   | モジュールのスタイルです。                                         |
| `disabled`               | `false`                                         | `directory`モジュールを無効にします。                              |
| `read_only_symbol`       | `"🔒"`                                           | The symbol indicating current directory is read only. |
| `read_only_symbol_style` | `"red"`                                         | The style for the read only symbol.                   |

<details>
<summary>このモジュールは、どのようにディレクトリを表示するかについての高度なオプションをいくつか持っています。</summary>

| Advanced Option             | デフォルト  | 説明                                               |
| --------------------------- | ------ | ------------------------------------------------ |
| `substitutions`             |        | A table of substitutions to be made to the path. |
| `fish_style_pwd_dir_length` | `0`    | fish shellのpwdパスロジックを適用するときに使用する文字数です。           |
| `use_logical_path`          | `true` | OSからのパスの代わりに、シェル(`PWD`) によって提供される論理パスを表示します。     |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variables

| 変数        | 設定例                   | 説明                                  |
| --------- | --------------------- | ----------------------------------- |
| path      | `"D:/Projects"`       | The current directory path          |
| style\* | `"black bold dimmed"` | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker Context

`docker_context`モジュールは、 [Dockerコンテキスト](https://docs.docker.com/engine/context/working-with-contexts/)が`デフォルト`に設定されていない場合、現在アクティブな <1>Dockerコンテキストを表示します。

### オプション

| Option            | デフォルト                              | 説明                                                                                      |
| ----------------- | ---------------------------------- | --------------------------------------------------------------------------------------- |
| `format`          | `"via [$symbol$context]($style) "` | The format for the module.                                                              |
| `symbol`          | `"🐳 "`                             | The symbol used before displaying the Docker context.                                   |
| `style`           | `"blue bold"`                      | モジュールのスタイルです。                                                                           |
| `only_with_files` | `false`                            | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `disabled`        | `true`                             | `docker_context`モジュールを無効にします。                                                           |

### Variables

| 変数        | 設定例            | 説明                                   |
| --------- | -------------- | ------------------------------------ |
| context   | `test_context` | The current docker context           |
| symbol    |                | Mirrors the value of option `symbol` |
| style\* |                | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

`dotnet` モジュールはカレントディレクトリに関係する.NET Core SDKのバージョンを表示します。 もし SDKは現在のディレクトリに固定されているのであれば、その固定されたバージョンが表示されます。 それ以外の場合、モジュール SDKの最新のインストールバージョンを示します。

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

内部的に、このモジュールは自身のバージョン検知のメカニズムを利用します。 `dotnet --version` を実行するより2倍速く実行できますが、.NET project一般的でないディレクトリlayoutの場合は間違ったバージョンが示されてしまうことがあります。 速度よりも精度が重要な場合は、次の方法でメカニズムを無効にできます。 モジュールオプションで`heuristic = false `を設定します。

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### オプション

| Option      | デフォルト                                    | 説明                                   |
| ----------- | ---------------------------------------- | ------------------------------------ |
| `format`    | `"v[$symbol$version( 🎯 $tfm)]($style) "` | The format for the module.           |
| `symbol`    | `•NET "`                                 | dotnetのバージョンを表示する前に使用される記号です。        |
| `heuristic` | `true`                                   | より高速なバージョン検出を使用して、starshipの動作を維持します。 |
| `style`     | `"bold blue"`                            | モジュールのスタイルです。                        |
| `disabled`  | `false`                                  | `dotnet`モジュールを無効にします。                |

### Variables

| 変数        | 設定例              | 説明                                                                 |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Mirrors the value of option `symbol`                               |
| style\* |                  | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`mix.exs`ファイルが含まれている.

### オプション

| Option     | デフォルト                                                         | 説明                                                              |
| ---------- | ------------------------------------------------------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`                                                        | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"`                                               | モジュールのスタイルです。                                                   |
| `format`   | `"via [$symbol$version \\(OTP $otp_version\\)]($style) "` | The format for the module elixir.                               |
| `disabled` | `false`                                                       | Disables the `elixir` module.                                   |

### Variables

| 変数          | 設定例     | 説明                                   |
| ----------- | ------- | ------------------------------------ |
| version     | `v1.10` | The version of `elixir`              |
| otp_version |         | The otp version of `elixir`          |
| symbol      |         | Mirrors the value of option `symbol` |
| style\*   |         | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

`elm`モジュールは、現在インストールされているElmのバージョンを示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`elm.json`ファイルが含まれている
- カレントディレクトリに`elm-package.json`ファイルが含まれている
- カレントディレクトリに`.elm-version`ファイルが含まれている
- カレントディレクトリに`elm-stuff`フォルダが含まれている
- カレントディレクトリに`*.elm`ファイルが含まれている

### オプション

| Option     | デフォルト                              | 説明                                              |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🌳 "`                             | A format string representing the symbol of Elm. |
| `style`    | `"cyan bold"`                      | モジュールのスタイルです。                                   |
| `disabled` | `false`                            | `elm`モジュールを無効にします。                              |

### Variables

| 変数        | 設定例       | 説明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v0.19.1` | The version of `elm`                 |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## 環境変数

`env_var`モジュールは、選択された環境変数の現在の値を表示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- `variable`オプションが、既存の環境変数と一致する
- `variable`オプションが定義されておらず、`default`オプションが定義されている

### オプション

| Option     | デフォルト                          | 説明                                    |
| ---------- | ------------------------------ | ------------------------------------- |
| `symbol`   |                                | 環境変数を表示する前に使用される記号です。                 |
| `variable` |                                | 表示される環境変数です。                          |
| `default`  |                                | 上のvariableが定義されていない場合に表示されるデフォルトの値です。 |
| `format`   | `"with [$env_value]($style) "` | The format for the module.            |
| `disabled` | `false`                        | `env_var`モジュールを無効にします。                |

### Variables

| 変数        | 設定例                                         | 説明                                         |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if *variable* would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Mirrors the value of option `symbol`       |
| style\* | `black bold dimmed`                         | Mirrors the value of option `style`        |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`rebar.config`ファイルが含まれている.
- カレントディレクトリに`erlang.mk`ファイルが含まれている.

### オプション

| Option     | デフォルト                              | 説明                                                       |
| ---------- | ---------------------------------- | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`                             | The symbol used before displaying the version of erlang. |
| `style`    | `"bold red"`                       | モジュールのスタイルです。                                            |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                               |
| `disabled` | `false`                            | Disables the `erlang` module.                            |

### Variables

| 変数        | 設定例       | 説明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v22.1.3` | The version of `erlang`              |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Git ブランチ

`git_branch`モジュールは、現在のディレクトリにあるリポジトリのアクティブなブランチを表示します。

### オプション

| Option              | デフォルト                            | 説明                                                                               |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------- |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.  Use `"$branch"` to refer to the current branch name. |
| `symbol`            | `" "`                           | A format string representing the symbol of git branch.                           |
| `style`             | `"bold purple"`                  | モジュールのスタイルです。                                                                    |
| `truncation_length` | `2^63 - 1`                       | Truncates a git branch to X graphemes.                                           |
| `truncation_symbol` | `"…"`                            | ブランチ名切り捨てられていることを示すための記号です。 You can use `""` for no symbol.                      |
| `disabled`          | `false`                          | `git_branch`モジュールを無効にします。                                                        |

### Variables

| 変数        | 設定例      | 説明                                                                                                   |
| --------- | -------- | ---------------------------------------------------------------------------------------------------- |
| branch    | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached HEAD). |
| symbol    |          | Mirrors the value of option `symbol`                                                                 |
| style\* |          | Mirrors the value of option `style`                                                                  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git コミット

`git_commit`モジュールは、現在のディレクトリにあるリポジトリの現在のコミットハッシュを表示します。

### オプション

| Option               | デフォルト                          | 説明                                     |
| -------------------- | ------------------------------ | -------------------------------------- |
| `commit_hash_length` | `7`                            | 表示されるgitコミットハッシュの長さです。                 |
| `format`             | `"[\\($hash\\)]($style) "` | The format for the module.             |
| `style`              | `"bold green"`                 | モジュールのスタイルです。                          |
| `only_detached`      | `true`                         | 切り離されたHEAD状態のときのみgit commit hashを表示します |
| `disabled`           | `false`                        | `git_commit`モジュールを無効にします。              |

### Variables

| 変数        | 設定例       | 説明                                  |
| --------- | --------- | ----------------------------------- |
| hash      | `b703eb3` | The current git commit hash         |
| style\* |           | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[git_commit]
truncation_length = 4
```

## Git の進行状態

`git_state`モジュールはgitディレクトリの進行状態を表します。 (例: _REBASING_, _BISECTING_, その他) 進捗情報がある場合(例: REBASING 3/10)はその情報も表示されます。

### オプション

| Option         | デフォルト                                                               | 説明                                                                                      |
| -------------- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                        | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                         | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                       | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                                  | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                       | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                              | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                       | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                     | モジュールのスタイルです。                                                                           |
| `format`       | `"[\\($state( $progress_current/$progress_total)\\)]($style) "` | The format for the module.                                                              |
| `disabled`     | `false`                                                             | `git_state`モジュールを無効にします。                                                                |

### Variables

| 変数               | 設定例        | 説明                                  |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | The current state of the repo       |
| progress_current | `1`        | The current operation progress      |
| progress_total   | `2`        | The total operation progress        |
| style\*        |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[git_state]
format = "[\\($state( $progress_current of $progress_total)\\)]($style) "
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git の状態

`git_status`モジュールは、現在のディレクトリのリポジトリの状態を表すシンボルを表示します。

### オプション

| Option            | デフォルト                                           | 説明                                  |
| ----------------- | ----------------------------------------------- | ----------------------------------- |
| `format`          | "([\[$all_status$ahead_behind\]]($style) )" | The default format for `git_status` |
| `conflicted`      | `"="`                                           | このブランチにはマージの競合があります。                |
| `ahead`           | `"⇡"`                                           | The format of `ahead`               |
| `behind`          | `"⇣"`                                           | The format of `behind`              |
| `diverged`        | `"⇕"`                                           | The format of `diverged`            |
| `untracked`       | `"?"`                                           | The format of `untracked`           |
| `stashed`         | `"$"`                                           | The format of `stashed`             |
| `modified`        | `"!"`                                           | The format of `modified`            |
| `staged`          | `"+"`                                           | The format of `staged`              |
| `renamed`         | `"»"`                                           | The format of `renamed`             |
| `deleted`         | `"✘"`                                           | The format of `deleted`             |
| `show_sync_count` | `false`                                         | 追跡されているブランチの先行/後方カウントを表示します。        |
| `style`           | `"bold red"`                                    | モジュールのスタイルです。                       |
| `disabled`        | `false`                                         | `git_status`モジュールを無効にします。           |

### Variables

The following variables can be used in `format`:

| 変数             | 説明                                                                                            |
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

| 変数             | 説明                                             |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| 変数      | 説明            |
| ------- | ------------- |
| `count` | ファイルの数を表示します。 |

### 設定例

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

`golang`モジュールは、現在インストールされているGolangのバージョンを示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`go.mod`ファイルが含まれている
- カレントディレクトリに`go.sum`ファイルが含まれている
- カレントディレクトリに`glide.yaml`ファイルが含まれている
- カレントディレクトリに`Gopkg.yml`ファイルが含まれている
- カレントディレクトリに`Gopkg.lock`ファイルが含まれている
- カレントディレクトリに`.go-version`ファイルが含まれている
- カレントディレクトリに`Godeps`ファイルが含まれている
- カレントディレクトリに`.go`の拡張子のファイルが含まれている

### オプション

| Option     | デフォルト                              | 説明                                             |
| ---------- | ---------------------------------- | ---------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                     |
| `symbol`   | `"🐹 "`                             | A format string representing the symbol of Go. |
| `style`    | `"bold cyan"`                      | モジュールのスタイルです。                                  |
| `disabled` | `false`                            | `golang`モジュールを無効にします。                          |

### Variables

| 変数        | 設定例       | 説明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v1.12.1` | The version of `go`                  |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of Helm. 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`helmfile.yaml`ファイルが含まれている
- The current directory contains a `Chart.yaml` file

### オプション

| Option     | デフォルト                              | 説明                                               |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"⎈ "`                             | A format string representing the symbol of Helm. |
| `style`    | `"bold white"`                     | モジュールのスタイルです。                                    |
| `disabled` | `false`                            | Disables the `helm` module.                      |

### Variables

| 変数        | 設定例      | 説明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v3.1.1` | The version of `helm`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## ホスト名

`hostname`モジュールには、システムのホスト名が表示されます。

### オプション

| Option     | デフォルト                       | 説明                                                                          |
| ---------- | --------------------------- | --------------------------------------------------------------------------- |
| `ssh_only` | `true`                      | SSHセッションに接続されている場合にのみホスト名を表示します。                                            |
| `trim_at`  | `"."`                       | この文字が最初にマッチするまでをホスト名と認識します。 `"."`は最初の. までをホスト名として認識します。 `""`を指定した場合トリムしません。 |
| `format`   | `"on [$hostname]($style) "` | The format for the module.                                                  |
| `style`    | `"bold dimmed green"`       | モジュールのスタイルです。                                                               |
| `disabled` | `false`                     | `hostname`モジュールを無効にします。                                                     |

### Variables

| 変数        | 設定例 | 説明                                   |
| --------- | --- | ------------------------------------ |
| number    | `1` | The number of jobs                   |
| symbol    |     | Mirrors the value of option `symbol` |
| style\* |     | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

`java`モジュールは、現在インストールされているJavaのバージョンを示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに `pom.xml`, `build.gradle.kts`, `build.sbt` ,もしくは`.java-version`が含まれている
- カレントディレクトリに拡張子が`.java`, `.class`, `.gradle`, もしくは`.jar`のファイルが含まれている

### オプション

| Option     | デフォルト                                  | 説明                                              |
| ---------- | -------------------------------------- | ----------------------------------------------- |
| `format`   | `"via [${symbol}${version}]($style) "` | The format for the module.                      |
| `symbol`   | `"☕ "`                                 | A format string representing the symbol of Java |
| `style`    | `"red dimmed"`                         | モジュールのスタイルです。                                   |
| `disabled` | `false`                                | `java`モジュールを無効にします。                             |

### Variables

| 変数        | 設定例   | 説明                                   |
| --------- | ----- | ------------------------------------ |
| version   | `v14` | The version of `java`                |
| symbol    |       | Mirrors the value of option `symbol` |
| style\* |       | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## ジョブ

`jobs`モジュールには、実行中のジョブの現在の数が表示されます。 このモジュールは、実行中のバックグラウンドジョブがある場合にのみ表示されます。 1つ以上のジョブがある、または`threshold`に指定した値以上にジョブがある場合は実行中のジョブの数を表示します。

### オプション

| Option      | デフォルト                         | 説明                                               |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | 超過した場合、ジョブの数を表示します。                              |
| `format`    | `"[$symbol$number]($style) "` | The format for the module.                       |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | モジュールのスタイルです。                                    |
| `disabled`  | `false`                       | `jobs`モジュールを無効にします。                              |

### Variables

| 変数        | 設定例 | 説明                                   |
| --------- | --- | ------------------------------------ |
| number    | `1` | The number of jobs                   |
| symbol    |     | Mirrors the value of option `symbol` |
| style\* |     | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

`julia`モジュールは、現在インストールされているJuliaのバージョンを示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Project.toml`ファイルが含まれている
- カレントディレクトリに`Manifest.toml`ファイルが含まれている
- カレントディレクトリに`.jl`の拡張子のファイルが含まれている

### オプション

| Option     | デフォルト                              | 説明                                                |
| ---------- | ---------------------------------- | ------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                        |
| `symbol`   | `"ஃ "`                             | A format string representing the symbol of Julia. |
| `style`    | `"bold purple"`                    | モジュールのスタイルです。                                     |
| `disabled` | `false`                            | `julia`モジュールを無効にします。                              |

### Variables

| 変数        | 設定例      | 説明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.4.0` | The version of `julia`               |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| Option                  | デフォルト                                                    | 説明                                                                    |
| ----------------------- | -------------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`                | `"☸ "`                                                   | A format string representing the symbol displayed before the Cluster. |
| `format`                | `"on [$symbol$context( \\($namespace\\))]($style) "` | The format for the module.                                            |
| `style`                 | `"cyan bold"`                                            | モジュールのスタイルです。                                                         |
| `namespace_spaceholder` | `none`                                                   | The value to display if no namespace was found.                       |
| `context_aliases`       |                                                          | Table of context aliases to display.                                  |
| `disabled`              | `true`                                                   | Disables the `kubernetes` module.                                     |

### Variables

| 変数        | 設定例                  | 説明                                       |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Mirrors the value of option `symbol`     |
| style\* |                      | Mirrors the value of option `style`      |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[kubernetes]
format = "on [⛵ $context \\($namespace\\)](dimmed green) "
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## 改行

`line_break`モジュールは、プロンプトを2行に分割します。

### オプション

| Option     | デフォルト   | 説明                                    |
| ---------- | ------- | ------------------------------------- |
| `disabled` | `false` | `line_break`モジュールを無効にして、プロンプトを1行にします。 |

### 設定例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## メモリ使用量

`memory_usage</ 0>モジュールは、現在のシステムメモリとスワップ使用量を示します。</p>

<p spaces-before="0">デフォルトでは、システムスワップの合計がゼロ以外の場合、スワップ使用量が表示されます。</p>

<p spaces-before="0">::: tip</p>

<p spaces-before="0">このモジュールはデフォルトで無効になっています。
有効にするには、設定ファイルで<code>disabled`を`false`に設定します。

:::

### オプション

| Option      | デフォルト                                         | 説明                          |
| ----------- | --------------------------------------------- | --------------------------- |
| `threshold` | `75`                                          | この閾値を超えない限り、メモリ使用率は表示されません。 |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | The format for the module.  |
| `symbol`    | `"🐏"`                                         | メモリ使用率を表示する前に使用される記号です。     |
| `style`     | `"bold dimmed white"`                         | モジュールのスタイルです。               |
| `disabled`  | `true`                                        | `memory_usage`モジュールを無効にします。 |

### Variables

| 変数            | 設定例           | 説明                                                                 |
| ------------- | ------------- | ------------------------------------------------------------------ |
| ram           | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct       | `48%`         | The percentage of the current system memory.                       |
| swap\**     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\** | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol        | `🐏`           | Mirrors the value of option `symbol`                               |
| style\*     |               | Mirrors the value of option `style`                                |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

### 設定例

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

## Mercurial ブランチ

` hg_branch `モジュールは、現在のディレクトリにあるリポジトリのアクティブなブランチを示します。

### オプション

| Option              | デフォルト                            | 説明                                                                                           |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | モジュールのスタイルです。                                                                                |
| `format`            | `"on [$symbol$branch]($style) "` | The format for the module.                                                                   |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`                            | ブランチ名切り捨てられていることを示すための記号です。                                                                  |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variables

| 変数        | 設定例      | 説明                                   |
| --------- | -------- | ------------------------------------ |
| branch    | `master` | The active mercurial branch          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of Nim. 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`nim.cfg`ファイルが含まれている
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### オプション

| Option     | デフォルト                              | 説明                                                    |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module                             |
| `symbol`   | `"👑 "`                             | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"`                    | モジュールのスタイルです。                                         |
| `disabled` | `false`                            | Disables the `nim` module.                            |

### Variables

| 変数        | 設定例      | 説明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.2.0` | The version of `nimc`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

`nix_shell`モジュールは、nix-shell環境を示しています。 このモジュールは、nixシェル環境内にあるときに表示されます。

### オプション

| Option       | デフォルト                                              | 説明                                                    |
| ------------ | -------------------------------------------------- | ----------------------------------------------------- |
| `format`     | `"via [$symbol$state( \\($name\\))]($style) "` | The format for the module.                            |
| `symbol`     | `"❄️  "`                                           | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                      | モジュールのスタイルです。                                         |
| `impure_msg` | `"impure"`                                         | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                           | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                            | `nix_shell`モジュールを無効にします。                              |

### Variables

| 変数        | 設定例     | 説明                                   |
| --------- | ------- | ------------------------------------ |
| state     | `pure`  | The state of the nix-shell           |
| name      | `lorri` | The name of the nix-shell            |
| symbol    |         | Mirrors the value of option `symbol` |
| style\* |         | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = "via [☃️ $state( \\($name\\))](bold blue) "
```

## NodeJS

`nodejs`モジュールは、現在インストールされているNodeJSのバージョンを示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`package.json`ファイルが含まれている
- The current directory contains a `.node-version` file
- カレントディレクトリに`node_modules`ディレクトリが含まれている
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### オプション

| Option     | デフォルト                              | 説明                                                 |
| ---------- | ---------------------------------- | -------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                         |
| `symbol`   | `"⬢ "`                             | A format string representing the symbol of NodeJS. |
| `style`    | `"bold green"`                     | モジュールのスタイルです。                                      |
| `disabled` | `false`                            | `nodejs`モジュールを無効にします。                              |

###  Variables

| 変数        | 設定例        | 説明                                   |
| --------- | ---------- | ------------------------------------ |
| version   | `v13.12.0` | The version of `node`                |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## パッケージのバージョン

`package`モジュールは、現在のディレクトリがパッケージのリポジトリである場合に表示され、現在のバージョンが表示されます。 The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia` and `mix` packages.

- **npm** – `npm`パッケージバージョンは、現在のディレクトリにある`package.json`から抽出されます
- **cargo** – `cargo`パッケージバージョンは、現在のディレクトリにある`Cargo.toml`から抽出されます。
- **poetry** – `poetry`パッケージバージョンは、現在のディレクトリにある`pyproject.toml`から抽出されます
- **composer** – `composer`パッケージバージョンは、現在のディレクトリにある`composer.json`から抽出されます
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present

> ⚠️ 表示されるバージョンは、パッケージマネージャーではなく、ソースコードが現在のディレクトリにあるパッケージのバージョンです。

### オプション

| Option            | デフォルト                              | 説明                                                        |
| ----------------- | ---------------------------------- | --------------------------------------------------------- |
| `format`          | `"via [$symbol$version]($style) "` | The format for the module.                                |
| `symbol`          | `"📦 "`                             | パッケージのバージョンを表示する前に使用される記号です。                              |
| `style`           | `"bold 208"`                       | モジュールのスタイルです。                                             |
| `display_private` | `false`                            | Enable displaying version for packages marked as private. |
| `disabled`        | `false`                            | `package` モジュールを無効にします。                                   |

### Variables

| 変数        | 設定例      | 説明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v1.0.0` | The version of your package          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### オプション

| Option     | デフォルト                              | 説明                                                      |
| ---------- | ---------------------------------- | ------------------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format string for the module.                       |
| `symbol`   | `"🐫 "`                             | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"`                    | モジュールのスタイルです。                                           |
| `disabled` | `false`                            | Disables the `ocaml` module.                            |

### Variables

| 変数        | 設定例       | 説明                                   |
| --------- | --------- | ------------------------------------ |
| version   | `v4.10.0` | The version of `ocaml`               |
| symbol    |           | Mirrors the value of option `symbol` |
| style\* |           | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## PHP

`php`モジュールは、現在インストールされているPHPのバージョンを示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`composer.json`ファイルが含まれている
- The current directory contains a `.php-version` file
- カレントディレクトリに`.php`の拡張子のファイルが含まれている

### オプション

| Option     | デフォルト                              | 説明                         |
| ---------- | ---------------------------------- | -------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module. |
| `symbol`   | `"🐘 "`                             | PHPのバージョンを表示する前に使用される記号です。 |
| `style`    | `"147 bold"`                       | モジュールのスタイルです。              |
| `disabled` | `false`                            | `php`モジュールを無効にします。         |

### Variables

| 変数        | 設定例      | 説明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v7.3.8` | The version of `php`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.python-version`ファイルが含まれている
- カレントディレクトリに`requirements.txt`ファイルが含まれている
- カレントディレクトリに`pyproject.toml`ファイルが含まれている
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- カレントディレクトリに`Pipfile`ファイルが含まれている
- カレントディレクトリに`tox.ini`ファイルが含まれている
- カレントディレクトリに`setup.py`ファイルが含まれている
- The current directory contains a `__init__.py` file
- 仮想環境がアクティブである

### オプション

| Option               | デフォルト                                                          | 説明                                                                         |
| -------------------- | -------------------------------------------------------------- | -------------------------------------------------------------------------- |
| `format`             | `"via [${symbol}${version}( \\($virtualenv\\))]($style) "` | The format for the module.                                                 |
| `symbol`             | `"🐍 "`                                                         | A format string representing the symbol of Python                          |
| `style`              | `"yellow bold"`                                                | モジュールのスタイルです。                                                              |
| `pyenv_version_name` | `false`                                                        | pyenvを使用してPythonバージョンを取得します                                                |
| `scan_for_pyfiles`   | `true`                                                         | If false, Python files in the current directory will not show this module. |
| `disabled`           | `false`                                                        | `python`モジュールを無効にします。                                                      |

### Variables

| 変数         | 設定例             | 説明                                   |
| ---------- | --------------- | ------------------------------------ |
| version    | `"v3.8.1"`      | The version of `python`              |
| symbol     | `"🐍 "`          | Mirrors the value of option `symbol` |
| style      | `"yellow bold"` | Mirrors the value of option `style`  |
| virtualenv | `"venv"`        | The current `virtualenv` name        |

<details>
<summary>This module has some advanced configuration options.</summary>

| 変数              | デフォルト    | 説明                                                                            |
| --------------- | -------- | ----------------------------------------------------------------------------- |
| `python_binary` | `python` | Configures the python binary that Starship executes when getting the version. |

The `python_binary` variable changes the binary that Starship executes to get the version of Python, it doesn't change the arguments that are used.

```toml
# ~/.config/starship.toml

[python]
python_binary = "python3"
```

</details>

### 設定例

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

`ruby`モジュールは、現在インストールされているRubyのバージョンを示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Gemfile`ファイルが含まれている
- The current directory contains a `.ruby-version` file
- カレントディレクトリに`.rb`の拡張子のファイルが含まれている

### オプション

| Option     | デフォルト                              | 説明                                               |
| ---------- | ---------------------------------- | ------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                       |
| `symbol`   | `"💎 "`                             | A format string representing the symbol of Ruby. |
| `style`    | `"bold red"`                       | モジュールのスタイルです。                                    |
| `disabled` | `false`                            | `ruby`モジュールを無効にします。                              |

### Variables

| 変数        | 設定例      | 説明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v2.5.1` | The version of `ruby`                |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

`rust`モジュールには、現在インストールされているRustのバージョンが表示されます。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Cargo.toml`ファイルが含まれている
- カレントディレクトリに`.rs`の拡張子のファイルが含まれている

### オプション

| Option     | デフォルト                              | 説明                                              |
| ---------- | ---------------------------------- | ----------------------------------------------- |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                      |
| `symbol`   | `"🦀 "`                             | A format string representing the symbol of Rust |
| `style`    | `"bold red"`                       | モジュールのスタイルです。                                   |
| `disabled` | `false`                            | `rust`モジュールを無効にします。                             |

### Variables

| 変数        | 設定例               | 説明                                   |
| --------- | ----------------- | ------------------------------------ |
| version   | `v1.43.0-nightly` | The version of `rustc`               |
| symbol    |                   | Mirrors the value of option `symbol` |
| style\* |                   | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### オプション

| Option     | デフォルト                                | 説明                                               |
| ---------- | ------------------------------------ | ------------------------------------------------ |
| `format`   | `"[$symbol\\[$env\\]]($style) "` | The format for the module.                       |
| `symbol`   | `""`                                 | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`                 | モジュールのスタイルです。                                    |
| `disabled` | `false`                              | Disables the `singularity` module.               |

### Variables

| 変数        | 設定例          | 説明                                   |
| --------- | ------------ | ------------------------------------ |
| env       | `centos.img` | The current singularity image        |
| symbol    |              | Mirrors the value of option `symbol` |
| style\* |              | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[singularity]
format = "[📦 \\[$env\\]]($style) "
```

## Terraform

`terraform`モジュールには、現在選択されているterraformワークスペースとバージョンが表示されます。 デフォルトでは、Terraformのバージョンは表示されません。これは、多くのプラグインが使用されている場合、Terraformの現在のバージョンでは遅いためです。 If you still want to enable it, [follow the example shown below](#with-version). 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.terraform`フォルダが含まれている
- カレントディレクトリに`.tf`の拡張子のファイルが含まれている

### オプション

| Option     | デフォルト                                | 説明                                                    |
| ---------- | ------------------------------------ | ----------------------------------------------------- |
| `format`   | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`   | `"💠 "`                               | A format string shown before the terraform workspace. |
| `style`    | `"bold 105"`                         | モジュールのスタイルです。                                         |
| `disabled` | `false`                              | `terraform`モジュールを無効にします。                              |

### Variables

| 変数        | 設定例        | 説明                                   |
| --------- | ---------- | ------------------------------------ |
| version   | `v0.12.24` | The version of `terraform`           |
| workspace | `default`  | The current terraform workspace      |
| symbol    |            | Mirrors the value of option `symbol` |
| style\* |            | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

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

## 時刻

`time`モジュールは、現在の**現地**時間を示します。 `format`設定は、時間の表示方法を制御するために[`chrono`](https://crates.io/crates/chrono)クレートによって使用されます。 使用可能なオプションを確認するには、[chrono strftimeのドキュメント](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)をご覧ください。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| Option            | デフォルト                   | 説明                                                                                                    |
| ----------------- | ----------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                     |
| `use_12hr`        | `false`                 | 12時間のフォーマットを有効にします。                                                                                   |
| `time_format`     | この表の下を参照してください          | 時刻のフォーマットに使用される[クロノフォーマット文字列](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) です。     |
| `style`           | `"bold yellow"`         | モジュールのスタイルです。                                                                                         |
| `utc_time_offset` | `"local"`               | 使用するUTCオフセットを設定します。 Range from -24 &lt; x &lt; 24. フロートが30/45分のタイムゾーンオフセットに対応できるようにします。   |
| `disabled`        | `true`                  | `time`モジュールを無効にします。                                                                                   |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. それ以外の場合、デフォルトは`"%T"`です。 Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| 変数        | 設定例        | 説明                                  |
| --------- | ---------- | ----------------------------------- |
| 時刻        | `13:08:10` | The current time.                   |
| style\* |            | Mirrors the value of option `style` |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[\\[ $time \\]]($style) "
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## ユーザー名

`username`モジュールには、アクティブなユーザーのユーザー名が表示されます。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントユーザーがroot
- カレントユーザーが、ログインしているユーザーとは異なる
- ユーザーがSSHセッションとして接続されている
- `show_always`変数がtrueに設定されている

### オプション

| Option        | デフォルト                    | 説明                         |
| ------------- | ------------------------ | -------------------------- |
| `style_root`  | `"bold red"`             | ユーザーがrootのときに使用されるスタイルです。  |
| `style_user`  | `"bold yellow"`          | 非rootユーザーに使用されるスタイルです。     |
| `format`      | `"via [$user]($style) "` | The format for the module. |
| `show_always` | `false`                  | `username` モジュールを常に表示します。  |
| `disabled`    | `false`                  | `username` モジュールを無効にします。   |

### Variables

| 変数      | 設定例          | 説明                                                                                          |
| ------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style` | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`  | `"matchai"`  | The currently logged-in user ID.                                                            |

### 設定例

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

The `zig` module shows the currently installed version of Zig. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `.zig` file

### オプション

| Option     | デフォルト                              | 説明                                                    |
| ---------- | ---------------------------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`                             | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"`                    | モジュールのスタイルです。                                         |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                            |
| `disabled` | `false`                            | Disables the `zig` module.                            |

### Variables

| 変数        | 設定例      | 説明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `v0.6.0` | The version of `zig`                 |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

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

### オプション

| Option        | デフォルト                         | 説明                                                                                                                         |
| ------------- | ----------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                               | The command whose output should be printed.                                                                                |
| `when`        |                               | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                               | [この表の下を参照してください](#custom-command-shell)                                                                                    |
| `説明`          | `"<custom module>"`     | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                          | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                          | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                          | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                          | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                | モジュールのスタイルです。                                                                                                              |
| `format`      | `"[$symbol$output]($style) "` | The format for the module.                                                                                                 |
| `disabled`    | `false`                       | Disables this `custom` module.                                                                                             |

### Variables

| 変数        | 説明                                     |
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

### 設定例

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

The `purescript` module shows the currently installed version of PureScript version. 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`spago.dhall`ファイルが含まれている
- The current directory contains a \*.purs files

### オプション

| Option     | デフォルト                              | 説明                                                           |
| ---------- | ---------------------------------- | ------------------------------------------------------------ |
| `format`   | `"via [$symbol$version]($style) "` | The format for the module.                                   |
| `symbol`   | `"<=> "`                     | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"`                     | モジュールのスタイルです。                                                |
| `disabled` | `false`                            | Disables the `purescript` module.                            |

### Variables

| 変数        | 設定例      | 説明                                   |
| --------- | -------- | ------------------------------------ |
| version   | `0.13.5` | The version of `purescript`          |
| symbol    |          | Mirrors the value of option `symbol` |
| style\* |          | Mirrors the value of option `style`  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```
