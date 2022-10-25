# 設定

Starshipの設定を開始するには、`~/.config/starship.toml` ファイルを作成します。

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Starshipのすべての設定は、この[TOML](https://github.com/toml-lang/toml)ファイルで行われます。

```toml
# エディターの補完を設定スキーマに合わせて取得
"$schema" = 'https://starship.rs/config-schema.json'

# シェルのプロンプトの間に空行を挿入する
add_newline = true

# 記号"❯"を記号"➜"に置き換える
[character] # 編集するモジュールの名前は"character"
success_symbol = "[➜](bold green)" # "success_symbol"を記号"➜"で"bold green"(太字の緑色)にセット
# パッケージモジュールを無効化することでプロンプトを完全に非表示にする
[package]
disabled = true
```

`STARSHIP_CONFIG` 環境変数を使用することによって、デフォルトの設定ファイルの場所を変更できます。

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

PowerShell (Windows) で同様に `$PROFILE` にこの行を追加します。

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

または、Cmd (Windows) の場合、`starship.lua`にこの行を追加します。

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### ロギング

デフォルトでは、Starship は警告やエラーログを `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log` という名前のファイルに出力します。このセッションキーはターミナルのインスタンスに相当します。 しかし、これは `STARSHIP_CACHE` という環境変数を使って変更できます：

```sh
export STARSHIP_CACHE=~/.starship/cache
```

PowerShell (Windows) で同様に `$PROFILE`にこの行を追加します。

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

または、Cmd (Windows) の場合、`starship.lua`にこの行を追加します。

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### 用語

**モジュール**: OSのコンテキスト情報に基づいて情報を提供するプロンプト内のコンポーネントです。 例えば、現在のディレクトリが Node.js のプロジェクトの場合、現在コンピュータにインストールされている Node.js のバージョンが表示されます。

**変数**: モジュールが提供する情報を含むサブコンポーネントを小さくする。 例えば、"nodejs" モジュールの "version" 変数には、Node.js の現在のバージョンが含まれています。

慣例により、ほとんどのモジュールにはデフォルトの端末色の接頭辞（「nodejs」の`via` など）と接尾辞として空のスペースがあります。

### 文字列のフォーマット

文字列の書式は、モジュールがすべての変数を出力する書式です。 ほとんどのモジュールには、モジュールの表示形式を設定する `format` というエントリがあります。 テキスト、変数、およびテキストグループをフォーマット文字列で使用できます。

#### 変数

変数には、 `$` 記号と、その変数の名前が続きます。 変数の名前には英字と数字、`_`のみを含めることができます。

例：

- `$version` は、`version` という名前の変数を持つフォーマット文字列です。
- `$git_branch$git_commit` は `git_branch` と `git_commit` という2つの変数を持つフォーマット文字列です。
- `$git_branch $git_commit` には空白で区切られた 2 つの変数があります。

#### テキストグループ

テキストグループは二つの異なる部分で構成されています。

`[]`で囲まれている最初の部分は、 [フォーマット文字列](#format-strings) です。 テキスト、変数、または入れ子になったテキストグループを追加できます。

2 番目の部分は、 `()`で囲まれている [スタイル文字列](#style-strings) です。 これは、最初のフォーマット文字列のスタイルを設定するために使用できます。

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

スタイリングがどのように見えるかは、端末エミュレータによって制御されることに注意してください。 たとえば、一部の端末エミュレータはテキストを太字にする代わりに色を明るくします。また、一部のカラーテーマは通常の色と明るい色と同じ値を使用します。また、斜体のテキストを取得するには、端末で斜体をサポートする必要があります。スタイリングがどのように見えるかは、端末エミュレータによって制御されることに注意してください。たとえば、一部の端末エミュレータはテキストを太字にする代わりに色を明るくします。また、一部のカラーテーマは通常の色と明るい色と同じ値を使用します。 また、斜体のテキストを取得するには、端末で斜体をサポートする必要があります。

#### 条件付きフォーマット設定

`(` と `)` 内のすべての変数が空の場合、条件付き書式文字列はレンダリングされません。

例：

- `(@$region)` は`region`が`None`または空だった場合表示されませんが、値がある場合は`@` に続いてregionの値が表示されます。
- `(some text)` は括弧の中に変数がないので、常に何も表示しません。
- `$all` が `\[$a$b\]` のショートカットである時、 `$a` と `$b` が両方とも `None` である場合に限り、`($all)` は何も表示しません。 これは `(\[$a$b\] )` と同じ動作をします。

#### 特殊文字

次の記号はフォーマット文字列で特殊な用途を持っているため、エスケープする必要があります。: `$ \ [ ] ( )`.

TOMLには、 [基本文字列とリテラル文字列](https://toml.io/en/v1.0.0#string)の両方があることに注意してください。 設定では、リテラル文字列(シングルクォートで囲まれた文字列)を使用することをお勧めします。基本文字列(ダブルクォートで囲まれた文字列)を使用したい場合は、バックスラッシュ自体をエスケープする必要があります。(例: `\\`を使用する)

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

### Negative matching

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. These take lists of strings to match or not match. "Negative" options, those which should not be matched, are indicated with a leading "!" character. The presence of _any_ negative indicator in the directory will result in the module not being matched.

Extensions are matched against both the characters after the last dot in a filename, and the characters after the first dot in a filename. For example, `foo.bar.tar.gz` will be matched against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Files whose name begins with a dot are not considered to have extensions at all.

To see how this works in practice, you could match TypeScript but not MPEG Transport Stream files thus:

```toml
detect_extensions = ["ts", "!video.ts", "!audio.ts"]
```

## プロンプト

これは、プロンプト全体のオプションのリストです。

### オプション

| オプション             | デフォルト                          | 説明                                                                                                                                                                               |
| ----------------- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | プロンプトの形式を設定します。                                                                                                                                                                  |
| `right_format`    | `""`                           | [右プロンプトの有効化](/advanced-config/#enable-right-prompt)を参照してください。                                                                                                                    |
| `scan_timeout`    | `30`                           | ファイルをスキャンする際のタイムアウト時間 (milliseconds) です。                                                                                                                                         |
| `command_timeout` | `500`                          | Starshipによって実行されたコマンドのタイムアウト時間 (milliseconds) です。                                                                                                                                |
| `add_newline`     | `true`                         | シェルプロンプトの間に空行を挿入します。                                                                                                                                                             |
| `palette`         | `""`                           | Sets which color palette from `palettes` to use.                                                                                                                                 |
| `palettes`        | `{}`                           | Collection of color palettes that assign [colors](/advanced-config/#style-strings) to user-defined names. Note that color palettes cannot reference their own color definitions. |

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

# Set "foo" as custom color palette
palette = "foo"

# Define custom colors
[palettes.foo]
# Overwrite existing color
blue = "21"
# Define new color
mustard = "#af8700"
```

### デフォルトのプロンプトフォーマット

デフォルトの `format` は、空または `format` が指定されていない場合、プロンプトのフォーマットを定義するために使用されます。 デフォルトは次のとおりです。

```toml
format = "$all"

# Which is equivalent to
format = """
$username\
$hostname\
$localip\
$shlvl\
$singularity\
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
$c\
$cmake\
$cobol\
$daml\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$guix_shell\
$haskell\
$helm\
$java\
$julia\
$kotlin\
$lua\
$nim\
$nodejs\
$ocaml\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vlang\
$vagrant\
$zig\
$buf\
$nix_shell\
$conda\
$meson\
$spack\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
$env_var\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$container\
$shell\
$character"""
```

デフォルトのフォーマットを拡張したいだけなら、`$all`を使用できます。 フォーマットに明示的に追加したモジュールは重複しません。 例:

```toml
# ディレクトリを2行目に移動
format = "$all$directory$character"
```

## AWS

`aws`モジュールは、一時的な資格情報を使用する場合、現在のAWSリージョンとプロファイル、および有効期限タイマーを表示します。 モジュールの出力は、必要に応じて`AWS_REGION`、`AWS_DEFAULT_REGION`と`AWS_PROFILE`の環境変数と、`~/.aws/config`と`~/.aws/credentials`のファイルが使用されます。

モジュールは、資格情報が`~/.aws/credentials`にある場合、または`~/.aws/config`に`credential_process`または`sso_start_url`が定義されている場合にのみプロファイルを表示します。 あるいは、環境変数に`AWS_ACCESS_KEY_ID`、`AWS_SECRET_ACCESS_KEY`または`AWS_SESSION_TOKEN`のいずれかが定義されていれば条件を満たします。 もし`force_display`のオプションを`true`に設定した場合、上記の条件による資格情報が検出されない場合でも、利用可能なすべての情報が表示されます。

[aws-vault](https://github.com/99designs/aws-vault)を使う場合、環境変数`AWS_VAULT`からプロファイルが、環境変数`AWS_SESSION_EXPIRATION`から資格情報の有効期限が読み込まれます。

[awsu](https://github.com/kreuzwerker/awsu) を使う場合、そのプロファイルは環境変数 `AWSU_PROFILE` から読まれます。

[AWSume](https://awsu.me)を使う場合、環境変数`AWSUME_PROFILE`からプロファイルが、環境変数`AWSUME_EXPIRATION`から資格情報の有効期限が読み込まれます。

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials` falls back to the `x_security_token_expires` key.

### オプション

| オプション               | デフォルト                                                                 | 説明                                                                                   |
| ------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | module のフォーマットです。                                                                    |
| `symbol`            | `"☁️ "`                                                               | 現在のAWSプロファイルを表示する前に表示される記号です。                                                        |
| `region_aliases`    | `{}`                                                                  | AWS名に加えて表示するリージョンのエイリアスです。                                                           |
| `profile_aliases`   | `{}`                                                                  | AWS名に加えて表示するプロファイルのエイリアスです。                                                          |
| `style`             | `"bold yellow"`                                                       | モジュールのスタイルです。                                                                        |
| `expiration_symbol` | `X`                                                                   | この記号は一時的な資格情報が有効期限切れの場合に表示されます。                                                      |
| `disabled`          | `false`                                                               | `aws`モジュールを無効にします。                                                                   |
| `force_display`     | `false`                                                               | `true`の場合、`credentials`、`credential_process`または`sso_start_url`が設定されていない場合でも情報を表示します。 |

### 変数

| 変数        | 設定例              | 説明                     |
| --------- | ---------------- | ---------------------- |
| region    | `ap-northeast-1` | 現在のAWSリージョン            |
| profile   | `astronauts`     | 現在のAWSプロファイル           |
| duration  | `2h27m20s`       | 一時的な資格情報の有効期限          |
| symbol    |                  | オプション `記号` の値をミラーする    |
| style\* |                  | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

#### すべてを表示

```toml
[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
[aws.profile_aliases]
CompanyGroupFrobozzOnCallAccess = 'Frobozz'
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
[aws.profile_aliases]
Enterprise_Naming_Scheme-voidstars = 'void**'
```

## Azure

`azure` モジュールは、現在のAzureサブスクリプションを表示します。 これは、 `~/.azure/azureProfile.json` ファイルで定義されているデフォルトのサブスクリプションの名前の表示に基づいています。

### オプション

| 変数         | デフォルト                                    | 説明                      |
| ---------- | ---------------------------------------- | ----------------------- |
| `format`   | `"on [$symbol($subscription)]($style) "` | Azure module のフォーマットです。 |
| `symbol`   | `"ﴃ "`                                   | フォーマットで使用される記号です。       |
| `style`    | `"blue bold"`                            | フォーマットで使用されるスタイルです。     |
| `disabled` | `true`                                   | `azure`モジュールを無効にします。    |

### 設定例

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($subscription)]($style) "
symbol = "ﴃ "
style = "blue bold"
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

| オプション                | デフォルト        | 説明                                                                                     |
| -------------------- | ------------ | -------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | バッテリーが表示される上限です。                                                                       |
| `style`              | `"red bold"` | displayオプションが使用されている場合のスタイルです。                                                         |
| `charging_symbol`    |              | displayオプションが使用されている場合はこののシンボルが表示されます。デフォルトはバッテリーの `charging_symbol` オプションと同じになります。    |
| `discharging_symbol` |              | displayオプションが使用されている場合はこののシンボルが表示されます。デフォルトはバッテリーの `discharging_symbol` オプションと同じになります。 |

#### 設定例

```toml
[[battery.display]] # "bold red"(太字の赤)でdischarging_symbolをバッテーリー残量が0%から10%の間で表示
threshold = 10
style = "bold red"

[[battery.display]] # "bold yellow"(太字の黄)で記号💦をバッテリー残量が10%から30%の間で表示
threshold = 30
style = "bold yellow"
discharging_symbol = "💦"

# バッテリー残量が30%を超えると、バッテリーインジケータは表示されません
```

## Buf

`buf`モジュールは、現在インストールされている[Buf](https://buf.build)のバージョンを表示します。 デフォルトでは次のすべての条件が満たされると、モジュールが表示されます。

- [`buf`](https://github.com/bufbuild/buf)CLI がインストールされている
- カレントディレクトリに、[`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml)、[`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml)または[`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml)が含まれている

### オプション

| オプション               | デフォルト                                           | 説明                         |
| ------------------- | ----------------------------------------------- | -------------------------- |
| `format`            | `"with [$symbol($version )]($style)"`           | `buf`モジュールの形式。             |
| `version_format`    | `"v${raw}"`                                     | バージョンのフォーマット。              |
| `symbol`            | `"🦬 "`                                          | Bufのバージョンを表示する前に使用される記号です。 |
| `detect_extensions` | `[]`                                            | どの拡張子がこのモジュールをアクティブにするか    |
| `detect_files`      | `["buf.yaml", "buf.gen.yaml", "buf.work.yaml"]` | どのファイル名がこのモジュールをアクティブにするか  |
| `detect_folders`    | `[]`                                            | どのフォルダーがこのモジュールをアクティブにするか  |
| `style`             | `"bold blue"`                                   | モジュールのスタイルです。              |
| `disabled`          | `false`                                         | `elixir`モジュールを無効にします。      |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| `version` | `v1.0.0` | `buf`のバージョン            |
| `symbol`  |          | オプション `記号` の値をミラーする    |
| `style`*  |          | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[buf]
symbol = "🦬 "
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime. デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`bun.lockb`ファイルが含まれている
- カレントディレクトリに`bunfig.toml`ファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🍞 "`                               | A format string representing the symbol of Node.js.    |
| `detect_extensions` | `[]`                                 | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["bun.lockb", "bunfig.toml"]`       | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `bun` module.                             |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v0.1.4` | The version of `bun`   |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[bun]
format = "via [🍔 $version](bold green) "
```

## C

`c` モジュールは、利用しているCコンパイラに関するいくつかの情報を表示します。 デフォルトでは、カレントディレクトリに`.c`または`.h`ファイルが含まれている場合、モジュールが表示されます。

### オプション

| オプション               | デフォルト                                                                       | 説明                                                     |
| ------------------- | --------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version(-$name) )]($style)"`                                | モジュールのフォーマット文字列。                                       |
| `version_format`    | `"v${raw}"`                                                                 | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"C "`                                                                      | コンパイラの詳細を表示する前に使用される記号です。                              |
| `detect_extensions` | `["c", "h"]`                                                                | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                                                        | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                        | どのフォルダーがこのモジュールをアクティブにするか                              |
| `commands`          | [ [ "cc", "--version" ], [ "gcc", "--version" ], [ "clang", "--version" ] ] | コンパイラを検出する方法                                           |
| `style`             | `"bold 149"`                                                                | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                     | `c`モジュールを無効にします。                                       |

### 変数

| 変数      | 設定例    | 説明                     |
| ------- | ------ | ---------------------- |
| name    | clang  | コンパイラ名                 |
| version | 13.0.0 | コンパイラのバージョン            |
| symbol  |        | オプション `記号` の値をミラーする    |
| style   |        | オプション `style` の値をミラーする |

`version`はデフォルトのフォーマットではないことに注意してください。

### Commands

`commands`オプションは、コンパイラのバージョンと名前を判別するためのコマンドのリストを受け入れます。

各コマンドは、実行可能ファイル名の後に引数を続けるリストとして表されます。通常は`["mycc", "--version"]`のようになります。 StarshipはSTDOUTから結果が得られるまで各コマンドを実行を試みます。

もし、Cコンパイラがこのモジュールでサポートされていない場合は、[GitHubで問題を提起する](https://github.com/starship/starship/)ことでリクエストできます。

### 設定例

```toml
# ~/.config/starship.toml

[c]
format = "via [$name $version]($style)"
```

## 文字

`character`モジュールは、端末でテキストが入力される場所の横に文字（通常は矢印）を表示します。

characterは、最後のコマンドが成功したかどうかを示します。 表し方は下記の2つです。

- 色の変更 (`赤`/`緑`)
- プロンプトの表示の変更 (`❯`/`✖`)

デフォルトでは、色だけが変更されます。 形も変えてみたい場合は[このサンプル](#with-custom-error-shape)も参考にしてください。

::: warning

`vimcmd_symbol`はcmd, fish and zshでのみサポートされています。 `vimcmd_replace_one_symbol`、`vimcmd_replace_symbol`と`vimcmd_visual_symbol`は、[zshでのモード検出による問題](https://github.com/starship/starship/issues/625#issuecomment-732454148)のため、fishでのみサポートされています。

:::

### オプション

| オプション                       | デフォルト                | 説明                                                     |
| --------------------------- | -------------------- | ------------------------------------------------------ |
| `format`                    | `"$symbol"`          | テキスト入力の前に使用される書式文字列。                                   |
| `success_symbol`            | `"[❯](bold green)"`  | 前のコマンドが成功した場合にテキスト入力の前に使用される書式文字列です。                   |
| `error_symbol`              | `"[❯](bold red)"`    | 前のコマンドが失敗した場合にテキスト入力の前に使用される書式文字列です。                   |
| `vimcmd_symbol`             | `"[❮](bold green)"`  | シェルが vim ノーマルモードの場合にテキスト入力の前に使用されるフォーマット文字列。           |
| `vimcmd_replace_one_symbol` | `"[❮](bold purple)"` | シェルがvimの`replace_one`モードの場合にテキスト入力の前に使用されるフォーマット文字列です。 |
| `vimcmd_replace_symbol`     | `"[❮](bold purple)"` | シェルがvimの置換モードの場合にテキスト入力の前に使用されるフォーマット文字列。              |
| `vimcmd_visual_symbol`      | `"[❮](bold yellow)"` | シェルがvimの置換モードの場合にテキスト入力の前に使用されるフォーマット文字列。              |
| `disabled`                  | `false`              | `character`モジュールを無効にします。                               |

### 変数

| 変数     | 設定例 | 説明                                                                                                       |
| ------ | --- | -------------------------------------------------------------------------------------------------------- |
| symbol |     | A mirror of either `success_symbol`, `error_symbol`, `vimcmd_symbol` or `vimcmd_replace_one_symbol` etc. |

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
| `format`            | `"via [$symbol($version )]($style)"`   | module のフォーマットです。                                      |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

## COBOL / GNUCOBOL

`COBOL` モジュールは、現在インストールされているCOBOLのバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに、`.cob`または`.COB`の拡張子のファイルが含まれている
- カレントディレクトリに、`.cbl`または`.CBL`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `symbol`            | `"⚙️ "`                              | COBOLのバージョンを表示する前に使用される記号です。                           |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `style`             | `"bold blue"`                        | モジュールのスタイルです。                                          |
| `detect_extensions` | `["cbl", "cob", "CBL", "COB"]`       | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | `cobol`モジュールを無効にします。                                   |

### 変数

| 変数        | 設定例        | 説明                     |
| --------- | ---------- | ---------------------- |
| version   | `v3.1.2.0` | `cobol`のバージョン          |
| symbol    |            | オプション `記号` の値をミラーする    |
| style\* |            | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

## コマンド実行時間

`cmd_duration`モジュールは、最後のコマンドの実行にかかった時間を示します。 モジュールが表示されるのは、コマンドが2秒以上かかった場合、または`min_time`値が存在する場合のみです。

::: warning BashでDEBUGトラップをhookしない

`bash`でStarshipを実行している場合、 `eval $(starship init $0)`実行した後に`DEBUG`トラップをフックしないでください。そうしないと、このモジュールが**おそらくですが**壊れます。

:::

preexecのような機能を必要とするBashユーザーは、 [rcalorasのbash_preexecフレームワーク](https://github.com/rcaloras/bash-preexec)を使用できます。 `eval $(starship init $0)` を実行する前に、`preexec_functions`、および`precmd_functions`定義するだけで、通常どおり続行します。

### オプション

| オプション                  | デフォルト                         | 説明                                                                                              |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | 実行時間を表示する最短期間（ミリ秒単位）です。                                                                         |
| `show_milliseconds`    | `false`                       | 実行時間の秒に加えてミリ秒を表示します。                                                                            |
| `format`               | `"took [$duration]($style) "` | module のフォーマットです。                                                                               |
| `style`                | `"bold yellow"`               | モジュールのスタイルです。                                                                                   |
| `disabled`             | `false`                       | `cmd_duration`モジュールを無効にします。                                                                     |
| `show_notifications`   | `false`                       | コマンドが完了したらデスクトップ通知を表示します。                                                                       |
| `min_time_to_notify`   | `45_000`                      | 通知を持続する最短期間(ミリ秒単位)                                                                              |
| `notification_timeout` |                               | 通知を表示する期間 (ミリ秒単位) です。 もし設定されていない場合、通知のタイムアウトはデーモンによって決定されます。 すべての通知デーモンがこのオプションを受け入れるわけではありません。 |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| duration  | `16m40s` | コマンドの実行時間              |
| style\* |          | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

`conda` モジュールは、`$CONDA_DEFAULT_ENV` が設定されている場合、現在の[Conda](https://docs.conda.io/en/latest/) 環境を表示します。

::: tip

Note: これはconda自身の プロンプト修飾子 を抑制しません。`conda config --set changeps1 False` で実行することができます。

:::

### オプション

| オプション               | デフォルト                                  | 説明                                                                                                               |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | 環境が`conda create -p [path]`で作成された場合、環境パスが切り捨てられるディレクトリ数。 `0`は切り捨てがないことを意味します。  [`directory`](#directory)もご覧ください。 |
| `symbol`            | `"🅒 "`                                 | 環境名の直前に使用されるシンボルです。                                                                                              |
| `style`             | `"bold green"`                         | モジュールのスタイルです。                                                                                                    |
| `format`            | `"via [$symbol$environment]($style) "` | module のフォーマットです。                                                                                                |
| `ignore_base`       | `true`                                 | アクティブになった時、環境`base`を無視します。                                                                                       |
| `disabled`          | `false`                                | `conda`モジュールを無効にします。                                                                                             |

### 変数

| 変数          | 設定例          | 説明                     |
| ----------- | ------------ | ---------------------- |
| environment | `astronauts` | 現在の conda 環境           |
| symbol      |              | オプション `記号` の値をミラーする    |
| style\*   |              | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## コンテナ

`container`モジュールは、コンテナ内の場合、シンボルとコンテナ名を表示します。

### オプション

| オプション      | デフォルト                              | 説明                        |
| ---------- | ---------------------------------- | ------------------------- |
| `symbol`   | `"⬢"`                              | コンテナ内にいる場合、このシンボルが表示されます。 |
| `style`    | `"bold red dimmed"`                | モジュールのスタイルです。             |
| `format`   | `'[$symbol \[$name\]]($style) '` | module のフォーマットです。         |
| `disabled` | `false`                            | `container`モジュールを無効にします。  |

### 変数

| 変数        | 設定例                 | 説明                     |
| --------- | ------------------- | ---------------------- |
| name      | `fedora-toolbox:35` | コンテナ名                  |
| symbol    |                     | オプション `記号` の値をミラーする    |
| style\* |                     | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

`crystal`モジュールは、現在インストールされている[Crystal](https://crystal-lang.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`shard.yml`ファイルが含まれている
- カレントディレクトリに`.cr`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `symbol`            | `"🔮 "`                               | Crystalのバージョンを表示する前に使用される記号です。                         |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Daml

`daml`モジュールは、Damlプロジェクトのルートディレクトリにいるときに、使用している[Daml](https://www.digitalasset.com/developers) SDKバージョンを表示します。 環境変数`DAML_SDK_VERSION`を上書きしない限り、`daml.yaml`ファイルの`sdk-version`が使用されます。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`daml.yaml`ファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"Λ "`                               | Damlの記号を表すフォーマット文字列です。                                 |
| `style`             | `"bold cyan"`                        | モジュールのスタイルです。                                          |
| `detect_extensions` | `[]`                                 | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["daml.yaml"]`                      | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | `daml`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v2.2.0` | `daml`のバージョン           |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[daml]
format = "via [D $version](bold bright-green) "
```

## Dart

`dart`モジュールは、現在インストールされている[Dart](https://dart.dev/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.dart`の拡張子のファイルが含まれている
- カレントディレクトリに`.dart_tool`ディレクトリが含まれている
- カレントディレクトリに`pubspec.yaml`、`pubspec.yml`または`pubspec.lock` が含まれている

### オプション

| オプション               | デフォルト                                             | 説明                                                     |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`              | module のフォーマットです。                                      |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

`deno`モジュールは、現在インストールされている[Deno](https://deno.land/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`deno.json`、`deno.jsonc`、`mod.ts`、`mod.js`、`deps.ts`もしくは`deps.js`が含まれている

### オプション

| オプション               | デフォルト                                                                   | 説明                                                     |
| ------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                    | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                                                             | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🦕 "`                                                                  | Dart のシンボルを表すフォーマット文字列                                 |
| `detect_extensions` | `[]`                                                                    | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["deno.json", "deno.jsonc", "mod.ts", "mod.js", "deps.ts", "deps.js"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                    | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"green bold"`                                                          | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                 | `deno`モジュールを無効化します。                                    |

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

`directory` モジュールは現在のディレクトリへのパスを表示します。親フォルダは3つまでに切り捨てられます。 git リポジトリ内にいる場合は、リポジトリのルートで切り捨てられます。

fish スタイルの pwd オプションを使用している場合、切り捨てられたパスを非表示にする代わりに、オプションで有効にした数値に基づいた各ディレクトリの短縮名が表示されます。

たとえば、`~/Dev/Nix/nixpkgs/pkgs`で、`nixpkgs` がリポジトリルートであり、オプションが `1` に設定されている場合を挙げます。 この場合、`nixpkgs/pkgs` の代わりに、`~/D/N/nixpkgs/pkgs` と表示されます。

### オプション

| オプション               | デフォルト                                                                                                       | 説明                                                     |
| ------------------- | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `truncation_length` | `3`                                                                                                         | 現在のディレクトリを切り捨てる親フォルダーの数です。                             |
| `truncate_to_repo`  | `true`                                                                                                      | 現在いるgitリポジトリのルートに切り捨てるかどうかです。                          |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "`                                                          | module のフォーマットです。                                      |
| `style`             | `"bold cyan"`                                                                                               | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                                                     | `directory`モジュールを無効にします。                               |
| `read_only`         | `"🔒"`                                                                                                       | このシンボルが表示されている時、現在のディレクトリは読み取り専用です。                    |
| `read_only_style`   | `"red"`                                                                                                     | 読み取り専用シンボルのスタイルです。                                     |
| `truncation_symbol` | `""`                                                                                                        | 切り捨てられたパスの接頭辞として付けるシンボルです。 例: "…/"                     |
| `repo_root_style`   |                                                                                                             | gitリポジトリのルートのスタイルです。 デフォルトの値は `style` と同じです。           |
| `repo_root_format`  | `"[$before_root_path]($style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) "` | `repo_root_style` が定義されている場合の git リポジトリのフォーマットです。      |
| `home_symbol`       | `"~"`                                                                                                       | ホームディレクトリを示すシンボルです。                                    |
| `use_os_path_sep`   | `true`                                                                                                      | `/`を使用する代わりに、OS固有のパスの区切り文字を使用します。(例: Windowsの場合`\`) |

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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

<details>
<summary>gitリポジトリは追加の変数があります。</summary>

`/path/to/home/git_repo/src/lib`のパスについて考えます。

| 変数                 | 設定例                   | 説明                     |
| ------------------ | --------------------- | ---------------------- |
| before_root_path | `"/path/to/home/"`    | gitルートディレクトリパスの前のパス    |
| repo_root          | `"git_repo"`          | gitルートディレクトリの名前        |
| path               | `"/src/lib"`          | 残りのパス                  |
| style              | `"black bold dimmed"` | オプション `style` の値をミラーする |
| repo_root_style  | `"underline white"`   | gitルートディレクトリの名前のスタイル   |

</details>

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

`docker_context`モジュールは、`default`に設定されていない場合、または環境変数`DOCKER_MACHINE_NAME`、`DOCKER_HOST`または`DOCKER_CONTEXT`が設定されている場合 (使用中のコンテキストを上書きするため)、現在アクティブな[Docker context](https://docs.docker.com/engine/context/working-with-contexts/)を表示します。

### オプション

| オプション               | デフォルト                              | 説明                                                             |
| ------------------- | ---------------------------------- | -------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "` | module のフォーマットです。                                              |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

`dotnet`モジュールはカレントディレクトリに関係する[.NET Core SDK](https://dotnet.microsoft.com/)のバージョンを表示します。 もし SDKは現在のディレクトリに固定されているのであれば、その固定されたバージョンが表示されます。 それ以外の場合、モジュール SDKの最新のインストールバージョンを示します。

デフォルトでは、このモジュールは現在のディレクトリに以下のファイルが 存在する場合にのみプロンプトで表示されます:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

正しく使用するには、.NET Core SDKもインストールする必要があります。

内部的に、このモジュールは自身のバージョン検知のメカニズムを利用します。 `dotnet --version` を実行するより2倍速く実行できますが、.NET project一般的でないディレクトリlayoutの場合は間違ったバージョンが示されてしまうことがあります。 速度よりも精度が重要な場合は、次の方法でメカニズムを無効にできます。 モジュールオプションで`heuristic = false `を設定します。

このモジュールは、カレントディレクトリに `.csproj` ファイルがある場合、Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>) も表示します。

### オプション

| オプション               | デフォルト                                                                                                   | 説明                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                                                                                             | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `".NET "`                                                                                               | dotnetのバージョンを表示する前に使用される記号です。                          |
| `heuristic`         | `true`                                                                                                  | より高速なバージョン検出を使用して、starshipの動作を維持します。                   |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                                                    | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold blue"`                                                                                           | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                                                 | `dotnet`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例              | 説明                                  |
| --------- | ---------------- | ----------------------------------- |
| version   | `v3.1.201`       | `dotnet sdk` のバージョンです               |
| tfm       | `netstandard2.0` | 現在のプロジェクトが対象としているターゲット フレームワーク モニカー |
| symbol    |                  | オプション `symbol` の値をミラーする             |
| style\* |                  | オプション `style` の値をミラーする              |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

`elixir` モジュールは、現在インストールされている[Elixir](https://elixir-lang.org/)と[Erlang/OTP](https://erlang.org/doc/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`mix.exs`ファイルが含まれている.

### オプション

| オプション               | デフォルト                                                       | 説明                                                     |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | module elixirのフォーマットです。                                |
| `version_format`    | `"v${raw}"`                                                 | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"💧 "`                                                      | Elixir/Erlangのバージョンを表示する前に使用される記号です。                   |
| `detect_extensions` | `[]`                                                        | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["mix.exs"]`                                               | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                        | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold purple"`                                             | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                     | `elixir`モジュールを無効にします。                                  |

### 変数

| 変数          | 設定例     | 説明                     |
| ----------- | ------- | ---------------------- |
| version     | `v1.10` | `elixir`のバージョン         |
| otp_version |         | `elixir`のotpバージョン      |
| symbol      |         | オプション `記号` の値をミラーする    |
| style\*   |         | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

`elm`モジュールは、現在インストールされている[Elm](https://elm-lang.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`elm.json`ファイルが含まれている
- カレントディレクトリに`elm-package.json`ファイルが含まれている
- カレントディレクトリに`.elm-version`ファイルが含まれている
- カレントディレクトリに`elm-stuff`フォルダが含まれている
- カレントディレクトリに`*.elm`ファイルが含まれている

### オプション

| オプション               | デフォルト                                              | 説明                                                     |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`               | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                                        | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🌳 "`                                             | Elmのシンボルを表すフォーマット文字列                                   |
| `detect_extensions` | `["elm"]`                                          | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `["elm-stuff"]`                                    | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"cyan bold"`                                      | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                            | `elm`モジュールを無効にします。                                     |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v0.19.1` | `elm`のバージョン            |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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

::: tip

`.`を使うことで複数の環境変数を表示することができます。 (例を確認してみてください) `variable`が設定されていない場合、このモジュールは`.`以降に書かれている環境変数の値を表示します。

例: 次の設定ではUSER環境変数を表示します。

```toml
# ~/.config/starship.toml

[env_var.USER]
default = "unknown user"
```

:::

### オプション

| オプション      | デフォルト                          | 説明                                    |
| ---------- | ------------------------------ | ------------------------------------- |
| `symbol`   | `""`                           | 環境変数を表示する前に使用される記号です。                 |
| `variable` |                                | 表示される環境変数です。                          |
| `default`  |                                | 上のvariableが定義されていない場合に表示されるデフォルトの値です。 |
| `format`   | `"with [$env_value]($style) "` | module のフォーマットです。                     |
| `disabled` | `false`                        | `env_var`モジュールを無効にします。                |

### 変数

| 変数        | 設定例                                         | 説明                     |
| --------- | ------------------------------------------- | ---------------------- |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | オプション`variable`の値      |
| symbol    |                                             | オプション `記号` の値をミラーする    |
| style\* | `black bold dimmed`                         | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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

`erlang`モジュールは、現在インストールされている[Erlang/OTP](https://erlang.org/doc/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`rebar.config`ファイルが含まれている.
- カレントディレクトリに`erlang.mk`ファイルが含まれている.

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `" "`                               | Erlangのバージョンを表示する前に使用される記号です。                          |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                          |
| `detect_extensions` | `[]`                                 | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | `erlang`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v22.1.3` | `erlang` のバージョン        |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Fill

`fill` モジュールは行の余分なスペースを記号で埋めます。 一行に複数の`fill`モジュールが存在する場合、それらはスペースを均等に分割します。 これは、他のモジュールの位置合わせに便利です。

### オプション

| オプション      | デフォルト          | 説明                  |
| ---------- | -------------- | ------------------- |
| `symbol`   | `"."`          | 行を埋めるために使う記号        |
| `style`    | `"bold black"` | モジュールのスタイルです。       |
| `disabled` | `false`        | `fill`モジュールを無効にします。 |

### 設定例

```toml
# ~/.config/starship.toml
format = "AA $fill BB $fill CC"

[fill]
symbol = "-"
style = "bold green"
```

このような出力になります:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Google Cloud (`gcloud`)

`gcloud` モジュールは、 [`gcloud`](https://cloud.google.com/sdk/gcloud) CLIの現在の設定が表示されます。 これは `~/.config/gcloud/active_config` ファイルと `~/.config/gcloud/configurations/config_{CONFIG NAME}` ファイルと `CLOUDSDK_CONFIG` 環境変数に基づきます。

### オプション

| オプション             | デフォルト                                                      | 説明                            |
| ----------------- | ---------------------------------------------------------- | ----------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | module のフォーマットです。             |
| `symbol`          | `"☁️ "`                                                    | 現在のGCPプロファイルを表示する前に表示される記号です。 |
| `region_aliases`  | `{}`                                                       | GCP名に加えて表示するリージョンのエイリアスです。    |
| `project_aliases` | `{}`                                                       | GCP名に加えて表示するプロジェクトのエイリアスです。   |
| `style`           | `"bold blue"`                                              | モジュールのスタイルです。                 |
| `disabled`        | `false`                                                    | `gcloud`モジュールを無効にします。         |

### 変数

| 変数        | 設定例           | 説明                                              |
| --------- | ------------- | ----------------------------------------------- |
| region    | `us-central1` | 現在のGCPリージョン                                     |
| account   | `foo`         | 現在のGCPプロファイル                                    |
| domain    | `example.com` | 現在のGCPプロファイルのドメイン                               |
| project   |               | 現在のGCPプロジェクト                                    |
| active    | `default`     | `~/.config/gcloud/active_config` に書かれたアクティブな設定名 |
| symbol    |               | オプション `記号` の値をミラーする                             |
| style\* |               | オプション `style` の値をミラーする                          |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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

#### アカウントとエイリアスされたプロジェクトを表示

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = "vlpn"
```

## Git Branch

`git_branch`モジュールは、現在のディレクトリにあるリポジトリのアクティブなブランチを表示します。

### オプション

| オプション                | デフォルト                                             | 説明                                                             |
| -------------------- | ------------------------------------------------- | -------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | ローカルブランチ名と等しい場合でも、リモート追跡ブランチ名を表示します。                           |
| `format`             | `"on [$symbol$branch(:$remote_branch)]($style) "` | module のフォーマットです。 現在のブランチ名を参照するには、`"$branch"`を使用します。           |
| `symbol`             | `" "`                                            | gitブランチのシンボルを表すフォーマット文字列。                                      |
| `style`              | `"bold purple"`                                   | モジュールのスタイルです。                                                  |
| `truncation_length`  | `2^63 - 1`                                        | Truncates a git branch to `N` graphemes.                       |
| `truncation_symbol`  | `"…"`                                             | ブランチ名切り捨てられていることを示すための記号です。 `""`で記号なしにできます。                    |
| `only_attached`      | `false`                                           | Only show the branch name when not in a detached `HEAD` state. |
| `ignore_branches`    | `[]`                                              | 表示しない名前のリスト。 "master"や"main"に有用。                               |
| `disabled`           | `false`                                           | `git_branch`モジュールを無効にします。                                      |

### 変数

| 変数            | 設定例      | 説明                                                         |
| ------------- | -------- | ---------------------------------------------------------- |
| branch        | `master` | 現在のブランチがない場合は、現在のブランチ名は`HEAD`に戻ります(例: git detached `HEAD`) |
| remote_name   | `origin` | リモート名                                                      |
| remote_branch | `master` | `remote_name`で追跡されたブランチ名                                   |
| symbol        |          | オプション `記号` の値をミラーする                                        |
| style\*     |          | オプション `style` の値をミラーする                                     |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
ignore_branches = ["master", "main"]
```

## Git コミット

`git_commit` モジュールは、カレントディレクトリのリポジトリの現在のコミットハッシュとタグ (もしあれば) を表示します。

### オプション

| オプション                | デフォルト                          | 説明                                                                                   |
| -------------------- | ------------------------------ | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                            | 表示されるgitコミットハッシュの長さです。                                                               |
| `format`             | `'[\($hash$tag\)]($style) '` | module のフォーマットです。                                                                    |
| `style`              | `"bold green"`                 | モジュールのスタイルです。                                                                        |
| `only_detached`      | `true`                         | detached `HEAD` 状態のときのみ git コミットハッシュを表示する                                            |
| `tag_disabled`       | `true`                         | `git_commit` モジュールのタグ情報の表示を無効にする。                                                    |
| `tag_max_candidates` | `0`                            | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `" 🏷 "`                        | 表示される情報の前に追加されるタグシンボル                                                                |
| `disabled`           | `false`                        | `git_commit` モジュールを無効にします。                                                           |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| hash      | `b703eb3` | 現在の git コミットハッシュ       |
| style\* |           | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

`git_state`モジュールはgitディレクトリの進行状態を表します。 (例: _REBASING_, _BISECTING_, その他) 進行状況の情報がある場合は (例:REBASING 3/10)、その情報も表示されます。

### オプション

| オプション          | デフォルト                                                           | 説明                                                       |
| -------------- | --------------------------------------------------------------- | -------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | `rebase`進行中に表示されるフォーマット文字列です。                            |
| `merge`        | `"MERGING"`                                                     | `merge`進行中に表示されるフォーマット文字列です。                             |
| `revert`       | `"REVERTING"`                                                   | `revert`進行中に表示されるフォーマット文字列です。                            |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | `cherry-pick`進行中に表示されるフォーマット文字列です。                       |
| `bisect`       | `"BISECTING"`                                                   | `bisect`進行中に表示されるフォーマット文字列です。                            |
| `am`           | `"AM"`                                                          | `apply-mailbox` (`git am`) 進行中に表示されるフォーマット文字列です。         |
| `am_or_rebase` | `"AM/REBASE"`                                                   | あいまいな`apply-mailbox`または`rebase`が進行中のときに表示されるフォーマット文字列です。 |
| `style`        | `"bold yellow"`                                                 | モジュールのスタイルです。                                            |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | module のフォーマットです。                                        |
| `disabled`     | `false`                                                         | `git_state`モジュールを無効にします。                                 |

### 変数

| 変数               | 設定例        | 説明                     |
| ---------------- | ---------- | ---------------------- |
| state            | `REBASING` | 現在のリポジトリの状態            |
| progress_current | `1`        | 現在の進行状態                |
| progress_total   | `2`        | 全体の進行状態                |
| style\*        |            | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション                | デフォルト                                                        | 説明                                    |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `"bold green"`                                               | The style for the added count.        |
| `deleted_style`      | `"bold red"`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `"([+$added]($added_style) )([-$deleted]($deleted_style) )"` | module のフォーマットです。                     |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.    |

### 変数

| 変数                | 設定例 | 説明                                          |
| ----------------- | --- | ------------------------------------------- |
| added             | `1` | The current number of added lines           |
| deleted           | `2` | The current number of deleted lines         |
| added_style\*   |     | Mirrors the value of option `added_style`   |
| deleted_style\* |     | Mirrors the value of option `deleted_style` |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = "bold blue"
format = "[+$added]($added_style)/[-$deleted]($deleted_style) "
```

## Git Status

`git_status` モジュールは、カレントディレクトリのリポジトリの状態を表すシンボルを表示します。

::: tip

WSL環境のWindowsディレクトリ(例: `/mnt/c/`以下) では、Git Statusモジュールは動作が非常に遅いです。 モジュールを無効にするか、`windows_starship`オプションを使用することで、WindowsネイティブのStarshipを使用し、対象の`git_status`を計算できます。

:::

### オプション

| オプション               | デフォルト                                           | 説明                                                                     |
| ------------------- | ----------------------------------------------- | ---------------------------------------------------------------------- |
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | `git_status` のデフォルトフォーマット                                              |
| `conflicted`        | `"="`                                           | このブランチにはマージの競合があります。                                                   |
| `ahead`             | `"⇡"`                                           | `ahead`のフォーマット                                                         |
| `behind`            | `"⇣"`                                           | `behind`のフォーマット                                                        |
| `diverged`          | `"⇕"`                                           | `diverged`のフォーマット                                                      |
| `up_to_date`        | `""`                                            | `up_to_date`のフォーマット                                                    |
| `untracked`         | `"?"`                                           | `untracked`のフォーマット                                                     |
| `stashed`           | `"$"`                                           | `stashed`のフォーマット                                                       |
| `modified`          | `"!"`                                           | `modified`のフォーマット                                                      |
| `staged`            | `"+"`                                           | `staged`のフォーマット                                                        |
| `renamed`           | `"»"`                                           | `renamed`のフォーマット                                                       |
| `deleted`           | `"✘"`                                           | `deleted`のフォーマット                                                       |
| `style`             | `"bold red"`                                    | モジュールのスタイルです。                                                          |
| `ignore_submodules` | `false`                                         | サブモジュールの変更を無視します。                                                      |
| `disabled`          | `false`                                         | `git_status`モジュールを無効にします。                                              |
| `windows_starship`  |                                                 | WSLでWindowsディレクトリの`git_status`で使用するWindows Starshipの実行ファイルのLinux上でのパス。 |

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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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
untracked = "🤷"
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

Use Windows Starship executable on Windows paths in WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = "/mnt/c/Users/username/scoop/apps/starship/current/starship.exe"
```

## Go

`golang`モジュールは、現在インストールされている[Go](https://golang.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`go.mod`ファイルが含まれている
- カレントディレクトリに`go.sum`ファイルが含まれている
- カレントディレクトリに`go.work`ファイルが含まれている
- カレントディレクトリに`glide.yaml`ファイルが含まれている
- カレントディレクトリに`Gopkg.yml`ファイルが含まれている
- カレントディレクトリに`Gopkg.lock`ファイルが含まれている
- カレントディレクトリに`.go-version`ファイルが含まれている
- カレントディレクトリに`Godeps`ファイルが含まれている
- カレントディレクトリに`.go`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                                                                     | 説明                                                     |
| ------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                                      | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                                                                               | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🐹 "`                                                                                    | A format string representing the symbol of Go.         |
| `detect_extensions` | `["go"]`                                                                                  | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["go.mod", "go.sum", "go.work", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `["Godeps"]`                                                                              | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold cyan"`                                                                             | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                                   | `golang`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v1.12.1` | The version of `go`    |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment. The module will be shown when inside a guix-shell environment.

### オプション

| オプション      | デフォルト                      | 説明                                                     |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `'via [$symbol]($style) '` | module のフォーマットです。                                      |
| `symbol`   | `"🐃 "`                     | A format string representing the symbol of guix-shell. |
| `style`    | `"yellow bold"`            | モジュールのスタイルです。                                          |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### 変数

| 変数        | 設定例 | 説明                     |
| --------- | --- | ---------------------- |
| symbol    |     | オプション `記号` の値をミラーする    |
| style\* |     | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`stack.yaml`ファイルが含まれている
- カレントディレクトリに`.hs`、`.cabal`または`.hs-boot`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                 |
| ------------------- | ------------------------------------ | -------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                  |
| `symbol`            | `"λ "`                               | A format string representing the symbol of Haskell |
| `detect_extensions` | `["hs", "cabal", "hs-boot"]`         | どの拡張子がこのモジュールをアクティブにするか                            |
| `detect_files`      | `["stack.yaml", "cabal.project"]`    | どのファイル名がこのモジュールをアクティブにするか                          |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                          |
| `style`             | `"bold purple"`                      | モジュールのスタイルです。                                      |
| `disabled`          | `false`                              | Disables the `haskell` module.                     |

### 変数

| 変数             | 設定例         | 説明                                                                                      |
| -------------- | ----------- | --------------------------------------------------------------------------------------- |
| version        |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot       | `lts-18.12` | Currently selected Stack snapshot                                                       |
| ghc\_version | `9.2.1`     | Currently installed GHC version                                                         |
| symbol         |             | オプション `記号` の値をミラーする                                                                     |
| style\*      |             | オプション `style` の値をミラーする                                                                  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`helmfile.yaml`ファイルが含まれている
- カレントディレクトリに`Chart.yaml`ファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## ホスト名

The `hostname` module shows the system hostname.

### オプション

| オプション        | デフォルト                                  | 説明                                                                                                                                   |
| ------------ | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only`   | `true`                                 | Only show hostname when connected to an SSH session.                                                                                 |
| `ssh_symbol` | `"🌐 "`                                 | A format string representing the symbol when connected to SSH session.                                                               |
| `trim_at`    | `"."`                                  | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `format`     | `"[$ssh_symbol$hostname]($style) in "` | module のフォーマットです。                                                                                                                    |
| `style`      | `"bold dimmed green"`                  | モジュールのスタイルです。                                                                                                                        |
| `disabled`   | `false`                                | Disables the `hostname` module.                                                                                                      |

### 変数

| 変数         | 設定例        | 説明                                                    |
| ---------- | ---------- | ----------------------------------------------------- |
| hostname   | `computer` | The hostname of the computer                          |
| style\*  |            | オプション `style` の値をミラーする                                |
| ssh_symbol | `"🌏 "`     | The symbol to represent when connected to SSH session |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = "[$ssh_symbol](bold blue) on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, or `build.boot` file
- カレントディレクトリに拡張子が`.java`、`.class`、`.gradle`、`.jar`、`.clj`または`.cljc`のファイルが含まれている

### オプション

| オプション               | デフォルト                                                                                                    | 説明                                                     |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                 | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                                                                                              | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                     | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", "deps.edn", "project.clj", "build.boot"]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                                                     | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `"☕ "`                                                                                                   | A format string representing the symbol of Java        |
| `style`             | `"red dimmed"`                                                                                           | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                                                  | Disables the `java` module.                            |

### 変数

| 変数        | 設定例   | 説明                     |
| --------- | ----- | ---------------------- |
| version   | `v14` | The version of `java`  |
| symbol    |       | オプション `記号` の値をミラーする    |
| style\* |       | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there are at least 2 jobs, or more than the `number_threshold` config value, if it exists. The module will show a symbol if there is at least 1 job, or more than the `symbol_threshold` config value, if it exists. You can set both values to 0 in order to _always_ show the symbol and number of jobs, even if there are 0 jobs running.

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
| `threshold`*       | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `"[$symbol$number]($style) "` | module のフォーマットです。                                                        |
| `symbol`           | `"✦"`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `"bold blue"`                 | モジュールのスタイルです。                                                            |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### 変数

| 変数        | 設定例 | 説明                     |
| --------- | --- | ---------------------- |
| number    | `1` | The number of jobs     |
| symbol    |     | オプション `記号` の値をミラーする    |
| style\* |     | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `["jl"]`                             | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia.      |
| `style`             | `"bold purple"`                      | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `julia` module.                           |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v1.4.0` | The version of `julia` |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.kt`もしくは`.kts`ファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                                            |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                                             |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。                        |
| `detect_extensions` | `["kt", "kts"]`                      | どの拡張子がこのモジュールをアクティブにするか                                                       |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                                                     |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                                                     |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-context --namespace astronaut`. Similarly the user and cluster can be set with `kubectl config set-context starship-context --user starship-user` and `kubectl config set-context starship-context --cluster starship-cluster`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

When the module is enabled it will always be active, unless any of `detect_extensions`, `detect_files` or `detect_folders` have been st in which case the module will only be active in directories that match those conditions.

:::

### オプション

| オプション               | デフォルト                                                | 説明                                                                    |
| ------------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`            | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | module のフォーマットです。                                                     |
| `style`             | `"cyan bold"`                                        | モジュールのスタイルです。                                                         |
| `context_aliases`   | `{}`                                                 | Table of context aliases to display.                                  |
| `user_aliases`      | `{}`                                                 | Table of user aliases to display.                                     |
| `detect_extensions` | `[]`                                                 | どの拡張子がこのモジュールをアクティブにするか                                               |
| `detect_files`      | `[]`                                                 | どのファイル名がこのモジュールをアクティブにするか                                             |
| `detect_folders`    | `[]`                                                 | どのフォルダーがこのモジュールをアクティブにするか                                             |
| `disabled`          | `true`                                               | Disables the `kubernetes` module.                                     |

### 変数

| 変数        | 設定例                  | 説明                                       |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbol    |                      | オプション `記号` の値をミラーする                      |
| style\* |                      | オプション `style` の値をミラーする                   |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
".*/openshift-cluster/.*" = "openshift"
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
[kubernetes.user_aliases]
"dev.local.cluster.k8s" = "dev"
"root/.*" = "root"
```

Only show the module in directories that contain a `k8s` file.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Regex Matching

Additional to simple aliasing, `context_aliases` and `user_aliases` also supports extended matching and renaming using regular expressions.

The regular expression must match on the entire kube context, capture groups can be referenced using `$name` and `$N` in the replacement. This is more explained in the [regex crate](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace) documentation.

Long and automatically generated cluster names can be identified and shortened using regular expressions:

```toml
[kubernetes.context_aliases]
# OpenShift contexts carry the namespace and user in the kube context: `namespace/name/user`:
".*/openshift-cluster/.*" = "openshift"
# Or better, to rename every OpenShift cluster at once:
".*/(?P<var_cluster>[\\w-]+)/.*" = "$var_cluster"

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
```

## Line Break

The `line_break` module separates the prompt into two lines.

### オプション

| オプション      | デフォルト   | 説明                                                                 |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### 設定例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Local IP

The `localip` module shows the IPv4 address of the primary network interface.

### オプション

| オプション      | デフォルト                     | 説明                                                     |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `"[$localipv4]($style) "` | module のフォーマットです。                                      |
| `style`    | `"bold yellow"`           | モジュールのスタイルです。                                          |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### 変数

| 変数        | 設定例          | 説明                                |
| --------- | ------------ | --------------------------------- |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address |
| style\* |              | オプション `style` の値をミラーする            |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = "@[$localipv4](bold red) "
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.lua-version`ファイルが含まれている
- カレントディレクトリに`lua`ディレクトリが含まれている
- カレントディレクトリに`.lua`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                                         |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                                          |
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
| version   | `v5.4.0` | The version of `lua`   |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション       | デフォルト                                           | 説明                                                       |
| ----------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | module のフォーマットです。                                        |
| `symbol`    | `"🐏"`                                           | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                           | モジュールのスタイルです。                                            |
| `disabled`  | `true`                                          | Disables the `memory_usage` module.                      |

### 変数

| 変数               | 設定例           | 説明                                                                 |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | オプション `記号` の値をミラーする                                                |
| style\*        |               | オプション `style` の値をミラーする                                             |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

### 設定例

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Meson

The `meson` module shows the current Meson developer environment status.

By default the Meson project name is displayed, if `$MESON_DEVENV` is set.

### オプション

| オプション               | デフォルト                              | 説明                                                                        |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                |
| `truncation_symbol` | `"…"`                              | The symbol used to indicate a project name was truncated. `""`で記号なしにできます。 |
| `format`            | `"via [$symbol$project]($style) "` | module のフォーマットです。                                                         |
| `symbol`            | `"⬢ "`                             | The symbol used before displaying the project name.                       |
| `style`             | `"blue bold"`                      | モジュールのスタイルです。                                                             |
| `disabled`          | `false`                            | Disables the `meson` module.                                              |

### 変数

| 変数        | 設定例        | 説明                             |
| --------- | ---------- | ------------------------------ |
| project   | `starship` | The current Meson project name |
| symbol    | `🐏`        | オプション `記号` の値をミラーする            |
| style\* |            | オプション `style` の値をミラーする         |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = "--"
symbol = " "
style = "bold dimmed green"
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### オプション

| オプション               | デフォルト                            | 説明                                                                                           |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | モジュールのスタイルです。                                                                                |
| `format`            | `"on [$symbol$branch]($style) "` | module のフォーマットです。                                                                            |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `"…"`                            | ブランチ名切り捨てられていることを示すための記号です。                                                                  |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### 変数

| 変数        | 設定例      | 説明                          |
| --------- | -------- | --------------------------- |
| branch    | `master` | The active mercurial branch |
| symbol    |          | オプション `記号` の値をミラーする         |
| style\* |          | オプション `style` の値をミラーする      |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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
- カレントディレクトリに拡張子が`.nim`のファイルが含まれている
- カレントディレクトリに拡張子が`.nims`のファイルが含まれている
- カレントディレクトリに拡張子が`.nimble`のファイルが含まれている

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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. The module will be shown when inside a nix-shell environment.

### オプション

| オプション        | デフォルト                                          | 説明                                                    |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | module のフォーマットです。                                     |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | モジュールのスタイルです。                                         |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Disables the `nix_shell` module.                      |

### 変数

| 変数        | 設定例     | 説明                         |
| --------- | ------- | -------------------------- |
| state     | `pure`  | The state of the nix-shell |
| name      | `lorri` | The name of the nix-shell  |
| symbol    |         | オプション `記号` の値をミラーする        |
| style\* |         | オプション `style` の値をミラーする     |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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
- カレントディレクトリに`.node-version`ファイルが含まれている
- カレントディレクトリに`.nvmrc`ファイルが含まれている
- カレントディレクトリに`node_modules`ディレクトリが含まれている
- カレントディレクトリに拡張子が`.js`、`.mjs`または`.cjs`のファイルが含まれている
- カレントディレクトリに拡張子が`.ts`、`.mts`または`.cts`のファイルが含まれている

### オプション

| オプション               | デフォルト                                      | 説明                                                                                                    |
| ------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`       | module のフォーマットです。                                                                                     |
| `version_format`    | `"v${raw}"`                                | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。                                                |
| `symbol`            | `" "`                                     | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts", "mts", "cts"]` | どの拡張子がこのモジュールをアクティブにするか                                                                               |
| `detect_files`      | `["package.json", ".node-version"]`        | どのファイル名がこのモジュールをアクティブにするか                                                                             |
| `detect_folders`    | `["node_modules"]`                         | どのフォルダーがこのモジュールをアクティブにするか                                                                             |
| `style`             | `"bold green"`                             | モジュールのスタイルです。                                                                                         |
| `disabled`          | `false`                                    | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                                 | The style for the module when an engines property in package.json does not match the Node.js version. |

### 変数

| 変数        | 設定例        | 説明                     |
| --------- | ---------- | ---------------------- |
| version   | `v13.12.0` | The version of `node`  |
| symbol    |            | オプション `記号` の値をミラーする    |
| style\* |            | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに拡張子`.opam`のファイルまたは`_opam`ディレクトリが含まれている
- カレントディレクトリに`esy.lock`ディレクトリが含まれている
- カレントディレクトリに`dune`または`dune-project`ファイルが含まれている
- カレントディレクトリに`jbuild`または`jbuild-ignore`ファイルが含まれている
- カレントディレクトリに`.merlin`ファイルが含まれている
- カレントディレクトリに拡張子が`.ml`、`.mli`、`.re`または`.rei`のファイルが含まれている

### オプション

| オプション                     | デフォルト                                                                      | 説明                                                      |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | モジュールのフォーマット文字列。                                        |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool. By default the module will be shown if the current directory contains a `.rego` file.

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🪖  "`                              | A format string representing the symbol of OPA.        |
| `detect_extensions` | `["rego"]`                           | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold blue"`                        | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `opa` module.                             |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| version   | `v0.44.0` | The version of `opa`   |
| symbol    |           | オプション `記号` の値をミラーする    |
| style\* |           | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[opa]
format = "via [⛑️  $version](bold red) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### オプション

| オプション      | デフォルト                                           | 説明                                                             |
| ---------- | ----------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | module のフォーマットです。                                              |
| `symbol`   | `"☁️ "`                                         | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                 | モジュールのスタイルです。                                                  |
| `disabled` | `false`                                         | Disables the `openstack` module.                               |

### 変数

| 変数        | 設定例    | 説明                            |
| --------- | ------ | ----------------------------- |
| cloud     | `corp` | The current OpenStack cloud   |
| project   | `dev`  | The current OpenStack project |
| symbol    |        | オプション `記号` の値をミラーする           |
| style\* |        | オプション `style` の値をミラーする        |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from a [PEP 621](https://peps.python.org/pep-0621/) compliant `pyproject.toml` or a `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present in the current directory
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present in the current directory
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present in the current directory
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present in the current directory
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present in the current directory
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present in the current directory
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present in the current directory
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present in the current directory
- [**SBT**](https://scala-sbt.org) - The `sbt` package version is extracted from the `build.sbt` present in the current directory
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) - The `dart` package version is extracted from the `pubspec.yaml` present in the current directory

> ⚠️ 表示されるバージョンは、パッケージマネージャーではなく、ソースコードが現在のディレクトリにあるパッケージのバージョンです。

### オプション

| オプション             | デフォルト                             | 説明                                                         |
| ----------------- | --------------------------------- | ---------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | module のフォーマットです。                                          |
| `symbol`          | `"📦 "`                            | The symbol used before displaying the version the package. |
| `version_format`  | `"v${raw}"`                       | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。     |
| `style`           | `"bold 208"`                      | モジュールのスタイルです。                                              |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.  |
| `disabled`        | `false`                           | Disables the `package` module.                             |

### 変数

| 変数        | 設定例      | 説明                          |
| --------- | -------- | --------------------------- |
| version   | `v1.0.0` | The version of your package |
| symbol    |          | オプション `記号` の値をミラーする         |
| style\* |          | オプション `style` の値をミラーする      |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Makefile.PL`または`Build.PL`ファイルが含まれている
- カレントディレクトリに`cpanfile`または`cpanfile.snapshot`ファイルが含まれている
- カレントディレクトリに`META.json`または`META.yml`ファイルが含まれている
- カレントディレクトリに`.perl-version`ファイルが含まれている
- カレントディレクトリに拡張子が`.pl`、`.pm`または`.pod`のファイルが含まれている

### オプション

| オプション               | デフォルト                                                                                                    | 説明                                                     |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | モジュールのフォーマット文字列。                                       |
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
- カレントディレクトリに`.php-version`ファイルが含まれている
- カレントディレクトリに拡張子が`.php`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🐘 "`                               | The symbol used before displaying the version of PHP.  |
| `detect_extensions` | `["php"]`                            | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["composer.json", ".php-version"]`  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"147 bold"`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `php` module.                             |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v7.3.8` | The version of `php`   |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

::: tip

By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms). If you still want to enable it, [follow the example shown below](#with-pulumi-version).

:::

デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Pulumi.yaml`または`Pulumi.yml`ファイルが含まれている
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### オプション

| オプション            | デフォルト                                        | 説明                                                             |
| ---------------- | -------------------------------------------- | -------------------------------------------------------------- |
| `format`         | `"via [$symbol($username@)$stack]($style) "` | モジュールのフォーマット文字列。                                               |
| `version_format` | `"v${raw}"`                                  | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。         |
| `symbol`         | `" "`                                       | A format string shown before the Pulumi stack.                 |
| `style`          | `"bold 5"`                                   | モジュールのスタイルです。                                                  |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories. |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                  |

### 変数

| 変数        | 設定例        | 説明                          |
| --------- | ---------- | --------------------------- |
| version   | `v0.12.24` | The version of `pulumi`     |
| stack     | `dev`      | The current Pulumi stack    |
| username  | `alice`    | The current Pulumi username |
| symbol    |            | オプション `記号` の値をミラーする         |
| style\* |            | オプション `style` の値をミラーする      |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

#### With Pulumi Version

```toml
# ~/.config/starship.toml

[pulumi]
format = "[🛥 ($version )$stack]($style) "
```

#### Without Pulumi version

```toml
# ~/.config/starship.toml
[pulumi]
symbol = "🛥 "
format = "[$symbol$stack]($style) "
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`spago.dhall`ファイルが含まれている
- カレントディレクトリに拡張子が`.purs`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                           |
| ------------------- | ------------------------------------ | ------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                            |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

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
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | module のフォーマットです。                                                                      |
| `version_format`     | `"v${raw}"`                                                                                                  | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。                                 |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | モジュールのスタイルです。                                                                          |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                        |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3", "python2"]`                                                                           | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `["py"]`                                                                                                     | どの拡張子がこのモジュールをアクティブにするか                                                                |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | どのファイル名がこのモジュールをアクティブにするか                                                              |
| `detect_folders`     | `[]`                                                                                                         | どのフォルダーがこのモジュールをアクティブにするか                                                              |
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

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). The module will be shown if any of the following conditions are met:

- カレントディレクトリに拡張子が`.R`のファイルが含まれている
- カレントディレクトリに拡張子が`.Rd`のファイルが含まれている
- カレントディレクトリに拡張子が`.Rmd`のファイルが含まれている
- カレントディレクトリに拡張子が`.Rproj`のファイルが含まれている
- カレントディレクトリに拡張子が`.Rsx`のファイルが含まれている
- カレントディレクトリに`.Rprofile`ファイルが含まれている
- カレントディレクトリに`.Rproj.user`フォルダが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
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

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### オプション

| オプション               | デフォルト                                            | 説明                                                     |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version-$vm_version )]($style)"` | モジュールのフォーマット文字列。                                       |
| `version_format`    | `"v${raw}"`                                      | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🦋 "`                                           | The symbol used before displaying the version of Raku  |
| `detect_extensions` | `["p6", "pm6", "pod6", "raku", "rakumod"]`       | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["META6.json"]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                             | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold 149"`                                     | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                          | Disables the `raku` module.                            |

### 変数

| 変数         | 設定例    | 説明                                   |
| ---------- | ------ | ------------------------------------ |
| version    | `v6.d` | The version of `raku`                |
| vm_version | `moar` | The version of VM `raku` is built on |
| symbol     |        | オプション `記号` の値をミラーする                  |
| style\*  |        | オプション `style` の値をミラーする               |

### 設定例

```toml
# ~/.config/starship.toml

[raku]
format = "via [🦪 $version]($style) "
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). 次の条件のいずれかが満たされると、モジュールが表示されます:

- カレントディレクトリに拡張子が`.red` or `.reds`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). 次の条件のいずれかが満たされると、モジュールが表示されます:

- カレントディレクトリに`Gemfile`ファイルが含まれている
- カレントディレクトリに `.ruby-version` ファイルが含まれている
- カレントディレクトリに`.rb`の拡張子のファイルが含まれている
- 環境変数に`RUBY_VERSION`または`RBENV_VERSION`が設定されている

Starship gets the current Ruby version by running `ruby -v`.

### オプション

| オプション               | デフォルト                                | 説明                                                      |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。  |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.        |
| `detect_extensions` | `["rb"]`                             | どの拡張子がこのモジュールをアクティブにするか                                 |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | どのファイル名がこのモジュールをアクティブにするか                               |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                               |
| `detect_variables`  | `["RUBY_VERSION", "RBENV_VERSION"]`  | Which environment variables should trigger this module. |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                           |
| `disabled`          | `false`                              | Disables the `ruby` module.                             |

### 変数

| 変数        | 設定例      | 説明                     |
| --------- | -------- | ---------------------- |
| version   | `v2.5.1` | The version of `ruby`  |
| symbol    |          | オプション `記号` の値をミラーする    |
| style\* |          | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). 次の条件のいずれかが満たされると、モジュールが表示されます:

- カレントディレクトリに`Cargo.toml`ファイルが含まれている
- カレントディレクトリに`.rs`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust        |
| `detect_extensions` | `["rs"]`                             | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `["Cargo.toml"]`                     | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold red"`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `rust` module.                            |

### 変数

| 変数        | 設定例               | 説明                                           |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbol    |                   | オプション `記号` の値をミラーする                          |
| style\* |                   | オプション `style` の値をミラーする                       |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`build.sbt`、`.scalaenv`または`.sbtenv`ファイルが含まれている
- カレントディレクトリに拡張子が`.scala`または`.sbt`のファイルが含まれている
- カレントディレクトリに`.metals`ディレクトリが含まれている

### オプション

| オプション               | デフォルト                                    | 説明                                                     |
| ------------------- | ---------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [${symbol}(${version} )]($style)"` | module のフォーマットです。                                      |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション                  | デフォルト                     | 説明                                                           |
| ---------------------- | ------------------------- | ------------------------------------------------------------ |
| `bash_indicator`       | `"bsh"`                   | A format string used to represent bash.                      |
| `fish_indicator`       | `"fsh"`                   | A format string used to represent fish.                      |
| `zsh_indicator`        | `"zsh"`                   | A format string used to represent zsh.                       |
| `powershell_indicator` | `"psh"`                   | A format string used to represent powershell.                |
| `ion_indicator`        | `"ion"`                   | A format string used to represent ion.                       |
| `elvish_indicator`     | `"esh"`                   | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `"tsh"`                   | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `"xsh"`                   | A format string used to represent xonsh.                     |
| `cmd_indicator`        | `"cmd"`                   | A format string used to represent cmd.                       |
| `nu_indicator`         | `"nu"`                    | A format string used to represent nu.                        |
| `unknown_indicator`    | `""`                      | The default value to be displayed when the shell is unknown. |
| `format`               | `"[$indicator]($style) "` | module のフォーマットです。                                            |
| `style`                | `"white bold"`            | モジュールのスタイルです。                                                |
| `disabled`             | `true`                    | Disables the `shell` module.                                 |

### 変数

| 変数        | デフォルト | 説明                                                         |
| --------- | ----- | ---------------------------------------------------------- |
| indicator |       | Mirrors the value of `indicator` for currently used shell. |
| style\* |       | Mirrors the value of option `style`.                       |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "mystery shell"
style = "cyan bold"
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### オプション

| オプション       | デフォルト                        | 説明                                                            |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | module のフォーマットです。                                             |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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
| `format`   | `'[$symbol\[$env\]]($style) '` | module のフォーマットです。                                |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | モジュールのスタイルです。                                    |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### 変数

| 変数        | 設定例          | 説明                            |
| --------- | ------------ | ----------------------------- |
| env       | `centos.img` | The current Singularity image |
| symbol    |              | オプション `記号` の値をミラーする           |
| style\* |              | オプション `style` の値をミラーする        |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### オプション

| オプション               | デフォルト                                  | 説明                                                                                                                             |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to. `0`は切り捨てがないことを意味します。  [`directory`](#directory)もご覧ください。 |
| `symbol`            | `"🅢  "`                                | 環境名の直前に使用されるシンボルです。                                                                                                            |
| `style`             | `"bold blue"`                          | モジュールのスタイルです。                                                                                                                  |
| `format`            | `"via [$symbol$environment]($style) "` | module のフォーマットです。                                                                                                              |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                   |

### 変数

| 変数          | 設定例          | 説明                            |
| ----------- | ------------ | ----------------------------- |
| environment | `astronauts` | The current spack environment |
| symbol      |              | オプション `記号` の値をミラーする           |
| style\*   |              | オプション `style` の値をミラーする        |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[spack]
format = "[$symbol$environment](dimmed blue) "
```

## Status

The `status` module displays the exit code of the previous command. If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`. The status code will cast to a signed 32-bit integer.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション                       | デフォルト                                                                              | 説明                                                                    |
| --------------------------- | ---------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `format`                    | `"[$symbol$status]($style) "`                                                      | The format of the module                                              |
| `symbol`                    | `"❌"`                                                                              | The symbol displayed on program error                                 |
| `success_symbol`            | `""`                                                                               | The symbol displayed on program success                               |
| `not_executable_symbol`     | `"🚫"`                                                                              | The symbol displayed when file isn't executable                       |
| `not_found_symbol`          | `"🔍"`                                                                              | The symbol displayed when the command can't be found                  |
| `sigint_symbol`             | `"🧱"`                                                                              | The symbol displayed on SIGINT (Ctrl + c)                             |
| `signal_symbol`             | `"⚡"`                                                                              | The symbol displayed on any signal                                    |
| `style`                     | `"bold red"`                                                                       | モジュールのスタイルです。                                                         |
| `recognize_signal_code`     | `true`                                                                             | Enable signal mapping from exit code                                  |
| `map_symbol`                | `false`                                                                            | Enable symbols mapping from exit code                                 |
| `pipestatus`                | `false`                                                                            | Enable pipestatus reporting                                           |
| `pipestatus_separator`      | <code>&vert;</code>                                                          | The symbol used to separate pipestatus segments (supports formatting) |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style)'` | The format of the module when the command is a pipeline               |
| `pipestatus_segment_format` |                                                                                    | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                             | Disables the `status` module.                                         |

### 変数

| 変数             | 設定例     | 説明                                                                                          |
| -------------- | ------- | ------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                           |
| hex_status     | `0x7F`  | The exit code of the last command in hex                                                    |
| int            | `127`   | The exit code of the last command                                                           |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                         |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                             |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                        |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                |
| pipestatus     |         | Rendering of in pipeline programs's exit codes, this is only available in pipestatus_format |
| symbol         |         | オプション `記号` の値をミラーする                                                                         |
| style\*      |         | オプション `style` の値をミラーする                                                                      |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴 "
success_symbol = "🟢 SUCCESS"
format = '[\[$symbol$common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

The `sudo` module displays if sudo credentials are currently cached. The module will only be shown if credentials are cached.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション           | デフォルト                    | 説明                                                      |
| --------------- | ------------------------ | ------------------------------------------------------- |
| `format`        | `"[as $symbol]($style)"` | The format of the module                                |
| `symbol`        | `"🧙 "`                   | The symbol displayed when credentials are cached        |
| `style`         | `"bold blue"`            | モジュールのスタイルです。                                           |
| `allow_windows` | `false`                  | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                   | Disables the `sudo` module.                             |

### 変数

| 変数        | 設定例 | 説明                     |
| --------- | --- | ---------------------- |
| symbol    |     | オプション `記号` の値をミラーする    |
| style\* |     | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[sudo]
style = "bold green"
symbol = "👩‍💻 "
disabled = false
```

```toml
# On windows
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). 次の条件のいずれかが満たされると、モジュールが表示されます:

- カレントディレクトリに`Package.swift`ファイルが含まれている
- カレントディレクトリに拡張子が`.swift`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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
- 現在のディレクトリに `.tf`, `.tfplan` または `.tfstate` のいずれかの拡張子を持つファイルがある。

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol$workspace]($style) "` | モジュールのフォーマット文字列。                                       |
| `version_format`    | `"v${raw}"`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.  |
| `detect_extensions` | `["tf", "tfplan", "tfstate"]`        | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[".terraform"]`                     | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `"bold 105"`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `terraform` module.                       |

### 変数

| 変数        | 設定例        | 説明                              |
| --------- | ---------- | ------------------------------- |
| version   | `v0.12.24` | The version of `terraform`      |
| workspace | `default`  | The current Terraform workspace |
| symbol    |            | オプション `記号` の値をミラーする             |
| style\* |            | オプション `style` の値をミラーする          |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

#### Terraform バージョン表示あり

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Terraform バージョン表示なし

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## 時刻

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション             | デフォルト                   | 説明                                                                                                                                 |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | モジュールのフォーマット文字列。                                                                                                                   |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

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

The `username` module shows active user's username. 次の条件のいずれかが満たされると、モジュールが表示されます:

- The current user is root/admin
- カレントユーザーが、ログインしているユーザーとは異なる
- ユーザーがSSHセッションとして接続されている
- `show_always`変数がtrueに設定されている

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### オプション

| オプション         | デフォルト                   | 説明                                          |
| ------------- | ----------------------- | ------------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root/admin. |
| `style_user`  | `"bold yellow"`         | The style used for non-root users.          |
| `format`      | `"[$user]($style) in "` | module のフォーマットです。                           |
| `show_always` | `false`                 | Always shows the `username` module.         |
| `disabled`    | `false`                 | Disables the `username` module.             |

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

- カレントディレクトリに`Vagrantfile`ファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに拡張子が`.v`のファイルが含まれている
- カレントディレクトリに`v.mod`、`vpkg.json`または`.vpkg-lock.json`ファイルが含まれている

### オプション

| オプション               | デフォルト                                        | 説明                                                     |
| ------------------- | -------------------------------------------- | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`         | module のフォーマットです。                                      |
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
| `symbol`   | `""`                             | The symbol used before displaying the repository name. |
| `style`    | `"bold yellow"`                  | モジュールのスタイルです。                                          |
| `format`   | `"vcsh [$symbol$repo]($style) "` | module のフォーマットです。                                      |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### 変数

| 変数        | 設定例                                         | 説明                         |
| --------- | ------------------------------------------- | -------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name |
| symbol    |                                             | オプション `記号` の値をミラーする        |
| style\* | `black bold dimmed`                         | オプション `style` の値をミラーする     |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). 次の条件のいずれかが満たされると、モジュールが表示されます:

- カレントディレクトリに拡張子が`.zig`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | module のフォーマットです。                                      |
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

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `detect_files`
- The current directory contains a directory whose name is in `detect_folders`
- The current directory contains a file whose extension is in `detect_extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matches with `os` field if defined.

::: tip

Multiple custom modules can be defined by using a `.`.

:::

::: tip

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

::: warning Command output is printed unescaped to the prompt

Whatever output the command generates is printed unmodified in the prompt. This means if the output contains special sequences that are interpreted by your shell they will be expanded when displayed. These special sequences are shell specific, e.g. you can write a command module that writes bash sequences, e.g. `\h`, but this module will not work in a fish or zsh shell.

Format strings can also contain shell specific prompt sequences, e.g. [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

:::

### オプション

| オプション               | デフォルト                           | 説明                                                                                                                                                                                                                                                                                            |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `""`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                                                                                                                                 |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the command returns a `0` status code.                                                                          |
| `shell`             |                                 | [See below](#custom-command-shell)                                                                                                                                                                                                                                                            |
| `description`       | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                  |
| `detect_files`      | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                                                                                                                                         |
| `detect_folders`    | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                                                                                                                                   |
| `detect_extensions` | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                                                                                                                                    |
| `symbol`            | `""`                            | The symbol used before displaying the command output.                                                                                                                                                                                                                                         |
| `style`             | `"bold green"`                  | モジュールのスタイルです。                                                                                                                                                                                                                                                                                 |
| `format`            | `"[$symbol($output )]($style)"` | module のフォーマットです。                                                                                                                                                                                                                                                                             |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                |
| `os`                |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                 |
| `use_stdin`         |                                 | An optional boolean value that overrides whether commands should be forwarded to the shell via the standard input or as an argument. If unset standard input is used by default, unless the shell does not support it (cmd, nushell). Setting this disables shell-specific argument handling. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                     |

### 変数

| 変数        | 説明                                     |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | オプション `記号` の値をミラーする                    |
| style\* | オプション `style` の値をミラーする                 |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. If `shell` is not given or only contains one element and Starship detects Cmd will be used, the following argument will automatically be added: `/C` and `stdin` will be set to `false`. If `shell` is not given or only contains one element and Starship detects Nushell will be used, the following arguments will automatically be added: `-c` and `stdin` will be set to `false`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

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
command = "echo foo" # shows output of command
detect_files = ["foo"] # can specify filters but wildcards are not supported
when = """ test "$HOME" = "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
detect_extensions = ["pst"] # filters *.pst files
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]

[custom.time-as-arg]
command = "time /T"
detect_extensions = ["pst"] # filters *.pst files
shell = ["pwsh.exe", "-NoProfile", "-Command"]
use_stdin = false
```
