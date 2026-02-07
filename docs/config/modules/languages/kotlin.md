# Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.kt` or a `.kts` file

### Options

| Option              | Default                              | Description                                                                   |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                    |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`     |
| `detect_extensions` | `['kt', 'kts']`                      | Which extensions should trigger this module.                                  |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                                   |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                    |
| `symbol`            | `'🅺 '`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `'bold blue'`                        | The style for the module.                                                     |
| `kotlin_binary`     | `'kotlin'`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v1.4.21` | The version of `kotlin`              |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[kotlin]
symbol = '🅺 '
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = 'kotlinc'
```
