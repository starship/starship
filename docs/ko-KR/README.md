---
home: true
heroImage: /logo.svg
heroText:
tagline: The minimal, blazing-fast, and infinitely customizable prompt for any shell!
actionText: Get Started →
actionLink: ./guide/
features:
  - 
    title: Compatibility First
    details: Works on the most common shells on the most common operating systems. Use it everywhere!
  - 
    title: Rust-Powered
    details: Brings the best-in-class speed and safety of Rust, to make your prompt as quick and reliable as possible.
  - 
    title: Customizable
    details: Every little detail is customizable to your liking, to make this prompt as minimal or feature-rich as you'd like it to be.
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: Starship is the minimal, blazing fast, and extremely customizable prompt for any shell! Shows the information you need, while staying sleek and minimal. Quick installation available for Bash, Fish, ZSH, Ion, and PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### 빠른 설치

1. **starship** 바이러니 설치:


   #### 최근 버전 설치

   With Shell:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   To update the Starship itself, rerun the above script. It will replace the current version without touching Starship's configuration.


   #### 패키지 매니저를 이용한 설치

   [Homebrew](https://brew.sh/)를 통한 설치:

   ```sh
   brew install starship
   ```

   With [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. 쉘 설정에 시동 스크립트를 추가:


   #### Bash

   `~/.bashrc`에 아래 라인을 추가

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   `~/.config/fish/config.fish`에 아래 라인을 추가

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   `~/.zshrc`에 아래 라인을 추가

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   `Microsoft.PowerShell_profile.ps1`의 끝부분에 아래 내용을 추가. 해당 설정파일은 파워쉘에서 `$PROFILE` 변수 확인을 통해 확인 가능. 일반적으로 해당 파일은 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` 혹은 -Nix의 경우 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`에 위치.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   아래의 라인을 `~/.config/ion/initrc` 마지막에 추가:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

   #### Elvish

   ::: 주의 elvish v0.15 이상만 지원. :::

   아래의 라인을 `~/.elvish/rc.elv` 마지막에 추가:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   아래의 라인을 `~/.tcshrc` 마지막에 추가:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```
