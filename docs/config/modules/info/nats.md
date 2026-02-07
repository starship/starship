# NATS

The `nats` module shows the name of the current [NATS](https://nats.io) context.

### Options

| Option     | Default                    | Description                                                  |
| ---------- | -------------------------- | ------------------------------------------------------------ |
| `symbol`   | `'✉️ '`                     | The symbol used before the NATS context (defaults to empty). |
| `style`    | `'bold purple'`            | The style for the module.                                    |
| `format`   | `'[$symbol$name]($style)'` | The format for the module.                                   |
| `disabled` | `false`                    | Disables the `nats` module.                                  |

### Variables

| Variable | Example     | Description                          |
| -------- | ----------- | ------------------------------------ |
| name     | `localhost` | The name of the NATS context         |
| symbol   |             | Mirrors the value of option `symbol` |
| style\*  |             | Mirrors the value of option `style`  |

### Example

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```
