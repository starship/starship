# 高度な設定

Starship は汎用性の高いシェルですが、時には特定の処理を行うために `starship.toml` を編集する以上のことをする必要があります。 このページでは starship で使用される、より高度な設定の一部を詳しく説明していきます。

::: warning

ここに載せられた設定は、Starship の将来のリリースで変更される可能性があります。

:::

## Bashのカスタムの事前プロンプトおよび事前実行コマンド

Bashには、他のほとんどのシェルとは違い、正式な preexec / precmd フレームワークを持っていません。 そのため、 `bash`で完全にカスタマイズ可能なフックを提供することは困難です。 ただし、Starship はプロンプトを描画する一連の流れに、限定的に独自の関数を挿入することができます。

- 関数をプロンプトが描画される直前に実行するためには、新しい関数を定義して `starship_precmd_user_func` に割り当ててください。 例として、ロケットをプロンプトの前に表示させたければ、下記のようにしてください。

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- コマンドの直前に関数を実行するために、[`DEBUG` トラップの仕組み](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/)を使うことができます。 しかし、Starship を初期化する前に DEBUG シグナルをトラップ*しなければいけません*！ Starship は DEBUGトラップの値を保護できますが、 starship の起動後にトラップが上書きされると、いくつかの機能は壊れてしまうでしょう。

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # starshipを起動する前にDEBUGをトラップする
eval $(starship init bash)
```

## ウィンドウのタイトルの変更

いくつかのシェルプロンプトはあなたのためにウィンドウのタイトルを自動的に変更します(例えば、カレントディレクトリを反映するために)。 特に Fish はデフォルトで変更を行います。 Starship はこれをしませんが、この機能を `bash` や `zsh` に追加することは簡単にできます。

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

## Enable Right Prompt

Some shells support a right prompt which renders on the same line as the input. Starship can set the content of the right prompt using the `right_format` option. Any module that can be used in `format` is also supported in `right_format`. The `$all` variable will only contain modules not explicitly used in either `format` or `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh.

### 設定例

```toml
# ~/.config/starship.toml

# A minimal left prompt
format = """$character"""

# move the rest of the prompt to the right
right_format = """$all"""
```

Produces a prompt like the following:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```


## スタイルの設定

Style strings are a list of words, separated by whitespace. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

where `<color>` is a color specifier (discussed below). `fg:<color>` and `<color>` currently do the same thing, though this may change in the future. `inverted` swaps the background and foreground colors. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - 標準的なターミナルカラーの `black`、 `red`、 `green`、 `blue`、 `yellow`、 `purple`、 `cyan`、 `white`。 必要に応じて、より明るい色を得るために `bright-` を前につけることができます。（例えば、 `bright-white` ）
 - `#` に続く16進数。 [RGB の16進数カラーコード](https://www.w3schools.com/colors/colors_hexadecimal.asp)を表します。
 - 0-255 までの間の数字。 [8-bit ANSI カラーコード](https://i.stack.imgur.com/KTSQa.png) を表します。

If multiple colors are specified for foreground/background, the last one in the string will take priority.
