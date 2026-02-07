# Fossil Metrics

The `fossil_metrics` module will show the number of added and deleted lines in the check-out in your current directory. At least v2.14 (2021-01-20) of Fossil is required.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option               | Default                                                      | Description                           |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | The format for the module.            |
| `added_style`        | `'bold green'`                                               | The style for the added count.        |
| `deleted_style`      | `'bold red'`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `disabled`           | `true`                                                       | Disables the `fossil_metrics` module. |

### Variables

| Variable        | Example | Description                                 |
| --------------- | ------- | ------------------------------------------- |
| added           | `1`     | The current number of added lines           |
| deleted         | `2`     | The current number of deleted lines         |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[fossil_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```
