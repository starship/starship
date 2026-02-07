# Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers)
SDK version when you are in the root directory of your Daml project. The `sdk-version` in
the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION`
environment variable.
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `daml.yaml` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'Λ '`                               | A format string representing the symbol of Daml                           |
| `style`             | `'bold cyan'`                        | The style for the module.                                                 |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['daml.yaml']`                      | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `daml` module.                                               |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v2.2.0` | The version of `daml`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```
