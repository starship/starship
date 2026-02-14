# Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot`, or `.sdkmanrc` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Options

| Option              | Default                                                                                                               | Description                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                                                                                           | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | Which extensions should trigger this module.                              |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                                  | Which folders should trigger this modules.                                |
| `symbol`            | `'☕ '`                                                                                                               | A format string representing the symbol of Java                           |
| `style`             | `'red dimmed'`                                                                                                        | The style for the module.                                                 |
| `disabled`          | `false`                                                                                                               | Disables the `java` module.                                               |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| version  | `v14`   | The version of `java`                |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```
