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

Yes, they can both be used to disable modules in the prompt. If all you plan to do is disable modules, `<module>.disabled` is the preferred way to do so for these reasons:

- Disabling modules is more explicit than omitting them from the top level `format`
- Starshipが更新されると、新しく作成されたモジュールがプロンプトに追加されます

## The docs say Starship is cross-shell, but it doesn't support X shell. Why?

The way Starship is built, it should be possible to add support for virtually any shell. The starship binary is stateless and shell agnostic, so as long as your shell supports prompt customization and shell expansion, Starship can be used.

Here's a small example getting Starship working with bash:

```sh
# Get the status code from the last command executed
STATUS=$?

# 実行中のジョブの数を取得します。
NUM_JOBS=$(jobs -p | wc -l)

# プロンプトを `starship prompt`に設定
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

The [Bash implementation](https://github.com/starship/starship/blob/master/src/init/starship.bash) built into Starship is slightly more complex to allow for advanced features like the [Command Duration module](https://starship.rs/config/#Command-Duration) and to ensure that Starship is compatible with pre-installed Bash configurations.

For a list of all flags accepted by `starship prompt`, use the following command:

```sh
starship prompt --help
```

The prompt will use as much context as is provided, but no flags are "required".

## How do I run Starship on Linux distributions with older versions of glibc?

If you get an error like "_version 'GLIBC_2.18' not found (required by starship)_" when using the prebuilt binary (for example, on CentOS 6 or 7), you can use a binary compiled with `musl` instead of `glibc`:

```sh
curl -fsSL https://starship.rs/install.sh | bash -s --- -platform unknown-linux-musl
```

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

- ロケールは、`de_DE.UTF-8`や` ja_JP.UTF-8</ 0>などのUTF-8値に設定されています。 <code>LC_ALL`がUTF-8値でない場合、[変更する必要があります](https://www.tecmint.com/set-system-locales-in-linux/)。
- 絵文字フォントがインストールされています。 ほとんどのシステムにはデフォルトで絵文字フォントが付属していますが、 一部(特にArch Linux) はそうではありません。 通常、システムの パッケージマネージャーからインストールすることができます--[noto emoji](https://www.google.com/get/noto/help/emoji/)は人気な選択肢です。
- [Nerd Font](https://www.nerdfonts.com/)を使用しています。

To test your system, run the following commands in a terminal:

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
