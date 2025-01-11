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
[character] # 設定対象のモジュール名は 'character'
success_symbol = '[➜](bold green)' # セグメント 'success_symbol' を '➜' 配色 'bold green' (太字の緑色) に設定

# package モジュールを無効化してプロンプトから完全に非表示にする
[package]
disabled = true
```

### 設定ファイルの場所

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

Starship は警告やエラーログを既定で `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log` というファイルに出力します。ただし、${STARSHIP_SESSION_KEY} は端末のそれぞれのインスタンスに対応して決まります。 ディレクトリ名は環境変数 `STARSHIP_CACHE` を使って変更できます：

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

慣例により、ほとんどのモジュールにはデフォルトの端末色の接頭辞（"nodejs" の `via` など）と接尾辞として空のスペースがあります。

### 文字列

TOML記法では、[文字列](https://toml.io/en/v1.0.0#string)は`'`、`"`、`'''`、`"""`で宣言されます。

これらのStarship記法の記号は文字列のフォーマットにおいて特別な用途があり、文字として表示するためにはエスケープしなければなりません: `$ [ ] ( )`.

| 記号    | 種類         | 備考                             |
| ----- | ---------- | ------------------------------ |
| `'`   | リテラル文字列    | 少ないエスケープ                       |
| `"`   | 文字列        | より多くのエスケープ                     |
| `'''` | 複数行リテラル文字列 | 少ないエスケープ                       |
| `"""` | 複数行文字列     | より多くのエスケープ。宣言内の改行はエスケープで無視できます |

例：

```toml
# リテラル文字列
format = '☺\☻ '

# 通常文字列
format = "☺\\☻ "

# Starship の特殊記号をエスケープ
format = '\[\$\] '
```

改行を指定したい場合、複数行宣言が使えます。 例えば、新しい行に `$` 記号を表示したい場合、以下の `format` の設定が等価です。

```toml
# リテラル文字列を用いる
format = '''

\$'''

# 複数行基本文字列を用いる
format = """

\\$"""

# 基本文字列を用いる
format = "\n\\$"
```

複数行基本文字列では、改行をエスケープすることで、実際の値に影響を与えずにソースコードを整形できます。

```toml
format = """
line1\
line1\
line1
line2\
line2\
line2
"""
```

### フォーマット文字列

フォーマット文字列は、モジュールが出力するすべての変数に使われる書式です。 ほとんどのモジュールには、モジュールの表示形式を設定する `format` というエントリがあります。 テキスト、変数、およびテキストグループをフォーマット文字列で使用できます。

#### 変数

変数には、 `$` 記号と、その変数の名前が続きます。 変数の名前には英字と数字、`_`のみを含めることができます。

例：

- `'$version'` は、`version` という名前の変数を指定するフォーマット文字列です。
- `'$git_branch$git_commit'` は `git_branch` と `git_commit` という2つの変数を指定するフォーマット文字列です。
- `'$git_branch $git_commit'` は空白で区切られた2つの変数を指定します。

#### テキストグループ

テキストグループは二つの異なる部分で構成されています。

`[]`で囲まれている最初の部分は、 [フォーマット文字列](#format-strings) です。 テキスト、変数、または入れ子になったテキストグループを追加できます。

2 番目の部分は、 `()`で囲まれている [スタイル文字列](#style-strings) です。 これは、最初のフォーマット文字列のスタイルを設定するために使用できます。

例：

- `'[on](red bold)'` は文字列 `on` を赤色の太字テキストで出力します。
- `'[⌘ $version](bold green)'` は記号 `⌘` に続いて変数 `version` の値を、緑色の太字で出力します。
- `'[a [b](red) c](green)'` は  `a b c` のうち `b` だけ赤色で表示し、 `a` と `c` を緑色で表示します。

#### スタイルの設定

Starshipのほとんどのモジュールでは、表示スタイルを設定できます。 これは、設定を指定する文字列であるエントリ（`style`）で行われます。 スタイル文字列の例とその機能を次に示します。 完全な構文については、[高度な設定](../advanced-config/)を参照してください 。

- `"fg:green bg:blue"` は、青色の背景上の緑色のテキストを設定します
- `"bg:blue fg:bright-green"` は、青色の背景上の明るい緑色のテキストを設定します
- `'bold fg:27'` は [ANSI 色 27](https://i.stack.imgur.com/KTSQa.png) の太字テキストを設定します
- `'underline bg:#bf5700'` は、バーントオレンジ色の背景に下線付きのテキストを設定します
- `'bold italic fg:purple'` は、紫色の太字斜体のテキストを設定します
- `''` はすべてのスタイルを明示的に無効にします

スタイルの実際の見た目は、端末エミュレータによることに注意してください。 たとえば、一部の端末エミュレータはテキストを太字にする代わりに色を明るくします。また、一部のカラーテーマは通常の色と明るい色と同じ値を使用します。 また、斜体のテキストを取得するには、端末で斜体をサポートする必要があります。

#### 条件付きフォーマット設定

`(` と `)` 内のすべての変数が空の場合、条件付き書式文字列はレンダリングされません。

例：

- `'(@$region)'` は変数 `region` が `None` または空の場合は表示されませんが、値がある場合は `@` に続いてregionの値が表示されます。
- `'(some text)'` は括弧の中に変数がないので、常に何も表示しません。
- `$combined` を `\[$a$b\]` のショートカットとした時、 `$a` と `$b` が共に `None` の場合にのみ `'($combined)'` は空になります。 これは `'(\[$a$b\] )'` と同じ動作になります。

### 否定的マッチング (Negative matching)

多くのモジュールには、変数として `detect_extensions`、 `detect_files`、および `detect_folders` があります。 これらの変数には、一致文字列または除外文字列のリストを指定します。 先頭に文字 '!' を指定することで、「否定的」な指定 (つまり除外する文字列) を設定できます。 ディレクトリに_何れかの_否定的文字列が一致すると、モジュールは選択されません。

拡張子に対する一致 (detect_extensions) は、ファイル名の中の最後または最初のドットに続く文字列に対して行われます。 例えばファイル名 `foo.bar.tar.gz` の場合は、 `bar.tar.gz` および `gz` が変数 `detect_extensions` の一致対象になります。 ドットで始まるファイル名は、拡張子が全くないと見なされます。

どのように動作するかの実例として、TypeScript と一致させたいが MPEGトランスポートストリームファイルを場外したい場合は以下のようにします。

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## プロンプト

これは、プロンプト全体のオプションのリストです。

### オプション

| オプション             | デフォルト                          | 説明                                                                                         |
| ----------------- | ------------------------------ | ------------------------------------------------------------------------------------------ |
| `format`          | [link](#default-prompt-format) | プロンプトの形式を設定します。                                                                            |
| `right_format`    | `''`                           | [右プロンプトの有効化](../advanced-config/#enable-right-prompt) を参照してください。                           |
| `scan_timeout`    | `30`                           | ファイルをスキャンする際のタイムアウト時間 (milliseconds) です。                                                   |
| `command_timeout` | `500`                          | Starshipによって実行されたコマンドのタイムアウト時間 (milliseconds) です。                                          |
| `add_newline`     | `true`                         | シェルプロンプトの間に空行を挿入します。                                                                       |
| `palette`         | `''`                           | `palettes` の中から使用する配色を指定します。                                                               |
| `palettes`        | `{}`                           | [色](../advanced-config/#style-strings) をユーザー定義の色名に割り当てる配色のコレクション。 ※配色で自身の色定義を参照することはできません。 |
| `follow_symlinks` | `true`                         | ディレクトリかどうかをチェックするためにシンボリックリンクをたどります。git などのモジュールで使われます。                                    |

::: tip

ネットワーク上のファイルシステムへのシンボリックリンクがある場合は、 `follow_symlinks` を `false` に設定することを検討してください。

:::

### 設定例

```toml
# ~/.config/starship.toml

# カスタムフォーマットを利用
format = '''
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) '''

# starship が現在のディレクトリのファイルをチェックするのを10ミリ秒待ちます
scan_timeout = 10

# プロンプトの前の空行を無効化
add_newline = false

# カスタム配色として 'foo' を指定
palette = 'foo'

