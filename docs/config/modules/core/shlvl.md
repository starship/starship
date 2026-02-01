# SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is
set to a number and meets or exceeds the specified threshold.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option          | Default                      | Description                                                         |
| --------------- | ---------------------------- | ------------------------------------------------------------------- |
| `threshold`     | `2`                          | Display threshold.                                                  |
| `format`        | `'[$symbol$shlvl]($style) '` | The format for the module.                                          |
| `symbol`        | `'↕️  '`                      | The symbol used to represent the `SHLVL`.                           |
| `repeat`        | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount.       |
| `repeat_offset` | `0`                          | Decrements number of times `symbol` is repeated by the offset value |
| `style`         | `'bold yellow'`              | The style for the module.                                           |
| `disabled`      | `true`                       | Disables the `shlvl` module.                                        |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| shlvl    | `3`     | The current value of `SHLVL`         |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

Using `repeat` and `repeat_offset` along with `character` module, one can get
prompt like `❯❯❯` where last character is colored appropriately for return
status code and preceding characters are provided by `shlvl`.

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '[$symbol]($style)'
repeat = true
symbol = '❯'
repeat_offset = 1
```
