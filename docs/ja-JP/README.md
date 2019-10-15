---
home: true
heroImage: /logo.svg
actionText: Get Started →
actionLink: /ja/guide/
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
  <video class="demo-video" autoplay muted loop>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### クイックインストール

1. **Starship** のバイナリをインストール

   もし以下のプラットフォームを使用していない場合は **[コンパイル済みのバイナリファイルをダウンロード](https://github.com/starship/starship/releases)** してください。


   #### Homebrew

   ```sh
   $ brew install starship
   ```


   #### Rust (v1.33 もしくはそれ以上)

   ```sh
   $ cargo install starship
   ```


   #### Arch Linux (AUR)

   Starship は AUR 上の `starship` というパッケージ名で利用可能です。 `yay` またはお好きな AUR ヘルパーでインストールしてください。

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

   eval (starship init fish)
   ```


   #### Zsh

   `~/.zshrc` の最後に以下を追記してください

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Add the following to the end of `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` (or `~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix):

   ```sh
   # ~\Documents\PowerShell\Profile.ps1
   Invoke-Expression (&starship init powershell)
   ```
