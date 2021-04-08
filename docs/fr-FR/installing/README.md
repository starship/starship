# 🚀 Installation avancée

Pour installer starship, vous devez faire deux choses:

1. Installez le binaire **starship** sur votre ordinateur
1. Dites à votre shell d'utiliser le binaire de starship comme invite en modifiant ses scripts d'initialisation

Pour la plupart des utilisateurs, les instructions sur [la page principale](/guide/#🚀-installation) fonctionneront bien. Cependant, pour certaines plateformes plus spécialisées, des instructions différentes sont nécessaires.

Il y a tellement de plates-formes, qu'il aurait été déraisonnable de les faire apparaître dans le README principal, voici donc quelques instructions d'installation supplémentaires pour celles-ci, écrient par la commaunauté. La vôtre n'est-elle pas là ? S'il vous plaît, ajoutez-la ici pour les suivants !

## [Chocolatey](https://chocolatey.org)

### Pré-requis

Head over to the [Chocolatey installation page](https://chocolatey.org/install) and follow the instructions to install Chocolatey.

### Installation

```powershell
choco install starship
```

## [termux](https://termux.com)

### Pré-requis

```sh
pkg install getconf
```

### Installation

```sh
curl -fsSL https://starship.rs/install.sh | bash -s -- -b /data/data/com.termux/files/usr/bin
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Obtention du binaire

#### Impératif

```sh
nix-env -iA nixos.starship
```

#### Déclaration, utilisateur unique, via [home-manager](https://github.com/nix-community/home-manager)

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

then run

```sh
home-manager switch
```

#### Déclaration, au niveau du système, avec NixOS

Ajoutez `pkgs.starship` à `environment.systemPackages` dans votre `configuration.nix`, puis exécutez

```sh
sudo nixos-rebuild switch
```
