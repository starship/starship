---
layout: home
hero:
  image: /logo.svg
  text: null
  tagline: Кез келген қабықша үшін шағын, аса жылдам әрі шексіз баптауға болатын командалық жол шақыруы!
  actions:
    - theme: brand
      text: Жұмысты бастау →
      link: ./kk-KZ/guide/
features:
  - title: Ең алдымен үйлесімділік
    details: Ең танымал операциялық жүйелердегі ең көп таралған қабықшалардың барлығында жұмыс істейді. Барлық жерде қолданыңыз!
  - title: Rust қуатымен жұмыс істейді
    details: Промптыңызды барынша жылдам әрі сенімді ету үшін Rust тілінің озық жылдамдығы мен қауіпсіздігін ұсынады.
  - title: Шексіз бапталады
    details: Промпты өз қалауыңызша барынша ықшам немесе функционалға бай ету үшін кез келген ұсақ-түйек бөлшекті өзгертуге болады.
footer: ISC Лицензиясы | Авторлық құқық © 2019-қазіргі уақыт Starship қауымдастығы

# Used for the description meta tag, for SEO
metaTitle: "Starship: Барлық қабықшаларға арналған промпт (Cross-Shell Prompt)"
description: Starship — кез келген қабықша үшін шағын, аса жылдам әрі шексіз баптауға болатын командалық жол шақыруы! Қажетті ақпаратты жинақы әрі көрнекі түрде көрсетеді. Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd және PowerShell үшін жылдам орнату қолжетімді.
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

### Қажетті алғышарттар

- Терминалыңызда [Nerd Font](https://www.nerdfonts.com/) қарпі орнатылған және қосылған болуы қажет.

### Жылдам орнату

1. **starship** орындалатын файлын орнатыңыз:

   #### Ең соңғы нұсқасын орнату

   Қабықша (Shell) арқылы:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Starship-тің өзін жаңарту үшін жоғарыдағы скриптті қайта іске қосыңыз. Ол Starship баптауларына тиіспестен ағымдағы нұсқаны жаңартады.

   #### Пакет менеджері арқылы орнату

   [Homebrew](https://brew.sh/) арқылы:

   ```sh
   brew install starship
   ```

   [Winget](https://github.com/microsoft/winget-cli) арқылы:

   ```powershell
   winget install starship
   ```

1. Қабықшаңыздың конфигурациялық файлына инициализация скриптін қосыңыз:

   #### Bash

   `~/.bashrc` файлының соңына келесі жолды қосыңыз:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```

   #### Fish

   `~/.config/fish/config.fish` файлының соңына келесі жолды қосыңыз:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```

   #### Zsh

   `~/.zshrc` файлының соңына келесі жолды қосыңыз:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```

   #### PowerShell

   `Microsoft.PowerShell_profile.ps1` файлының соңына келесі жолды қосыңыз. Бұл файлдың орнын PowerShell ішінде `$PROFILE` айнымалысын тексеру арқылы көре аласыз. Әдетте бұл жол Windows-та `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` немесе Unix тәрізді жүйелерде `~/.config/powershell/Microsoft.PowerShell_profile.ps1` болады.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```

   #### Ion

   `~/.config/ion/initrc` файлының соңына келесі жолды қосыңыз:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

   #### Elvish
   > [!WARNING]
   > Тек Elvish v0.18 немесе одан жоғары нұсқаларына қолдау көрсетіледі.

   `~/.config/elvish/rc.elv` (Windows жүйесінде `%AppData%\elvish\rc.elv`) файлының соңына келесі жолды қосыңыз:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```
   v0.21.0 нұсқасына дейінгі Elvish нұсқалары үшін конфигурация файлының орны `~/.elvish/rc.elv` болуы мүмкін.

   #### Tcsh

   `~/.tcshrc` файлының соңына келесі жолды қосыңыз:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```

   #### Nushell
   > [!WARNING]
   > Бұл болашақта өзгереді.
   > Тек Nushell v0.96+ нұсқаларына қолдау көрсетіледі.

   Nushell баптауларының соңына келесіні қосыңыз (Nushell ішінде `$nu.config-path` пәрменін орындау арқылы файл жолын таба аласыз):

   ```sh
   mkdir ($nu.data-dir | path join "vendor/autoload")
   starship init nu | save -f ($nu.data-dir | path join "vendor/autoload/starship.nu")
   ```

   #### Xonsh

   `~/.xonshrc` файлының соңына келесі жолды қосыңыз:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```

   #### Cmd

   Cmd бағдарламасымен бірге [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) қолдануыңыз қажет. `starship.lua` атты файлға келесі кодты жазып, бұл файлды Clink скрипттер каталогына орналастырыңыз:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
