---
home: true
heroImage: /logo.svg
heroText:
tagline: 適合任何 shell 的最小、極速、無限客製化的提示字元！
actionText: 馬上開始 →
actionLink: ./guide/
features:
  - 
    title: 相容性優先
    details: 能夠在最常見的作業系統之中最常見的 shell 上運作。 在各處使用它吧！
  - 
    title: 以 Rust 開發
    details: 帶來同類中最快的速度以及 Rust 的安全性，讓你的提示字元盡可能快速與可靠。
  - 
    title: 可客製化
    details: 任何些微的細節都可以隨你喜愛地客製化，讓你的提示字元可以隨你所欲地最小化或是充滿各種特色。
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship：跨 Shell 提示字元"
description: Starship 是適合任何 shell 的最小、極速、高度客製化的提示字元！ 顯示你需要的訊息，同時保持順暢與最小化。 Quick installation available for Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### 先決要求

- 安裝至少一個 [Nerd Font](https://www.nerdfonts.com/) 字體，並在終端中啟用。

### 快速安裝

1. 安裝 **starship** 執行檔：


   #### 安裝最新版本

   使用 Shell 安裝：

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   如果想更新已安裝的 Starship，請重新執行上述指令。 指令只會更新 Starship 執行檔本身，不會影響到任何已撰寫的設定檔。


   #### 使用套件管理器安裝：

   使用 [Homebrew](https://brew.sh/)：

   ```sh
   brew install starship
   ```
   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
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

   將以下內容放到 `Microsoft.PowerShell_profile.ps1` 的結尾。 你可以藉由在 PowerShell 查詢 `$PROFILE` 變數以取得這個檔案的位置。 一般來說，檔案會出現在 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1`，若是在 -Nix 上，檔案則會出現在 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`。

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   將以下內容放到 `~/.config/ion/initrc` 的結尾：

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Only elvish v0.18 or higher is supported.

   :::

   將以下內容放到 `~/.elvish/rc.elv` 的結尾：

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   將以下內容放到 `~/.tcshrc` 的結尾：

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   This will change in the future. Only Nushell v0.61+ is supported.

   :::

   Add the following to to the end of your Nushell env file (find it by running `$nu.env-path` in Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save ~/.cache/starship/init.nu
   ```

   And add the following to the end of your Nushell configuration (find it by running `$nu.config-path`):

   ```sh
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   將以下內容加到 `~/.xonshrc` 的結尾：

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### 命令提示字元

   You need to use [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) with Cmd. Add the following to a file `starship.lua` and place this file in Clink scripts directory:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
