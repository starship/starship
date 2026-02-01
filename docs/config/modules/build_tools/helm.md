# Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `helmfile.yaml` file
- The current directory contains a `Chart.yaml` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `symbol`            | `'⎈ '`                               | A format string representing the symbol of Helm.                          |
| `style`             | `'bold white'`                       | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `helm` module.                                               |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v3.1.1` | The version of `helm`                |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```
