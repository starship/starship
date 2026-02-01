# Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/).
The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐦 '`                              | A format string representing the symbol of Swift                          |
| `detect_extensions` | `['swift']`                          | Which extensions should trigger this module.                              |
| `detect_files`      | `['Package.swift']`                  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'bold 202'`                         | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `swift` module.                                              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v5.2.4` | The version of `swift`               |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[swift]
format = 'via [🏎  $version](red bold)'
```
