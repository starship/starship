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
      alt="在 Discord 上聊天" /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Follow @StarshipPrompt on Twitter" /></a>
</p>

<p align="center">
  <a href="https://starship.rs/zh-TW/">網站</a>
  ·
  <a href="#-安裝">安裝</a>
  ·
  <a href="https://starship.rs/zh-TW/config/">設定</a>
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

## 🚀 安裝

### 先決要求

- 一個已經安裝並在你的終端機內啟動的 [Powerline 字型](https://github.com/powerline/fonts) (例如，試試看 [Fira Code](https://github.com/tonsky/FiraCode))。

### 入門

1. 安裝 **starship** 執行檔：


   #### Install Latest Version


   ##### From prebuilt binary, with Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   ##### From source on [crates.io](https://crates.io/):

   ```sh
   cargo install starship
   ```


   #### Install via Package Manager


   ##### With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### With [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. 將初始化腳本 (script) 加入你的 shell 的設定檔：


   #### Bash

   將以下內容放到 `~/.bashrc` 的結尾：

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   將以下內容放到 `~/.config/fish/config.fish` 的結尾：

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   將以下內容放到 `~/.zshrc` 的結尾：

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   將以下內容放到 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` 的結尾 (或是在 -Nix 上的 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`)：

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Add the following to the end of `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

## 🤝 貢獻

我們歡迎具有**各式各樣能力**的貢獻者！ 如果你正在尋找容易加入的方法，試試看標註為「[good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue)」的 issue。

如果你對貢獻 Starship 有興趣，請看我們的 [貢獻指南](https://github.com/starship/starship/blob/master/CONTRIBUTING.md) 。 另外，請不用客氣加入我們的 [Discord 伺服器](https://discord.gg/8Jzqu3T) 並來問候一下。 👋

## 💭 發想來自

請看之前這些幫助我們創造 Starship 的前任作品。 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - 給太空人的 ZSH 提示。

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - 使用 Javascript 撰寫的跨 shell robbyrussell 主題。

- **[reujab/silver](https://github.com/reujab/silver)** - 一個跨 shell、可客製化、像 powerline 的圖案提示字元。

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 許可

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> 這個專案使用 [ISC](https://github.com/starship/starship/blob/master/LICENSE) 許可。
