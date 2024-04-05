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

Starship は、プロンプトに表示する情報 (例えば、プログラムのバージョンや現在の git ステータス) を取得するために、異なるコマンドを実行します。 これらのコマンドの実行中に Starship の応答がなくなるのを防ぐためにタイムリミットが設定されています。コマンド実行にタイムリミットよりも長い時間がかかった時、コマンド実行がキャンセルされ上記の警告が表示されます。これは意図的な動作です。 タイムリミットは [`command_timeout`key](../config/#prompt) を用いて変更可能ですので、お望みであればタイムリミットを長くできます。 更に以下のデバッグ手順によって、どのコマンドの実行に時間がかかっているかや、それを高速化できるかについて確認できます。 最終手段として、環境変数 `STARSHIP_LOG` に `error` を設定することでこれらの警告を非表示にできます。

## よくわからない記号を見つけました。これはどういった意味ですか？

不明な記号に遭遇した場合、`starship explain` を使用することで、現在表示しているモジュールの説明を見ることができます。

## Starshipがなにか想定外の挙動をしているとき、どのようにデバッグすればよいですか？

環境変数 `STARSHIP_LOG` を使用してデバッグログを有効にできます。 特定のモジュールをデバグしようとしているとき、デバッグログが過度に冗長になることがありますが、その場合は `module` コマンドが役立ちます。例えば、`rust` モジュールをデバグしようとしているとき、以下のコマンドを用いてこのモジュールからのログと出力を取得できます。

```sh
env STARSHIP_LOG=trace starship module rust
```

Starship が遅い場合は、 `timings` コマンドを使って特定のモジュールまたはコマンドが悪さをしているか確認できます。

```sh
env STARSHIP_LOG=trace starship timings
```

トレースログおよび、実行に 1 ミリ秒以上かかったか何か出力をした全てのモジュールの個別解析を出力します。

バグを見つけた場合は、 `bug-report` コマンドを用いて GitHub の問題を作成できます。

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

## `sudo` を使わずに Starship をインストールするにはどうすればいいですか?

インストールシェルスクリプト (`https://starship.rs/install.sh`) はインストール先のディレクトリが現在のユーザによって書き込みできない時に限り `sudo` の使用を試みます。 既定のインストールディレクトリは、 `$BIN_DIR` 環境変数の値、または`$BIN_DIR` が設定されていない場合は `/usr/local/bin` です。 インストールディレクトリを現在のユーザが書き込みできるディレクトリに設定すれば、 `sudo` なしで Starship をインストールできます。 例えば `curl -sS https://starship.rs/install.sh | sh -s -- -b ~/.local/bin` は、インストールスクリプトにコマンドラインオプション `-b` を指定してインストールディレクトリを `~/.local/bin` に設定します。

Starship の非対話形式でのインストールでは、確認をスキップするために `-y` オプションを追加することを忘れないでください。 対応しているオプションの一覧についてはインストールスクリプトのソースをご確認ください。

パッケージマネージャーを使う時は、 `sudo` を使ったまたは使わないインストールに関して、パッケージマネージャーのドキュメントを参照してください。
