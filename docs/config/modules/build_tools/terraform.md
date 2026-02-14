# Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.
It supports both Hashicorp Terraform and OpenTofu for version detection.

> [!TIP]
> By default the Terraform/OpenTofu version is not shown, since this is slow for current versions when a lot of plugins are in use.
> If you still want to enable it, [follow the example shown below](#with-terraform-version).

By default the module will be shown if any of the following conditions are met:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Options

| Option              | Default                                                 | Description                                                               |
| ------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$workspace]($style) '`                    | The format string for the module.                                         |
| `version_format`    | `'v${raw}'`                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💠'`                                                  | A format string shown before the terraform workspace.                     |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `['.terraform']`                                        | Which folders should trigger this module.                                 |
| `style`             | `'bold 105'`                                            | The style for the module.                                                 |
| `disabled`          | `false`                                                 | Disables the `terraform` module.                                          |
| `commands`          | `[ [ 'terraform', 'version' ], [ 'tofu', 'version' ] ]` | How to detect what the Terraform version is.                              |

### Variables

| Variable  | Example    | Description                          |
| --------- | ---------- | ------------------------------------ |
| version   | `v0.12.24` | The version of `terraform`           |
| workspace | `default`  | The current Terraform workspace      |
| symbol    |            | Mirrors the value of option `symbol` |
| style\*   |            | Mirrors the value of option `style`  |

*: This variable can only be used as a part of a style string

### Example

#### With Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$version $workspace]($style) '
```

#### Without Terraform version

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$workspace]($style) '
```
