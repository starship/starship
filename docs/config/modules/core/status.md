# Status

The `status` module displays the exit code of the previous command.
If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`.
The status code will cast to a signed 32-bit integer.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option                      | Default                                                                        | Description                                                           |
| --------------------------- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                  | The format of the module                                              |
| `symbol`                    | `'❌'`                                                                         | The symbol displayed on program error                                 |
| `success_symbol`            | `''`                                                                           | The symbol displayed on program success                               |
| `not_executable_symbol`     | `'🚫'`                                                                         | The symbol displayed when file isn't executable                       |
| `not_found_symbol`          | `'🔍'`                                                                         | The symbol displayed when the command can't be found                  |
| `sigint_symbol`             | `'🧱'`                                                                         | The symbol displayed on SIGINT (Ctrl + c)                             |
| `signal_symbol`             | `'⚡'`                                                                         | The symbol displayed on any signal                                    |
| `style`                     | `'bold red'`                                                                   | The style for the module.                                             |
| `success_style`             |                                                                                | The style used on program success (defaults to `style` if unset).     |
| `failure_style`             |                                                                                | The style used on program failure (defaults to `style` if unset).     |
| `recognize_signal_code`     | `true`                                                                         | Enable signal mapping from exit code                                  |
| `map_symbol`                | `false`                                                                        | Enable symbols mapping from exit code                                 |
| `pipestatus`                | `false`                                                                        | Enable pipestatus reporting                                           |
| `pipestatus_separator`      | <code>&vert;</code>                                                            | The symbol used to separate pipestatus segments (supports formatting) |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style) '` | The format of the module when the command is a pipeline               |
| `pipestatus_segment_format` |                                                                                | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                         | Disables the `status` module.                                         |

### Variables

| Variable       | Example | Description                                                                                  |
| -------------- | ------- | -------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                            |
| hex_status     | `0x7F`  | The exit code of the last command in hex                                                     |
| int            | `127`   | The exit code of the last command                                                            |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                          |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                              |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                         |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                 |
| pipestatus     |         | Rendering of in pipeline programs' exit codes, this is only available in pipestatus_format   |
| symbol         |         | Mirrors the value of option `symbol`                                                         |
| style\*        |         | Mirrors the value of option `success_style` on program success and `failure_style` otherwise |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[status]
style = 'bg:blue'
symbol = '🔴 '
success_symbol = '🟢 SUCCESS'
format = '[\[$symbol$common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```
