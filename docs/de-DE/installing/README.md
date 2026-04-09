# Erweiterte Installation

Um Starship zu installieren, musst du zwei Dinge tun:

1. Lade die **starship** Datei auf den Computer herunter
1. Weise deine Shell an die Starship Datei als Eingabeaufforderung zu nutzen, indem du eines der Initialisierungs-Skripte benutzt

Für die meisten Benutzer funktionieren die Anweisungen auf [der Hauptseite](../guide/#🚀-installation) hervorragend. Für einige speziellere Plattformen sind jedoch andere Anweisungen erforderlich.

Es gibt sehr viele Plattformen, sodass diese nicht alle in die Hauptanleitung passen, aus diesem Grund sind hier ein paar Installationsanweisungen für ein paar Plattformen von der Community. Ist Ihre nicht dabei? Dann füge bitte deine hinzu, sobald du herausgefunden hast wie man starship mit dieser benutzt!

## [Chocolatey](https://chocolatey.org)

### Voraussetzungen

Gehe zur [Chocolatey's Installations-Seite](https://chocolatey.org/install) und folge den Anweisungen um Chocolatey zu installieren.

### Installation

```powershell
choco install starship
```

## [termux](https://termux.com)

### Installation

```sh
pkg install starship
```

## [Funtoo Linux](https://www.funtoo.org/Welcome)

### Installation

Unter Funtoo Linux kann starship von [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) über Portage installiert werden:

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

### Das Binary holen

#### Imperativ

```sh
nix-env -iA nixos.starship
```

#### Deklarativ, Einzel-Benutzer, über [home-manager](https://github.com/nix-community/home-manager)

Aktiviere das Modul `programs.starship` in deiner `home.nix`-Datei und füge deine Einstellungen hinzu

```nix
{
  programs.starship = {
    enable = true;
    # Konfiguration die nach ~/.config/starship.toml geschrieben wird
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

führe danach folgendes aus

```sh
home-manager switch
```

#### Deklarativ, systemweit, mit NixOS

Füge `pkgs.starship` zu der Sektion `environment.systemPackages` in deiner `configuration.nix` hinzu, und führe folgenden Befehl aus

```sh
sudo nixos-rebuild switch
```
