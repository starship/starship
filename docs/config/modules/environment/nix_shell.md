# Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment.
The module will be shown when inside a nix-shell environment.

### Options

| Option        | Default                                      | Description                                                           |
| ------------- | -------------------------------------------- | --------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | The format for the module.                                            |
| `symbol`      | `'❄️ '`                                       | A format string representing the symbol of nix-shell.                 |
| `style`       | `'bold blue'`                                | The style for the module.                                             |
| `impure_msg`  | `'impure'`                                   | A format string shown when the shell is impure.                       |
| `pure_msg`    | `'pure'`                                     | A format string shown when the shell is pure.                         |
| `unknown_msg` | `''`                                         | A format string shown when it is unknown if the shell is pure/impure. |
| `disabled`    | `false`                                      | Disables the `nix_shell` module.                                      |
| `heuristic`   | `false`                                      | Attempts to detect new `nix shell`-style shells with a heuristic.     |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| state    | `pure`  | The state of the nix-shell           |
| name     | `lorri` | The name of the nix-shell            |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = '[impure shell](bold red)'
pure_msg = '[pure shell](bold green)'
unknown_msg = '[unknown shell](bold yellow)'
format = 'via [☃️ $state( \($name\))](bold blue) '
```
