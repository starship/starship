[Return to Presets](./#shades)

# Shades Preset

This preset is based on [Catppuccin Powerline](./catppuccin-powerline.md), with monochromatic gradient palettes in eight hues. It also adds a `user@host` segment, and puts the prompt character on its own line. Each palette is a six-step ramp from light to dark in a single hue, and the text color of each segment switches between dark and light to stay readable.

![Screenshot of Shades preset](/presets/img/shades.png)

### Prerequisites

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal

### Configuration

```sh
starship preset shades -o ~/.config/starship.toml
```

By default this preset uses the `gray` palette. To switch palettes, change the value of `palette` or use `starship config`:

```sh
starship config palette red
```

The available palettes are:

- `gray`
- `red`
- `orange`
- `yellow`
- `green`
- `teal`
- `blue`
- `purple`

[Click to download TOML](/presets/toml/shades.toml){download}

<<< @/public/presets/toml/shades.toml
