# R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). The module will be shown if
any of the following conditions are met:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'📐'`                               | A format string representing the symbol of R.                             |
| `style`             | `'blue bold'`                        | The style for the module.                                                 |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Which extensions should trigger this module                               |
| `detect_files`      | `['.Rprofile']`                      | Which filenames should trigger this module                                |
| `detect_folders`    | `['.Rproj.user']`                    | Which folders should trigger this module                                  |
| `disabled`          | `false`                              | Disables the `r` module.                                                  |

### Variables

| Variable | Example       | Description                          |
| -------- | ------------- | ------------------------------------ |
| version  | `v4.0.5`      | The version of `R`                   |
| symbol   |               | Mirrors the value of option `symbol` |
| style    | `'blue bold'` | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```
