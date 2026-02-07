# Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['jl']`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this modules.                                |
| `symbol`            | `'ஃ '`                               | A format string representing the symbol of Julia.                         |
| `style`             | `'bold purple'`                      | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `julia` module.                                              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v1.4.0` | The version of `julia`               |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```
