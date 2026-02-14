# COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL.
By default, the module will be shown if any of the following conditions are met:

- The current directory contains any files ending in `.cob` or `.COB`
- The current directory contains any files ending in `.cbl` or `.CBL`

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `symbol`            | `'⚙️ '`                               | The symbol used before displaying the version of COBOL.                   |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold blue'`                        | The style for the module.                                                 |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `disabled`          | `false`                              | Disables the `cobol` module.                                              |

### Variables

| Variable | Example    | Description                          |
| -------- | ---------- | ------------------------------------ |
| version  | `v3.1.2.0` | The version of `cobol`               |
| symbol   |            | Mirrors the value of option `symbol` |
| style\*  |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string
