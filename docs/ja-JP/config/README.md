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

`STARSHIP_CONFIG` 環境変数を使用して、デフォルトの`starship.toml` ファイルの場所を変更できます。
```shell
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

`aws` モジュールは現在のAWSプロファイルが表示されます。 これは `~/.aws/config` に記述されている `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` 環境変数に基づいています。

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

| 変数                  | デフォルト           | 説明                                                         |
| ------------------- | --------------- | ---------------------------------------------------------- |
| `min_time`          | `2_000`         | Shortest duration to show time for (in milliseconds).      |
| `show_milliseconds` | `false`         | Show milliseconds in addition to seconds for the duration. |
| `prefix`            | `took`          | コマンド実行時間の直前に表示する文字列です。                                     |
| `style`             | `"bold yellow"` | モジュールのスタイルです。                                              |
| `disabled`          | `false`         | `cmd_duration`モジュールを無効にします。                                |

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

</details>

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
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

## Elm

The `elm` module shows the currently installed version of Elm version. The module will be shown if any of the following conditions are met:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `elm-stuff` folder
- The current directory contains a `*.elm` files

### オプション

| 変数         | デフォルト         | 説明                                                    |
| ---------- | ------------- | ----------------------------------------------------- |
| `symbol`   | `"🌳 "`        | The symbol used before displaying the version of Elm. |
| `style`    | `"bold cyan"` | The style for the module.                             |
| `disabled` | `false`       | Disables the `elm` module.                            |


### 設定例

```toml
# ~/.config/starship.toml

[elm]
symbol = " "
```

## Environment Variable

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

### オプション

| 変数         | デフォルト            | 説明                                                                           |
| ---------- | ---------------- | ---------------------------------------------------------------------------- |
| `symbol`   |                  | The symbol used before displaying the variable value.                        |
| `variable` |                  | The environment variable to be displayed.                                    |
| `default`  |                  | The default value to be displayed when the selected variable is not defined. |
| `prefix`   | `""`             | Prefix to display immediately before the variable value.                     |
| `suffix`   | `""`             | Suffix to display immediately after the variable value.                      |
| `style`    | `"dimmed black"` | The style for the module.                                                    |
| `disabled` | `false`          | Disables the `env_var` module.                                               |

### 設定例

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### オプション

| 変数                  | デフォルト           | 説明                                                                                    |
| ------------------- | --------------- | ------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the branch name of the repo in your current directory.         |
| `truncation_length` | `2^63 - 1`      | Truncates a git branch to X graphemes                                                 |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated. You can use "" for no symbol |
| `style`             | `"bold purple"` | モジュールのスタイルです。                                                                         |
| `disabled`          | `false`         | Disables the `git_branch` module.                                                     |

### 設定例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git Commit

The `git_commit` module shows the current commit hash of the repo in your current directory.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### オプション

| 変数                   | デフォルト          | 説明                                               |
| -------------------- | -------------- | ------------------------------------------------ |
| `commit_hash_length` | `7`            | The length of the displayed git commit hash.     |
| `prefix`             | `"("`          | Prefix to display immediately before git commit. |
| `suffix`             | `")"`          | Suffix to display immediately after git commit.  |
| `style`              | `"bold green"` | The style for the module.                        |
| `disabled`           | `true`         | Disables the `git_commit` module.                |

### 設定例

```toml
# ~/.config/starship.toml

[git_commit]
disabled = false
commit_hash_length = 4
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### オプション

| 変数                 | デフォルト              | 説明                                                                                                               |
| ------------------ | ------------------ | ---------------------------------------------------------------------------------------------------------------- |
| `rebase`           | `"REBASING"`       | The text displayed when a `rebase` is in progress.                                                               |
| `merge`            | `"MERGING"`        | The text displayed when a `merge` is in progress.                                                                |
| `revert`           | `"REVERTING"`      | The text displayed when a `revert` is in progress.                                                               |
| `cherry_pick`      | `"CHERRY-PICKING"` | The text displayed when a `cherry-pick` is in progress.                                                          |
| `bisect`           | `"BISECTING"`      | The text displayed when a `bisect` is in progress.                                                               |
| `am`               | `"AM"`             | The text displayed when an `apply-mailbox` (`git am`) is in progress.                                            |
| `am_or_rebase`     | `"AM/REBASE"`      | The text displayed when an ambiguous `apply-mailbox` or `rebase` is in progress.                                 |
| `progress_divider` | `"/"`              | The symbol or text which will separate the current and total progress amounts. (e.g., `" of "`, for `"3 of 10"`) |
| `style`            | `"bold yellow"`    | The style for the module.                                                                                        |
| `disabled`         | `false`            | Disables the `git_state` module.                                                                                 |

### 設定例

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### オプション

