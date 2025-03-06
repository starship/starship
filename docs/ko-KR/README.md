---
layout: home
hero:
  image: /logo.svg
  text:
  tagline: 아무 셸에나 적용할 수 있는 간결하고, 매우 빠르며, 무한히 커스텀 가능한 프롬프트입니다!
  actions:
    - 
      theme: brand
      text: 시작하기 →
      link: ./guide/
features:
  - 
    title: 호환성 우선
    details: 거의 모든 운영 체제의 거의 모든 셸에서 동작합니다. 모든 곳에서 사용해 보세요!
  - 
    title: Rust 기반
    details: Rust의 최고 수준의 속도와 안정성으로 프롬프트를 가능한 한 빠르고 안정적으로 만들어 보세요.
  - 
    title: 커스텀 가능
    details: 모든 사소한 디테일들을 마음대로 커스텀할 수 있어, 프롬프트를 원하는 만큼 간단하게 만들거나 기능이 풍부하게 만들 수 있습니다.
footer: ISC Licensed | Copyright © 2019-present Starship Contributors
#Used for the description meta tag, for SEO
metaTitle: "Starship: 크로스-셸 프롬프트"
description: Starship은 아무 셸에나 적용할 수 있는 작고, 매우 빠르며, 무한히 커스텀 가능한 프롬프트입니다! 필요한 정보를 깔끔하고 간략하게 표시합니다. Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, 및 PowerShell에 빠르게 설치할 수 있습니다.
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

### 준비 사항

- 터미널에 [Nerd Font](https://www.nerdfonts.com/)가 설치되어 있고 사용 가능해야 합니다.

### 빠른 설치

1. **starship** 바이너리 설치:


   #### 최근 버전 설치

   셸로 설치:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Starship을 업데이트하고 싶은 경우에도 위의 스크립트를 실행시키면 됩니다. Starship의 설정은 변경되지 않고 버전만 최근 버전으로 대체될 것입니다.


   #### 패키지 매니저로 설치하기

   [Homebrew](https://brew.sh/)로 설치:

   ```sh
   brew install starship
   ```

   [Winget](https://github.com/microsoft/winget-cli)으로 설치:

   ```powershell
   winget install starship
   ```

1. 쉘 설정 파일에 init 스크립트 추가:


   #### Bash

   `~/.bashrc`의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   `~/.config/fish/config.fish`의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   `~/.zshrc`의 끝부분에 아래 라인을 추가

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   `Microsoft.PowerShell_profile.ps1`의 끝부분에 아래 내용을 추가합니다. 해당 설정파일은 파워쉘에서 `$PROFILE` 변수 확인을 통해 확인 가능합니다. 일반적으로 해당 파일은 `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` 혹은 -Nix의 경우 `~/.config/powershell/Microsoft.PowerShell_profile.ps1`에 있습니다.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   `~/.config/ion/initrc` 의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   elvish 버전 v0.18 이상에서만 지원됩니다.

   :::

   `~/.elvish/rc.elv` 의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   `~/.tcshrc` 의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   추후에 변경될 예정입니다. Only Nushell v0.96+ is supported.

   :::

   Add the following to the end of your Nushell configuration (find it by running `$nu.config-path` in Nushell):

   ```sh
   mkdir ($nu.data-dir | path join "vendor/autoload")
   starship init nu | save -f ($nu.data-dir | path join "vendor/autoload/starship.nu")
   ```


   #### Xonsh

   `~/.xonshrc` 의 끝부분에 아래 라인을 추가:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   Cmd를 이용하려면 [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) 를 사용해야 합니다. `starship.lua` 파일에 아래의 라인을 추가하고 파일을 Clink scripts 폴더에 저장합니다.

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
