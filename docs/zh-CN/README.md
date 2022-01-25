---
home: true
heroImage: /logo.svg
heroText:
tagline: 轻量级、反应迅速，可定制的高颜值终端！
actionText: 入门 →
actionLink: ./guide/
features:
  - 
    title: 兼容性优先
    details: Starship 可以在各种常见的操作系统和常见的 shell 上运行。 尝试着在各种地方使用它吧！
  - 
    title: 使用 Rust 编写
    details: 具有 Rust 独树一帜的速度与安全性，使你的提示符尽可能的快速可靠。
  - 
    title: 可自定义
    details: 每个小细节都可以按您喜欢的自定义，不论是最小化以求速度，还是最大化以获得最完善的功能。
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship：可用于各种 Shell 的提示符"
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### 前置要求

- 安装并在终端中启用 [Nerd Font](https://www.nerdfonts.com/)

### 快速安装

1. 安装 **starship** 二进制文件：


   #### 安装最新版本

   使用 Shell 命令：

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   自更新 Starship ，运行下面脚本 将会在不改动 Starship 配置文件的情况下升级版本


   #### 通过软件包管理器安装

   使用 [Homebrew](https://brew.sh/)：

   ```sh
   brew install starship
   ```

   使用 [Scoop](https://scoop.sh)：

   ```powershell
   scoop install starship
   ```

1. 将初始化脚本添加到您的 shell 的配置文件：


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


   #### Powershell

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


   #### Elvish

   ::: warning 仅支持 v0.17 或更高版本的 elvish。 :::

   在 `~/.config/fish/rc.elv` 的最后，添加以下内容：

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   在 `~/.bashhrc` 的最后，添加以下内容：

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning 这在将来会发生改变。仅支持 v0.33 或更高版本的 Nu。::: 

   在 nu 配置文件的最后加入以下内容。你可以通过 `config path` 获取配置文件的路径。

   ```toml
   startup = [
    "mkdir ~/.cache/starship",
    "starship init nu | save ~/.cache/starship/init.nu",
    "source ~/.cache/starship/init.nu"
   ]
   prompt = "starship_prompt"
   ```


   #### Xonsh

   在 `~/.xonshrc` 的末尾加入以下内容：

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   您需要使用 [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) 与 Cmd. 将以下文件添加到文件 `starship.lua` 中，并将此文件放置在 Clink脚本目录中：

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
