<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – Cross-shell prompt" />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/workflow/status/starship/starship/Main workflow/master?label=workflow&style=flat-square"
      alt="GitHub Actions workflow status" /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Crates.io version" /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Packaging status" /></a
><br />
  <a href="https://discord.gg/8Jzqu3T"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Chat on Discord" /></a>
  <a href="https://github.com/starship/starship/graphs/contributors"
    ><img
      src="https://img.shields.io/github/contributors/starship/starship?style=flat-square"
      alt="Contributors" /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Website</a>
  ·
  <a href="#-installation">Installation</a>
  ·
  <a href="https://starship.rs/config/">Configuration</a>
</p>

<p align="center">
  <a href="https://github.com/starship/starship/blob/master/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-us.png"
      alt="English" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ja-JP/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-jp.png"
      alt="日本語" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png"
      alt="繁體中文" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ru-RU/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ru.png"
      alt="Русский" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/de-DE/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png"
      alt="Deutsch" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-CN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-cn.png"
      alt="简体中文" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/es"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Español" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/fr"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Français" /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship with iTerm2 and the Snazzy theme"
  width="50%"
  align="right" />


**The minimal, blazing-fast, and infinitely customizable prompt for any shell!**


- **Fast:** it's fast – _really really_ fast! 🚀
- **Customizable:** configure every aspect of your prompt.
- **Universal:** works on any shell, on any operating system.
- **Intelligent:** shows relevant information at a glance.
- **Feature rich:** support for all your favorite tools.
- **Easy:** quick to install – start using it in minutes.

<p align="center">
<a href="https://starship.rs/"><strong>Explore the Starship docs&nbsp;&nbsp;▶</strong></a>
</p>

## 🚀 Installation

### 必要なもの

- [Powerline フォント](https://github.com/powerline/fonts) がターミナルにインストールされて有効になっている必要があります（例えば [Fira Code](https://github.com/tonsky/FiraCode) を試してみてください）。

### 入門

1. **Starship** のバイナリをインストール


   #### 最新版のインストール


   ##### ビルド済みのバイナリをインストール

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   ##### [crates.io](https://crates.io/)からソースをインストール

   ```sh
   cargo install starship
   ```


   #### パッケージマネージャー経由でインストール


   ##### [ Homebrew ](https://brew.sh/)の場合：

   ```sh
   brew install starship
   ```


   ##### [ Scoop ](https://scoop.sh)の場合：

   ```powershell
   scoop install starship
   ```

1. 初期化のためのスクリプトをシェルの設定ファイルに追加


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

   `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` (それか Nix上の `~/.config/powershell/Microsoft.PowerShell_profile.ps1` )の末尾に以下を追加してください。

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   `~/.config/ion/initrc `の最後に次を追加してください

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

## 🤝 Contributing

We are always looking for contributors of **all skill levels**! If you're looking to ease your way into the project, try out a [good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue).

If you are interested in helping contribute to starship, please take a look at our [Contributing Guide](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Also, feel free to drop into our [Discord server](https://discord.gg/8Jzqu3T) and say hi. 👋

## 💭 Inspired By

Please check out these previous works that helped inspire the creation of starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - A ZSH prompt for astronauts.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - Cross-shell robbyrussell theme written in JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - A cross-shell customizable powerline-like prompt with icons.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 License

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> This project is [ISC](https://github.com/starship/starship/blob/master/LICENSE) licensed.
