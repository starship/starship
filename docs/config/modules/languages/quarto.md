# Quarto

The `quarto` module shows the current installed version of Quarto used in a project.

By default, the module will be shown if any of the following conditions are met:

- The current directory contains a `_quarto.yml` file
- The current directory contains any `*.qmd` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⨁ '`                               | A format string representing the symbol of Quarto                         |
| `style`             | `'bold #75AADB'`                     | The style for the module.                                                 |
| `detect_extensions` | `['.qmd']`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `['_quarto.yml']`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `quarto` module.                                             |

### Variables

| Variable | Example   | Description                          |
| -------- | --------- | ------------------------------------ |
| version  | `1.4.549` | The version of `quarto`              |
| symbol   |           | Mirrors the value of option `symbol` |
| style\*  |           | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string
