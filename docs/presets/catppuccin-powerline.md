[Return to Presets](./#catppuccin-powerline)

# Catppuccin Powerline Preset

This preset is a minimally modified version of [Gruvbox Rainbow](./gruvbox-rainbow.md) using the [Catppuccin](https://github.com/catppuccin/catppuccin) theme palette.

![Screenshot of Catppuccin Powerline preset](/presets/img/catppuccin-powerline.png)

### Prerequisites

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal

### Configuration

```sh
starship preset catppuccin-powerline -o ~/.config/starship.toml
```

By default this preset uses the Mocha flavour of Catppucin, but you can specify any of the flavours by modifying the value of `palette`:

- `catppuccin_mocha`
- `catppuccin_frappe`
- `catppuccin_macchiato`
- `catppuccin_latte`

The [Ayu](https://github.com/ayu-theme/ayu-colors) color palette is also included, in all three of its official flavors:

- `ayu_mirage`
- `ayu_dark`
- `ayu_light`

> [!NOTE]
> [Ghostty](https://ghostty.org)'s built-in terminal theme named plain `Ayu` is the **Dark** flavor of Ayu, not Mirage. Ghostty ships `Ayu Mirage` and `Ayu Light` as separately named themes. Match `palette` to whichever one your terminal is using.

[Click to download TOML](/presets/toml/catppuccin-powerline.toml)

<<< @/public/presets/toml/catppuccin-powerline.toml
