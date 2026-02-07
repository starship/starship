# Gradle

The `gradle` module shows the version of the [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html)
currently used in the project directory.

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `gradle/wrapper/gradle-wrapper.properties` directory.
- The current directory contains a file ending with `.gradle` or `.gradle.kts`.

The `gradle` module is only able to read your Gradle Wrapper version from your config file, we don't execute your wrapper, because of the security concerns.

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🅶 '`                               | A format string representing the symbol of Gradle.                        |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `['gradle']`                         | Which folders should trigger this module.                                 |
| `style`             | `'bold bright-cyan'`                 | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `gradle` module.                                             |
| `recursive`         | `false`                              | Enables recursive finding for the `gradle` directory.                     |

### Variables

| Variable | Example  | Description                          |
| -------- | -------- | ------------------------------------ |
| version  | `v7.5.1` | The version of `gradle`              |
| symbol   |          | Mirrors the value of option `symbol` |
| style*   |          | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string
