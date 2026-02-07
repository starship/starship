# Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/).
By default the module will be shown if any of the following conditions are met:

- The current directory contains a `mix.exs` file.

### Options

| Option              | Default                                                   | Description                                                               |
| ------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                                         |
| `version_format`    | `'v${raw}'`                                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💧 '`                                                   | The symbol used before displaying the version of Elixir/Erlang.           |
| `detect_extensions` | `[]`                                                      | Which extensions should trigger this module.                              |
| `detect_files`      | `['mix.exs']`                                             | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                      | Which folders should trigger this modules.                                |
| `style`             | `'bold purple'`                                           | The style for the module.                                                 |
| `disabled`          | `false`                                                   | Disables the `elixir` module.                                             |

### Variables

| Variable    | Example | Description                          |
| ----------- | ------- | ------------------------------------ |
| version     | `v1.10` | The version of `elixir`              |
| otp_version |         | The otp version of `elixir`          |
| symbol      |         | Mirrors the value of option `symbol` |
| style\*     |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```
