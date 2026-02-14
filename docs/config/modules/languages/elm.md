# Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains `*.elm` files

### Options

| Option              | Default                                            | Description                                                               |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`               | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                        | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🌳 '`                                            | A format string representing the symbol of Elm.                           |
| `detect_extensions` | `['elm']`                                          | Which extensions should trigger this module.                              |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `['elm-stuff']`                                    | Which folders should trigger this modules.                                |
| `style`             | `'cyan bold'`                                      | The style for the module.                                                 |
| `disabled`          | `false`                                            | Disables the `elm` module.                                                |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `v0.19.1` | The version of `elm`                 |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```
