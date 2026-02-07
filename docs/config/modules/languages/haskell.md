# Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `stack.yaml` file
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### Options

| Option              | Default                              | Description                                        |
| ------------------- | ------------------------------------ | -------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                         |
| `symbol`            | `'λ '`                               | A format string representing the symbol of Haskell |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Which extensions should trigger this module.       |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | Which filenames should trigger this module.        |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.          |
| `style`             | `'bold purple'`                      | The style for the module.                          |
| `disabled`          | `false`                              | Disables the `haskell` module.                     |

### Variables

| Variable     | Example     | Description                                                                             |
| ------------ | ----------- | --------------------------------------------------------------------------------------- |
| version      |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot     | `lts-18.12` | Currently selected Stack snapshot                                                       |
| ghc\_version | `9.2.1`     | Currently installed GHC version                                                         |
| symbol       |             | Mirrors the value of option `symbol`                                                    |
| style\*      |             | Mirrors the value of option `style`                                                     |

*: This variable can only be used as a part of a style string