| 変数                 | デフォルト                      | 説明                                                      |
| ------------------ | -------------------------- | ------------------------------------------------------- |
| `conflicted`       | `"="`                      | This branch has merge conflicts.                        |
| `conflicted_count` | [link](#git-status-counts) | Show and style the number of conflicts.                 |
| `ahead`            | `"⇡"`                      | This branch is ahead of the branch being tracked.       |
| `behind`           | `"⇣"`                      | This branch is behind of the branch being tracked.      |
| `diverged`         | `"⇕"`                      | This branch has diverged from the branch being tracked. |
| `untracked`        | `"?"`                      | There are untracked files in the working directory.     |
| `untracked_count`  | [link](#git-status-counts) | Show and style the number of untracked files.           |
| `stashed`          | `"$"`                      | A stash exists for the local repository.                |
| `stashed_count`    | [link](#git-status-counts) | Show and style the number of stashes.                   |
| `modified`         | `"!"`                      | There are file modifications in the working directory.  |
| `modified_count`   | [link](#git-status-counts) | Show and style the number of modified files.            |
| `staged`           | `"+"`                      | A new file has been added to the staging area.          |
| `staged_count`     | [link](#git-status-counts) | Show and style the number of files staged files.        |
| `renamed`          | `"»"`                      | A renamed file has been added to the staging area.      |
| `renamed_count`    | [link](#git-status-counts) | Show and style the number of renamed files.             |
| `deleted`          | `"✘"`                      | A file's deletion has been added to the staging area.   |
| `deleted_count`    | [link](#git-status-counts) | Show and style the number of deleted files.             |
| `show_sync_count`  | `false`                    | Show ahead/behind count of the branch being tracked.    |
| `prefix`           | `[`                        | Prefix to display immediately before git status.        |
| `suffix`           | `]`                        | Suffix to display immediately after git status.         |
| `style`            | `"bold red"`               | The style for the module.                               |
| `disabled`         | `false`                    | Disables the `git_status` module.                       |

#### Git Statusのカウント

| 変数        | デフォルト   | 説明                                                     |
| --------- | ------- | ------------------------------------------------------ |
| `enabled` | `false` | Show the number of files                               |
| `style`   |         | Optionally style the count differently than the module |

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

The `golang` module shows the currently installed version of Golang. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### オプション

| 変数         | デフォルト         | 説明                                                       |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`        | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | モジュールのスタイルです。                                            |
| `disabled` | `false`       | Disables the `golang` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```
## Haskell

The `haskell` module shows the currently installed version of Haskell Stack version. The module will be shown if any of the following conditions are met:

- The current directory contains a `stack.yaml` file

### オプション

| 変数         | デフォルト        | 説明                                                        |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"λ "`       | The symbol used before displaying the version of Haskell. |
| `style`    | `"bold red"` | The style for the module.                                 |
| `disabled` | `false`      | Disables the `haskell` module.                            |


### 設定例

```toml
# ~/.config/starship.toml

[haskell]
symbol = " "
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### オプション

| 変数                  | デフォルト           | 説明                                                                                           |
| ------------------- | --------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `truncation_length` | `2^63 - 1`      | Truncates the hg branch name to X graphemes                                                  |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated.                                     |
| `style`             | `"bold purple"` | The style for the module.                                                                    |
| `disabled`          | `true`          | Disables the `hg_branch` module.                                                             |

### 設定例

```toml
# ~/.config/starship.toml

[hg_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Hostname

The `hostname` module shows the system hostname.

### オプション

| 変数         | デフォルト                 | 説明                                                                                                                                   |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                | Only show hostname when connected to an SSH session.                                                                                 |
| `prefix`   | `""`                  | Prefix to display immediately before the hostname.                                                                                   |
| `suffix`   | `""`                  | Suffix to display immediately after the hostname.                                                                                    |
| `trim_at`  | `"."`                 | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `style`    | `"bold dimmed green"` | The style for the module.                                                                                                            |
| `disabled` | `false`               | Disables the `hostname` module.                                                                                                      |

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

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### オプション

| 変数          | デフォルト         | 説明                                                    |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦"`         | The symbol used before displaying the number of jobs. |
| `threshold` | `1`           | Show number of jobs if exceeded.                      |
| `style`     | `"bold blue"` | The style for the module.                             |
| `disabled`  | `false`       | Disables the `jobs` module.                           |

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

| 変数         | デフォルト         | 説明                                                  |
| ---------- | ------------- | --------------------------------------------------- |
| `symbol`   | `"☸ "`        | The symbol used before displaying the Cluster info. |
| `style`    | `"bold blue"` | The style for the module.                           |
| `disabled` | `true`        | Disables the `kubernetes` module                    |

### 設定例

```toml
# ~/.config/starship.toml

[kubernetes]
symbol = "⛵ "
style = "dim green"
disabled = false
```

## Line Break

The `line_break` module separates the prompt into two lines.

### オプション

| 変数         | デフォルト   | 説明                                                                 |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### 設定例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### オプション

| 変数           | デフォルト        | 説明                                 |
| ------------ | ------------ | ---------------------------------- |
| `use_name`   | `false`      | Display the name of the nix-shell. |
| `impure_msg` | `"impure"`   | Customize the "impure" msg.        |
| `pure_msg`   | `"pure"`     | Customize the "pure" msg.          |
| `style`      | `"bold red"` | The style for the module.          |
| `disabled`   | `false`      | Disables the `nix_shell` module.   |

### 設定例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
use_name = true
impure_msg = "impure shell"
pure_msg = "pure shell"
```

## Java

The `java` module shows the currently installed version of Java. The module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle.kts` or `build.sbt` file
- The current directory contains a file with the `.java`, `.class`, `.gradle` or `.jar` extension

### オプション

| 変数         | デフォルト          | 説明                                                     |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"☕ "`         | The symbol used before displaying the version of Java. |
| `style`    | `"dimmed red"` | The style for the module.                              |
| `disabled` | `false`        | Disables the `java` module.                            |

### 設定例

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

### オプション

| 変数                | デフォルト                 | 説明                                                            |
| ----------------- | --------------------- | ------------------------------------------------------------- |
| `show_percentage` | `false`               | Display memory usage as a percentage of the available memory. |
| `show_swap`       | `true`                | Display swap usage if total swap is non-zero.                 |
| `threshold`       | `75`                  | Hide the memory usage unless it exceeds this percentage.      |
| `symbol`          | `"🐏 "`                | The symbol used before displaying the memory usage.           |
| `separator`       | `" | "`               | The symbol or text that will seperate the ram and swap usage. |
| `style`           | `"bold dimmed white"` | The style for the module.                                     |
| `disabled`        | `true`                | Disables the `memory_usage` module.                           |

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

## Crystal

The `crystal` module shows the currently installed version of Crystal. The module will be shown if any of the following conditions are met:

- The current directory contains a `shard.yml` file
- The current directory contains a `.cr` file

### オプション

| 変数         | デフォルト        | 説明                                                        |
| ---------- | ------------ | --------------------------------------------------------- |
| `symbol`   | `"🔮 "`       | The symbol used before displaying the version of crystal. |
| `style`    | `"bold red"` | モジュールのスタイルです。                                             |
| `disabled` | `false`      | Disables the `crystal` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[crystal]
symbol = "✨ "
style = "bold blue"
```

## NodeJS

The `nodejs` module shows the currently installed version of NodeJS. The module will be shown if any of the following conditions are met:

- The current directory contains a `package.json` file
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

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, and `poetry` packages.

- **npm** – The `npm` package version is extracted from the `package.json` present in the current directory
- **cargo** – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- **composer** – The `composer` package version is extracted from the `composer.json` present in the current directory

> ⚠️ 表示されるバージョンは、パッケージマネージャーではなく、ソースコードが現在のディレクトリにあるパッケージのバージョンです。

### オプション

| 変数         | デフォルト        | 説明                                                         |
| ---------- | ------------ | ---------------------------------------------------------- |
| `symbol`   | `"📦 "`       | The symbol used before displaying the version the package. |
| `style`    | `"bold red"` | The style for the module.                                  |
| `disabled` | `false`      | Disables the `package` module.                             |

### 設定例

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## PHP

The `php` module shows the currently installed version of PHP. The module will be shown if any of the following conditions are met:

- The current directory contains a `composer.json` file
- The current directory contains a `.php` file

### オプション

| 変数         | デフォルト        | 説明                                                    |
| ---------- | ------------ | ----------------------------------------------------- |
| `symbol`   | `"🐘 "`       | The symbol used before displaying the version of PHP. |
| `style`    | `"bold red"` | The style for the module.                             |
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

- The current directory contains a `.python-version` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a file with the `.py` extension
- The current directory contains a `Pipfile` file
- The current directory contains a `tox.ini` file
- A virtual environment is currently activated

### オプション

| 変数                   | デフォルト           | 説明                                                                          |
| -------------------- | --------------- | --------------------------------------------------------------------------- |
| `symbol`             | `"🐍 "`          | The symbol used before displaying the version of Python.                    |
| `pyenv_version_name` | `false`         | Use pyenv to get Python version                                             |
| `pyenv_prefix`       | `"pyenv "`      | Prefix before pyenv version display (default display is `pyenv MY_VERSION`) |
| `style`              | `"bold yellow"` | The style for the module.                                                   |
| `disabled`           | `false`         | Disables the `python` module.                                               |

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

- The current directory contains a `Gemfile` file
- The current directory contains a `.rb` file

### オプション

| 変数         | デフォルト        | 説明                                                     |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"💎 "`       | The symbol used before displaying the version of Ruby. |
| `style`    | `"bold red"` | The style for the module.                              |
| `disabled` | `false`      | Disables the `ruby` module.                            |

### 設定例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust. The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### オプション

| 変数         | デフォルト        | 説明                                                     |
| ---------- | ------------ | ------------------------------------------------------ |
| `symbol`   | `"🦀 "`       | The symbol used before displaying the version of Rust. |
| `style`    | `"bold red"` | The style for the module.                              |
| `disabled` | `false`      | Disables the `rust` module.                            |

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

### Options

| Variable          | Default       | Description                                                                                                         |
| ----------------- | ------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`       | Enables 12 hour formatting                                                                                          |
| `format`          | see below     | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `bold yellow` | The style for the module time                                                                                       |
| `utc_time_offset` | `local`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |
| `disabled`        | `true`        | Disables the `time` module.                                                                                         |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `use_12hr` setting.

### Example

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
