# OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Options

| Option                    | Default                                                                  | Description                                                               |
| ------------------------- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | The format string for the module.                                         |
| `version_format`          | `'v${raw}'`                                                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `'🐫 '`                                                                  | The symbol used before displaying the version of OCaml.                   |
| `global_switch_indicator` | `''`                                                                     | The format string used to represent global OPAM switch.                   |
| `local_switch_indicator`  | `'*'`                                                                    | The format string used to represent local OPAM switch.                    |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                     | Which extensions should trigger this module.                              |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`         | Which filenames should trigger this module.                               |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                  | Which folders should trigger this module.                                 |
| `style`                   | `'bold yellow'`                                                          | The style for the module.                                                 |
| `disabled`                | `false`                                                                  | Disables the `ocaml` module.                                              |

### Variables

| Variable         | Example      | Description                                                       |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Mirrors the value of option `symbol`                              |
| style\*          |              | Mirrors the value of option `style`                               |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```
