# Container

The `container` module displays a symbol and container name, if inside a container.

### Options

| Option     | Default                          | Description                               |
| ---------- | -------------------------------- | ----------------------------------------- |
| `symbol`   | `'⬢'`                            | The symbol shown, when inside a container |
| `style`    | `'bold red dimmed'`              | The style for the module.                 |
| `format`   | `'[$symbol \[$name\]]($style) '` | The format for the module.                |
| `disabled` | `false`                          | Disables the `container` module.          |

### Variables

| Variable | Example             | Description                          |
| -------- | ------------------- | ------------------------------------ |
| name     | `fedora-toolbox:35` | The name of the container            |
| symbol   |                     | Mirrors the value of option `symbol` |
| style\*  |                     | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```
