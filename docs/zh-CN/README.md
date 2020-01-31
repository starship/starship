---
home: true
heroImage: /logo.svg
actionText: 入门 →
actionLink: /guide/
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
---

<div class="features">
  <div class="feature">
    <h2>兼容性优先</h2>
    <p>Starship 可以在各种常见的操作系统和常见的 shell 上运行。 尝试着在各种地方使用它吧！</p>
  </div>
  <div class="feature">
    <h2>使用 Rust 编写</h2>
    <p>具有 Rust 独树一帜的速度与安全性，使你的提示符尽可能的快速可靠。</p>
  </div>
  <div class="feature">
    <h2>可自定义</h2>
    <p>每个小细节都可以按您喜欢的自定义，不论是最小化以求速度，还是最大化以获得最完善的功能。</p>
  </div>
</div>

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

   通过 [Homebrew](https://brew.sh/) 安装：

   ```sh
   brew install starship
   ```

    通过 [Scoop](https://scoop.sh) 安装：

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

   添加 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` (或者`~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix)到：

   ```sh
   # ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Add the following to the end of `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
