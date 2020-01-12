---
home: true
heroImage: /logo.svg
actionText: Get Started →
actionLink: /ja-JP/guide/
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
---

<div class="features">
  <div class="feature">
    <h2>互換性優先</h2>
    <p>一般的なほとんどのOSの一般的なほとんどのシェル上で動作します。 あらゆるところで使用してください！</p>
  </div>
  <div class="feature">
    <h2>Rust 製</h2>
    <p>Rust の最高レベルの速度と安全性を用いることで、可能な限り高速かつ信頼性を高くしています。</p>
  </div>
  <div class="feature">
    <h2>カスタマイズ可能</h2>
    <p>それぞれの細かい点は好みにカスタマイズが出来るため、ミニマルにも多機能にも好きなようにプロンプトを設定することができます。</p>
  </div>
</div>

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### クイックインストール

1. **Starship** のバイナリをインストール


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

   Add the following to the end of `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```
