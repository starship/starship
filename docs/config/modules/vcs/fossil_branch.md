# Fossil Branch

The `fossil_branch` module shows the name of the active branch of the check-out in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                          | Description                                                                              |
| ------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | The format for the module. Use `'$branch'` to refer to the current branch name.          |
| `symbol`            | `' '`                           | The symbol used before the branch name of the check-out in your current directory.       |
| `style`             | `'bold purple'`                  | The style for the module.                                                                |
| `truncation_length` | `2^63 - 1`                       | Truncates a Fossil branch name to `N` graphemes                                          |
| `truncation_symbol` | `'…'`                            | The symbol used to indicate a branch name was truncated. You can use `''` for no symbol. |
| `disabled`          | `true`                           | Disables the `fossil_branch` module.                                                     |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| branch   | `trunk` | The active Fossil branch             |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```
