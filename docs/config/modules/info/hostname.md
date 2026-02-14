# Hostname

The `hostname` module shows the system hostname.

### Options

| Option            | Default                                | Description                                                                                                                           |
| ----------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`        | `true`                                 | Only show hostname when connected to an SSH session.                                                                                  |
| `ssh_symbol`      | `'🌐 '`                                | A format string representing the symbol when connected to SSH session.                                                                |
| `trim_at`         | `'.'`                                  | String that the hostname is cut off at, after the first match. `'.'` will stop after the first dot. `''` will disable any truncation. |
| `detect_env_vars` | `[]`                                   | Which environment variable(s) should trigger this module.                                                                             |
| `format`          | `'[$ssh_symbol$hostname]($style) in '` | The format for the module.                                                                                                            |
| `style`           | `'bold dimmed green'`                  | The style for the module.                                                                                                             |
| `disabled`        | `false`                                | Disables the `hostname` module.                                                                                                       |
| `aliases`         | `{}`                                   | Translate system hostnames to something else. If `trim_at` is specified, only the first part will be matched and replaced.            |

### Variables

| Variable   | Example    | Description                                           |
| ---------- | ---------- | ----------------------------------------------------- |
| hostname   | `computer` | The hostname of the computer                          |
| style\*    |            | Mirrors the value of option `style`                   |
| ssh_symbol | `'🌏 '`    | The symbol to represent when connected to SSH session |

*: This variable can only be used as a part of a style string

### Examples

#### Always show the hostname

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

#### Hide the hostname in remote tmux sessions

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
detect_env_vars = ['!TMUX', 'SSH_CONNECTION']
disabled = false
```

#### Replace the hostname with a nickname

```toml
# ~/.config/starship.toml
[hostname]
aliases = { "Max's MacBook Pro" = "home" }
```
