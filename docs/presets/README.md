# Presets

Here is a collection of community-submitted configuration presets for Starship.
If you have a preset to share, please [submit a PR](https://github.com/starship/starship/edit/master/docs/presets/README.md) updating this file! 😊

## Starship Config

command_timeout = 10_000
## FIRST LINE/ROW: Info & Status
## First param ─┌

[username]
format = " [ ╭─$user]($style)"
show_always = true
style_root = "bold red"
style_user = "bold red"

# Second param
[hostname]
disabled = false
format = " <<<[$hostname]($style)>>> [In Directory [-->](blue)](bold dimmed blue) "
ssh_only = false
style = "bold dimmed red"
trim_at = "-"

# Third param
[directory]
format = "| [($path)]($style)($read_only) | on [ master ](bold purple)" 
truncate_to_repo = true
truncation_length = 10
truncation_symbol = "/"
read_only = "🛡️"
home_symbol = "~/ "
style = "#FFAA00"
disabled = false 

[git_branch]
always_show_remote = true 
format = ': [($remote_branch)]($style) '
symbol = ' '
style = 'blink bold dimmed purple'
truncation_length = 10
truncation_symbol = '...'
disabled = false 

[memory_usage]
format = "<< [Ram : ${ram}(, Usage : ${ram_pct})]($style) >> "
threshold = 10
style = "bold dimmed green"
disabled = false

# Fourth param
[sudo]
disabled = true

[git_commit]
commit_hash_length = 8
style = "bold white"

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '

[git_status]
ahead = "⇡${count}"
behind = "⇣${count}"
deleted = "x"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
style = "white"

# Last param in the first line/row
[cmd_duration]
disabled = false
format = " [<--](#FF0059) [This action took](bold #00FF27) [$duration]($style)"
min_time = 0
style = "bold #FF2E00"

## SECOND LINE/ROW: Prompt

# Somewhere at the beginning
[battery]
full_symbol = " "
charging_symbol = " "
discharging_symbol = " "
format = " $[full_symbol$charging_symbol$discharging_symbol$percentage] "
disabled = true 

[[battery.display]]  # "bold red" style when capacity is between 0% and 10%
disabled = false
style = "bold red"
threshold = 10

[[battery.display]]  # "bold yellow" style when capacity is between 10% and 30%
disabled = false
style = "bold yellow"
threshold = 50

[[battery.display]]  # "bold green" style when capacity is between 10% and 30%
disabled = false
style = "bold green"
threshold = 101

# Prompt: optional param 1
[time]
disabled = true
format = " 🕙 $time($style)\n"
style = "bright-white"
time_format = "%T"

# Prompt: param 2
[character]
error_symbol = " [δ-->](bold red)"
success_symbol = " [\\[╰─∫\\]](bold red) [Φ-->](bold red)"

# SYMBOLS
[status]
disabled = false 
format = ' [\[$symbol$status_common_meaning$status_signal_name$status_maybe_integer\]]($style)'
map_symbol = true
pipestatus = true
symbol = "╰─ζ"
style = "bold red"

[aws]
symbol = " "

[conda]
symbol = " "

[dart]
symbol = " "

[docker_context]
symbol = " "

[elixir]
symbol = " "

[elm]
symbol = " "

[golang]
symbol = " "

[java]
symbol = " "

[julia]
symbol = " "

[nim]
symbol = " "

[nix_shell]
symbol = " "

[nodejs]
symbol = " "

[package]
symbol = " "
disabled = false

[perl]
symbol = " "

[php]
symbol = " "

[ruby]
symbol = " "

[rust]
symbol = "Rust "

[swift]
symbol = "ﯣ "

[python]
symbol = " "

# You can edit this file to this liking.....
