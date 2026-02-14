# Environment Variable

The `env_var` module displays the current value of a selected environment variables.
The module will be shown only if any of the following conditions are met:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

> [!TIP]
> The order in which env_var modules are shown can be individually set by including
> `${env_var.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`).
> By default, the `env_var` module will simply show all env_var modules in the order they were defined.

> [!TIP]
> Multiple environmental variables can be displayed by using a `.`. (see example)
> If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.
>
> Example: following configuration will display value of USER environment variable
>
> ```toml
> # ~/.config/starship.toml
>
> [env_var.USER]
> default = 'unknown user'
> ```

### Options

| Option        | Default                        | Description                                                                  |
| ------------- | ------------------------------ | ---------------------------------------------------------------------------- |
| `symbol`      | `""`                           | The symbol used before displaying the variable value.                        |
| `variable`    |                                | The environment variable to be displayed.                                    |
| `default`     |                                | The default value to be displayed when the selected variable is not defined. |
| `format`      | `"with [$env_value]($style) "` | The format for the module.                                                   |
| `description` | `"<env_var module>"`           | The description of the module that is shown when running `starship explain`. |
| `disabled`    | `false`                        | Disables the `env_var` module.                                               |

### Variables

| Variable  | Example                                     | Description                                |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol    |                                             | Mirrors the value of option `symbol`       |
| style\*   | `black bold dimmed`                         | Mirrors the value of option `style`        |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[env_var]
variable = 'SHELL'
default = 'unknown shell'
```

Displaying multiple environmental variables:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = 'SHELL'
default = 'unknown shell'
[env_var.USER]
default = 'unknown user'
```
