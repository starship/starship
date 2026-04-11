# Erweiterte Installation

Um Starship zu installieren, musst du zwei Dinge tun:

1. Laden Sie die **starship** Binärdatei auf Ihren Computer herunter
1. Weisen Sie Ihre Shell an, die Starship-Binärdatei als Eingabeaufforderung zu verwenden, indem Sie die Init-Skripte entsprechend anpassen

Für die meisten Benutzer funktionieren die Anweisungen auf [der Hauptseite](../guide/#🚀-installation) hervorragend. Für einige speziellere Plattformen sind jedoch andere Anweisungen erforderlich.

Es gibt so viele Plattformen, dass sie nicht alle in die Hauptdatei README.md passten; daher finden Sie hier einige Installationsanweisungen für andere Plattformen aus der Community. Ist Ihre nicht dabei? Bitte fügen Sie es hier hinzu, wenn Sie es herausgefunden haben!

## [Chocolatey](https://chocolatey.org)

### Voraussetzungen

Gehen Sie zu [Chocolatey Installations-Seite](https://chocolatey.org/install) und folgen Sie den Anweisungen, um Chocolatey zu installieren.

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

Unter Funtoo Linux kann starship aus [core-kit](https://github.com/funtoo/core-kit/tree/1.4-release/app-shells/starship) über Portage installiert werden:

```sh
emerge app-shells/starship
```

## [Nix](https://wiki.nixos.org/wiki/Nix)

### Abrufen der Binärdatei

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

Danach ausführen

```sh
home-manager switch
```

#### Deklarativ, systemweit, mit NixOS

Fügen Sie `pkgs.starship` zu `environment.systemPackages` in Ihrer `configuration.nix` hinzu, danach führen Sie aus

```sh
sudo nixos-rebuild switch
```
