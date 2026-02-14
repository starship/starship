# Maven

The `maven` module indicates the presence of a Maven project in the current directory. If the [Maven Wrapper](https://maven.apache.org/wrapper/) is enabled, the Maven version will be parsed from `.mvn/wrapper/maven-wrapper.properties` and shown.

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `pom.xml` file.
- The current directory contains a `.mvn/wrapper/maven-wrapper.properties` file.

If you use an alternate POM syntax (for example `pom.hocon`), add its filename to `detect_files`.

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🅼 '`                               | A format string representing the symbol of Maven.                         |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `['pom.xml']`                        | Which filenames should trigger this module.                               |
| `detect_folders`    | `['.mvn']`                           | Which folders should trigger this module.                                 |
| `style`             | `'bold bright-cyan'`                 | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `maven` module.                                              |
| `recursive`         | `false`                              | Enables recursive finding for the `.mvn` directory.                       |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v3.2.0` | The version of `maven`               |
| symbol   |          | Mirrors the value of option `symbol` |
| style*   |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string
