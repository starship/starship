# Fennel

The `fennel` module shows the currently installed version of [Fennel](https://fennel-lang.org).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.fnl` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🧅 '`                              | The symbol used before displaying the version of fennel.                  |
| `style`             | `'bold green'`                       | The style for the module.                                                 |
| `detect_extensions` | `['fnl']`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `disabled`          | `false`                              | Disables the `fennel` module.                                             |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v1.2.1` | The version of `fennel`              |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```
