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

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:
```ps1
$ENV:STARSHIP_CONFIG = "$HOME\.starship"
```

### 用語

**Module**: A component in the prompt giving information based on contextual information from your OS. For example, the "nodejs" module shows the version of NodeJS that is currently installed on your computer, if your current directory is a NodeJS project.

**Segment**: Smaller sub-components that compose a module. For example, the "symbol" segment in the "nodejs" module contains the character that is shown before the version number (⬢ by default).

Here is the representation of the node module. In the following example, "symbol" and "version" are segments within it. Every module also has a prefix and suffix that are the default terminal color.

```
[prefix]      [symbol]     [version]    [suffix]
 "via "         "⬢"        "v10.4.1"       ""
```

### スタイルの設定

Most modules in starship allow you to configure their display styles. This is done with an entry (usually called `style`) which is a string specifying the configuration. Here are some examples of style strings along with what they do. For details on the full syntax, consult the [advanced config guide](/advanced-config/).

- `"fg:green bg:blue"` は、青色の背景に緑色のテキストを設定します
- `"bg:blue fg:bright-green"` は、青色の背景に明るい緑色のテキストを設定します
- `"bold fg:27"` は、 [ANSIカラー](https://i.stack.imgur.com/KTSQa.png) 27の太字テキストを設定します
- `"underline bg:#bf5700"` は、焦げたオレンジ色の背景に下線付きのテキストを設定します
- `"bold italic fg:purple"`は、紫色の太字斜体のテキストを設定します
- `""` はすべてのスタイルを明示的に無効にします

Note that what styling looks like will be controlled by your terminal emulator. For example, some terminal emulators will brighten the colors instead of bolding text, and some color themes use the same values for the normal and bright colors. Also, to get italic text, your terminal must support italics.

## プロンプト

This is the list of prompt-wide configuration options.

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

The default `prompt_order` is used to define the order in which modules are shown in the prompt, if empty or no `prompt_order` is provided. The default is as shown:

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

The `aws` module shows the current AWS region and profile. This is based on `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env var with `~/.aws/config` file.

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

The `battery` module shows how charged the device's battery is and its current charging status. The module is only visible when the device's battery is below 10%.

### オプション

| 変数                   | デフォルト             | 説明                        |
| -------------------- | ----------------- | ------------------------- |
| `full_symbol`        | `"•"`             | バッテリーが満タンのときに表示される記号です。   |
| `charging_symbol`    | `"⇡"`             | バッテリーの充電中に表示される記号です。      |
| `discharging_symbol` | `"⇣"`             | バッテリーが放電しているときに表示される記号です。 |
| `display`            | [link](#バッテリーの表示) | モジュールの閾値とスタイルを表示します。      |
| `disabled`           | `false`           | `battery`モジュールを無効にします。    |

<details>
<summary>There are also options for some uncommon battery states.</summary>

| 変数               | 説明                       |
| ---------------- | ------------------------ |
| `unknown_symbol` | バッテリー状態が不明なときに表示される記号です。 |
| `empty_symbol`   | バッテリーが空のときに表示される記号です。    |

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

| 変数          | 説明                             |
| ----------- | ------------------------------ |
| `threshold` | バッテリーが表示される上限です。               |
| `style`     | displayオプションが使用されている場合のスタイルです。 |

#### 設定例

```toml
[[battery.display]]  # "bold red" style when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" style when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"

# when capacity is over 30%, the battery indicator will not be displayed

```

## 文字

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways: by changing color (red/green) or by changing its shape (❯/✖). The latter will only be done if `use_symbol_for_status` is set to `true`.

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

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

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

The `conda` module shows the current conda environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

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

The `crystal` module shows the currently installed version of Crystal. 次の条件のいずれかが満たされると、モジュールが表示されます。

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

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### オプション

| 変数                  | デフォルト         | 説明                            |
| ------------------- | ------------- | ----------------------------- |
| `truncation_length` | `3`           | 現在のディレクトリを切り捨てる親フォルダーの数です。    |
| `truncate_to_repo`  | `true`        | 現在いるgitリポジトリのルートに切り捨てるかどうかです。 |
| `prefix`            | `"in "`       | ディレクトリ名の直前に表示するprefixです。      |
| `style`             | `"bold cyan"` | モジュールのスタイルです。                 |
| `disabled`          | `false`       | `directory`モジュールを無効にします。      |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

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

The `elm` module shows the currently installed version of Elm. 次の条件のいずれかが満たされると、モジュールが表示されます。

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

The `env_var` module displays the current value of a selected environment variable. The module will be shown only if any of the following conditions are met:

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

## Git コミット

The `git_commit` module shows the current commit hash of the repo in your current directory.

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

The `golang` module shows the currently installed version of Golang. 次の条件のいずれかが満たされると、モジュールが表示されます。

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

## Java

The `java` module shows the currently installed version of Java. 次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt` or `.java-version` file
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

## Julia

The `julia` module shows the currently installed version of Julia. 次の条件のいずれかが満たされると、モジュールが表示されます。

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

Displays the current Kubernetes context name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

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

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

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

## Nix-shell

The `nix_shell` module shows the nix-shell environment. The module will be shown when inside a nix-shell environment.

### オプション

| 変数           | デフォルト         | 説明                                                |
| ------------ | ------------- | ------------------------------------------------- |
| `use_name`   | `false`       | nix-shellの名前を表示します。                               |
| `impure_msg` | `"impure"`    | impureメッセージをカスタマイズします。                            |
| `pure_msg`   | `"pure"`      | pureメッセージをカスタマイズします。                              |
| `symbol`     | `"❄️  "`      | The symbol used before displaying the shell name. |
| `style`      | `"bold blue"` | モジュールのスタイルです。                                     |
| `disabled`   | `false`       | `nix_shell`モジュールを無効にします。                          |

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

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `cargo`, `poetry`, `composer`, `gradle`, `julia` and `mix` packages.

- **npm** – `npm`パッケージバージョンは、現在のディレクトリにある`package.json`から抽出されます
- **cargo** – `cargo`パッケージバージョンは、現在のディレクトリにある`Cargo.toml`から抽出されます。
- **poetry** – `poetry`パッケージバージョンは、現在のディレクトリにある`pyproject.toml`から抽出されます
- **composer** – `composer`パッケージバージョンは、現在のディレクトリにある`composer.json`から抽出されます
- **gradle** – The `gradle` package version is extracted from the `build.gradle` present
- **julia** - The package version is extracted from the `Project.toml` present
- **mix** - The `mix` package version is extracted from the `mix.exs` present

> ⚠️ 表示されるバージョンは、パッケージマネージャーではなく、ソースコードが現在のディレクトリにあるパッケージのバージョンです。

### オプション

| 変数                | デフォルト        | 説明                                                        |
| ----------------- | ------------ | --------------------------------------------------------- |
| `symbol`          | `"📦 "`       | パッケージのバージョンを表示する前に使用される記号です。                              |
| `style`           | `"bold 208"` | モジュールのスタイルです。                                             |
| `display_private` | `false`      | Enable displaying version for packages marked as private. |
| `disabled`        | `false`      | `package` モジュールを無効にします。                                   |

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

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### オプション

| 変数                | デフォルト           | 説明                                                                                                                  |
| ----------------- | --------------- | ------------------------------------------------------------------------------------------------------------------- |
| `use_12hr`        | `false`         | Enables 12 hour formatting                                                                                          |
| `format`          | see below       | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time. |
| `style`           | `"bold yellow"` | The style for the module time                                                                                       |
| `utc_time_offset` | `"local"`       | Sets the UTC offset to use. Range from -24 < x < 24. Allows floats to accommodate 30/45 minute timezone offsets.    |
| `disabled`        | `true`          | Disables the `time` module.                                                                                         |

If `use_12hr` is `true`, then `format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `format` will override the `use_12hr` setting.

### 設定例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = "🕙[ %T ]"
utc_time_offset = "-5"
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
| `shell`       |                           | The path to the shell to use to execute the command. If unset, it will fallback to STARSHIP_SHELL and then to "sh".        |
| `description` | `"<custom module>"` | The description of the module that is shown when running `starship explain`.                                               |
| `files`       | `[]`                      | The files that will be searched in the working directory for a match.                                                      |
| `directories` | `[]`                      | The directories that will be searched in the working directory for a match.                                                |
| `extensions`  | `[]`                      | The extensions that will be searched in the working directory for a match.                                                 |
| `symbol`      | `""`                      | The symbol used before displaying the command output.                                                                      |
| `style`       | `"bold green"`            | モジュールのスタイルです。                                                                                                              |
| `prefix`      | `""`                      | Prefix to display immediately before the command output.                                                                   |
| `suffix`      | `""`                      | Suffix to display immediately after the command output.                                                                    |
| `disabled`    | `false`                   | Disables this `custom` module.                                                                                             |

### 設定例

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
prefix = " transcending "
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
