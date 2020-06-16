# FAQ

## デモGIFで使用される構成は何ですか？

- **ターミナルエミュレータ**:[ iTerm2 ](https://iterm2.com/)
  - **テーマ**: Minimal
  - **カラースキーム**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **フォント**: [Fira Code](https://github.com/tonsky/FiraCode)
- **シェル**: [Fish Shell](https://fishshell.com/)
  - **設定**: [matchaiのDotfiles](https://github.com/matchai/dotfiles/blob/master/.config/fish/config.fish)
  - **プロンプト**: [Starship](https://starship.rs/)

## `prompt_order` と `<module>.disabled` は同じことをしますか？

はい、両方ともプロンプトでモジュールを無効にするために使用できます。 モジュールを無効にするだけの場合は、これらの理由から` <module> .disabled `を無効にする方法をお勧めします。

- モジュールを無効にすると、prompt_orderからモジュールを省略するよりも明確になります。
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

"*version 'GLIBC_2のようなエラーが表示された場合。 8' が見つかりません (starshipで要求されます)*" プリビルドバイナリを使用しています（例えば、 CentOS 6 または 7 では、`glibc`の代わりに`musl`でコンパイルされたバイナリを使用できます。

```sh
curl -fsSL https://starship.rs/install.sh | bash -s --- -platform unknown-linux-musl
```
