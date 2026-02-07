# Shell

The `shell` module shows an indicator for currently used shell.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option                 | Default                   | Description                                                                                            |
| ---------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------ |
| `bash_indicator`       | `'bsh'`                   | A format string used to represent bash.                                                                |
| `fish_indicator`       | `'fsh'`                   | A format string used to represent fish.                                                                |
| `zsh_indicator`        | `'zsh'`                   | A format string used to represent zsh.                                                                 |
| `powershell_indicator` | `'psh'`                   | A format string used to represent powershell.                                                          |
| `pwsh_indicator`       |                           | A format string used to represent pwsh. The default value mirrors the value of `powershell_indicator`. |
| `ion_indicator`        | `'ion'`                   | A format string used to represent ion.                                                                 |
| `elvish_indicator`     | `'esh'`                   | A format string used to represent elvish.                                                              |
| `tcsh_indicator`       | `'tsh'`                   | A format string used to represent tcsh.                                                                |
| `xonsh_indicator`      | `'xsh'`                   | A format string used to represent xonsh.                                                               |
| `cmd_indicator`        | `'cmd'`                   | A format string used to represent cmd.                                                                 |
| `nu_indicator`         | `'nu'`                    | A format string used to represent nu.                                                                  |
| `unknown_indicator`    | `''`                      | The default value to be displayed when the shell is unknown.                                           |
| `format`               | `'[$indicator]($style) '` | The format for the module.                                                                             |
| `style`                | `'white bold'`            | The style for the module.                                                                              |
| `disabled`             | `true`                    | Disables the `shell` module.                                                                           |

### Variables

| Variable  | Default | Description                                                |
| --------- | ------- | ---------------------------------------------------------- |
| indicator |         | Mirrors the value of `indicator` for currently used shell. |
| style\*   |         | Mirrors the value of option `style`.                       |

*: This variable can only be used as a part of a style string

### Examples

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = '󰈺 '
powershell_indicator = '_'
unknown_indicator = 'mystery shell'
style = 'cyan bold'
disabled = false
```
