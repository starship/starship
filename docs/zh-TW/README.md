---
home: true
heroImage: /logo.svg
actionText: Get Started →
actionLink: /zh-TW/guide/
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
---

<div class="features">
  <div class="feature">
    <h2>兼容性優先</h2>
    <p>能夠在最常見的作業系統之中最常見的 shell 上運作。 到處使用它吧！</p>
  </div>
  <div class="feature">
    <h2>由 Rust 支持</h2>
    <p>帶來同類最快速度以及 Rust 的安全性，讓你的提示字元盡可能快速與可靠。</p>
  </div>
  <div class="feature">
    <h2>可客製化</h2>
    <p>任何些微的細節都可以隨你喜愛地客製化，讓你的提示字元可以隨你所欲地最小化或是充滿各種特色。</p>
  </div>
</div>

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### 快速安裝

1. 安裝 **starship** 執行檔：

   如果你並非使用以下平台，請直接**[下載編譯好的執行檔的壓縮檔](https://github.com/starship/starship/releases)**。


   #### Homebrew

   ```sh
   $ brew install starship
   ```


   #### Rust (v1.38 或更高版本)

   ```sh
   $ cargo install starship
   ```


   #### Arch Linux (AUR)

   Starship 在 AUR 中已經以 `starship` 這個名字存在。 請使用 `yay` 或你喜歡的 AUR 幫手安裝。

   ```sh
   $ yay -S starship
   ```


   #### Nix (不穩定)

   ```sh
   $ nix-env --install starship
   ```


   #### Termux

   ```sh
   $ pkg install starship
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


   #### Powershell

   將以下內容放到 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` 的結尾 (或是在 -Nix 上的 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`)：

   ```sh
   # ~\Documents\PowerShell\Profile.ps1
   Invoke-Expression (&starship init powershell)
   ```
