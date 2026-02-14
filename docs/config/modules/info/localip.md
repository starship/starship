# Local IP

The `localip` module shows the IPv4 address of the primary network interface.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option     | Default                   | Description                                            |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `'[$localipv4]($style) '` | The format for the module.                             |
| `style`    | `'bold yellow'`           | The style for the module.                              |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### Variables

| Variable  | Example      | Description                         |
| --------- | ------------ | ----------------------------------- |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address   |
| style\*   |              | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```
