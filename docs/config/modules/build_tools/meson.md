# Meson

The `meson` module shows the current Meson developer environment status.

By default the Meson project name is displayed, if `$MESON_DEVENV` is set.

### Options

| Option              | Default                            | Description                                                                               |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | The format for the module.                                                                |
| `symbol`            | `'⬢ '`                             | The symbol used before displaying the project name.                                       |
| `style`             | `'blue bold'`                      | The style for the module.                                                                 |
| `disabled`          | `false`                            | Disables the `meson` module.                                                              |

### Variables

| Variable | Example    | Description                          |
| -------- | ---------- | ------------------------------------ |
| project  | `starship` | The current Meson project name       |
| symbol   | `🐏`       | Mirrors the value of option `symbol` |
| style\*  |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = '--'
symbol = ' '
style = 'bold dimmed green'
```
