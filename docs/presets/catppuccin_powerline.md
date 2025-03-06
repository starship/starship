[Return to Presets](./README.md#catppuccin_powerline)

# Catppuccin Powerline Preset

This preset is a minimally modified version of [Gruvbox Rainbow](./gruvbox-rainbow.md) using the [Catppuccin](https://github.com/catppuccin/catppuccin) theme palette.

![Screenshot of Catppuccin Powerline preset](/presets/img/catppuccin_powerline.png)

### Prerequisites

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal

### Configuration

```sh
starship preset catppuccin_powerline -o ~/.config/starship.toml
```

By default this preset uses the Mocha flavour of Catppucin, but you can specify any of the flavours by modifying the value of `palette`:

- `catppuccin_mocha`
- `catppuccin_frappe`
- `catppuccin_macchiato`
- `catppucin_latte`

[Click to download TOML](/presets/toml/catppuccin_powerline.toml)

<<< @/.vuepress/public/presets/toml/catppuccin.toml
