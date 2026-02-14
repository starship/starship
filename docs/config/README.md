# Configuration

To get started configuring starship, create the following file: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

All configuration for starship is done in this [TOML](https://github.com/toml-lang/toml) file:

```toml
# Get editor completions based on the config schema
"$schema" = 'https://starship.rs/config-schema.json'

# Inserts a blank line between shell prompts
add_newline = true

# Replace the '❯' symbol in the prompt with '➜'
[character] # The name of the module we are configuring is 'character'
success_symbol = '[➜](bold green)' # The 'success_symbol' segment is being set to '➜' with the color 'bold green'

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

### Config File Location

You can change default configuration file location with `STARSHIP_CONFIG` environment variable:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Logging

By default starship logs warnings and errors into a file named `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, where the session key is corresponding to an instance of your terminal.
This, however can be changed using the `STARSHIP_CACHE` environment variable:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

Equivalently in PowerShell (Windows) would be adding this line to your `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Terminology

**Module**: A component in the prompt giving information based on contextual information from your OS. For example, the "nodejs" module shows the version of Node.js that is currently installed on your computer, if your current directory is a Node.js project.

**Variable**: Smaller sub-components that contain information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of Node.js.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Strings

In TOML syntax, [text values](https://toml.io/en/v1.0.0#string) are declared with `'`, `"`, `'''`, or `"""`.

The following Starship syntax symbols have special usage in a format string and must be escaped to display as that character: `$ [ ] ( )`.

| Symbol | Type                      | Notes                                                  |
| ------ | ------------------------- | ------------------------------------------------------ |
| `'`    | literal string            | less escaping                                          |
| `"`    | string                    | more escaping                                          |
| `'''`  | multi-line literal string | less escaping                                          |
| `"""`  | multi-line string         | more escaping, newlines in declarations can be ignored |

For example:

```toml
# literal string
format = '☺\☻ '

# regular string
format = "☺\\☻ "

# escaping Starship symbols
format = '\[\$\] '
```

When using line breaks, multi-line declarations can be used.
For example, if you want to print a `$` symbol on a new line, the following values for `format` are equivalent:

```toml
# with literal string
format = '''

\$'''

# with multiline basic string
format = """

\\$"""

# with basic string
format = "\n\\$"
```

In multiline basic strings, newlines can be used for formatting without being present in the value by escaping them.

```toml
format = """
line1\
line1\
line1
line2\
line2\
line2
"""
```

### Format Strings

Format strings are the format that a module prints all its variables with.
Most modules have an entry called `format` that configures the display format of the module.
You can use texts, variables and text groups in a format string.

#### Variable

A variable contains a `$` symbol followed by the name of the variable.
The name of a variable can only contain letters, numbers and `_`.

For example:

- `'$version'` is a format string with a variable named `version`.
- `'$git_branch$git_commit'` is a format string with two variables named `git_branch` and `git_commit`.
- `'$git_branch $git_commit'` has the two variables separated with a space.

#### Text Group

A text group is made up of two different parts.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings).
You can add texts, variables, or even nested text groups in it.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). This can be used to style the first part.

For example:

- `'[on](red bold)'` will print a string `on` with bold text colored red.
- `'[⌘ $version](bold green)'` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `'[a [b](red) c](green)'` will print `a b c` with `b` red, and `a` and `c` green.

#### Style Strings

Most modules in starship allow you to configure their display styles. This is done with an entry (usually called `style`) which is a string specifying the configuration. Here are some examples of style strings along with what they do. For details on the full syntax, consult the [advanced config guide](../advanced-config/).

- `'fg:green bg:blue'` sets green text on a blue background
- `'bg:blue fg:bright-green'` sets bright green text on a blue background
- `'bold fg:27'` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `'underline bg:#bf5700'` sets underlined text on a burnt orange background
- `'bold italic fg:purple'` sets bold italic purple text
- `''` explicitly disables all styling

Note that what styling looks like will be controlled by your terminal emulator. For example, some terminal emulators will brighten the colors instead of bolding text, and some color themes use the same values for the normal and bright colors. Also, to get italic text, your terminal must support italics.

#### Conditional Format Strings

A conditional format string wrapped in `(` and `)` will not render if all variables inside are empty.

For example:

- `'(@$region)'` will show nothing if the variable `region` is `None` or empty string, otherwise `@` followed by the value of region.
- `'(some text)'` will always show nothing since there are no variables wrapped in the braces.
- When `$combined` is a shortcut for `\[$a$b\]`, `'($combined)'` will show nothing only if `$a` and `$b` are both `None`.
  This works the same as `'(\[$a$b\] )'`.

### Negative matching

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. These take
lists of strings to match or not match. "Negative" options, those which should not be matched, are
indicated with a leading '!' character. The presence of _any_ negative indicator in the directory
will result in the module not being matched.

Extensions are matched against both the characters after the last dot in a filename, and the
characters after the first dot in a filename. For example, `foo.bar.tar.gz` will be matched
against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Files whose name begins with a
dot are not considered to have extensions at all.

To see how this works in practice, you could match TypeScript but not MPEG Transport Stream files thus:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Modules

### Prompt Configuration

- [Prompt](modules/prompt)

### Core & System

- [Character](modules/core/character)
- [Directory](modules/core/directory)
- [Fill](modules/core/fill)
- [Line Break](modules/core/line_break)
- [Shell](modules/core/shell)
- [SHLVL](modules/core/shlvl)
- [Status](modules/core/status)
- [Sudo](modules/core/sudo)
- [Time](modules/core/time)
- [Username](modules/core/username)

### Version Control

- [Fossil Branch](modules/vcs/fossil_branch)
- [Fossil Metrics](modules/vcs/fossil_metrics)
- [Git Branch](modules/vcs/git_branch)
- [Git Commit](modules/vcs/git_commit)
- [Git State](modules/vcs/git_state)
- [Git Metrics](modules/vcs/git_metrics)
- [Git Status](modules/vcs/git_status)
- [Mercurial Branch](modules/vcs/hg_branch)
- [Mercurial State](modules/vcs/hg_state)
- [Pijul Channel](modules/vcs/pijul_channel)
- [VCS](modules/vcs/vcs)
- [VCSH](modules/vcs/vcsh)

### Programming Languages

- [Bun](modules/languages/bun)
- [C](modules/languages/c)
- [COBOL / GNUCOBOL](modules/languages/cobol)
- [CPP](modules/languages/cpp)
- [Crystal](modules/languages/crystal)
- [Daml](modules/languages/daml)
- [Dart](modules/languages/dart)
- [Deno](modules/languages/deno)
- [Dotnet](modules/languages/dotnet)
- [Elixir](modules/languages/elixir)
- [Elm](modules/languages/elm)
- [Erlang](modules/languages/erlang)
- [Fennel](modules/languages/fennel)
- [Fortran](modules/languages/fortran)
- [Gleam](modules/languages/gleam)
- [Go](modules/languages/golang)
- [Haskell](modules/languages/haskell)
- [Haxe](modules/languages/haxe)
- [Java](modules/languages/java)
- [Julia](modules/languages/julia)
- [Kotlin](modules/languages/kotlin)
- [Lua](modules/languages/lua)
- [Maven](modules/languages/maven)
- [Mojo](modules/languages/mojo)
- [Nim](modules/languages/nim)
- [Node.js](modules/languages/nodejs)
- [OCaml](modules/languages/ocaml)
- [Odin](modules/languages/odin)
- [Perl](modules/languages/perl)
- [PHP](modules/languages/php)
- [PureScript](modules/languages/purescript)
- [Python](modules/languages/python)
- [Quarto](modules/languages/quarto)
- [R](modules/languages/rlang)
- [Raku](modules/languages/raku)
- [Red](modules/languages/red)
- [Ruby](modules/languages/ruby)
- [Rust](modules/languages/rust)
- [Scala](modules/languages/scala)
- [Solidity](modules/languages/solidity)
- [Swift](modules/languages/swift)
- [Typst](modules/languages/typst)
- [V](modules/languages/vlang)
- [Zig](modules/languages/zig)

### Cloud & Container

- [AWS](modules/cloud/aws)
- [Azure](modules/cloud/azure)
- [Container](modules/cloud/container)
- [Docker Context](modules/cloud/docker_context)
- [Google Cloud (`gcloud`)](modules/cloud/gcloud)
- [Kubernetes](modules/cloud/kubernetes)
- [OpenStack](modules/cloud/openstack)
- [Singularity](modules/cloud/singularity)

### Package & Environment

- [Conda](modules/environment/conda)
- [Guix-shell](modules/environment/guix_shell)
- [Mise](modules/environment/mise)
- [Nix-shell](modules/environment/nix_shell)
- [Package Version](modules/environment/package)
- [Pixi](modules/environment/pixi)
- [Spack](modules/environment/spack)

### Build & Tools

- [Buf](modules/build_tools/buf)
- [CMake](modules/build_tools/cmake)
- [Gradle](modules/build_tools/gradle)
- [Helm](modules/build_tools/helm)
- [Meson](modules/build_tools/meson)
- [Open Policy Agent](modules/build_tools/opa)
- [Pulumi](modules/build_tools/pulumi)
- [Terraform](modules/build_tools/terraform)
- [XMake](modules/build_tools/xmake)

### System Info & Status

- [Battery](modules/info/battery)
- [Command Duration](modules/info/cmd_duration)
- [Direnv](modules/info/direnv)
- [Hostname](modules/info/hostname)
- [Jobs](modules/info/jobs)
- [Local IP](modules/info/localip)
- [Memory Usage](modules/info/memory_usage)
- [NATS](modules/info/nats)
- [Network Namespace](modules/info/network_namespace)
- [OS](modules/info/os)
- [Vagrant](modules/info/vagrant)

### Custom

- [Custom commands](modules/custom/custom)
- [Environment Variable](modules/custom/env_var)
