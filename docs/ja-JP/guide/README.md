<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – Cross-shell prompt"
 />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/workflow/status/starship/starship/Main workflow/master?label=workflow&style=flat-square"
      alt="GitHub Actions workflow status"
 /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Crates.io version"
 /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Packaging status" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Chat on Discord"
 /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Follow @StarshipPrompt on Twitter"
 /></a>
</p>

<p align="center">
  <a href="https://starship.rs">ウェブサイト</a>
  ·
  <a href="#🚀-installation">インストール</a>
  ·
  <a href="https://starship.rs/config/">設定</a>
</p>

<p align="center">
  <a href="https://github.com/starship/starship/blob/master/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-us.png"
      alt="English"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ja-JP/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-jp.png"
      alt="日本語"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png"
      alt="繁體中文"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ru-RU/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ru.png"
      alt="Русский"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/de-DE/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png"
      alt="Deutsch"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-CN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-cn.png"
      alt="简体中文"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/es-ES/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Español"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/fr-FR/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Français"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/vi-VN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-vn.png"
      alt="Tiếng Việt"
 /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship with iTerm2 and the Snazzy theme"
  width="50%"
  align="right"
 />

**シェル用の最小限の、非常に高速で、無限にカスタマイズ可能なプロンプトです！**

- **高速:** _本当に_ 高速です！ 🚀
- **カスタマイズ可能:** プロンプトのあらゆる側面を構成します。
- **ユニバーサル:** あらゆるシェル、あらゆるオペレーティングシステムで動作します。
- **インテリジェント:** 関連情報を一目で示します。
- **豊富な機能:** お気に入りのツールをすべてサポートします。
- **簡単:** 迅速なインストールが可能であり、数分で使用開始可能です。

<p align="center">
<a href="https://starship.rs/config/"><strong>Starshipのドキュメントを見る&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀 インストール

### 必要なもの

- [Nerd Font](https://www.nerdfonts.com/)がインストールされ、端末にて有効になっている（例えば、[Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)を試してみてください）。

### 入門

**注意**: さまざまなプラットフォームが急増しているため、ここでは対応プラットフォームの一部だけを挙げています。 自分が使っているシェルがありませんか？ [extra platform instructions](https://starship.rs/installing/) を確認してください。

1. **Starship** のバイナリをインストール


   #### 最新版のインストール


   ##### ビルド済みのバイナリをインストール

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   Starship自体を更新するには、上記のスクリプトを再度実行してください。 最新のバージョンに置き換わり、設定ファイルには変更を加えません。


   **※**インストールスクリプトのデフォルト値は上書きすることができます。詳しくはヘルプを参照してください。

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --help
   ```


   #### パッケージマネージャー経由でインストール


   ##### 例: [Homebrew の場合](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### [ Scoop ](https://scoop.sh)の場合：

   ```powershell
   scoop install starship
   ```

2. 初期化のためのスクリプトをシェルの設定ファイルに追加


   #### Bash

   `~/.bashrc` の最後に以下を追記してください

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   `~/.config/fish/config.fish` の最後に以下を追記してください

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   `~/.zshrc` の最後に以下を追記してください

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   `Microsoft.PowerShell_profile.ps1` の最後に以下を追記してください。 PowerShell 上で `$PROFILE` 変数を問い合わせると、ファイルの場所を確認できます。 通常、パスは `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` または -Nix 上では `~/.config/powershell/Microsoft.PowerShell_profile.ps1` です。

   ```powershell
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   `~/.config/ion/initrc `の最後に次を追加してください

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   **警告** elvish v0.15 以上のみがサポートされています。 `~/.elvish/rc.elv` の最後に以下を追記してください。

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   `~/.tcshrc` の最後に以下を追加します:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Xonsh

   Add the following to the end of `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Nushell

   **Warning** This will change in the future. Only nu version v0.33 or higher is supported. Add the following to your nu config file. You can check the location of this file by running `config path` in nu.

   ```toml
   startup = [
    "mkdir ~/.cache/starship",
    "starship init nu | save ~/.cache/starship/init.nu",
    "source ~/.cache/starship/init.nu"
   ]
   prompt = "starship_prompt"
   ```

## 🤝 貢献

私たちは常に**すべてのスキルレベル**の貢献者を探しています！ もし簡単にプロジェクトへ参加する方法をお探しなら、 [good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue) に取り組んでみてください。

あなたが英語以外の言語に堪能な場合、ドキュメントの翻訳と更新に協力していただけると嬉しいです。 協力してくれる場合、翻訳は [Starship Crowdin](https://translate.starship.rs/) から貢献できます。

もしあなたが Starship への貢献に興味がある場合は、我々の[貢献ガイド](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)をご覧ください。 また、気軽に我々の[Discord サーバー](https://discord.gg/8Jzqu3T)へ顔を出してください。 👋

### コードに貢献していただいた方々

このプロジェクトは、貢献したすべての人々のおかげで存在します。 [[Contribute](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)].
<a href="https://github.com/starship/starship/graphs/contributors"><img src="https://opencollective.com/starship/contributors.svg?width=890&button=false" /></a>

### 財政的な貢献をしていただいた方々

財政的な貢献者になって、私たちのコミュニティを維持するのを助けていただけると嬉しいです。 [[Contribute](https://opencollective.com/starship/contribute)]

#### 個人

<a href="https://opencollective.com/starship"><img src="https://opencollective.com/starship/individuals.svg?width=890"></a>

#### 組織

あなたの組織でこのプロジェクトをサポートします。 あなたのロゴはあなたのウェブサイトへのリンクとここに表示されます。 [[Contribute](https://opencollective.com/starship/contribute)]

<a href="https://opencollective.com/starship/organization/0/website"><img src="https://opencollective.com/starship/organization/0/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/1/website"><img src="https://opencollective.com/starship/organization/1/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/2/website"><img src="https://opencollective.com/starship/organization/2/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/3/website"><img src="https://opencollective.com/starship/organization/3/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/4/website"><img src="https://opencollective.com/starship/organization/4/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/5/website"><img src="https://opencollective.com/starship/organization/5/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/6/website"><img src="https://opencollective.com/starship/organization/6/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/7/website"><img src="https://opencollective.com/starship/organization/7/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/8/website"><img src="https://opencollective.com/starship/organization/8/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/9/website"><img src="https://opencollective.com/starship/organization/9/avatar.svg"></a>

## 💭影響を受けたプロダクト

よければStarship の作成に影響を与えた、これまでのプロジェクトをチェックしてください 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - 宇宙飛行士のための ZSH プロンプト。

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - 多くの shell に対応した JavaScript で書かれた robbyrussell テーマ。

- **[reujab/silver](https://github.com/reujab/silver)** - 多くの shell に対応しているカスタマイズ可能でアイコンを表示できる powerline のようなプロンプト。

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 ライセンス

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> このプロジェクトは [ISC](https://github.com/starship/starship/blob/master/LICENSE) でライセンスされています。
