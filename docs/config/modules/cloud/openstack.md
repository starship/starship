# OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module
only active when the `OS_CLOUD` env var is set, in which case it will read
`clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files).
to fetch the current project in use.

### Options

| Option     | Default                                       | Description                                                    |
| ---------- | --------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | The format for the module.                                     |
| `symbol`   | `'☁️ '`                                        | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `'bold yellow'`                               | The style for the module.                                      |
| `disabled` | `false`                                       | Disables the `openstack` module.                               |

### Variables

| Variable | Example | Description                          |
| -------- | ------- | ------------------------------------ |
| cloud    | `corp`  | The current OpenStack cloud          |
| project  | `dev`   | The current OpenStack project        |
| symbol   |         | Mirrors the value of option `symbol` |
| style\*  |         | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```
