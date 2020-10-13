# FAQ

## デモGIFで使用される構成は何ですか？

- **ターミナルエミュレータ**:[ iTerm2 ](https://iterm2.com/)
  - **テーマ**: Minimal
  - **カラースキーム**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **フォント**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **シェル**: [Fish Shell](https://fishshell.com/)
  - **設定**: [matchaiのDotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **プロンプト**: [Starship](https://starship.rs/)

## How do I get command completion as shown in the demo GIF?

Completion support is provided by your shell of choice. In the case of the demo, the demo was done with [Fish Shell](https://fishshell.com/), which provides completions by default. If you use Z Shell (zsh), I'd suggest taking a look at [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Do top level `format` and `<module>.disabled` do the same thing?

はい、両方ともプロンプトでモジュールを無効にするために使用できます。 モジュールを無効にするだけの場合は、これらの理由から` <module> .disabled `を無効にする方法をお勧めします。

- Disabling modules is more explicit than omitting them from the top level `format`
- Starshipが更新されると、新しく作成されたモジュールがプロンプトに追加されます

## ドキュメントによると、Starshipはクロスシェル対応をしているようですが、Xシェルはサポートしていません。 なぜですか？

Starshipの構築方法は、事実上すべてのシェルのサポートを追加できるはずです。 Starshipのバイナリはステートレスであり、シェルに依存しないため、シェルがプロンプトのカスタマイズとシェルの拡張をサポートしている限り、Starshipを使用できます。

Starshipをbashで動作させる小さな例を次に示します。

```sh
# Get the status code from the last command executed
STATUS=$?

# 実行中のジョブの数を取得します。
NUM_JOBS=$(jobs -p | wc -l)

# プロンプトを `starship prompt`に設定
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

Starshipに組み込まれた[Bashの実装](https://github.com/starship/starship/blob/master/src/init/starship.bash)は、[ Command Durationモジュール](https://starship.rs/config/#Command-Duration)などの高度な機能を可能にし、Starshipが事前にインストールされたBash構成と互換性があるようにするため、少し複雑です。

`Starshipのプロンプト`で受け入れられるすべてのフラグのリストは、次のコマンドを取得できます。

```sh
starship prompt --help
```

プロンプトは提供されているコンテキストを使用しますが、フラグは「必須」ではありません。

## 古いバージョンの glibc を使用する Linux ディストリビューションで Starship を実行するにはどうすればよいですか?

"_version 'GLIBC_2のようなエラーが表示された場合。 8' が見つかりません (starshipで要求されます)_" プリビルドバイナリを使用しています（例えば、 CentOS 6 または 7 では、`glibc`の代わりに`musl`でコンパイルされたバイナリを使用できます。

```sh
curl -fsSL https://starship.rs/install.sh | bash -s --- -platform unknown-linux-musl
```

## プロンプトにグリフ記号が表示されないのはなぜですか?

これの最も一般的な原因は、システムの設定ミスです。 いくつかのLinuxディストリビューション 特に、すぐに使用できるフォントサポートは付属していません。 次のことを確認する必要があります。

- ロケールは、`de_DE.UTF-8`や` ja_JP.UTF-8</ 0>などのUTF-8値に設定されています。 <code>LC_ALL`がUTF-8値でない場合、[変更する必要があります](https://www.tecmint.com/set-system-locales-in-linux/)。
- 絵文字フォントがインストールされています。 ほとんどのシステムにはデフォルトで絵文字フォントが付属していますが、 一部(特にArch Linux) はそうではありません。 通常、システムの パッケージマネージャーからインストールすることができます--[noto emoji](https://www.google.com/get/noto/help/emoji/)は人気な選択肢です。
- [Nerd Font](https://www.nerdfonts.com/)を使用しています。

システムをテストするには、ターミナルで次のコマンドを実行します。

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

1行目は[snake emoji](https://emojipedia.org/snake/)を生成し、2行目は[powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs)を生成するはずです。

いずれかのシンボルが正しく表示されない場合でも、システムの設定が間違っています。 残念ながら、フォント設定を正しくするのは難しい場合があります。 Discordのユーザーがお役に立てるかもしれません。 両方の記号が正しく表示されているにもかかわらず、まだStarshipに表示されていない場合は、[バグ報告をしてください!](https://github.com/starship/starship/issues/new/choose)

## Starshipをアンインストールするにはどうすればいいですか?

Starshipは、最初の場所にインストールするのと同じくらい簡単にアンインストールできます。

1. Starshipを初期化するために使用されるシェル設定の行を削除します(例:`~/.bashrc`)。
1. Starshipのバイナリを削除します。

Starship がパッケージマネージャを使用してインストールされている場合は、アンインストール手順については、そのドキュメントを参照してください。

Starship が `curl | bash` スクリプトを使用してインストールされた場合、次のコマンドはバイナリを削除します:

```sh
# starshipバイナリを見つけて削除します
rm "$(which starship)"
```
