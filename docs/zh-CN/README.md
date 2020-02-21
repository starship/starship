---
layout: Home
heroImage: /logo.svg
heroText:
tagline: The minimal, blazing-fast, and infinitely customizable prompt for any shell!
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
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### 快速安装

1. 安装 **starship** 二进制文件：


   #### 安装最新版本

   使用 Shell 命令：

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


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

   在 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` （对于 *nix 系统是`~/.config/powershell/Microsoft.PowerShell_profile.ps1`）的最后，添加以下内容：

   ```sh
   # ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   在 `~/.config/ion/initrc` 的最后，添加以下内容：

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
