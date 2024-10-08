# 高度な設定

Starship は汎用性の高いシェルですが、時には特定の処理を行うために `starship.toml` を編集する以上のことをする必要があります。 このページでは starship で使用される、より高度な設定の一部を詳しく説明していきます。

::: warning

ここに載せられた設定は、Starship の将来のリリースで変更される可能性があります。

:::

## PowerShell の TransientPrompt

直前に出力されたプロンプトを置き換えることができます。 プロンプトの内容全てが常に必要ではない時に役立ちます。 有効にするには、 `Enable-TransientPrompt` をシェルで実行してください。 `$PROFILE` に追記することによって常時有効にすることが出来ます。 また、 `Disable-TransientPrompt` によっていつでも無効化することが出来ます。

デフォルトでは、入力した文字列の左側を `>` で置換します。 カスタマイズするには、関数を `Invoke-Starship-TransientFunction` という名前で定義してください。 Starshipの `character` モジュールを表示する場合はこのようにします：

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## Cmd の TransientPrompt と TransientRightPrompt

Clink を使うと直前に出力したプロント文字列をカスタマイズできます。 全ての情報が必要では無い時に役に立ちます。 有効化するには次のコマンドを実行します。 `clink set prompt.transient <value>` 。 \<value\> には次のいずれかの値を指定します。

- `always`: 直前に出力したプロンプト文字列を常に置換します。
- `same_dir`: 作業ディレクトリが同じなら、直前に出力したプロンプト文字列を置換します。
- `off`: プロンプト文字列を置換しません(無効化します)。

この操作が必要なのは1度だけです。 自分の `starship.lua` を次のように編集すると、プロンプト文字列の左側や右側に出力する文字列を変更できます。

- デフォルトでは、入力した文字列の左側を `>` へ置換します。 カスタマイズするには、新しい関数  `starship_transient_prompt_func` を定義します。 この関数の受け取る引数は今のプロンプト文字列で、あなたが変更できるようになっています。 Starshipの `character` モジュールを表示する場合はこのようにします：

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- デフォルトでは、入力した文字列の右側は空です。 カスタマイズするには、新しい関数 `starship_transient_rprompt_func` を定義します。 この関数の受け取る引数は今のプロンプト文字列で、あなたが変更できるようになっています。 例えば、直前のコマンドを実行した時刻を表示するには次のようにします。

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## Fish の TransientPrompt と TransientRightPrompt

直前に出力されたプロンプトを指定の内容で置き換えることができます。 プロンプトの内容全てが常に必要ではない時に役立ちます。 有効にするには、 `enable_transience` を現在のシェルセッションで実行してください。 設定を永続化するにはこのコマンドを `~/.config/fish/config.fish` に記述してください。 また、 `disable_transience` を実行することでいつでも無効化できます。

※Fishの場合は、コマンドラインが空ではなく構文的に正しい場合にのみ、transient プロンプトが出力されます。

- デフォルトでは入力した文字列の左側が太字・緑色の  `❯` に置き換えられます。 カスタマイズするには、新しい関数  `starship_transient_prompt_func` を定義します。 Starshipの `character` モジュールを表示する場合はこのようにします：

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- デフォルトでは、入力した文字列の右側は空です。 カスタマイズするには、新しい関数 `starship_transient_rprompt_func` を定義します。 例えば、直前のコマンドを実行した時刻を表示するには次のようにします。

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## Bash の TransientPrompt と TransientRightPrompt

