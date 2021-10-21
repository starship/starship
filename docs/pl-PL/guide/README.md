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
      alt="Status Github Actions"
 /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Wersja na crates.io"
 /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Status pakietów" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Porozmawiaj na Discord"
 /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Śledź @StarshipPrompt na Twitterze"
 /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Strona internetowa</a>
  ·
  <a href="#🚀-installation">Instalacja</a>
  ·
  <a href="https://starship.rs/config/">Konfiguracja</a>
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
  alt="Starship na iTerm2 z motywem Snazzy"
  width="50%"
  align="right"
 />

**Minimalny, szybki i nieskończenie konfigurowalny wiersz poleceń dla dowolnej powłoki!**

- **Szybkość:** jest szybki - _naprawdę_ szybki! 🚀
- **Konfigurowalny:** dostosowuj swój wiersz poleceń w każdym aspekcie.
- **Uniwersalny:** działa na każdej powłoce, na każdym systemie operacyjnym.
- **Inteligentny:** pokazuje istotne informacje na pierwszy rzut oka.
- **Bogata funkcjonalność:** wspiera wszystkie twoje ulubione narzędzia.
- **Łatwy w obsłudze:** szybka instalacja - zacznij korzystać w kilka minut.

<p align="center">
<a href="https://starship.rs/config/"><strong>Przeglądaj dokumentację Starship&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀Instalacja

### Wymagania wstępne

- Czcionka typu [Nerd Font](https://www.nerdfonts.com/) zainstalowana i ustawiona w twoim terminalu (wypróbuj na przykład [Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)).

### Pierwsze kroki

**Uwaga**: z powodu szybko rosnącej liczby platform poniżej pokazano wybrane platformy spośród obecnie obsługiwanych. Nie widzisz swojej? Sprawdź w [dodatkowych instrukcjach dla platform](https://starship.rs/installing/).

1. Zainstaluj plik programu **starship**:


   #### Instalacja najnowszej wersji


   ##### Z wstępnie zbudowanego pliku wykonywalnego, za pomocą powłoki Shell:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```

   Aby zaktualizować Starship, uruchom ponownie powyższy skrypt. Obecna wersja zostanie zastąpiona nową, bez modyfikowania konfiguracji Starship.

   **Uwaga** - Domyślne ustawienia skryptu instalacyjnego można zmienić - szczegóły znajdziesz we wbudowanym poleceniu pomocy.

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

## 💭Inspiracje

Please check out these previous works that helped inspire the creation of starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - Wiersz poleceń ZSH dla astronautów.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - Motyw robyrussell dla wielu powłok, napisany w JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - Wiersz polecenia typu Powerline z ikonami, działa na wielu różnych powłokach.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 Licencja

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> This project is [ISC](https://github.com/starship/starship/blob/master/LICENSE) licensed.
