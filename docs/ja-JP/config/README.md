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
    "golang",
    "haskell",
    "java",
    "julia",
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

`aws` モジュールは現在のAWSプロファイルが表示されます。 これは `~/.aws/config` に記述されている `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` 環境変数に基づいています。

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var.

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

| 変数                  | デフォルト          | 説明                                                                                                                                                                                                          |
| ------------------- | -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`            | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `"C "`         | 環境名の直前に使用されるシンボルです。                                                                                                                                                                                         |
| `style`             | `"bold green"` | モジュールのスタイルです。                                                                                                                                                                                               |
| `disabled`          | `false`        | `conda`モジュールを無効にします。                                                                                                                                                                                        |

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

| 変数                          | デフォルト  | 説明                                           |
| --------------------------- | ------ | -------------------------------------------- |
| `fish_style_pwd_dir_length` | `0`    | fish shellのpwdパスロジックを適用するときに使用する文字数です。       |
| `use_logical_path`          | `true` | OSからのパスの代わりに、シェル(`PWD`) によって提供される論理パスを表示します。 |

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default`.

### オプション

| 変数                | デフォルト         | 説明                                                                                      |
| ----------------- | ------------- | --------------------------------------------------------------------------------------- |
| `symbol`          | `"🐳 "`        | The symbol used before displaying the Docker context .                                  |
| `only_with_files` | `false`       | Only show when there's a `docker-compose.yml` or `Dockerfile` in the current directory. |
| `style`           | `"bold blue"` | モジュールのスタイルです。                                                                           |
| `disabled`        | `true`        | Disables the `docker_context` module.                                                   |

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

| 変数         | デフォルト   | 説明                                                              |
| ---------- | ------- | --------------------------------------------------------------- |
| `symbol`   | `"💧 "`  | The symbol used before displaying the version of Elixir/Erlang. |
| `disabled` | `false` | Disables the `elixir` module.                                   |

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
- The current directory contains a `.elm-version` file
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

| 変数         | デフォルト            | 説明                                    |
| ---------- | ---------------- | ------------------------------------- |
| `symbol`   |                  | 環境変数を表示する前に使用される記号です。                 |
| `variable` |                  | 表示される環境変数です。                          |
| `default`  |                  | 上のvariableが定義されていない場合に表示されるデフォルトの値です。 |
| `prefix`   | `""`             | 変数の直前に表示するprefixです。                   |
| `suffix`   | `""`             | 変数の直後に表示するsuffixです。                   |
| `style`    | `"dimmed black"` | モジュールのスタイルです。                         |
| `disabled` | `false`          | `env_var`モジュールを無効にします。                |

### 設定例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
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
- The current directory contains a `.go-version` file
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

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
- カレントディレクトリに拡張子が`.java`, `.class`, `.gradle`, もしくは`.jar`のファイルが含まれている

### オプション

| 変数         | デフォルト          | 説明                          |
| ---------- | -------------- | --------------------------- |
| `symbol`   | `"☕ "`         | Javaのバージョンを表示する前に使用される記号です。 |
| `style`    | `"dimmed red"` | モジュールのスタイルです。               |
| `disabled` | `false`        | `Java`モジュールを無効にします。         |

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

The `julia` module shows the currently installed version of Julia. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### オプション

| 変数         | デフォルト           | 説明                                                      |
| ---------- | --------------- | ------------------------------------------------------- |
| `symbol`   | `"ஃ "`          | The symbol used before displaying the version of Julia. |
| `style`    | `"bold purple"` | モジュールのスタイルです。                                           |
| `disabled` | `false`         | Disables the `julia` module.                            |

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

`line_break`モジュールは、プロンプトを2行に分割します。

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

`memory_usage`モジュールは、現在のシステムメモリとスワップ使用量を示します。

デフォルトでは、システムスワップの合計がゼロ以外の場合、スワップ使用量が表示されます。

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

` hg_branch `モジュールは、現在のディレクトリにあるリポジトリのアクティブなブランチを示します。

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

## Nix-shell

`nix_shell`モジュールは、nix-shell環境を示しています。 このモジュールは、nixシェル環境内にあるときに表示されます。

### オプション

