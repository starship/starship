# Typst

The `typst` module shows the current installed version of Typst used in a project.

By default, the module will be shown if any of the following conditions are met:

- The current directory contains a `template.typ` file
- The current directory contains any `*.typ` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'t '`                               | A format string representing the symbol of Typst                          |
| `style`             | `'bold #0093A7'`                     | The style for the module.                                                 |
| `detect_extensions` | `['.typ']`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `['template.typ']`                   | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `typst` module.                                              |

### Variables

| Variable      | Example   | Description                                     |
| ------------- | --------- | ----------------------------------------------- |
| version       | `v0.9.0`  | The version of `typst`, alias for typst_version |
| typst_version | `default` | The current Typst version                       |
| symbol        |           | Mirrors the value of option `symbol`            |
| style\*       |           | Mirrors the value of option `style`             |

*: This variable can only be used as a part of a style string
