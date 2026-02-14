# CPP

The `cpp` module shows some information about your `C++` compiler. By default,
the module will be shown if the current directory contains a `.cpp`, `.hpp`, or other `C++`-related files.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Options

| Option              | Default                                                                          | Description                                                               |
| ------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                     | The format string for the module.                                         |
| `version_format`    | `'v${raw}'`                                                                      | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'C++ '`                                                                         | The symbol used before displaying the compiler details                    |
| `detect_extensions` | `['cpp', 'cc', 'cxx', 'c++', 'hpp', 'hh', 'hxx', 'h++', 'tcc']`                  | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                                                             | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                             | Which folders should trigger this module.                                 |
| `commands`          | `[ [ 'c++', '--version' ], [ 'g++', '--version' ], [ 'clang++', '--version' ] ]` | How to detect what the compiler is                                        |
| `style`             | `'bold 149'`                                                                     | The style for the module.                                                 |
| `disabled`          | `true`                                                                           | Disables the `cpp` module.                                                |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| name     | clang++ | The name of the compiler             |
| version  | 13.0.0  | The version of the compiler          |
| symbol   |         | Mirrors the value of option `symbol` |
| style    |         | Mirrors the value of option `style`  |

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycpp', '--version']`. Starship will try executing each command until it gets a result on STDOUT.

If a C++ compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Example

```toml
# ~/.config/starship.toml

[cpp]
disabled = false
format = 'via [$name $version]($style)'
```
