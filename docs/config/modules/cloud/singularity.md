# Singularity

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container
and `$SINGULARITY_NAME` is set.

### Options

| Option     | Default                        | Description                                      |
| ---------- | ------------------------------ | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | The format for the module.                       |
| `symbol`   | `''`                           | A format string displayed before the image name. |
| `style`    | `'bold dimmed blue'`           | The style for the module.                        |
| `disabled` | `false`                        | Disables the `singularity` module.               |

### Variables

| Variable | Example      | Description                          |
| -------- | ------------ | ------------------------------------ |
| env      | `centos.img` | The current Singularity image        |
| symbol   |              | Mirrors the value of option `symbol` |
| style\*  |              | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```
