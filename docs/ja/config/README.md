# 設定

::: tip 🔥Starshipの開発は現在も進んでいます。 多くの新しいオプションが今後のリリースで利用可能になります。 :::

Starshipの設定を開始するには、`~/.config/starship.toml` ファイルを作成します。

```shell
$ touch ~/.config/starship.toml
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

    [prefix]      [symbol]     [version]    [suffix]
     "via "         "⬢"        "v10.4.1"       ""
    

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

| 変数             | デフォルト                         | 説明                       |
| -------------- | ----------------------------- | ------------------------ |
| `add_newline`  | `true`                        | プロンプトの開始前に新しい行を追加します。    |
| `prompt_order` | [link](#default-prompt-order) | プロンプトモジュールを出力する順序を設定します。 |


### 設定例

```toml
# ~/.config/starship.toml

# プロンプト表示の改行を無効にする
add_newline = false
# デフォルトのプロンプト表示順を書き換える
prompt_order=["rust","line_break","package","line_break","character"]
```

### デフォルトのプロンプト表示順

デフォルトの`prompt_order`は、空の場合、または`prompt_order`が指定されていない場合に、プロンプトにモジュールが表示される順序を定義するために使用されます。 デフォルトは次のとおりです。

```toml
prompt_order = [
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
    "java",
    "nix_shell",
    "cmd_duration",
    "line_break",
    "jobs",
    "battery",
    "time",
    "character",
]
```

## バッテリー

`battery`モジュールは、デバイスのバッテリー残量と現在の充電状態を示します。 モジュールは、デバイスのバッテリー残量が10％未満の場合にのみ表示されます。

### オプション

| 変数                   | デフォルト                    | 説明                        |
| -------------------- | ------------------------ | ------------------------- |
| `full_symbol`        | `"•"`                    | バッテリーが満タンのときに表示される記号です。   |
| `charging_symbol`    | `"⇡"`                    | バッテリーの充電中に表示される記号です。      |
| `discharging_symbol` | `"⇣"`                    | バッテリーが放電しているときに表示される記号です。 |
| `display`            | [link](#battery-display) | モジュールの閾値とスタイルを表示します。      |
| `disabled`           | `false`                  | `battery`モジュールを無効にします。    |


<details>
<summary>There are also options for some uncommon battery states.</summary>

| 変数               | 説明                                                  |
| ---------------- | --------------------------------------------------- |
| `unknown_symbol` | The symbol shown when the battery state is unknown. |
| `empty_symbol`   | The symbol shown when the battery state is empty.   |


Note: Battery indicator will be hidden if the status is `unknown` or `empty` unless you specify the option in the config.

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

The `display` configuration option is used to define when the battery indicator should be shown (threshold) and what it looks like (style). If no `display` is provided. The default is as shown:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

#### オプション

The `display` option is an array of the following table.

| 変数          | Description                                     |
| ----------- | ----------------------------------------------- |
| `threshold` | The upper bound for the display option.         |
| `style`     | The style used if the display option is in use. |


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

The character will tell you whether the last command was successful or not. It can do this in two ways: by changing color (red/green) or by changing its shape (❯/✖). The latter will only be done if `use_symbol_for_status` is set to `true`.

### オプション

| 変数                      | デフォルト          | 説明                                                                                  |
| ----------------------- | -------------- | ----------------------------------------------------------------------------------- |
| `symbol`                | `"❯"`          | The symbol used before the text input in the prompt.                                |
| `error_symbol`          | `"✖"`          | The symbol used before text input if the previous command failed.                   |
| `use_symbol_for_status` | `false`        | Indicate error status by changing the symbol.                                       |
| `vicmd_symbol`          | `"❮"`          | The symbol used before the text input in the prompt if shell is in vim normal mode. |
| `style_success`         | `"bold green"` | The style used if the last command was successful.                                  |
| `style_failure`         | `"bold red"`   | The style used if the last command failed.                                          |
| `disabled`              | `false`        | Disables the `character` module.                                                    |


### 設定例

```toml
# ~/.config/starship.toml

