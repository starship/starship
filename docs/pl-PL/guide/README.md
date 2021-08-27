<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – Cross-shell prompt"
 />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/workflow/status/starship/starship/Main workflow/master?label=workflow&style=flat-square"
      alt="GitHub Actions workflow status"
 /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Crates.io version"
 /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Packaging status" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Chat on Discord"
 /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Follow @StarshipPrompt on Twitter"
 /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Website</a>
  ·
  <a href="#🚀-installation">Installation</a>
  ·
  <a href="https://starship.rs/config/">Configuration</a>
</p>

<p align="center">
  <a href="https://github.com/starship/starship/blob/master/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-us.png"
      alt="English"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/de-DE/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png"
      alt="Deutsch"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/es-ES/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Español"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/fr-FR/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Français"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ja-JP/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-jp.png"
      alt="日本語"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/pt-BR/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-br.png"
      alt="Português do Brasil"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ru-RU/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ru.png"
      alt="Русский"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/vi-VN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-vn.png"
      alt="Tiếng Việt"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-CN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-cn.png"
      alt="简体中文"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png"
      alt="繁體中文"
 /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship with iTerm2 and the Snazzy theme"
  width="50%"
  align="right"
 />

**The minimal, blazing-fast, and infinitely customizable prompt for any shell!**

- **Fast:** it's fast – _really really_ fast! 🚀
- **Customizable:** configure every aspect of your prompt.
- **Universal:** works on any shell, on any operating system.
- **Intelligent:** shows relevant information at a glance.
- **Bogata funkcjonalność:** wspiera wszystkie twoje ulubione narzędzia.
- **Łatwość:** szybka instalacja - zacznij korzystać w kilka minut.

<p align="center">
<a href="https://starship.rs/config/"><strong>Przeglądaj dokumentację Starship&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀Instalacja

### Wymagania wstępne

- Czcionka typu [Nerd Font](https://www.nerdfonts.com/) zainstalowana i ustawiona w twoim terminalu (wypróbuj na przykład [Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)).

### Pierwsze kroki

**Note**: due to the proliferation of different platforms, only a subset of supported platforms are shown below. Can't see yours? Have a look at the [extra platform instructions](https://starship.rs/installing/).

1. Zainstaluj plik programu **starship**:


   #### Instalacja najnowszej wersji


   ##### Z wstępnie zbudowanego pliku wykonywalnego, za pomocą powłoki Shell:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```

   Aby zaktualizować Starship, uruchom ponownie powyższy skrypt. Obecna wersja zostanie zastąpiona nową, bez modyfikowania konfiguracji Starship.

   **Note** - The defaults of the install script can be overridden see the built-in help.

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --help
   ```


   #### Instalacja za pomocą menedżera pakietów


   ##### Przykład: [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### Za pomocą [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

2. Dodaj skrypt inicjalizacyjny do konfiguracji twojej powłoki:


   #### Bash

   Dodaj na koniec pliku `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Dodaj na koniec pliku `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Dodaj na koniec pliku `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   Dodaj na koniec pliku `Microsoft.PowerShell_profile.ps1`. Możesz sprawdzić lokalizację tego pliku odczytując zmienną środowiskową `$PROFILE` w PowerShell. Zazwyczaj jest to `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` lub `~/.config/powershell/Microsoft.PowerShell_profile.ps1` na -Nixie.

   ```powershell
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Dodaj na koniec pliku `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   **Uwaga** Działa tylko dla wersji powłoki elvish v0.15 lub wyższej. Dodaj na koniec pliku `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Dodaj na koniec pliku `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Xonsh

   Dodaj na koniec pliku `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Nushell

   **Uwaga** Może ulec zmianie w przyszłości. Działa tylko dla wersji powłoki nu v0.33 lub wyższej. Dodaj do pliku konfiguracyjnego nu. Możesz sprawdzić lokalizację tego pliku wywołując polecenie `config path` w powłoce nu.

   ```toml
   startup = [
    "mkdir ~/.cache/starship",
    "starship init nu | save ~/.cache/starship/init.nu",
    "source ~/.cache/starship/init.nu"
   ]
   prompt = "starship_prompt"
   ```

## 🤝Wspomóż nas

Zawsze szukamy pomocy od osób **na każdym poziomie zaawansowania**! Jeśli potrzebujesz łatwiejszego wdrożenia w projekt, wypróbuj [good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue).

Jeśli płynnie władasz językiem innym niż angielski, bardzo doceniamy każdą pomoc w tłumaczeniu dokumentacji. Jeśli chcesz pomóc, tłumaczenia można dodawać na [Starship Crowdin](https://translate.starship.rs/).

Jeżeli chcesz wspomóc tworzenie starship, zapoznaj się z naszym [Poradnikiem Współpracy](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Ponadto możesz wejść na nasz [serwer Discord](https://discord.gg/8Jzqu3T) i się przywitać. 👋

### Współtwórcy kodu

Ten projekt istnieje dzięki pomocy wszystkich zaangażowanych osób. [[Wspomóż nas](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)].
<a href="https://github.com/starship/starship/graphs/contributors"><img src="https://opencollective.com/starship/contributors.svg?width=890&button=false" /></a>

### Wsparcie finansowe

Udziel wsparcia finansowego i pomóż nam utrzymywać społeczność. [[Wspomóż nas](https://opencollective.com/starship/contribute)]

#### Osoby prywatne

<a href="https://opencollective.com/starship"><img src="https://opencollective.com/starship/individuals.svg?width=890"></a>

#### Organizacje

Wspieraj ten projekt za pomocą swojej organizacji. Twoje logo pojawi się tutaj z linkiem do Twojej witryny. [[Wspomóż nas](https://opencollective.com/starship/contribute)]

<a href="https://opencollective.com/starship/organization/0/website"><img src="https://opencollective.com/starship/organization/0/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/1/website"><img src="https://opencollective.com/starship/organization/1/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/2/website"><img src="https://opencollective.com/starship/organization/2/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/3/website"><img src="https://opencollective.com/starship/organization/3/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/4/website"><img src="https://opencollective.com/starship/organization/4/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/5/website"><img src="https://opencollective.com/starship/organization/5/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/6/website"><img src="https://opencollective.com/starship/organization/6/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/7/website"><img src="https://opencollective.com/starship/organization/7/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/8/website"><img src="https://opencollective.com/starship/organization/8/avatar.svg"></a>
<a href="https://opencollective.com/starship/organization/9/website"><img src="https://opencollective.com/starship/organization/9/avatar.svg"></a>

## 💭Inspiracje

Zapoznaj się z wcześniejszymi projektami które zainspirowały nas do stworzenia starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - wiersz poleceń ZSH dla astronautów.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - Cross-shell robbyrussell theme written in JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - A cross-shell customizable powerline-like prompt with icons.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 Licencja

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> This project is [ISC](https://github.com/starship/starship/blob/master/LICENSE) licensed.
