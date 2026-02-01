# XMake

The `xmake` module shows the currently installed version of [XMake](https://xmake.io/). By default
the module will be activated if any of the following conditions are met:

- The current directory contains a `xmake.lua` file

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'△ '`                               | The symbol used before the version of cmake.                              |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module                               |
| `detect_files`      | `['xmake.lua']`                      | Which filenames should trigger this module                                |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module                                  |
| `style`             | `'bold green'`                       | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `xmake` module.                                              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v2.9.5` | The version of xmake                 |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string
