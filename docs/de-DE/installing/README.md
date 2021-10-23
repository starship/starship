# 🚀 Erweiterte Installation

Um Starship zu installieren, musst du zwei Dinge tun:

1. Holen Sie sich das **starship** Binary auf Ihren Computer
1. Sagen Sie ihrer Shell, dass sie das Starship-Binary als Prompt benutzt, indem sie die Initialisierungs-Skripte ändern

Die Anweisungen auf [der Hauptseite](/guide/#🚀-installation) werden für die meisten Benutzer gut funktionieren. Für einige speziellere Plattformen werden jedoch andere Anweisungen benötigt.

Es gibt so viele Plattformen, dass sie nicht in das Haupt-README passen, also hier einige Installationsanweisungen für andere Plattformen von der Community. Ist Ihre nicht hier? Bitte fügen Sie es hier hinzu, wenn Sie es herausfinden!

## [Chocolatey](https://chocolatey.org)

### Voraussetzungen

Gehen Sie zur [Chocolatey's Installations-Seite](https://chocolatey.org/install) und folge den Anweisungen um Chocolatey zu installieren.

### Installation

```powershell
choco install starship
```

## [termux](https://termux.com)

### Voraussetzungen

```sh
pkg install getconf
```

### Installation

```sh
sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- --bin-dir /data/data/com.termux/files/usr/bin
```

## [Nix](https://nixos.wiki/wiki/Nix)

### Das Binary holen

#### Imperativ

```sh
nix-env -iA nixos.starship
```

#### Deklarativ, Einzel-Benutzer, über [home-manager](https://github.com/nix-community/home-manager)

Aktivieren Sie das Modul `programs.starship` in Ihrer `home.nix`-Datei und fügen Sie Ihre Einstellungen hinzu

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

und führen Sie dann folgendes aus

```sh
home-manager switch
```

#### Deklarativ, systemweit, mit NixOS

Fügen Sie `pkgs.starship` zu `environment.systemPackages` in Ihrer `configuration.nix` hinzu, und führen Sie folgendes aus

```sh
sudo nixos-rebuild switch
```