| 変数           | デフォルト        | 説明                       |
| ------------ | ------------ | ------------------------ |
| `use_name`   | `false`      | nix-shellの名前を表示します。      |
| `impure_msg` | `"impure"`   | impureメッセージをカスタマイズします。   |
| `pure_msg`   | `"pure"`     | pureメッセージをカスタマイズします。     |
| `style`      | `"bold red"` | モジュールのスタイルです。            |
| `disabled`   | `false`      | `nix_shell`モジュールを無効にします。 |

### 設定例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
```

## NodeJS

`nodejs`モジュールは、現在インストールされているNodeJSのバージョンを示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`package.json`ファイルが含まれている
- The current directory contains a `.node-version` file
- カレントディレクトリに`node_modules`ディレクトリが含まれている
- カレントディレクトリに`.js`の拡張子のファイルが含まれている

### オプション

| 変数         | デフォルト          | 説明                            |
| ---------- | -------------- | ----------------------------- |
| `symbol`   | `"⬢ "`         | NodeJSのバージョンを表示する前に使用される記号です。 |
| `style`    | `"bold green"` | モジュールのスタイルです。                 |
| `disabled` | `false`        | `nodejs`モジュールを無効にします。         |

### 設定例

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## パッケージのバージョン

`package`モジュールは、現在のディレクトリがパッケージのリポジトリである場合に表示され、現在のバージョンが表示されます。 The module currently supports `npm`, `cargo`, `poetry`, `composer`, and `gradle` packages.

- **npm** – `npm`パッケージバージョンは、現在のディレクトリにある`package.json`から抽出されます
- **cargo** – `cargo`パッケージバージョンは、現在のディレクトリにある`Cargo.toml`から抽出されます。
- **poetry** – `poetry`パッケージバージョンは、現在のディレクトリにある`pyproject.toml`から抽出されます
- **composer** – `composer`パッケージバージョンは、現在のディレクトリにある`composer.json`から抽出されます
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present

> ⚠️ 表示されるバージョンは、パッケージマネージャーではなく、ソースコードが現在のディレクトリにあるパッケージのバージョンです。

### オプション

| 変数         | デフォルト        | 説明                           |
| ---------- | ------------ | ---------------------------- |
| `symbol`   | `"📦 "`       | パッケージのバージョンを表示する前に使用される記号です。 |
| `style`    | `"bold red"` | モジュールのスタイルです。                |
| `disabled` | `false`      | `package`モジュールを無効にします。       |

### 設定例

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## PHP

`php`モジュールは、現在インストールされているPHPのバージョンを示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`composer.json`ファイルが含まれている
- The current directory contains a `.php-version` file
- カレントディレクトリに`.php`の拡張子のファイルが含まれている

### オプション

| 変数         | デフォルト        | 説明                         |
| ---------- | ------------ | -------------------------- |
| `symbol`   | `"🐘 "`       | PHPのバージョンを表示する前に使用される記号です。 |
| `style`    | `"bold red"` | モジュールのスタイルです。              |
| `disabled` | `false`      | `php`モジュールを無効にします。         |

### 設定例

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

`python`モジュールは、現在インストールされているPythonのバージョンを示します。

`pyenvversionname`が`true`に設定されている場合 、pyenvでのバージョン名が表示されます 。

それ以外の場合は、 `python --version`バージョン番号が表示され、アクティブになっている場合は現在のPython仮想環境が表示されます。

次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.python-version`ファイルが含まれている
- カレントディレクトリに`requirements.txt`ファイルが含まれている
- カレントディレクトリに`pyproject.toml`ファイルが含まれている
- カレントディレクトリに`.py`の拡張子のファイルが含まれている
- カレントディレクトリに`Pipfile`ファイルが含まれている
- カレントディレクトリに`tox.ini`ファイルが含まれている
- 仮想環境がアクティブである

### オプション

| 変数                   | デフォルト           | 説明                                                   |
| -------------------- | --------------- | ---------------------------------------------------- |
| `symbol`             | `"🐍 "`          | Pythonのバージョンを表示する前に使用される記号です。                        |
| `pyenv_version_name` | `false`         | pyenvを使用してPythonバージョンを取得します                          |
| `pyenv_prefix`       | `"pyenv "`      | pyenvバージョン表示の前のprefix（デフォルトの表示は`pyenv MY_VERSION`）です |
| `style`              | `"bold yellow"` | モジュールのスタイルです。                                        |
| `disabled`           | `false`         | `python`モジュールを無効にします。                                |

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

