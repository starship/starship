<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – Shell prompt multiplatformes" />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/workflow/status/starship/starship/Main workflow/master?label=workflow&style=flat-square"
      alt="GitHub Actions état du workflow" /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Version Crates.io" /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Statut de l'empaquetage" /></a
><br />
  <a href="https://discord.gg/8Jzqu3T"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Discuter sur Discord" /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Suivez @StarshipPrompt sur Twitter" /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Site web</a>
  ·
  <a href="#-installation">Installation</a>
  ·
  <a href="https://starship.rs/config/">Configuration</a>
</p>

<p align="center">
  <a href="https://github.com/starship/starship/blob/master/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-us.png"
      alt="English" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ja-JP/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-jp.png"
      alt="日本語" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-TW/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-tw.png"
      alt="繁體中文" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/ru-RU/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-ru.png"
      alt="Русский" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/de-DE/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-de.png"
      alt="Deutsch" /></a>
  &nbsp;
  <a
    href="https://github.com/starship/starship/blob/master/docs/zh-CN/guide/README.md"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-cn.png"
      alt="简体中文" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/es"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-es.png"
      alt="Espagnol" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/fr"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Français" /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship avec iTerm2 et le thème Snazzy"
  width="50%"
  align="right" />


**L'invite minimaliste, ultra-rapide et personnalisable à l'infini pour n'importe quel shell !**


- **Rapide** : il est rapide - _vraiment vraiment_ rapide ! 🚀
- **Personnalisable:** configurer chaque élément de votre invite.
- **Universel:** fonctionne avec n'importe quel shell, sur n'importe quel système d'exploitation.
- **Intelligent:** affiche les informations utiles en un coup d'oeil.
- **Riche en fonctionnalités:** supporte tous vos outils favoris.
- **Facile:** rapide à installer - commencer à l'utiliser en quelques minutes.

<p align="center">
<a href="https://starship.rs/"><strong>Consulter la documentation de Starship&nbsp;&nbsp;▶</strong></a>
</p>

## 🚀 Installation

### Pré-requis

- Une [police d'écriture Powerline](https://github.com/powerline/fonts) installée et activée dans votre terminal (Par exemple, essayez [Fira Code](https://github.com/tonsky/FiraCode)).

### Démarrage rapide

1. Installer le binaire **starship** :


   #### Installer la dernière version


   ##### Depuis une version pré-compilée depuis le shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   ##### Depuis la source sur [crates.io](https://crates.io/):

   ```sh
   cargo install starship
   ```


   #### Installer via le gestionnaire de paquets


   ##### Avec [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### Avec [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Ajouter le script d'initialization à la fiche config de votre shell:


   #### Bash

   Ajouter ce qui suit à la fin de `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Ajoute ce qui suit à la fin de `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Ajouter ce qui suit à la fin de `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   Ajouter ce qui suit à la fin de `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` (ou `~/.config/powershell/Microsoft.PowerShell_profile.ps1` sur -Nix):

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Ajouter ce qui suit à la fin de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

## 🤝Contribution

Nous sommes toujours à la recherche de contributeurs de **tous les niveaux de compétence**! Si vous cherchez à faciliter votre entrée dans le projet, essayez un [good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue).

Si vous êtes intéressé à aider à contribuer à Starship, veuillez jeter un coup d'oeil à notre [Guide de contribution](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Aussi, n'hésitez pas à vous rendre sur notre [serveur Discord](https://discord.gg/8Jzqu3T) pour dire bonjour. 👋

## 💭Inspiré par

Voyez ces travaux précédents qui ont contribué à inspirer la création de vaisseau. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - Un ZSH prompt pour les astronautes.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - Thème Cross-shell robbyrussell écrit en JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - Un shell multi-platformes de type powerline personnalisable avec des icônes.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Icône de fusée de Starship">
</p>

## 📝 License

Copyright © 2019-présent, [Contributeurs Starship](https://github.com/starship/starship/graphs/contributors).<br /> Ce projet est sous licence[ISC](https://github.com/starship/starship/blob/master/LICENSE).
