# PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version.
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `spago.dhall` file
- The current directory contains a `spago.yaml` file
- The current directory contains a `spago.lock` file
- The current directory contains a file with the `.purs` extension

### Options

| Option              | Default                                       | Description                                                               |
| ------------------- | --------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                   | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'<=> '`                                      | The symbol used before displaying the version of PureScript.              |
| `detect_extensions` | `['purs']`                                    | Which extensions should trigger this module.                              |
| `detect_files`      | `['spago.dhall', 'spago.yaml', 'spago.lock']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                          | Which folders should trigger this module.                                 |
| `style`             | `'bold white'`                                | The style for the module.                                                 |
| `disabled`          | `false`                                       | Disables the `purescript` module.                                         |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `0.13.5` | The version of `purescript`          |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```
