# Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option      | Default                                        | Description                                              |
| ----------- | ---------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                           | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | The format for the module.                               |
| `symbol`    | `'🐏'`                                         | The symbol used before displaying the memory usage.      |
| `style`     | `'bold dimmed white'`                          | The style for the module.                                |
| `disabled`  | `true`                                         | Disables the `memory_usage` module.                      |

### Variables

| Variable     | Example       | Description                                                        |
| ------------ | ------------- | ------------------------------------------------------------------ |
| ram          | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct      | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol       | `🐏`          | Mirrors the value of option `symbol`                               |
| style\*      |               | Mirrors the value of option `style`                                |

*: This variable can only be used as a part of a style string
*\*: The SWAP file information is only displayed if detected on the current system

### Example

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = ' '
style = 'bold dimmed green'
```
