# Gleam

The `gleam` module shows the currently installed version of [Gleam](https://gleam.run/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `gleam.toml` file
- The current directory contains a file with the `.gleam` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⭐ '`                              | A format string representing the symbol of Gleam.                         |
| `detect_extensions` | `['gleam']`                          | Which extensions should trigger this module.                              |
| `detect_files`      | `['gleam.toml']`                     | Which filenames should trigger this module.                               |
| `style`             | `'bold #FFAFF3'`                     | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `gleam` module.                                              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v1.0.0` | The version of `gleam`               |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[gleam]
format = 'via [⭐ $version](bold red) '
```
