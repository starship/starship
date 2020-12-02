## 🚀 Installation (extended)

To install starship, you need to do two things:

1. Get the **starship** binary onto your computer
1. Tell your shell to use the starship binary as its prompt

For most users, the instructions on [the main page](/) will work great. However,
for some more specialized platforms, different instructions are needed.

There are so many platforms out there that they didn't fit into the main
README.md file, so here are some installation instructions for other platforms
from the community. Is yours not here? Please do add it here if you figure it
out!

1. Installing the **starship** binary:

   ##### From source on [crates.io](https://crates.io/):

   ```sh
   cargo install starship
   ```

   ### With [Nix](https://nixos.wiki/wiki/Nix):

   #### Imperatively

   ```sh
   nix-env -iA nixos.starship
   ```

   #### Declarative, single user, via [home-manager](home-manager)

   Add `pkgs.starship` to your `home.packages` in your `home.nix` file, then run

   ```sh
   home-manager switch
   ```

   #### Declarative, system-wide, with NixOS

   Add `pkgs.starship` to `environment.packages` in your `configuration.nix`,
   then run

   ```sh
   sudo nixos-rebuild switch
   ```

2. Adding the init script to your shell's config file

   ### With Nix and home-manager, using zsh:

   Add the following to `programs.zsh.initExtra` in your `home.nix` file, then
   run

   ```sh
   home-manager switch
   ```
