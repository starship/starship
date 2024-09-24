[Return to Presets](./#power10)

# Power10 Preset

This preset is inspired by a [powerlevel10k](https://github.com/romkatv/powerlevel10k) configuration variant.

![Screenshot of Power10 preset](/presets/img/power10.png)

### Prerequisites

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal
  - Tested / recommended: 'MesloLGS NF Regular'
    - Font download: [Meslo Nerd Font patched for Powerlevel10k]([https://github.com/romkatv/powerlevel10k](https://github.com/romkatv/powerlevel10k?tab=readme-ov-file#manual-font-installation))
- Optional / recommended: Shell with right-prompt support
  - Only for ssh to show the ipv4 address on the right side of the prompt line

### Modifications

- Shell icon (Shell Module [shell])
  - If you want to see an icon for the currently used shell (right to the os icon):
  - Search for [shell] in the TOML-file and change 'disabled = true' to 'disabled = false'
- Filling top solid line (Fill Module [fill])
  - If you do not want to see the filling solid line in the top row:
  - Search for [fill] in the TOML-file and set the fill symbol to ' ' (space character !)

### Configuration

```sh
starship preset power10 -o ~/.config/starship.toml
```

[Click to download TOML](/presets/toml/power10.toml){download}

<<< @/public/presets/toml/power10.toml
