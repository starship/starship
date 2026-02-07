# Pixi

The `pixi` module shows the installed [pixi](https://pixi.sh) version as well as the activated environment, if `$PIXI_ENVIRONMENT_NAME` is set.

> [!TIP]
> This does not suppress pixi's own prompt modifier, you may want to run `pixi config set shell.change-ps1 false`.

### Options

| Option                     | Default                                                 | Description                                                                       |
| -------------------------- | ------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`                   | `'via [$symbol($version )(\($environment\) )]($style)'` | The format for the module.                                                        |
| `version_format`           | `'v${raw}'`                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch`.        |
| `symbol`                   | `'🧚 '`                                                 | The symbol used before the environment name.                                      |
| `style`                    | `'yellow bold'`                                         | The style for the module.                                                         |
| `show_default_environment` | `true`                                                  | Whether to indicate that the `default` environment of your project is activated.  |
| `pixi_binary`              | `['pixi']`                                              | Configures the pixi binary that Starship should execute when getting the version. |
| `detect_extensions`        | `[]`                                                    | Which extensions should trigger this module.                                      |
| `detect_files`             | `['pixi.toml']`                                         | Which filenames should trigger this module.                                       |
| `detect_folders`           | `[]`                                                    | Which folders should trigger this module.                                         |
| `disabled`                 | `false`                                                 | Disables the `pixi` module.                                                       |

### Variables

| Variable    | Example   | Description                          |
| ----------- | --------- | ------------------------------------ |
| version     | `v0.33.0` | The version of `pixi`                |
| environment | `py311`   | The current pixi environment         |
| symbol      |           | Mirrors the value of option `symbol` |
| style       |           | Mirrors the value of option `style`  |

### Example

```toml
# ~/.config/starship.toml

[pixi]
format = '[$symbol$environment](yellow) '
```
