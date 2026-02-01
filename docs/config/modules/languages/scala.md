# Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Options

| Option              | Default                                  | Description                                                               |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['sbt', 'scala']`                       | Which extensions should trigger this module.                              |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `['.metals']`                            | Which folders should trigger this modules.                                |
| `symbol`            | `'🆂 '`                                   | A format string representing the symbol of Scala.                         |
| `style`             | `'red dimmed'`                           | The style for the module.                                                 |
| `disabled`          | `false`                                  | Disables the `scala` module.                                              |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `2.13.5` | The version of `scala`               |
| symbol   |          | Mirrors the value of option `symbol` |
| style\*  |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```
