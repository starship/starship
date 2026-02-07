# Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/).
The module will be shown if any of the following conditions are met:

- The current directory contains a file with `.red` or `.reds` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🔺 '`                              | A format string representing the symbol of Red.                           |
| `detect_extensions` | `['red']`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'red bold'`                         | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `red` module.                                                |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v2.5.1` | The version of `red`                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```
