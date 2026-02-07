# Fortran

The `fortran` module shows the current compiler version of Fortran.

### Options

| Option              | Default                                                                                                                     | Description                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `symbol`            | `' '`                                                                                                                      | The symbol used before displaying the version of Fortran.                 |
| `format`            | `'via [$symbol($version )]($style)'`                                                                                        | The format for the module.                                                |
| `version_format`    | `'${raw}'`                                                                                                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold purple'`                                                                                                             | The style for the module.                                                 |
| `detect_extensions` | `['f', 'F', 'for', 'FOR', 'ftn', 'FTN', 'f77', 'F77', 'f90', 'F90', 'f95', 'F95','f03', 'F03', 'f08', 'F08', 'f18', 'F18']` | Which extensions should trigger this module.                              |
| `detect_files`      | `['fpm.toml']`                                                                                                              | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                                        | Which folders should trigger this module.                                 |
| `commands`          | `[ [ 'gfortran', '--version' ], [ 'flang', '--version' ], [ 'flang-new', '--version' ] ]`                                   | How to detect what the compiler is                                        |
| `disabled`          | `false`                                                                                                                     | Disables the `fortran` module.                                            |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| name     | gfortran | The name of the compiler             |
| version  | `14.2.0` | The version of the Fortran compiler  |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Commands

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['myfortran', '--version']`. Starship will try executing each command until it gets a result on STDOUT.

If a Fortran compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/).
