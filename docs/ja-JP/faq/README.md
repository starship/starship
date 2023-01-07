# よくある質問

## デモGIFで使用される構成は何ですか？

- **ターミナルエミュレータ**:[ iTerm2 ](https://iterm2.com/)
  - **テーマ**: Minimal
  - **カラースキーム**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **フォント**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **シェル**: [Fish Shell](https://fishshell.com/)
  - **設定**: [matchaiのDotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **プロンプト**: [Starship](https://starship.rs/)

## デモのGIFのようにコマンド補完はどうしたら使用できますか？

補完サポート、または自動補完は選択したシェルによって提供されます。 デモ中では、デフォルトの[Fish Shell](https://fishshell.com/)によって補完されています。 Z Shell (zsh) を利用しているのであれば、[zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions)を参照してください。

## トップレベルの`format`と`<module>.disabled`は同じように動作しますか？

はい、両方ともプロンプトでモジュールを無効にするために使用できます。 モジュールを無効にするだけの場合は、これらの理由から` <module> .disabled `を無効にする方法をお勧めします。

- モジュールを無効化することは、トップレベルの`format`を削除するよりも明示的です。
- Starshipが更新されると、新しく作成されたモジュールがプロンプトに追加されます

## Starshipはcross-shellとのことです。 私の好みのshellはサポートしていないようですが。

Starshipの構築方法は、事実上すべてのシェルのサポートを追加できるはずです。 Starshipのバイナリはステートレスであり、シェルに依存しないため、シェルがプロンプトのカスタマイズとシェルの拡張をサポートしている限り、Starshipを使用できます。

Starshipをbashで動作させる例を次に示します。

```sh
# Get the status code from the last command executed
STATUS=$?

# 実行中のジョブの数を取得します。
NUM_JOBS=$(jobs -p | wc -l)

# プロンプトを `starship prompt`に設定
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

Starshipに組み込まれた[Bashの実装](https://github.com/starship/starship/blob/master/src/init/starship.bash)は、[ Command Durationモジュール](https://starship.rs/config/#command-duration)などの高度な機能を可能にし、Starshipが事前にインストールされたBash構成と互換性があるようにするため、少し複雑です。

`Starshipのプロンプト`で受け入れられるすべてのフラグのリストは、次のコマンドを取得できます。

```sh
starship prompt --help
```

プロンプトは提供されているコンテキストを使用しますが、フラグは「必須」ではありません。

## 古いバージョンの glibc を使用する Linux ディストリビューションで Starship を実行するにはどうすればよいですか?

CentOS6や7などで事前にビルドされたバイナリを使用していて、"_version 'GLIBC_2.18' not found (required by starship)_" のようなエラーが出た場合、`glibc`の替わりに `musl`でコンパイルされたバイナリを使用できます。

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## どうして`Executing command "..." timed out.`という警告が出てくるのでしょうか?

Starship executes different commands to get information to display in the prompt, for example the version of a program or the current git status. To make sure starship doesn't hang while trying to execute these commands we set a time limit, if a command takes longer than this limit starship will stop the execution of the command and output the above warning, this is expected behaviour. This time limit is configurable using the [`command_timeout`key](/config/#prompt) so if you want you can increase the time limit. You can also follow the debugging steps below to see which command is being slow and see if you can optimise it. Finally you can set the `STARSHIP_LOG` env var to `error` to hide these warnings.

## よくわからない記号を見つけました。これはどういった意味ですか？

不明な記号に遭遇した場合、`starship explain` を使用することで、現在表示しているモジュールの説明を見ることができます。

## Starshipがなにか想定外の挙動をしているとき、どのようにデバッグすればよいですか？

You can enable the debug logs by using the `STARSHIP_LOG` env var. These logs can be very verbose so it is often useful to use the `module` command if you are trying to debug a particular module, for example, if you are trying to debug the `rust` module you could run the following command to get the trace logs and output from the module.

```sh
env STARSHIP_LOG=trace starship module rust
```

If starship is being slow you can try using the `timings` command to see if there is a particular module or command that to blame.

```sh
env STARSHIP_LOG=trace starship timings
```

This will output the trace log and a breakdown of all modules that either took more than 1ms to execute or produced some output.

Finally if you find a bug you can use the `bug-report` command to create a Github issue.

```sh
starship bug-report
```

## 私のプロンプトで記号のグリフがないのはなぜですか？

よくある原因はシステム上での設定ミスです。 いくつかのLinuxディストリビューションの初期設定にフォントサポートがありません。 次のことを確認してください。

- ロケールが、`de_DE.UTF-8`や` ja_JP.UTF-8`などのUTF-8に設定されている。 `LC_ALL`がUTF-8でない場合、[変更する必要があります](https://www.tecmint.com/set-system-locales-in-linux/)。
- 絵文字フォントがインストールされている。 ほとんどのシステムにはデフォルトで絵文字フォントが付属していますが、 一部 (特にArch Linux) はそうではありません。 通常、システムの パッケージマネージャーからインストールすることができます。--[noto emoji](https://www.google.com/get/noto/help/emoji/)は一般的な選択肢です。
- [Nerd Font](https://www.nerdfonts.com/)を使用している。

ターミナルで以下のコマンドを実行することでテストできます。

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

一行目は[蛇の絵文字](https://emojipedia.org/snake/)、二行目は[powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs)が表示されるはずです。

もし、どちらの記号とも正しく表示されない場合は、システムの設定が間違っています。 不幸にも、正しくフォントを設定するのは難しいものです。 Discordのユーザーが助けてくれるかもしれません！ もし記号が正しく表示されているのにもかかわらず、Starshipが正しく表示されていない場合は、[バグの報告](https://github.com/starship/starship/issues/new/choose)をお願いします。

## Starshipをアンインストールしたい

Starshipのアンインストールはインストールと同じぐらい簡単です。

1. Starshipを初期化するために使用した、シェルの設定行を削除します (例:`~/.bashrc`)。
1. Starshipのバイナリを削除します。

パッケージマネージャーを使用してStarshipをインストールした場合は、パッケージマネージャーのアンインストールガイドを参照してください。

Starship をインストールスクリプトを使用してインストールした場合、次のコマンドでバイナリが削除されます。

```sh
# starshipのバイナリを見つけて削除
sh -c 'rm "$(command -v 'starship')"'
```
