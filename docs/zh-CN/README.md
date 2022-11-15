---
home: true
heroImage: /logo.svg
heroText:
tagline: 轻量、迅速、可无限定制的高颜值终端！
actionText: 快速上手 →
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
footer: ISC 许可 | 版权所有 © 2019 - 目前 Starship 贡献者
#Used for the description meta tag, for SEO
metaTitle: "Starship：可用于各种 Shell 的提示符"
description: Starship是一款轻量级、反应迅速、可自定义的高颜值终端！ 只显示所需要的信息，将优雅和轻量化合二为一。 可以为Bash、Fish、ZSH、Ion、Tcsh、Elvish、Nu、Xonsh、Cmd和PowerShell执行快速安装。
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### 前置要求

- 安装并在你的终端启用 [Nerd Font](https://www.nerdfonts.com/) 。

### 快速安装

1. 安装 **starship** 二进制文件：


   #### 安装最新版本

   使用 Shell 命令：

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   自更新 Starship ，运行下面脚本 将会在不改动 Starship 配置文件的情况下升级版本


   #### 通过软件包管理器安装

   使用 [Homebrew](https://brew.sh/)：

   ```sh
   brew install starship
   ```
   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. 将初始化脚本添加到您的 shell 的配置文件：


   #### Bash

   在 `~/.bashrc` 的最后，添加以下内容：

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

   ::: warning

   仅支持 elvish 0.18 及更高的版本。

   :::

   在 `~/.config/fish/rc.elv` 的最后，添加以下内容：

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   在 `~/.tcshrc` 的最后，添加以下内容：

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   这部分今后可能会改变。 仅支持 Nushell v0.61+。

   :::

   将以下内容添加到您 Nushell 环境文件的末尾（使用 `$nu.env-path` 来获取它的路径）：
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save ~/.cache/starship/init.nu
   ```

   然后将以下内容添加到您 Nushell 配置文件的末尾（使用 `$nu.config-path` 来获取它的路径）：

   ```sh
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   在 `~/.bashrc` 的最后，添加以下内容：

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
