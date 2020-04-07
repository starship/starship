<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/starship/starship/master/media/logo.png"
    alt="Starship - Prompt multi interfaz de línea de comandos" />
</p>

<p align="center">
  <a href="https://github.com/starship/starship/actions"
    ><img
      src="https://img.shields.io/github/workflow/status/starship/starship/Main workflow/master?label=workflow&style=flat-square"
      alt="Estado del flujo de trabajo de GitHub Actions" /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Versión de Crates.io" /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Estado de empaquetado" /></a
><br />
  <a href="https://discord.gg/8Jzqu3T"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Chat en Discord" /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Sigue a @StarshipPrompt en Twitter" /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Sitio</a>
  ·
<a href="#-installation">Instalación</a>
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
      alt="Español" /></a>
  &nbsp;
  <a href="https://translate.starship.rs/project/starship-prompt/fr"
    ><img
      height="20"
      src="https://raw.githubusercontent.com/starship/starship/master/media/flag-fr.png"
      alt="Francés" /></a>
</p>

<h1></h1>

<img
  src="https://raw.githubusercontent.com/starship/starship/master/media/demo.gif"
  alt="Starship con iTerm 2 y el tema Snazzy"
  width="50%"
  align="right" />


**The minimal, blazing-fast, and infinitely customizable prompt for any shell!**


- **Fast:** it's fast – _really really_ fast! 🚀
- **Customizable:** configure every aspect of your prompt.
- **Universal:** works on any shell, on any operating system.
- **Intelligent:** shows relevant information at a glance.
- **Feature rich:** support for all your favorite tools.
- **Easy:** quick to install – start using it in minutes.

<p align="center">
<a href="https://starship.rs/"><strong>Explore the Starship docs&nbsp;&nbsp;▶</strong></a>
</p>

## 🚀 Instalacíon

### Prerequisitos

- Una [fuente Powerline](https://github.com/powerline/fonts) instalado y activada en tu terminal (por ejemplo, prueba con [Fira Code](https://github.com/tonsky/FiraCode)).

### Comenzando

1. Instala el binario de **starship**:


   #### Instalar la última versión


   ##### From prebuilt binary, with Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```


   ##### From source on [crates.io](https://crates.io/):

   ```sh
   cargo install starship
   ```


   #### Instalar con un gestor de paquetes


   ##### Con [Homebew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### Con [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Añade el script de inicio al archivo de configuración de tu interfaz de línea de comandos:


   #### Bash

   Añade el siguiente código al final de `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Añade el siguiente código al final de `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Añade el siguiente código al final de `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   Añade el siguiente código al final de `~\Documentos\PowerShell\Microsoft.PowerShell_profile.ps1` (o `~/.config/powershell/Microsoft.PowerShell_profile.ps1` en *nix):

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Añade el siguiente código al final de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

## 🤝 Colaborando

¡Siempre estamos buscando por colaboradores de **cualquier nivel**! Si esta buscando una manera fácil de ayudar este proyecto, puede intentar a resolver una de las propuestas con la etiqueta "[good first issue](https://github.com/starship/starship/labels/🌱%20good%20first%20issue)" (primera buena propuesta).

Si quiere ayudar a colaborar a starship, por favor mira a nuestra [Guía de Colaboradores](https://github.com/starship/starship/blob/master/CONTRIBUTING.md) (Contributing Guide). Además, juntarse con nosotros en nuestro [servidor de Discord](https://discord.gg/8Jzqu3T) y di "¡hola!". 👋

### Code Contributors

This project exists thanks to all the people who contribute. [[Contribute](CONTRIBUTING.md)].
<a href="https://github.com/starship/starship/graphs/contributors"><img src="https://opencollective.com/starship/contributors.svg?width=890&button=false" /></a>

### Financial Contributors

Become a financial contributor and help us sustain our community. [[Contribute](https://opencollective.com/starship/contribute)]

#### Individuals

<a href="https://opencollective.com/starship"><img src="https://opencollective.com/starship/individuals.svg?width=890"></a>

#### Organizations

Support this project with your organization. Your logo will show up here with a link to your website. [[Contribute](https://opencollective.com/starship/contribute)]

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

## 💭 Inspiracíon

Please check out these previous works that helped inspire the creation of starship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - Una prompt Zsh para astronautas.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - robbyrussel, tema multi interfaz de línea de comandos escrito en JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - Una prompt con iconos, personalizable y multi interfaz de línea de comandos basada en PowerLine.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Starship rocket icon">
</p>

## 📝 Licencia

Copyright © 2019-present, [Starship Contributors](https://github.com/starship/starship/graphs/contributors).<br /> This project is [ISC](https://github.com/starship/starship/blob/master/LICENSE) licensed.
