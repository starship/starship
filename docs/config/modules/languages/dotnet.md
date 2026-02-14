# Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. If
the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module
shows the latest installed version of the SDK.

By default this module will only be shown in your prompt when one or more of
the following files are present in the current directory:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

You'll also need the .NET Core SDK installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast
as running `dotnet --version`, but it may show an incorrect version if your .NET project has an
unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by
setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker
(<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>)
when there is a `.csproj` file in the current directory.

### Options

| Option              | Default                                                                                                 | Description                                                               |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                          | The format for the module.                                                |
| `version_format`    | `'v${raw}'`                                                                                             | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'.NET '`                                                                                               | The symbol used before displaying the version of dotnet.                  |
| `heuristic`         | `true`                                                                                                  | Use faster version detection to keep starship snappy.                     |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Which extensions should trigger this module.                              |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                    | Which folders should trigger this modules.                                |
| `style`             | `'bold blue'`                                                                                           | The style for the module.                                                 |
| `disabled`          | `false`                                                                                                 | Disables the `dotnet` module.                                             |

### Variables

| Variable | Example          | Description                                                        |
| -------- | ---------------- | ------------------------------------------------------------------ |
| version  | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm      | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol   |                  | Mirrors the value of option `symbol`                               |
| style\*  |                  | Mirrors the value of option `style`                                |

*: This variable can only be used as a part of a style string

### Example

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```
