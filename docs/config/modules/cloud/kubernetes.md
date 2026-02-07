# Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file.
The namespace needs to be set in the kubeconfig file, this can be done via
`kubectl config set-context starship-context --namespace astronaut`.
Similarly, the user and cluster can be set with `kubectl config set-context starship-context --user starship-user`
and `kubectl config set-context starship-context --cluster starship-cluster`.
If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.
>
> When the module is enabled it will always be active, unless any of
> `detect_env_vars`, `detect_extensions`, `detect_files` or `detect_folders` have
> been set in which case the module will only be active in directories that match
> those conditions or one of the environmental variables has been set.

### Options

> [!WARNING]
> The `context_aliases` and `user_aliases` options are deprecated. Use `contexts` and the corresponding `context_alias`
> and `user_alias` options instead.

| Option              | Default                                            | Description                                                           |
| ------------------- | -------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`            | `'☸ '`                                             | A format string representing the symbol displayed before the Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | The format for the module.                                            |
| `style`             | `'cyan bold'`                                      | The style for the module.                                             |
| `context_aliases`*  | `{}`                                               | Table of context aliases to display.                                  |
| `user_aliases`*     | `{}`                                               | Table of user aliases to display.                                     |
| `detect_extensions` | `[]`                                               | Which extensions should trigger this module.                          |
| `detect_files`      | `[]`                                               | Which filenames should trigger this module.                           |
| `detect_folders`    | `[]`                                               | Which folders should trigger this modules.                            |
| `detect_env_vars`   | `[]`                                               | Which environmental variables should trigger this module              |
| `contexts`          | `[]`                                               | Customized styles and symbols for specific contexts.                  |
| `disabled`          | `true`                                             | Disables the `kubernetes` module.                                     |

*: This option is deprecated, please add `contexts` with the corresponding `context_alias` and `user_alias` options instead.

To customize the style of the module for specific environments, use the following configuration as
part of the `contexts` list:

| Variable          | Description                                                                              |
| ----------------- | ---------------------------------------------------------------------------------------- |
| `context_pattern` | **Required** Regular expression to match current Kubernetes context name.                |
| `user_pattern`    | Regular expression to match current Kubernetes user name.                                |
| `context_alias`   | Context alias to display instead of the full context name.                               |
| `user_alias`      | User alias to display instead of the full user name.                                     |
| `style`           | The style for the module when using this context. If not set, will use module's style.   |
| `symbol`          | The symbol for the module when using this context. If not set, will use module's symbol. |

Note that all regular expression are anchored with `^<pattern>$` and so must match the whole string. The `*_pattern`
regular expressions may contain capture groups, which can be referenced in the corresponding alias via `$name` and `$N`
(see example below and the
[rust Regex::replace() documentation](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)).

### Variables

| Variable  | Example              | Description                              |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbol    |                      | Mirrors the value of option `symbol`     |
| style\*   |                      | Mirrors the value of option `style`      |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
contexts = [
  { context_pattern = "dev.local.cluster.k8s", style = "green", symbol = "💔 " },
]
```

Only show the module in directories that contain a `k8s` file.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Kubernetes Context specific config

The `contexts` configuration option is used to customise what the current Kubernetes context name looks
like (style and symbol) if the name matches the defined regular expression.

```toml
# ~/.config/starship.toml

[[kubernetes.contexts]]
# "bold red" style + default symbol when Kubernetes current context name equals "production" *and* the current user
# equals "admin_user"
context_pattern = "production"
user_pattern = "admin_user"
style = "bold red"
context_alias = "prod"
user_alias = "admin"

[[kubernetes.contexts]]
# "green" style + a different symbol when Kubernetes current context name contains openshift
context_pattern = ".*openshift.*"
style = "green"
symbol = "💔 "
context_alias = "openshift"

[[kubernetes.contexts]]
# Using capture groups
# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
context_pattern = "gke_.*_(?P<cluster>[\\w-]+)"
context_alias = "gke-$cluster"
```
