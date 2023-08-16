# Instalação avançada

Para instalar o starship, você precisa de duas coisas:

1. O binário do **Starship** em seu computador
1. Altere seu shell para usar o binário do starship como seu prompt alterando os scripts init

Para a maioria dos usuários as instruções na [pagina principal](/guide/#🚀-installation) irá funcionar perfeitamente. No entanto para algumas plataformas mais especificas, instruções diferentes são necessárias.

Existem tantas plataformas que elas não cabem no arquivo README.md principal, então aqui estão algumas instruções de instalação para outras plataformas da comunidade. A sua não está aqui? Por favor, adicione-o aqui se você descobrir!

## [Chocolatey](https://chocolatey.org)

### Pré-requisitos

Vá para a [pagina de instalação do Chocolatey](https://chocolatey.org/install) e siga as instruções para instalar o Chocolatey.

### Instalação

```powershell
choco install starship
```

## [termux](https://termux.com)

### Pré-requisitos

```sh
pkg install getconf
```

### Instalação

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Instalação

No Funtoo Linux, o starship pode ser instalado a partir do [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) via Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Obtendo o Binário

#### Imperativamente

```sh
nix-env -iA nixos.starship
```

#### Declarativo, usuário único, via [home-manager](https://github.com/nix-community/home-manager)

Ative o modulo `programs.starship` em seu arquivo `home.nix`, e adicione suas configurações

```nix
{
  programs.starship = {
    enable = true;
    # Configuração gravada em ~/.config/starship.toml
    settings = {
      # add_newline = false;

      # character = {
      #   success_symbol = "[➜](bold green)";
      #   error_symbol = "[➜](bold red)";
      # };

      # package.disabled = true;
    };
  };
}
```

então execute

```sh
home-manager switch
```

#### Declarativo, em todo o sistema com NixOS

Adicione `pkgs.starship` em `environment.systemPackages` no arquivo `configuration.nix`, então execute

```sh
sudo nixos-rebuild switch
```
