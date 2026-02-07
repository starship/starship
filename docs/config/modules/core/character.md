# Character

The `character` module shows a character (usually an arrow) beside where the text
is entered in your terminal.

The character will tell you whether the last command was successful or not. It
can do this in two ways:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

By default it only changes color. If you also want to change its shape take a
look at [this example](#with-custom-error-shape).

> [!WARNING]
> `vimcmd_symbol` is only supported in cmd, fish and zsh.
> `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol`
> are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

### Options

| Option                      | Default              | Description                                                                             |
| --------------------------- | -------------------- | --------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | The format string used before the text input.                                           |
| `success_symbol`            | `'[❯](bold green)'`  | The format string used before the text input if the previous command succeeded.         |
| `error_symbol`              | `'[❯](bold red)'`    | The format string used before the text input if the previous command failed.            |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | The format string used before the text input if the shell is in vim normal mode.        |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim `replace_one` mode. |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim replace mode.       |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | The format string used before the text input if the shell is in vim visual mode.        |
| `disabled`                  | `false`              | Disables the `character` module.                                                        |

### Variables

| Variable | Example | Description                                                                                              |
| -------- | ------- | -------------------------------------------------------------------------------------------------------- |
| symbol   |         | A mirror of either `success_symbol`, `error_symbol`, `vimcmd_symbol` or `vimcmd_replace_one_symbol` etc. |

### Examples

#### With custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[✗](bold red) '
```

#### Without custom error shape

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[➜](bold red) '
```

#### With custom vim shape

```toml
# ~/.config/starship.toml

[character]
vimcmd_symbol = '[V](bold green) '
```
