# Geavanceerde Installatie

Om Starship te installeren moet je twee dingen doen:

1. Sla het **Starship** programma op je computer op
1. Stel je shell in om het Starship programma te gebruiken bij de invoer door de opstartscripts aan te passen

Voor de meeste gebruikers werken de instructies op de [hoofdpagina](../guide/#🚀-installation) prima, maar voor specifieke platformen zijn er andere instructies.

Er bestaan dermate veel platformen dat ze niet meer in het hoofdbestand voor de README, dus zijn andere instructies door de gemeenschap hier samengebracht. Staat jouw platform er niet tussen? Voeg het hier toe als je het werkend krijgt!

## [Chocolatey](https://chocolatey.org)

### Benodigdheden

Ga naar de [installatie pagina van Chocolatey](https://chocolatey.org/install) en volg de instructies op om Chocolatey te installeren.

### Installatie

```powershell
choco install starship
```

## [termux](https://termux.com)

### Benodigdheden

```sh
pkg install getconf
```

### Installatie

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Installatie

Op Funtoo Linux kan Starship worden geïnstalleerd van [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) via Portage:

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

### Het programma bemachtigen

#### Imperatief

```sh
nix-env -iA nixos.starship
```

#### Declaratief voor een enkele gebruiker, via [home-manager](https://github.com/nix-community/home-manager)

Activeer de `programs.starship` module in het `home.nix` bestand en voeg je instellingen in

```nix
{
  programs.starship = {
    enable = true;
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

en dan

```sh
home-manager switch
```

#### Declaratief voor het hele systeem, met NixOS

Voeg `pkgs.starship` toe aan `environment.systemPackages` in `configuration.nix` gevolgd door het aanroepen van

```sh
sudo nixos-rebuild switch
```
