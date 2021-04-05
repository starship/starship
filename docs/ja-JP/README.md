---
home: true
heroImage: /logo.svg
heroText:
tagline: シェル用の最小限の、非常に高速で、無限にカスタマイズ可能なプロンプトです！
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
description: Starship はミニマルで、非常に高速で、カスタマイズ性の高い、あらゆるシェルのためのプロンプトです！ ミニマルかつ洗練された形で、あなたに必要な情報を表示します。 Bash, Fish, ZSH, Ion, PowerShellへ簡単にインストール出来ます。
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
   To update the Starship itself, rerun the above script. It will replace the current version without touching Starship's configuration.


   #### パッケージマネージャー経由でインストール

   With [Homebrew](https://brew.sh/):

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

   Add the following to the end of `Microsoft.PowerShell_profile.ps1`. You can check the location of this file by querying the `$PROFILE` variable in PowerShell. 通常、パスは `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` または -Nix 上では `~/.config/powershell/Microsoft.PowerShell_profile.ps1` です。

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   `~/.config/ion/initrc `の最後に次を追加してください

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

   #### Elvish

   ::: warning Only elvish v0.15 or higher is supported. :::

   `~/.elvish/rc.elv` の最後に以下を追記してください。

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   `~/.tcshrc` の最後に以下を追加します:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```
