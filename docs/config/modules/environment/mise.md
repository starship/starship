# Mise

The `mise` module shows the current mise health as reported by running `mise doctor`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                                                              | Description                                      |
| ------------------- | -------------------------------------------------------------------- | ------------------------------------------------ |
| `symbol`            | `'mise '`                                                            | The symbol used before displaying _mise_ health. |
| `style`             | `'bold purple'`                                                      | The style for the module.                        |
| `format`            | `'on [$symbol$health]($style) '`                                     | The format for the module.                       |
| `detect_extensions` | `[]`                                                                 | Which extensions should trigger this module.     |
| `detect_files`      | `['mise.toml', 'mise.local.toml', '.mise.toml', '.mise.local.toml']` | Which filenames should trigger this module.      |
| `detect_folders`    | `['.mise']`                                                          | Which folders should trigger this module.        |
| `healthy_symbol`    | `healthy`                                                            | The message displayed when _mise_ is healthy.    |
| `unhealthy_symbol`  | `unhealthy`                                                          | The message displayed when _mise_ is unhealthy.  |
| `disabled`          | `true`                                                               | Disables the `mise` module.                      |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| health   | `healthy` | The health of _mise_                 |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[mise]
health = 'ready'
```
