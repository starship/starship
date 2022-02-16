# 高度な設定

Starship は汎用性の高いシェルですが、時には特定の処理を行うために `starship.toml` を編集する以上のことをする必要があります。 このページでは starship で使用される、より高度な設定の一部を詳しく説明していきます。

::: warning

ここに載せられた設定は、Starship の将来のリリースで変更される可能性があります。

:::

## Custom pre-prompt and pre-execution Commands in Cmd

Clink provides extremely flexible APIs to run pre-prompt and pre-exec commands in Cmd shell. It is fairly simple to use with Starship. Make the following changes to your `starship.lua` file as per your requirements:

- To run a custom function right before the prompt is drawn, define a new function called `starship_preprompt_user_func`. This function receives the current prompt as a string that you can utilize. For example, to draw a rocket before the prompt, you would do

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- To run a custom function right before a command is executed, define a new function called `starship_precmd_user_func`. This function receives the current commandline as a string that you can utilize. For example, to print the command that's about to be executed, you would do

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

- コマンドの直前に関数を実行するために、[`DEBUG` トラップの仕組み](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/)を使うことができます。 However, you **must** trap the DEBUG signal _before_ initializing Starship! Starship は DEBUGトラップの値を保護できますが、 starship の起動後にトラップが上書きされると、いくつかの機能は壊れてしまうでしょう。

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap DEBUG *before* running starship
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Custom pre-prompt and pre-execution Commands in PowerShell

PowerShell does not have a formal preexec/precmd framework like most other shells. Because of this, it is difficult to provide fully customizable hooks in `powershell`. ただし、Starship はプロンプトを描画する一連の流れに、限定的に独自の関数を挿入することができます。

Create a function named `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## ウィンドウのタイトルの変更

いくつかのシェルプロンプトはあなたのためにウィンドウのタイトルを自動的に変更します(例えば、カレントディレクトリを反映するために)。 特に Fish はデフォルトで変更を行います。 Starship does not do this, but it's fairly straightforward to add this functionality to `bash`, `zsh`, `cmd` or `powershell`.

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

For Cmd, you can change the window title using the `starship_preprompt_user_func` function.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

You can also set a similar output with PowerShell by creating a function named `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (&starship init powershell)
```

## 右プロンプトの有効化

シェルによっては、入力と同じ行にレンダリングされる右プロンプトをサポートしています。 Starship では `right_format` オプションを使って右プロンプトの内容を設定できます。 `format`で使用できるモジュールはすべて`right_format`でも使用できます。 変数`$all`には、`format`や`right_format`で明示的に使用されていないモジュールのみが格納されます。

注意: 右プロンプトは入力の場所に続く単一の行です。 複数行のプロンプトで入力行の上を右寄せにするには、[fillモジュール](/config/#fill)を参照してください。

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd.

### 設定例

```toml
# ~/.config/starship.toml

# A minimal left prompt
format = """$character"""

# move the rest of the prompt to the right
right_format = """$all"""
```

次のようなプロンプトが生成されます:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## Continuation Prompt

Some shells support a continuation prompt along with the normal prompt. This prompt is rendered instead of the normal prompt when the user has entered an incomplete statement (such as a single left parenthesis or quote).

Starship can set the continuation prompt using the `continuation_prompt` option. The default prompt is `"[∙](bright-black) "`.

Note: `continuation_prompt` should be set to a literal string without any variables.

Note: Continuation prompts are only available in the following shells:

- `bash`
- `zsh`
- `PowerShell`

### 設定例

```toml
# ~/.config/starship.toml

# A continuation prompt that displays two filled in arrows
continuation_prompt = "▶▶"
```

## スタイルの設定

スタイル文字列は空白で区切られた単語のリストです。 大文字小文字を区別しません（例えば、 `bold` と`BoLd` は同じだとみなされます）。 それぞれ以下のいずれか一つが該当します。

- `bold`
- `italic`
- `underline`
- `dimmed`
- `inverted`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

ここで、 `<color>` は色を指定します（以下で述べます）。 現在 `fg:<color>` と `<color>` は同様の動作ですが、将来変更される可能性があります。 `inverted` は背景と前景の色を交換します。 文字列中の単語の順序は関係ありません。

`none` トークンは、文字列中の`bg:` 指定子の一部でない場合、他のすべてのトークンをオーバーライドします。そのため、たとえば、`fg:red none fg:blue` と指定した場合、スタイルなしの文字列が作られます。 `bg:none` は背景色をデフォルトの色にセットするので、`fg:red bg:none` は `red` や `fg:red` と同じ意味になり、`bg:green fg:red bg:none` も `fg:red` や `red` と同じ意味になります。 将来 `none` を他の単語と一緒に使用することはエラーになるかもしれません。

色は以下のいずれか1つを指定できます。

- 標準的なターミナルカラーの `black`、 `red`、 `green`、 `blue`、 `yellow`、 `purple`、 `cyan`、 `white`。 必要に応じて、より明るい色を得るために `bright-` を前につけることができます。（例えば、 `bright-white` ）
- `#` に続く16進数。 [RGB の16進数カラーコード](https://www.w3schools.com/colors/colors_hexadecimal.asp)を表します。
- 0-255 までの間の数字。 [8-bit ANSI カラーコード](https://i.stack.imgur.com/KTSQa.png) を表します。

複数の色が文字色/背景色に指定された際には、最後の指定が優先して選ばれます。
