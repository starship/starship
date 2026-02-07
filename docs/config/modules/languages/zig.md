# Zig

By default the `zig` module shows the currently installed version of [Zig](https://ziglang.org/).
The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'↯ '`                               | The symbol used before displaying the version of Zig.                     |
| `style`             | `'bold yellow'`                      | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `zig` module.                                                |
| `detect_extensions` | `['zig']`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v0.6.0` | The version of `zig`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```
