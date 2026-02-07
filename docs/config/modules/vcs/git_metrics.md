# Git Metrics

The `git_metrics` module will show the number of added and deleted lines in
the current git repository.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option               | Default                                                      | Description                           |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `'bold green'`                                               | The style for the added count.        |
| `deleted_style`      | `'bold red'`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | The format for the module.            |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.    |
| `ignore_submodules`  | `false`                                                      | Ignore changes to submodules          |

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

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```
