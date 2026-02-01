# Solidity

The `solidity` module shows the currently installed version of [Solidity](https://soliditylang.org/)
The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.sol` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${major}.${minor}.${patch}'`      | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'S '`                               | A format string representing the symbol of Solidity                       |
| `compiler           | ['solc']                             | The default compiler for Solidity.                                        |
| `detect_extensions` | `['sol']`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `'bold blue'`                        | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables this module.                                                     |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v0.8.1` | The version of `solidity`            |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml
[solidity]
format = "via [S $version](blue bold)"
```
