# 設定

::: tip

🔥Starshipの開発は現在も進んでいます。 多くの新しいオプションが今後のリリースで利用可能になります。

:::

Starshipの設定を開始するには、`~/.config/starship.toml` ファイルを作成します。

```sh
$ mkdir -p ~/.config && touch ~/.config/starship.toml
```

Starshipのすべての設定は、この[TOML](https://github.com/toml-lang/toml)ファイルで行われます。

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

**セグメント**: モジュールを構成する小さなサブコンポーネントです。 たとえば、「nodejs」モジュールの「symbol」セグメントには、バージョン番号の前に表示される文字が含まれています（デフォルト: ⬢）。

以下はNode モジュールの表現です。 次の例では、「シンボル」と「バージョン」はその中のセグメントです。 すべてのモジュールには、デフォルトの端末色であるprefixとsuffixもあります。

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### スタイルの設定

Starshipのほとんどのモジュールでは、表示スタイルを設定できます。 これは、設定を指定する文字列であるエントリ（`style`）で行われます。 スタイル文字列の例とその機能を次に示します。 完全な構文の詳細については、詳細は [高度な設定](/advanced-config/)を参照してください 。

- `"fg:green bg:blue"` は、青色の背景に緑色のテキストを設定します
- `"bg:blue fg:bright-green"` は、青色の背景に明るい緑色のテキストを設定します
- `"bold fg:27"` は、 [ANSIカラー](https://i.stack.imgur.com/KTSQa.png) 27の太字テキストを設定します
- `"underline bg:#bf5700"` は、焦げたオレンジ色の背景に下線付きのテキストを設定します
- `"bold italic fg:purple"`は、紫色の太字斜体のテキストを設定します
- `""` はすべてのスタイルを明示的に無効にします

スタイリングがどのように見えるかは、端末エミュレータによって制御されることに注意してください。 たとえば、一部の端末エミュレータはテキストを太字にする代わりに色を明るくします。また、一部のカラーテーマは通常の色と明るい色と同じ値を使用します。 また、斜体のテキストを取得するには、端末で斜体をサポートする必要があります。

## プロンプト

これは、プロンプト全体のオプションのリストです。

### オプション

| 変数             | デフォルト                   | 説明                                       |
| -------------- | ----------------------- | ---------------------------------------- |
| `add_newline`  | `true`                  | プロンプトの開始前に新しい行を追加します。                    |
| `prompt_order` | [link](#デフォルトのプロンプト表示順) | プロンプトモジュールを出力する順序を設定します。                 |
| `scan_timeout` | `30`                    | ファイルをスキャンする際のタイムアウト時間 (milliseconds) です。 |

### 設定例

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
add_newline = false
# Overwrite a default_prompt_order and  use custom prompt_order
prompt_order=["rust","line_break","package","line_break","character"]
# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10
```

### デフォルトのプロンプト表示順

デフォルトの`prompt_order`は、空の場合、または`prompt_order`が指定されていない場合に、プロンプトにモジュールが表示される順序を定義するために使用されます。 デフォルトは次のとおりです。

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
    "nim",
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

`aws` モジュールは現在のAWSプロファイルが表示されます。 これは `~/.aws/config` に記述されている `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` 環境変数に基づいています。

[aws-vault](https://github.com/99designs/aws-vault)を使用する場合、プロファイル は`AWS_VAULT`env varから読み取られます。

### オプション

| 変数                | デフォルト           | 説明                                                       |
| ----------------- | --------------- | -------------------------------------------------------- |
| `symbol`          | `"☁️ "`         | 現在のAWSプロファイルを表示する前に表示される記号です。                            |
| `displayed_items` | `all`           | 表示するアイテムを選択します。 指定可能な値は以下です。[`all`, `profile`, `region`] |
| `region_aliases`  |                 | AWS名に加えて表示するリージョンのエイリアスです。                               |
| `style`           | `"bold yellow"` | モジュールのスタイルです。                                            |
| `disabled`        | `false`         | `aws`モジュールを無効にします。                                       |

### 設定例

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

## バッテリー

`battery`モジュールは、デバイスのバッテリー残量と現在の充電状態を示します。 モジュールは、デバイスのバッテリー残量が10％未満の場合にのみ表示されます。

### オプション

| 変数                   | デフォルト             | 説明                        |
| -------------------- | ----------------- | ------------------------- |
| `full_symbol`        | `"•"`             | バッテリーが満タンのときに表示される記号です。   |
| `charging_symbol`    | `"⇡"`             | バッテリーの充電中に表示される記号です。      |
| `discharging_symbol` | `"⇣"`             | バッテリーが放電しているときに表示される記号です。 |
| `display`            | [link](#バッテリーの表示) | モジュールの閾値とスタイルを表示します。      |
| `disabled`           | `false`           | `battery`モジュールを無効にします。    |

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

文字は、最後のコマンドが成功したかどうかを示します。 これは、色の変更（赤/緑）またはその形状の変更(❯/✖) の2つの方法で行うことができます。 後者は`use_symbol_for_status`に`true`設定されている場合にのみ行われます。

### オプション

| 変数                      | デフォルト          | 説明                                           |
| ----------------------- | -------------- | -------------------------------------------- |
| `symbol`                | `"❯"`          | プロンプトでテキストを入力する前に使用される記号です。                  |
| `error_symbol`          | `"✖"`          | 前のコマンドが失敗した場合にテキスト入力の前に使用される記号です。            |
| `use_symbol_for_status` | `false`        | シンボルを変更してエラーステータスを示します。                      |
| `vicmd_symbol`          | `"❮"`          | シェルがvimの通常モードである場合、プロンプトのテキスト入力の前に使用される記号です。 |
| `style_success`         | `"bold green"` | 最後のコマンドが成功した場合に使用されるスタイルです。                  |
| `style_failure`         | `"bold red"`   | 最後のコマンドが失敗した場合に使用されるスタイルです。                  |
| `disabled`              | `false`        | `character`モジュールを無効にします。                     |

### 設定例

```toml
# ~/.config/starship.toml

[character]
symbol = "➜"
error_symbol = "✗"
use_symbol_for_status = true
```

## コマンド実行時間

`cmd_duration`モジュールは、最後のコマンドの実行にかかった時間を示します。 モジュールが表示されるのは、コマンドが2秒以上かかった場合、または`min_time`値が存在する場合のみです。

::: warning BashでDEBUGトラップをhookしない

`bash`でStarshipを実行している場合、 `eval $(starship init $0)`実行した後に`DEBUG`トラップをフックしないでください。そうしないと、このモジュールが**おそらくですが**壊れます。

:::

preexecのような機能を必要とするBashユーザーは、 [rcalorasのbash_preexecフレームワーク](https://github.com/rcaloras/bash-preexec)を使用できます。 `eval $(starship init $0)` を実行する前に、`preexec_functions`、および`precmd_functions`定義するだけで、通常どおり続行します。

### オプション

| 変数                  | デフォルト           | 説明                          |
| ------------------- | --------------- | --------------------------- |
| `min_time`          | `2_000`         | 実行時間を表示する最短期間（ミリ秒単位）です。     |
| `show_milliseconds` | `false`         | 実行時間の秒に加えてミリ秒を表示します。        |
| `prefix`            | `took`          | コマンド実行時間の直前に表示する文字列です。      |
| `style`             | `"bold yellow"` | モジュールのスタイルです。               |
| `disabled`          | `false`         | `cmd_duration`モジュールを無効にします。 |

### 設定例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
prefix = "underwent "
```

## Conda

`$CONDA_DEFAULT_ENV`が設定されている場合、`conda`モジュールは現在のcondaの環境を表示します。

::: tip

Note: これはconda自身の プロンプト修飾子 を抑制しません。`conda config --set changeps1 False` で実行することができます。

:::

### オプション

| 変数                  | デフォルト          | 説明                                                                                                               |
| ------------------- | -------------- | ---------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`            | 環境が`conda create -p [path]`で作成された場合、環境パスが切り捨てられるディレクトリ数。 `0`は切り捨てがないことを意味します。  [`directory`](#directory)もご覧ください。 |
| `symbol`            | `"C "`         | 環境名の直前に使用されるシンボルです。                                                                                              |
| `style`             | `"bold green"` | モジュールのスタイルです。                                                                                                    |
| `disabled`          | `false`        | `conda`モジュールを無効にします。                                                                                             |

### 設定例

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## Crystal

`crystal`モジュールには、現在インストールされているCrystalのバージョンが表示されます。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`shard.yml`ファイルが含まれている
- カレントディレクトリに`.cr`の拡張子のファイルが含まれている

### オプション

| 変数         | デフォルト        | 説明                             |
| ---------- | ------------ | ------------------------------ |
| `symbol`   | `"🔮 "`       | Crystalのバージョンを表示する前に使用される記号です。 |
| `style`    | `"bold red"` | モジュールのスタイルです。                  |
| `disabled` | `false`      | `crystal`モジュールを無効にします。         |

### 設定例

```toml
# ~/.config/starship.toml

[crystal]
symbol = "✨ "
style = "bold blue"
```

## ディレクトリ

`directory`モジュールには、現在のディレクトリへのパスが表示され、3つの親フォルダは切り捨てられます。 ディレクトリは、現在のgitリポジトリであるとルートとなります。

fishスタイルのpwdオプションを使用すると、切り捨てられたパスを非表示にする代わりに、オプションで有効にした番号に基づいて各ディレクトリの短縮名が表示されます。

例として、`~/Dev/Nix/nixpkgs/pkgs`で、`nixpkgs`がリポジトリルートであり、オプションが`1`に設定されている場合を挙げます。 以前は`nixpkgs/pkgs`でしたが、`~/D/N/nixpkgs/pkgs`が表示されます。

### オプション

| 変数                  | デフォルト         | 説明                            |
| ------------------- | ------------- | ----------------------------- |
| `truncation_length` | `3`           | 現在のディレクトリを切り捨てる親フォルダーの数です。    |
| `truncate_to_repo`  | `true`        | 現在いるgitリポジトリのルートに切り捨てるかどうかです。 |
| `prefix`            | `"in "`       | ディレクトリ名の直前に表示するprefixです。      |
| `style`             | `"bold cyan"` | モジュールのスタイルです。                 |
| `disabled`          | `false`       | `directory`モジュールを無効にします。      |

<details>
<summary>このモジュールは、どのようにディレクトリを表示するかについての高度なオプションをいくつか持っています。</summary>

| 変数                          | デフォルト  | 説明                                                                                       |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------- |
| `substitutions`             |        | A table of substitutions to be made to the path.                                         |
| `fish_style_pwd_dir_length` | `0`    | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true` | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker Context

`docker_context`モジュールは、 [Dockerコンテキスト](https://docs.docker.com/engine/context/working-with-contexts/)が`デフォルト`に設定されていない場合、現在アクティブな <1>Dockerコンテキストを表示します。

### オプション

| 変数                | デフォルト         | 説明                                                                                      |
| ----------------- | ------------- | --------------------------------------------------------------------------------------- |
| `symbol`          | `"🐳 "`        | The symbol used before displaying the Docker context .                                  |
| `only_with_files` | `false`       | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `style`           | `"bold blue"` | モジュールのスタイルです。                                                                           |
| `disabled`        | `true`        | `docker_context`モジュールを無効にします。                                                           |

### 設定例

```toml
# ~/.config/starship.toml

[docker_context]
symbol = "🐋 "
```

## Dotnet

`dotnet` モジュールはカレントディレクトリに関係する.NET Core SDKのバージョンを表示します。 もし SDKは現在のディレクトリに固定されているのであれば、その固定されたバージョンが表示されます。 それ以外の場合、モジュール SDKの最新のインストールバージョンを示します。

このモジュールは、カレントディレクトリに次のファイルのいずれかが存在する場合にのみプロンプトに表示されます。: `global.json`, `project.json`, `*.sln`, `*.csproj`, `*.fsproj`, `*.xproj` 正しく使用するには、.NET Coreコマンドラインツールもインストールする必要があります。

内部的に、このモジュールは自身のバージョン検知のメカニズムを利用します。 `dotnet --version` を実行するより2倍速く実行できますが、.NET project一般的でないディレクトリlayoutの場合は間違ったバージョンが示されてしまうことがあります。 速度よりも精度が重要な場合は、次の方法でメカニズムを無効にできます。 モジュールオプションで`heuristic = false `を設定します。

### オプション

| 変数          | デフォルト         | 説明                                   |
| ----------- | ------------- | ------------------------------------ |
| `symbol`    | `•NET "`      | dotnetのバージョンを表示する前に使用される記号です。        |
| `heuristic` | `true`        | より高速なバージョン検出を使用して、starshipの動作を維持します。 |
| `style`     | `"bold blue"` | モジュールのスタイルです。                        |
| `disabled`  | `false`       | `dotnet`モジュールを無効にします。                |

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

| 変数         | デフォルト           | 説明                                                              |
| ---------- | --------------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`          | The symbol used before displaying the version of Elixir/Erlang. |
| `style`    | `"bold purple"` | モジュールのスタイルです。                                                   |
| `disabled` | `false`         | Disables the `elixir` module.                                   |

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

| 変数         | デフォルト         | 説明                         |
| ---------- | ------------- | -------------------------- |
| `symbol`   | `"🌳 "`        | Elmのバージョンを表示する前に使用される記号です。 |
| `style`    | `"bold cyan"` | モジュールのスタイルです。              |
| `disabled` | `false`       | `elm`モジュールを無効にします。         |


### 設定例

```toml
# ~/.config/starship.toml

[elm]
symbol = " "
```

## 環境変数

`env_var`モジュールは、選択された環境変数の現在の値を表示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- `variable`オプションが、既存の環境変数と一致する
- `variable`オプションが定義されておらず、`default`オプションが定義されている

### オプション

| 変数         | デフォルト                 | 説明                                    |
| ---------- | --------------------- | ------------------------------------- |
| `symbol`   |                       | 環境変数を表示する前に使用される記号です。                 |
| `variable` |                       | 表示される環境変数です。                          |
| `default`  |                       | 上のvariableが定義されていない場合に表示されるデフォルトの値です。 |
| `prefix`   | `""`                  | 変数の直前に表示するprefixです。                   |
| `suffix`   | `""`                  | 変数の直後に表示するsuffixです。                   |
| `style`    | `"dimmed bold black"` | モジュールのスタイルです。                         |
| `disabled` | `false`               | `env_var`モジュールを無効にします。                |

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

| 変数         | デフォルト      | 説明                                                       |
| ---------- | ---------- | -------------------------------------------------------- |
| `symbol`   | `"🖧 "`     | The symbol used before displaying the version of Erlang. |
| `style`    | `bold red` | The style for this module.                               |
| `disabled` | `false`    | Disables the `erlang` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[erlang]
symbol = "e "
```

## Git ブランチ

`git_branch`モジュールは、現在のディレクトリにあるリポジトリのアクティブなブランチを表示します。

### オプション

| 変数                  | デフォルト           | 説明                                          |
| ------------------- | --------------- | ------------------------------------------- |
| `symbol`            | `" "`          | 現在のディレクトリのリポジトリのブランチ名の前に使用されるシンボルです。        |
| `truncation_length` | `2^63 - 1`      | gitブランチをX書記素に切り捨てます。                        |
| `truncation_symbol` | `"…"`           | ブランチ名切り捨てられていることを示すための記号です。 記号なしに「」も使用できます。 |
| `style`             | `"bold purple"` | モジュールのスタイルです。                               |
| `disabled`          | `false`         | `git_branch`モジュールを無効にします。                   |

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

| 変数                   | デフォルト          | 説明                                     |
| -------------------- | -------------- | -------------------------------------- |
| `commit_hash_length` | `7`            | 表示されるgitコミットハッシュの長さです。                 |
| `prefix`             | `"("`          | このモジュールの先頭に表示される文字列です。                 |
| `suffix`             | `")"`          | このモジュールの末尾に表示される文字列です。                 |
| `style`              | `"bold green"` | モジュールのスタイルです。                          |
| `only_detached`      | `true`         | 切り離されたHEAD状態のときのみgit commit hashを表示します |
| `disabled`           | `false`        | `git_commit`モジュールを無効にします。              |

### 設定例

```toml
# ~/.config/starship.toml

[git_commit]
truncation_length = 4
```

## Git の進行状態

`git_state`モジュールはgitディレクトリの進行状態を表します。 (例: _REBASING_, _BISECTING_, その他) 進捗情報がある場合(例: REBASING 3/10)はその情報も表示されます。

### オプション

| 変数                 | デフォルト              | 説明                                                       |
| ------------------ | ------------------ | -------------------------------------------------------- |
| `rebase`           | `"REBASING"`       | `rebase`進行中に表示されるテキストです。                                 |
| `merge`            | `"MERGING"`        | `merge`進行中に表示されるテキストです。                                  |
| `revert`           | `"REVERTING"`      | `revert`進行中に表示されるテキストです。                                 |
| `cherry_pick`      | `"CHERRY-PICKING"` | `cherry-pick`進行中に表示されるテキストです。                            |
| `bisect`           | `"BISECTING"`      | `disect`進行中に表示されるテキストです。                                 |
| `am`               | `"AM"`             | `apply-mailbox` (`git am`) の進行中に表示されるテキストです。             |
| `am_or_rebase`     | `"AM/REBASE"`      | あいまいな`apply-mailbox`または`rebase`が進行中のときに表示されるテキストです。      |
| `progress_divider` | `"/"`              | 現在の進行量と合計進行量を分ける記号またはテキストです。 (例: `" of "` 、 `"3 of 10"`) |
| `style`            | `"bold yellow"`    | モジュールのスタイルです。                                            |
| `disabled`         | `false`            | `git_state`モジュールを無効にします。                                 |

### 設定例

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Git の状態

`git_status`モジュールは、現在のディレクトリのリポジトリの状態を表すシンボルを表示します。

### オプション

| 変数                 | デフォルト                      | 説明                              |
| ------------------ | -------------------------- | ------------------------------- |
| `conflicted`       | `"="`                      | このブランチにはマージの競合があります。            |
| `conflicted_count` | [link](#git-status-counts) | 競合の数の表示およびスタイル設定します。            |
| `ahead`            | `"⇡"`                      | このブランチは、追跡されるブランチよりも先にあります。     |
| `behind`           | `"⇣"`                      | このブランチは、追跡されているブランチの背後にあります。    |
| `diverged`         | `"⇕"`                      | このブランチは、追跡されているブランチから分岐しています。   |
| `untracked`        | `"?"`                      | 作業ディレクトリに追跡されていないファイルがあります。     |
| `untracked_count`  | [link](#git-status-counts) | 追跡されていないファイルの数を表示およびスタイル設定します。  |
| `stashed`          | `"$"`                      | ローカルリポジトリ用のスタッシュが存在します。         |
| `stashed_count`    | [link](#git-status-counts) | スタッシュの数の表示およびスタイル設定します。         |
| `modified`         | `"!"`                      | 作業ディレクトリにファイルの変更があります。          |
| `modified_count`   | [link](#git-status-counts) | 変更されたファイルの数を表示およびスタイル設定します。     |
| `staged`           | `"+"`                      | 新しいファイルがステージング領域に追加されました。       |
| `staged_count`     | [link](#git-status-counts) | ステージングされたファイルの数を表示およびスタイル設定します。 |
| `renamed`          | `"»"`                      | 名前が変更されたファイルがステージング領域に追加されました。  |
| `renamed_count`    | [link](#git-status-counts) | 名前を変更したファイルの数を表示およびスタイル設定します。   |
| `deleted`          | `"✘"`                      | ファイルの削除がステージング領域に追加されました。       |
| `deleted_count`    | [link](#git-status-counts) | 削除されたファイルの数を表示およびスタイルします。       |
| `show_sync_count`  | `false`                    | 追跡されているブランチの先行/後方カウントを表示します。    |
| `prefix`           | `[`                        | このモジュールの先頭に表示される文字列です。          |
| `suffix`           | `]`                        | このモジュールの末尾に表示される文字列です。          |
| `style`            | `"bold red"`               | モジュールのスタイルです。                   |
| `disabled`         | `false`                    | `git_status`モジュールを無効にします。       |

#### Git Statusのカウント

| 変数        | デフォルト   | 説明                                |
| --------- | ------- | --------------------------------- |
| `enabled` | `false` | ファイルの数を表示します。                     |
| `style`   |         | オプションで、モジュールとは異なるカウントのスタイルを設定します。 |

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
staged.value = "++"
staged.style = "green"
staged_count.enabled = true
staged_count.style = "green"
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

| 変数         | デフォルト         | 説明                            |
| ---------- | ------------- | ----------------------------- |
| `symbol`   | `"🐹 "`        | Golangのバージョンを表示する前に使用される記号です。 |
| `style`    | `"bold cyan"` | モジュールのスタイルです。                 |
| `disabled` | `false`       | `golang`モジュールを無効にします。         |

### 設定例

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```
## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version. 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`stack.yaml`ファイルが含まれている

### オプション

| 変数         | デフォルト        | 説明                                                        |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"λ "`       | The symbol used before displaying the version of Haskell. |
| `style`    | `"bold red"` | モジュールのスタイルです。                                             |
| `disabled` | `false`      | Disables the `haskell` module.                            |


### 設定例

```toml
# ~/.config/starship.toml

[haskell]
symbol = " "
```

## ホスト名

`hostname`モジュールには、システムのホスト名が表示されます。

### オプション

| 変数         | デフォルト                 | 説明                                                                          |
| ---------- | --------------------- | --------------------------------------------------------------------------- |
| `ssh_only` | `true`                | SSHセッションに接続されている場合にのみホスト名を表示します。                                            |
| `prefix`   | `""`                  | ホスト名の直前に表示するprefixです。                                                       |
| `suffix`   | `""`                  | ホスト名の直後に表示するsuffixです。                                                       |
| `trim_at`  | `"."`                 | この文字が最初にマッチするまでをホスト名と認識します。 `"."`は最初の. までをホスト名として認識します。 `""`を指定した場合トリムしません。 |
| `style`    | `"bold dimmed green"` | モジュールのスタイルです。                                                               |
| `disabled` | `false`               | `hostname`モジュールを無効にします。                                                     |

### 設定例

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

`java`モジュールは、現在インストールされているJavaのバージョンを示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに `pom.xml`, `build.gradle.kts`, `build.sbt` ,もしくは`.java-version`が含まれている
- カレントディレクトリに拡張子が`.java`, `.class`, `.gradle`, もしくは`.jar`のファイルが含まれている

### オプション

| 変数         | デフォルト          | 説明                          |
| ---------- | -------------- | --------------------------- |
| `symbol`   | `"☕ "`         | Javaのバージョンを表示する前に使用される記号です。 |
| `style`    | `"dimmed red"` | モジュールのスタイルです。               |
| `disabled` | `false`        | `java`モジュールを無効にします。         |

### 設定例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## ジョブ

`jobs`モジュールには、実行中のジョブの現在の数が表示されます。 このモジュールは、実行中のバックグラウンドジョブがある場合にのみ表示されます。 1つ以上のジョブがある、または`threshold`に指定した値以上にジョブがある場合は実行中のジョブの数を表示します。

### オプション

| 変数          | デフォルト         | 説明                     |
| ----------- | ------------- | ---------------------- |
| `symbol`    | `"✦"`         | ジョブの数を表示する前に使用される記号です。 |
| `threshold` | `1`           | 超過した場合、ジョブの数を表示します。    |
| `style`     | `"bold blue"` | モジュールのスタイルです。          |
| `disabled`  | `false`       | `jobs`モジュールを無効にします。    |

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

| 変数         | デフォルト           | 説明                           |
| ---------- | --------------- | ---------------------------- |
| `symbol`   | `"ஃ "`          | Juliaのバージョンを表示する前に使用される記号です。 |
| `style`    | `"bold purple"` | モジュールのスタイルです。                |
| `disabled` | `false`         | `julia`モジュールを無効にします。         |

### 設定例

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```
## Kubernetes

現在のKubernetesコンテキスト名と、設定されている場合は、kubeconfigファイルに基づいてネームスペースを表示します。 ネームスペースはkubconfigで設定されている必要があります。それは `kubectl config set-context starship-cluster --namespace astronaut` のようなコマンドで設定することができます。 `$KUBECONFIG` 環境変数が設定されている場合、モジュールはそれを使用します `~/.kube/config` は使用しません。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| 変数                | デフォルト         | 説明                                  |
| ----------------- | ------------- | ----------------------------------- |
| `symbol`          | `"☸ "`        | クラスタ情報を表示する前に使用される記号です。             |
| `context_aliases` |               | Table of context aliases to display |
| `style`           | `"bold blue"` | モジュールのスタイルです。                       |
| `disabled`        | `true`        | `Kubernetes`モジュールを無効にします。           |

### 設定例

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dimmed green"
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
```

## 改行

The `line_break` module separates the prompt into two lines.

### オプション

| 変数         | デフォルト   | 説明                                    |
| ---------- | ------- | ------------------------------------- |
| `disabled` | `false` | `line_break`モジュールを無効にして、プロンプトを1行にします。 |

### 設定例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## メモリ使用量

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| 変数                | デフォルト                 | 説明                            |
| ----------------- | --------------------- | ----------------------------- |
| `show_percentage` | `false`               | メモリ使用量を割合で表示します。              |
| `show_swap`       | `true`                | 合計スワップがゼロ以外の場合、スワップ使用量を表示します。 |
| `threshold`       | `75`                  | この閾値を超えない限り、メモリ使用率は表示されません。   |
| `symbol`          | `"🐏 "`                | メモリ使用率を表示する前に使用される記号です。       |
| `separator`       | `" | "`               | RAMとスワップの使用を分離する記号またはテキストです。  |
| `style`           | `"bold dimmed white"` | モジュールのスタイルです。                 |
| `disabled`        | `true`                | `memory_usage`モジュールを無効にします。   |

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

The `hg_branch` module shows the active branch of the repo in your current directory.

### オプション

| 変数                  | デフォルト           | 説明                                                                                           |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | ブランチ名切り捨てられていることを示すための記号です。                                                                  |
| `style`             | `"bold purple"` | モジュールのスタイルです。                                                                                |
| `disabled`          | `true`          | Disables the `hg_branch` module.                                                             |

### 設定例

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of Nim. 次の条件のいずれかが満たされると、モジュールが表示されます。
- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### オプション

| 変数         | デフォルト           | 説明                                                    |
| ---------- | --------------- | ----------------------------------------------------- |
| `symbol`   | `"👑 "`          | The symbol used before displaying the version of Nim. |
| `style`    | `"bold yellow"` | モジュールのスタイルです。                                         |
| `disabled` | `false`         | Disables the `nim` module.                            |

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

| 変数           | デフォルト         | 説明                                                |
| ------------ | ------------- | ------------------------------------------------- |
| `use_name`   | `false`       | Display the name of the nix-shell.                |
| `impure_msg` | `"impure"`    | Customize the "impure" msg.                       |
| `pure_msg`   | `"pure"`      | Customize the "pure" msg.                         |
| `symbol`     | `"❄️  "`      | The symbol used before displaying the shell name. |
| `style`      | `"bold blue"` | モジュールのスタイルです。                                     |
| `disabled`   | `false`       | Disables the `nix_shell` module.                  |

### 設定例

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

The `nodejs` module shows the currently installed version of NodeJS. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js` extension

### オプション

| 変数         | デフォルト          | 説明                                                       |
| ---------- | -------------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`         | The symbol used before displaying the version of NodeJS. |
| `style`    | `"bold green"` | モジュールのスタイルです。                                            |
| `disabled` | `false`        | Disables the `nodejs` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia` and `mix` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present

> ⚠️ 表示されるバージョンは、パッケージマネージャーではなく、ソースコードが現在のディレクトリにあるパッケージのバージョンです。

### オプション

| 変数                | デフォルト        | 説明                                                         |
| ----------------- | ------------ | ---------------------------------------------------------- |
| `symbol`          | `"📦 "`       | The symbol used before displaying the version the package. |
| `style`           | `"bold 208"` | モジュールのスタイルです。                                              |
| `display_private` | `false`      | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`      | Disables the `package` module.                             |

### 設定例

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
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

| 変数         | デフォルト           | 説明                                                      |
| ---------- | --------------- | ------------------------------------------------------- |
| `symbol`   | `"🐫 "`          | The symbol used before displaying the version of OCaml. |
| `style`    | `"bold yellow"` | モジュールのスタイルです。                                           |
| `disabled` | `false`         | Disables the `ocaml` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[ocaml]
symbol = "🐪 "
```

## PHP

The `php` module shows the currently installed version of PHP. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` file

### オプション

| 変数         | デフォルト        | 説明                                                    |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold 147"` | モジュールのスタイルです。                                         |
| `disabled` | `false`      | Disables the `php` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

The `python` module shows the currently installed version of Python and the current Python virtual environment if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension (and `scan_for_pyfiles` is true)
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- The current directory contains a `setup.py` file
- The current directory contains a `__init__.py` file
- A virtual environment is currently activated

### オプション

| 変数                   | デフォルト           | 説明                                                                          |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
| `scan_for_pyfiles`   | `true`          | If false, Python files in the current directory will not show this module.  |
| `style`              | `"bold yellow"` | モジュールのスタイルです。                                                               |
| `disabled`           | `false`         | Disables the `python` module.                                               |

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

The `ruby` module shows the currently installed version of Ruby. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### オプション

| 変数         | デフォルト        | 説明                                                     |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"💎 "`       | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold red"` | モジュールのスタイルです。                                          |
| `disabled` | `false`      | Disables the `ruby` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### オプション

| 変数         | デフォルト        | 説明                                                     |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`       | The symbol used before displaying the version of Rust. |
| `style`    | `"bold red"` | モジュールのスタイルです。                                          |
| `disabled` | `false`      | Disables the `rust` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

### オプション

| 変数         | デフォルト                | 説明                                               |
| ---------- | -------------------- | ------------------------------------------------ |
| `label`    | `""`                 | Prefix before the image name display.            |
| `prefix`   | `"["`                | Prefix to display immediately before image name. |
| `suffix`   | `"]"`                | Suffix to display immediately after image name.  |
| `symbol`   | `""`                 | The symbol used before the image name.           |
| `style`    | `"bold dimmed blue"` | モジュールのスタイルです。                                    |
| `disabled` | `false`              | Disables the `singularity` module.               |

### 設定例

```toml
# ~/.config/starship.toml

[singularity]
symbol = "📦 "
```

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### オプション

| 変数             | デフォルト        | 説明                                                          |
| -------------- | ------------ | ----------------------------------------------------------- |
| `symbol`       | `"💠 "`       | The symbol used before displaying the terraform workspace.  |
| `show_version` | `false`      | Shows the terraform version. Very slow on large workspaces. |
| `style`        | `"bold 105"` | モジュールのスタイルです。                                               |
| `disabled`     | `false`      | Disables the `terraform` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## Time

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| 変数                | デフォルト           | 説明                                                                                                                  |
| ----------------- | --------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`         | Enables 12 hour formatting.                                                                                         |
| `format`          | see below       | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `"bold yellow"` | The style for the module time.                                                                                      |
| `utc_time_offset` | `"local"`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |
| `disabled`        | `true`          | Disables the `time` module.                                                                                         |
| `time_range`      | `"-"`           | Sets the time range during which the module will be shown. Times must be specified in 24-hours format               |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `use_12hr` setting.

### 設定例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

### オプション

| 変数            | デフォルト           | 説明                                    |
| ------------- | --------------- | ------------------------------------- |
| `style_root`  | `"bold red"`    | The style used when the user is root. |
| `style_user`  | `"bold yellow"` | The style used for non-root users.    |
| `show_always` | `false`         | Always shows the `username` module.   |
| `disabled`    | `false`         | Disables the `username` module.       |

### 設定例

```toml
# ~/.config/starship.toml

[username]
disabled = true
```


## Zig

The `zig` module shows the currently installed version of Zig. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `.zig` file

### オプション

| 変数         | デフォルト           | 説明                                                    |
| ---------- | --------------- | ----------------------------------------------------- |
| `symbol`   | `"↯ "`          | The symbol used before displaying the version of Zig. |
| `style`    | `"bold yellow"` | モジュールのスタイルです。                                         |
| `disabled` | `false`         | Disables the `zig` module.                            |

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

| 変数            | デフォルト                     | 説明                                                                                                                         |
| ------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                           | The command whose output should be printed.                                                                                |
| `when`        |                           | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code. |
| `shell`       |                           | [See below](#custom-command-shell)                                                                                         |
| `description` | `"<custom module>"` | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                      | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                      | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                      | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                      | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`            | モジュールのスタイルです。                                                                                                              |
| `prefix`      | `""`                      | Prefix to display immediately before the command output.                                                                   |
| `suffix`      | `""`                      | Suffix to display immediately after the command output.                                                                    |
| `disabled`    | `false`                   | Disables this `custom` module.                                                                                             |

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

- The current directory contains a `spago.dhall` file
- The current directory contains a \*.purs files

### オプション

| 変数         | デフォルト          | 説明                                                           |
| ---------- | -------------- | ------------------------------------------------------------ |
| `symbol`   | `"<=> "` | The symbol used before displaying the version of PureScript. |
| `style`    | `"bold white"` | モジュールのスタイルです。                                                |
| `disabled` | `false`        | Disables the `purescript` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[purescript]
symbol = "<=> "
```
