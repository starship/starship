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
description: O Starship é o prompt minimalista, extremamente rápido e extremamente personalizável para qualquer shell! Mostra as informações que você precisa, mantendo-se elegante e minimalista. Instalação rápida disponível para Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd e PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Pré-requisitos

- Uma fonte [Nerd Font](https://www.nerdfonts.com/) instalada e ativada em seu terminal.

### Instalação

1. Instale o binário do **starship**:


   #### Instalando a última versão

   Com o Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Para atualizar o Starship de maneira manual, execute novamente o script acima. Isto irá substituir a versão atual sem alterar as configurações do Starship.


   #### Instalar via gerenciador de pacotes

   Com o [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```
   Com o [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Adicione o script de inicialização no arquivo de configuração do seu shell:


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

   Adicione o comando a seguir ao final do arquivo `Microsoft.PowerShell_profile.ps1`. Você pode checar a localização deste arquivo consultando a variável `$PROFILE` no PowerShell. Normalmente o caminho é  `~\Documentos\PowerShell\Microsoft.PowerShell_profile.ps1` ou `~/.config/powershell/Microsoft.PowerShell_profile.ps1` no -Nix.

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

   ::: warning

   Apenas elvish v0.18 ou superior é suportado.

   :::

   Adicione o comando a seguir ao final do arquivo `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Adicione ao final do arquivo `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   Isto irá mudar no futuro. Somente Nushell v0.78+ é suportado.

   :::

   Adicione o seguinte ao final do seu arquivo env do Nushell (enconte-o rodando  `$nu.env-path` no Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save -f ~/.cache/starship/init.nu
   ```

   E adicione o seguinte ao final da sua configuração do Nushell (encontre-o executando `$nu.config-path`):

   ```sh
   use ~/.cache/starship/init.nu
   ```


   #### Xonsh

   Adicione o seguinte ao final do arquivo `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   Você precisa do [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) com Cmd. Adicione o seguinte num arquivo `starship.lua` e coloque este arquivo no diretório scripts do Clink:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
