# 設定

::: tip
🔥設定は現在作業中です。多くの新しいオプションが今後のリリースで利用可能になります。 :::

starshipの設定を開始するには、`~/.config/starship.toml` ファイルを作成します。

```shell
$ touch ~/.config/starship.toml
```

starshipのすべての設定は、この[TOML](https://github.com/toml-lang/toml)ファイルで行われます。

```toml
# プロンプト表示時に改行しない
add_newline = false

# "❯" から "➜" に表示を変更
[character]      # The name of the module we are confguring is "character"
symbol = "➜"     # The "symbol" segment is being set to "➜"

# packageモジュールを無効にする
[package]
disabled = true
```

### Terminology

**モジュール**: OSのコンテキスト情報に基づいて情報を提供するプロンプト内のコンポーネントです。たとえば、現在のディレクトリがNodeJSプロジェクトである場合、「nodejs」モジュールは、現在コンピューターにインストールされているNodeJSのバージョンを表示します。

**セグメント**: モジュールを構成する小さなサブコンポーネントです。たとえば、「nodejs」モジュールの「symbol」セグメントには、バージョン番号の前に表示される文字が含まれています（デフォルト: ⬢）。

以下はNode モジュールの表現です。次の例では、「シンボル」と「バージョン」はその中のセグメントです。すべてのモジュールには、デフォルトの端末色であるprefixとsuffixもあります。

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### スタイルの設定

starshipのほとんどのモジュールでは、表示スタイルを設定できます。これは、設定を指定する文字列であるエントリ（`style`）で行われます。スタイル文字列の例とその機能を次に示します。完全な構文の詳細については、詳細は [高度な設定](/advanced-config/)を参照してください 。

- `"fg:green bg:blue"` sets green text on a blue background
- `"bg:blue fg:bright-green"` sets bright green text on a blue background
- `"bold fg:27"` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `"underline bg:#bf5700"` sets underlined text on a burnt orange background
- `""` explicitly disables all styling

スタイリングがどのように見えるかは、端末エミュレータによって制御されることに注意してください。たとえば、一部の端末エミュレータはテキストを太字にする代わりに色を明るくします。また、一部のカラーテーマは通常の色と明るい色と同じ値を使用します。

## Prompt

これは、プロンプト全体のオプションのリストです。

### Options

変数 | デフォルト | 説明
--- | --- | ---
`add_newline` | `true` | Add a new line before the start of the prompt.
`prompt_order` | [link](#%E3%83%87%E3%83%95%E3%82%A9%E3%83%AB%E3%83%88%E3%81%AE%E3%83%97%E3%83%AD%E3%83%B3%E3%83%97%E3%83%88%E9%A0%86) | プロンプトモジュールを出力する順序を設定します。

### 設定例

```toml
# ~/.config/starship.toml

# Disable the newline at the start of the prompt
add_newline = false
# Overwrite a default_prompt_order and  use custom prompt_order
prompt_order=["rust","line_break","package","line_break","character"]
```

### デフォルトのプロンプト表示順

`default_prompt_order`オプションは、空または`prompt_order`が指定されていない場合に、プロンプトにモジュールが表示される順序を定義するために使用されます。デフォルトは次のとおりです。

```
default_prompt_order = [
    "username",
    "hostname",
    "directory",
    "git_branch",
    "git_state",
    "git_status",
    "package",
    "nodejs",
    "ruby",
    "rust",
    "python",
    "golang",
    "nix_shell",
    "cmd_duration",
    "line_break",
    "jobs",
    "time",
    "battery",
    "character",
]
```

## バッテリー

The `battery` module shows how charged the device's battery is and its current charging status.
The module is only visible when the device's battery is below 10%.

### オプション

Variable | デフォルト | 説明
--- | --- | ---
`full_symbol` | `"•"` | バッテリーが満タンのときに表示される記号です。
`charging_symbol` | `"⇡"` | バッテリーの充電中に表示される記号です。
`discharging_symbol` | `"⇣"` | バッテリーが放電しているときに表示される記号です。
`style` | `"bold red"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `battery` module.

### 設定例

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋"
charging_symbol = "⚡️"
discharging_symbol = "💀"
```

## 文字

The `character` module shows a character (usually an arrow) beside where the text
is entered in your terminal.

文字は、最後のコマンドが成功したかどうかを示します。これは、色の変更（赤/緑）またはその形状の変更(❯/✖)の2つの方法で行うことができます。後者は`use_symbol_for_status`に`true`設定されている場合にのみ行われます。

### オプション

Variable | Default | 説明
--- | --- | ---
`symbol` | `"❯"` | プロンプトでテキストを入力する前に使用される記号です。
`error_symbol` | `"✖"` | 前のコマンドが失敗した場合にテキスト入力の前に使用される記号です。
`use_symbol_for_status` | `false` | Indicate error status by changing the symbol.
`vicmd_symbol` | `"❮"` | シェルがvimの通常モードである場合、プロンプトのテキスト入力の前に使用される記号です。
`style_success` | `"bold green"` | 最後のコマンドが成功した場合に使用されるスタイルです。
`style_failure` | `"bold red"` | 最後のコマンドが失敗した場合に使用されるスタイルです。
`disabled` | `false` | Disables the `character` module.

### 設定例

```toml
# ~/.config/starship.toml

[character]
symbol = "➜"
error_symbol = "✗"
use_symbol_for_status = true
```

## コマンド実行時間

The `cmd_duration` module shows how long the last command took to execute.
The module will be shown only if the command took longer than two seconds, or
the `min_time` config value, if it exists.

::: warning BashでDEBUGトラップをhookしない
`bash`でStarshipを実行している場合、 `eval $(starship init $0)`実行した後に`DEBUG`トラップをフックしないでください。そうしないと、このモジュールが**おそらくですが**壊れます。 :::

preexecのような機能を必要とするBashユーザーは、 [rcalorasのbash_preexecフレームワーク](https://github.com/rcaloras/bash-preexec)を使用できます。 `eval $(starship init $0)` を実行する前に、`preexec_functions` 、および`precmd_functions`定義するだけで、通常どおり続行します。

### Options

Variable | Default | Description
--- | --- | ---
`min_time` | `2` | 時間を表示する最短期間です。
`style` | `"bold yellow"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `cmd_duration` module.

### 設定例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 4
```

## ディレクトリ

`directory`モジュールには、現在のディレクトリへのパスが表示され、3つの親フォルダは切り捨てられます。ディレクトリは、現在のgitリポジトリであるとルートとなります。

When using the fish style pwd option, instead of hiding the path that is
truncated, you will see a shortened name of each directory based on the number
you enable for the option.

たとえば、`~/Dev/Nix/nixpkgs/pkgs`で、`nixpkgs`がリポジトリルートであり、オプションが`1`に設定されている場合、`~/D/N/nixpkgs/pkgsが表示されますが、以前はnixpkgs/pkgs`でした。

### Options

Variable | Default | 説明
--- | --- | ---
`truncation_length` | `3` | 現在のディレクトリを切り捨てる親フォルダーの数です。
`truncate_to_repo` | `true` | 現在いるgitリポジトリのルートに切り捨てるかどうかです。
`fish_style_pwd_dir_length` | `0` | fish shellのpwdパスロジックを適用するときに使用する文字数です。
`style` | `"bold cyan"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `directory` module.

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Git ブランチ

`git_branch`Moduleは、現在のディレクトリにあるレポのアクティブなブランチを表示します。

### オプション

変数 | デフォルト | Description
--- | --- | ---
`symbol` | `" "` | 現在のディレクトリのリポジトリのブランチ名の前に使用されるシンボルです。
`truncation_length` | `2^63 - 1` | Truncates a git branch to X graphemes
`truncation_symbol` | `"…"` | ブランチ名切り捨てられていることを示すための記号です。記号なしに「」も使用できます。
`style` | `"bold purple"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `git_branch` module.

### 設定例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = "4"
truncation_symbol = ""
```

## Git の進行状態

`git_state` Module はgitディレクトリの進行状態を表します。
例: *REBASING*, *BISECTING*, その他

進捗情報がある場合(例: REBASING 3/10)はその情報も表示されます。

### Options

Variable | Default | Description
--- | --- | ---
`rebase` | `"REBASING"` | `rebase`進行中に表示されるテキストです。
`merge` | `"MERGING"` | `merge`進行中に表示されるテキストです。
`revert` | `"REVERTING"` | `revert`進行中に表示されるテキストです。
`cherry_pick` | `"CHERRY-PICKING"` | `cherry-pick`進行中に表示されるテキストです。
`bisect` | `"BISECTING"` | `bisect`進行中に表示されるテキストです。
`am` | `"AM"` | `apply-mailbox` （ `git am` ）の進行中に表示されるテキストです。
`am_or_rebase` | `"AM/REBASE"` | あいまいな`apply-mailbox`または`rebase`が進行中のときに表示されるテキストです。
`progress_divider` | `"/"` | 現在の進行量と合計進行量を分ける記号またはテキスト(例えば、 `" of "` 、 `"3 of 10"` )です。
`style` | `"bold yellow"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `git_state` module.

### 設定例

```toml
# ~/.config/starship.toml

[git_state]
progress_divider = " of "
cherry_pick = "🍒 PICKING"
```

## Gitの状態

The `git_status` module shows symbols representing the state of the repo in your
current directory.

### Options

Variable | Default | 説明
--- | --- | ---
`conflicted` | `"="` | This branch has merge conflicts.
`ahead` | `"⇡"` | This branch is ahead of the branch being tracked.
`behind` | `"⇣"` | This branch is behind of the branch being tracked.
`diverged` | `"⇕"` | This branch has diverged from the branch being tracked.
`untracked` | `"?"` | There are untracked files in the working directory.
`stashed` | `"$"` | A stash exists for the local repository.
`modified` | `"!"` | There are file modifications in the working directory.
`staged` | `"+"` | A new file has been added to the staging area.
`renamed` | `"»"` | A renamed file has been added to the staging area.
`deleted` | `"✘"` | A file's deletion has been added to the staging area.
`show_sync_count` | `false` | Show ahead/behind count of the branch being tracked.
`style` | `"bold red"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `git_status` module.

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
staged = "➕"
renamed = "👅"
deleted = "🗑"
```

## Golang

The `golang` module shows the currently installed version of Golang.
The module will be shown if any of the following conditions are met:

- カレントディレクトリに`go.mod`ファイルが含まれている
- カレントディレクトリに`go.sum`ファイルが含まれている
- カレントディレクトリに`glide.yaml`ファイルが含まれている
- カレントディレクトリに`Gopkg.yml`ファイルが含まれている
- カレントディレクトリに`Gopkg.lock`ファイルが含まれている
- カレントディレクトリに`Godeps`ファイルが含まれている
- カレントディレクトリに`.go`の拡張子のファイルが含まれている

### Options

Variable | Default | Description
--- | --- | ---
`symbol` | `"🐹 "` | Golangのバージョンを表示する前に使用される記号です。
`style` | `"bold cyan"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `golang` module.

### 設定例

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```

## ホスト名

The `hostname` module shows the system hostname.

### オプション

Variable | Default | Description
--- | --- | ---
`ssh_only` | `true` | Only show hostname when connected to an SSH session.
`prefix` | `""` | ホスト名の直前に表示するprefixです。
`suffix` | `""` | ホスト名の直後に表示するsuffixです。
`style` | `"bold dimmed green"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `hostname` module.

### 設定例

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
prefix = "⟪"
suffix = "⟫"
disabled = false
```

## ジョブ

`jobs`モジュールには、実行中のジョブの現在の数が表示されます。このモジュールは、実行中のバックグラウンドジョブがある場合にのみ表示されます。1つ以上のジョブがある、または`threshold`に指定した値以上にジョブがある場合は実行中のジョブの数を表示します。

### オプション

Variable | Default | Description
--- | --- | ---
`symbol` | `"✦ "` | ジョブの数を表示する前に使用される記号です。
`threshold` | `1` | Show number of jobs if exceeded.
`style` | `"bold blue"` | モジュールのスタイルです。
`disabled` | `false` | `jobs`モジュールを無効にします。

### 設定例

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
threshold = 4
```

## 改行

The `line_break` module separates the prompt into two lines.

### Options

Variable | Default | Description
--- | --- | ---
`disabled` | `false` | Disables the `line_break` module, making the prompt a single line.

### 設定例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Nix-shell

`nix_shell`モジュールは、nix-shell環境を示しています。このモジュールは、nixシェル環境内にあるときに表示されます。

### Options

Variable | Default | Description
--- | --- | ---
`use_name` | `false` | Display the name of the nix-shell.
`impure_msg` | `impure` | impureメッセージをカスタマイズします。
`pure_msg` | `pure` | pureメッセージをカスタマイズします。
`style` | `"bold red"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `nix_shell` module.

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

The `nodejs` module shows the currently installed version of NodeJS.
The module will be shown if any of the following conditions are met:

- カレントディレクトリに`package.json`ファイルが含まれている
- カレントディレクトリに`node_modules`ディレクトリが含まれている
- カレントディレクトリに`.js`の拡張子のファイルが含まれている

### Options

Variable | デフォルト | 説明
--- | --- | ---
`symbol` | `"⬢ "` | NodeJSのバージョンを表示する前に使用される記号です。
`style` | `"bold green"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `nodejs` module.

### 設定例

```toml
# ~/.config/starship.toml

[nodejs]
symbol = "🤖 "
```

## パッケージのバージョン

The `package` module is shown when the current directory is the repository for a
package, and shows its current version. The module currently supports `npm`, `cargo`,
and `poetry` packages.

- **npm** – The `npm` package version is extracted from the `package.json` presentin the current directory
- **cargo** – `cargo`パッケージバージョンは、現在のディレクトリにある`Cargo.toml`から抽出されます。
- **poetry** – The `poetry` package version is extracted from the `pyproject.toml` presentin the current directory

> ⚠️ 表示されるバージョンは、パッケージマネージャーではなく、ソースコードが現在のディレクトリにあるパッケージのバージョンです。

### Options

Variable | Default | Description
--- | --- | ---
`symbol` | `"📦 "` | パッケージのバージョンを表示する前に使用される記号です。
`style` | `"bold red"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `package` module.

### 設定例

```toml
# ~/.config/starship.toml

[package]
symbol = "🎁 "
```

## Python

The `python` module shows the currently installed version of Python.

`pyenv_version_name`が`true`に設定されている場合 、pyenvでのバージョン名が表示されます 。

Otherwise, it will display the version number from `python --version`
and show the current Python virtual environment if one is
activated.

The module will be shown if any of the following conditions are met:

- カレントディレクトリに`.python-version`ファイルが含まれている
- カレントディレクトリに`requirements.txt`ファイルが含まれている
- カレントディレクトリに`pyproject.toml`ファイルが含まれている
- カレントディレクトリに`.py`の拡張子のファイルが含まれている
- カレントディレクトリに`Pipfile`ファイルが含まれている

### Options

変数 | Default | 説明
--- | --- | ---
`symbol` | `"🐍 "` | Pythonのバージョンを表示する前に使用される記号です。
`pyenv_version_name` | `false` | pyenvを使用してPythonバージョンを取得します
`pyenv_prefix` | `"pyenv "` | pyenvバージョン表示の前のprefix（デフォルトの表示は`pyenv MY_VERSION` ）です
`style` | `"bold yellow"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `python` module.

### 設定例

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
pyenv_prefix = "foo "
```

## Ruby

The `ruby` module shows the currently installed version of Ruby.
The module will be shown if any of the following conditions are met:

- カレントディレクトリに`Gemfile`ファイルが含まれている
- カレントディレクトリに`.rb`の拡張子のファイルが含まれている

### Options

Variable | デフォルト | 説明
--- | --- | ---
`symbol` | `"💎 "` | Rubyのバージョンを表示する前に使用される記号です。
`style` | `"bold red"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `ruby` module.

### 設定例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

The `rust` module shows the currently installed version of Rust.
The module will be shown if any of the following conditions are met:

- カレントディレクトリに`Cargo.toml`ファイルが含まれている
- カレントディレクトリに`.rs`の拡張子のファイルが含まれている

### Options

Variable | Default | Description
--- | --- | ---
`symbol` | `"🦀 "` | Rustのバージョンを表示する前に使用される記号です。
`style` | `"bold red"` | モジュールのスタイルです。
`disabled` | `false` | Disables the `rust` module.

### 設定例

```toml
# ~/.config/starship.toml

[rust]
symbol = "⚙️ "
```

## 時刻

`time`モジュールは、現在の**現地**時間を示します。 `format`設定は、時間の表示方法を制御するために[`chrono`](https://crates.io/crates/chrono)クレートによって使用されます。使用可能なオプションを確認するには、[chrono strftimeのドキュメント](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)をご覧ください。

::: tip
このモジュールはデフォルトで無効になっています。有効にするには、設定ファイルで`disabled`を`false`に設定します。 :::

### Options

Variable | Default | Description
--- | --- | ---
`12hr` | `false` | 12時間のフォーマットを有効にします。
`format` | この表の下を参照してください | 時刻のフォーマットに使用される[クロノフォーマット文字列](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) です。
`style` | `bold yellow` | モジュールのスタイルです。
`disabled` | `true` | Disables the `time` module.

`12hr`が`true` 、 `format`デフォルトで`"%r"`です。それ以外の場合、デフォルトは`"%T"`です。 `format`を手動で設定すると、 `12hr`の設定が上書きされます。

### 設定例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
```

## ユーザ名

The `username` module shows active user's username.
The module will be shown if any of the following conditions are met:

- カレントユーザーがroot
- カレントユーザーが、ログインしているユーザーとは異なる
- ユーザーがSSHセッションとして接続されている

### オプション

Variable | Default | Description
--- | --- | ---
`style_root` | `"bold red"` | ユーザーがrootのときに使用されるスタイルです。
`style_user` | `"bold yellow"` | 非rootユーザーに使用されるスタイルです。
`disabled` | `false` | Disables the `username` module.

### 設定例

```toml
# ~/.config/starship.toml

[username]
disabled = true
```
