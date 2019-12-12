# 設定

::: tip

🔥Starshipの開発は現在も進んでいます。 多くの新しいオプションが今後のリリースで利用可能になります。

:::

Starshipの設定を開始するには、`~/.config/starship.toml` ファイルを作成します。

```shell
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

スタイリングがどのように見えるかは、端末エミュレータによって制御されることに注意してください。 たとえば、一部の端末エミュレータはテキストを太字にする代わりに色を明るくします。また、一部のカラーテーマは通常の色と明るい色と同じ値を使用します。また、斜体のテキストを取得するには、端末で斜体をサポートする必要があります。スタイリングがどのように見えるかは、端末エミュレータによって制御されることに注意してください。たとえば、一部の端末エミュレータはテキストを太字にする代わりに色を明るくします。また、一部のカラーテーマは通常の色と明るい色と同じ値を使用します。 また、斜体のテキストを取得するには、端末で斜体をサポートする必要があります。

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
    "package",
    "dotnet",
    "golang",
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

### オプション

| 変数                | デフォルト           | 説明                                                       |
| ----------------- | --------------- | -------------------------------------------------------- |
| `symbol`          | `"☁️ "`         | 現在のAWSプロファイルを表示する前に表示される記号です。                            |
| `style`           | `"bold yellow"` | モジュールのスタイルです。                                            |
| `disabled`        | `false`         | `aws`モジュールを無効にします。                                       |
| `displayed_items` | `all`           | 表示するアイテムを選択します。 指定可能な値は以下です。[`all`, `profile`, `region`] |

### 設定例

```toml
# ~/.config/starship.toml

[aws]
style = "bold blue"
symbol = "🅰 "
displayed_items = "region"
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

`display</ 0> オプションを使用して、バッテリーインジケーターを表示するタイミング（閾値）と外観（スタイル）を定義します。
<code>display` が提供されない場合、 デフォルトは次のとおりです。

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

| 変数         | デフォルト           | 説明                          |
| ---------- | --------------- | --------------------------- |
| `min_time` | `2`             | 時間を表示する最短期間です。              |
| `prefix`   | `took`          | コマンド実行時間の直前に表示する文字列です。      |
| `style`    | `"bold yellow"` | モジュールのスタイルです。               |
| `disabled` | `false`         | `cmd_duration`モジュールを無効にします。 |

### 設定例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 4
prefix = "underwent "
```

## Conda

The `conda` module shows the current conda environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### オプション

| 変数                  | デフォルト          | 説明                                                                                                                                                                                                          |
| ------------------- | -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`            | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `symbol`            | `"C "`         | The symbol used before the environment name.                                                                                                                                                                |
| `style`             | `"bold green"` | The style for the module.                                                                                                                                                                                   |
| `disabled`          | `false`        | Disables the `conda` module.                                                                                                                                                                                |

### 設定例

```toml
# ~/.config/starship.toml

[conda]
style = "dimmed green"
```

## ディレクトリ

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### オプション

| 変数                  | デフォルト         | 説明                                                  |
| ------------------- | ------------- | --------------------------------------------------- |
| `truncation_length` | `3`           | 現在のディレクトリを切り捨てる親フォルダーの数です。                          |
| `truncate_to_repo`  | `true`        | 現在いるgitリポジトリのルートに切り捨てるかどうかです。                       |
| `prefix`            | `"in "`       | Prefix to display immediately before the directory. |
| `style`             | `"bold cyan"` | The style for the module.                           |
| `disabled`          | `false`       | Disables the `directory` module.                    |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| 変数                          | デフォルト  | 説明                                           |
| --------------------------- | ------ | -------------------------------------------- |
| `fish_style_pwd_dir_length` | `0`    | fish shellのpwdパスロジックを適用するときに使用する文字数です。       |
| `use_logical_path`          | `true` | OSからのパスの代わりに、シェル(`PWD`) によって提供される論理パスを表示します。 |

</details>

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Dotnet

The `dotnet` module shows the relevant version of the .NET Core SDK for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

This module will only be shown in your prompt when one of the following files are present in the current directory: `global.json`, `project.json`, `*.sln`, `*.csproj`, `*.fsproj`, `*.xproj`. You'll also need the .NET Core command-line tools installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

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

## 環境変数

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

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

The `git_branch` module shows the active branch of the repo in your current directory.

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

## Git Commit

The `git_commit` module shows the active branch of the repo in your current directory.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### オプション

| 変数                   | デフォルト          | 説明                                               |
| -------------------- | -------------- | ------------------------------------------------ |
| `commit_hash_length` | `7`            | The length of the displayed git commit hash.     |
| `prefix`             | `"("`          | Prefix to display immediately before git commit. |
| `suffix`             | `")"`          | Suffix to display immediately after git commit.  |
| `style`              | `"bold green"` | モジュールのスタイルです。                                    |
| `disabled`           | `true`         | Disables the `git_commit` module.                |

### 設定例

```toml
# ~/.config/starship.toml

[git_commit]
disabled = false
commit_hash_length = 4
```

## Git の進行状態

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

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

The `git_status` module shows symbols representing the state of the repo in your current directory.

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

The `golang` module shows the currently installed version of Golang. The module will be shown if any of the following conditions are met:

