# VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository.
The module will be shown only if a repository is currently in use.

### Options

| Option     | Default                          | Description                                            |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   | `''`                             | The symbol used before displaying the repository name. |
| `style`    | `'bold yellow'`                  | The style for the module.                              |
| `format`   | `'vcsh [$symbol$repo]($style) '` | The format for the module.                             |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variables

| Variable | Example                                     | Description                          |
| -------- | ------------------------------------------- | ------------------------------------ |
| repo     | `dotfiles` if in a VCSH repo named dotfiles | The active repository name           |
| symbol   |                                             | Mirrors the value of option `symbol` |
| style\*  | `black bold dimmed`                         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```
