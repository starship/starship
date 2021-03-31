# 設定

Starshipの設定を開始するには、`~/.config/starship.toml` ファイルを作成します。

```sh
mkdir -p ~/.config && starship print-config --default > ~/.config/starship.toml
```

Starshipのすべての設定は、この[TOML](https://github.com/toml-lang/toml)ファイルで行われます。

```toml
# Inserts a blank line between shell prompts
add_newline = true

# Replace the "❯" symbol in the prompt with "➜"
[character]                            # The name of the module we are configuring is "character"
success_symbol = "[➜](bold green)"     # The "success_symbol" segment is being set to "➜" with the color "bold green"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

You can change default configuration file location with `STARSHIP_CONFIG` environment variable:

```sh
export STARSHIP_CONFIG=~/.starship/config.toml
```

PowerShell (Windows) で同様に `$PROFILE`にこの行を追加します。

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship\config.toml"
```

### ロギング

デフォルトでは、Starship は警告やエラーログを `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log` という名前のファイルに出力します。このセッションキーはターミナルのインスタンスに相当します。しかし、これは `STARSHIP_CACHE` という環境変数を使って変更できます： しかし、これは `STARSHIP_CACHE` という環境変数を使って変更できます：

```sh
export STARSHIP_CACHE=~/.starship/cache
```

PowerShell (Windows) で同様に `$PROFILE`にこの行を追加します。

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### 用語

**モジュール**: OSのコンテキスト情報に基づいて情報を提供するプロンプト内のコンポーネントです。 たとえば、現在のディレクトリがNodeJSプロジェクトである場合、「nodejs」モジュールは、現在コンピューターにインストールされているNodeJSのバージョンを表示します。

**変数**: モジュールが提供する情報を含むサブコンポーネントを小さくする。 例えば、"nodejs" モジュール内の "version" 変数には、NodeJS の現在のバージョンが含まれています。

慣例により、ほとんどのモジュールにはデフォルトの端末色の接頭辞（「nodejs」の`via` など）と接尾辞として空のスペースがあります。

### 文字列のフォーマット

文字列の書式は、モジュールがすべての変数を出力する書式です。 ほとんどのモジュールには、モジュールの表示形式を設定する `format` というエントリがあります。 テキスト、変数、およびテキストグループをフォーマット文字列で使用できます。

#### 変数

変数には、 `$` 記号と、その変数の名前が続きます。 変数の名前は、文字、数字、 `_` のみを含みます。

例：

- `$version` は、`version` という名前の変数を持つフォーマット文字列です。
- `$git_branch$git_commit` は `git_branch` と `git_commit` という2つの変数を持つフォーマット文字列です。
- `$git_branch $git_commit` には空白で区切られた 2 つの変数があります。

#### テキストグループ

テキストグループは二つの異なる部分で構成されています。

`[]`で囲まれている最初の部分は、 [フォーマット文字列](#format-strings) です。 テキスト、変数、または入れ子になったテキストグループを追加できます。

2 番目の部分では、 `()`で囲まれている [スタイル文字列](#style-strings) です。 これは最初のパートのスタイルを使用することができます。

例：

- `[on](red bold)` は文字列 `on` に太字のテキストを赤色で表示します。
- `[⌘ $version](bold green)` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `[a [b](red) c](green)` は  `a b c` を  `b` だけ赤色に表示し、 `a` と `c`  を緑色に表示します。

#### スタイルの設定

Starshipのほとんどのモジュールでは、表示スタイルを設定できます。 これは、設定を指定する文字列であるエントリ（`style`）で行われます。 スタイル文字列の例とその機能を次に示します。 完全な構文の詳細については、詳細は [高度な設定](/advanced-config/)を参照してください 。

- `"fg:green bg:blue"` は、青色の背景に緑色のテキストを設定します
- `"bg:blue fg:bright-green"` は、青色の背景に明るい緑色のテキストを設定します
- `"bold fg:27"` は、 [ANSIカラー](https://i.stack.imgur.com/KTSQa.png) 27の太字テキストを設定します
- `"underline bg:#bf5700"` は、焦げたオレンジ色の背景に下線付きのテキストを設定します
- `"bold italic fg:purple"`は、紫色の太字斜体のテキストを設定します
- `""` はすべてのスタイルを明示的に無効にします

スタイリングがどのように見えるかは、端末エミュレータによって制御されることに注意してください。 たとえば、一部の端末エミュレータはテキストを太字にする代わりに色を明るくします。また、一部のカラーテーマは通常の色と明るい色と同じ値を使用します。 また、斜体のテキストを取得するには、端末で斜体をサポートする必要があります。

#### 条件付きフォーマット設定

`(` と `)` 内のすべての変数が空の場合、条件付き書式文字列はレンダリングされません。

例：

- `(@$region)` will show nothing if the variable `region` is `None`, otherwise `@` followed by the value of region.
- `(some text)` will always show nothing since there are no variables wrapped in the braces.
- When `$all` is a shortcut for `\[$a$b\]`, `($all)` will show nothing only if `$a` and `$b` are both `None`. This works the same as `(\[$a$b\] )`.

#### エスケープ可能な文字

以下の記号は、フォーマット文字列に特別な使用法があります。 次の記号を印刷したい場合は、バックスラッシュ(`\`)でエスケープする必要があります。

- \$
- \\
- [
- ]
- (
- )

`toml` は [独自のエスケープ構文](https://github.com/toml-lang/toml#user-content-string) を持っていることに注意してください。 It is recommended to use a literal string (`''`) in your config. If you want to use a basic string (`""`), pay attention to escape the backslash `\`.

For example, when you want to print a `$` symbol on a new line, the following configs for `format` are equivalent:

```toml
# 基本文字列と
format = "\n\\$"

# 複数行の基本文字列と
format = """

\\$"""

# リテラル文字列と
format = '''

\$'''
```

## プロンプト

これは、プロンプト全体のオプションのリストです。

### オプション

| オプション          | デフォルト                          | 説明                                        |
| -------------- | ------------------------------ | ----------------------------------------- |
| `format`       | [link](#default-prompt-format) | プロンプトの形式を設定します。                           |
| `scan_timeout` | `30`                           | ファイルをスキャンする際のタイムアウト時間 (milliseconds) です。  |
| `add_newline`  | `true`                         | Inserts blank line between shell prompts. |

### 設定例

```toml
# ~/.config/starship.toml

# カスタムフォーマットを利用します
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# starshipが現在のディレクトリ下のファイルをチェックするまで10ミリ秒待ちます

scan_timeout = 10

# Disable the blank line at the start of the prompt
add_newline = false
```

### デフォルトのプロンプトフォーマット

デフォルトの `format` は、空または `format` が指定されていない場合、プロンプトのフォーマットを定義するために使用されます。 デフォルトは次のとおりです。

```toml
format = "$all"

# Which is equivalent to
format = """
$username\
$hostname\
$shlvl\
$kubernetes\
$directory\
$vcsh\
$git_branch\
$git_commit\
$git_state\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dart\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$kotlin\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vagrant\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$env_var\
$crystal\
$custom\
$cmd_duration\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$shell\
$character"""
```

## AWS

`aws` モジュールは現在のAWSプロファイルが表示されます。 これは `~/.aws/config` に記述されている `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` 環境変数に基づいています。

[aws-vault](https://github.com/99designs/aws-vault)を使用する場合、プロファイル は`AWS_VAULT`env varから読み取られます。

When using [awsu](https://github.com/kreuzwerker/awsu) the profile is read from the `AWSU_PROFILE` env var.

### オプション

| オプション            | デフォルト                                               | 説明                            |
| ---------------- | --------------------------------------------------- | ----------------------------- |
| `format`         | `'on [$symbol($profile )(\($region\) )]($style)'` | moduleのフォーマットです。              |
| `symbol`         | `"☁️ "`                                             | 現在のAWSプロファイルを表示する前に表示される記号です。 |
| `region_aliases` |                                                     | AWS名に加えて表示するリージョンのエイリアスです。    |
| `style`          | `"bold yellow"`                                     | モジュールのスタイルです。                 |
| `disabled`       | `false`                                             | Disables the `aws` module.    |

### 変数

| 変数        | 設定例              | 説明                     |
| --------- | ---------------- | ---------------------- |
| region    | `ap-northeast-1` | 現在のAWSリージョン            |
| profile   | `astronauts`     | 現在のAWSプロファイル           |
| symbol    |                  | オプション `記号` の値をミラーする    |
| style\* |                  | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

#### すべてを表示

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### リージョンを表示

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

#### プロファイルを表示

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
```

## バッテリー

The `battery` module shows how charged the device's battery is and its current charging status. The module is only visible when the device's battery is below 10%.

### オプション

| オプション                | デフォルト                             | 説明                        |
| -------------------- | --------------------------------- | ------------------------- |
| `full_symbol`        | `""`                             | バッテリーが満タンのときに表示される記号です。   |
| `charging_symbol`    | `""`                             | バッテリーの充電中に表示される記号です。      |
| `discharging_symbol` | `""`                             | バッテリーが放電しているときに表示される記号です。 |
| `unknown_symbol`     | `""`                             | バッテリー状態が不明なときに表示される記号です。  |
| `empty_symbol`       | `""`                             | バッテリーが空のときに表示される記号です。     |
| `format`             | `"[$symbol$percentage]($style) "` | moduleのフォーマットです。          |
| `display`            | [link](#battery-display)          | モジュールの閾値とスタイルを表示します。      |
| `disabled`           | `false`                           | `battery`モジュールを無効にします。    |

### 設定例

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

### バッテリーの表示

The `display` configuration option is used to define when the battery indicator should be shown (threshold) and what it looks like (style). If no `display` is provided. デフォルトは次のとおりです。

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### オプション

The `display` option is an array of the following table.

| オプション       | 説明                             |
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

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways:

- 色の変更 (`赤`/`緑`)
- プロンプトの表示の変更 (`❯`/`✖`)

By default it only changes color. If you also want to change it's shape take a look at [this example](#with-custom-error-shape).

::: warning `error_symbol` is not supported on elvish shell. :::

### オプション

| オプション            | デフォルト               | 説明                                           |
| ---------------- | ------------------- | -------------------------------------------- |
| `format`         | `"$symbol"`         | テキスト入力の前に使用される書式文字列。                         |
| `success_symbol` | `"[❯](bold green)"` | 前のコマンドが成功した場合にテキスト入力の前に使用される書式文字列です。         |
| `error_symbol`   | `"[❯](bold red)"`   | 前のコマンドが失敗した場合にテキスト入力の前に使用される書式文字列です。         |
| `vicmd_symbol`   | `"[❮](bold green)"` | シェルが vim ノーマルモードの場合にテキスト入力の前に使用されるフォーマット文字列。 |
| `disabled`       | `false`             | `character`モジュールを無効にします。                     |

### 変数

| 変数     | 設定例 | 説明                                                                    |
| ------ | --- | --------------------------------------------------------------------- |
| symbol |     | A mirror of either `success_symbol`, `error_symbol` or `vicmd_symbol` |

### 設定例

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

The `cmake` module shows the currently installed version of CMake. By default the module will be activated if any of the following conditions are met:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### オプション

| オプション               | デフォルト                                  | 説明                                           |
| ------------------- | -------------------------------------- | -------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`   | moduleのフォーマットです。                             |
| `symbol`            | `"△ "`                                 | The symbol used before the version of cmake. |
| `detect_extensions` | `[]`                                   | Which extensions should trigger this module  |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | Which filenames should trigger this module   |
| `detect_folders`    | `[]`                                   | Which folders should trigger this module     |
| `style`             | `"bold blue"`                          | モジュールのスタイルです。                                |
| `disabled`          | `false`                                | Disables the `cmake` module.                 |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v3.17.3` | The version of cmake   |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

## コマンド実行時間

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### オプション

| オプション                | デフォルト                         | 説明                                                    |
| -------------------- | ----------------------------- | ----------------------------------------------------- |
| `min_time`           | `2_000`                       | 実行時間を表示する最短期間（ミリ秒単位）です。                               |
| `show_milliseconds`  | `false`                       | 実行時間の秒に加えてミリ秒を表示します。                                  |
| `format`             | `"took [$duration]($style) "` | moduleのフォーマットです。                                      |
| `style`              | `"bold yellow"`               | モジュールのスタイルです。                                         |
| `disabled`           | `false`                       | `cmd_duration`モジュールを無効にします。                           |
| `show_notifications` | `false`                       | Show desktop notifications when command completes.    |
| `min_time_to_notify` | `45_000`                      | Shortest duration for notification (in milliseconds). |

::: tip

Showing desktop notifications requires starship to be built with `rust-notify` support. You check if your starship supports notifications by running `STARSHIP_LOG=debug starship module cmd_duration -d 60000` when `show_notifications` is set to `true`.

:::

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| duration  | `16m40s` | コマンドの実行時間              |
| style\* |          | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

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

### オプション

| オプション               | デフォルト                                  | 説明                                                                                                               |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | 環境が`conda create -p [path]`で作成された場合、環境パスが切り捨てられるディレクトリ数。 `0`は切り捨てがないことを意味します。  [`directory`](#directory)もご覧ください。 |
| `symbol`            | `"🅒 "`                                 | 環境名の直前に使用されるシンボルです。                                                                                              |
| `style`             | `"bold green"`                         | モジュールのスタイルです。                                                                                                    |
| `format`            | `"via [$symbol$environment]($style) "` | moduleのフォーマットです。                                                                                                 |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                       |
| `disabled`          | `false`                                | `conda`モジュールを無効にします。                                                                                             |

### 変数

| 変数          | 設定例          | 説明                            |
| ----------- | ------------ | ----------------------------- |
| environment | `astronauts` | The current conda environment |
| symbol      |              | オプション `記号` の値をミラーする           |
| style\*   |              | オプション `style` の値をミラーする        |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

The `crystal` module shows the currently installed version of Crystal. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`shard.yml`ファイルが含まれている
- カレントディレクトリに`.cr`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                           |
| ------------------- | ------------------------------------ | -------------------------------------------- |
| `symbol`            | `"🔮 "`                               | Crystalのバージョンを表示する前に使用される記号です。               |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                |
| `detect_extensions` | `["cr"]`                             | Which extensions should trigger this module. |
| `detect_files`      | `["shard.yml"]`                      | Which filenames should trigger this module.  |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.    |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                             |
| `disabled`          | `false`                              | Disables the `crystal` module.               |

### 変数

| 変数        | 設定例       | 説明                       |
| --------- | --------- | ------------------------ |
| version   | `v0.32.1` | The version of `crystal` |
| symbol    |           | オプション `記号` の値をミラーする      |
| style\* |           | オプション `style` の値をミラーする   |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

The `dart` module shows the currently installed version of Dart. By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml`, `pubspec.yml` or `pubspec.lock` file

### オプション

| オプション               | デフォルト                                             | 説明                                              |
| ------------------- | ------------------------------------------------- | ----------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`              | moduleのフォーマットです。                                |
| `symbol`            | `"🎯 "`                                            | A format string representing the symbol of Dart |
| `detect_extensions` | `['dart']`                                        | Which extensions should trigger this module.    |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Which filenames should trigger this module.     |
| `detect_folders`    | `[".dart_tool"]`                                  | Which folders should trigger this module.       |
| `style`             | `"bold blue"`                                     | モジュールのスタイルです。                                   |
| `disabled`          | `false`                                           | Disables the `dart` module.                     |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v2.8.4` | The version of `dart`  |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## ディレクトリ

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### オプション

| オプション               | デフォルト                                              | 説明                                                    |
| ------------------- | -------------------------------------------------- | ----------------------------------------------------- |
| `truncation_length` | `3`                                                | 現在のディレクトリを切り捨てる親フォルダーの数です。                            |
| `truncate_to_repo`  | `true`                                             | 現在いるgitリポジトリのルートに切り捨てるかどうかです。                         |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | moduleのフォーマットです。                                      |
| `style`             | `"bold cyan"`                                      | モジュールのスタイルです。                                         |
| `disabled`          | `false`                                            | `directory`モジュールを無効にします。                              |
| `read_only`         | `"🔒"`                                              | The symbol indicating current directory is read only. |
| `read_only_style`   | `"red"`                                            | The style for the read only symbol.                   |
| `truncation_symbol` | `""`                                               | The symbol to prefix to truncated paths. eg: "…/"     |
| `home_symbol`       | `"~"`                                              | The symbol indicating home directory.                 |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | デフォルト  | 説明                                                                                                                                                                     |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`    | fish shellのpwdパスロジックを適用するときに使用する文字数です。                                                                                                                                 |
| `use_logical_path`          | `true` | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### 変数

| 変数        | 設定例                   | 説明                         |
| --------- | --------------------- | -------------------------- |
| path      | `"D:/Projects"`       | The current directory path |
| style\* | `"black bold dimmed"` | オプション `style` の値をミラーする     |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### オプション

| オプション               | デフォルト                                                         | 説明                                                                                |
| ------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | moduleのフォーマットです。                                                                  |
| `symbol`            | `"🐳 "`                                                        | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                        | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                          | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                          | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `"blue bold"`                                                 | モジュールのスタイルです。                                                                     |
| `disabled`          | `false`                                                       | Disables the `docker_context` module.                                             |

### 変数

| 変数        | 設定例            | 説明                         |
| --------- | -------------- | -------------------------- |
| context   | `test_context` | The current docker context |
| symbol    |                | オプション `記号` の値をミラーする        |
| style\* |                | オプション `style` の値をミラーする     |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

By default this module will only be shown in your prompt when one or more of the following files are present in the current directory:

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

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### オプション

| オプション               | デフォルト                                                                                                   | 説明                                           |
| ------------------- | ------------------------------------------------------------------------------------------------------- | -------------------------------------------- |
| `format`            | `"[$symbol($version )(🎯 $tfm )]($style)"`                                                               | moduleのフォーマットです。                             |
| `symbol`            | `".NET "`                                                                                               | dotnetのバージョンを表示する前に使用される記号です。                |
| `heuristic`         | `true`                                                                                                  | より高速なバージョン検出を使用して、starshipの動作を維持します。         |
| `detect_extensions` | `["sln", "csproj", "fsproj", "xproj"]`                                                                  | Which extensions should trigger this module. |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Which filenames should trigger this module.  |
| `detect_folders`    | `[]`                                                                                                    | Which folders should trigger this modules.   |
| `style`             | `"bold blue"`                                                                                           | モジュールのスタイルです。                                |
| `disabled`          | `false`                                                                                                 | Disables the `dotnet` module.                |

### 変数

| 変数        | 設定例              | 説明                                                                 |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | オプション `記号` の値をミラーする                                                |
| style\* |                  | オプション `style` の値をミラーする                                             |

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

The `elixir` module shows the currently installed version of Elixir and Erlang/OTP. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`mix.exs`ファイルが含まれている.

### オプション

| オプション               | デフォルト                                                       | 説明                                                              |
| ------------------- | ----------------------------------------------------------- | --------------------------------------------------------------- |
| `symbol`            | `"💧 "`                                                      | The symbol used before displaying the version of Elixir/Erlang. |
| `detect_extensions` | `[]`                                                        | Which extensions should trigger this module.                    |
| `detect_files`      | `["mix.exs"]`                                               | Which filenames should trigger this module.                     |
| `detect_folders`    | `[]`                                                        | Which folders should trigger this modules.                      |
| `style`             | `"bold purple"`                                             | モジュールのスタイルです。                                                   |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                               |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                   |

### 変数

| 変数          | 設定例     | 説明                          |
| ----------- | ------- | --------------------------- |
| version     | `v1.10` | The version of `elixir`     |
| otp_version |         | The otp version of `elixir` |
| symbol      |         | オプション `記号` の値をミラーする         |
| style\*   |         | オプション `style` の値をミラーする      |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of Elm. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`elm.json`ファイルが含まれている
- カレントディレクトリに`elm-package.json`ファイルが含まれている
- カレントディレクトリに`.elm-version`ファイルが含まれている
- カレントディレクトリに`elm-stuff`フォルダが含まれている
- カレントディレクトリに`*.elm`ファイルが含まれている

### オプション

| オプション               | デフォルト                                              | 説明                                              |
| ------------------- | -------------------------------------------------- | ----------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`               | moduleのフォーマットです。                                |
| `symbol`            | `"🌳 "`                                             | A format string representing the symbol of Elm. |
| `detect_extensions` | `["elm"]`                                          | Which extensions should trigger this module.    |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Which filenames should trigger this module.     |
| `detect_folders`    | `["elm-stuff"]`                                    | Which folders should trigger this modules.      |
| `style`             | `"cyan bold"`                                      | モジュールのスタイルです。                                   |
| `disabled`          | `false`                                            | Disables the `elm` module.                      |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v0.19.1` | The version of `elm`   |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## 環境変数

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- `variable`オプションが、既存の環境変数と一致する
- `variable`オプションが定義されておらず、`default`オプションが定義されている

### オプション

| オプション      | デフォルト                          | 説明                                    |
| ---------- | ------------------------------ | ------------------------------------- |
| `symbol`   |                                | 環境変数を表示する前に使用される記号です。                 |
| `variable` |                                | 表示される環境変数です。                          |
| `default`  |                                | 上のvariableが定義されていない場合に表示されるデフォルトの値です。 |
| `format`   | `"with [$env_value]($style) "` | moduleのフォーマットです。                      |
| `disabled` | `false`                        | `env_var`モジュールを無効にします。                |

### 変数

| 変数        | 設定例                                         | 説明                                         |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | オプション `記号` の値をミラーする                        |
| style\* | `black bold dimmed`                         | オプション `style` の値をミラーする                     |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Erlang

The `erlang` module shows the currently installed version of Erlang/OTP. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`rebar.config`ファイルが含まれている.
- カレントディレクトリに`erlang.mk`ファイルが含まれている.

### オプション

| オプション               | デフォルト                                | 説明                                                       |
| ------------------- | ------------------------------------ | -------------------------------------------------------- |
| `symbol`            | `" "`                               | The symbol used before displaying the version of erlang. |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                            |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.             |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Which filenames should trigger this module.              |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.               |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                         |
| `disabled`          | `false`                              | Disables the `erlang` module.                            |

### 変数

| 変数        | 設定例       | 説明                      |
| --------- | --------- | ----------------------- |
| version   | `v22.1.3` | The version of `erlang` |
| symbol    |           | オプション `記号` の値をミラーする     |
| style\* |           | オプション `style` の値をミラーする  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Gcloud

`gcloud` モジュールは、 [`gcloud`](https://cloud.google.com/sdk/gcloud) CLIの現在の設定が表示されます。 これは `~/.config/gcloud/active_config` ファイルと `~/.config/gcloud/configurations/config_{CONFIG NAME}` ファイルと `CLOUDSDK_CONFIG` 環境変数に基づきます。

### オプション

| オプション            | デフォルト                                            | 説明                            |
| ---------------- | ------------------------------------------------ | ----------------------------- |
| `format`         | `'on [$symbol$account(\($region\))]($style) '` | moduleのフォーマットです。              |
| `symbol`         | `"☁️ "`                                          | 現在のGCPプロファイルを表示する前に表示される記号です。 |
| `region_aliases` |                                                  | GCP名に加えて表示するリージョンのエイリアスです。    |
| `style`          | `"bold blue"`                                    | モジュールのスタイルです。                 |
| `disabled`       | `false`                                          | `gcloud`モジュールを無効にします。         |

### 変数

| 変数        | 設定例               | 説明                                              |
| --------- | ----------------- | ----------------------------------------------- |
| region    | `us-central1`     | 現在のGCPリージョン                                     |
| account   | `foo@example.com` | 現在のGCPプロファイル                                    |
| project   |                   | 現在のGCPプロジェクト                                    |
| active    | `default`         | `~/.config/gcloud/active_config` に書かれたアクティブな設定名 |
| symbol    |                   | オプション `記号` の値をミラーする                             |
| style\* |                   | オプション `style` の値をミラーする                          |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

#### アカウントとプロジェクトを表示

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(\($project\))]($style) '
```

#### アクティブな設定名のみ表示

```toml
# ~/.config/starship.toml

[gcloud]
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### アカウントとエイリアスされたリージョンを表示する

```toml
# ~/.config/starship.toml

[gcloud]
symbol = "️🇬️ "
[gcloud.region_aliases]
us-central1 = "uc1"
asia-northeast1 = "an1"
```

## Git ブランチ

The `git_branch` module shows the active branch of the repo in your current directory.

### オプション

| オプション                | デフォルト                            | 説明                                                                                   |
| -------------------- | -------------------------------- | ------------------------------------------------------------------------------------ |
| `always_show_remote` | `false`                          | Shows the remote tracking branch name, even if it is equal to the local branch name. |
| `format`             | `"on [$symbol$branch]($style) "` | moduleのフォーマットです。 Use `"$branch"` to refer to the current branch name.                |
| `symbol`             | `" "`                           | A format string representing the symbol of git branch.                               |
| `style`              | `"bold purple"`                  | モジュールのスタイルです。                                                                        |
| `truncation_length`  | `2^63 - 1`                       | Truncates a git branch to `N` graphemes.                                             |
| `truncation_symbol`  | `"…"`                            | ブランチ名切り捨てられていることを示すための記号です。 You can use `""` for no symbol.                          |
| `only_attached`      | `false`                          | Only show the branch name when not in a detached `HEAD` state.                       |
| `disabled`           | `false`                          | `git_branch`モジュールを無効にします。                                                            |

### 変数

| 変数            | 設定例      | 説明                                                                                                     |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | オプション `記号` の値をミラーする                                                                                    |
| style\*     |          | オプション `style` の値をミラーする                                                                                 |

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

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### オプション

| オプション                | デフォルト                                                  | 説明                                                      |
| -------------------- | ------------------------------------------------------ | ------------------------------------------------------- |
| `commit_hash_length` | `7`                                                    | 表示されるgitコミットハッシュの長さです。                                  |
| `format`             | `"[\\($hash\\)]($style) [\\($tag\\)]($style)"` | moduleのフォーマットです。                                        |
| `style`              | `"bold green"`                                         | モジュールのスタイルです。                                           |
| `only_detached`      | `true`                                                 | Only show git commit hash when in detached `HEAD` state |
| `tag_disabled`       | `true`                                                 | Disables showing tag info in `git_commit` module.       |
| `tag_symbol`         | `"🏷 "`                                                 | Tag symbol prefixing the info shown                     |
| `disabled`           | `false`                                                | `git_commit`モジュールを無効にします。                               |

### 変数

| 変数        | 設定例       | 説明                          |
| --------- | --------- | --------------------------- |
| hash      | `b703eb3` | The current git commit hash |
| style\* |           | オプション `style` の値をミラーする      |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git の進行状態

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### オプション

| オプション          | デフォルト                                                           | 説明                                                                                      |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                 | モジュールのスタイルです。                                                                           |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | moduleのフォーマットです。                                                                        |
| `disabled`     | `false`                                                         | `git_state`モジュールを無効にします。                                                                |

### 変数

| 変数               | 設定例        | 説明                             |
| ---------------- | ---------- | ------------------------------ |
| state            | `REBASING` | The current state of the repo  |
| progress_current | `1`        | The current operation progress |
| progress_total   | `2`        | The total operation progress   |
| style\*        |            | オプション `style` の値をミラーする         |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git の状態

The `git_status` module shows symbols representing the state of the repo in your current directory.

### オプション

| オプション        | デフォルト                                           | 説明                        |
| ------------ | ----------------------------------------------- | ------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | `git_status` のデフォルトフォーマット |
| `conflicted` | `"="`                                           | このブランチにはマージの競合があります。      |
| `ahead`      | `"⇡"`                                           | `ahead`のフォーマット            |
| `behind`     | `"⇣"`                                           | `behind`のフォーマット           |
| `diverged`   | `"⇕"`                                           | `diverged`のフォーマット         |
| `untracked`  | `"?"`                                           | The format of `untracked` |
| `stashed`    | `"$"`                                           | The format of `stashed`   |
| `modified`   | `"!"`                                           | The format of `modified`  |
| `staged`     | `"+"`                                           | The format of `staged`    |
| `renamed`    | `"»"`                                           | The format of `renamed`   |
| `deleted`    | `"✘"`                                           | The format of `deleted`   |
| `style`      | `"bold red"`                                    | モジュールのスタイルです。             |
| `disabled`   | `false`                                         | `git_status`モジュールを無効にします。 |

### 変数

The following variables can be used in `format`:

| 変数             | 説明                                                                                            |
| -------------- | --------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                   |
| `ahead_behind` | Displays `diverged` `ahead` or `behind` format string based on the current status of the repo |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                   |
| `untracked`    | Displays `untracked` when there are untracked files in the working directory.                 |
| `stashed`      | Displays `stashed` when a stash exists for the local repository.                              |
| `modified`     | Displays `modified` when there are file modifications in the working directory.               |
| `staged`       | Displays `staged` when a new file has been added to the staging area.                         |
| `renamed`      | Displays `renamed` when a renamed file has been added to the staging area.                    |
| `deleted`      | Displays `deleted` when a file's deletion has been added to the staging area.                 |
| style\*      | オプション `style` の値をミラーする                                                                        |

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

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Golang

The `golang` module shows the currently installed version of Golang. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`go.mod`ファイルが含まれている
- カレントディレクトリに`go.sum`ファイルが含まれている
- カレントディレクトリに`glide.yaml`ファイルが含まれている
- カレントディレクトリに`Gopkg.yml`ファイルが含まれている
- カレントディレクトリに`Gopkg.lock`ファイルが含まれている
- カレントディレクトリに`.go-version`ファイルが含まれている
- カレントディレクトリに`Godeps`ファイルが含まれている
- カレントディレクトリに`.go`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                                                          | 説明                                             |
| ------------------- | ------------------------------------------------------------------------------ | ---------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                           | moduleのフォーマットです。                               |
| `symbol`            | `"🐹 "`                                                                         | A format string representing the symbol of Go. |
| `detect_extensions` | `["go"]`                                                                       | Which extensions should trigger this module.   |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Which filenames should trigger this module.    |
| `detect_folders`    | `["Godeps"]`                                                                   | Which folders should trigger this module.      |
| `style`             | `"bold cyan"`                                                                  | モジュールのスタイルです。                                  |
| `disabled`          | `false`                                                                        | Disables the `golang` module.                  |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v1.12.1` | The version of `go`    |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of Helm. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`helmfile.yaml`ファイルが含まれている
- The current directory contains a `Chart.yaml` file

### オプション

| オプション               | デフォルト                                | 説明                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                 |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.     |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Which filenames should trigger this module.      |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.       |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm. |
| `style`             | `"bold white"`                       | モジュールのスタイルです。                                    |
| `disabled`          | `false`                              | Disables the `helm` module.                      |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v3.1.1` | The version of `helm`  |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## ホスト名

The `hostname` module shows the system hostname.

### オプション

| オプション      | デフォルト                       | 説明                                                                          |
| ---------- | --------------------------- | --------------------------------------------------------------------------- |
| `ssh_only` | `true`                      | SSHセッションに接続されている場合にのみホスト名を表示します。                                            |
| `trim_at`  | `"."`                       | この文字が最初にマッチするまでをホスト名と認識します。 `"."`は最初の. までをホスト名として認識します。 `""`を指定した場合トリムしません。 |
| `format`   | `"[$hostname]($style) in "` | moduleのフォーマットです。                                                            |
| `style`    | `"bold dimmed green"`       | モジュールのスタイルです。                                                               |
| `disabled` | `false`                     | `hostname`モジュールを無効にします。                                                     |

### 変数

| 変数        | 設定例 | 説明                     |
| --------- | --- | ---------------------- |
| symbol    |     | オプション `記号` の値をミラーする    |
| style\* |     | オプション `style` の値をミラーする |

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

The `java` module shows the currently installed version of Java. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### オプション

| オプション               | デフォルト                                                                                                     | 説明                                              |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | moduleのフォーマットです。                                |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Which extensions should trigger this module.    |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Which filenames should trigger this module.     |
| `detect_folders`    | `[]`                                                                                                      | Which folders should trigger this modules.      |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java |
| `style`             | `"red dimmed"`                                                                                            | モジュールのスタイルです。                                   |
| `disabled`          | `false`                                                                                                   | Disables the `java` module.                     |

### 変数

| 変数        | 設定例   | 説明                     |
| --------- | ----- | ---------------------- |
| version   | `v14` | The version of `java`  |
| symbol    |       | オプション `記号` の値をミラーする    |
| style\* |       | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## ジョブ

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

::: warning

This module is not supported on tcsh.

:::

### オプション

| オプション       | デフォルト                         | 説明                                               |
| ----------- | ----------------------------- | ------------------------------------------------ |
| `threshold` | `1`                           | 超過した場合、ジョブの数を表示します。                              |
| `format`    | `"[$symbol$number]($style) "` | moduleのフォーマットです。                                 |
| `symbol`    | `"✦"`                         | A format string representing the number of jobs. |
| `style`     | `"bold blue"`                 | モジュールのスタイルです。                                    |
| `disabled`  | `false`                       | `jobs`モジュールを無効にします。                              |

### 変数

| 変数        | 設定例 | 説明                     |
| --------- | --- | ---------------------- |
| number    | `1` | The number of jobs     |
| symbol    |     | オプション `記号` の値をミラーする    |
| style\* |     | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## Julia

The `julia` module shows the currently installed version of Julia. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`Project.toml`ファイルが含まれている
- カレントディレクトリに`Manifest.toml`ファイルが含まれている
- カレントディレクトリに`.jl`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                |
| ------------------- | ------------------------------------ | ------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                  |
| `detect_extensions` | `["jl"]`                             | Which extensions should trigger this module.      |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Which filenames should trigger this module.       |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.        |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia. |
| `style`             | `"bold purple"`                      | モジュールのスタイルです。                                     |
| `disabled`          | `false`                              | Disables the `julia` module.                      |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v1.4.0` | The version of `julia` |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of Kotlin. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.kt` or a `.kts` file

### オプション

| オプション               | デフォルト                                | 説明                                                                            |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                                              |
| `detect_extensions` | `["kt", "kts"]`                      | Which extensions should trigger this module.                                  |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                                   |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                    |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `"bold blue"`                        | モジュールのスタイルです。                                                                 |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### 変数

| 変数        | 設定例       | 説明                      |
| --------- | --------- | ----------------------- |
| version   | `v1.4.21` | The version of `kotlin` |
| symbol    |           | オプション `記号` の値をミラーする     |
| style\* |           | オプション `style` の値をミラーする  |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = "kotlinc"
```

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### オプション

| オプション             | デフォルト                                                | 説明                                                                    |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | moduleのフォーマットです。                                                      |
| `style`           | `"cyan bold"`                                        | モジュールのスタイルです。                                                         |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Disables the `kubernetes` module.                                     |

### 変数

| 変数        | 設定例                  | 説明                                       |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | オプション `記号` の値をミラーする                      |
| style\* |                      | オプション `style` の値をミラーする                   |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## 改行

The `line_break` module separates the prompt into two lines.

### オプション

| オプション      | デフォルト   | 説明                                    |
| ---------- | ------- | ------------------------------------- |
| `disabled` | `false` | `line_break`モジュールを無効にして、プロンプトを1行にします。 |

### 設定例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Lua

The `lua` module shows the currently installed version of Lua. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### オプション

| オプション               | デフォルト                                | 説明                                                                         |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                                           |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `["lua"]`                            | Which extensions should trigger this module.                               |
| `detect_files`      | `[".lua-version"]`                   | Which filenames should trigger this module.                                |
| `detect_folders`    | `["lua"]`                            | Which folders should trigger this module.                                  |
| `style`             | `"bold blue"`                        | モジュールのスタイルです。                                                              |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v5.4.0` | The version of `lua`   |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## メモリ使用量

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### オプション

| オプション       | デフォルト                                         | 説明                          |
| ----------- | --------------------------------------------- | --------------------------- |
| `threshold` | `75`                                          | この閾値を超えない限り、メモリ使用率は表示されません。 |
| `format`    | `"via $symbol [${ram}( | ${swap})]($style) "` | moduleのフォーマットです。            |
| `symbol`    | `"🐏"`                                         | メモリ使用率を表示する前に使用される記号です。     |
| `style`     | `"bold dimmed white"`                         | モジュールのスタイルです。               |
| `disabled`  | `true`                                        | `memory_usage`モジュールを無効にします。 |

### 変数

| 変数               | 設定例           | 説明                                                                 |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | オプション `記号` の値をミラーする                                                |
| style\*        |               | オプション `style` の値をミラーする                                             |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

### 設定例

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Mercurial ブランチ

The `hg_branch` module shows the active branch of the repo in your current directory.

### オプション

| オプション               | デフォルト                            | 説明                                                                                           |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | モジュールのスタイルです。                                                                                |
| `format`            | `"on [$symbol$branch]($style) "` | moduleのフォーマットです。                                                                             |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `"…"`                            | ブランチ名切り捨てられていることを示すための記号です。                                                                  |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### 変数

| 変数        | 設定例      | 説明                          |
| --------- | -------- | --------------------------- |
| branch    | `master` | The active mercurial branch |
| symbol    |          | オプション `記号` の値をミラーする         |
| style\* |          | オプション `style` の値をミラーする      |

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

The `nim` module shows the currently installed version of Nim. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`nim.cfg`ファイルが含まれている
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### オプション

| オプション               | デフォルト                                | 説明                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module                             |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim. |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Which extensions should trigger this module.          |
| `detect_files`      | `["nim.cfg"]`                        | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.             |
| `style`             | `"bold yellow"`                      | モジュールのスタイルです。                                         |
| `disabled`          | `false`                              | Disables the `nim` module.                            |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v1.2.0` | The version of `nimc`  |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### オプション

| オプション        | デフォルト                                          | 説明                                                    |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | moduleのフォーマットです。                                      |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | モジュールのスタイルです。                                         |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | `nix_shell`モジュールを無効にします。                              |

### 変数

| 変数        | 設定例     | 説明                         |
| --------- | ------- | -------------------------- |
| state     | `pure`  | The state of the nix-shell |
| name      | `lorri` | The name of the nix-shell  |
| symbol    |         | オプション `記号` の値をミラーする        |
| style\* |         | オプション `style` の値をミラーする     |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`package.json`ファイルが含まれている
- The current directory contains a `.node-version` file
- カレントディレクトリに`node_modules`ディレクトリが含まれている
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### オプション

| オプション               | デフォルト                                | 説明                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                                                                       |
| `symbol`            | `" "`                               | A format string representing the symbol of NodeJS.                                                     |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Which extensions should trigger this module.                                                           |
| `detect_files`      | `["package.json", ".node-version"]`  | Which filenames should trigger this module.                                                            |
| `detect_folders`    | `["node_modules"]`                   | Which folders should trigger this module.                                                              |
| `style`             | `"bold green"`                       | モジュールのスタイルです。                                                                                          |
| `disabled`          | `false`                              | Disables the `nodejs` module.                                                                          |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in `package.json` does not match the NodeJS version. |

### 変数

| 変数        | 設定例        | 説明                     |
| --------- | ---------- | ---------------------- |
| version   | `v13.12.0` | The version of `node`  |
| symbol    |            | オプション `記号` の値をミラーする    |
| style\* |            | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of OCaml. By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### オプション

| オプション               | デフォルト                                                            | 説明                                                      |
| ------------------- | ---------------------------------------------------------------- | ------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                             | The format string for the module.                       |
| `symbol`            | `"🐫 "`                                                           | The symbol used before displaying the version of OCaml. |
| `detect_extensions` | `["opam", "ml", "mli", "re", "rei"]`                             | Which extensions should trigger this module.            |
| `detect_files`      | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]` | Which filenames should trigger this module.             |
| `detect_folders`    | `["_opam", "esy.lock"]`                                          | Which folders should trigger this module.               |
| `style`             | `"bold yellow"`                                                  | モジュールのスタイルです。                                           |
| `disabled`          | `false`                                                          | Disables the `ocaml` module.                            |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v4.10.0` | The version of `ocaml` |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### オプション

| オプション      | デフォルト                                               | 説明                                                             |
| ---------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | moduleのフォーマットです。                                               |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | モジュールのスタイルです。                                                  |
| `disabled` | `false`                                             | Disables the `openstack` module.                               |

### 変数

| 変数        | 設定例    | 説明                            |
| --------- | ------ | ----------------------------- |
| cloud     | `corp` | The current OpenStack cloud   |
| project   | `dev`  | The current OpenStack project |
| symbol    |        | オプション `記号` の値をミラーする           |
| style\* |        | オプション `style` の値をミラーする        |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## パッケージのバージョン

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- **npm** – `npm`パッケージバージョンは、現在のディレクトリにある`package.json`から抽出されます
- **cargo** – `cargo`パッケージバージョンは、現在のディレクトリにある`Cargo.toml`から抽出されます。
- **poetry** – `poetry`パッケージバージョンは、現在のディレクトリにある`pyproject.toml`から抽出されます
- **composer** – `composer`パッケージバージョンは、現在のディレクトリにある`composer.json`から抽出されます
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present
- **helm** - The `helm` chart version is extracted from the `Chart.yaml` present
- **maven** - The `maven` package version is extracted from the `pom.xml` present
- **meson** - The `meson` package version is extracted from the `meson.build` present

> ⚠️ 表示されるバージョンは、パッケージマネージャーではなく、ソースコードが現在のディレクトリにあるパッケージのバージョンです。

### オプション

| オプション             | デフォルト                             | 説明                                                        |
| ----------------- | --------------------------------- | --------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | moduleのフォーマットです。                                          |
| `symbol`          | `"📦 "`                            | パッケージのバージョンを表示する前に使用される記号です。                              |
| `style`           | `"bold 208"`                      | モジュールのスタイルです。                                             |
| `display_private` | `false`                           | Enable displaying version for packages marked as private. |
| `disabled`        | `false`                           | `package` モジュールを無効にします。                                   |

### 変数

| 変数        | 設定例      | 説明                          |
| --------- | -------- | --------------------------- |
| version   | `v1.0.0` | The version of your package |
| symbol    |          | オプション `記号` の値をミラーする         |
| style\* |          | オプション `style` の値をミラーする      |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of Perl. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### オプション

| オプション               | デフォルト                                                                                                    | 説明                                                    |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ----------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | The format string for the module.                     |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Which extensions should trigger this module.          |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                                                                                     | Which folders should trigger this module.             |
| `style`             | `"bold 149"`                                                                                             | モジュールのスタイルです。                                         |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                           |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v5.26.1` | The version of `perl`  |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

### 設定例

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of PHP. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`composer.json`ファイルが含まれている
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### オプション

| オプション               | デフォルト                                | 説明                                           |
| ------------------- | ------------------------------------ | -------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                             |
| `symbol`            | `"🐘 "`                               | PHPのバージョンを表示する前に使用される記号です。                   |
| `detect_extensions` | `["php"]`                            | Which extensions should trigger this module. |
| `detect_files`      | `["composer.json", ".php-version"]`  | Which filenames should trigger this module.  |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.    |
| `style`             | `"147 bold"`                         | モジュールのスタイルです。                                |
| `disabled`          | `false`                              | Disables the `php` module.                   |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v7.3.8` | The version of `php`   |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of PureScript version. By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`spago.dhall`ファイルが含まれている
- The current directory contains a file with the `.purs` extension

### オプション

| オプション               | デフォルト                                | 説明                                                           |
| ------------------- | ------------------------------------ | ------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                             |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript. |
| `detect_extensions` | `["purs"]`                           | Which extensions should trigger this module.                 |
| `detect_files`      | `["spago.dhall"]`                    | Which filenames should trigger this module.                  |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                    |
| `style`             | `"bold white"`                       | モジュールのスタイルです。                                                |
| `disabled`          | `false`                              | Disables the `purescript` module.                            |

### 変数

| 変数        | 設定例      | 説明                          |
| --------- | -------- | --------------------------- |
| version   | `0.13.5` | The version of `purescript` |
| symbol    |          | オプション `記号` の値をミラーする         |
| style\* |          | オプション `style` の値をミラーする      |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

By default the module will be shown if any of the following conditions are met:

- カレントディレクトリに`.python-version`ファイルが含まれている
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
- The current directory contains a file with the `.py` extension.
- 仮想環境がアクティブである

### オプション

| オプション                | デフォルト                                                                                                        | 説明                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | moduleのフォーマットです。                                                                       |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | モジュールのスタイルです。                                                                          |
| `pyenv_version_name` | `false`                                                                                                      | pyenvを使用してPythonバージョンを取得します                                                            |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3, "python2"]`                                                                            | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `[".py"]`                                                                                                    | Which extensions should trigger this module                                            |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | Which filenames should trigger this module                                             |
| `detect_folders`     | `[]`                                                                                                         | Which folders should trigger this module                                               |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                          |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### 変数

| 変数           | 設定例             | 説明                                         |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | オプション `記号` の値をミラーする                        |
| style        | `"yellow bold"` | オプション `style` の値をミラーする                     |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |

### 設定例

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = "python3"
```

```toml
# ~/.config/starship.toml

[python]
# Don't trigger for files with the py extension
detect_extensions = []
```

## Ruby

By default the `ruby` module shows the currently installed version of Ruby. The module will be shown if any of the following conditions are met:

- カレントディレクトリに`Gemfile`ファイルが含まれている
- The current directory contains a `.ruby-version` file
- カレントディレクトリに`.rb`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                 |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby. |
| `detect_extensions` | `["rb"]`                             | Which extensions should trigger this module.     |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Which filenames should trigger this module.      |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.        |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                    |
| `disabled`          | `false`                              | Disables the `ruby` module.                      |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v2.5.1` | The version of `ruby`  |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of Rust. The module will be shown if any of the following conditions are met:

- カレントディレクトリに`Cargo.toml`ファイルが含まれている
- カレントディレクトリに`.rs`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                              |
| ------------------- | ------------------------------------ | ----------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust |
| `detect_extensions` | `["rs"]`                             | Which extensions should trigger this module.    |
| `detect_files`      | `["Cargo.toml"]`                     | Which filenames should trigger this module.     |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.       |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                   |
| `disabled`          | `false`                              | Disables the `rust` module.                     |

### 変数

| 変数        | 設定例               | 説明                     |
| --------- | ----------------- | ---------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc` |
| symbol    |                   | オプション `記号` の値をミラーする    |
| style\* |                   | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of Scala. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### オプション

| オプション               | デフォルト                                    | 説明                                                |
| ------------------- | ---------------------------------------- | ------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | moduleのフォーマットです。                                  |
| `detect_extensions` | `["sbt", "scala"]`                       | Which extensions should trigger this module.      |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Which filenames should trigger this module.       |
| `detect_folders`    | `[".metals"]`                            | Which folders should trigger this modules.        |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala. |
| `style`             | `"red dimmed"`                           | モジュールのスタイルです。                                     |
| `disabled`          | `false`                                  | Disables the `scala` module.                      |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `2.13.5` | The version of `scala` |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### オプション

| オプション                  | デフォルト        | 説明                                            |
| ---------------------- | ------------ | --------------------------------------------- |
| `bash_indicator`       | `bsh`        | A format string used to represent bash.       |
| `fish_indicator`       | `fsh`        | A format string used to represent fish.       |
| `zsh_indicator`        | `zsh`        | A format string used to represent zsh.        |
| `powershell_indicator` | `psh`        | A format string used to represent powershell. |
| `ion_indicator`        | `ion`        | A format string used to represent ion.        |
| `elvish_indicator`     | `esh`        | A format string used to represent elvish.     |
| `tcsh_indicator`       | `tsh`        | A format string used to represent tcsh.       |
| `format`               | `$indicator` | moduleのフォーマットです。                              |
| `disabled`             | `true`       | Disables the `shell` module.                  |

### 変数

| 変数        | デフォルト | 説明                                                         |
| --------- | ----- | ---------------------------------------------------------- |
| indicator |       | Mirrors the value of `indicator` for currently used shell. |

### 設定例

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
disabled = false
```

## SHLVL

The `shlvl` module shows the current `SHLVL` ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### オプション

| オプション       | デフォルト                        | 説明                                                            |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | moduleのフォーマットです。                                              |
| `symbol`    | `"↕️ "`                      | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | モジュールのスタイルです。                                                 |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### 変数

| 変数        | 設定例 | 説明                           |
| --------- | --- | ---------------------------- |
| shlvl     | `3` | The current value of `SHLVL` |
| symbol    |     | オプション `記号` の値をミラーする          |
| style\* |     | オプション `style` の値をミラーする       |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### オプション

| オプション      | デフォルト                            | 説明                                               |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | moduleのフォーマットです。                                 |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | モジュールのスタイルです。                                    |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### 変数

| 変数        | 設定例          | 説明                            |
| --------- | ------------ | ----------------------------- |
| env       | `centos.img` | The current singularity image |
| symbol    |              | オプション `記号` の値をミラーする           |
| style\* |              | オプション `style` の値をミラーする        |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

::: warning This module is not supported on elvish shell. :::

### オプション

| オプション                   | デフォルト                         | 説明                                                   |
| ----------------------- | ----------------------------- | ---------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "` | The format of the module                             |
| `symbol`                | `"✖"`                         | The symbol displayed on program error                |
| `not_executable_symbol` | `"🚫"`                         | The symbol displayed when file isn't executable      |
| `not_found_symbol`      | `"🔍"`                         | The symbol displayed when the command can't be found |
| `sigint_symbol`         | `"🧱"`                         | The symbol displayed on SIGINT (Ctrl + c)            |
| `signal_symbol`         | `"⚡"`                         | The symbol displayed on any signal                   |
| `style`                 | `"bold red"`                  | モジュールのスタイルです。                                        |
| `recognize_signal_code` | `true`                        | Enable signal mapping from exit code                 |
| `map_symbol`            | `false`                       | Enable symbols mapping from exit code                |
| `disabled`              | `true`                        | Disables the `status` module.                        |

### 変数

| 変数             | 設定例     | 説明                                                                   |
| -------------- | ------- | -------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                    |
| int            | `127`   | The exit code of the last command                                    |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                  |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled      |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found         |
| symbol         |         | オプション `記号` の値をミラーする                                                  |
| style\*      |         | オプション `style` の値をミラーする                                               |

\*: This variable can only be used as a part of a style string

### 設定例

```toml

# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴"
format = '[\[$symbol $common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false

```

## Swift

By default the `swift` module shows the currently installed version of Swift. The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### オプション

| オプション               | デフォルト                                | 説明                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                 |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift |
| `detect_extensions` | `["swift"]`                          | Which extensions should trigger this module.     |
| `detect_files`      | `["Package.swift"]`                  | Which filenames should trigger this module.      |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.        |
| `style`             | `"bold 202"`                         | モジュールのスタイルです。                                    |
| `disabled`          | `false`                              | Disables the `swift` module.                     |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v5.2.4` | The version of `swift` |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version.

::: tip

By default the terraform version is not shown, since this is slow for current versions of terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-version).

:::

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` or `.hcl` extensions

### オプション

| オプション               | デフォルト                                | 説明                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                     |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace. |
| `detect_extensions` | `["tf", "hcl"]`                      | Which extensions should trigger this module.          |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.           |
| `detect_folders`    | `[".terraform"]`                     | Which folders should trigger this module.             |
| `style`             | `"bold 105"`                         | モジュールのスタイルです。                                         |
| `disabled`          | `false`                              | Disables the `terraform` module.                      |

### 変数

| 変数        | 設定例        | 説明                              |
| --------- | ---------- | ------------------------------- |
| version   | `v0.12.24` | The version of `terraform`      |
| workspace | `default`  | The current terraform workspace |
| symbol    |            | オプション `記号` の値をミラーする             |
| style\* |            | オプション `style` の値をミラーする          |

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

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### オプション

| オプション             | デフォルト                   | 説明                                                                                                                                 |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                  |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `"bold yellow"`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `"local"`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### 変数

| 変数        | 設定例        | 説明                     |
| --------- | ---------- | ---------------------- |
| time      | `13:08:10` | The current time.      |
| style\* |            | オプション `style` の値をミラーする |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. The module will be shown if any of the following conditions are met:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### オプション

| オプション         | デフォルト                   | 説明                                    |
| ------------- | ----------------------- | ------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root. |
| `style_user`  | `"bold yellow"`         | The style used for non-root users.    |
| `format`      | `"[$user]($style) in "` | moduleのフォーマットです。                      |
| `show_always` | `false`                 | Always shows the `username` module.   |
| `disabled`    | `false`                 | Disables the `username` module.       |

### 変数

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

## Vagrant

The `vagrant` module shows the currently installed version of Vagrant. By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Vagrantfile` file

### オプション

| オプション               | デフォルト                                | 説明                                                  |
| ------------------- | ------------------------------------ | --------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                    |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant. |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.        |
| `detect_files`      | `["Vagrantfile"]`                    | Which filenames should trigger this module.         |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.           |
| `style`             | `"cyan bold"`                        | モジュールのスタイルです。                                       |
| `disabled`          | `false`                              | Disables the `vagrant` module.                      |

### 変数

| 変数        | 設定例              | 説明                       |
| --------- | ---------------- | ------------------------ |
| version   | `Vagrant 2.2.10` | The version of `Vagrant` |
| symbol    |                  | オプション `記号` の値をミラーする      |
| style\* |                  | オプション `style` の値をミラーする   |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## VCSH

The `vcsh` module displays the current active VCSH repository. The module will be shown only if a repository is currently in use.

### オプション

| オプション      | デフォルト                            | 説明                                                     |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   |                                  | The symbol used before displaying the repository name. |
| `style`    | `"bold yellow"`                  | モジュールのスタイルです。                                          |
| `format`   | `"vcsh [$symbol$repo]($style) "` | moduleのフォーマットです。                                       |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### 変数

| 変数        | 設定例                                         | 説明                         |
| --------- | ------------------------------------------- | -------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name |
| symbol    |                                             | オプション `記号` の値をミラーする        |
| style\* | `black bold dimmed`                         | オプション `style` の値をミラーする     |

\*: This variable can only be used as a part of a style string

### 設定例

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of Zig. The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### オプション

| オプション               | デフォルト                                | 説明                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig. |
| `style`             | `"bold yellow"`                      | モジュールのスタイルです。                                         |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                      |
| `disabled`          | `false`                              | Disables the `zig` module.                            |
| `detect_extensions` | `["zig"]`                            | Which extensions should trigger this module.          |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.           |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.             |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v0.6.0` | The version of `zig`   |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

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

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

### オプション

| オプション         | デフォルト                           | 説明                                                                                                                         |
| ------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                                 | The command whose output should be printed. The command will be passed on stdin to the shell.                              |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`                  | モジュールのスタイルです。                                                                                                              |
| `format`      | `"[$symbol($output )]($style)"` | moduleのフォーマットです。                                                                                                           |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                             |

### 変数

| 変数        | 説明                                     |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | オプション `記号` の値をミラーする                    |
| style\* | オプション `style` の値をミラーする                 |

\*: This variable can only be used as a part of a style string

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

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
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
