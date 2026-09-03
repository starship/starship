[Return to Presets](./#ayu-powerline)

# Ayu Powerline Preset

This preset is a minimally modified version of [Catppuccin Powerline](./catppuccin-powerline.md) using the [Ayu](https://github.com/ayu-theme/ayu-colors) color palette.

![Screenshot of Ayu Powerline preset](/presets/img/ayu-powerline.png)

### Prerequisites

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal

### Configuration

```sh
starship preset ayu-powerline -o ~/.config/starship.toml
```

By default this preset uses the Mirage flavor of Ayu, but you can specify any of the flavors by modifying the value of `palette`:

- `ayu_mirage`
- `ayu_dark`
- `ayu_light`

> [!NOTE]
> If your terminal's own color scheme is [Ghostty](https://ghostty.org)'s built-in `Ayu` theme, that theme is actually the **Dark** flavor of Ayu (its background/foreground match this preset's `ayu_dark` exactly), not Mirage. Ghostty ships `Ayu Mirage` and `Ayu Light` as separately named themes for those flavors. Match `palette` to whichever one your terminal is using for the two to look consistent.

[Click to download TOML](/presets/toml/ayu-powerline.toml)

<<< @/public/presets/toml/ayu-powerline.toml
