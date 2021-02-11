# 🚀 Advanced Installation

To install starship, you need to do two things:

1. Get the **starship** binary onto your computer
1. Tell your shell to use the starship binary as its prompt by modifying its init scripts

For most users, the instructions on [the main page](/guide/#🚀-installation) will work great. However,
for some more specialized platforms, different instructions are needed.

There are so many platforms out there that they didn't fit into the main
README.md file, so here are some installation instructions for other platforms
from the community. Is yours not here? Please do add it here if you figure it
out!

## [Nix](https://nixos.wiki/wiki/Nix)

### Getting the Binary

#### Imperatively

```sh
nix-env -iA nixos.starship
```

#### Declarative, single user, via [home-manager](home-manager)

Enable the `programs.starship` module in your `home.nix` file, and add your settings

```nix
{
  programs.starship = {
    enable = true;
    enableZshIntegration = true;
    settings = {
      add_newline = false;
      format = lib.concatStrings [
        "$line_break"
        "$package"
        "$line_break"
        "$character"
      ];
      scan_timeout = 10;
      character = {
        success_symbol = "➜";
        error_symbol = "➜";
      };
    };
  };
}
```

then run

```sh
home-manager switch
```

#### Declarative, system-wide, with NixOS

Add `pkgs.starship` to `environment.systemPackages` in your `configuration.nix`,
then run

```sh
sudo nixos-rebuild switch
```