- カレントディレクトリに`go.mod`ファイルが含まれている
- カレントディレクトリに`go.sum`ファイルが含まれている
- カレントディレクトリに`glide.yaml`ファイルが含まれている
- カレントディレクトリに`Gopkg.yml`ファイルが含まれている
- カレントディレクトリに`Gopkg.lock`ファイルが含まれている
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

## Mercurial Branch

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

## ホスト名

The `hostname` module shows the system hostname.

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

## ジョブ

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

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

## Kubernetes

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### オプション

| 変数         | デフォルト         | 説明                        |
| ---------- | ------------- | ------------------------- |
| `symbol`   | `"☸ "`        | クラスタ情報を表示する前に使用される記号です。   |
| `style`    | `"bold blue"` | モジュールのスタイルです。             |
| `disabled` | `true`        | `Kubernetes`モジュールを無効にします。 |

### 設定例

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dim green"
disabled = false
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

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### オプション

| 変数           | デフォルト        | 説明                       |
| ------------ | ------------ | ------------------------ |
| `use_name`   | `false`      | nix-shellの名前を表示します。      |
| `impure_msg` | `impure`     | impureメッセージをカスタマイズします。   |
| `pure_msg`   | `pure`       | pureメッセージをカスタマイズします。     |
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

## メモリ使用量

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### オプション

| 変数                | デフォルト                 | 説明                                                            |
| ----------------- | --------------------- | ------------------------------------------------------------- |
| `show_percentage` | `false`               | メモリ使用量を割合で表示します。                                              |
| `show_swap`       | `true`                | 合計スワップがゼロ以外の場合、スワップ使用量を表示します。                                 |
| `threshold`       | `75`                  | この閾値を超えない限り、メモリ使用率は表示されません。                                   |
| `symbol`          | `"🐏 "`                | メモリ使用率を表示する前に使用される記号です。                                       |
| `separator`       | `" | "`               | The symbol or text that will seperate the ram and swap usage. |
| `style`           | `"bold dimmed white"` | モジュールのスタイルです。                                                 |
| `disabled`        | `true`                | `memory_usage`モジュールを無効にします。                                   |

### 設定例

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

## Java

The `java` module shows the currently installed version of Java. 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`pom.xml`, `build.gradle`,もしくは`build.sbt`が含まれている
- カレントディレクトリに拡張子が`.java`, `.class`, もしくは`.jar`のファイルが含まれている

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

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. The module will be shown if any of the following conditions are met:

- カレントディレクトリに`package.json`ファイルが含まれている
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

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, and `poetry` packages.

- **npm** – `npm`パッケージバージョンは、現在のディレクトリにある`package.json`から抽出されます
- **cargo** – `cargo`パッケージバージョンは、現在のディレクトリにある`Cargo.toml`から抽出されます。
- **poetry** – `poetry`パッケージバージョンは、現在のディレクトリにある`pyproject.toml`から抽出されます
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory

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

The `php` module shows the currently installed version of PHP. The module will be shown if any of the following conditions are met:

- カレントディレクトリに`composer.json`ファイルが含まれている
- The current directory contains a `.php` file

### オプション

| 変数         | デフォルト        | 説明                                                    |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold red"` | モジュールのスタイルです。                                         |
| `disabled` | `false`      | Disables the `php` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[php]
symbol = "🔹 "
```

## Python

The `python` module shows the currently installed version of Python.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name.

Otherwise, it will display the version number from `python --version` and show the current Python virtual environment if one is activated.

The module will be shown if any of the following conditions are met:

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

The `ruby` module shows the currently installed version of Ruby. The module will be shown if any of the following conditions are met:

- カレントディレクトリに`Gemfile`ファイルが含まれている
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

The `rust` module shows the currently installed version of Rust. The module will be shown if any of the following conditions are met:

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

## Terraform

The `terraform` module shows the currently selected terraform workspace and version. By default the terraform version is not shown, since this is slow on current versions of terraform when a lot of plugins are in use. The module will be shown if any of the following conditions are met:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` extension

### オプション

| 変数             | デフォルト        | 説明                                                          |
| -------------- | ------------ | ----------------------------------------------------------- |
| `symbol`       | `"💠 "`       | The symbol used before displaying the terraform workspace.  |
| `show_version` | `false`      | Shows the terraform version. Very slow on large workspaces. |
| `style`        | `"bold 105"` | The style for the module.                                   |
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

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### オプション

| 変数                | デフォルト         | 説明                                                                                                                  |
| ----------------- | ------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`       | Enables 12 hour formatting                                                                                          |
| `format`          | see below     | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `bold yellow` | The style for the module time                                                                                       |
| `disabled`        | `true`        | Disables the `time` module.                                                                                         |
| `utc_time_offset` | `local`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `use_12hr` setting.

### 設定例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = -5
```

## Username

The `username` module shows active user's username. The module will be shown if any of the following conditions are met:

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

### Options

| Variable      | Default         | Description                           |
| ------------- | --------------- | ------------------------------------- |
| `style_root`  | `"bold red"`    | The style used when the user is root. |
| `style_user`  | `"bold yellow"` | The style used for non-root users.    |
| `show_always` | `false`         | Always shows the `username` module.   |
| `disabled`    | `false`         | Disables the `username` module.       |

### Example

```toml
# ~/.config/starship.toml

[username]
disabled = true
```
