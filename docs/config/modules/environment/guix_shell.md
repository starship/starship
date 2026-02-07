# Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment.
The module will be shown when inside a guix-shell environment.

### Options

| Option     | Default                    | Description                                            |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `'via [$symbol]($style) '` | The format for the module.                             |
| `symbol`   | `'🐃 '`                    | A format string representing the symbol of guix-shell. |
| `style`    | `'yellow bold'`            | The style for the module.                              |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```