バージョン 0.4 以降の [ble.sh](https://github.com/akinomyoga/ble.sh) の枠組みを用いると、直前に表示されたプロンプトを指定の文字列に置き換えることができます。 プロンプトの内容全てが常に必要ではない時に役立ちます。 有効にするには、 `~/.bashrc` に `bleopt prompt_ps1_transient=<value>` を記述してください。

\<value\> は `always`、 `same-dir` 、 `trim` からなるコロン区切りのリストです。 設定 `prompt_ps1_final` が空でかつ設定 `prompt_ps1_transient` が非空の \<value\> を持つ場合、 `PS1` に基づくプロンプトは現在のコマンドラインを去るときに消去されます。 \<value\> が `trim` を含む場合、複数行 `PS1` の最後の行だけを残して他の行は消去されます。 それ以外の場合は、あたかも `PS1=` が指定されたかのようにコマンドラインが再描画されます。 \<value\> が項目 `same-dir` を含む時に現在のワーキングディレクトリが前のコマンドラインの最終的なワーキングディレクトリと異なる場合、設定 `prompt_ps1_transient` は無視されます。

左側や右側に transient プロンプトとして出力する文字列を変更するには、以下の変更を `~/.blerc` (または `~/.config/blesh/init.sh`) に適用してください。

- 入力文字列の左側を何に置き換えるか変更するには、 ble.sh 設定 `prompt_ps1_final` を設定します。 例えば Starship の `character` モジュールをここに表示するには、次のようにします。

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- 入力文字列の右側を何に置き換えるか変更するには、 ble.sh 設定 `prompt_rps1_final` を設定します。 例えば、直前のコマンドを実行した時刻を表示するには次のようにします。

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Cmdのカスタムの事前プロンプトおよび事前実行コマンド

Clinkはプロンプト表示前と実行前にCmd shellコマンドを実行するための非常に柔軟なAPIを提供します。 Starshipからこれを利用するのはとても簡単です。 好みに従って `starship.lua` を以下のように変更してください。

- プロンプトが描画される直前に好みの関数を実行するには、 `starship_preprompt_user_func` という名前で新しい関数を定義してください。 この関数は現在のプロンプトを再利用できる文字列で受け取ります。 例として、ロケットをプロンプトの前に表示するには、以下のようにします。

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a")()
```

- コマンドが実行される直前に好みの関数を実行するには、 `starship_precmd_user_func` という名前で新しい関数を定義してください。 この関数は現在のコマンドライン文字列を再利用できる文字列で受け取ります。 例えば、これから実行されるコマンドを出力するには、以下のようにします。

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Bashのカスタムの事前プロンプトおよび事前実行コマンド

Bashには、他のほとんどのシェルとは違い、正式な preexec / precmd フレームワークを持っていません。 そのため、 `bash`で完全にカスタマイズ可能なフックを提供することは困難です。 ただし、Starship はプロンプトを描画する一連の流れに、限定的に独自の関数を挿入することができます。

- 関数をプロンプトが描画される直前に実行するためには、新しい関数を定義して `starship_precmd_user_func` に割り当ててください。 例として、ロケットをプロンプトの前に表示させたければ、下記のようにしてください。

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- コマンドの直前に関数を実行するために、[`DEBUG` トラップの仕組み](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/)を使うことができます。 ただし、Starship を初期化する_前_に DEBUG シグナルをトラップ**しなければいけません**! Starship は DEBUGトラップの値を保護できますが、 starship の起動後にトラップが上書きされると、いくつかの機能は壊れてしまうでしょう。

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## PowerShell のカスタム事前プロンプトおよび事前実行コマンド

PowerShell は、他のほとんどのシェルと違い、正式な preexec/precmd の枠組みを持ちません。 そのため、`powershell`で完全にカスタマイズ可能なフックを提供することは困難です。 ただし、Starship はプロンプトを描画する一連の流れに、限定的に独自の関数を挿入することができます。

`Invoke-Starship-PreCommand` という名前の関数を作成してください。

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## ウィンドウタイトルの変更

いくつかのシェルプロンプトはあなたのためにウィンドウのタイトルを自動的に変更します(例えば、カレントディレクトリを反映するために)。 特に Fish はデフォルトで変更を行います。 Starship はこれに対応しませんが、この機能を `bash`, `zsh`, `cmd`, `powershell` に追加することは簡単です。

まず、ウィンドウのタイトルを変更する関数を定義してください（ bash も zsh も同様に）

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

タイトルをカスタマイズするために変数を利用することができます (`$USER` 、 `$HOSTNAME`、 `$PWD` が一般的です)。

`bash` では関数を starship の precmd 関数としてセットしてください。

```bash
starship_precmd_user_func="set_win_title"
```

`zsh`では関数を `precmd_functions` の配列に追加してください。

```bash
precmd_functions+=(set_win_title)
```

もし結果に満足したら、永続化のためそれぞれの行をシェルの設定ファイル (`~/.bashrc` もしくは `~/.zshrc`) に追加してください。

たとえば、現在のディレクトリをターミナルタブのタイトルに表示したい場合は、 `~/.bashrc`または`~/.zshrc`に以下のスニペットを追加します。

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

Cmd では、 `starship_preprompt_user_func` 関数を使用してウィンドウのタイトルを変更できます。

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

PowerShell でも `Invoke-Starship-PreCommand` という名前の関数を作成することで、同様の出力を設定できます。

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## 右プロンプトの有効化

シェルによっては、入力と同じ行にレンダリングされる右プロンプトをサポートしています。 Starship では `right_format` オプションを使って右プロンプトの内容を設定できます。 `format`で使用できるモジュールはすべて`right_format`でも使用できます。 変数`$all`には、`format`や`right_format`で明示的に使用されていないモジュールのみが格納されます。

注意: 右プロンプトは入力の場所に続く単一の行です。 複数行プロンプトで入力行の上にあるモジュールを右寄せしたいときは、 [`fill` module](../config/#fill) を参照してください。

`right_format` は現在、次のシェルでサポートされています: elvish, fish, zsh, xonsh, cmd, nushell, bash

注意: 右プロンプトを Bash で利用するには [ble.sh](https://github.com/akinomyoga/ble.sh) のバージョン 0.4 以降をインストールする必要があります。

### 設定例

```toml
# ~/.config/starship.toml

# 最低限の左プロンプト
format = """$character"""

# 残りのプロンプトの内容を右に寄せる
right_format = """$all"""
```

次のようなプロンプトが生成されます:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## 継続プロンプト (Continuation Prompt)

一部のシェルは、通常のプロンプトの他に継続プロンプトをサポートしています。 このプロンプトは、ユーザーが不完全な文 (単一の左括弧や引用符など) を確定したときに通常のプロンプトの代わりに表示されます。

Starship では、 `contination_prompt` オプションを使用して継続プロンプトを設定できます。 既定の継続プロンプトは `'[・](bright-black) '` です。

注意: `contination_prompt` には変数を含まないそのまま文字列を設定する必要があります。

注意: 継続プロンプトは次のシェルでのみ使用できます。

- `bash`
- `zsh`
- `PowerShell`

### 設定例

```toml
# ~/.config/starship.toml

# 2つの塗りつぶし右三角を表示する継続プロンプト
continuation_prompt = '▶▶ '
```

## スタイルの設定

スタイル文字列は空白で区切られた単語のリストです。 大文字小文字を区別しません（例えば、 `bold` と`BoLd` は同じだとみなされます）。 それぞれ以下のいずれか一つが該当します。

- `bold`
- `italic`
- `underline`
- `dimmed`
- `inverted`
- `blink`
- `hidden`
- `strikethrough`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

ここで、 `<color>` は色を指定します（以下で述べます）。 現在 `fg:<color>` と `<color>` は同様の動作ですが、将来変更される可能性があります。 `<color>` can also be set to `prev_fg` or `prev_bg` which evaluates to the previous item's foreground or background color respectively if available or `none` otherwise. `inverted` は背景と前景の色を交換します。 文字列中の単語の順序は関係ありません。

`none` トークンは、文字列中の`bg:` 指定子の一部でない場合、他のすべてのトークンをオーバーライドします。そのため、たとえば、`fg:red none fg:blue` と指定した場合、スタイルなしの文字列が作られます。 `bg:none` は背景色をデフォルトの色にセットするので、`fg:red bg:none` は `red` や `fg:red` と同じ意味になり、`bg:green fg:red bg:none` も `fg:red` や `red` と同じ意味になります。 将来 `none` を他の単語と一緒に使用することはエラーになるかもしれません。

色は以下のいずれか1つを指定できます。

- 標準的なターミナルカラーの `black`、 `red`、 `green`、 `blue`、 `yellow`、 `purple`、 `cyan`、 `white`。 必要に応じて、より明るい色を得るために `bright-` を前につけることができます。（例えば、 `bright-white` ）
- `#` に続く16進数。 [RGB の16進数カラーコード](https://www.w3schools.com/colors/colors_hexadecimal.asp)を表します。
- 0-255 までの間の数字。 [8-bit ANSI カラーコード](https://i.stack.imgur.com/KTSQa.png) を表します。

複数の色が文字色/背景色に指定された際には、最後の指定が優先して選ばれます。

すべてのスタイル文字列がすべての端末で正しく表示できるわけではありません。 特に、端末の変な動作として以下のようなものが知られています。

- 多くの端末はデフォルトで `blink` のサポートを無効にしています。
- `hidden` は [iTerm ではサポートされていません](https://gitlab.com/gnachman/iterm2/-/issues/4564)。
- `strikethrough` は、macOS 既定の Terminal.app ではサポートされていません。
