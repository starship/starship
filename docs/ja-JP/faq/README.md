# FAQ

## デモGIFで使用される構成は何ですか？

- **ターミナルエミュレータ**:[ iTerm2 ](https://iterm2.com/)
  - **テーマ**: Minimal
  - **カラースキーム**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **フォント**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **シェル**: [Fish Shell](https://fishshell.com/)
  - **設定**: [matchaiのDotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **プロンプト**: [Starship](https://starship.rs/)

## デモのGIFのようにコマンド補完はどうしたら使用できますか？

補完サポート、または自動補完は選択したシェルによって提供されます。 デモ中では、デフォルトの[Fish Shell](https://fishshell.com/)によって補完されています。 Z Shell (zsh) を利用しているのであれば、[zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions)を照参してください。

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

Starshipに組み込まれた[Bashの実装](https://github.com/starship/starship/blob/master/src/init/starship.bash)は、[ Command Durationモジュール](https://starship.rs/config/#Command-Duration)などの高度な機能を可能にし、Starshipが事前にインストールされたBash構成と互換性があるようにするため、少し複雑です。

`Starshipのプロンプト`で受け入れられるすべてのフラグのリストは、次のコマンドを取得できます。

```sh
starship prompt --help
```

プロンプトは提供されているコンテキストを使用しますが、フラグは「必須」ではありません。

## 古いバージョンの glibc を使用する Linux ディストリビューションで Starship を実行するにはどうすればよいですか?

CentOS6や7などで事前にビルドされたバイナリを使用していて、"_version 'GLIBC_2.18' not found (required by starship)_" のようなエラーが出た場合、`glibc`の替わりに `musl`でコンパイルされたバイナリを使用できます。

```sh
curl -fsSL https://starship.rs/install.sh | bash -s --- -platform unknown-linux-musl
```

## よくわからない記号を見つけました。これはどういった意味ですか？

不明な記号に遭遇した場合、`starship explain` を使用することで、現在表示しているモジュールの説明を見ることができます。

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

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## How do I uninstall Starship?

Starship is just as easy to uninstall as it is to install in the first place.

1. Starshipを初期化するために使用されるシェル設定の行を削除します(例:`~/.bashrc`)。
1. Starshipのバイナリを削除します。

If Starship was installed using a package manager, please refer to their docs for uninstallation instructions.

If Starship was installed using the `curl | bash` script, the following command will delete the binary:

```sh
# starshipバイナリを見つけて削除します
rm "$(which starship)"
```
