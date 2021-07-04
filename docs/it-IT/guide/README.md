<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship: Cross-Shell Prompt"
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
      alt="Versione Crates.io"
 /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Stato del pacchetto" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Chat su Discord"
 /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Segui @StarshipPrompt su Twitter"
 /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Sito web</a>
  ·
  <a href="#🚀-installation">Installazione</a>
  ·
  <a href="https://starship.rs/config/">Configurazione</a>
</p>

<p align="center">
  <a href="https://github.com/starship/starship/blob/master/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-us.png"
      alt="Inglese"
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
    href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png"
      alt="繁體中文"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ru-RU/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ru.png"
      alt="Russo"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/de-DE/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png"
      alt="Tedesco"
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
    href="https://github.com/starship/starship/blob/master/docs/es-ES/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Spagnolo"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/fr-FR/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Francese"
 /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/vi-VN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-vn.png"
      alt="Tiếng Việt"
 /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship con iTerm2 e il tema Snazzy"
  width="50%"
  align="right"
 />

**Il prompt minimalista, super veloce e infinitamente personalizzabile per qualsiasi shell!**

- **Velocità:** è veloce – _davvero_ veloce! 🚀
- **Personalizzabile:** configura ogni aspetto del tuo prompt.
- **Universale:** funziona su qualsiasi shell, con qualsiasi sistema operativo.
- **Intelligente:** mostra le informazioni rilevanti a colpo d'occhio.
- **Ricco di funzionalità:** supporta tutti i tuoi strumenti preferiti.
- **Facile:** veloce da installare - inizia ad usarlo in pochi minuti.

<p align="center">
<a href="https://starship.rs/config/"><strong>Esplora la documentazione di Starship&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀 Installazione

### Prerequisiti

- Un [Nerd Font](https://www.nerdfonts.com/) installato e abilitato nel tuo terminale (per esempio, prova [Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)).

### Inizia

**Nota**: a causa della proliferazione di piattaforme diverse, un sottoinsieme di piattaforme supportate sono mostrate di seguito. Non riesci a vedere la tua? Dai un'occhiata alle [istruzioni aggiuntive della piattaforma](https://starship.rs/installing/).

1. Installa il binario **starship**:


   #### Installa l'ultima Versione


   ##### Da un binario precompilato, con Shell:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   Per aggiornare Starship stesso, riavviare lo script sopra. Sostituirà la versione corrente senza toccare la configurazione di Starship.


   **Nota** - I valori predefiniti dello script di installazione possono essere sovrascritti, vedi la guida integrata.

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --help
   ```


   #### Installa via Package Manager


   ##### Esempio: [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### Con [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

2. Aggiungi lo script di inizializzazione al file di configurazione della shell:


   #### Bash

   Aggiungi quanto segue alla fine di `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Aggiungi quanto segue alla fine di `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Aggiungi quanto segue alla fine di `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   Aggiungi quanto segue alla fine di `Microsoft.PowerShell_profile.ps1`. Puoi controllare la posizione di questo file interrogando la variabile `$PROFILE` in PowerShell. Tipicamente il percorso è `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` oppure `~/.config/powershell/Microsoft.PowerShell_profile.ps1` su -Nix.

   ```powershell
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Aggiungi quanto segue alla fine di `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   **Attenzione** È supportato solo elvish v0.15 o superiore. Aggiungi quanto segue alla fine di `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Aggiungi quanto segue alla fine di `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   **Warning** This will change in the future. Only nu version v0.33 or higher is supported. Add the following to your nu config file:

   ```toml
   startup = [
    "mkdir ~/.cache/starship",
    "starship init nu | save ~/.cache/starship/init.nu",
    "source ~/.cache/starship/init.nu"
   ]
   prompt = "starship_prompt"
   ```

## 🤝 Contribuire

Siamo sempre alla ricerca di collaboratori di **tutti i livelli**! Se stai cercando di entrare facilmente nel progetto, prova un [buon primo problema](https://github.com/starship/starship/labels/🌱%20good%20first%20issue).

Se parli correntemente una lingua diversa dall'inglese, apprezziamo molto qualsiasi aiuto per mantenere i nostri documenti tradotti e aggiornati in altre lingue. Se desideri collaborare, le traduzioni possono essere fornite su [Starship Crowdin](https://translate.starship.rs/).

Se sei interessato ad aiutare a contribuire a Starship, dai un'occhiata alla nostra [Guida al Contributo](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Inoltre, sentiti libero di entrare nel nostro [server Discord](https://discord.gg/8Jzqu3T) e dire ciao. 👋

### Contributori di codice

Questi progetto esiste grazie a tutte le persone che contribuiscono. [[Contributo](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)].
<a href="https://github.com/starship/starship/graphs/contributors"><img src="https://opencollective.com/starship/contributors.svg?width=890&button=false" /></a>

### Contributori Finanziari

Diventa un contributore finanziario e aiutaci a sostenere la nostra comunità. [[Contribuisci](https://opencollective.com/starship/contribute)]

#### Privati

<a href="https://opencollective.com/starship"><img src="https://opencollective.com/starship/individuals.svg?width=890"></a>

#### Organizzazioni

Supporta questo progetto con la tua organizzazione. Il tuo logo apparirà qui con un link al tuo sito web. [[Contribuisci](https://opencollective.com/starship/contribute)]

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

## 💭 Ispirato Da

Ti invito di controllare questi lavori precedenti che hanno contribuito a ispirare la creazione di Starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - Un prompt ZSH per astronauti.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - Tema robbyrussell Cross-shell scritto in JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - Un prompt multi-shell personalizzabile powerline-like con icone.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 Licenza

Copyright © 2019-presente, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> Questo progetto è sotto licenza [ISC](https://github.com/starship/starship/blob/master/LICENSE).