# カスタム配色を定義
[palettes.foo]
# 既存の色を上書き
blue = '21'
# 新しい色を定義
mustard = '#af8700'
```

### デフォルトのプロンプトフォーマット

デフォルトの `format` は、空または `format` が指定されていない場合、プロンプトのフォーマットを定義するために使用されます。 デフォルトは次のとおりです。

```toml
format = '$all'

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
$fossil_branch\
$fossil_metrics\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$pijul_channel\
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
$fennel\
$gleam\
$golang\
$guix_shell\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$gradle\
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
$quarto\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$solidity\
$swift\
$terraform\
$typst\
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
$nats\
$direnv\
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
$os\
$container\
$shell\
$character"""
```

デフォルトのフォーマットを拡張したいだけなら、`$all`を使用できます。 フォーマットに明示的に追加したモジュールは重複しません。 例:

```toml
# ディレクトリを2行目に移動
format = '$all$directory$character'
```

## AWS

`aws`モジュールは、一時的な資格情報を使用する場合、現在のAWSリージョンとプロファイル、および有効期限タイマーを表示します。 モジュールの出力は、必要に応じて`AWS_REGION`、`AWS_DEFAULT_REGION`と`AWS_PROFILE`の環境変数と、`~/.aws/config`と`~/.aws/credentials`のファイルが使用されます。

モジュールは、資格情報が`~/.aws/credentials`にある場合、または`~/.aws/config`に`credential_process`か`sso_start_url`または`sso_session`が定義されている場合にのみプロファイルを表示します。 あるいは、環境変数に`AWS_ACCESS_KEY_ID`、`AWS_SECRET_ACCESS_KEY`または`AWS_SESSION_TOKEN`のいずれかが定義されていれば条件を満たします。 もし`force_display`のオプションを`true`に設定した場合、上記の条件による資格情報が検出されない場合でも、利用可能なすべての情報が表示されます。

[aws-vault](https://github.com/99designs/aws-vault)を使う場合、環境変数`AWS_VAULT`からプロファイルが、環境変数`AWS_SESSION_EXPIRATION`から資格情報の有効期限が読み込まれます。

[awsu](https://github.com/kreuzwerker/awsu) を使う場合、そのプロファイルは環境変数 `AWSU_PROFILE` から読まれます。

[AWSume](https://awsu.me)を使う場合、環境変数`AWSUME_PROFILE`からプロファイルが、環境変数`AWSUME_EXPIRATION`から資格情報の有効期限が読み込まれます。

[saml2aws](https://github.com/Versent/saml2aws) を使用する場合、 `~/.aws/credentials` から得られる有効期限情報は `x_security_token_expires` キーで代替されます。

[aws-sso-cli](https://github.com/synfinatic/aws-sso-cli) を使う場合、そのプロファイルは環境変数 `AWSU_PROFILE` から読まれます。

### オプション

| オプション               | デフォルト                                                                 | 説明                                                                                   |
| ------------------- | --------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | module のフォーマットです。                                                                    |
| `symbol`            | `'☁️ '`                                                               | 現在のAWSプロファイルを表示する前に表示される記号です。                                                        |
| `region_aliases`    | `{}`                                                                  | AWS名に加えて表示するリージョンのエイリアスです。                                                           |
| `profile_aliases`   | `{}`                                                                  | AWS名に加えて表示するプロファイルのエイリアスです。                                                          |
| `style`             | `'bold yellow'`                                                       | モジュールのスタイルです。                                                                        |
| `expiration_symbol` | `'X'`                                                                 | この記号は一時的な資格情報が有効期限切れの場合に表示されます。                                                      |
| `disabled`          | `false`                                                               | `aws`モジュールを無効にします。                                                                   |
| `force_display`     | `false`                                                               | `true`の場合、`credentials`、`credential_process`または`sso_start_url`が設定されていない場合でも情報を表示します。 |

### 変数

| 変数        | 設定例              | 説明                      |
| --------- | ---------------- | ----------------------- |
| region    | `ap-northeast-1` | 現在のAWSリージョン             |
| profile   | `astronauts`     | 現在のAWSプロファイル            |
| duration  | `2h27m20s`       | 一時的な資格情報の有効期限           |
| symbol    |                  | オプション `symbol` の値をミラーする |
| style\* |                  | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

#### すべてを表示

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
[aws.profile_aliases]
CompanyGroupFrobozzOnCallAccess = 'Frobozz'
```

#### リージョンを表示

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$region]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
```

#### プロファイルを表示

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$profile]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.profile_aliases]
Enterprise_Naming_Scheme-voidstars = 'void**'
```

## Azure

`azure` モジュールは、現在のAzureサブスクリプションを表示します。 これは、 `~/.azure/azureProfile.json` ファイルで定義されているデフォルトのサブスクリプション名またはユーザー名の表示に基づいています。

### オプション

| 変数                     | デフォルト                                    | 説明                                    |
| ---------------------- | ---------------------------------------- | ------------------------------------- |
| `format`               | `'on [$symbol($subscription)]($style) '` | Azure module のフォーマットです。               |
| `symbol`               | `'󰠅 '`                                   | フォーマットで使用される記号です。                     |
| `style`                | `'blue bold'`                            | フォーマットで使用されるスタイルです。                   |
| `disabled`             | `true`                                   | `azure`モジュールを無効にします。                  |
| `subscription_aliases` | `{}`                                     | Azure サブスクリプション名に加えて表示されるそれらのエイリアスの表。 |

### 設定例

#### サブスクリプション名を表示

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = '󰠅 '
style = 'blue bold'
```

#### ユーザー名を表示

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "󰠅 "
style = "blue bold"
```

#### サブスクリプション名のエイリアスを表示

```toml
# ~/.config/starship.toml

[azure.subscription_aliases]
very-long-subscription-name = 'vlsn'
```

## バッテリー

`battery`モジュールは、デバイスのバッテリー残量と現在の充電状態を示します。 モジュールは、デバイスのバッテリー残量が10％未満の場合にのみ表示されます。

### オプション

| オプション                | デフォルト                             | 説明                        |
| -------------------- | --------------------------------- | ------------------------- |
| `full_symbol`        | `'󰁹 '`                            | バッテリーが満タンのときに表示される記号です。   |
| `charging_symbol`    | `'󰂄 '`                            | バッテリーの充電中に表示される記号です。      |
| `discharging_symbol` | `'󰂃 '`                            | バッテリーが放電しているときに表示される記号です。 |
| `unknown_symbol`     | `'󰁽 '`                            | バッテリー状態が不明なときに表示される記号です。  |
| `empty_symbol`       | `'󰂎 '`                            | バッテリーが空のときに表示される記号です。     |
| `format`             | `'[$symbol$percentage]($style) '` | module のフォーマットです。         |
| `display`            | [link](#battery-display)          | モジュールの閾値とスタイルを表示します。      |
| `disabled`           | `false`                           | `battery`モジュールを無効にします。    |

### 設定例

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### バッテリーの表示

`display`オプションを使用して、バッテリーインジケーターを表示するタイミング（threshold）、どのシンボルが使われるか(symbol) と外観（style）を定義します。 `display` が提供されない場合、 デフォルトは次のとおりです。

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

`charging_symbol`と`discharging_symbol`オプションのデフォルト値はそれぞれ`battery`の `charging_symbol`と`discharging_symbol`になります。

#### オプション

`display`オプションは、次の表の通りです。

| オプション                | デフォルト        | 説明                                                                                     |
| -------------------- | ------------ | -------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | バッテリーが表示される上限です。                                                                       |
| `style`              | `'red bold'` | displayオプションが使用されている場合のスタイルです。                                                         |
| `charging_symbol`    |              | displayオプションが使用されている場合はこののシンボルが表示されます。デフォルトはバッテリーの `charging_symbol` オプションと同じになります。    |
| `discharging_symbol` |              | displayオプションが使用されている場合はこののシンボルが表示されます。デフォルトはバッテリーの `discharging_symbol` オプションと同じになります。 |

#### 設定例

```toml
[[battery.display]] # バッテリー容量が0～10%の時'bold red'(太字の赤)でdischarging_symbolを表示
threshold = 10
style = 'bold red'

[[battery.display]] # バッテリー容量が10～30%の時'bold yellow'(太字の黄)で記号💦を表示
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦 '

# バッテリー容量が30%より大きい時、バッテリーインジケータは非表示
```

## Buf

`buf`モジュールは、現在インストールされている[Buf](https://buf.build)のバージョンを表示します。 By default, the module is shown if the current directory contains a [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), or [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) configuration file.

### オプション

| オプション               | デフォルト                                           | 説明                         |
| ------------------- | ----------------------------------------------- | -------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | `buf`モジュールの形式。             |
| `version_format`    | `'v${raw}'`                                     | バージョンのフォーマット。              |
| `symbol`            | `'🐃 '`                                          | Bufのバージョンを表示する前に使用される記号です。 |
| `detect_extensions` | `[]`                                            | どの拡張子がこのモジュールをアクティブにするか    |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | どのファイル名がこのモジュールをアクティブにするか  |
| `detect_folders`    | `[]`                                            | どのフォルダーがこのモジュールをアクティブにするか  |
| `style`             | `'bold blue'`                                   | モジュールのスタイルです。              |
| `disabled`          | `false`                                         | `elixir`モジュールを無効にします。      |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| `version` | `v1.0.0` | `buf`のバージョン             |
| `symbol`  |          | オプション `symbol` の値をミラーする |
| `style`*  |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

`bun` モジュールは、現在インストールされている[bun](https://bun.sh) JavaScript ランタイムのバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- 現在のディレクトリに`bun.lock`ファイルが含まれている
- カレントディレクトリに`bun.lockb`ファイルが含まれている
- カレントディレクトリに`bunfig.toml`ファイルが含まれている

### オプション

| オプション               | デフォルト                                      | 説明                                                     |
| ------------------- | ------------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`       | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                                | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🥟 '`                                     | Bun のシンボルを表すフォーマット文字列                                  |
| `detect_extensions` | `[]`                                       | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                       | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold red'`                               | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                    | `bun` モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v0.1.4` | `bun`のバージョン             |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

#### フォーマットの変更

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

#### Node.js を置き換える

Bun ランタイムだけが表示されるように、設定中の [nodejs モジュール](#nodejs) の `detect_files` プロパティを上書きできます。

```toml
[nodejs]
detect_files = ['package.json', '.node-version', '!bunfig.toml', '!bun.lockb']
```

## C

`c` モジュールは、利用しているCコンパイラに関するいくつかの情報を表示します。 デフォルトでは、カレントディレクトリに`.c`または`.h`ファイルが含まれている場合、モジュールが表示されます。

### オプション

| オプション               | デフォルト                                                                         | 説明                                                     |
| ------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | モジュールのフォーマット文字列。                                       |
| `version_format`    | `'v${raw}'`                                                                   | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'C '`                                                                        | コンパイラの詳細を表示する前に使用される記号です。                              |
| `detect_extensions` | `['c', 'h']`                                                                  | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                                                          | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                          | どのフォルダーがこのモジュールをアクティブにするか                              |
| `commands`          | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | コンパイラを検出する方法                                           |
| `style`             | `'bold 149'`                                                                  | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                       | `c`モジュールを無効にします。                                       |

### 変数

| 変数      | 設定例    | 説明                      |
| ------- | ------ | ----------------------- |
| name    | clang  | コンパイラ名                  |
| version | 13.0.0 | コンパイラのバージョン             |
| symbol  |        | オプション `symbol` の値をミラーする |
| style   |        | オプション `style` の値をミラーする  |

`version`はデフォルトのフォーマットではないことに注意してください。

### Commands

`commands`オプションは、コンパイラのバージョンと名前を判別するためのコマンドのリストを受け入れます。

各コマンドは、実行可能ファイル名 (とそれに続く引数) のリストで表現されます。通常は`['mycc', '--version']`のようになります。 StarshipはSTDOUTから結果が得られるまで各コマンドを実行を試みます。

もし、Cコンパイラがこのモジュールでサポートされていない場合は、[GitHubで問題を提起する](https://github.com/starship/starship/)ことでリクエストできます。

### 設定例

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## Character

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
| `format`                    | `'$symbol '`         | テキスト入力の前に使用されるフォーマット文字列。                               |
| `success_symbol`            | `'[❯](bold green)'`  | 前のコマンドが成功した場合にテキスト入力の前に使用されるフォーマット文字列です。               |
| `error_symbol`              | `'[❯](bold red)'`    | 前のコマンドが失敗した場合にテキスト入力の前に使用されるフォーマット文字列です。               |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | シェルがvimノーマルモードの場合にテキスト入力の前に使用されるフォーマット文字列です。           |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | シェルがvimの`replace_one`モードの場合にテキスト入力の前に使用されるフォーマット文字列です。 |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | シェルがvimの置換モードの場合にテキスト入力の前に使用されるフォーマット文字列です。            |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | シェルがvimビジュアルモードの場合にテキスト入力の前に使用されるフォーマット文字列です。          |
| `disabled`                  | `false`              | `character`モジュールを無効にします。                               |

### 変数

| 変数     | 設定例 | 説明                                                                                                   |
| ------ | --- | ---------------------------------------------------------------------------------------------------- |
| symbol |     | 現在の状態に応じた `success_symbol`, `error_symbol`, `vimcmd_symbol` または `vimcmd_replace_one_symbol` の何れかのミラー |

### 設定例

#### エラーの形状をカスタムする

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[✗](bold red) '
```

#### エラーの形状をカスタムしない

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[➜](bold red) '
```

#### vimの形状をカスタムする

```toml
# ~/.config/starship.toml

[character]
vimcmd_symbol = '[V](bold green) '
```

## CMake

`cmake`モジュールは、現在インストールされている[Cmake](https://cmake.org/)のバージョンを表示します。 デフォルトでは次のいずれかの条件が満たされると、モジュールがアクティブになります。

- カレントディレクトリに `CMakeLists.txt` ファイルが含まれている
- カレントディレクトリに `CMakeCache.txt` ファイルが含まれている

### オプション

| オプション               | デフォルト                                  | 説明                                                     |
| ------------------- | -------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`   | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                            | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'△ '`                                 | cmakeのバージョンの前に使用される記号                                  |
| `detect_extensions` | `[]`                                   | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                   | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold blue'`                          | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                | `cmake`モジュールを無効にします。                                   |

### 変数

| 変数        | 設定例       | 説明                      |
| --------- | --------- | ----------------------- |
| version   | `v3.17.3` | cmake のバージョン            |
| symbol    |           | オプション `symbol` の値をミラーする |
| style\* |           | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

## COBOL / GNUCOBOL

`COBOL` モジュールは、現在インストールされているCOBOLのバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに、`.cob`または`.COB`の拡張子のファイルが含まれている
- カレントディレクトリに、`.cbl`または`.CBL`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `symbol`            | `'⚙️ '`                              | COBOLのバージョンを表示する前に使用される記号です。                           |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `style`             | `'bold blue'`                        | モジュールのスタイルです。                                          |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | `cobol`モジュールを無効にします。                                   |

### 変数

| 変数        | 設定例        | 説明                      |
| --------- | ---------- | ----------------------- |
| version   | `v3.1.2.0` | `cobol`のバージョン           |
| symbol    |            | オプション `symbol` の値をミラーする |
| style\* |            | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

## Command Duration

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
| `format`               | `'took [$duration]($style) '` | module のフォーマットです。                                                                               |
| `style`                | `'bold yellow'`               | モジュールのスタイルです。                                                                                   |
| `disabled`             | `false`                       | `cmd_duration`モジュールを無効にします。                                                                     |
| `show_notifications`   | `false`                       | コマンドが完了したらデスクトップ通知を表示します。                                                                       |
| `min_time_to_notify`   | `45_000`                      | 通知を持続する最短期間 (ミリ秒単位) です。                                                                         |
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
format = 'underwent [$duration](bold yellow)'
```

## Conda

`conda` モジュールは、`$CONDA_DEFAULT_ENV` が設定されている場合、現在の[Conda](https://docs.conda.io/en/latest/) 環境を表示します。

::: tip

Note: これはconda自身の プロンプト修飾子 を抑制しません。`conda config --set changeps1 False` で実行することができます。 If you use [pixi](https://pixi.sh), you can disable pixi's prompt modifier by running `pixi config set change-ps1 false`.

:::

### オプション

| オプション               | デフォルト                                  | 説明                                                                                                               |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | 環境が`conda create -p [path]`で作成された場合、環境パスが切り捨てられるディレクトリ数。 `0`は切り捨てがないことを意味します。  [`directory`](#directory)もご覧ください。 |
| `symbol`            | `'🅒 '`                                 | 環境名の直前に使用されるシンボルです。                                                                                              |
| `style`             | `'bold green'`                         | モジュールのスタイルです。                                                                                                    |
| `format`            | `'via [$symbol$environment]($style) '` | module のフォーマットです。                                                                                                |
| `ignore_base`       | `true`                                 | アクティブになった時、環境`base`を無視します。                                                                                       |
| `disabled`          | `false`                                | `conda`モジュールを無効にします。                                                                                             |

### 変数

| 変数          | 設定例          | 説明                      |
| ----------- | ------------ | ----------------------- |
| environment | `astronauts` | 現在の conda 環境            |
| symbol      |              | オプション `symbol` の値をミラーする |
| style\*   |              | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## コンテナ

`container`モジュールは、コンテナ内の場合、シンボルとコンテナ名を表示します。

### オプション

| オプション      | デフォルト                              | 説明                        |
| ---------- | ---------------------------------- | ------------------------- |
| `symbol`   | `'⬢'`                              | コンテナ内にいる場合、このシンボルが表示されます。 |
| `style`    | `'bold red dimmed'`                | モジュールのスタイルです。             |
| `format`   | `'[$symbol \[$name\]]($style) '` | module のフォーマットです。         |
| `disabled` | `false`                            | `container`モジュールを無効にします。  |

### 変数

| 変数        | 設定例                 | 説明                      |
| --------- | ------------------- | ----------------------- |
| name      | `fedora-toolbox:35` | コンテナ名                   |
| symbol    |                     | オプション `symbol` の値をミラーする |
| style\* |                     | オプション `style` の値をミラーする  |

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
| `symbol`            | `'🔮 '`                               | Crystalのバージョンを表示する前に使用される記号です。                         |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `style`             | `'bold red'`                         | モジュールのスタイルです。                                          |
| `detect_extensions` | `['cr']`                             | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['shard.yml']`                      | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | `crystal`モジュールを無効にします。                                 |

### 変数

| 変数        | 設定例       | 説明                      |
| --------- | --------- | ----------------------- |
| version   | `v0.32.1` | `crystal` のバージョン        |
| symbol    |           | オプション `symbol` の値をミラーする |
| style\* |           | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

`daml`モジュールは、Damlプロジェクトのルートディレクトリにいるときに、使用している[Daml](https://www.digitalasset.com/developers) SDKバージョンを表示します。 環境変数`DAML_SDK_VERSION`を上書きしない限り、`daml.yaml`ファイルの`sdk-version`が使用されます。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`daml.yaml`ファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'Λ '`                               | Damlの記号を表すフォーマット文字列です。                                 |
| `style`             | `'bold cyan'`                        | モジュールのスタイルです。                                          |
| `detect_extensions` | `[]`                                 | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['daml.yaml']`                      | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | `daml`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v2.2.0` | `daml`のバージョン            |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

`dart`モジュールは、現在インストールされている[Dart](https://dart.dev/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.dart`の拡張子のファイルが含まれている
- カレントディレクトリに`.dart_tool`ディレクトリが含まれている
- カレントディレクトリに`pubspec.yaml`、`pubspec.yml`または`pubspec.lock` が含まれている

### オプション

| オプション               | デフォルト                                             | 説明                                                     |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`              | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                                       | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🎯 '`                                            | Dartのシンボルを表すフォーマット文字列                                  |
| `detect_extensions` | `['dart']`                                        | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `['.dart_tool']`                                  | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold blue'`                                     | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                           | `dart`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v2.8.4` | `dart` のバージョン           |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

`deno`モジュールは、現在インストールされている[Deno](https://deno.land/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### オプション

| オプション               | デフォルト                                                                                | 説明                                                     |
| ------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                                                                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🦕 '`                                                                               | Deno のシンボルを表すフォーマット文字列                                 |
| `detect_extensions` | `[]`                                                                                 | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'green bold'`                                                                       | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                              | `deno`モジュールを無効化します。                                    |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v1.8.3` | `deno`のバージョン            |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

### 設定例

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Directory

`directory` モジュールは現在のディレクトリへのパスを表示します。親フォルダは3つまでに切り捨てられます。 git リポジトリ内にいる場合は、リポジトリのルートで切り捨てられます。

`fish_style_pwd_dir_length` を使用している場合、切り捨てられたパスを非表示にする代わりに、オプションで有効にした数値に基づいた各ディレクトリの短縮名が表示されます。

たとえば、`~/Dev/Nix/nixpkgs/pkgs`で、`nixpkgs` がリポジトリルートであり、オプションが `1` に設定されている場合を挙げます。 この場合、`nixpkgs/pkgs` の代わりに、`~/D/N/nixpkgs/pkgs` と表示されます。

### オプション

| オプション                    | デフォルト                                                                                                                        | 説明                                                                     |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | 現在のディレクトリを切り捨てる親フォルダーの数です。                                             |
| `truncate_to_repo`       | `true`                                                                                                                       | 現在いるgitリポジトリのルートに切り捨てるかどうかです。                                          |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | module のフォーマットです。                                                      |
| `style`                  | `'bold cyan'`                                                                                                                | モジュールのスタイルです。                                                          |
| `disabled`               | `false`                                                                                                                      | `directory`モジュールを無効にします。                                               |
| `read_only`              | `'🔒'`                                                                                                                        | このシンボルが表示されている時、現在のディレクトリは読み取り専用です。                                    |
| `read_only_style`        | `'red'`                                                                                                                      | 読み取り専用シンボルのスタイルです。                                                     |
| `truncation_symbol`      | `''`                                                                                                                         | 切り捨てられたパスの接頭辞として付けるシンボルです。 例: '…/'                                     |
| `before_repo_root_style` |                                                                                                                              | パス名のうち、git リポジトリのルートより上の階層のスタイル。 デフォルトの値は `style` と同じです。               |
| `repo_root_style`        |                                                                                                                              | gitリポジトリのルートのスタイルです。 デフォルトの値は `style` と同じです。                           |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | `before_repo_root_style` と `repo_root_style` が定義されているときの git リポジトリの書式。 |
| `home_symbol`            | `'~'`                                                                                                                        | ホームディレクトリを示すシンボルです。                                                    |
| `use_os_path_sep`        | `true`                                                                                                                       | `/`を使用する代わりに、OS固有のパスの区切り文字を使用します。(例: Windowsの場合`\`)                 |

<details>
<summary>このモジュールは、どのようにディレクトリを表示するかについての高度なオプションをいくつか持っています。</summary>

| 詳細設定                        | デフォルト  | 説明                                                                                                                          |
| --------------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | パスに適用される置換の辞書。                                                                                                              |
| `fish_style_pwd_dir_length` | `0`    | fish shellのpwdパスロジックを適用するときに使用する文字数です。                                                                                      |
| `use_logical_path`          | `true` | `true` の場合、シェルによって `PWD` または `--logical-path` を通して指定される起点からの論理パスを表示します。 `false` の場合、代わりにシンボリックリンクを解決したファイルシステム上の物理パスを表示します。 |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories of Java. ※これは fish 形式の PWD を無効化します。

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length` は標準の短縮設定と組み合わさって、一見して意外な結果をもたらすかもしれません。非ゼロの値の場合、通常省略されるディレクトリ名がその文字数だけ表示されます。 例えばパス `/built/this/city/on/rock/and/roll` は通常 `rock/and/roll` と表示されますが、 `fish_style_pwd_dir_length = 1` の時は `/b/t/c/o/rock/and/roll` と表示されます。つまり、通常削除されるパスコンポーネントが代わりに一文字で表示されます。 `fish_style_pwd_dir_length = 2`の場合、 `/bu/th/ci/on/rock/and/roll` になります。

</details>

### 変数

| 変数        | 設定例                   | 説明                     |
| --------- | --------------------- | ---------------------- |
| path      | `'D:/Projects'`       | カレントディレクトリのパス          |
| style\* | `'black bold dimmed'` | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

<details>
<summary>gitリポジトリは追加の変数があります。</summary>

`/path/to/home/git_repo/src/lib`のパスについて考えます。

| 変数                 | 設定例                   | 説明                     |
| ------------------ | --------------------- | ---------------------- |
| before_root_path | `'/path/to/home/'`    | gitルートディレクトリパスの前のパス    |
| repo_root          | `'git_repo'`          | gitルートディレクトリの名前        |
| path               | `'/src/lib'`          | 残りのパス                  |
| style              | `'black bold dimmed'` | オプション `style` の値をミラーする |
| repo_root_style  | `'underline white'`   | gitルートディレクトリの名前のスタイル   |

</details>

### 設定例

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Direnv

`direnv` モジュールは、もし存在すれば、現在の rc ファイルの状態を示します。 状態として rc ファイルへのパス、ロードされているかどうか、および `direnv` によって許可されているかどうかを含みます。

### オプション

| オプション               | デフォルト                                  | 説明                                                      |
| ------------------- | -------------------------------------- | ------------------------------------------------------- |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | module のフォーマットです。                                       |
| `symbol`            | `'direnv '`                            | Direnv コンテキストの前に表示される記号です。                              |
| `style`             | `'bold orange'`                        | モジュールのスタイルです。                                           |
| `disabled`          | `true`                                 | `Direnv`モジュールを無効にします。                                   |
| `detect_extensions` | `[]`                                   | どの拡張子がこのモジュールをアクティブにするか                                 |
| `detect_files`      | `['.envrc']`                           | どのファイル名がこのモジュールをアクティブにするか                               |
| `detect_folders`    | `[]`                                   | どのフォルダーがこのモジュールをアクティブにするか                               |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | Which environment variables should trigger this module. |
| `allowed_msg`       | `'allowed'`                            | Rcファイルが許可されているとき (allowed) に表示されるメッセージです。               |
| `not_allowed_msg`   | `'not allowed'`                        | Rcファイルが不許可のとき (not_allowed) に表示されるメッセージです。              |
| `denied_msg`        | `'denied'`                             | Rcファイルが拒否されているとき (denied) に表示されるメッセージです。                |
| `loaded_msg`        | `'loaded'`                             | Rcファイルがロードされているときに表示されるメッセージです。                         |
| `unloaded_msg`      | `'not loaded'`                         | Rcファイルがロードされていないときに表示されるメッセージです。                        |

### 変数

| 変数        | 設定例                 | 説明                      |
| --------- | ------------------- | ----------------------- |
| loaded    | `loaded`            | 現在のrcファイルがロードされているかどうか。 |
| allowed   | `denied`            | 現在のrcファイルが許可されているかどうか。  |
| rc_path   | `/home/test/.envrc` | 現在の rc ファイルパス。          |
| symbol    |                     | オプション `symbol` の値のミラー。  |
| style\* | `red bold`          | オプション `style` の値のミラー。   |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default` or `desktop-linux`, or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or `DOCKER_CONTEXT` environment variables are set (as they are meant to override the context in use).

### オプション

| オプション               | デフォルト                                                         | 説明                                                             |
| ------------------- | ------------------------------------------------------------- | -------------------------------------------------------------- |
| `format`            | `'via [$symbol$context]($style) '`                            | module のフォーマットです。                                              |
| `symbol`            | `'🐳 '`                                                        | Dockerコンテキストを表示する前に使用される記号です。                                  |
| `only_with_files`   | `true`                                                        | ファイルに一致する場合にのみ表示                                               |
| `detect_extensions` | `[]`                                                          | どの拡張子がこのモジュールをトリガーするか(`only_with_files`がtrueになっている必要があります)。    |
| `detect_files`      | `['docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | どんなファイル名がこのモジュールをトリガーするか(`only_with_files`がtrueになっている必要があります)。 |
| `detect_folders`    | `[]`                                                          | どんなフォルダがこのモジュールをトリガーするか(`only_with_files`がtrueになっている必要があります)。  |
| `style`             | `'blue bold'`                                                 | モジュールのスタイルです。                                                  |
| `disabled`          | `false`                                                       | `docker_context`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例            | 説明                      |
| --------- | -------------- | ----------------------- |
| context   | `test_context` | 現在の Docker コンテキスト       |
| symbol    |                | オプション `symbol` の値をミラーする |
| style\* |                | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
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
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                           | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                                                                                             | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'.NET '`                                                                                               | dotnetのバージョンを表示する前に使用される記号です。                          |
| `heuristic`         | `true`                                                                                                  | より高速なバージョン検出を使用して、starshipの動作を維持します。                   |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                                                    | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold blue'`                                                                                           | モジュールのスタイルです。                                          |
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
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

`elixir` モジュールは、現在インストールされている[Elixir](https://elixir-lang.org/)と[Erlang/OTP](https://erlang.org/doc/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`mix.exs`ファイルが含まれている.

### オプション

| オプション               | デフォルト                                                       | 説明                                                     |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | module elixirのフォーマットです。                                |
| `version_format`    | `'v${raw}'`                                                 | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'💧 '`                                                      | Elixir/Erlangのバージョンを表示する前に使用される記号です。                   |
| `detect_extensions` | `[]`                                                        | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['mix.exs']`                                               | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                        | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold purple'`                                             | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                     | `elixir`モジュールを無効にします。                                  |

### 変数

| 変数          | 設定例     | 説明                      |
| ----------- | ------- | ----------------------- |
| version     | `v1.10` | `elixir`のバージョン          |
| otp_version |         | `elixir`のotpバージョン       |
| symbol      |         | オプション `symbol` の値をミラーする |
| style\*   |         | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
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
| `format`            | `'via [$symbol($version )]($style)'`               | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                                        | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🌳 '`                                             | Elmのシンボルを表すフォーマット文字列                                   |
| `detect_extensions` | `['elm']`                                          | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `['elm-stuff']`                                    | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'cyan bold'`                                      | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                            | `elm`モジュールを無効にします。                                     |

### 変数

| 変数        | 設定例       | 説明                      |
| --------- | --------- | ----------------------- |
| version   | `v0.19.1` | `elm`のバージョン             |
| symbol    |           | オプション `symbol` の値をミラーする |
| style\* |           | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## 環境変数

`env_var`モジュールは、選択された環境変数の現在の値を表示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

- `variable`オプションが、既存の環境変数と一致する
- `variable`オプションが定義されておらず、`default`オプションが定義されている

::: tip

env_var モジュールが表示される順序は、`${env_var.foo}` (ドットが含まれるので `${...}` を使う必要があります) をトップレベルの `format` に入れることで個別に設定できます。 既定では、 `env_var` モジュールは、単にすべての env_var モジュールを定義順で表示します。

:::

::: tip

`.`を使うことで複数の環境変数を表示することができます。 (例を確認してみてください) `variable`が設定されていない場合、このモジュールは`.`以降に書かれている環境変数の値を表示します。

例: 次の設定ではUSER環境変数を表示します。

```toml
# ~/.config/starship.toml

[env_var.USER]
default = 'unknown user'
```

:::

### オプション

| オプション         | デフォルト                          | 説明                                     |
| ------------- | ------------------------------ | -------------------------------------- |
| `symbol`      | `""`                           | 環境変数を表示する前に使用される記号です。                  |
| `variable`    |                                | 表示される環境変数です。                           |
| `default`     |                                | 上のvariableが定義されていない場合に表示されるデフォルトの値です。  |
| `format`      | `"with [$env_value]($style) "` | module のフォーマットです。                      |
| `description` | `"<env_var module>"`     | `starship explain` 実行の際に表示されるモジュールの説明。 |
| `disabled`    | `false`                        | `env_var`モジュールを無効にします。                 |

### 変数

| 変数        | 設定例                                         | 説明                      |
| --------- | ------------------------------------------- | ----------------------- |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | オプション`variable`の値       |
| symbol    |                                             | オプション `symbol` の値をミラーする |
| style\* | `black bold dimmed`                         | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[env_var]
variable = 'SHELL'
default = 'unknown shell'
```

Displaying multiple environmental variables:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = 'SHELL'
default = 'unknown shell'
[env_var.USER]
default = 'unknown user'
```

## Erlang

`erlang`モジュールは、現在インストールされている[Erlang/OTP](https://erlang.org/doc/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`rebar.config`ファイルが含まれている.
- カレントディレクトリに`erlang.mk`ファイルが含まれている.

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `' '`                               | Erlangのバージョンを表示する前に使用される記号です。                          |
| `style`             | `'bold red'`                         | モジュールのスタイルです。                                          |
| `detect_extensions` | `[]`                                 | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | `erlang`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例       | 説明                      |
| --------- | --------- | ----------------------- |
| version   | `v22.1.3` | `erlang` のバージョン         |
| symbol    |           | オプション `symbol` の値をミラーする |
| style\* |           | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Fennel

`fennel`モジュールは、現在インストールされている[Fennel](https://fennel-lang.org)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに拡張子 `.fnl` のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🧅 '`                               | Fennel のバージョンの前に表示される記号です。                             |
| `style`             | `'bold green'`                       | モジュールのスタイルです。                                          |
| `detect_extensions` | `['fnl']`                            | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | `fennel`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v1.2.1` | `fennel`のバージョン          |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```

## Fill

`fill` モジュールは行の余分なスペースを記号で埋めます。 一行に複数の`fill`モジュールが存在する場合、それらはスペースを均等に分割します。 これは、他のモジュールの位置合わせに便利です。

### オプション

| オプション      | デフォルト          | 説明                  |
| ---------- | -------------- | ------------------- |
| `symbol`   | `'.'`          | 行を埋めるために使う記号        |
| `style`    | `'bold black'` | モジュールのスタイルです。       |
| `disabled` | `false`        | `fill`モジュールを無効にします。 |

### 設定例

```toml
# ~/.config/starship.toml
format = 'AA $fill BB $fill CC'

[fill]
symbol = '-'
style = 'bold green'
```

このような出力になります:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Fossil Branch

`fossil_branch`モジュールは、現在のディレクトリにあるチェックアウトのアクティブなブランチ名を表示します。

### オプション

| オプション               | デフォルト                            | 説明                                                    |
| ------------------- | -------------------------------- | ----------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | module のフォーマットです。 現在のブランチ名を参照するには、`'$branch'` を使用します。 |
| `symbol`            | `' '`                           | 現在のディレクトリのチェックアウトのブランチ名の前に使用されるシンボルです。                |
| `style`             | `'bold purple'`                  | モジュールのスタイルです。                                         |
| `truncation_length` | `2^63 - 1`                       | Fossil のブランチ名を `N` 書記素までで切り捨てます。                      |
| `truncation_symbol` | `'…'`                            | ブランチ名が切り捨てられていることを示すための記号です。 `''` で記号なしにできます。         |
| `disabled`          | `true`                           | `fossil_branch` モジュールを無効にします。                         |

### 変数

| 変数        | 設定例     | 説明                      |
| --------- | ------- | ----------------------- |
| branch    | `trunk` | アクティブな Fossil ブランチ      |
| symbol    |         | オプション `symbol` の値をミラーする |
| style\* |         | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```

## Fossil Metrics

`fossil_branch` モジュールは、現在のディレクトリのチェックアウトにおける追加・削除された行数を表示します。 少なくとも Fossil バージョン 2.14 (2021-01-20) が必要です。

### オプション

| オプション                | デフォルト                                                        | 説明                             |
| -------------------- | ------------------------------------------------------------ | ------------------------------ |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | module のフォーマットです。              |
| `added_style`        | `'bold green'`                                               | 追加行数のスタイルです。                   |
| `deleted_style`      | `'bold red'`                                                 | 削除行数のスタイルです。                   |
| `only_nonzero_diffs` | `true`                                                       | 変更された項目についてのみステータスを表示します。      |
| `disabled`           | `true`                                                       | `fossil_metrics` モジュールを無効にします。 |

### 変数

| 変数                | 設定例 | 説明                              |
| ----------------- | --- | ------------------------------- |
| added             | `1` | 現在の追加行数です。                      |
| deleted           | `2` | 現在の削除行数です。                      |
| added_style\*   |     | オプション `added_style` の値を反映します。   |
| deleted_style\* |     | オプション `deleted_style` の値を反映します。 |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[fossil_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Google Cloud (`gcloud`)

`gcloud` モジュールは、 [`gcloud`](https://cloud.google.com/sdk/gcloud) CLIの現在の設定が表示されます。 これは `~/.config/gcloud/active_config` ファイルと `~/.config/gcloud/configurations/config_{CONFIG NAME}` ファイルと `CLOUDSDK_CONFIG` 環境変数に基づきます。

`detect_env_vars` が設定されていない限り、モジュールが有効になっているときは常に活性化します。<0>detect_env_vars</0> が設定されている場合、何れかの環境変数が設定されているときにのみ活性化します。

### オプション

| オプション             | デフォルト                                                      | 説明                            |
| ----------------- | ---------------------------------------------------------- | ----------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | module のフォーマットです。             |
| `symbol`          | `'☁️  '`                                                   | 現在のGCPプロファイルを表示する前に表示される記号です。 |
| `region_aliases`  | `{}`                                                       | GCP名に加えて表示するリージョンのエイリアスです。    |
| `project_aliases` | `{}`                                                       | GCP名に加えて表示するプロジェクトのエイリアスです。   |
| `detect_env_vars` | `[]`                                                       | このモジュールを活性化する環境変数です。          |
| `style`           | `'bold blue'`                                              | モジュールのスタイルです。                 |
| `disabled`        | `false`                                                    | `gcloud`モジュールを無効にします。         |

### 変数

| 変数        | 設定例           | 説明                                              |
| --------- | ------------- | ----------------------------------------------- |
| region    | `us-central1` | 現在のGCPリージョン                                     |
| account   | `foo`         | 現在のGCPプロファイル                                    |
| domain    | `example.com` | 現在のGCPプロファイルのドメイン                               |
| project   |               | 現在のGCPプロジェクト                                    |
| active    | `default`     | `~/.config/gcloud/active_config` に書かれたアクティブな設定名 |
| symbol    |               | オプション `symbol` の値をミラーする                         |
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
format = '[$symbol$active]($style) '
style = 'bold yellow'
```

#### アカウントとエイリアスされたリージョンを表示する

```toml
# ~/.config/starship.toml

[gcloud]
symbol = '️🇬️ '
[gcloud.region_aliases]
us-central1 = 'uc1'
asia-northeast1 = 'an1'
```

#### アカウントとエイリアスされたプロジェクトを表示

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = 'vlpn'
```

## Git Branch

`git_branch`モジュールは、現在のディレクトリにあるリポジトリのアクティブなブランチを表示します。

### オプション

| オプション                | デフォルト                                             | 説明                                                    |
| -------------------- | ------------------------------------------------- | ----------------------------------------------------- |
| `always_show_remote` | `false`                                           | ローカルブランチ名と等しい場合でも、リモート追跡ブランチ名を表示します。                  |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | module のフォーマットです。 現在のブランチ名を参照するには、`'$branch'` を使用します。 |
| `symbol`             | `' '`                                            | gitブランチのシンボルを表すフォーマット文字列。                             |
| `style`              | `'bold purple'`                                   | モジュールのスタイルです。                                         |
| `truncation_length`  | `2^63 - 1`                                        | gitブランチ名を `N` 書記素までで切り捨てます。                           |
| `truncation_symbol`  | `'…'`                                             | ブランチ名が切り捨てられていることを示すための記号です。 `''` で記号なしにできます。         |
| `only_attached`      | `false`                                           | デタッチ `HEAD` 状態にない時はブランチ名のみ表示します。                      |
| `ignore_branches`    | `[]`                                              | 表示しない名前のリスト。 'master' や 'main' に対して有用です。              |
| `disabled`           | `false`                                           | `git_branch`モジュールを無効にします。                             |

### 変数

| 変数            | 設定例      | 説明                                                         |
| ------------- | -------- | ---------------------------------------------------------- |
| branch        | `master` | 現在のブランチがない場合は、現在のブランチ名は`HEAD`に戻ります(例: git detached `HEAD`) |
| remote_name   | `origin` | リモート名                                                      |
| remote_branch | `master` | `remote_name`で追跡されたブランチ名                                   |
| symbol        |          | オプション `symbol` の値をミラーする                                    |
| style\*     |          | オプション `style` の値をミラーする                                     |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[git_branch]
symbol = '🌱 '
truncation_length = 4
truncation_symbol = ''
ignore_branches = ['master', 'main']
```

## Git コミット

`git_commit` モジュールは、カレントディレクトリのリポジトリの現在のコミットハッシュとタグ (もしあれば) を表示します。

### オプション

| オプション                | デフォルト                          | 説明                                        |
| -------------------- | ------------------------------ | ----------------------------------------- |
| `commit_hash_length` | `7`                            | 表示される git コミットハッシュの長さ。                    |
| `format`             | `'[\($hash$tag\)]($style) '` | module のフォーマットです。                         |
| `style`              | `'bold green'`                 | モジュールのスタイルです。                             |
| `only_detached`      | `true`                         | detached `HEAD` 状態のときのみ git コミットハッシュを表示する |
| `tag_disabled`       | `true`                         | `git_commit` モジュールのタグ情報の表示を無効にする。         |
| `tag_max_candidates` | `0`                            | タグ表示で考慮するコミットの数。 既定では完全一致のみ許可します。         |
| `tag_symbol`         | `' 🏷  '`                       | 表示される情報の前に追加されるタグシンボル                     |
| `disabled`           | `false`                        | `git_commit` モジュールを無効にします。                |

### 変数

| 変数        | 設定例       | 説明                     |
| --------- | --------- | ---------------------- |
| hash      | `b703eb3` | 現在の git コミットハッシュ       |
| tag       | `v1.0.0`  | タグ情報の表示が有効の場合のタグ名。     |
| style\* |           | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## Git State

`git_state`モジュールはgitディレクトリの進行状態を表します。 (例: _REBASING_, _BISECTING_, その他) 進行状況の情報がある場合は (例:REBASING 3/10)、その情報も表示されます。

### オプション

| オプション          | デフォルト                                                           | 説明                                                       |
| -------------- | --------------------------------------------------------------- | -------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                    | `rebase`進行中に表示されるフォーマット文字列です。                            |
| `merge`        | `'MERGING'`                                                     | `merge`進行中に表示されるフォーマット文字列です。                             |
| `revert`       | `'REVERTING'`                                                   | `revert`進行中に表示されるフォーマット文字列です。                            |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | `cherry-pick`進行中に表示されるフォーマット文字列です。                       |
| `bisect`       | `'BISECTING'`                                                   | `bisect`進行中に表示されるフォーマット文字列です。                            |
| `am`           | `'AM'`                                                          | `apply-mailbox` (`git am`) 進行中に表示されるフォーマット文字列です。         |
| `am_or_rebase` | `'AM/REBASE'`                                                   | あいまいな`apply-mailbox`または`rebase`が進行中のときに表示されるフォーマット文字列です。 |
| `style`        | `'bold yellow'`                                                 | モジュールのスタイルです。                                            |
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
cherry_pick = '[🍒 PICKING](bold red)'
```

## Git Metrics

`git_metrics` モジュールは、現在の git リポジトリにおける追加・削除された行数を表示します。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション                | デフォルト                                                        | 説明                          |
| -------------------- | ------------------------------------------------------------ | --------------------------- |
| `added_style`        | `'bold green'`                                               | 追加行数のスタイルです。                |
| `deleted_style`      | `'bold red'`                                                 | 削除行数のスタイルです。                |
| `only_nonzero_diffs` | `true`                                                       | 変更された項目についてのみステータスを表示します。   |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | module のフォーマットです。           |
| `disabled`           | `true`                                                       | `git_metrics` モジュールを無効にします。 |
| `ignore_submodules`  | `false`                                                      | Git サブモジュールの変更を無視します。       |

### 変数

| 変数                | 設定例 | 説明                              |
| ----------------- | --- | ------------------------------- |
| added             | `1` | 現在の追加行数です。                      |
| deleted           | `2` | 現在の削除行数です。                      |
| added_style\*   |     | オプション `added_style` の値を反映します。   |
| deleted_style\* |     | オプション `deleted_style` の値を反映します。 |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
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
| `conflicted`        | `'='`                                           | このブランチにはマージの競合があります。                                                   |
| `ahead`             | `'⇡'`                                           | `ahead`のフォーマット                                                         |
| `behind`            | `'⇣'`                                           | `behind`のフォーマット                                                        |
| `diverged`          | `'⇕'`                                           | `diverged`のフォーマット                                                      |
| `up_to_date`        | `''`                                            | `up_to_date`のフォーマット                                                    |
| `untracked`         | `'?'`                                           | `untracked`のフォーマット                                                     |
| `stashed`           | `'$'`                                           | `stashed`のフォーマット                                                       |
| `modified`          | `'!'`                                           | `modified`のフォーマット                                                      |
| `staged`            | `'+'`                                           | `staged`のフォーマット                                                        |
| `renamed`           | `'»'`                                           | `renamed`のフォーマット                                                       |
| `deleted`           | `'✘'`                                           | `deleted`のフォーマット                                                       |
| `typechanged`       | `""`                                            | The format of `typechanged`                                            |
| `style`             | `'bold red'`                                    | モジュールのスタイルです。                                                          |
| `ignore_submodules` | `false`                                         | サブモジュールの変更を無視します。                                                      |
| `disabled`          | `false`                                         | `git_status`モジュールを無効にします。                                              |
| `windows_starship`  |                                                 | WSLでWindowsディレクトリの`git_status`で使用するWindows Starshipの実行ファイルのLinux上でのパス。 |

### 変数

` format` 内では以下の変数が利用できます。

| 変数             | 説明                                                                                   |
| -------------- | ------------------------------------------------------------------------------------ |
| `all_status`   | `$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked` のショートカット |
| `ahead_behind` | 現在のリポジトリに応じてフォーマット文字列 `diverged`, `ahead`, `behind`, `up_to_date` の何れかを表示します。        |
| `conflicted`   | このブランチにマージコンフリクトがある場合、 `conflicted` を表示します。                                          |
| `untracked`    | 作業ディレクトリに追跡されていないファイルがある場合、 `untracked` を表示します。                                      |
| `stashed`      | Stash がローカルリポジトリに存在する場合、 `stashed` を表示します。                                           |
| `modified`     | 作業ディレクトリのファイルに変更がある場合に、 `modified` を表示します。                                           |
| `staged`       | インデックスに新しく追加されたファイルがあるときに、 `staged` を表示します。                                          |
| `renamed`      | インデックスに名前が変更されたファイルがあるときに、 `renamed` を表示します。                                         |
| `deleted`      | インデックスに削除されたファイルがあるときに、 `deleted` を表示します。                                            |
| `typechanged`  | Displays `typechanged` when a file's type has been changed in the staging area.      |
| style\*      | オプション `style` の値をミラーする                                                               |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

` diverged` 内では以下の変数が利用できます。

| 変数             | 説明                        |
| -------------- | ------------------------- |
| `ahead_count`  | 追跡対象のブランチよりこちらが進んでいるコミット数 |
| `behind_count` | 追跡対象のブランチよりこちらが遅れているコミット数 |

`conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` および `deleted` の中で以下の変数が使えます:

| 変数      | 説明            |
| ------- | ------------- |
| `count` | ファイルの数を表示します。 |

### 設定例

```toml
# ~/.config/starship.toml

[git_status]
conflicted = '🏳'
ahead = '🏎💨'
behind = '😰'
diverged = '😵'
up_to_date = '✓'
untracked = '🤷'
stashed = '📦'
modified = '📝'
staged = '[++\($count\)](green)'
renamed = '👅'
deleted = '🗑'
```

以下は、追跡対象のブランチと比べて進んでいる・遅れているコミット数を表示します。

```toml
# ~/.config/starship.toml

[git_status]
ahead = '⇡${count}'
diverged = '⇕⇡${ahead_count}⇣${behind_count}'
behind = '⇣${count}'
```

以下は WSL で Windows のパスにある Windows 用の Starship 実行ファイルを使用します。

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/username/scoop/apps/starship/current/starship.exe'
```

## Gleam

`gleam` モジュールは、現在インストールされている[Gleam](https://gleam.run/) のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- 現在のディレクトリに`gleam.toml`ファイルが含まれている
- 現在のディレクトリに拡張子が `.gleam` のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'⭐ '`                               | A format string representing the symbol of Gleam.      |
| `detect_extensions` | `['gleam']`                          | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['gleam.toml']`                     | どのファイル名がこのモジュールをアクティブにするか                              |
| `style`             | `'bold #FFAFF3'`                     | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `gleam` モジュールを無効にします                                   |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v1.0.0` | `gleam` のバージョン          |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[gleam]
format = 'via [⭐ $version](bold red) '
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

| オプション               | デフォルト                                                                                     | 説明                                                                 |
| ------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | module のフォーマットです。                                                  |
| `version_format`    | `'v${raw}'`                                                                               | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。             |
| `symbol`            | `'🐹 '`                                                                                    | Go のシンボルを表すフォーマット文字列                                               |
| `detect_extensions` | `['go']`                                                                                  | どの拡張子がこのモジュールをアクティブにするか                                            |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | どのファイル名がこのモジュールをアクティブにするか                                          |
| `detect_folders`    | `['Godeps']`                                                                              | どのフォルダーがこのモジュールをアクティブにするか                                          |
| `style`             | `'bold cyan'`                                                                             | モジュールのスタイルです。                                                      |
| `not_capable_style` | `'bold red'`                                                                              | go.mod ファイル中の go ディレクティブがインストールされている Go のバージョンと一致しないときのモジュールのスタイル。 |
| `disabled`          | `false`                                                                                   | `golang`モジュールを無効にします。                                              |

### 変数

| 変数          | 設定例       | 説明                                                                                |
| ----------- | --------- | --------------------------------------------------------------------------------- |
| version     | `v1.12.1` | `go` のバージョン                                                                       |
| mod_version | `1.16`    | `go.mod` の go ディレクティブに設定されている `go` のバージョン要件 バージョン要件が `go` バージョンと一致しない場合にのみ表示されます。 |
| symbol      |           | オプション `symbol` の値をミラーする                                                           |
| style\*   |           | オプション `style` の値をミラーする                                                            |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

### `mod_version` を使用する

```toml
# ~/.config/starship.toml

[golang]
format = 'via [$symbol($version )($mod_version )]($style)'
```

## Guix-shell

`guix_shell`モジュールは、 [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) 環境を表示します。 このモジュールは、 guix-shell 環境内にあるときに表示されます。

### オプション

| オプション      | デフォルト                      | 説明                            |
| ---------- | -------------------------- | ----------------------------- |
| `format`   | `'via [$symbol]($style) '` | module のフォーマットです。             |
| `symbol`   | `'🐃 '`                     | Guix-shell のシンボルを表すフォーマット文字列。 |
| `style`    | `'yellow bold'`            | モジュールのスタイルです。                 |
| `disabled` | `false`                    | `guix_shell` モジュールを無効にします。    |

### 変数

| 変数        | 設定例 | 説明                      |
| --------- | --- | ----------------------- |
| symbol    |     | オプション `symbol` の値をミラーする |
| style\* |     | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

`gradle` モジュールは、プロジェクトディレクトリで現在使用されている [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) のバージョンを表示します。

デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに `gradle/wrapper/gradle-wrapper.properties` ディレクトリが含まれている。
- カレントディレクトリに  `.gradle` または `.gradle.kts` で終わるファイルが含まれている。

`gradle` モジュールは、設定ファイルからしか Gradle Wrapper バージョンを読み取れません。セキュリティー上の理由で、 Starship がラッパーを実行することはありません。

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🅶 '`                               | Gradle のシンボルを表すフォーマット文字列                               |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `['gradle']`                         | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold bright-cyan'`                 | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `gradle` モジュールを無効にします。                                 |
| `recursive`         | `false`                              | `gradle` ディレクトリの再帰的な検索を有効にします。                         |

### 変数

| 変数      | 設定例      | 説明                      |
| ------- | -------- | ----------------------- |
| version | `v7.5.1` | `gradle`のバージョン          |
| symbol  |          | オプション `symbol` の値をミラーする |
| style*  |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

## Haskell

`haskell` モジュールは、現在選択されている GHC バージョンおよび選択されている Stack スナップショットを特定します。

デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`stack.yaml`ファイルが含まれている
- カレントディレクトリに`.hs`、`.cabal`または`.hs-boot`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                         |
| ------------------- | ------------------------------------ | -------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。          |
| `symbol`            | `'λ '`                               | Haskell の記号を表すフォーマット文字列です。 |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | どの拡張子がこのモジュールをアクティブにするか    |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | どのファイル名がこのモジュールをアクティブにするか  |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか  |
| `style`             | `'bold purple'`                      | モジュールのスタイルです。              |
| `disabled`          | `false`                              | `haskell` モジュールを無効にします。    |

### 変数

| 変数             | 設定例         | 説明                                                                   |
| -------------- | ----------- | -------------------------------------------------------------------- |
| version        |             | 現在のプロジェクトが Stack プロジェクトかどうかに応じて `ghc_version` または `snapshot` を反映します。 |
| snapshot       | `lts-18.12` | 現在選択されている Stack スナップショットです。                                          |
| ghc\_version | `9.2.1`     | 現在インストールされている GHC バージョンです。                                           |
| symbol         |             | オプション `symbol` の値をミラーします。                                            |
| style\*      |             | オプション `style` の値をミラーします。                                             |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

## Haxe

`haxe` モジュールは、現在インストールされている[Haxe](https://haxe.org/) のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- 現在のディレクトリが  `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json`, `.haxerc` の何れかのファイルを含んでいる。
- 現在のディレクトリが `.haxelib` または `haxe_library` のディレクトリを含んでいる。
- 現在のディレクトリに拡張子が `.hx` または `.hxml`のファイルが含まれている。

### オプション

| オプション               | デフォルト                                                                                           | 説明                                                     |
| ------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                            | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                                                                                     | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `'⌘ '`                                                                                          | A format string representing the symbol of Haxe.       |
| `style`             | `'bold fg:202'`                                                                                 | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                                         | `haxe`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v4.2.5` | `haxe` のバージョン           |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

`helm`モジュールは、現在インストールされている[Helm](https://helm.sh/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`helmfile.yaml`ファイルが含まれている
- カレントディレクトリに`Chart.yaml`ファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `[]`                                 | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `'⎈ '`                               | Helm のシンボルを表すフォーマット文字列                                 |
| `style`             | `'bold white'`                       | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `helm` モジュールを無効にします。                                   |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v3.1.1` | `helm` のバージョン           |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## ホスト名

`hostname`モジュールには、システムのホスト名が表示されます。

### オプション

| オプション             | デフォルト                                  | 説明                                                                                                                         |
| ----------------- | -------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`        | `true`                                 | SSHセッションに接続されている場合にのみホスト名を表示します。                                                                                           |
| `ssh_symbol`      | `'🌐 '`                                 | SSH セッションに接続しているときのシンボルを表すフォーマット文字列。                                                                                       |
| `trim_at`         | `'.'`                                  | この文字が最初にマッチするまでをホスト名と認識します。 `'.'` は最初の . 以降の文字列を切り捨てます。 `''`を指定した場合、文字列の切り捨ては行われません。                                       |
| `detect_env_vars` | `[]`                                   | このモジュールを活性化する環境変数。                                                                                                         |
| `format`          | `'[$ssh_symbol$hostname]($style) in '` | module のフォーマットです。                                                                                                          |
| `style`           | `'bold dimmed green'`                  | モジュールのスタイルです。                                                                                                              |
| `disabled`        | `false`                                | `hostname`モジュールを無効にします。                                                                                                    |
| `aliases`         | `{}`                                   | Translate system hostnames to something else. If `trim_at` is specified, only the first part will be matched and replaced. |

### 変数

| 変数         | 設定例        | 説明                       |
| ---------- | ---------- | ------------------------ |
| hostname   | `computer` | コンピュータのホスト名です。           |
| style\*  |            | オプション `style` の値をミラーします。 |
| ssh_symbol | `'🌏 '`     | SSHセッションに接続していることを表すシンボル |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

#### ホスト名を常に表示する

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

#### リモートのtmuxセッションでホスト名を非表示にする

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
detect_env_vars = ['!TMUX', 'SSH_CONNECTION']
disabled = false
```

#### Replace the hostname with a nickname

```toml
# ~/.config/starship.toml
[hostname]
aliases = { "Max's MacBook Pro" = "home" }
```

## Java

`Java`モジュールは、現在インストールされている[Java](https://www.oracle.com/java/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- 現在のディレクトリに `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot`, `.sdkmanrc` の何れかのファイルが含まれている
- カレントディレクトリに拡張子が`.java`、`.class`、`.gradle`、`.jar`、`.clj`または`.cljc`のファイルが含まれている

### オプション

| オプション               | デフォルト                                                                                                                 | 説明                                                     |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                                                                                                           | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                                                                  | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `'☕ '`                                                                                                                | Java の記号を表すフォーマット文字列です。                                |
| `style`             | `'red dimmed'`                                                                                                        | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                                                               | `java`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例   | 説明                      |
| --------- | ----- | ----------------------- |
| version   | `v14` | `java` のバージョン           |
| symbol    |       | オプション `symbol` の値をミラーする |
| style\* |       | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## ジョブ

`jobs`モジュールには、実行中のジョブの現在の数が表示されます。 このモジュールは、実行中のバックグラウンドジョブがある場合にのみ表示されます。 設定変数 `number_threshold` が存在すればその値以上、さもなければ2つ以上のジョブがある場合に、実行中のジョブの数を表示します。 1つ以上のジョブがある、設定変数 `symbol_threshold` が存在すればその数以上、さもなければ1つ以上のジョブがある場合に、シンボルを表示します。 ジョブがない時も含めて_常_にシンボルとジョブ数を表示するには、両方に 0 を設定します。

デフォルトの機能は次のとおりです。

- 0個のジョブ -> 何も表示しません。
- 1個のジョブ -> `symbol` を表示します。
- 2個以上のジョブ-> `symbol` + `number` を表示します。

::: warning

このモジュールは tcsh と nu ではサポートされません。

:::

::: warning

オプション `threshold` は非推奨になりましたが、`threshold` が指定されている場合には、モジュールは走っているジョブの数を表示します。 `threshold` が 0 に設定されている場合は、ジョブが走っていない場合にもモジュールが表示されます。

:::

### オプション

| オプション              | デフォルト                         | 説明                                                     |
| ------------------ | ----------------------------- | ------------------------------------------------------ |
| `threshold`*       | `1`                           | 超過した場合、ジョブの数を表示します。                                    |
| `symbol_threshold` | `1`                           | ジョブの数が少なくとも `symbol_threshold` ある場合に  `symbol` を表示します。 |
| `number_threshold` | `2`                           | ジョブの数が少なくとも `number_threshold` ある場合に、ジョブ数を表示します。       |
| `format`           | `'[$symbol$number]($style) '` | module のフォーマットです。                                      |
| `symbol`           | `'✦'`                         | `symbol` 変数を表すために使用される文字列。                             |
| `style`            | `'bold blue'`                 | モジュールのスタイルです。                                          |
| `disabled`         | `false`                       | `jobs`モジュールを無効にします。                                    |

*: このオプションは非推奨です。代わりに `number_threshold` と `symbol_threshold` オプションを指定してください。

### 変数

| 変数        | 設定例 | 説明                      |
| --------- | --- | ----------------------- |
| number    | `1` | ジョブの数                   |
| symbol    |     | オプション `symbol` の値をミラーする |
| style\* |     | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
symbol_threshold = 0
```

## Julia

`julia`モジュールは、現在インストールされている[Julia](https://julialang.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Project.toml`ファイルが含まれている
- カレントディレクトリに`Manifest.toml`ファイルが含まれている
- カレントディレクトリに`.jl`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `['jl']`                             | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `'ஃ '`                               | Juliaのシンボルを表すフォーマット文字列                                 |
| `style`             | `'bold purple'`                      | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `julia`モジュールを無効にします。                                   |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v1.4.0` | `julia`のバージョン           |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

`kotlin`モジュールは、現在インストールされている[Kotlin](https://kotlinlang.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.kt`もしくは`.kts`ファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `['kt', 'kts']`                      | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `'🅺 '`                               | Kotlinのシンボルを表すフォーマット文字列                                |
| `style`             | `'bold blue'`                        | モジュールのスタイルです。                                          |
| `kotlin_binary`     | `'kotlin'`                           | Starshipがバージョンを取得するときに実行するkotlinバイナリを設定します。            |
| `disabled`          | `false`                              | `kotlin`モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例       | 説明                      |
| --------- | --------- | ----------------------- |
| version   | `v1.4.21` | `kotlin`のバージョン          |
| symbol    |           | オプション `symbol` の値をミラーする |
| style\* |           | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[kotlin]
symbol = '🅺 '
```

```toml
# ~/.config/starship.toml

[kotlin]
# Kotlinコンパイラバイナリを使ってバージョンを確認する
kotlin_binary = 'kotlinc'
```

## Kubernetes

現在の[Kubernetes コンテキスト](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context)名を表示します。 kubeconfigファイルに設定されている場合は、名前空間、ユーザー、クラスターも表示します。 名前空間は kubeconfig ファイルの中で設定する必要があります。次のようにして行います: `kubectl config set-context starship-context --namespace astronaut`。 同様に、ユーザーとクラスターは `kubectl config set-context starship-context --user starship-user` と `kubectl config set-context starship-context ---cluster starship-cluster` で設定できます。 環境変数 `$KUBECONFIG` が設定されている場合、このモジュールはそれを利用し、 `~/.kube/config` を利用しません。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

モジュールが有効化されているとき、`detect_env_vars`, `detect_extensions`, `detect_files`, `detect_folders` の何れかのオプションが設定されていない限りモジュールは常に表示されます。これらのオプションが設定されている場合は、対応する条件が満たされるディレクトリ内にいるときまたは環境変数が存在するときに、モジュールが表示されます。

:::

### オプション

::: warning

`context_aliases` と `user_aliases` オプションは非推奨になりました。 代わりに `contexts` と対応するオプション `context_alias` と `user_alias` をお使いください。

:::

| オプション               | デフォルト                                                | 説明                              |
| ------------------- | ---------------------------------------------------- | ------------------------------- |
| `symbol`            | `'☸ '`                                               | クラスター名の前に表示されるシンボルを表すフォーマット文字列。 |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | モジュールのフォーマットです。                 |
| `style`             | `'cyan bold'`                                        | モジュールのスタイルです。                   |
| `context_aliases`*  | `{}`                                                 | 表示するコンテキストエイリアスを定義するテーブル。       |
| `user_aliases`*     | `{}`                                                 | 表示するユーザーエイリアスを定義するテーブル。         |
| `detect_extensions` | `[]`                                                 | どの拡張子がこのモジュールをアクティブにするか         |
| `detect_files`      | `[]`                                                 | どのファイル名がこのモジュールをアクティブにするか       |
| `detect_folders`    | `[]`                                                 | どのフォルダーがこのモジュールをアクティブにするか       |
| `detect_env_vars`   | `[]`                                                 | このモジュールを活性化する環境変数です。            |
| `contexts`          | `[]`                                                 | 特定のコンテキストのカスタマイズされたスタイルとシンボルです。 |
| `disabled`          | `true`                                               | `kubernetes` モジュールを無効にする。       |

*: このオプションは非推奨になりました。代わりに `contexts` を対応するオプション `context_alias` と `user_alias` と一緒に追加してください。

特定の環境におけるモジュールのスタイルを変更するには、以下の設定をリスト `contexts` に含めます。

| 変数                | 説明                                                                                       |
| ----------------- | ---------------------------------------------------------------------------------------- |
| `context_pattern` | **Required** Regular expression to match current Kubernetes context name.                |
| `user_pattern`    | Regular expression to match current Kubernetes user name.                                |
| `context_alias`   | Context alias to display instead of the full context name.                               |
| `user_alias`      | User alias to display instead of the full user name.                                     |
| `style`           | The style for the module when using this context. If not set, will use module's style.   |
| `symbol`          | The symbol for the module when using this context. If not set, will use module's symbol. |

Note that all regular expression are anchored with `^<pattern>$` and so must match the whole string. The `*_pattern` regular expressions may contain capture groups, which can be referenced in the corresponding alias via `$name` and `$N` (see example below and the [rust Regex::replace() documentation](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)).

### 変数

| 変数        | 設定例                  | 説明                               |
| --------- | -------------------- | -------------------------------- |
| context   | `starship-context`   | 現在の Kubernetes のコンテキスト名          |
| namespace | `starship-namespace` | 設定されている場合、現在の Kubernetes の名前空間名  |
| user      | `starship-user`      | 設定されている場合、現在の Kubernetes のユーザー名  |
| cluster   | `starship-cluster`   | 設定されている場合、現在の Kubernetes のクラスター名 |
| symbol    |                      | オプション `symbol` の値をミラーする          |
| style\* |                      | オプション `style` の値をミラーする           |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
contexts = [
  { context_pattern = "dev.local.cluster.k8s", style = "green", symbol = "💔 " },
]
```

以下は `k8s` ファイルを含むディレクトリの中でのみモジュールを表示します。

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Kubernetes Context specific config

The `contexts` configuration option is used to customise what the current Kubernetes context name looks like (style and symbol) if the name matches the defined regular expression.

```toml
# ~/.config/starship.toml

[[kubernetes.contexts]]
# "bold red" style + default symbol when Kubernetes current context name equals "production" *and* the current user
# equals "admin_user"
context_pattern = "production"
user_pattern = "admin_user"
style = "bold red"
context_alias = "prod"
user_alias = "admin"

[[kubernetes.contexts]]
# "green" style + a different symbol when Kubernetes current context name contains openshift
context_pattern = ".*openshift.*"
style = "green"
symbol = "💔 "
context_alias = "openshift"

[[kubernetes.contexts]]
# Using capture groups
# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
context_pattern = "gke_.*_(?P<cluster>[\\w-]+)"
context_alias = "gke-$cluster"
```

## Line Break

`line_break` モジュールは、プロンプトを2行に分割します。

### オプション

| オプション      | デフォルト   | 説明                                     |
| ---------- | ------- | -------------------------------------- |
| `disabled` | `false` | `line_break` モジュールを無効にして、プロンプトを1行にします。 |

### 設定例

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## ローカルIP

`localip`モジュールは、プライマリネットワークインターフェイスのIPv4アドレスを表示します。

### オプション

| オプション      | デフォルト                     | 説明                                  |
| ---------- | ------------------------- | ----------------------------------- |
| `ssh_only` | `true`                    | SSHセッションに接続されている場合にのみ、IPアドレスを表示します。 |
| `format`   | `'[$localipv4]($style) '` | module のフォーマットです。                   |
| `style`    | `'bold yellow'`           | モジュールのスタイルです。                       |
| `disabled` | `true`                    | `localip`モジュールを無効にします。              |

### 変数

| 変数        | 設定例          | 説明                     |
| --------- | ------------ | ---------------------- |
| localipv4 | 192.168.1.13 | プライマリIPv4アドレスが含まれています  |
| style\* |              | オプション `style` の値をミラーする |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

`lua`モジュールは、現在インストールされている[Lua](http://www.lua.org/) のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.lua-version`ファイルが含まれている
- カレントディレクトリに`lua`ディレクトリが含まれている
- カレントディレクトリに`.lua`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🌙 '`                               | Luaのシンボルを表すフォーマット文字列                                   |
| `detect_extensions` | `['lua']`                            | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['.lua-version']`                   | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `['lua']`                            | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold blue'`                        | モジュールのスタイルです。                                          |
| `lua_binary`        | `'lua'`                              | Starshipがバージョンを取得するときに実行するLuaバイナリを設定します。               |
| `disabled`          | `false`                              | `lua`モジュールを無効にします。                                     |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v5.4.0` | `lua` のバージョン            |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## メモリ使用量

`memory_usage` モジュールは、現在のシステムメモリとスワップ使用量を示します。

デフォルトでは、システムスワップの合計がゼロ以外の場合、スワップ使用量が表示されます。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション       | デフォルト                                           | 説明                          |
| ----------- | ----------------------------------------------- | --------------------------- |
| `threshold` | `75`                                            | この閾値を超えない限り、メモリ使用率は表示されません。 |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | module のフォーマットです。           |
| `symbol`    | `'🐏'`                                           | メモリ使用率を表示する前に使用される記号です。     |
| `style`     | `'bold dimmed white'`                           | モジュールのスタイルです。               |
| `disabled`  | `true`                                          | `memory_usage`モジュールを無効にします。 |

### 変数

| 変数               | 設定例           | 説明                                                                 |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | オプション `symbol` の値をミラーする                                            |
| style\*        |               | オプション `style` の値をミラーする                                             |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

### 設定例

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = ' '
style = 'bold dimmed green'
```

## Meson

The `meson` module shows the current Meson developer environment status.

By default the Meson project name is displayed, if `$MESON_DEVENV` is set.

### オプション

| オプション               | デフォルト                              | 説明                                                                         |
| ------------------- | ---------------------------------- | -------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                 |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. `''` で記号なしにできます。 |
| `format`            | `'via [$symbol$project]($style) '` | module のフォーマットです。                                                          |
| `symbol`            | `'⬢ '`                             | The symbol used before displaying the project name.                        |
| `style`             | `'blue bold'`                      | モジュールのスタイルです。                                                              |
| `disabled`          | `false`                            | Disables the `meson` module.                                               |

### 変数

| 変数        | 設定例        | 説明                             |
| --------- | ---------- | ------------------------------ |
| project   | `starship` | The current Meson project name |
| symbol    | `🐏`        | オプション `symbol` の値をミラーする        |
| style\* |            | オプション `style` の値をミラーする         |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = '--'
symbol = ' '
style = 'bold dimmed green'
```

## Mercurial Branch

The `hg_branch` module shows the active branch and topic of the repo in your current directory.

### オプション

| オプション               | デフォルト                                     | 説明                                                                                           |
| ------------------- | ----------------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `' '`                                    | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `'bold purple'`                           | モジュールのスタイルです。                                                                                |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | module のフォーマットです。                                                                            |
| `truncation_length` | `2^63 - 1`                                | Truncates the hg branch / topic name to `N` graphemes                                        |
| `truncation_symbol` | `'…'`                                     | ブランチ名が切り捨てられていることを示すための記号です。                                                                 |
| `disabled`          | `true`                                    | Disables the `hg_branch` module.                                                             |

### 変数

| 変数        | 設定例       | 説明                          |
| --------- | --------- | --------------------------- |
| branch    | `master`  | The active mercurial branch |
| topic     | `feature` | The active mercurial topic  |
| symbol    |           | オプション `symbol` の値をミラーする     |
| style\* |           | オプション `style` の値をミラーする      |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Mojo

The `mojo` module shows the current version of [Mojo programming language](https://www.modular.com/mojo) installed

### オプション

| オプション               | デフォルト                                 | 説明                                                     |
| ------------------- | ------------------------------------- | ------------------------------------------------------ |
| `format`            | `'with [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `symbol`            | `'🔥 '`                                | The symbol used before displaying the version of Mojo. |
| `style`             | `'bold 208'`                          | モジュールのスタイルです。                                          |
| `disabled`          | `false`                               | Disables the `mojo` module.                            |
| `detect_extensions` | `['mojo', '🔥']`                       | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                  | どのフォルダーがこのモジュールをアクティブにするか                              |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `24.4.0` | The version of `mojo`   |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[mojo]
format = 'via [mojo ($version )($hash )]($style)'
```

## NATS

The `nats` module shows the name of the current [NATS](https://nats.io) context.

### オプション

| オプション      | デフォルト                      | 説明                                                           |
| ---------- | -------------------------- | ------------------------------------------------------------ |
| `symbol`   | `'✉️ '`                    | The symbol used before the NATS context (defaults to empty). |
| `style`    | `'bold purple'`            | モジュールのスタイルです。                                                |
| `format`   | `'[$symbol$name]($style)'` | module のフォーマットです。                                            |
| `disabled` | `false`                    | Disables the `nats` module.                                  |

### 変数

| 変数        | 設定例         | 説明                           |
| --------- | ----------- | ---------------------------- |
| name      | `localhost` | The name of the NATS context |
| symbol    |             | オプション `symbol` の値をミラーする      |
| style\* |             | オプション `style` の値をミラーする       |

### 設定例

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```

## Nim

`nim`モジュールは、現在インストールされている[Nim](https://nim-lang.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`nim.cfg`ファイルが含まれている
- カレントディレクトリに拡張子が`.nim`のファイルが含まれている
- カレントディレクトリに拡張子が`.nims`のファイルが含まれている
- カレントディレクトリに拡張子が`.nimble`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module                              |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'👑 '`                               | The symbol used before displaying the version of Nim.  |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['nim.cfg']`                        | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold yellow'`                      | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `nim` module.                             |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v1.2.0` | The version of `nimc`   |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

`nix_shell`モジュールは[nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html)環境を表示します。 このモジュールは、nixシェル環境内にあるときに表示されます。

### オプション

| オプション         | デフォルト                                          | 説明                                                                    |
| ------------- | ---------------------------------------------- | --------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | module のフォーマットです。                                                     |
| `symbol`      | `'❄️ '`                                        | A format string representing the symbol of nix-shell.                 |
| `style`       | `'bold blue'`                                  | モジュールのスタイルです。                                                         |
| `impure_msg`  | `'impure'`                                     | A format string shown when the shell is impure.                       |
| `pure_msg`    | `'pure'`                                       | A format string shown when the shell is pure.                         |
| `unknown_msg` | `''`                                           | A format string shown when it is unknown if the shell is pure/impure. |
| `disabled`    | `false`                                        | `nix_shell`モジュールを無効にします。                                              |
| `heuristic`   | `false`                                        | Attempts to detect new `nix shell`-style shells with a heuristic.     |

### 変数

| 変数        | 設定例     | 説明                         |
| --------- | ------- | -------------------------- |
| state     | `pure`  | The state of the nix-shell |
| name      | `lorri` | The name of the nix-shell  |
| symbol    |         | オプション `symbol` の値をミラーする    |
| style\* |         | オプション `style` の値をミラーする     |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = '[impure shell](bold red)'
pure_msg = '[pure shell](bold green)'
unknown_msg = '[unknown shell](bold yellow)'
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

`nodejs`モジュールは、現在インストールされている[Node.js](https://nodejs.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`package.json`ファイルが含まれている
- カレントディレクトリに`.node-version`ファイルが含まれている
- カレントディレクトリに`.nvmrc`ファイルが含まれている
- カレントディレクトリに`node_modules`ディレクトリが含まれている
- カレントディレクトリに拡張子が`.js`、`.mjs`または`.cjs`のファイルが含まれている
- カレントディレクトリに拡張子が`.ts`、`.mts`または`.cts`のファイルが含まれている

### オプション

| オプション               | デフォルト                                         | 説明                                                                                                    |
| ------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | module のフォーマットです。                                                                                     |
| `version_format`    | `'v${raw}'`                                   | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。                                                |
| `symbol`            | `' '`                                        | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | どの拡張子がこのモジュールをアクティブにするか                                                                               |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | どのファイル名がこのモジュールをアクティブにするか                                                                             |
| `detect_folders`    | `['node_modules']`                            | どのフォルダーがこのモジュールをアクティブにするか                                                                             |
| `style`             | `'bold green'`                                | モジュールのスタイルです。                                                                                         |
| `disabled`          | `false`                                       | `nodejs`モジュールを無効にします。                                                                                 |
| `not_capable_style` | `'bold red'`                                  | The style for the module when an engines property in package.json does not match the Node.js version. |

### 変数

| 変数              | 設定例           | 説明                                                                                                                                                        |
| --------------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version         | `v13.12.0`    | The version of `node`                                                                                                                                     |
| engines_version | `>=12.0.0` | `node` version requirement as set in the engines property of `package.json`. Will only show if the version requirement does not match the `node` version. |
| symbol          |               | オプション `symbol` の値をミラーする                                                                                                                                   |
| style\*       |               | オプション `style` の値をミラーする                                                                                                                                    |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

`ocaml`モジュールは、現在インストールされている[OCaml](https://ocaml.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに拡張子`.opam`のファイルまたは`_opam`ディレクトリが含まれている
- カレントディレクトリに`esy.lock`ディレクトリが含まれている
- カレントディレクトリに`dune`または`dune-project`ファイルが含まれている
- カレントディレクトリに`jbuild`または`jbuild-ignore`ファイルが含まれている
- カレントディレクトリに`.merlin`ファイルが含まれている
- カレントディレクトリに拡張子が`.ml`、`.mli`、`.re`または`.rei`のファイルが含まれている

### オプション

| オプション                     | デフォルト                                                                      | 説明                                                      |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------- |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | モジュールのフォーマット文字列。                                        |
| `version_format`          | `'v${raw}'`                                                                | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。  |
| `symbol`                  | `'🐫 '`                                                                     | The symbol used before displaying the version of OCaml. |
| `global_switch_indicator` | `''`                                                                       | The format string used to represent global OPAM switch. |
| `local_switch_indicator`  | `'*'`                                                                      | The format string used to represent local OPAM switch.  |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                       | どの拡張子がこのモジュールをアクティブにするか                                 |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | どのファイル名がこのモジュールをアクティブにするか                               |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                    | どのフォルダーがこのモジュールをアクティブにするか                               |
| `style`                   | `'bold yellow'`                                                            | モジュールのスタイルです。                                           |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                            |

### 変数

| 変数               | 設定例          | 説明                                                                |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | オプション `symbol` の値をミラーする                                           |
| style\*        |              | オプション `style` の値をミラーする                                            |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Odin

The `odin` module shows the currently installed version of [Odin](https://odin-lang.org/). By default the module will be shown if the current directory contains a `.odin` file.

### オプション

| オプション               | デフォルト                                | 説明                                       |
| ------------------- | ------------------------------------ | ---------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                        |
| `show_commit`       | `false`                              | Shows the commit as part of the version. |
| `symbol`            | `'Ø '`                               | Zig のバージョンの前に表示されるシンボルです。                |
| `style`             | `'bold bright-blue'`                 | モジュールのスタイルです。                            |
| `disabled`          | `false`                              | Disables the `odin` module.              |
| `detect_extensions` | `['odin']`                           | どの拡張子がこのモジュールをアクティブにするか                  |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                |

### 変数

| 変数        | 設定例           | 説明                      |
| --------- | ------------- | ----------------------- |
| version   | `dev-2024-03` | The version of `odin`   |
| symbol    |               | オプション `symbol` の値をミラーする |
| style\* |               | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[odin]
format = 'via [󰹩 ($version )]($style)'
show_commit = true
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool. By default the module will be shown if the current directory contains a `.rego` file.

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🪖  '`                              | A format string representing the symbol of OPA.        |
| `detect_extensions` | `['rego']`                           | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold blue'`                        | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `opa` module.                             |

### 変数

| 変数        | 設定例       | 説明                      |
| --------- | --------- | ----------------------- |
| version   | `v0.44.0` | The version of `opa`    |
| symbol    |           | オプション `symbol` の値をミラーする |
| style\* |           | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### オプション

| オプション      | デフォルト                                           | 説明                                                             |
| ---------- | ----------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | module のフォーマットです。                                              |
| `symbol`   | `'☁️ '`                                         | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `'bold yellow'`                                 | モジュールのスタイルです。                                                  |
| `disabled` | `false`                                         | Disables the `openstack` module.                               |

### 変数

| 変数        | 設定例    | 説明                            |
| --------- | ------ | ----------------------------- |
| cloud     | `corp` | The current OpenStack cloud   |
| project   | `dev`  | The current OpenStack project |
| symbol    |        | オプション `symbol` の値をミラーする       |
| style\* |        | オプション `style` の値をミラーする        |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```

## OS

The `os` module shows the current operating system. OS information is detected via the [os_info](https://lib.rs/crates/os_info) crate.

::: warning

The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

:::

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション      | デフォルト                 | 説明                                                     |
| ---------- | --------------------- | ------------------------------------------------------ |
| `format`   | `'[$symbol]($style)'` | module のフォーマットです。                                      |
| `style`    | `'bold white'`        | モジュールのスタイルです。                                          |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type. Operating system types not defined by your configuration use the default symbols table below. All operating systems currently supported by the module are listed below. If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# This is the default symbols table.
[os.symbols]
AIX = "➿ "
Alpaquita = "🔔 "
AlmaLinux = "💠 "
Alpine = "🏔️ "
Amazon = "🙂 "
Android = "🤖 "
Arch = "🎗️ "
Artix = "🎗️ "
CachyOS = "🎗️ "
CentOS = "💠 "
Debian = "🌀 "
DragonFly = "🐉 "
Emscripten = "🔗 "
EndeavourOS = "🚀 "
Fedora = "🎩 "
FreeBSD = "😈 "
Garuda = "🦅 "
Gentoo = "🗜️ "
HardenedBSD = "🛡️ "
Illumos = "🐦 "
Kali = "🐉 "
Linux = "🐧 "
Mabox = "📦 "
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
Nobara = "🎩 "
OpenBSD = "🐡 "
OpenCloudOS = "☁️ "
openEuler = "🦉 "
openSUSE = "🦎 "
OracleLinux = "🦴 "
Pop = "🍭 "
Raspbian = "🍓 "
Redhat = "🎩 "
RedHatEnterprise = "🎩 "
RockyLinux = "💠 "
Redox = "🧪 "
Solus = "⛵ "
SUSE = "🦎 "
Ubuntu = "🎯 "
Ultramarine = "🔷 "
Unknown = "❓ "
Uos = "🐲 "
Void = "  "
Windows = "🪟 "
```

### 変数

| 変数        | 設定例          | 説明                                                                 |
| --------- | ------------ | ------------------------------------------------------------------ |
| symbol    | `🎗️`         | The current operating system symbol from advanced option `symbols` |
| name      | `Arch Linux` | The current operating system name                                  |
| type      | `Arch`       | The current operating system type                                  |
| codename  |              | The current operating system codename, if applicable               |
| edition   |              | The current operating system edition, if applicable                |
| version   |              | The current operating system version, if applicable                |
| style\* |              | オプション `style` の値をミラーする                                             |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[os]
format = "on [($name )]($style)"
style = "bold blue"
disabled = false

[os.symbols]
Windows = " "
Arch = "Arch is the best! "
```

## パッケージのバージョン

`package` モジュールは、現在のディレクトリがパッケージのリポジトリである場合に表示され、現在のバージョンが表示されます。 The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `daml` and `dart` packages.

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

| オプション             | デフォルト                             | 説明                                                        |
| ----------------- | --------------------------------- | --------------------------------------------------------- |
| `format`          | `'is [$symbol$version]($style) '` | module のフォーマットです。                                         |
| `symbol`          | `'📦 '`                            | パッケージのバージョンを表示する前に使用される記号です。                              |
| `version_format`  | `'v${raw}'`                       | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。    |
| `style`           | `'bold 208'`                      | モジュールのスタイルです。                                             |
| `display_private` | `false`                           | Enable displaying version for packages marked as private. |
| `disabled`        | `false`                           | `package` モジュールを無効にします。                                   |

### 変数

| 変数        | 設定例      | 説明                          |
| --------- | -------- | --------------------------- |
| version   | `v1.0.0` | The version of your package |
| symbol    |          | オプション `symbol` の値をミラーする     |
| style\* |          | オプション `style` の値をミラーする      |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[package]
format = 'via [🎁 $version](208 bold) '
```

## Perl

`perl`モジュールは、現在インストールされている[Perl](https://www.perl.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Makefile.PL`または`Build.PL`ファイルが含まれている
- カレントディレクトリに`cpanfile`または`cpanfile.snapshot`ファイルが含まれている
- カレントディレクトリに`META.json`または`META.yml`ファイルが含まれている
- カレントディレクトリに`.perl-version`ファイルが含まれている
- カレントディレクトリに拡張子が`.pl`、`.pm`または`.pod`のファイルが含まれている

### オプション

| オプション               | デフォルト                                                                                                    | 説明                                                     |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | モジュールのフォーマット文字列。                                       |
| `version_format`    | `'v${raw}'`                                                                                              | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🐪 '`                                                                                                   | The symbol used before displaying the version of Perl  |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                                                                                     | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold 149'`                                                                                             | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                            |

### 変数

| 変数        | 設定例       | 説明                      |
| --------- | --------- | ----------------------- |
| version   | `v5.26.1` | The version of `perl`   |
| symbol    |           | オプション `symbol` の値をミラーする |
| style\* |           | オプション `style` の値をミラーする  |

### 設定例

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

`php`モジュールは、現在インストールされている[PHP](https://www.php.net/) のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`composer.json`ファイルが含まれている
- カレントディレクトリに`.php-version`ファイルが含まれている
- カレントディレクトリに拡張子が`.php`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🐘 '`                               | PHPのバージョンを表示する前に使用される記号です。                             |
| `detect_extensions` | `['php']`                            | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['composer.json', '.php-version']`  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'147 bold'`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `php`モジュールを無効にします。                                     |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v7.3.8` | The version of `php`    |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Pijul Channel

The `pijul_channel` module shows the active channel of the repo in your current directory.

### オプション

| オプション               | デフォルト                             | 説明                                                                                   |
| ------------------- | --------------------------------- | ------------------------------------------------------------------------------------ |
| `symbol`            | `' '`                            | The symbol used before the pijul channel name of the repo in your current directory. |
| `style`             | `'bold purple'`                   | モジュールのスタイルです。                                                                        |
| `format`            | `'on [$symbol$channel]($style) '` | module のフォーマットです。                                                                    |
| `truncation_length` | `2^63 - 1`                        | Truncates the pijul channel name to `N` graphemes                                    |
| `truncation_symbol` | `'…'`                             | ブランチ名が切り捨てられていることを示すための記号です。                                                         |
| `disabled`          | `true`                            | Disables the `pijul` module.                                                         |

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

::: tip

By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms). それでも有効にしたい場合は、 [以下の例に従ってください](#with-pulumi-version).

:::

デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Pulumi.yaml`または`Pulumi.yml`ファイルが含まれている
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### オプション

| オプション            | デフォルト                                        | 説明                                                             |
| ---------------- | -------------------------------------------- | -------------------------------------------------------------- |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | モジュールのフォーマット文字列。                                               |
| `version_format` | `'v${raw}'`                                  | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。         |
| `symbol`         | `' '`                                       | A format string shown before the Pulumi stack.                 |
| `style`          | `'bold 5'`                                   | モジュールのスタイルです。                                                  |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories. |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                  |

### 変数

| 変数        | 設定例        | 説明                          |
| --------- | ---------- | --------------------------- |
| version   | `v0.12.24` | The version of `pulumi`     |
| stack     | `dev`      | The current Pulumi stack    |
| username  | `alice`    | The current Pulumi username |
| symbol    |            | オプション `symbol` の値をミラーする     |
| style\* |            | オプション `style` の値をミラーする      |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

#### With Pulumi Version

```toml
# ~/.config/starship.toml

[pulumi]
format = '[🛥 ($version )$stack]($style) '
```

#### Without Pulumi version

```toml
# ~/.config/starship.toml
[pulumi]
symbol = '🛥 '
format = '[$symbol$stack]($style) '
```

## PureScript

`purescript`モジュールは、現在インストールされている[PureScript](https://www.purescript.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`spago.dhall`ファイルが含まれている
- 現在のディレクトリに`spago.yaml`ファイルが含まれている
- 現在のディレクトリに`spago.lock`ファイルが含まれている
- カレントディレクトリに拡張子が`.purs`のファイルが含まれている

### オプション

| オプション               | デフォルト                                         | 説明                                                           |
| ------------------- | --------------------------------------------- | ------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`          | module のフォーマットです。                                            |
| `version_format`    | `'v${raw}'`                                   | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。       |
| `symbol`            | `'<=> '`                                | The symbol used before displaying the version of PureScript. |
| `detect_extensions` | `['purs']`                                    | どの拡張子がこのモジュールをアクティブにするか                                      |
| `detect_files`      | `['spago.dhall', 'spago.yaml', 'spago.lock']` | どのファイル名がこのモジュールをアクティブにするか                                    |
| `detect_folders`    | `[]`                                          | どのフォルダーがこのモジュールをアクティブにするか                                    |
| `style`             | `'bold white'`                                | モジュールのスタイルです。                                                |
| `disabled`          | `false`                                       | Disables the `purescript` module.                            |

### 変数

| 変数        | 設定例      | 説明                          |
| --------- | -------- | --------------------------- |
| version   | `0.13.5` | The version of `purescript` |
| symbol    |          | オプション `symbol` の値をミラーする     |
| style\* |          | オプション `style` の値をミラーする      |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
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
- 現在のディレクトリに`pixi.toml`ファイルが含まれている
- カレントディレクトリに`.py`の拡張子のファイルが含まれている.
- The current directory contains a file with the `.ipynb` extension.
- 仮想環境がアクティブである

### オプション

| オプション                | デフォルト                                                                                                                     | 説明                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                               | module のフォーマットです。                                                                      |
| `version_format`     | `'v${raw}'`                                                                                                               | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。                                 |
| `symbol`             | `'🐍 '`                                                                                                                    | A format string representing the symbol of Python                                      |
| `style`              | `'yellow bold'`                                                                                                           | モジュールのスタイルです。                                                                          |
| `pyenv_version_name` | `false`                                                                                                                   | pyenvを使用してPythonバージョンを取得します                                                            |
| `pyenv_prefix`       | `'pyenv'`                                                                                                                 | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                                        | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `['py', 'ipynb']`                                                                                                         | どの拡張子がこのモジュールをアクティブにするか                                                                |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini', 'pixi.toml']` | どのファイル名がこのモジュールをアクティブにするか                                                              |
| `detect_folders`     | `[]`                                                                                                                      | どのフォルダーがこのモジュールをアクティブにするか                                                              |
| `disabled`           | `false`                                                                                                                   | `python`モジュールを無効にします。                                                                  |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `'python3'` will hide any Python version 2, see example below.

:::

### 変数

| 変数           | 設定例             | 説明                                         |
| ------------ | --------------- | ------------------------------------------ |
| version      | `'v3.8.1'`      | The version of `python`                    |
| symbol       | `'🐍 '`          | オプション `symbol` の値をミラーする                    |
| style        | `'yellow bold'` | オプション `style` の値をミラーする                     |
| pyenv_prefix | `'pyenv '`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `'venv'`        | The current `virtualenv` name              |

### 設定例

```toml
# ~/.config/starship.toml

[python]
symbol = '👾 '
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = 'python3'
```

```toml
# ~/.config/starship.toml

[python]
# Don't trigger for files with the py extension
detect_extensions = []
```

## Quarto

The `quarto` module shows the current installed version of Quarto used in a project.

デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `_quarto.yml` file
- The current directory contains any `*.qmd` file

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'⨁ '`                               | A format string representing the symbol of Quarto      |
| `style`             | `'bold #75AADB'`                     | モジュールのスタイルです。                                          |
| `detect_extensions` | `['.qmd']`                           | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['_quarto.yml']`                    | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | Disables the `quarto` module.                          |

### 変数

| 変数        | 設定例       | 説明                      |
| --------- | --------- | ----------------------- |
| version   | `1.4.549` | The version of `quarto` |
| symbol    |           | オプション `symbol` の値をミラーする |
| style\* |           | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

## R

`rlang`モジュールは、現在インストールされている[R](https://www.r-project.org/)のバージョンを表示します。 次の条件のいずれかが満たされると、モジュールが表示されます。

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
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'📐'`                                | A format string representing the symbol of R.          |
| `style`             | `'blue bold'`                        | モジュールのスタイルです。                                          |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['.Rprofile']`                      | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `['.Rproj.user']`                    | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | Disables the `r` module.                               |

### 変数

| 変数      | 設定例           | 説明                      |
| ------- | ------------- | ----------------------- |
| version | `v4.0.5`      | The version of `R`      |
| symbol  |               | オプション `symbol` の値をミラーする |
| style   | `'blue bold'` | オプション `style` の値をミラーする  |

### 設定例

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/). デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### オプション

| オプション               | デフォルト                                            | 説明                                                     |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | モジュールのフォーマット文字列。                                       |
| `version_format`    | `'v${raw}'`                                      | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🦋 '`                                           | The symbol used before displaying the version of Raku  |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['META6.json']`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                             | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold 149'`                                     | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                          | Disables the `raku` module.                            |

### 変数

| 変数         | 設定例    | 説明                                   |
| ---------- | ------ | ------------------------------------ |
| version    | `v6.d` | The version of `raku`                |
| vm_version | `moar` | The version of VM `raku` is built on |
| symbol     |        | オプション `symbol` の値をミラーする              |
| style\*  |        | オプション `style` の値をミラーする               |

### 設定例

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

デフォルトでは`red`モジュールは、現在インストールされている[Red](https://www.red-lang.org/)のバージョンを表示します。 次の条件のいずれかが満たされると、モジュールが表示されます:

- カレントディレクトリに拡張子が`.red` or `.reds`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🔺 '`                               | A format string representing the symbol of Red.        |
| `detect_extensions` | `['red']`                            | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'red bold'`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables the `red` module.                             |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v2.5.1` | The version of `red`    |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```

## Ruby

デフォルトでは`ruby`モジュールは現在インストールされている[Ruby](https://www.ruby-lang.org/)のバージョンを表示します。 次の条件のいずれかが満たされると、モジュールが表示されます:

- カレントディレクトリに`Gemfile`ファイルが含まれている
- カレントディレクトリに `.ruby-version` ファイルが含まれている
- カレントディレクトリに `.rb` ファイルが含まれている
- 環境変数に`RUBY_VERSION`または`RBENV_VERSION`が設定されている

Starship gets the current Ruby version by running `ruby -v`.

### オプション

| オプション               | デフォルト                                | 説明                                                      |
| ------------------- | ------------------------------------ | ------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                       |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。  |
| `symbol`            | `'💎 '`                               | Rubyのシンボルを表すフォーマット文字列.                                  |
| `detect_extensions` | `['rb']`                             | どの拡張子がこのモジュールをアクティブにするか                                 |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | どのファイル名がこのモジュールをアクティブにするか                               |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                               |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Which environment variables should trigger this module. |
| `style`             | `'bold red'`                         | モジュールのスタイルです。                                           |
| `disabled`          | `false`                              | `ruby`モジュールを無効にします。                                     |

### 変数

| 変数        | 設定例      | 説明                                          |
| --------- | -------- | ------------------------------------------- |
| version   | `v2.5.1` | The version of `ruby`                       |
| symbol    |          | オプション `symbol` の値をミラーする                     |
| style\* |          | オプション `style` の値をミラーする                      |
| gemset    | `test`   | Optional, gets the current RVM gemset name. |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

デフォルトでは`rust`モジュールは現在インストールされている[Rust](https://www.rust-lang.org/)のバージョンを表示します。 次の条件のいずれかが満たされると、モジュールが表示されます:

- カレントディレクトリに`Cargo.toml`ファイルが含まれている
- カレントディレクトリに`.rs`の拡張子のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🦀 '`                               | Rustのシンボルを表すフォーマット文字列                                  |
| `detect_extensions` | `['rs']`                             | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['Cargo.toml']`                     | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold red'`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `rust`モジュールを無効にします。                                    |

### 変数

| 変数        | 設定例               | 説明                                           |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | `rustc`のバージョン                                |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | The toolchain version                        |
| symbol    |                   | オプション `symbol` の値をミラーする                      |
| style\* |                   | オプション `style` の値をミラーする                       |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

`scala` モジュールは、現在インストールされている[Scala](https://www.scala-lang.org/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`build.sbt`、`.scalaenv`または`.sbtenv`ファイルが含まれている
- カレントディレクトリに拡張子が`.scala`または`.sbt`のファイルが含まれている
- カレントディレクトリに`.metals`ディレクトリが含まれている

### オプション

| オプション               | デフォルト                                    | 説明                                                     |
| ------------------- | ---------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [${symbol}(${version} )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                              | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `detect_extensions` | `['sbt', 'scala']`                       | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `['.metals']`                            | どのフォルダーがこのモジュールをアクティブにするか                              |
| `symbol`            | `'🆂 '`                                   | A format string representing the symbol of Scala.      |
| `style`             | `'red dimmed'`                           | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                  | Disables the `scala` module.                           |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `2.13.5` | The version of `scala`  |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション                  | デフォルト                     | 説明                                                                                                     |
| ---------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------ |
| `bash_indicator`       | `'bsh'`                   | A format string used to represent bash.                                                                |
| `fish_indicator`       | `'fsh'`                   | A format string used to represent fish.                                                                |
| `zsh_indicator`        | `'zsh'`                   | A format string used to represent zsh.                                                                 |
| `powershell_indicator` | `'psh'`                   | A format string used to represent powershell.                                                          |
| `pwsh_indicator`       |                           | A format string used to represent pwsh. The default value mirrors the value of `powershell_indicator`. |
| `ion_indicator`        | `'ion'`                   | A format string used to represent ion.                                                                 |
| `elvish_indicator`     | `'esh'`                   | A format string used to represent elvish.                                                              |
| `tcsh_indicator`       | `'tsh'`                   | A format string used to represent tcsh.                                                                |
| `xonsh_indicator`      | `'xsh'`                   | A format string used to represent xonsh.                                                               |
| `cmd_indicator`        | `'cmd'`                   | A format string used to represent cmd.                                                                 |
| `nu_indicator`         | `'nu'`                    | A format string used to represent nu.                                                                  |
| `unknown_indicator`    | `''`                      | The default value to be displayed when the shell is unknown.                                           |
| `format`               | `'[$indicator]($style) '` | module のフォーマットです。                                                                                      |
| `style`                | `'white bold'`            | モジュールのスタイルです。                                                                                          |
| `disabled`             | `true`                    | Disables the `shell` module.                                                                           |

### 変数

| 変数        | デフォルト | 説明                                                         |
| --------- | ----- | ---------------------------------------------------------- |
| indicator |       | Mirrors the value of `indicator` for currently used shell. |
| style\* |       | オプション `style` の値のミラー。                                      |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = '󰈺 '
powershell_indicator = '_'
unknown_indicator = 'mystery shell'
style = 'cyan bold'
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is set to a number and meets or exceeds the specified threshold.

### オプション

| オプション           | デフォルト                        | 説明                                                                  |
| --------------- | ---------------------------- | ------------------------------------------------------------------- |
| `threshold`     | `2`                          | Display threshold.                                                  |
| `format`        | `'[$symbol$shlvl]($style) '` | module のフォーマットです。                                                   |
| `symbol`        | `'↕️  '`                     | The symbol used to represent the `SHLVL`.                           |
| `repeat`        | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount.       |
| `repeat_offset` | `0`                          | Decrements number of times `symbol` is repeated by the offset value |
| `style`         | `'bold yellow'`              | モジュールのスタイルです。                                                       |
| `disabled`      | `true`                       | Disables the `shlvl` module.                                        |

### 変数

| 変数        | 設定例 | 説明                           |
| --------- | --- | ---------------------------- |
| shlvl     | `3` | The current value of `SHLVL` |
| symbol    |     | オプション `symbol` の値をミラーする      |
| style\* |     | オプション `style` の値をミラーする       |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

Using `repeat` and `repeat_offset` along with `character` module, one can get prompt like `❯❯❯` where last character is colored appropriately for return status code and preceding characters are provided by `shlvl`.

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '[$symbol$shlvl]($style)'
repeat = true
symbol = '❯'
repeat_offset = 1
threshold = 0
```

## Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### オプション

| オプション      | デフォルト                            | 説明                                               |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | module のフォーマットです。                                |
| `symbol`   | `''`                             | A format string displayed before the image name. |
| `style`    | `'bold dimmed blue'`             | モジュールのスタイルです。                                    |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### 変数

| 変数        | 設定例          | 説明                            |
| --------- | ------------ | ----------------------------- |
| env       | `centos.img` | The current Singularity image |
| symbol    |              | オプション `symbol` の値をミラーする       |
| style\* |              | オプション `style` の値をミラーする        |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Solidity

The `solidity` module shows the currently installed version of [Solidity](https://soliditylang.org/) The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.sol` extension

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${major}.${minor}.${patch}'`      | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'S '`                               | A format string representing the symbol of Solidity    |
| `compiler          | ['solc']                             | The default compiler for Solidity.                     |
| `detect_extensions` | `['sol']`                            | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold blue'`                        | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | Disables this module.                                  |

### 変数

| 変数        | 設定例      | 説明                        |
| --------- | -------- | ------------------------- |
| version   | `v0.8.1` | The version of `solidity` |
| symbol    |          | オプション `symbol` の値をミラーする   |
| style\* |          | オプション `style` の値をミラーする    |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml
[solidity]
format = "via [S $version](blue bold)"
```

## Spack

`spack` モジュールは、`$SPACK_ENV` が設定されているときに、現在の [Spack](https://spack.readthedocs.io/en/latest/) 環境を表示します。

### オプション

| オプション               | デフォルト                                  | 説明                                                                              |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | 環境パスを切り捨てて表示するディレクトリの数。 `0`は切り捨てがないことを意味します。  [`directory`](#directory)もご覧ください。 |
| `symbol`            | `'🅢  '`                                | 環境名の直前に使用されるシンボルです。                                                             |
| `style`             | `'bold blue'`                          | モジュールのスタイルです。                                                                   |
| `format`            | `'via [$symbol$environment]($style) '` | module のフォーマットです。                                                               |
| `disabled`          | `false`                                | `spack` モジュールを無効にします。                                                           |

### 変数

| 変数          | 設定例          | 説明                      |
| ----------- | ------------ | ----------------------- |
| environment | `astronauts` | 現在の spack 環境            |
| symbol      |              | オプション `symbol` の値をミラーする |
| style\*   |              | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Status

`status` モジュールは、直前のコマンドの終了ステータスを表示します。 $success_symbol が空 (既定) の場合、モジュールは終了ステータスが `0` でない場合にのみ表示されます。 終了ステータスは符号付き32ビット整数にキャストされます。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション                       | デフォルト                                                                               | 説明                                                                |
| --------------------------- | ----------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                       | モジュールのフォーマットです。                                                   |
| `symbol`                    | `'❌'`                                                                               | プログラムエラー時に表示される記号です。                                              |
| `success_symbol`            | `''`                                                                                | プログラム成功時に表示される記号です。                                               |
| `not_executable_symbol`     | `'🚫'`                                                                               | ファイルが実行可能ファイルでないときに表示されるシンボルです。                                   |
| `not_found_symbol`          | `'🔍'`                                                                               | コマンドが見つからないときに表示されるシンボルです。                                        |
| `sigint_symbol`             | `'🧱'`                                                                               | SIGINT (Ctrl + c) に際して表示されるシンボルです。                                |
| `signal_symbol`             | `'⚡'`                                                                               | 任意のシグナルに対して表示されるシンボルです。                                           |
| `style`                     | `'bold red'`                                                                        | モジュールのスタイルです。                                                     |
| `success_style`             |                                                                                     | The style used on program success (defaults to `style` if unset). |
| `failure_style`             |                                                                                     | The style used on program failure (defaults to `style` if unset). |
| `recognize_signal_code`     | `true`                                                                              | 終了ステータスからシグナルへのマッピングを有効にします。                                      |
| `map_symbol`                | `false`                                                                             | 終了ステータスからシンボルへのマッピングを有効にします。                                      |
| `pipestatus`                | `false`                                                                             | パイプステータス表示を有効にします。                                                |
| `pipestatus_separator`      | <code>&vert;</code>                                                           | パイプラインの各要素を分割するのに使う記号を指定します。フォーマット文字列がサポートされます。                   |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style) '` | コマンドがパイプラインである場合のモジュールのフォーマットです。                                  |
| `pipestatus_segment_format` |                                                                                     | 指定されている場合、パイプステータスの要素を書式設定する際に `format` の代わりに使われます。               |
| `disabled`                  | `true`                                                                              | `status`モジュールを無効にします。                                             |

### 変数

| 変数             | 設定例     | 説明                                                                                           |
| -------------- | ------- | -------------------------------------------------------------------------------------------- |
| status         | `127`   | 直前のコマンドの終了ステータス                                                                              |
| hex_status     | `0x7F`  | 直前のコマンドの終了ステータスを16進数で表記したもの                                                                  |
| int            | `127`   | 直前のコマンドの終了ステータス                                                                              |
| common_meaning | `ERROR` | シグナルでない場合、終了ステータスの意味                                                                         |
| signal_number  | `9`     | シグナルで終了した場合、終了ステータスに対応するシグナル番号                                                               |
| signal_name    | `KILL`  | シグナルで終了した場合、終了ステータスに対応するシグナル名                                                                |
| maybe_int      | `7`     | 対応する意味が見つからない場合にのみ、終了コード番号を含みます。                                                             |
| pipestatus     |         | パイプラインプログラムの終了ステータスの表示です。pipestatus_format の中でのみ使用できます。                                      |
| symbol         |         | オプション `symbol` の値をミラーする                                                                      |
| style\*      |         | Mirrors the value of option `success_style` on program success and `failure_style` otherwise |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[status]
style = 'bg:blue'
symbol = '🔴 '
success_symbol = '🟢 SUCCESS'
format = '[\[$symbol$common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

`sudo` モジュールは、sudo 資格情報が現在キャッシュされているかどうかを表示します。 モジュールは資格情報がキャッシュされている場合にのみ表示されます。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション           | デフォルト                    | 説明                                      |
| --------------- | ------------------------ | --------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | モジュールのフォーマットです。                         |
| `symbol`        | `'🧙 '`                   | 資格情報がキャッシュされたときに表示されるシンボルです。            |
| `style`         | `'bold blue'`            | モジュールのスタイルです。                           |
| `allow_windows` | `false`                  | Windows にはデフォルトで sudo がないため、既定で無効になります。 |
| `disabled`      | `true`                   | `sudo` モジュールを無効にします。                    |

### 変数

| 変数        | 設定例 | 説明                      |
| --------- | --- | ----------------------- |
| symbol    |     | オプション `symbol` の値をミラーする |
| style\* |     | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[sudo]
style = 'bold green'
symbol = '👩‍💻 '
disabled = false
```

```toml
# Windows では
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

デフォルトでは`swift` モジュールは、現在インストールされている[Swift](https://swift.org/)のバージョンを表示します。 次の条件のいずれかが満たされると、モジュールが表示されます:

- カレントディレクトリに`Package.swift`ファイルが含まれている
- カレントディレクトリに拡張子が`.swift`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'🐦 '`                               | Swift のシンボルを表すフォーマット文字列                                |
| `detect_extensions` | `['swift']`                          | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['Package.swift']`                  | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold 202'`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `swift` モジュールを無効にします。                                  |

### 変数

| 変数        | 設定例      | 説明                       |
| --------- | -------- | ------------------------ |
| version   | `v5.2.4` | `swift` のバージョン           |
| symbol    |          | オプション `symbol` の値をミラーします |
| style\* |          | オプション `style` の値をミラーする   |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[swift]
format = 'via [🏎  $version](red bold)'
```

## Terraform

`terraform` モジュールは、現在選択されている[Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) とバージョンを表示します。

::: tip

Terraformのバージョンはデフォルトでは表示されません。多くのプラグインが使用されている場合、現在のTerraformのバージョンでは遅くなるからです。 それでも有効にしたい場合は、 [以下の例に従ってください](#with-terraform-version).

:::

デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`.terraform`フォルダが含まれている
- 現在のディレクトリに `.tf`, `.tfplan` または `.tfstate` のいずれかの拡張子を持つファイルがある。

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol$workspace]($style) '` | モジュールのフォーマット文字列。                                       |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'💠'`                                | ワークスペースの前に表示されるフォーマット文字列。                              |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`        | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `['.terraform']`                     | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'bold 105'`                         | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `terraform` モジュールを無効にします。                              |

### 変数

| 変数        | 設定例        | 説明                      |
| --------- | ---------- | ----------------------- |
| version   | `v0.12.24` | `terraform` のバージョン      |
| workspace | `default`  | 現在のTerraformワークスペース     |
| symbol    |            | オプション `symbol` の値をミラーする |
| style\* |            | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

#### Terraform バージョン表示あり

```toml
# ~/.config/starship.toml

[terraform]
format = '[🏎💨 $version$workspace]($style) '
```

#### Terraform バージョン表示なし

```toml
# ~/.config/starship.toml

[terraform]
format = '[🏎💨 $workspace]($style) '
```

## 時刻

`time`モジュールは、現在の**現地**時間を示します。 `format`設定は、時間の表示方法を制御するために[`chrono`](https://crates.io/crates/chrono)クレートによって使用されます。 使用可能なオプションを確認するには、[chrono strftimeのドキュメント](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)をご覧ください。

::: tip

このモジュールはデフォルトで無効になっています。 有効にするには、設定ファイルで `disabled` を `false` に設定します。

:::

### オプション

| オプション             | デフォルト                   | 説明                                                                                                |
| ----------------- | ----------------------- | ------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | モジュールのフォーマット文字列。                                                                                  |
| `use_12hr`        | `false`                 | 12時間のフォーマットを有効にします。                                                                               |
| `time_format`     | この表の下を参照してください          | 時刻のフォーマットに使用される[クロノフォーマット文字列](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) です。 |
| `style`           | `'bold yellow'`         | モジュールのスタイルです。                                                                                     |
| `utc_time_offset` | `'local'`               | 使用するUTCオフセットを設定します。 -24 から 24 までの間で設定可能です。 フロートが30/45分のタイムゾーンオフセットに対応できるようにします。                   |
| `disabled`        | `true`                  | `time`モジュールを無効にします。                                                                               |
| `time_range`      | `'-'`                   | モジュールを表示する時間帯を設定します。 時刻は24時間形式で指定する必要があります。                                                       |

`use_12hr` が `true` の場合、`format` のデフォルトは `'%r'` です。 それ以外の場合、デフォルトは`'%T'`です。 `time_format` を手動で設定すると、設定 `use_12hr` が上書きされます。

### 変数

| 変数        | 設定例        | 説明                      |
| --------- | ---------- | ----------------------- |
| time      | `13:08:10` | 現在時刻                    |
| style\* |            | オプション `style` の値をミラーします |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = '%T'
utc_time_offset = '-5'
time_range = '10:00:00-14:00:00'
```

## Typst

`typst` モジュールは、現在インストールされてプロジェクトで使われている Typst のバージョンを表示します。

デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`template.typ`ファイルが含まれている
- カレントディレクトリに拡張子が `.typ` のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'t '`                               | A format string representing the symbol of Typst       |
| `style`             | `'bold #0093A7'`                     | モジュールのスタイルです。                                          |
| `detect_extensions` | `['.typ']`                           | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['template.typ']`                   | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `disabled`          | `false`                              | Disables the `typst` module.                           |

### 変数

| 変数            | 設定例       | 説明                                    |
| ------------- | --------- | ------------------------------------- |
| version       | `v0.9.0`  | `typst`のバージョン。typst_version のエイリアスです。 |
| typst_version | `default` | 現在の Typst バージョン                       |
| symbol        |           | オプション `symbol` の値をミラーする               |
| style\*     |           | オプション `style` の値をミラーする                |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

## ユーザー名

`username`モジュールはアクティブなユーザーのユーザー名を表示します。 次の条件のいずれかが満たされると、モジュールが表示されます:

- 現在のユーザーが root/admin である
- カレントユーザーが、ログインしているユーザーとは異なる
- ユーザーがSSHセッションとして接続されている
- `show_always`変数がtrueに設定されている
- 配列 `detect_env_var` に、少なくとも1つの現在シェルで設定されている環境変数の名前が含まれている

::: tip

SSH接続は、環境変数 `SSH_CONNECTION`、 `SSH_CLIENT`、および `SSH_TTY` をチェックすることで検出されます。 お使いの SSH ホストがこれらの変数を設定しない場合、回避策として例えばこれらの変数にダミー値を設定してください。

:::

### オプション

| オプション             | デフォルト                   | 説明                                            |
| ----------------- | ----------------------- | --------------------------------------------- |
| `style_root`      | `'bold red'`            | ユーザーが root/admin のときに使用されるスタイルです。             |
| `style_user`      | `'bold yellow'`         | 非rootユーザーに使用されるスタイルです。                        |
| `detect_env_vars` | `[]`                    | このモジュールを活性化する環境変数。                            |
| `format`          | `'[$user]($style) in '` | module のフォーマットです。                             |
| `show_always`     | `false`                 | `username` モジュールを常に表示します。                     |
| `disabled`        | `false`                 | `username` モジュールを無効にします。                      |
| `aliases`         | `{}`                    | Translate system usernames to something else. |

### 変数

| 変数      | 設定例          | 説明                                                                         |
| ------- | ------------ | -------------------------------------------------------------------------- |
| `style` | `'red bold'` | root がログインしている場合は `style_root` オプションの値をミラーし、そうでない場合は `style_user` をミラーします。 |
| `user`  | `'matchai'`  | 現在ログインしているユーザーID。                                                          |

### 設定例

#### Always show the username

```toml
# ~/.config/starship.toml

[username]
style_user = 'white bold'
style_root = 'black bold'
format = 'user: [$user]($style) '
disabled = false
show_always = true
aliases = { "corpuser034g" = "matchai" }
```

## Vagrant

`vagrant`モジュールは、現在インストールされている[Vagrant](https://www.vagrantup.com/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに`Vagrantfile`ファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'⍱ '`                               | Vagrant のシンボルを表すフォーマット文字列.                             |
| `detect_extensions` | `[]`                                 | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['Vagrantfile']`                    | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'cyan bold'`                        | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `vagrant`モジュールを無効にします。                                 |

### 変数

| 変数        | 設定例              | 説明                      |
| --------- | ---------------- | ----------------------- |
| version   | `Vagrant 2.2.10` | `Vagrant` のバージョン        |
| symbol    |                  | オプション `symbol` の値をミラーする |
| style\* |                  | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

`vlang`モジュールは、現在インストールされている[V](https://vlang.io/)のバージョンを表示します。 デフォルトでは次の条件のいずれかが満たされると、モジュールが表示されます。

- カレントディレクトリに拡張子が`.v`のファイルが含まれている
- カレントディレクトリに`v.mod`、`vpkg.json`または`.vpkg-lock.json`ファイルが含まれている

### オプション

| オプション               | デフォルト                                        | 説明                                                     |
| ------------------- | -------------------------------------------- | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`         | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                                  | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'V '`                                       | V のシンボルを表すフォーマット文字列                                    |
| `detect_extensions` | `['v']`                                      | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                         | どのフォルダーがこのモジュールをアクティブにするか                              |
| `style`             | `'blue bold'`                                | モジュールのスタイルです。                                          |
| `disabled`          | `false`                                      | `vlang`モジュールを無効にします。                                   |

### 変数

| 変数        | 設定例    | 説明                      |
| --------- | ------ | ----------------------- |
| version   | `v0.2` | `v` のバージョン              |
| symbol    |        | オプション `symbol` の値をミラーする |
| style\* |        | オプション `style` の値をミラーする  |

### 設定例

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCSH

`vcsh` モジュールは、現在アクティブな [VCSH](https://github.com/RichiH/vcsh) リポジトリを表示します。 モジュールは、現在使用中のリポジトリがある場合にのみ表示されます。

### オプション

| オプション      | デフォルト                            | 説明                  |
| ---------- | -------------------------------- | ------------------- |
| `symbol`   | `''`                             | リポジトリ名の前に表示される記号です。 |
| `style`    | `'bold yellow'`                  | モジュールのスタイルです。       |
| `format`   | `'vcsh [$symbol$repo]($style) '` | module のフォーマットです。   |
| `disabled` | `false`                          | `vcsh`モジュールを無効にします。 |

### 変数

| 変数        | 設定例                                           | 説明                      |
| --------- | --------------------------------------------- | ----------------------- |
| repo      | `dotfiles` (dotfiles という名の VCSH  リポジトリ内にいる場合) | アクティブなリポジトリ名            |
| symbol    |                                               | オプション `symbol` の値をミラーする |
| style\* | `black bold dimmed`                           | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## Zig

既定で `zig`モジュールは、現在インストールされている[Zig](https://ziglang.org/)のバージョンを表示します。 このモジュールは次の条件のいずれかが満たされると表示されます:

- カレントディレクトリに拡張子が`.zig`のファイルが含まれている

### オプション

| オプション               | デフォルト                                | 説明                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | module のフォーマットです。                                      |
| `version_format`    | `'v${raw}'`                          | バージョンのフォーマット。 使用可能な変数は`raw`、`major`、`minor`と`patch`です。 |
| `symbol`            | `'↯ '`                               | Zig のバージョンの前に表示されるシンボルです。                              |
| `style`             | `'bold yellow'`                      | モジュールのスタイルです。                                          |
| `disabled`          | `false`                              | `zig` モジュールを無効にします。                                    |
| `detect_extensions` | `['zig']`                            | どの拡張子がこのモジュールをアクティブにするか                                |
| `detect_files`      | `[]`                                 | どのファイル名がこのモジュールをアクティブにするか                              |
| `detect_folders`    | `[]`                                 | どのフォルダーがこのモジュールをアクティブにするか                              |

### 変数

| 変数        | 設定例      | 説明                      |
| --------- | -------- | ----------------------- |
| version   | `v0.6.0` | `zig` のバージョン            |
| symbol    |          | オプション `symbol` の値をミラーする |
| style\* |          | オプション `style` の値をミラーする  |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

### 設定例

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```

## カスタムコマンド

`custom` モジュールは、任意のコマンドの出力を表示します。

以下のいずれかの条件が満たされる場合に、モジュールが表示されます:

- 現在のディレクトリに `detect_files` に挙げた名前のファイルが存在する。
- 現在のディレクトリに `detect_folders` に挙げた名前のサブディレクトリが存在する。
- 現在のディレクトリに `detect_extensions` に挙げた拡張子に一致するファイルが存在する。
- `when` コマンドが 0 を返す。
- `os` フィールドが指定されている場合、現在のオペレーティング システム (std::env::consts::OS) がそれに一致する。

::: tip

`.` を使用して複数のカスタムモジュールを定義できます。

:::

::: tip

カスタムモジュールが表示される順序は、`${custom.foo}` (ドットが含まれるので `${...}` を使う必要があります) をトップレベルの `format` に入れることで個別に設定できます。 既定では、 `custom` モジュールは、単にすべての custom モジュールを定義順で表示します。

:::

::: tip

[イシュー #1252](https://github.com/starship/starship/discussions/1252) にはカスタムモジュールの例が含まれています。 紹介されていない興味深い用例をお持ちでしたら、そちらで気軽に共有してください!

:::

::: warning If `unsafe_no_escape` is enabled or prior to starship v1.20 command output is printed unescaped to the prompt.

コマンドが生成するいかなる出力もそのままプロンプト内に表示されます。 This means if the output contains shell-specific interpretable sequences, they could be interpreted on display. Depending on the shell, this can mean that e.g. strings enclosed by backticks are executed by the shell. Such sequences are usually shell specific, e.g. you can write a command module that writes bash sequences, e.g. `\h`, but this module will not work in a fish or zsh shell.

フォーマット文字列には、シェル固有のプロンプトシーケンスを含めることもできます。例えば [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

:::

### オプション

| オプション               | デフォルト                           | 説明                                                                                                                                                                           |
| ------------------- | ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | その出力が表示されるコマンド。 コマンドはシェルの標準入力に渡されます。                                                                                                                                         |
| `when`              | `false`                         | ブール値 (クォートなしの `true` または `false`) かモジュールを表示する条件として使用されるシェルコマンド文字列。 In case of a string, the module will be shown if the `shell` returns a `0` status code from executing it. |
| `require_repo`      | `false`                         | `true`の場合、モジュールは (Git の) リポジトリを含むパスにのみ表示されます。 他のオプションが指定されていない場合、このオプション単体では表示条件として不十分です。                                                                                    |
| `shell`             |                                 | [この表の下を参照してください](#custom-command-shell)                                                                                                                                      |
| `説明`                | `'<custom module>'`       | `starship explain` 実行の際に表示されるモジュールの説明。                                                                                                                                       |
| `unsafe_no_escape`  | `false`                         | When set, command output is not escaped of characters that could be interpreted by the shell.                                                                                |
| `detect_files`      | `[]`                            | 表示条件として確認する作業ディレクトリ内のファイル名を指定します。                                                                                                                                            |
| `detect_folders`    | `[]`                            | 表示条件として確認する作業ディレクトリ内のディレクトリ名を指定します。                                                                                                                                          |
| `detect_extensions` | `[]`                            | 表示条件として確認する作業ディレクトリ内のファイルの拡張子を指定します。                                                                                                                                         |
| `symbol`            | `''`                            | コマンド出力の前に表示される記号です。                                                                                                                                                          |
| `style`             | `'bold green'`                  | モジュールのスタイルです。                                                                                                                                                                |
| `format`            | `'[$symbol($output )]($style)'` | module のフォーマットです。                                                                                                                                                            |
| `disabled`          | `false`                         | `custom` モジュールを無効にします。                                                                                                                                                       |
| `os`                |                                 | モジュールが表示されるオペレーティングシステムの名前 (unix, linux, macos, windows, ...)。 [利用可能な値](https://doc.rust-lang.org/std/env/consts/constant.OS.html) を参照してください。                                |
| `use_stdin`         |                                 | シェルに対してコマンドを標準出力を介して指定するか引数を介して指定するかの振る舞い上書きするおまけの boolen 値の設定です。 指定がない場合、(cmd, nushell などのようにシェルのサポート外でない限り) 標準入力が既定で使われます。 設定するとシェル固有の引数処理を無効にします。                         |
| `ignore_timeout`    | `false`                         | グローバルな `command_timeout` 設定を無視して、いくら時間がかかっても外部コマンド実行を継続します。                                                                                                                  |

### 変数

| 変数        | 説明                                     |
| --------- | -------------------------------------- |
| output    | The output of `command` run in `shell` |
| symbol    | オプション `symbol` の値をミラーする                |
| style\* | オプション `style` の値をミラーする                 |

*: この変数は、スタイル文字列の一部としてのみ使用することができます。

#### カスタムコマンドを実行するシェル

`shell` には、以下のように空でない文字列のリストを指定します:

- 最初の文字列は、コマンドを実行するために使用するシェルへのパスです。
- その他の文字列は引数としてシェルに渡されます。

未設定の場合は、最初に STARSHIP_SHELL に対して、そして Linux では 'sh' に、Windowsでは 'cmd / C' に対してフォールバックが試みられます。

The `command` (and `when`, if applicable) will be passed in on stdin.

`shell` が指定されていないか要素の数が1つでかつ Starship が PowerShell が使われると判定した場合、次の引数が自動的に付加されます: `-NoProfile -Command -`。 `shell` が指定されていないか要素の数が1つでかつ Starship が Cmd が使われると判定した場合、引数 `/C` が自動的に付加され、`stdin` が `false` に設定されます。 `shell` が指定されていないか要素の数が1つでかつ Starship が Nushell が使われると判定した場合、引数 `-c` が自動的に付加され、`stdin` が `false` に設定されます。 この動作は、シェルに明示的に引数を渡すことで回避できます。例:

```toml
shell = ['pwsh', '-Command', '-']
```

::: warning 設定したカスタムシェルが正常終了することを再度ご確認ください

カスタムコマンドを設定するときは、(`shell` オプションを通して指定される) Starship が使用する既定のシェルがコマンドを正しく実行し正しく終了することを確認してください。

たとえば、PowerShell では一行コマンドを実行するために `-Command` パラメータを指定する必要があります。 このパラメータを省略すると、呼び出されたシェルが初期化のために再帰的に Starship を呼び出し、その Starship が再びカスタムコマンドを実行することによって、無限ループになる可能性があります。

PowerShell の `-NoProfile` に似たパラメータは、他のシェルでも推奨されます。これは、Starship の呼び出しごとにカスタムプロファイルの追加ロード時間が発生することを避けるためです。

現在、シェルと適切なパラメータの自動検出は実装されていますが、すべてのシェルがカバーされているとは限りません。 このような状況にお気づきの場合は、[イシューを開いて](https://github.com/starship/starship/issues/new/choose)シェルの詳細と Starship の設定をご報告ください。

:::

### 設定例

```toml
# ~/.config/starship.toml

[custom.foo]
command = 'echo foo' # コマンドの出力を表示する
detect_files = ['foo'] # フィルター。ワイルドカードは非対応
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'

[custom.time]
command = 'time /T'
detect_extensions = ['pst'] # *.pst ファイルを選択
shell = ['pwsh.exe', '-NoProfile', '-Command', '-']

[custom.time-as-arg]
command = 'time /T'
detect_extensions = ['pst'] # *.pst ファイルを選択
shell = ['pwsh.exe', '-NoProfile', '-Command']
use_stdin = false
```