[character]
symbol = "➜"
error_symbol = "✗"
use_symbol_for_status = true
```

## コマンド実行時間

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break. :::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### オプション

| 変数         | デフォルト           | 説明                                  |
| ---------- | --------------- | ----------------------------------- |
| `min_time` | `2`             | Shortest duration to show time for. |
| `style`    | `"bold yellow"` | The style for the module.           |
| `disabled` | `false`         | Disables the `cmd_duration` module. |


### 設定例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 4
```

## ディレクトリ

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### オプション

| 変数                  | デフォルト         | 説明                                                                               |
| ------------------- | ------------- | -------------------------------------------------------------------------------- |
| `truncation_length` | `3`           | The number of parent folders that the current directory should be truncated to.  |
| `truncate_to_repo`  | `true`        | Whether or not to truncate to the root of the git repo that you're currently in. |
| `style`             | `"bold cyan"` | The style for the module.                                                        |
| `disabled`          | `false`       | Disables the `directory` module.                                                 |


<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| 変数                          | デフォルト  | 説明                                                                                       |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------- |
| `fish_style_pwd_dir_length` | `0`    | The number of characters to use when applying fish shell pwd path logic.                 |
| `use_logical_path`          | `true` | Displays the logical path provided by the shell (`PWD`) instead of the path from the OS. |


</details>

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
```

## Git ブランチ

The `git_branch` module shows the active branch of the repo in your current directory.

### オプション

| 変数                  | デフォルト           | 説明                                                                                    |
| ------------------- | --------------- | ------------------------------------------------------------------------------------- |
| `symbol`            | `" "`          | The symbol used before the branch name of the repo in your current directory.         |
| `truncation_length` | `2^63 - 1`      | Truncates a git branch to X graphemes                                                 |
| `truncation_symbol` | `"…"`           | The symbol used to indicate a branch name was truncated. You can use "" for no symbol |
| `style`             | `"bold purple"` | The style for the module.                                                             |
| `disabled`          | `false`         | Disables the `git_branch` module.                                                     |


### 設定例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = "4"
truncation_symbol = ""
```

## Git の進行状態

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: *REBASING*, *BISECTING*, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

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

## Git の状態

The `git_status` module shows symbols representing the state of the repo in your current directory.

### オプション

| 変数                | デフォルト        | 説明                                                      |
| ----------------- | ------------ | ------------------------------------------------------- |
| `conflicted`      | `"="`        | This branch has merge conflicts.                        |
| `ahead`           | `"⇡"`        | This branch is ahead of the branch being tracked.       |
| `behind`          | `"⇣"`        | This branch is behind of the branch being tracked.      |
| `diverged`        | `"⇕"`        | This branch has diverged from the branch being tracked. |
| `untracked`       | `"?"`        | There are untracked files in the working directory.     |
| `stashed`         | `"$"`        | A stash exists for the local repository.                |
| `modified`        | `"!"`        | There are file modifications in the working directory.  |
| `staged`          | `"+"`        | A new file has been added to the staging area.          |
| `renamed`         | `"»"`        | A renamed file has been added to the staging area.      |
| `deleted`         | `"✘"`        | A file's deletion has been added to the staging area.   |
| `show_sync_count` | `false`      | Show ahead/behind count of the branch being tracked.    |
| `prefix`          | `[`          | Prefix to display immediately before git status.        |
| `suffix`          | `]`          | Suffix to display immediately after git status.         |
| `style`           | `"bold red"` | The style for the module.                               |
| `disabled`        | `false`      | Disables the `git_status` module.                       |


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

The `golang` module shows the currently installed version of Golang. The module will be shown if any of the following conditions are met:

- カレントディレクトリに`go.mod`ファイルが含まれている
- カレントディレクトリに`go.sum`ファイルが含まれている
- カレントディレクトリに`glide.yaml`ファイルが含まれている
- カレントディレクトリに`Gopkg.yml`ファイルが含まれている
- カレントディレクトリに`Gopkg.lock`ファイルが含まれている
- カレントディレクトリに`Godeps`ファイルが含まれている
- カレントディレクトリに`.go`の拡張子のファイルが含まれている

