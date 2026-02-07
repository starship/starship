# Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/).
The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Options

| Option              | Default                              | Description                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💎 '`                              | A format string representing the symbol of Ruby.                          |
| `detect_extensions` | `['rb']`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Which environment variables should trigger this module.                   |
| `style`             | `'bold red'`                         | The style for the module.                                                 |
| `disabled`          | `false`                              | Disables the `ruby` module.                                               |

### Variables

| Variable | Example  | Description                                 |
| -------- | -------- | ------------------------------------------- |
| version  | `v2.5.1` | The version of `ruby`                       |
| symbol   |          | Mirrors the value of option `symbol`        |
| style\*  |          | Mirrors the value of option `style`         |
| gemset   | `test`   | Optional, gets the current RVM gemset name. |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```
