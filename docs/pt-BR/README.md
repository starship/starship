---
home: true
heroImage: /logo.svg
heroText:
tagline: O prompt minimalista, extremamente rápido e infinitamente personalizável para qualquer shell!
actionText: Primeiros passos →
actionLink: ./guide/
features:
  - 
    title: Compatibilidade primeiro
    details: Funciona nos principais shells nos principais sistemas operacionais. Use em qualquer lugar!
  - 
    title: Poder do Rust
    details: Tenha o melhor da velocidade e segurança do Rust, para tornar seu prompt o mais rápido e confiável possível.
  - 
    title: Personalizável
    details: Cada pequeno detalhe é personalizável ao seu gosto, para tornar esse prompt o mínimo possível ou rico em recursos, como você preferir.
footer: Licenciado pelo ISC | Todos os direitos reservados © 2019-Presente | Contribuidores Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship: Cross-Shell Prompt"
description: O Starship é o prompt minimalista, extremamente rápido e extremamente personalizável para qualquer shell! Mostra as informações que você precisa, mantendo-se elegante e minimalista. Instalação rápida disponível para Bash, Fish, ZSH, Ion e Powershell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Instalação

1. Instale o binário do **starship**:


   #### Instalar a última versão

   Com o Shell:

   ```sh
   curl -fsSL https://starship.rs/install.sh | bash
   ```
   To update the Starship itself, rerun the above script. It will replace the current version without touching Starship's configuration.


   #### Instalar via Gerenciador de Pacotes

   Com o [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```

   Com o [Scoop](https://scoop.sh):

   ```powershell
   scoop install starship
   ```

1. Adicione o script de inicialização ao arquivo de configuração do shell:


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


   #### Powershell

   Add the following to the end of `Microsoft.PowerShell_profile.ps1`. You can check the location of this file by querying the `$PROFILE` variable in PowerShell. Typically the path is `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` or `~/.config/powershell/Microsoft.PowerShell_profile.ps1` on -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Adicione o seguinte comando no final do arquivo `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```

   #### Elvish

   ::: warning Only elvish v0.15 or higher is supported. :::

   Add the following to the end of `~/.elvish/rc.elv`:

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
