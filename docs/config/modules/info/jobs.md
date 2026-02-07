# Jobs

The `jobs` module shows the current number of jobs running.
The module will be shown only if there are background jobs running.
The module will show the number of jobs running if there are at least
2 jobs, or more than the `number_threshold` config value, if it exists.
The module will show a symbol if there is at least 1 job, or more than the
`symbol_threshold` config value, if it exists. You can set both values
to 0 in order to _always_ show the symbol and number of jobs, even if there are
0 jobs running.

The default functionality is:

- 0 jobs -> Nothing is shown.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

> [!WARNING]
> This module is not supported on tcsh.

> [!WARNING]
> The `threshold` option is deprecated, but if you want to use it,
> the module will show the number of jobs running if there is more than 1 job, or
> more than the `threshold` config value, if it exists. If `threshold` is set to 0,
> then the module will also show when there are 0 jobs running.

### Options

| Option             | Default                       | Description                                                              |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | The format for the module.                                               |
| `symbol`           | `'✦'`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `'bold blue'`                 | The style for the module.                                                |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| number   | `1`     | The number of jobs                   |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Examples

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
symbol_threshold = 0
```

#### Changing process grouping behavior in fish

When using the Fish shell, Starship counts **job groups** instead of individual process IDs by default. This prevents overcounting when a pipeline has multiple processes but only one suspended group. To revert to the legacy PID-based counting, please add the following to your shell config:

```fish
set -g __starship_fish_use_job_groups "false"
```
