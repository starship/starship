# 高度な設定

Starship は汎用性の高いシェルですが、時には特定の処理を行うために `starship.toml` を編集する以上のことをする必要があります。 このページでは starship で使用される、より高度な設定の一部を詳しく説明していきます。

::: warning ここに載せられた設定は、Starship の将来のリリースで変更される可能性があります。 :::

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

もし結果に満足したら、永続化のためそれぞれの行を (`~/.bashrc` もしくは `~/.zsrhc`) に追加してください。

## スタイルの設定

スタイル文字列は空白で区切られた単語のリストです。 大文字小文字を区別しません（例えば、 `bold` と`BoLd` は同じだとみなされます）。 それぞれ以下のいずれか一つが該当します。

- `bold`
- `underline`
- `dimmed`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

ここで、 `<color>` は色を指定します（以下で述べます）。 `fg:<color>` と `<color>` は現在同様の動作ですが、将来変更される可能性があります。 文字列中の単語の順序は関係ありません。

例えば `fg:red none fg:blue` は依然としてスタイルのない文字列となるように、 `none` は他の文字列中の他の単語すべてを上書きします。 将来 `none` を他の単語と一緒に使用することはエラーになるかもしれません。

色は以下のいずれか1つを指定できます。

- 標準的なターミナルカラーの `black`、 `red`、 `green`、 `blue`、 `yellow`、 `purple`、 `cyan`、 `white`。 必要に応じて、より明るい色を得るために `bright-` を前につけることができます。（例えば、 `bright-white` ）
- `#` に続く16進数。 [RGB の16進数カラーコード](https://www.w3schools.com/colors/colors_hexadecimal.asp)を表します。
- 0-255 までの間の数字。 [8-bit ANSI カラーコード](https://i.stack.imgur.com/KTSQa.png) を表します。

複数の色が文字色/背景色に指定された際には、最後の指定が優先して選ばれます。