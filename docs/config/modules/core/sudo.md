# Sudo

The `sudo` module displays if sudo credentials are currently cached.
The module will only be shown if credentials are cached.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option          | Default                  | Description                                             |
| --------------- | ------------------------ | ------------------------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | The format of the module                                |
| `symbol`        | `'🧙 '`                  | The symbol displayed when credentials are cached        |
| `style`         | `'bold blue'`            | The style for the module.                               |
| `allow_windows` | `false`                  | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                   | Disables the `sudo` module.                             |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[sudo]
style = 'bold green'
symbol = '👩‍💻 '
disabled = false
```

```toml
# On windows
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```
