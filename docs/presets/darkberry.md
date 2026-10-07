[Return to Presets](./#darkberry)

# Darkberry Preset

This preset is a two-line box-drawing prompt in the [Darkberry](https://github.com/shythulu/DarkBerry) palette: wine-dark surfaces with berry accents. Row one is where you are, row two is the caret with the last command's duration, exit status and jobs on the right margin.

![Screenshot of Darkberry preset](/presets/img/darkberry.png)

### Prerequisites

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal (the frame itself is plain Unicode; only the module icons need one)

### Configuration

```sh
starship preset darkberry -o ~/.config/starship.toml
```

By default this preset uses the Mire flavour of Darkberry, but you can specify any of the flavours by modifying the value of `palette`:

- `darkberry_wisp` (light)
- `darkberry_fen` (soft dark)
- `darkberry_mire` (dark)
- `darkberry_blackwater` (darkest)

[Click to download TOML](/presets/toml/darkberry.toml){download}

<<< @/public/presets/toml/darkberry.toml
