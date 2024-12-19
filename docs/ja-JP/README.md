---
layout: home
hero:
  image: /logo.svg
  text:
  tagline: シェル用の最小限の、非常に高速で、無限にカスタマイズ可能なプロンプトです！
  actions:
    - 
      theme: brand
      text: Get Started →
      link: ./guide/
features:
  - 
    title: 互換性優先
    details: 一般的なほとんどのOSの一般的なほとんどのシェル上で動作します。 あらゆるところで使用してください！
  - 
    title: Rust製
    details: Rustの最高レベルの速度と安全性を用いることで、可能な限り高速かつ信頼性を高くしています。
  - 
    title: カスタマイズ可能
    details: それぞれの細かい点は好みにカスタマイズが出来るため、ミニマルにも多機能にも好きなようにプロンプトを設定することができます。
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship はミニマルで、非常に高速で、カスタマイズ性の高い、あらゆるシェルのためのプロンプトです！ ミニマルかつ洗練された形で、あなたに必要な情報を表示します。 Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, PowerShellで簡単に利用できます。
---

<script setup>
import { onMounted } from 'vue'

onMounted(() => {
  const urlParams = new URLSearchParams(window.location.search)
  if (urlParams.has('uwu') || urlParams.has('kawaii')) {
    const img = document.querySelector('.VPHero .VPImage.image-src')
    img.classList.add('uwu')
    img.src = '/logo-uwu.png'
    img.alt = 'Kawaii Starship Logo by @sawaratsuki1004'
  }
})
</script>

<video class="demo-video" muted autoplay loop playsinline>
  <source src="/demo.webm" type="video/webm">
  <source src="/demo.mp4" type="video/mp4">
</video>

### 必要なもの

- [Nerd Font](https://www.nerdfonts.com/)の一つがインストールされていて、ターミナルで有効になっていること。

### クイックインストール

1. **Starship** のバイナリをインストール


   #### 最新版のインストール

   Shellを利用する

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Starship自体を更新するには、上記のスクリプトを再度実行してください。 最新のバージョンに置き換わり、設定ファイルには変更を加えません。


   #### パッケージマネージャー経由でインストール

   [ Homebrew](https://brew.sh/)を使用する

   ```sh
   brew install starship
   ```

   [Winget](https://github.com/microsoft/winget-cli)を使用する

   ```powershell
   winget install starship
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

   `Microsoft.PowerShell_profile.ps1` の最後に以下を追記してください。 PowerShell 上で `$PROFILE` 変数を問い合わせると、ファイルの場所を確認できます。 通常、パスは `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` または -Nix 上では `~/.config/powershell/Microsoft.PowerShell_profile.ps1` です。

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

   ::: warning

   elvish v0.18 以降のみサポートされます。

   :::

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


   #### Nushell

   ::: warning

   これは将来的に変更される可能性があります。 Nushell v0.78+ のみサポートされています。

   :::

   そして、Nushellの設定ファイルの最後に以下を追加してください（ `$nu.config-path` を実行してください）:

   ```sh
   mkdir ~/.cache/starship
   starship init nu | save -f ~/.cache/starship/init.nu
   ```

   そして、Nushellの設定ファイルの最後に以下を追記してください (`$nu.config-path` を実行してください):

   ```sh
   use ~/.cache/starship/init.nu
   ```


   #### Xonsh

   `~/.xonshrc` の最後に以下を追加してください:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) コマンドを使用する必要があります。 以下をファイル `starship.lua` に追加し、Clinkスクリプトディレクトリに配置します:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
