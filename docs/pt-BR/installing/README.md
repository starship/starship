# 🚀 Instalação Avançada

Para instalar o Starship, você precisa de duas coisas:

1. O binário do **Starship** em seu computador
1. Altere seu shell para usar o binário do starship como seu prompt alterando os scripts init

For most users, the instructions on [the main page](/guide/#🚀-installation) will work great. However, for some more specialized platforms, different instructions are needed.

There are so many platforms out there that they didn't fit into the main README.md file, so here are some installation instructions for other platforms from the community. Is yours not here? Please do add it here if you figure it out!

## [Chocolatey](https://chocolatey.org)

### Pré-requisitos

Head over to the [Chocolatey installation page](https://chocolatey.org/install) and follow the instructions to install Chocolatey.

### Installation

```powershell
choco install starship
```

## [termux](https://termux.com)

### Pré-requisitos

```sh
pkg install getconf
```

### Installation

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Getting the Binary

#### Imperatively

```sh
nix-env -iA nixos.starship
```

#### Declarative, single user, via [home-manager](https://github.com/nix-community/home-manager)

Enable the `programs.starship` module in your `home.nix` file, and add your settings

```nix
{
  programs.starship = {
    enable = true;
    enableZshIntegration = true;
    # Configuration written to ~/.config/starship.toml
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

#### Declarative, system-wide, with NixOS

Add `pkgs.starship` to `environment.systemPackages` in your `configuration.nix`, then run

```sh
sudo nixos-rebuild switch
```
