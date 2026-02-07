# Time

The `time` module shows the current **local** time.
The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option            | Default                 | Description                                                                                                            |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | The format string for the module.                                                                                      |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                             |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.    |
| `style`           | `'bold yellow'`         | The style for the module time                                                                                          |
| `utc_time_offset` | `'local'`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                            |
| `time_range`      | `'-'`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                  |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`.
Manually setting `time_format` will override the `use_12hr` setting.

### Variables

| Variable | Example    | Description                         |
| -------- | ---------- | ----------------------------------- |
| time     | `13:08:10` | The current time.                   |
| style\*  |            | Mirrors the value of option `style` |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = '%T'
utc_time_offset = '-5'
time_range = '10:00:00-14:00:00'
```
