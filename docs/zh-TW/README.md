---
home: true
heroImage: /logo.svg
heroText:
tagline: 適合任何 shell 的最小、極速、無限客製化的提示字元！
actionText: 馬上開始 →
actionLink: ./guide/
features:
  - 
    title: 兼容性優先
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
description: Starship 是適合任何 shell 的最小、極速、高度客製化的提示字元！ 顯示你需要的訊息，同時保持順暢與最小化。 針對 Bash、Fish、ZSH、Ion、Tcsh、Elvish、Nu、Xonsh 以及 Powershell 有快速安裝可供使用。
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### 先決要求

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal.

### 快速安裝

1. 安裝 **starship** 執行檔：


   #### 安裝最新版本

   使用 Shell 安裝：

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   如果想更新已安裝的 Starship，請重新執行上述指令。 指令只會更新 Starship 執行檔本身，不會影響到任何已撰寫的設定檔。


   #### 使用套件管理器安裝：

   使用 [Homebrew](https://brew.sh/)：

   ```sh
   brew install starship
   ```

   使用 [Scoop](https://scoop.sh)：

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


   #### Powershell

   將以下內容放到 `Microsoft.PowerShell_profile.ps1` 的結尾。 你可以藉由在 PowerShell 查詢 `$PROFILE` 變數以取得這個檔案的位置。 一般來說檔案會出現在 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` 或是在 -Nix 上的話會在 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`。

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

   ::: warning 只有 elvish v0.15 或以上版本才有支援 :::

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

   ::: warning This will change in the future. 只支援 nu v0.33 以上的版本。 ::: Add the following to your nu config file. 你可以透過在 nu 執行 `config path` 指令來取得設定檔的位置。

   ```toml
   startup = [
    "mkdir ~/.cache/starship",
    "starship init nu | save ~/.cache/starship/init.nu",
    "source ~/.cache/starship/init.nu"
   ]
   prompt = "starship_prompt"
   ```


   #### Xonsh

   Add the following to the end of `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```
