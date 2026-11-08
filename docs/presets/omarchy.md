[Return to Presets](./#omarchy)

# Omarchy Preset

This preset replicates the default minimal Starship prompt shipped with [Omarchy](https://omarchy.org/): only directory, git branch, and git status in cyan, with a `❯` prompt character and `✗` on error. It intentionally leaves out username and time, since Omarchy shows the time in its top bar.

### Configuration

```sh
starship preset omarchy -o ~/.config/starship.toml
```

[Click to download TOML](/presets/toml/omarchy.toml){download}

<<< @/public/presets/toml/omarchy.toml
