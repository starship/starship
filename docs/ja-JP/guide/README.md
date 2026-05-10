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
      src="https://img.shields.io/github/actions/workflow/status/starship/starship/workflow.yml?branch=master&label=workflow&style=flat-square"
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
  <a href="https://stand-with-ukraine.pp.ua"
    ><img
      src="https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraineFlat.svg"
      alt="ウクライナと共に歩む"
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
    href="https://github.com/starship/starship/blob/master/docs/de-DE/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png"
      alt="Deutsch"
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
    href="https://github.com/starship/starship/blob/master/docs/id-ID/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-id.png"
      alt="Bahasa Indonesia"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/it-IT/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-it.png"
      alt="Italiano"
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
    href="https://github.com/starship/starship/blob/master/docs/pt-BR/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-br.png"
      alt="Português do Brasil"
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
    href="https://github.com/starship/starship/blob/master/docs/uk-UA/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ua.png"
      alt="Українська"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/vi-VN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-vn.png"
      alt="Tiếng Việt"
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
    href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png"
      alt="繁體中文"
 /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship with iTerm2 and the Snazzy theme"
  width="50%"
  align="right"
 />

**最小限の、非常に高速で、無限にカスタマイズ可能なシェル用のプロンプトです！**

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

- [Nerd Font](https://www.nerdfonts.com/)がインストールされ、端末にて有効になっている（例えば、[FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)を試してみてください）。

### Step 1. Starshipをインストール

以下のリストからお使いのOSを選択し、インストール方法をご確認ください。

<details>
<summary>Android</summary>

以下のいずれかのパッケージマネージャーを使用して、Starshipをインストールします。

| リポジトリ                                                                             | 手順                     |
| --------------------------------------------------------------------------------- | ---------------------- |
| [Termux](https://github.com/termux/termux-packages/tree/master/packages/starship) | `pkg install starship` |

</details>

<details>
<summary>BSD</summary>

以下のいずれかのパッケージマネージャーを使用して、Starshipをインストールします。

| ディストリビューション | リポジトリ                                                    | 手順                                |
| ----------- | -------------------------------------------------------- | --------------------------------- |
| **_Any_**   | **[crates.io](https://crates.io/crates/starship)**       | `cargo install starship --locked` |
| FreeBSD     | [FreshPorts](https://www.freshports.org/shells/starship) | `pkg install starship`            |
| NetBSD      | [pkgsrc](https://pkgsrc.se/shells/starship)              | `pkgin install starship`          |

</details>

<details>
<summary>Linux</summary>

お使いのシステムに合わせた最新版をインストールしてください。

```sh
curl -sS https://starship.rs/install.sh | sh
```

または、以下のパッケージマネージャを使用して、Starshipをインストールします。

| ディストリビューション        | リポジトリ                                                                                           | 手順                                                                             |
| ------------------ | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| **_Any_**          | **[crates.io](https://crates.io/crates/starship)**                                              | `cargo install starship --locked`                                              |
| _Any_              | [conda-forge](https://anaconda.org/conda-forge/starship)                                        | `conda install -c conda-forge starship`                                        |
| _Any_              | [Linuxbrew](https://formulae.brew.sh/formula/starship)                                          | `brew install starship`                                                        |
| Alpine Linux 3.13+ | [Alpine Linux Packages](https://pkgs.alpinelinux.org/packages?name=starship)                    | `apk add starship`                                                             |
| Arch Linux         | [Arch Linux Extra](https://archlinux.org/packages/extra/x86_64/starship)                        | `pacman -S starship`                                                           |
| CentOS 7+          | [Copr](https://copr.fedorainfracloud.org/coprs/atim/starship)                                   | `dnf copr enable atim/starship` <br /> `dnf install starship` |
| Debian 13+         | [Debian Main](https://sources.debian.org/src/starship/1.22.1-1/)                                | `apt install starship`                                                         |
| Fedora 40+         | [Copr](https://copr.fedorainfracloud.org/coprs/atim/starship)                                   | `dnf copr enable atim/starship` <br /> `dnf install starship` |
| Gentoo             | [Gentoo Packages](https://packages.gentoo.org/packages/app-shells/starship)                     | `emerge app-shells/starship`                                                   |
| Manjaro            |                                                                                                 | `pacman -S starship`                                                           |
| NixOS              | [nixpkgs](https://github.com/NixOS/nixpkgs/blob/master/pkgs/by-name/st/starship/package.nix)    | `nix-env -iA nixpkgs.starship`                                                 |
| openSUSE           | [OSS](https://software.opensuse.org/package/starship)                                           | `zypper in starship`                                                           |
| Ubuntu 25.04+      | [Ubuntu Universe](https://packages.ubuntu.com/source/plucky/starship)                           | `apt install starship`                                                         |
| Void Linux         | [Void Linux Packages](https://github.com/void-linux/void-packages/tree/master/srcpkgs/starship) | `xbps-install -S starship`                                                     |

</details>

<details>
<summary>macOS</summary>

お使いのシステムに合わせた最新版をインストールしてください。

```sh
curl -sS https://starship.rs/install.sh | sh
```

または、以下のパッケージマネージャを使用して、Starshipをインストールします。

| リポジトリ                                                    | 手順                                      |
| -------------------------------------------------------- | --------------------------------------- |
| **[crates.io](https://crates.io/crates/starship)**       | `cargo install starship --locked`       |
| [conda-forge](https://anaconda.org/conda-forge/starship) | `conda install -c conda-forge starship` |
| [Homebrew](https://formulae.brew.sh/formula/starship)    | `brew install starship`                 |
| [MacPorts](https://ports.macports.org/port/starship)     | `port install starship`                 |

</details>

<details>
<summary>Windows</summary>

[Releasesセクション](https://github.com/starship/starship/releases/latest) のMSIインストーラーでシステムに合った最新バージョンをインストールしてください。

以下のいずれかのパッケージマネージャーを使用して、Starshipをインストールします。

| リポジトリ                                                                                        | 手順                                      |
| -------------------------------------------------------------------------------------------- | --------------------------------------- |
| **[crates.io](https://crates.io/crates/starship)**                                           | `cargo install starship --locked`       |
| [Chocolatey](https://community.chocolatey.org/packages/starship)                             | `choco install starship`                |
| [conda-forge](https://anaconda.org/conda-forge/starship)                                     | `conda install -c conda-forge starship` |
| [Scoop](https://github.com/ScoopInstaller/Main/blob/master/bucket/starship.json)             | `scoop install starship`                |
| [winget](https://github.com/microsoft/winget-pkgs/tree/master/manifests/s/Starship/Starship) | `winget install --id Starship.Starship` |

</details>

### Step 2. Starshipをシェルにセットアップ

Starshipを初期化するためのシェルの設定。 以下のリストからお使いのシェルを選択してください。

<details>
<summary>Bash</summary>

`~/.bashrc` の最後に以下を追記してください

```sh
eval "$(starship init bash)"
```

</details>

<details>
<summary>Cmd</summary>

[Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) コマンドを使用する必要があります。 このパス`%LocalAppData%\clink\starship.lua` に以下の内容を含むファイルを作成する。

```lua
load(io.popen('starship init cmd'):read("*a"))()
```

</details>

<details>
<summary>Elvish</summary>

Add the following to the end of `~/.config/elvish/rc.elv` (`%AppData%\elvish\rc.elv` on Windows):

```sh
eval (starship init elvish)
```

注意: Elvish v0.18以降でサポートされています. For elvish versions prior to v0.21.0 the config file might instead be `~/.elvish/rc.elv`

</details>

<details>
<summary>Fish</summary>

`~/.config/fish/config.fish` の最後に以下を追記してください

```fish
starship init fish | source
```

</details>

<details>
<summary>Ion</summary>

`~/.config/ion/initrc `の最後に次を追加してください

```sh
eval $(starship init ion)
```

</details>

<details>
<summary>Nushell</summary>

Add the following to the end of your Nushell configuration (find it by running `$nu.config-path` in Nushell):

```sh
mkdir ($nu.data-dir | path join "vendor/autoload")
starship init nu | save -f ($nu.data-dir | path join "vendor/autoload/starship.nu")
```

注意: Nushell v0.96以降でサポートされています

</details>

<details>
<summary>PowerShell</summary>

PowerShellの設定ファイルの最後に以下を追加してください（ `$PROFILE` を実行してください）:

```powershell
Invoke-Expression (&starship init powershell)
```

</details>

<details>
<summary>Tcsh</summary>

`~/.tcshrc` の最後に以下を追加します:

```sh
eval `starship init tcsh`
```

</details>

<details>
<summary>Xonsh</summary>

`~/.xonshrc` の最後に以下を追加してください:

```python
execx($(starship init xonsh))
```

</details>

<details>
<summary>Zsh</summary>

`~/.zshrc` の最後に以下を追記してください

```sh
eval "$(starship init zsh)"
```

</details>

### Step 3. Starshipの設定

新しいシェルインスタンスを起動すると、美しくて新しいシェルプロンプトが表示されるはずです。 デフォルトで問題なければ、お楽しみください！

Starshipのさらなるカスタマイズを目指すなら以下を参考にしてみてください。

- **[設定](https://starship.rs/config/)** - あなたの好みに応じてプロンプトを調整するためにStarshipを設定する方法を学ぶ

- **[プリセット](https://starship.rs/presets/)** - コミュニティのために用意されたコンフィグから刺激を受ける

## 🤝 貢献

私たちは常に**すべてのスキルレベル**の貢献者を探しています！ If you're looking to ease your way into the project, try out a [good first issue](https://github.com/starship/starship/issues?q=state%3Aopen%20label%3A%22%F0%9F%8C%B1%20good%20first%20issue%22).

あなたが英語以外の言語に堪能な場合、ドキュメントの翻訳と更新に協力していただけると嬉しいです。 協力してくれる場合、翻訳は [Starship Crowdin](https://translate.starship.rs/) から貢献できます。

もしあなたが Starship への貢献に興味がある場合は、我々の[貢献ガイド](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)をご覧ください。 また、気軽に我々の[Discord サーバー](https://discord.gg/8Jzqu3T)へ顔を出してください。 👋

## 💭影響を受けたプロダクト

よければStarship の作成に影響を与えた、これまでのプロジェクトをチェックしてください 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** – 宇宙飛行士のための ZSH プロンプト。

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** – JavaScript で書かれた Cross-shell 対応の robbyrussell テーマ。

- **[reujab/silver](https://github.com/reujab/silver)** – アイコンと cross-shell 対応のカスタマイズ可能な powerline ライクなプロンプト。

## ❤️ スポンサー

[スポンサーになることで](https://github.com/sponsors/starship)このプロジェクトをサポートできます。 名前やロゴがあなたのホームページへのリンクとともに表示されます。

## 🔒 コード署名のポリシー

[SignPath.io](https://signpath.io)での無料のコード署名が[SignPath Foundation](https://signpath.org)によって提供されています。

コード署名のロール：

- レビュワー：[Astronauts](https://github.com/orgs/starship/teams/astronauts)
- Approver、Author：[Mission Control](https://github.com/orgs/starship/teams/mission-control)

This program will not transfer any information to other networked systems unless specifically requested by the user or the person installing or operating it.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 ライセンス

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> このプロジェクトは [ISC](https://github.com/starship/starship/blob/master/LICENSE) でライセンスされています。
