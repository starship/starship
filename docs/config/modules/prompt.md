# Prompt

This is the list of prompt-wide configuration options.

### Options

| Option            | Default                                    | Description                                                                                                                                                                      |
| ----------------- | ------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [link](../README.md#default-prompt-format) | Configure the format of the prompt.                                                                                                                                              |
| `right_format`    | `''`                                       | See [Enable Right Prompt](/advanced-config/#enable-right-prompt)                                                                                                                 |
| `scan_timeout`    | `30`                                       | Timeout for starship to scan files (in milliseconds).                                                                                                                            |
| `command_timeout` | `500`                                      | Timeout for commands executed by starship (in milliseconds).                                                                                                                     |
| `add_newline`     | `true`                                     | Inserts blank line between shell prompts.                                                                                                                                        |
| `palette`         | `''`                                       | Sets which color palette from `palettes` to use.                                                                                                                                 |
| `palettes`        | `{}`                                       | Collection of color palettes that assign [colors](/advanced-config/#style-strings) to user-defined names. Note that color palettes cannot reference their own color definitions. |
| `follow_symlinks` | `true`                                     | Follows symlinks to check if they're directories; used in modules such as git.                                                                                                   |

> [!TIP]
> If you have symlinks to networked filesystems, consider setting
> `follow_symlinks` to `false`.

### Example

```toml
# ~/.config/starship.toml

# Use custom format
format = '''
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) '''

# Wait 10 milliseconds for starship to check files under the current directory.
scan_timeout = 10

# Disable the blank line at the start of the prompt
add_newline = false

# Set 'foo' as custom color palette
palette = 'foo'

# Define custom colors
[palettes.foo]
# Overwrite existing color
blue = '21'
# Define new color
mustard = '#af8700'
```

### Default Prompt Format

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. The default is as shown:

```toml
format = '$all'

# Which is equivalent to
format = """
$username\
$hostname\
$localip\
$shlvl\
$singularity\
$kubernetes\
$directory\
$vcsh\
$fossil_branch\
$fossil_metrics\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$hg_state\
$pijul_channel\
$docker_context\
$package\
$c\
$cmake\
$cobol\
$daml\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$fennel\
$fortran\
$gleam\
$golang\
$guix_shell\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$gradle\
$lua\
$maven\
$nim\
$nodejs\
$ocaml\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$quarto\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$solidity\
$swift\
$terraform\
$typst\
$vlang\
$vagrant\
$zig\
$buf\
$nix_shell\
$conda\
$meson\
$spack\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
$nats\
$direnv\
$env_var\
$mise\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$os\
$container\
$netns\
$shell\
$character"""
```

If you just want to extend the default format, you can use `$all`;
modules you explicitly add to the format will not be duplicated. Eg.

```toml
# Move the directory to the second line
format = '$all$directory$character'
```
