---
layout: Home
heroImage: /logo.svg
heroText:
tagline: The minimal, blazing-fast, and infinitely customizable prompt for any shell!
actionText: Get Started →
actionLink: ./guide/
features:
  - 
    title: 互換性優先
    details: 一般的なほとんどのOSの一般的なほとんどのシェル上で動作します。 あらゆるところで使用してください！
  - 
    title: Rust 製
    details: Rust の最高レベルの速度と安全性を用いることで、可能な限り高速かつ信頼性を高くしています。
  - 
    title: カスタマイズ可能
    details: それぞれの細かい点は好みにカスタマイズが出来るため、ミニマルにも多機能にも好きなようにプロンプトを設定することができます。
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship はミニマルで、非常に高速で、カスタマイズ性の高い、あらゆるシェルのためのプロンプトです！ ミニマルかつ洗練された形で、あなたに必要な情報を表示します。 Quick installation available for Bash, Fish, ZSH, Ion, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### クイックインストール

1. **Starship** のバイナリをインストール


   #### 最新版のインストール

   Shellを利用する

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### パッケージマネージャー経由でインストール

   [ Homebrew ](https://brew.sh/)の場合：

   ```sh
   brew install starship
   ```

   [ Scoop ](https://scoop.sh)の場合：

   ```powershell
   scoop install starship
   ```

1. 初期化のためのスクリプトをシェルの設定ファイルに追加


   #### Bash

   `~/.bashrc` の最後に以下を追記してください

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   `~/.config/fish/config.fish` の最後に以下を追記してください

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   `~/.zshrc` の最後に以下を追記してください

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` (それか Nix上の `~/.config/powershell/Microsoft.PowerShell_profile.ps1` )の末尾に以下を追加してください。

   ```sh
   # ~\Documents\PowerShell\Profile.ps1

   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   `~/.config/ion/initrc `の最後に次を追加してください

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
