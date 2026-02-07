# Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI.
This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

When the module is enabled it will always be active, unless `detect_env_vars` has
been set in which case the module will only be active when one of the
environment variables has been set.

### Options

| Option            | Default                                                  | Description                                                      |
| ----------------- | -------------------------------------------------------- | ---------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | The format for the module.                                       |
| `symbol`          | `'☁️  '`                                                  | The symbol used before displaying the current GCP profile.       |
| `region_aliases`  | `{}`                                                     | Table of region aliases to display in addition to the GCP name.  |
| `project_aliases` | `{}`                                                     | Table of project aliases to display in addition to the GCP name. |
| `detect_env_vars` | `[]`                                                     | Which environmental variables should trigger this module         |
| `style`           | `'bold blue'`                                            | The style for the module.                                        |
| `disabled`        | `false`                                                  | Disables the `gcloud` module.                                    |

### Variables

| Variable | Example       | Description                                                        |
| -------- | ------------- | ------------------------------------------------------------------ |
| region   | `us-central1` | The current GCP region                                             |
| account  | `foo`         | The current GCP profile                                            |
| domain   | `example.com` | The current GCP profile domain                                     |
| project  |               | The current GCP project                                            |
| active   | `default`     | The active config name written in `~/.config/gcloud/active_config` |
| symbol   |               | Mirrors the value of option `symbol`                               |
| style\*  |               | Mirrors the value of option `style`                                |

*: This variable can only be used as a part of a style string

### Examples

#### Display account and project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
```

#### Display active config name only

```toml
# ~/.config/starship.toml

[gcloud]
format = '[$symbol$active]($style) '
style = 'bold yellow'
```

#### Display account and aliased region

```toml
# ~/.config/starship.toml

[gcloud]
symbol = '️🇬️ '
[gcloud.region_aliases]
us-central1 = 'uc1'
asia-northeast1 = 'an1'
```

#### Display account and aliased project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = 'vlpn'
```
