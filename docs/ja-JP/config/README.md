# 設定

Starshipの設定を開始するには、`~/.config/starship.toml` ファイルを作成します。

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Starshipのすべての設定は、この[TOML](https://github.com/toml-lang/toml)ファイルで行われます。

```toml
# シェルプロンプトの間に空行を追加します
add_newline = true

# "❯" 記号を "➜" 記号に置き換えます
[character]                            # 設定しようとしているモジュールの名前は "character" です。
success_symbol = "[➜](bold green)"     # "success_symbol" セグメントが「太字の緑色」スタイルの "➜" に設定されています

# packageモジュールを無効にし、プロンプトから完全に隠します
[package]
disabled = true
```

`STARSHIP_CONFIG` 環境変数を使用することによって、デフォルトの設定ファイルの場所を変更できます。

```sh
export STARSHIP_CONFIG=~/.starship/config.toml
```

PowerShell (Windows) で同様に `$PROFILE` にこの行を追加します。

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship\config.toml"
```

### ロギング

デフォルトでは、Starship は警告やエラーログを `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log` という名前のファイルに出力します。このセッションキーはターミナルのインスタンスに相当します。 しかし、これは `STARSHIP_CACHE` という環境変数を使って変更できます：

```sh
export STARSHIP_CACHE=~/.starship/cache
```

PowerShell (Windows) で同様に `$PROFILE` にこの行を追加します。

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

### 用語

**モジュール**: OSのコンテキスト情報に基づいて情報を提供するプロンプト内のコンポーネントです。 例えば、現在のディレクトリが Node.js のプロジェクトの場合、現在コンピュータにインストールされている Node.js のバージョンが表示されます。

**変数**: モジュールが提供する情報を含むサブコンポーネントを小さくする。 例えば、"nodejs" モジュールの "version" 変数には、Node.js の現在のバージョンが含まれています。

慣例により、ほとんどのモジュールにはデフォルトの端末色の接頭辞（"nodejs" の `via` など）と接尾辞として空のスペースがあります。

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
- `[⌘ $version](bold green)` は `⌘` 記号とその後に続く変数 `version` の値を、太字の緑色で表示します。
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

- `(@$region)` は`region`が`None`または空だった場合表示されませんが、値がある場合は`@` に続いてregionの値が表示されます。
- `(some text)` は括弧の中に変数がないので、常に何も表示しません。
- `$all` が `\[$a$b\]` のショートカットである時、 `$a` と `$b` が両方とも `None` である場合に限り、`($all)` は何も表示しません。 これは `(\[$a$b\] )` と同じ動作をします。

#### エスケープ可能な文字

以下の記号は、フォーマット文字列に特別な使用法があります。 次の記号を印刷したい場合は、バックスラッシュ(`\`)でエスケープする必要があります。

- \$
- \\
- [
- ]
- (
- )

`toml` は [独自のエスケープ構文](https://github.com/toml-lang/toml#user-content-string) を持っていることに注意してください。 設定ファイル内では文字列リテラル (`''`) を使うのがおすすめです。 基本文字列 (`""`) を使う場合は, バックスラッシュ `\` をエスケープするよう気を付けてください。

例えば、新しい行に `$` 記号を表示したい場合、以下の `format` の設定が等価です。

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

| オプション             | デフォルト                          | 説明                                                |
| ----------------- | ------------------------------ | ------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | プロンプトの形式を設定します。                                   |
| `scan_timeout`    | `30`                           | ファイルをスキャンする際のタイムアウト時間 (milliseconds) です。          |
| `command_timeout` | `500`                          | Starshipによって実行されたコマンドのタイムアウト時間 (milliseconds) です。 |
| `add_newline`     | `true`                         | シェルプロンプトの間に空行を挿入します。                              |

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

# プロンプトの先頭の空行を無効にします
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
$git_metrics\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$dart\
$deno\
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
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vlang\
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

`aws` モジュールは現在のAWSプロファイルが表示されます。 これは `~/.aws/config` に記述されている `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` 環境変数に基づいています。 This module also shows an expiration timer when using temporary credentials.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var and the credentials expiration date is read from the `AWS_SESSION_EXPIRATION` env var.

[awsu](https://github.com/kreuzwerker/awsu) を使う場合、そのプロファイルは環境変数 `AWSU_PROFILE` から読まれます。

When using [AWSume](https://awsu.me) the profile is read from the `AWSUME_PROFILE` env var and the credentials expiration date is read from the `AWSUME_EXPIRATION` env var.

### オプション

| オプション               | デフォルト                                                                | 説明                                                                |
| ------------------- | -------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\])]($style)'` | moduleのフォーマットです。                                                  |
| `symbol`            | `"☁️ "`                                                              | 現在のAWSプロファイルを表示する前に表示される記号です。                                     |
| `region_aliases`    |                                                                      | AWS名に加えて表示するリージョンのエイリアスです。                                        |
| `style`             | `"bold yellow"`                                                      | モジュールのスタイルです。                                                     |
| `expiration_symbol` | `X`                                                                  | The symbol displayed when the temporary credentials have expired. |
| `disabled`          | `false`                                                              | `aws`モジュールを無効にします。                                                |

### 変数

| 変数        | 設定例              | 説明                                          |
| --------- | ---------------- | ------------------------------------------- |
| region    | `ap-northeast-1` | 現在のAWSリージョン                                 |
| profile   | `astronauts`     | 現在のAWSプロファイル                                |
| duration  | `2h27m20s`       | The temporary credentials validity duration |
| symbol    |                  | オプション `記号` の値をミラーする                         |
| style\* |                  | オプション `style` の値をミラーする                      |

\*: この変数はスタイル文字列の一部としてのみ使用できます

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

`battery`モジュールは、デバイスのバッテリー残量と現在の充電状態を示します。 モジュールは、デバイスのバッテリー残量が10％未満の場合にのみ表示されます。

### オプション

| オプション                | デフォルト                             | 説明                        |
| -------------------- | --------------------------------- | ------------------------- |
| `full_symbol`        | `" "`                            | バッテリーが満タンのときに表示される記号です。   |
| `charging_symbol`    | `" "`                            | バッテリーの充電中に表示される記号です。      |
| `discharging_symbol` | `" "`                            | バッテリーが放電しているときに表示される記号です。 |
| `unknown_symbol`     | `" "`                            | バッテリー状態が不明なときに表示される記号です。  |
| `empty_symbol`       | `" "`                            | バッテリーが空のときに表示される記号です。     |
| `format`             | `"[$symbol$percentage]($style) "` | moduleのフォーマットです。          |
| `display`            | [link](#battery-display)          | モジュールの閾値とスタイルを表示します。      |
| `disabled`           | `false`                           | `battery`モジュールを無効にします。    |

### 設定例

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "
```

### バッテリーの表示

`display`オプションを使用して、バッテリーインジケーターを表示するタイミング（threshold）、どのシンボルが使われるか(symbol) と外観（style）を定義します。 `display` が提供されない場合、 デフォルトは次のとおりです。

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

`charging_symbol`と`discharging_symbol`オプションのデフォルト値はそれぞれ`battery`の `charging_symbol`と`discharging_symbol`になります。

#### オプション

`display`オプションは、次の表の通りです。

| オプション                | デフォルト      | 説明                                                                                     |
| -------------------- | ---------- | -------------------------------------------------------------------------------------- |
| `threshold`          | `10`       | バッテリーが表示される上限です。                                                                       |
| `style`              | `bold red` | displayオプションが使用されている場合のスタイルです。                                                         |
| `charging_symbol`    | `-`        | displayオプションが使用されている場合はこののシンボルが表示されます。デフォルトはバッテリーの `charging_symbol` オプションと同じになります。    |
| `discharging_symbol` | `-`        | displayオプションが使用されている場合はこののシンボルが表示されます。デフォルトはバッテリーの `discharging_symbol` オプションと同じになります。 |

#### 設定例

```toml
[[battery.display]]  # "bold red"のスタイルとバッテリー残量が0%~10%の放電時のシンボル
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow"のスタイルとバッテリー残量が10%~30%の放電時の💦シンボル
threshold = 30
style = "bold yellow"
discharging_symbol = 💦

# 残量が30%以上の場合バッテリーインジケータは表示されません

```

## 文字

`character`モジュールは、端末でテキストが入力される場所の横に文字（通常は矢印）を表示します。

文字は、最後のコマンドが成功したかどうかを示します。 表し方は下記の2つです。

- 色の変更 (`赤`/`緑`)
- プロンプトの表示の変更 (`❯`/`✖`)

デフォルトでは、色だけが変更されます。 形も変えてみたい場合は[このサンプル](#with-custom-error-shape)も参考にしてください。

::: warning

`error_symbol`はelvishとnu shellでサポートされていません。

:::

::: warning

`vicmd_symbol`はfishとzshのみでサポートされています。

:::

### オプション

| オプション            | デフォルト               | 説明                                           |
| ---------------- | ------------------- | -------------------------------------------- |
| `format`         | `"$symbol"`         | テキスト入力の前に使用される書式文字列。                         |
| `success_symbol` | `"[❯](bold green)"` | 前のコマンドが成功した場合にテキスト入力の前に使用される書式文字列です。         |
| `error_symbol`   | `"[❯](bold red)"`   | 前のコマンドが失敗した場合にテキスト入力の前に使用される書式文字列です。         |
| `vicmd_symbol`   | `"[❮](bold green)"` | シェルが vim ノーマルモードの場合にテキスト入力の前に使用されるフォーマット文字列。 |
| `disabled`       | `false`             | `character`モジュールを無効にします。                     |

### 変数

| 変数     | 設定例 | 説明                                                          |
| ------ | --- | ----------------------------------------------------------- |
| symbol |     | `success_symbol` 、もしくは `error_symbol` 、 `vicmd_symbol` のミラー |

### 設定例

#### エラーの形状をカスタムする

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### エラーの形状をカスタムしない

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### vimの形状をカスタムする

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

`cmake`モジュールは、現在インストールされている[Cmake](https://cmake.org/)のバージョンを表示します。 デフォルトでは次のいずれかの条件が満たされると、モジュールがアクティブになります。

- カレントディレクトリに `CMakeLists.txt` ファイルが含まれている
- カレントディレクトリに `CMakeCache.txt` ファイルが含まれている

### オプション

| オプション               | デフォルト                                  | 説明                                                     |
| ------------------- | -------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`   | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                            | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"△ "`                                 | cmakeのバージョンの前に使用される記号                                  |
| `detect_extensions` | `[]`                                   | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                   | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold blue"`                          | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                | `cmake`モジュールを無効にします。                                   |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v3.17.3` | cmake のバージョン           |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

## コマンド実行時間

`cmd_duration`モジュールは、最後のコマンドの実行にかかった時間を示します。 モジュールが表示されるのは、コマンドが2秒以上かかった場合、または`min_time`値が存在する場合のみです。

::: warning BashでDEBUGトラップをhookしない

`bash`でStarshipを実行している場合、 `eval $(starship init $0)`実行した後に`DEBUG`トラップをフックしないでください。そうしないと、このモジュールが**おそらくですが**壊れます。

:::

preexecのような機能を必要とするBashユーザーは、 [rcalorasのbash_preexecフレームワーク](https://github.com/rcaloras/bash-preexec)を使用できます。 `eval $(starship init $0)` を実行する前に、`preexec_functions`、および`precmd_functions`定義するだけで、通常どおり続行します。

### オプション

| オプション                | デフォルト                         | 説明                          |
| -------------------- | ----------------------------- | --------------------------- |
| `min_time`           | `2_000`                       | 実行時間を表示する最短期間（ミリ秒単位）です。     |
| `show_milliseconds`  | `false`                       | 実行時間の秒に加えてミリ秒を表示します。        |
| `format`             | `"took [$duration]($style) "` | moduleのフォーマットです。            |
| `style`              | `"bold yellow"`               | モジュールのスタイルです。               |
| `disabled`           | `false`                       | `cmd_duration`モジュールを無効にします。 |
| `show_notifications` | `false`                       | コマンドが完了したらデスクトップ通知を表示します。   |
| `min_time_to_notify` | `45_000`                      | 通知を持続する最短期間(ミリ秒単位)          |

::: tip

デスクトップ通知を表示するには、 `rust-notify` をサポートしているstarshipをビルドする必要があります。 `show_notifications` が `true` となっている状態で `STARSHIP_LOG=debug starship module cmd_duration -d 60000` を実行することにより、starshipが通知をサポートしているかを確認することができます。

:::

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| duration  | `16m40s` | コマンドの実行時間              |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

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

| オプション               | デフォルト                                  | 説明                                                                                                               |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | 環境が`conda create -p [path]`で作成された場合、環境パスが切り捨てられるディレクトリ数。 `0`は切り捨てがないことを意味します。  [`directory`](#directory)もご覧ください。 |
| `symbol`            | `"🅒 "`                                 | 環境名の直前に使用されるシンボルです。                                                                                              |
| `style`             | `"bold green"`                         | モジュールのスタイルです。                                                                                                    |
| `format`            | `"via [$symbol$environment]($style) "` | moduleのフォーマットです。                                                                                                 |
| `ignore_base`       | `true`                                 | アクティブになった時、環境`base`を無視します。                                                                                       |
| `disabled`          | `false`                                | `conda`モジュールを無効にします。                                                                                             |

### 変数

| 変数          | 設定例          | 説明                     |
| ----------- | ------------ | ---------------------- |
| environment | `astronauts` | 現在の conda 環境           |
| symbol      |              | オプション `記号` の値をミラーする    |
| style\*   |              | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

`crystal`モジュールは、現在インストールされている[Crystal](https://crystal-lang.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`shard.yml`ファイルが含まれている
- カレントディレクトリに`.cr`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `symbol`            | `"🔮 "`                               | Crystalのバージョンを表示する前に使用される記号です。                         |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                          |
| `detect_extensions` | `["cr"]`                             | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["shard.yml"]`                      | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | `crystal`モジュールを無効にします。                                 |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v0.32.1` | `crystal` のバージョン       |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

`dart`モジュールは、現在インストールされている[Dart](https://dart.dev/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.dart`の拡張子のファイルが含まれている
- カレントディレクトリに`.dart_tool`ディレクトリが含まれている
- カレントディレクトリに`pubspec.yaml`, `pubspec.yml`,もしくは`pubspec.lock`が含まれている

### オプション

| オプション               | デフォルト                                             | 説明                                                     |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`              | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                                       | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🎯 "`                                            | Dartのシンボルを表すフォーマット文字列                                  |
| `detect_extensions` | `["dart"]`                                        | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[".dart_tool"]`                                  | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold blue"`                                     | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                           | `dart`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v2.8.4` | `dart` のバージョン          |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

`deno`モジュールは、現在インストールされている[Deno](https://deno.land/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。
- カレントディレクトリに`mod.ts`、`mod.js`、`deps.ts`か`deps.js`が含まれている

### オプション

| オプション               | デフォルト                                        | 説明                                                     |
| ------------------- | -------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`         | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                                  | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🦕 "`                                       | Dart のシンボルを表すフォーマット文字列                                 |
| `detect_extensions` | `[]`                                         | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["mod.ts", "mod.js", "deps.ts", "deps.js"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                         | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"green bold"`                               | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                      | `deno`モジュールを無効化します。                                    |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v1.8.3` | `deno`のバージョン           |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

### 設定例

```toml
# ~/.config/starship.toml

[deno]
format = "via [🦕 $version](green bold) "
```

## Directory

`directory`モジュールには、現在のディレクトリへのパスが表示され、3つの親フォルダは切り捨てられます。 ディレクトリは、現在のgitリポジトリであるとルートとなります。

fishスタイルのpwdオプションを使用すると、切り捨てられたパスを非表示にする代わりに、オプションで有効にした番号に基づいて各ディレクトリの短縮名が表示されます。

例として、`~/Dev/Nix/nixpkgs/pkgs`で、`nixpkgs`がリポジトリルートであり、オプションが`1`に設定されている場合を挙げます。 以前は`nixpkgs/pkgs`でしたが、`~/D/N/nixpkgs/pkgs`が表示されます。

### オプション

| オプション               | デフォルト                                              | 説明                                               |
| ------------------- | -------------------------------------------------- | ------------------------------------------------ |
| `truncation_length` | `3`                                                | 現在のディレクトリを切り捨てる親フォルダーの数です。                       |
| `truncate_to_repo`  | `true`                                             | 現在いるgitリポジトリのルートに切り捨てるかどうかです。                    |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | moduleのフォーマットです。                                 |
| `style`             | `"bold cyan"`                                      | モジュールのスタイルです。                                    |
| `disabled`          | `false`                                            | `directory`モジュールを無効にします。                         |
| `read_only`         | `"🔒"`                                              | このシンボルが表示されている時、現在のディレクトリは読み取り専用です。              |
| `read_only_style`   | `"red"`                                            | 読み取り専用シンボルのスタイルです。                               |
| `truncation_symbol` | `""`                                               | The symbol to prefix to truncated paths. 例: "…/" |
| `home_symbol`       | `"~"`                                              | ホームディレクトリを示すシンボルです。                              |

<details>
<summary>このモジュールは、どのようにディレクトリを表示するかについての高度なオプションをいくつか持っています。</summary>

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

| 変数        | 設定例                   | 説明                     |
| --------- | --------------------- | ---------------------- |
| path      | `"D:/Projects"`       | カレントディレクトリのパス          |
| style\* | `"black bold dimmed"` | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default` or if the `DOCKER_HOST` or `DOCKER_CONTEXT` environment variables are set (as they are meant to override the context in use).

### オプション

| オプション               | デフォルト                              | 説明                                                             |
| ------------------- | ---------------------------------- | -------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "` | moduleのフォーマットです。                                               |
| `symbol`            | `"🐳 "`                             | Dockerコンテキストを表示する前に使用される記号です。                                  |
| `only_with_files`   | `true`                             | ファイルに一致する場合にのみ表示                                               |
| `detect_extensions` | `[]`                               | どの拡張子がこのモジュールをトリガーするか(`only_with_files`がtrueになっている必要があります)。    |
| `detect_files`      | `The format for the module.`       | どんなファイル名がこのモジュールをトリガーするか(`only_with_files`がtrueになっている必要があります)。 |
| `detect_folders`    | `[]`                               | どんなフォルダがこのモジュールをトリガーするか(`only_with_files`がtrueになっている必要があります)。  |
| `style`             | `"blue bold"`                      | モジュールのスタイルです。                                                  |
| `disabled`          | `false`                            | `docker_context`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例            | 説明                     |
| --------- | -------------- | ---------------------- |
| context   | `test_context` | 現在の Docker コンテキスト      |
| symbol    |                | オプション `記号` の値をミラーする    |
| style\* |                | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

`dotnet`モジュールはカレントディレクトリに関係する[.NET Core SDK](https://dotnet.microsoft.com/)のバージョンを表示します。 もし SDKは現在のディレクトリに固定されているのであれば、その固定されたバージョンが表示されます。 それ以外の場合、モジュール SDKの最新のインストールバージョンを示します。

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

内部的に、このモジュールは自身のバージョン検知のメカニズムを利用します。 `dotnet --version` を実行するより2倍速く実行できますが、.NET project一般的でないディレクトリlayoutの場合は間違ったバージョンが示されてしまうことがあります。 速度よりも精度が重要な場合は、次の方法でメカニズムを無効にできます。 モジュールオプションで`heuristic = false `を設定します。

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### オプション

| オプション               | デフォルト                                                                                                   | 説明                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                                                                                             | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `".NET "`                                                                                               | dotnetのバージョンを表示する前に使用される記号です。                          |
| `heuristic`         | `true`                                                                                                  | より高速なバージョン検出を使用して、starshipの動作を維持します。                   |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                                                    | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold blue"`                                                                                           | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                                                 | `dotnet`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例              | 説明                                                                 |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | オプション `記号` の値をミラーする                                                |
| style\* |                  | オプション `style` の値をミラーする                                             |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`mix.exs`ファイルが含まれている.

### オプション

| オプション               | デフォルト                                                       | 説明                                                              |
| ------------------- | ----------------------------------------------------------- | --------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                               |
| `version_format`    | `"v${raw}"`                                                 | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。          |
| `symbol`            | `"💧 "`                                                      | The symbol used before displaying the version of Elixir/Erlang. |
| `detect_extensions` | `[]`                                                        | どの拡張子がこのモジュールをアクティブにするか                                         |
| `detect_files`      | `["mix.exs"]`                                               | どのファイル名がこのモジュールをアクティブにするか                                       |
| `detect_folders`    | `[]`                                                        | どのフォルダーがこのモジュールをアクティブにするか                                       |
| `style`             | `"bold purple"`                                             | モジュールのスタイルです。                                                   |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                   |

### 変数

| 変数          | 設定例     | 説明                          |
| ----------- | ------- | --------------------------- |
| version     | `v1.10` | The version of `elixir`     |
| otp_version |         | The otp version of `elixir` |
| symbol      |         | オプション `記号` の値をミラーする         |
| style\*   |         | オプション `style` の値をミラーする      |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`elm.json`ファイルが含まれている
- カレントディレクトリに`elm-package.json`ファイルが含まれている
- カレントディレクトリに`.elm-version`ファイルが含まれている
- カレントディレクトリに`elm-stuff`フォルダが含まれている
- カレントディレクトリに`*.elm`ファイルが含まれている

### オプション

| オプション               | デフォルト                                              | 説明                                                     |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`               | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                                        | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🌳 "`                                             | A format string representing the symbol of Elm.        |
| `detect_extensions` | `["elm"]`                                          | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `["elm-stuff"]`                                    | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"cyan bold"`                                      | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                            | `elm`モジュールを無効にします。                                     |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v0.19.1` | The version of `elm`   |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## 環境変数

The `env_var` module displays the current value of a selected environment variables. 次の条件のいずれかが満たされると、モジュールが表示されます。

- `variable`オプションが、既存の環境変数と一致する
- `variable`オプションが定義されておらず、`default`オプションが定義されている


::: tip Multiple environmental variables can be displayed by using a `.`. (see example) If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.

Example: following configuration will display value of USER environment variable
```toml
# ~/.config/starship.toml

[env_var.USER]
default = "unknown user"
```
:::

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

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

Displaying multiple environmental variables:
```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = "SHELL"
default = "unknown shell"
[env_var.USER]
default = "unknown user"
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`rebar.config`ファイルが含まれている.
- カレントディレクトリに`erlang.mk`ファイルが含まれている.

### オプション

| オプション               | デフォルト                                | 説明                                                       |
| ------------------- | ------------------------------------ | -------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                         |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。   |
| `symbol`            | `" "`                               | The symbol used before displaying the version of erlang. |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                            |
| `detect_extensions` | `[]`                                 | どの拡張子がこのモジュールをアクティブにするか                                  |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | どのファイル名がこのモジュールをアクティブにするか                                |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                                |
| `disabled`          | `false`                              | `erlang`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v22.1.3` | `erlang` のバージョン        |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Google Cloud (`gcloud`)

`gcloud` モジュールは、 [`gcloud`](https://cloud.google.com/sdk/gcloud) CLIの現在の設定が表示されます。 これは `~/.config/gcloud/active_config` ファイルと `~/.config/gcloud/configurations/config_{CONFIG NAME}` ファイルと `CLOUDSDK_CONFIG` 環境変数に基づきます。

### オプション

| オプション            | デフォルト                                                      | 説明                            |
| ---------------- | ---------------------------------------------------------- | ----------------------------- |
| `format`         | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | moduleのフォーマットです。              |
| `symbol`         | `"☁️ "`                                                    | 現在のGCPプロファイルを表示する前に表示される記号です。 |
| `region_aliases` |                                                            | GCP名に加えて表示するリージョンのエイリアスです。    |
| `style`          | `"bold blue"`                                              | モジュールのスタイルです。                 |
| `disabled`       | `false`                                                    | `gcloud`モジュールを無効にします。         |

### 変数

| 変数        | 設定例           | 説明                                              |
| --------- | ------------- | ----------------------------------------------- |
| region    | `us-central1` | 現在のGCPリージョン                                     |
| account   | `foo`         | 現在のGCPプロファイル                                    |
| domain    | `example.com` | The current GCP profile domain                  |
| project   |               | 現在のGCPプロジェクト                                    |
| active    | `default`     | `~/.config/gcloud/active_config` に書かれたアクティブな設定名 |
| symbol    |               | オプション `記号` の値をミラーする                             |
| style\* |               | オプション `style` の値をミラーする                          |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

#### アカウントとプロジェクトを表示

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
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

## Git Branch

`git_branch`モジュールは、現在のディレクトリにあるリポジトリのアクティブなブランチを表示します。

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

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

`git_commit` モジュールは、カレントディレクトリのリポジトリの現在のコミットハッシュとタグ (もしあれば) を表示します。

### オプション

| オプション                | デフォルト                              | 説明                                        |
| -------------------- | ---------------------------------- | ----------------------------------------- |
| `commit_hash_length` | `7`                                | 表示される git コミットハッシュの長さ。                    |
| `format`             | `"[\\($hash$tag\\)]($style) "` | moduleのフォーマットです。                          |
| `style`              | `"bold green"`                     | モジュールのスタイルです。                             |
| `only_detached`      | `true`                             | detached `HEAD` 状態のときのみ git コミットハッシュを表示する |
| `tag_disabled`       | `true`                             | `git_commit` モジュールのタグ情報の表示を無効にする。         |
| `tag_symbol`         | `" 🏷 "`                            | 表示される情報の前に追加されるタグシンボル                     |
| `disabled`           | `false`                            | `git_commit` モジュールを無効にします。                |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| hash      | `b703eb3` | 現在の git コミットハッシュ       |
| style\* |           | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

`git_state`モジュールはgitディレクトリの進行状態を表します。 (例: _REBASING_, _BISECTING_, その他) 進捗情報がある場合(例: REBASING 3/10)はその情報も表示されます。

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

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in the current git repository.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| オプション                | デフォルト                                                        | 説明                                    |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `"bold green"`                                               | The style for the added count.        |
| `deleted_style`      | `"bold red"`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | moduleのフォーマットです。                      |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.    |

### 変数

| 変数                | 設定例 | 説明                                          |
| ----------------- | --- | ------------------------------------------- |
| added             | `1` | The current number of added lines           |
| deleted           | `2` | The current number of deleted lines         |
| added_style\*   |     | Mirrors the value of option `added_style`   |
| deleted_style\* |     | Mirrors the value of option `deleted_style` |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = "bold blue"
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git Status

`git_status`モジュールは、現在のディレクトリのリポジトリの状態を表すシンボルを表示します。

### オプション

| オプション        | デフォルト                                           | 説明                         |
| ------------ | ----------------------------------------------- | -------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | `git_status` のデフォルトフォーマット  |
| `conflicted` | `"="`                                           | このブランチにはマージの競合があります。       |
| `ahead`      | `"⇡"`                                           | `ahead`のフォーマット             |
| `behind`     | `"⇣"`                                           | `behind`のフォーマット            |
| `diverged`   | `"⇕"`                                           | `diverged`のフォーマット          |
| `up_to_date` | `""`                                            | The format of `up_to_date` |
| `untracked`  | `"?"`                                           | The format of `untracked`  |
| `stashed`    | `"$"`                                           | The format of `stashed`    |
| `modified`   | `"!"`                                           | The format of `modified`   |
| `staged`     | `"+"`                                           | The format of `staged`     |
| `renamed`    | `"»"`                                           | The format of `renamed`    |
| `deleted`    | `"✘"`                                           | The format of `deleted`    |
| `style`      | `"bold red"`                                    | モジュールのスタイルです。              |
| `disabled`   | `false`                                         | `git_status`モジュールを無効にします。  |

### 変数

` format` 内では以下の変数が利用できます。

| 変数             | 説明                                                                                                            |
| -------------- | ------------------------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                                   |
| `ahead_behind` | Displays `diverged`, `ahead`, `behind` or `up_to_date` format string based on the current status of the repo. |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                                   |
| `untracked`    | Displays `untracked` when there are untracked files in the working directory.                                 |
| `stashed`      | Displays `stashed` when a stash exists for the local repository.                                              |
| `modified`     | Displays `modified` when there are file modifications in the working directory.                               |
| `staged`       | Displays `staged` when a new file has been added to the staging area.                                         |
| `renamed`      | Displays `renamed` when a renamed file has been added to the staging area.                                    |
| `deleted`      | Displays `deleted` when a file's deletion has been added to the staging area.                                 |
| style\*      | オプション `style` の値をミラーする                                                                                        |

\*: この変数はスタイル文字列の一部としてのみ使用できます

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
up_to_date = "✓"
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

The `golang` module shows the currently installed version of [Golang](https://golang.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`go.mod`ファイルが含まれている
- カレントディレクトリに`go.sum`ファイルが含まれている
- カレントディレクトリに`glide.yaml`ファイルが含まれている
- カレントディレクトリに`Gopkg.yml`ファイルが含まれている
- カレントディレクトリに`Gopkg.lock`ファイルが含まれている
- カレントディレクトリに`.go-version`ファイルが含まれている
- カレントディレクトリに`Godeps`ファイルが含まれている
- カレントディレクトリに`.go`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                                                          | 説明                                                     |
| ------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                           | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                                                                    | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🐹 "`                                                                         | A format string representing the symbol of Go.         |
| `detect_extensions` | `["go"]`                                                                       | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `["Godeps"]`                                                                   | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold cyan"`                                                                  | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                        | `golang`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v1.12.1` | The version of `go`    |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`helmfile.yaml`ファイルが含まれている
- The current directory contains a `Chart.yaml` file

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `[]`                                 | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm.       |
| `style`             | `"bold white"`                       | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `helm` module.                            |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v3.1.1` | The version of `helm`  |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

`hostname`モジュールには、システムのホスト名が表示されます。

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

\*: この変数はスタイル文字列の一部としてのみ使用できます

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

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### オプション

| オプション               | デフォルト                                                                                                     | 説明                                                     |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                                                                                               | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                                                      | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java        |
| `style`             | `"red dimmed"`                                                                                            | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                                                   | `Java`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例   | 説明                     |
| --------- | ----- | ---------------------- |
| version   | `v14` | The version of `java`  |
| symbol    |       | オプション `記号` の値をミラーする    |
| style\* |       | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## ジョブ

`jobs`モジュールには、実行中のジョブの現在の数が表示されます。 このモジュールは、実行中のバックグラウンドジョブがある場合にのみ表示されます。 The module will show the number of jobs running if there are at least 2 jobs, or more than the `number_threshold` config value, if it exists. The module will show a symbol if there is at least 1 job, or more than the `symbol_threshold` config value, if it exists. You can set both values to 0 in order to *always* show the symbol and number of jobs, even if there are 0 jobs running.

The default functionality is:

- 0 jobs -> Nothing is shown.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

::: warning

This module is not supported on tcsh and nu.

:::

::: warning

The `threshold` option is deprecated, but if you want to use it, the module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

:::

### オプション

| オプション              | デフォルト                         | 説明                                                                       |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`\*    | `1`                           | 超過した場合、ジョブの数を表示します。                                                      |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `"[$symbol$number]($style) "` | moduleのフォーマットです。                                                         |
| `symbol`           | `"✦"`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `"bold blue"`                 | モジュールのスタイルです。                                                            |
| `disabled`         | `false`                       | `jobs`モジュールを無効にします。                                                      |
 \*: This option is deprecated, please use the 

`number_threshold` and `symbol_threshold` options instead.

### 変数

| 変数        | 設定例 | 説明                     |
| --------- | --- | ---------------------- |
| number    | `1` | ジョブの数                  |
| symbol    |     | オプション `記号` の値をミラーする    |
| style\* |     | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
number_threshold = 4
symbol_threshold = 0
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Project.toml`ファイルが含まれている
- カレントディレクトリに`Manifest.toml`ファイルが含まれている
- カレントディレクトリに`.jl`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `["jl"]`                             | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `"ஃ "`                               | Juliaのシンボルを表すフォーマット文字列                                 |
| `style`             | `"bold purple"`                      | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `julia`モジュールを無効にします。                                   |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v1.4.0` | `julia`のバージョン          |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

`kotlin`モジュールは、現在インストールされている[Kotlin](https://kotlinlang.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.kt`もしくは`.kts`ファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `["kt", "kts"]`                      | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `"🅺 "`                               | Kotlinのシンボルを表すフォーマット文字列                                |
| `style`             | `"bold blue"`                        | モジュールのスタイルです。                                          |
| `kotlin_binary`     | `"kotlin"`                           | Starshipがバージョンを取得するときに実行するkotlinバイナリを設定します。            |
| `disabled`          | `false`                              | `kotlin`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v1.4.21` | `kotlin`のバージョン         |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Kotlinコンパイラバイナリを使用してバージョンを確認する
kotlin_binary = "kotlinc"
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace from the kubeconfig file. namespace は kubconfigで設定する必要があります。設定は、`kubectl config set-context starship-cluster --namespace astronaut` といったコマンド行えます。 `$KUBECONFIG` 環境変数が設定されている場合、このモジュールは環境変数を優先して使用し、`~/.kube/config` は使用しません。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| オプション             | デフォルト                                                | 説明                              |
| ----------------- | ---------------------------------------------------- | ------------------------------- |
| `symbol`          | `"☸ "`                                               | クラスター名の前に表示されるシンボルを表すフォーマット文字列。 |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | moduleのフォーマットです。                |
| `style`           | `"cyan bold"`                                        | モジュールのスタイルです。                   |
| `context_aliases` |                                                      | コンテキストの表示エイリアスを定義するテーブル。        |
| `disabled`        | `true`                                               | `kubernetes` モジュールを無効にする。       |

### 変数

| 変数        | 設定例                  | 説明                                     |
| --------- | -------------------- | -------------------------------------- |
| context   | `starship-cluster`   | 現在の Kubernetes のコンテキスト                 |
| namespace | `starship-namespace` | 設定されている場合、現在の Kubernetes の namespace 名 |
| symbol    |                      | オプション `記号` の値をミラーする                    |
| style\* |                      | オプション `style` の値をミラーする                 |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
".*/openshift-cluster/.*" = "openshift"
"gke_.*_(?P<cluster>[\\w-]+)" = "gke-$cluster"
```

#### Regex Matching

Additional to simple aliasing, `context_aliases` also supports extended matching and renaming using regular expressions.

The regular expression must match on the entire kube context, capture groups can be referenced using `$name` and `$N` in the replacement. This is more explained in the [regex crate](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace) documentation.

Long and automatically generated cluster names can be identified and shortened using regular expressions:

```toml
[kubernetes.context_aliases]
# OpenShift contexts carry the namespace and user in the kube context: `namespace/name/user`:
".*/openshift-cluster/.*" = "openshift"
# Or better, to rename every OpenShift cluster at once:
".*/(?P<cluster>[\\w-]+)/.*" = "$cluster"

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
"gke_.*_(?P<cluster>[\\w-]+)" = "gke-$cluster"
```

## Line Break

`line_break`モジュールは、プロンプトを2行に分割します。

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

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### オプション

| オプション               | デフォルト                                | 説明                                                                         |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                                           |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。                     |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `["lua"]`                            | どの拡張子がこのモジュールをアクティブにするか                                                    |
| `detect_files`      | `[".lua-version"]`                   | どのファイル名がこのモジュールをアクティブにするか                                                  |
| `detect_folders`    | `["lua"]`                            | どのフォルダーがこのモジュールをアクティブにするか                                                  |
| `style`             | `"bold blue"`                        | モジュールのスタイルです。                                                              |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v5.4.0` | `lua` のバージョン           |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## メモリ使用量

`memory_usage` モジュールは、現在のシステムメモリとスワップ使用量を示します。

デフォルトでは、システムスワップの合計がゼロ以外の場合、スワップ使用量が表示されます。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| オプション       | デフォルト                                           | 説明                          |
| ----------- | ----------------------------------------------- | --------------------------- |
| `threshold` | `75`                                            | この閾値を超えない限り、メモリ使用率は表示されません。 |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | moduleのフォーマットです。            |
| `symbol`    | `"🐏"`                                           | メモリ使用率を表示する前に使用される記号です。     |
| `style`     | `"bold dimmed white"`                           | モジュールのスタイルです。               |
| `disabled`  | `true`                                          | `memory_usage`モジュールを無効にします。 |

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

## Mercurial Branch

` hg_branch `モジュールは、現在のディレクトリにあるリポジトリのアクティブなブランチを示します。

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

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`nim.cfg`ファイルが含まれている
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module                              |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim.  |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["nim.cfg"]`                        | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold yellow"`                      | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `nim` module.                             |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v1.2.0` | The version of `nimc`  |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. このモジュールは、nixシェル環境内にあるときに表示されます。

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

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`package.json`ファイルが含まれている
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- カレントディレクトリに`node_modules`ディレクトリが含まれている
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### オプション

| オプション               | デフォルト                                | 説明                                                                                                    |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                                                                      |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。                                                |
| `symbol`            | `" "`                               | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | どの拡張子がこのモジュールをアクティブにするか                                                                               |
| `detect_files`      | `["package.json", ".node-version"]`  | どのファイル名がこのモジュールをアクティブにするか                                                                             |
| `detect_folders`    | `["node_modules"]`                   | どのフォルダーがこのモジュールをアクティブにするか                                                                             |
| `style`             | `"bold green"`                       | モジュールのスタイルです。                                                                                         |
| `disabled`          | `false`                              | `nodejs`モジュールを無効にします。                                                                                 |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in package.json does not match the Node.js version. |

### 変数

| 変数        | 設定例        | 説明                     |
| --------- | ---------- | ---------------------- |
| version   | `v13.12.0` | The version of `node`  |
| symbol    |            | オプション `記号` の値をミラーする    |
| style\* |            | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### オプション

| オプション                     | デフォルト                                                                      | 説明                                                      |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | The format string for the module.                       |
| `version_format`          | `"v${raw}"`                                                                | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。  |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml. |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch. |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.  |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | どの拡張子がこのモジュールをアクティブにするか                                 |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | どのファイル名がこのモジュールをアクティブにするか                               |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | どのフォルダーがこのモジュールをアクティブにするか                               |
| `style`                   | `"bold yellow"`                                                            | モジュールのスタイルです。                                           |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                            |

### 変数

| 変数               | 設定例          | 説明                                                                |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | オプション `記号` の値をミラーする                                               |
| style\*        |              | オプション `style` の値をミラーする                                            |

\*: この変数はスタイル文字列の一部としてのみ使用できます

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

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## パッケージのバージョン

`package`モジュールは、現在のディレクトリがパッケージのリポジトリである場合に表示され、現在のバージョンが表示されます。 The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**python**](https://www.python.org) - The `python` package version is extracted from the `setup.cfg` present in the current directory
- [**composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present
- [**julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present
- [**mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present
- [**helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present
- [**maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present
- [**meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present
- [**vlang**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present

> ⚠️ 表示されるバージョンは、パッケージマネージャーではなく、ソースコードが現在のディレクトリにあるパッケージのバージョンです。

### オプション

| オプション             | デフォルト                             | 説明                                                        |
| ----------------- | --------------------------------- | --------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | moduleのフォーマットです。                                          |
| `symbol`          | `"📦 "`                            | パッケージのバージョンを表示する前に使用される記号です。                              |
| `version_format`  | `"v${raw}"`                       | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。    |
| `style`           | `"bold 208"`                      | モジュールのスタイルです。                                             |
| `display_private` | `false`                           | Enable displaying version for packages marked as private. |
| `disabled`        | `false`                           | `package`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例      | 説明                          |
| --------- | -------- | --------------------------- |
| version   | `v1.0.0` | The version of your package |
| symbol    |          | オプション `記号` の値をミラーする         |
| style\* |          | オプション `style` の値をミラーする      |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### オプション

| オプション               | デフォルト                                                                                                    | 説明                                                     |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | The format string for the module.                      |
| `version_format`    | `"v${raw}"`                                                                                              | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl  |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                                                     | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold 149"`                                                                                             | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                            |

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

The `php` module shows the currently installed version of [PHP](https://www.php.net/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`composer.json`ファイルが含まれている
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🐘 "`                               | PHPのバージョンを表示する前に使用される記号です。                             |
| `detect_extensions` | `["php"]`                            | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["composer.json", ".php-version"]`  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"147 bold"`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `php`モジュールを無効にします。                                     |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v7.3.8` | The version of `php`   |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`spago.dhall`ファイルが含まれている
- The current directory contains a file with the `.purs` extension

### オプション

| オプション               | デフォルト                                | 説明                                                           |
| ------------------- | ------------------------------------ | ------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                             |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。       |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript. |
| `detect_extensions` | `["purs"]`                           | どの拡張子がこのモジュールをアクティブにするか                                      |
| `detect_files`      | `["spago.dhall"]`                    | どのファイル名がこのモジュールをアクティブにするか                                    |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                                    |
| `style`             | `"bold white"`                       | モジュールのスタイルです。                                                |
| `disabled`          | `false`                              | Disables the `purescript` module.                            |

### 変数

| 変数        | 設定例      | 説明                          |
| --------- | -------- | --------------------------- |
| version   | `0.13.5` | The version of `purescript` |
| symbol    |          | オプション `記号` の値をミラーする         |
| style\* |          | オプション `style` の値をミラーする      |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

`pyenvversionname` が `true` に設定されている場合 、pyenv でのバージョン名が表示されます 。 そうでなければ、`python --version` を元にバージョン番号を表示します。

デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.python-version`ファイルが含まれている
- カレントディレクトリに`Pipfile`ファイルが含まれている
- The current directory contains a `__init__.py` file
- カレントディレクトリに`pyproject.toml`ファイルが含まれている
- カレントディレクトリに`requirements.txt`ファイルが含まれている
- カレントディレクトリに`setup.py`ファイルが含まれている
- カレントディレクトリに`tox.ini`ファイルが含まれている
- カレントディレクトリに`.py`の拡張子のファイルが含まれている.
- 仮想環境がアクティブである

### オプション

| オプション                | デフォルト                                                                                                        | 説明                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | moduleのフォーマットです。                                                                       |
| `version_format`     | `"v${raw}"`                                                                                                  | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。                                 |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | モジュールのスタイルです。                                                                          |
| `pyenv_version_name` | `false`                                                                                                      | pyenvを使用してPythonバージョンを取得します                                                            |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3", "python2"]`                                                                           | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `["py"]`                                                                                                     | どの拡張子がこのモジュールをアクティブにするか                                                                |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | どのファイル名がこのモジュールをアクティブにするか                                                              |
| `detect_folders`     | `[]`                                                                                                         | どのフォルダーがこのモジュールをアクティブにするか                                                              |
| `disabled`           | `false`                                                                                                      | `python`モジュールを無効にします。                                                                  |

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

```toml
# ~/.config/starship.toml

[python]
# Display the version of python from inside a local venv.
#
# Note this will only work when the venv is inside the project and it will only
# work in the directory that contains the venv dir but maybe this is ok?
python_binary = ["./venv/bin/python", "python", "python3", "python2"]
```

## R

The `rlang` module shows the currently installed version of R. The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"📐"`                                | A format string representing the symbol of R.          |
| `style`             | `"blue bold"`                        | モジュールのスタイルです。                                          |
| `detect_extensions` | `["R", "Rd", "Rmd", "Rproj", "Rsx"]` | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[".Rprofile"]`                      | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[".Rproj.user"]`                    | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | Disables the `r` module.                               |

### 変数

| 変数      | 設定例           | 説明                     |
| ------- | ------------- | ---------------------- |
| version | `v4.0.5`      | The version of `R`     |
| symbol  |               | オプション `記号` の値をミラーする    |
| style   | `"blue bold"` | オプション `style` の値をミラーする |

### 設定例

```toml
# ~/.config/starship.toml

[rlang]
format = "with [📐 $version](blue bold) "
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a file with `.red` or `.reds` extension

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.        |
| `detect_extensions` | `["red"]`                            | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"red bold"`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `red` module.                             |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v2.5.1` | The version of `red`   |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Gemfile`ファイルが含まれている
- The current directory contains a `.ruby-version` file
- カレントディレクトリに`.rb`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.       |
| `detect_extensions` | `["rb"]`                             | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `ruby`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v2.5.1` | The version of `ruby`  |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

デフォルトでは`rust`モジュールは現在インストールされている[Rust](https://www.rust-lang.org/)のバージョンを表示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Cargo.toml`ファイルが含まれている
- カレントディレクトリに`.rs`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🦀 "`                               | Rustのシンボルを表すフォーマット文字列                                  |
| `detect_extensions` | `["rs"]`                             | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["Cargo.toml"]`                     | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `rust`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例               | 説明                     |
| --------- | ----------------- | ---------------------- |
| version   | `v1.43.0-nightly` | `rustc`のバージョン          |
| symbol    |                   | オプション `記号` の値をミラーする    |
| style\* |                   | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### オプション

| オプション               | デフォルト                                    | 説明                                                     |
| ------------------- | ---------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [${symbol}(${version} )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                              | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `["sbt", "scala"]`                       | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[".metals"]`                            | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.      |
| `style`             | `"red dimmed"`                           | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                  | Disables the `scala` module.                           |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `2.13.5` | The version of `scala` |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| オプション                  | デフォルト        | 説明                                                           |
| ---------------------- | ------------ | ------------------------------------------------------------ |
| `bash_indicator`       | `bsh`        | A format string used to represent bash.                      |
| `fish_indicator`       | `fsh`        | A format string used to represent fish.                      |
| `zsh_indicator`        | `zsh`        | A format string used to represent zsh.                       |
| `powershell_indicator` | `psh`        | A format string used to represent powershell.                |
| `ion_indicator`        | `ion`        | A format string used to represent ion.                       |
| `elvish_indicator`     | `esh`        | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `tsh`        | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `xsh`        | A format string used to represent xonsh.                     |
| `unknown_indicator`    |              | The default value to be displayed when the shell is unknown. |
| `format`               | `$indicator` | moduleのフォーマットです。                                             |
| `disabled`             | `true`       | Disables the `shell` module.                                 |

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
unknown_indicator = "mystery shell"
disabled = false
```

## SHLVL

The `shlvl` module shows the current `SHLVL` ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### オプション

| オプション       | デフォルト                        | 説明                                                            |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | moduleのフォーマットです。                                              |
| `symbol`    | `"↕️  "`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | モジュールのスタイルです。                                                 |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### 変数

| 変数        | 設定例 | 説明                           |
| --------- | --- | ---------------------------- |
| shlvl     | `3` | The current value of `SHLVL` |
| symbol    |     | オプション `記号` の値をミラーする          |
| style\* |     | オプション `style` の値をミラーする       |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

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
| env       | `centos.img` | The current Singularity image |
| symbol    |              | オプション `記号` の値をミラーする           |
| style\* |              | オプション `style` の値をミラーする        |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

::: warning This module is not supported on elvish and nu shell. :::

### オプション

| オプション                   | デフォルト                                                                                | 説明                                                      |
| ----------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "`                                                        | The format of the module                                |
| `symbol`                | `"✖"`                                                                                | The symbol displayed on program error                   |
| `success_symbol`        | `"✔️"`                                                                               | The symbol displayed on program success                 |
| `not_executable_symbol` | `"🚫"`                                                                                | The symbol displayed when file isn't executable         |
| `not_found_symbol`      | `"🔍"`                                                                                | The symbol displayed when the command can't be found    |
| `sigint_symbol`         | `"🧱"`                                                                                | The symbol displayed on SIGINT (Ctrl + c)               |
| `signal_symbol`         | `"⚡"`                                                                                | The symbol displayed on any signal                      |
| `style`                 | `"bold red"`                                                                         | モジュールのスタイルです。                                           |
| `recognize_signal_code` | `true`                                                                               | Enable signal mapping from exit code                    |
| `map_symbol`            | `false`                                                                              | Enable symbols mapping from exit code                   |
| `pipestatus`            | `false`                                                                              | Enable pipestatus reporting                             |
| `pipestatus_separator`  | `|`                                                                                  | The symbol that separate in pipe program exit codes     |
| `pipestatus_format`     | `\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)` | The format of the module when the command is a pipeline |
| `disabled`              | `true`                                                                               | Disables the `status` module.                           |

### 変数

| 変数             | 設定例     | 説明                                                                                          |
| -------------- | ------- | ------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                           |
| int            | `127`   | The exit code of the last command                                                           |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                         |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                             |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                        |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                |
| pipestatus     |         | Rendering of in pipeline programs's exit codes, this is only available in pipestatus_format |
| symbol         |         | オプション `記号` の値をミラーする                                                                         |
| style\*      |         | オプション `style` の値をミラーする                                                                      |

\*: この変数はスタイル文字列の一部としてのみ使用できます

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

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift       |
| `detect_extensions` | `["swift"]`                          | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["Package.swift"]`                  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold 202"`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `swift` module.                           |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v5.2.4` | The version of `swift` |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.

::: tip

By default the Terraform version is not shown, since this is slow for current versions of Terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-terraform-version).

:::

デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.terraform`フォルダが含まれている
- Current directory contains a file with the `.tf` or `.hcl` extensions

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                      |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.  |
| `detect_extensions` | `["tf", "hcl"]`                      | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[".terraform"]`                     | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold 105"`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `terraform`モジュールを無効にします。                               |

### 変数

| 変数        | 設定例        | 説明                              |
| --------- | ---------- | ------------------------------- |
| version   | `v0.12.24` | The version of `terraform`      |
| workspace | `default`  | The current Terraform workspace |
| symbol    |            | オプション `記号` の値をミラーする             |
| style\* |            | オプション `style` の値をミラーする          |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

#### With Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Without Terraform version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Time

`time`モジュールは、現在の**現地**時間を示します。 `format`設定は、時間の表示方法を制御するために[`chrono`](https://crates.io/crates/chrono)クレートによって使用されます。 使用可能なオプションを確認するには、[chrono strftimeのドキュメント](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)をご覧ください。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| オプション             | デフォルト                   | 説明                                                                                                    |
| ----------------- | ----------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                     |
| `use_12hr`        | `false`                 | 12時間のフォーマットを有効にします。                                                                                   |
| `time_format`     | この表の下を参照してください          | 時刻のフォーマットに使用される[クロノフォーマット文字列](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) です。     |
| `style`           | `"bold yellow"`         | モジュールのスタイルです。                                                                                         |
| `utc_time_offset` | `"local"`               | 使用するUTCオフセットを設定します。 Range from -24 &lt; x &lt; 24. フロートが30/45分のタイムゾーンオフセットに対応できるようにします。   |
| `disabled`        | `true`                  | `time`モジュールを無効にします。                                                                                   |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. それ以外の場合、デフォルトは`"%T"`です。 Manually setting `time_format` will override the `use_12hr` setting.

### 変数

| 変数        | 設定例        | 説明                     |
| --------- | ---------- | ---------------------- |
| 時刻        | `13:08:10` | The current time.      |
| style\* |            | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

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

## ユーザー名

`username`モジュールには、アクティブなユーザーのユーザー名が表示されます。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントユーザーがroot
- カレントユーザーが、ログインしているユーザーとは異なる
- ユーザーがSSHセッションとして接続されている
- `show_always`変数がtrueに設定されている

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### オプション

| オプション         | デフォルト                   | 説明                        |
| ------------- | ----------------------- | ------------------------- |
| `style_root`  | `"bold red"`            | ユーザーがrootのときに使用されるスタイルです。 |
| `style_user`  | `"bold yellow"`         | 非rootユーザーに使用されるスタイルです。    |
| `format`      | `"[$user]($style) in "` | moduleのフォーマットです。          |
| `show_always` | `false`                 | `username`モジュールを常に表示します。  |
| `disabled`    | `false`                 | `username`モジュールを無効にします。   |

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

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `Vagrantfile` file

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.    |
| `detect_extensions` | `[]`                                 | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["Vagrantfile"]`                    | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"cyan bold"`                        | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `vagrant` module.                         |

### 変数

| 変数        | 設定例              | 説明                       |
| --------- | ---------------- | ------------------------ |
| version   | `Vagrant 2.2.10` | The version of `Vagrant` |
| symbol    |                  | オプション `記号` の値をミラーする      |
| style\* |                  | オプション `style` の値をミラーする   |

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## VLang

The `vlang` module shows you your currently installed version of V. By default the module will be shown if any of the following conditions are met:
- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### オプション

| オプション               | デフォルト                                        | 説明                                                     |
| ------------------- | -------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`         | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                                  | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"V "`                                       | A format string representing the symbol of V           |
| `detect_extensions` | `["v"]`                                      | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                         | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"blue bold"`                                | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                      | Disables the `vlang` module.                           |

### 変数

| 変数        | 設定例    | 説明                     |
| --------- | ------ | ---------------------- |
| version   | `v0.2` | The version of `v`     |
| symbol    |        | オプション `記号` の値をミラーする    |
| style\* |        | オプション `style` の値をミラーする |

### 設定例

```toml
# ~/.config/starship.toml
[vlang]
format = "via [V $version](blue bold) "
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

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

\*: この変数はスタイル文字列の一部としてのみ使用できます

### 設定例

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `.zig` file

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | moduleのフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig.  |
| `style`             | `"bold yellow"`                      | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `zig` module.                             |
| `detect_extensions` | `["zig"]`                            | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v0.6.0` | The version of `zig`   |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

\*: この変数はスタイル文字列の一部としてのみ使用できます

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
- The current Operating System (std::env::consts::OS) matchs with `os` field if defined.

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

| オプション         | デフォルト                           | 説明                                                                                                                                                                            |
| ------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                                 | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                 |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code.                                                    |
| `shell`       |                                 | [この表の下を参照してください](#custom-command-shell)                                                                                                                                       |
| `説明`          | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                                                                                  |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                         |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                   |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                    |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                                                                         |
| `style`       | `"bold green"`                  | モジュールのスタイルです。                                                                                                                                                                 |
| `format`      | `"[$symbol($output )]($style)"` | moduleのフォーマットです。                                                                                                                                                              |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                                                                                |
| `os`          |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html). |

### 変数

| 変数        | 説明                                     |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | オプション `記号` の値をミラーする                    |
| style\* | オプション `style` の値をミラーする                 |

\*: この変数はスタイル文字列の一部としてのみ使用できます

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
