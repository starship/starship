# Network Namespace

The `netns` module shows the current network namespace.
This uses `ip netns identify` to get the network namespace, so only network namespaces mounted at `/var/run/netns` will be detected.

### Options

| Option     | Default                         | Description                                                       |
| ---------- | ------------------------------- | ----------------------------------------------------------------- |
| `format`   | `'[$symbol \[$name\]]($style)'` | The format for the module.                                        |
| `symbol`   | `'🛜 '`                         | The symbol used before the network namespace (defaults to empty). |
| `style`    | `'blue bold dimmed'`            | The style for the module.                                         |
| `disabled` | `false`                         | Disables the `netns` module.                                      |

### Variables

| Variable | Example    | Description                               |
| -------- | ---------- | ----------------------------------------- |
| name     | `my-netns` | The name of the current network namespace |
| symbol   |            | Mirrors the value of option `symbol`      |
| style\*  |            | Mirrors the value of option `style`       |

### Example

```toml
# ~/.config/starship.toml

[netns]
style = 'bold yellow'
symbol = '🌐 '
```