| 変数         | デフォルト        | 説明                          |
| ---------- | ------------ | --------------------------- |
| `symbol`   | `"💎 "`       | Rubyのバージョンを表示する前に使用される記号です。 |
| `style`    | `"bold red"` | モジュールのスタイルです。               |
| `disabled` | `false`      | `ruby`モジュールを無効にします。         |

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

| 変数         | デフォルト        | 説明                          |
| ---------- | ------------ | --------------------------- |
| `symbol`   | `"🦀 "`       | Rustのバージョンを表示する前に使用される記号です。 |
| `style`    | `"bold red"` | モジュールのスタイルです。               |
| `disabled` | `false`      | `rust`モジュールを無効にします。         |

### 設定例

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## Singularity

The `singularity` module shows the current singularity image, if inside a container and `$SINGULARITY_NAME` is set.

:::

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

`terraform`モジュールには、現在選択されているterraformワークスペースとバージョンが表示されます。 デフォルトでは、Terraformのバージョンは表示されません。これは、多くのプラグインが使用されている場合、Terraformの現在のバージョンでは遅いためです。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.terraform`フォルダが含まれている
- カレントディレクトリに`.tf`の拡張子のファイルが含まれている

### オプション

| 変数             | デフォルト        | 説明                                            |
| -------------- | ------------ | --------------------------------------------- |
| `symbol`       | `"💠 "`       | Terraform ワークスペースを表示する前に使用される記号です。            |
| `show_version` | `false`      | Terraformのバージョンを表示します。 大きなワークスペースでは非常に遅くなります。 |
| `style`        | `"bold 105"` | モジュールのスタイルです。                                 |
| `disabled`     | `false`      | `terraform`モジュールを無効にします。                      |

### 設定例

```toml
# ~/.config/starship.toml

[terraform]
symbol = "🏎💨 "
```

## 時刻

`time`モジュールは、現在の**現地**時間を示します。 `format`設定は、時間の表示方法を制御するために[`chrono`](https://crates.io/crates/chrono)クレートによって使用されます。 使用可能なオプションを確認するには、[chrono strftimeのドキュメント](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)をご覧ください。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで`disabled`を`false`に設定します。

:::

### オプション

| 変数                | デフォルト           | 説明                                                                                                |
| ----------------- | --------------- | ------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`         | 12時間のフォーマットを有効にします。                                                                               |
| `format`          | この表の下を参照してください  | 時刻のフォーマットに使用される[クロノフォーマット文字列](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) です。 |
| `style`           | `"bold yellow"` | モジュールのスタイルです。                                                                                     |
| `utc_time_offset` | `"local"`       | 使用するUTCオフセットを設定します。 -24から24までの間で設定可能です。 フロートが30/45分のタイムゾーンオフセットに対応できるようにします。                      |
| `disabled`        | `true`          | `time`モジュールを無効にします。                                                                               |

`use_12hr` が `true` の場合、`format` のデフォルトは `"%r"` です。 それ以外の場合、デフォルトは`"%T"`です。 `format`を手動で設定すると、`use_12hr`の設定が上書きされます。

### 設定例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = "-5"
```

## ユーザー名

`username`モジュールには、アクティブなユーザーのユーザー名が表示されます。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントユーザーがroot
- カレントユーザーが、ログインしているユーザーとは異なる
- ユーザーがSSHセッションとして接続されている
- `show_always`変数がtrueに設定されている

### オプション

| 変数            | デフォルト           | 説明                        |
| ------------- | --------------- | ------------------------- |
| `style_root`  | `"bold red"`    | ユーザーがrootのときに使用されるスタイルです。 |
| `style_user`  | `"bold yellow"` | 非rootユーザーに使用されるスタイルです。    |
| `show_always` | `false`         | `username`モジュールを常に表示します。  |
| `disabled`    | `false`         | `username`モジュールを無効にします。   |

### 設定例

```toml
# ~/.config/starship.toml

[username]
disabled = true
```
