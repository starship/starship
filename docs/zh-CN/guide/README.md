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
      alt="加入 Discord" /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Follow @StarshipPrompt on Twitter" /></a>
</p>

<p align="center">
  <a href="https://starship.rs">网站</a>
  ·
  <a href="#🚀-installation">安装</a>
  ·
  <a href="https://starship.rs/config/">配置</a>
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
  <a 
    href="https://github.com/starship/starship/blob/master/docs/es-ES/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Español" /></a>
  &nbsp;
  <a 
    href="https://github.com/starship/starship/blob/master/docs/fr-FR/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Français" /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="使用 iTerm 和 Snazz 主题的 Starship"
  width="50%"
  align="right" />

**轻量级、反应迅速，可定制的高颜值终端！**

- **快：** 很快 —— 真的真的非常快！ 🚀
- **定制化：** 可定制各种各样的提示符。
- **通用：** 适用于任何 Shell、任何操作系统。
- **智能：** 一目了然地显示相关信息。
- **功能丰富：** 支持所有你喜欢的工具。
- **易用：** 安装快速 —— 几分钟就可上手。

<p align="center">
<a href="https://starship.rs/config/"><strong>Explore the Starship docs&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀 安装

### 前置要求

- 安装一个 [Nerd Font](https://www.nerdfonts.com/) 的字体，并在终端启用（例如，可以尝试使用 [Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads) 字体）。

### 开始安装

**Note**: due to the proliferation of different platforms, only a subset of supported platforms are shown below. Can't see yours? Have a look at the [extra platform instructions](https://starship.rs/installing/).

1. 安装 **starship** 二进制文件：


   #### 安装最新版本


   ##### 下载预构建的二进制文件，使用 Shell 命令：

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### 通过软件包管理器安装


   ##### Example: [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### 使用 [Scoop](https://scoop.sh)：

   ```powershell
   scoop install starship
   ```

2. 将初始化脚本添加到您的 shell 的配置文件：


   #### Bash

   在 `~/.bashhrc` 的最后，添加以下内容：

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   在 `~/.config/fish/config.fish` 的最后，添加以下内容：

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   在 `~/.zshrc` 的最后，添加以下内容：

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   将以下内容添加到 `Microsoft.PowerShell_profile.ps1`。 你可以在 PowerShell 通过 `$PROFILE` 变量来查询文件的位置。 对于 -Nix 来说，通常文件路径是 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` 或 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`。

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   在 `~/.config/ion/initrc` 的最后，添加以下内容：

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

## 🤝 贡献

We are always looking for contributors of **all skill levels**! If you're looking to ease your way into the project, try out a [good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue).

If you are fluent in a non-English language, we greatly appreciate any help keeping our docs translated and up-to-date in other languages. If you would like to help, translations can be contributed on the [Starship Crowdin](https://translate.starship.rs/).

If you are interested in helping contribute to starship, please take a look at our [Contributing Guide](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Also, feel free to drop into our [Discord server](https://discord.gg/8Jzqu3T) and say hi. 👋

### 代码贡献者

This project exists thanks to all the people who contribute. [[Contribute](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)].
<a href="https://github.com/starship/starship/graphs/contributors"><img src="https://opencollective.com/starship/contributors.svg?width=890&button=false" /></a>

### 项目资助者

Become a financial contributor and help us sustain our community. [[Contribute](https://opencollective.com/starship/contribute)]

#### 资助人

<a href="https://opencollective.com/starship"><img src="https://opencollective.com/starship/individuals.svg?width=890"></a>

#### 资助组织

Support this project with your organization. Your logo will show up here with a link to your website. [[Contribute](https://opencollective.com/starship/contribute)]

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

## 💭该项目受以下项目启发

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
