# Clean Text

A two-line prompt designed for environments where **Nerd Fonts or patched icon fonts are not available**—such as stock Windows Terminal, remote SSH sessions, minimal Linux servers, or CI/CD containers.

It keeps full runtime visibility and Git tracking using standard ASCII character labels instead of unicode symbols.

## Preview

```text
pwsh in ~/projects/backend on git:main took 2s [ERR 1]
via Node v20.11.0 via Py v3.12.1 ❯
```

## Configuration

Add the following to your `~/.config/starship.toml`:

```toml
# Starship Configuration - Clean Text (No Nerd Fonts Required)
"$schema" = '[https://starship.rs/config-schema.json](https://starship.rs/config-schema.json)'

add_newline = true

format = """
$shell$directory$git_branch$git_status$git_state$cmd_duration$status
$dotnet$nodejs$python$golang$rust$java$docker_context$character"""

[shell]
disabled = false
powershell_indicator = 'pwsh'
style = 'bold blue'
format = '[$indicator]($style) '

[directory]
truncation_length = 4
truncation_symbol = '.../'
home_symbol = '~'
style = 'bold cyan'
read_only = ' [RO]'
format = 'in [$path]($style)[$read_only]($read_only_style) '

[git_branch]
symbol = 'git:'
style = 'bold purple'
format = 'on [$symbol$branch]($style) '

[git_status]
style = 'bold red'
format = '([$all_status$ahead_behind]($style) )'
conflicted = '='
ahead = '>'
behind = '<'
diverged = '<>'
untracked = '?'
stashed = '$'
modified = '!'
staged = '+'
renamed = 'r'
deleted = 'x'

[cmd_duration]
min_time = 2_000
style = 'bold yellow'
format = 'took [$duration]($style) '

[status]
disabled = false
symbol = '[ERR]'
style = 'bold red'
format = '[$symbol $code]($style) '

[dotnet]
symbol = '.NET '
style = 'bold blue'
format = 'via [$symbol$version]($style) '

[nodejs]
symbol = 'Node '
style = 'bold green'
format = 'via [$symbol$version]($style) '

[python]
symbol = 'Py '
style = 'bold yellow'
format = 'via [$symbol$version]($style) '

[golang]
symbol = 'Go '
style = 'bold cyan'
format = 'via [$symbol$version]($style) '

[rust]
symbol = 'Rust '
style = 'bold red'
format = 'via [$symbol$version]($style) '

[java]
symbol = 'Java '
style = 'bold red'
format = 'via [$symbol$version]($style) '

[docker_context]
symbol = 'Docker '
style = 'bold blue'
format = 'on [$symbol$context]($style) '

[character]
success_symbol = '[❯](bold green)'
error_symbol = '[❯](bold red)'
vimcmd_symbol = '[❮](bold green)'
```