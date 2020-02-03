---
home: true
heroImage: /logo.svg
heroText: null
tagline: The cross-shell prompt for astronauts
actionText: 入门 →
actionLink: ./guide/
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
---

<div class="features">
  <div class="feature">
    <h2>注重兼容性</h2>
    <p>能在各种常见的 Shell 上运行。 尝试着在各种地方使用它吧！</p>
  </div>
  <div class="feature">
    <h2>Rust 制造</h2>
    <p>具有 Rust 独树一帜的速度与安全性。</p>
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


   #### Install Latest Version

   With Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Install via Package Manager

   With [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

    With [Scoop](https://scoop.sh):

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
