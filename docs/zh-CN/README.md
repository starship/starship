---
home: true
heroImage: /logo.svg
actionText: 入门 →
actionLink: /guide/
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

   如果您不使用下面的平台，你可以**[下载预编译的可执行文件](https://github.com/starship/starship/releases)**


   #### Homebrew

   ```sh
   $ brew install starship
   ```


   #### Rust (v1.38 or higher)

   ```sh
   $ cargo install starship
   ```


   #### Arch Linux (AUR)

   Starship可在AUR下以 `starship`"之名使用 使用`yay`或你最偏爱的AUR helper来安装。

   ```sh
   $ yay -S starship
   ```


   #### Nix (unstable)

   ```sh
   $ nix-env --install starship
   ```


   #### Termux

   ```sh
   $ pkg install starship
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
