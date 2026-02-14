# Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

> [!TIP]
> This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.
> If you use [pixi](https://pixi.sh), you can disable pixi's prompt modifier by running `pixi config set shell.change-ps1 false`.

### Options

| Option              | Default                                | Description                                                                                                                                                                                                        |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](../core/directory) module. |
| `symbol`            | `'🅒 '`                                 | The symbol used before the environment name.                                                                                                                                                                       |
| `style`             | `'bold green'`                         | The style for the module.                                                                                                                                                                                          |
| `format`            | `'via [$symbol$environment]($style) '` | The format for the module.                                                                                                                                                                                         |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                                                                                                                         |
| `detect_env_vars`   | `["!PIXI_ENVIRONMENT_NAME"]`           | Which environment variable(s) should trigger this module. If it's a pixi environment, this module is not being triggered by default.                                                                               |
| `disabled`          | `false`                                | Disables the `conda` module.                                                                                                                                                                                       |

### Variables

| Variable    | Example      | Description                          |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | The current conda environment        |
| symbol      |              | Mirrors the value of option `symbol` |
| style\*     |              | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```
