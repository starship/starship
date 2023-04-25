---
home: true
heroImage: /logo.svg
heroText:
tagline: L'invite minimaliste, ultra-rapide et personnalisable à l'infini pour n'importe quel shell !
actions:
  - text: Commencez →
    link: ./guide/
    type: primary
features:
  -
    title: Compatibilité avant tout
    details: Fonctionne sur tous les principaux shells et système d'exploitation. Utilisez-le partout !
  -
    title: Propulsé par Rust
    details: Profitez de toute la rapidité et la sécurité de Rust pour rendre votre invite de commandes la plus rapide et fiable possible.
  -
    title: Personnalisable
    details: Tous les petits détails sont personnalisable à votre goût, pour rendre votre invite de commandes aussi légère ou complète que le vous souhaitez.
footer: Licence ISC | Copyright © 2019-présent Contributeurs Starship
#Used for the description meta tag, for SEO
metaTitle: "Starship : Invite Multi-Shell"
description: Starship est une invite minimaliste, ultra-rapide et hautement personnalisable pour n'importe quel shell ! Montre les informations dont vous avez besoin tout en restant élégante et minimaliste. Installation rapide disponible pour Bash, Fish, ZSH, Ion, Tcsh, Elvish, Nu, Xonsh, Cmd, et PowerShell.
---

<div class="center">
  <video class="demo-video" muted autoplay loop playsinline>
    <source src="/demo.webm" type="video/webm">
    <source src="/demo.mp4" type="video/mp4">
  </video>
</div>

### Pré-requis

- Une [Nerd Font](https://www.nerdfonts.com/) est installée et activée dans votre terminal.

### Installation

1. Installer le binaire **starship** :


   #### Installer la dernière version

   Avec Shell:

   ```sh
   curl -sS https://starship.rs/install.sh | sh
   ```

   Pour mettre à jour Starship, relancez le script ci-dessus. Cela remplacera la version actuelle sans toucher à la configuration de Starship.


   #### Installer via le gestionnaire de paquets

   Avec [Homebrew](https://brew.sh/):

   ```sh
   brew install starship
   ```
   With [Winget](https://github.com/microsoft/winget-cli):

   ```powershell
   winget install starship
   ```

1. Ajouter le script d’initialisation au fichier configuration de votre shell:


   #### Bash

   Ajouter ce qui suit à la fin de `~/.bashrc`:

   ```sh
   # ~/.bashrc

   eval "$(starship init bash)"
   ```


   #### Fish

   Ajoute ce qui suit à la fin de `~/.config/fish/config.fish`:

   ```sh
   # ~/.config/fish/config.fish

   starship init fish | source
   ```


   #### Zsh

   Ajouter ce qui suit à la fin de `~/.zshrc`:

   ```sh
   # ~/.zshrc

   eval "$(starship init zsh)"
   ```


   #### Powershell

   Ajouter ce qui suit à la fin de `Microsoft.PowerShell_profile.ps1`. Vous pouvez vérifier l'emplacement de ce fichier en regardant la variable `$PROFILE` dans PowerShell. Habituellement, son chemin est `~\Documents\PowerShell\Microsoft.PowerShell_profile.ps1` ou `~/.config/powershell/Microsoft.PowerShell_profile.ps1` sur -Nix.

   ```sh
   Invoke-Expression (&starship init powershell)
   ```


   #### Ion

   Ajouter ce qui suit à la fin de `~/.config/ion/initrc`:

   ```sh
   # ~/.config/ion/initrc

   eval $(starship init ion)
   ```


   #### Elvish

   ::: warning

   Seul elvish v0.18 ou supérieur est pris en charge.

   :::

   Ajoutez ce qui suit à la fin de `~/.elvish/rc.elv`:

   ```sh
   # ~/.elvish/rc.elv

   eval (starship init elvish)
   ```


   #### Tcsh

   Ajoutez ce qui suit à la fin de `~/.tcshrc`:

   ```sh
   # ~/.tcshrc

   eval `starship init tcsh`
   ```


   #### Nushell

   ::: warning

   Ceci va changer dans le futur. Seul Nushell v0.73+ est supporté.

   :::

   Add the following to the end of your Nushell env file (find it by running `$nu.env-path` in Nushell):
   ```sh
   mkdir ~/.cache/starship
   starship init nu | save -f ~/.cache/starship/init.nu
   ```

   Ajoutez le code suivant à la fin de votre configuration Nushell (trouvez-la en exécutant `$nu.config path`):

   ```sh
   source ~/.cache/starship/init.nu
   ```

   #### Xonsh

   Ajouter ce qui suit à la fin de `~/.xonshrc`:

   ```sh
   # ~/.xonshrc

   execx($(starship init xonsh))
   ```


   #### Cmd

   Vous devez utiliser [Clink](https://chrisant996.github.io/clink/clink.html) (v1.2.30+) avec Cmd. Ajoutez le code ci-dessous dans un fichier `starship.lua` et placez-le dans le dossier des scripts Clink:

   ```lua
   -- starship.lua

   load(io.popen('starship init cmd'):read("*a"))()
   ```