### オプション

| 変数         | デフォルト         | 説明                                                       |
| ---------- | ------------- | -------------------------------------------------------- |
| `symbol`   | `"🐹 "`        | The symbol used before displaying the version of Golang. |
| `style`    | `"bold cyan"` | The style for the module.                                |
| `disabled` | `false`       | Disables the `golang` module.                            |


### 設定例

```toml
# ~/.config/starship.toml

[golang]
symbol = "🏎💨 "
```

## ホスト名

The `hostname` module shows the system hostname.

### オプション

| 変数         | デフォルト                 | 説明                                                   |
| ---------- | --------------------- | ---------------------------------------------------- |
| `ssh_only` | `true`                | Only show hostname when connected to an SSH session. |
| `prefix`   | `""`                  | Prefix to display immediately before the hostname.   |
| `suffix`   | `""`                  | Suffix to display immediately after the hostname.    |
| `style`    | `"bold dimmed green"` | The style for the module.                            |
| `disabled` | `false`               | Disables the `hostname` module.                      |


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

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists.

### オプション

| 変数          | デフォルト         | 説明                                                    |
| ----------- | ------------- | ----------------------------------------------------- |
| `symbol`    | `"✦ "`        | The symbol used before displaying the number of jobs. |
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

## 改行

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
| `impure_msg` | `impure`     | Customize the "impure" msg.        |
| `pure_msg`   | `pure`       | Customize the "pure" msg.          |
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

- カレントディレクトリに`pom.xml`, もしくは`build.gradle`ファイルが含まれている
- カレントディレクトリに拡張子が`.java`, `.class`, もしくは`.jar`のファイルが含まれている

### オプション

| 変数         | デフォルト          | 説明                                                     |
| ---------- | -------------- | ------------------------------------------------------ |
| `symbol`   | `"☕ "`         | The symbol used before displaying the version of Java. |
| `style`    | `"dimmed red"` | モジュールのスタイルです。                                          |
| `disabled` | `false`        | Disables the `java` module.                            |


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

| 変数         | デフォルト          | 説明                                                       |
| ---------- | -------------- | -------------------------------------------------------- |
| `symbol`   | `"⬢ "`         | The symbol used before displaying the version of NodeJS. |
| `style`    | `"bold green"` | The style for the module.                                |
| `disabled` | `false`        | Disables the `nodejs` module.                            |


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

- カレントディレクトリに`Gemfile`ファイルが含まれている
- カレントディレクトリに`.rb`の拡張子のファイルが含まれている

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

- カレントディレクトリに`Cargo.toml`ファイルが含まれている
- カレントディレクトリに`.rs`の拡張子のファイルが含まれている

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

## 時刻

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file. :::

### オプション

| Variable   | Default       | Description                                                                                                         |
| ---------- | ------------- | ------------------------------------------------------------------------------------------------------------------- |
| `12hr`     | `false`       | Enables 12 hour formatting                                                                                          |
| `format`   | see below     | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`    | `bold yellow` | The style for the module time                                                                                       |
| `disabled` | `true`        | Disables the `time` module.                                                                                         |


If `12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `12hr` setting.

### 設定例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
```

## ユーザー名

`username`モジュールには、アクティブなユーザーのユーザー名が表示されます。 次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントユーザーがroot
- カレントユーザーが、ログインしているユーザーとは異なる
- ユーザーがSSHセッションとして接続されている
- The variable `show_always` is set to true

### オプション

| 変数            | デフォルト           | 説明                                  |
| ------------- | --------------- | ----------------------------------- |
| `style_root`  | `"bold red"`    | ユーザーがrootのときに使用されるスタイルです。           |
| `style_user`  | `"bold yellow"` | 非rootユーザーに使用されるスタイルです。              |
| `show_always` | `false`         | Always shows the `username` module. |
| `disabled`    | `false`         | Disables the `username` module.     |


### 設定例

```toml
# ~/.config/starship.toml

[username]
disabled = true
```