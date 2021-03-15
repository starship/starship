<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship – Prompt Shell multi-platformes"
 />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/workflow/status/starship/starship/Main workflow/master?label=workflow&style=flat-square"
      alt="Statut du workflow actions GitHub"
 /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Version Crates.io"
 /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Statut du package" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Discuter sur Discord"
 /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Suivez @StarshipPrompt sur Twitter"
 /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Site web</a>
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
      alt="Русский"
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
      alt="Espagnol"
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
  alt="Starship avec iTerm2 et le thème Snazzy"
  width="50%"
  align="right"
 />

**L'invite minimaliste, ultra-rapide et personnalisable à l'infini pour n'importe quel shell !**

- **Rapide** : il est rapide - _vraiment vraiment_ rapide ! 🚀
- **Personnalisable:** configurer chaque élément de votre invite.
- **Universel:** fonctionne avec n'importe quel shell, sur n'importe quel système d'exploitation.
- **Intelligent:** affiche les informations utiles en un coup d'œil.
- **Riche en fonctionnalités:** supporte tous vos outils favoris.
- **Facile:** rapide à installer - commencer à l'utiliser en quelques minutes.

<p align="center">
<a href="https://starship.rs/config/"><strong>Consulter la documentation de Starship&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀 Installation

### Pré-requis

- Une police [Nerd Font](https://www.nerdfonts.com/) installée et activée dans votre terminal (Par exemple, essayez [Fira Code](https://www.nerdfonts.com/font-downloads)).

### Démarrage rapide

**Note**: due to the proliferation of different platforms, only a subset of supported platforms are shown below. Can't see yours? Have a look at the [extra platform instructions](https://starship.rs/installing/).

1. Installer l'exécutable **starship** :


   #### Installer la dernière version


   ##### Depuis une version pré-compilée, depuis le shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   #### Installer via le gestionnaire de paquets


   ##### Example: [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### Avec [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

2. Ajouter le script d’initialisation au fichier de configuration de votre shell:


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

   Ajouter ce qui suit à la fin de `Microsoft.PowerShell_profile.ps1`. Vous pouvez vérifier l'emplacement de ce fichier en regardant la variable `$PROFILE` dans PowerShell. Typically the path is `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` or `~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix.

   ```powershell
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Ajouter ce qui suit à la fin de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   **Warning** Only elvish v0.15 or higher is supported. Add the following to the end of `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Add the following to the end of `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```

## 🤝Contribution

Nous sommes toujours à la recherche de contributeurs de **tous niveaux de compétence**! Si vous cherchez à faciliter votre entrée dans le projet, essayez un [good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue).

If you are fluent in a non-English language, we greatly appreciate any help keeping our docs translated and up-to-date in other languages. If you would like to help, translations can be contributed on the [Starship Crowdin](https://translate.starship.rs/).

Si vous êtes intéressé pour aider et contribuer à Starship, veuillez jeter un coup d'œil à notre [Guide de contribution](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Aussi, n'hésitez pas à vous rendre sur notre [serveur Discord](https://discord.gg/8Jzqu3T) pour dire bonjour. 👋

### Contributeurs

Ce projet existe grâce à toutes les personnes qui y contribuent. [[Contribuer](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)].
<a href="https://github.com/starship/starship/graphs/contributors"><img src="https://opencollective.com/starship/contributors.svg?width=890&button=false" /></a>

### Contributeurs financiers

Devenez un contributeur financier et aidez-nous à soutenir notre communauté. [[Contribuer](https://opencollective.com/starship/contribute)]

#### Particuliers

<a href="https://opencollective.com/starship"><img src="https://opencollective.com/starship/individuals.svg?width=890"></a>

#### Organisations

Soutenez ce projet avec votre organisation. Votre logo apparaîtra ici avec un lien vers votre site Web. [[Contribuer](https://opencollective.com/starship/contribute)]

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

## 💭Inspiré par

Voyez ces travaux précédents qui ont contribué à inspirer la création de vaisseau. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - Un invite de commandes ZSH pour les astronautes.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - Thème multi-shell robbyrussell écrit en JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - Un shell multi-platformes de type powerline personnalisable avec des icônes.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Icône de fusée de Starship">
</p>

## 📝 License

Copyright © 2019-présent, [Contributeurs Starship](https://github.com/starship/starship/graphs/contributors).<br /> Ce projet est sous licence[ISC](https://github.com/starship/starship/blob/master/LICENSE).
