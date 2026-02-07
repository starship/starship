# Battery

The `battery` module shows how charged the device's battery is and its current charging status.
The module is only visible when the device's battery is below 10%.

### Options

| Option               | Default                           | Description                                         |
| -------------------- | --------------------------------- | --------------------------------------------------- |
| `full_symbol`        | `'󰁹 '`                            | The symbol shown when the battery is full.          |
| `charging_symbol`    | `'󰂄 '`                            | The symbol shown when the battery is charging.      |
| `discharging_symbol` | `'󰂃 '`                            | The symbol shown when the battery is discharging.   |
| `unknown_symbol`     | `'󰁽 '`                            | The symbol shown when the battery state is unknown. |
| `empty_symbol`       | `'󰂎 '`                            | The symbol shown when the battery state is empty.   |
| `format`             | `'[$symbol$percentage]($style) '` | The format for the module.                          |
| `display`            | [link](#battery-display)          | Display threshold and style for the module.         |
| `disabled`           | `false`                           | Disables the `battery` module.                      |

### Example

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### Battery Display

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style).
If no `display` is provided. The default is as shown:

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Options

The `display` option is an array of the following table.

| Option               | Default      | Description                                                                                               |
| -------------------- | ------------ | --------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | The upper bound for the display option.                                                                   |
| `style`              | `'red bold'` | The style used if the display option is in use.                                                           |
| `charging_symbol`    |              | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.    |
| `discharging_symbol` |              | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option. |

#### Example

```toml
[[battery.display]] # 'bold red' style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = 'bold red'

[[battery.display]] # 'bold yellow' style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦 '

# when capacity is over 30%, the battery indicator will not be displayed
```
