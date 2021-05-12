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
      alt="Status do workflow Actions do GitHub"
 /></a>
  <a href="https://crates.io/crates/starship"
    ><img
      src="https://img.shields.io/crates/v/starship?style=flat-square"
      alt="Versão no Crates.io"
 /></a>
  <a href="https://repology.org/project/starship/versions"
    ><img
      src="https://img.shields.io/repology/repositories/starship?label=in%20repositories&style=flat-square"
      alt="Status do pacote" /></a
><br />
  <a href="https://discord.gg/starship"
    ><img
      src="https://img.shields.io/discord/567163873606500352?label=discord&logoColor=white&style=flat-square"
      alt="Chat no Discord"
 /></a>
  <a href="https://twitter.com/StarshipPrompt"
    ><img
      src="https://img.shields.io/badge/twitter-@StarshipPrompt-1DA1F3?style=flat-square"
      alt="Siga o @StarshipPrompt no Twitter"
 /></a>
</p>

<p align="center">
  <a href="https://starship.rs">Site</a>
  ·
  <a href="#🚀-installation">Instalação</a>
  ·
  <a href="https://starship.rs/config/">Configuração</a>
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
  alt="Starship com iTerm2 e o tema Snazzy"
  width="50%"
  align="right"
 />

**O prompt minimalista, extremamente rápido e infinitamente personalizável para qualquer shell!**

- **Rápido:** É rápido – _muito muito_ rápido! 🚀
- **Personalizável:** Configure todos os detalhes do seu prompt.
- **Universal:** Funciona em qualquer shell, em qualquer sistema operacional.
- **Inteligente:** Mostra informações relevantes rapidamente.
- **Muitos recursos:** Suporte para todas as suas ferramentas favoritas.
- **Fácil:**Instalação rápida – comece a usar em minutos.

<p align="center">
<a href="https://starship.rs/config/"><strong>Consulte a documentação&nbsp;&nbsp;▶</strong></a>
</p>

<a name="🚀-installation"></a>

## 🚀 Instalação

### Pré-requisitos

- Uma [Nerd Font](https://www.nerdfonts.com/) instalada e funcionando no seu terminal (por exemplo, experimente a [Fira Code Nerd Font](https://www.nerdfonts.com/font-downloads)).

### Primeiros passos

**Note**: due to the proliferation of different platforms, only a subset of supported platforms are shown below. Can't see yours? Have a look at the [extra platform instructions](https://starship.rs/installing/).

1. Instale o binário do **starship**:


   #### Instalar a última versão


   ##### Binário pré-compilado, utilizando o shell:

   ```sh
   sh -c "$(curl -fsSL https://starship.rs/install.sh)"
   ```
   To update the Starship itself, rerun the above script. It will replace the current version without touching Starship's configuration.


   #### Instalar via gerenciador de pacotes


   ##### Exemplo: [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```


   ##### Com o [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

2. Adicione o script de inicialização no arquivo de configuração do seu shell:


   #### Bash

   Adicione o seguinte comando no final do arquivo `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Adicione o seguinte comando no final do arquivo `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Adicione o seguinte comando no final do arquivo `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### PowerShell

   Add the following to the end of `Microsoft.PowerShell_profile.ps1`. You can check the location of this file by querying the `$PROFILE` variable in PowerShell. Typically the path is `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` or `~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix.

   ```powershell
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Adicione o seguinte comando no final do arquivo `~/.config/ion/initrc`:

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

## 🤝 Contribuindo

Nós estamos sempre procurando contribuidores de **todos os níveis de conhecimento**! Se você está buscando um caminho mais fácil para começar no projeto, veja essas [boas issues para começar](https://github.com/starship/starship/labels/🌱%20good%20first%20issue).

If you are fluent in a non-English language, we greatly appreciate any help keeping our docs translated and up-to-date in other languages. If you would like to help, translations can be contributed on the [Starship Crowdin](https://translate.starship.rs/).

Se você está interessado em ajudar contribuindo com o projeto, dê uma olhada no nosso [Guia de Contribuição](https://github.com/starship/starship/blob/master/CONTRIBUTING.md). Além disso, sinta-se à vontade para entrar no nosso [servidor no Discord](https://discord.gg/8Jzqu3T) e dizer oi. 👋

### Contribuidores de código

Este projeto existe graças a todas as pessoas que contribuem. [[Contribuir](https://github.com/starship/starship/blob/master/CONTRIBUTING.md)].
<a href="https://github.com/starship/starship/graphs/contributors"><img src="https://opencollective.com/starship/contributors.svg?width=890&button=false" /></a>

### Contribuidores Financeiros

Torne-se um contribuidor financeiro e nos ajude a sustentar a nossa comunidade. [[Contribuir](https://opencollective.com/starship/contribute)]

#### Pessoas

<a href="https://opencollective.com/starship"><img src="https://opencollective.com/starship/individuals.svg?width=890"></a>

#### Organizações

Apoie este projeto com sua organização. Seu logotipo aparecerá aqui com um link para o seu site. [[Contribuir](https://opencollective.com/starship/contribute)]

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

## 💭 Inspirado por

Por favor, confira estes projetos anteriores que ajudaram a inspirar a criação do startship. 🙏

- **[denysdovhan/spaceship-prompt](https://github.com/denysdovhan/spaceship-prompt)** - Um prompt ZSH para astronautas.

- **[denysdovhan/robbyrussell-node](https://github.com/denysdovhan/robbyrussell-node)** - Tema robbyrussell multi-shell escrito em JavaScript.

- **[reujab/silver](https://github.com/reujab/silver)** - Um prompt multi-shell personalizável com ícones como o powerline.

<p align="center">
    <br>
    <img width="100" src="https://raw.githubusercontent.com/starship/starship/master/media/icon.png" alt="Ícone de foguete do Starship">
</p>

## 📝 Licença

Todos os direitos reservados © 2019-Presente, [Contribuidores Starship](https://github.com/starship/starship/graphs/contributors).<br /> Este projeto está licenciado pelo [ISC](https://github.com/starship/starship/blob/master/LICENSE).
