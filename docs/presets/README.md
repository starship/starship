# Presets

Here is a collection of community-submitted configuration presets for Starship.
If you have a preset to share, please [submit a PR](https://github.com/starship/starship/edit/master/docs/presets/README.md) updating this file! 😊

## Nerd Font Symbols

This preset doesn't change anything except for the symbols used for each module.
If emojis aren't your thing, this might catch your eye!

![Screenshot of Nerd Font Symbols preset](/presets/nerd-font-symbols.png)

### Prerequisites

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal (the example uses Fira Code Nerd Font)

### Configuration

```toml
[aws]
symbol = "  "

[conda]
symbol = " "

[dart]
symbol = " "

[directory]
read_only = " "

[docker_context]
symbol = " "

[elixir]
symbol = " "

[elm]
symbol = " "

[git_branch]
symbol = " "

[golang]
symbol = " "

[hg_branch]
symbol = " "

[java]
symbol = " "

[julia]
symbol = " "

[memory_usage]
symbol = " "

[nim]
symbol = " "

[nix_shell]
symbol = " "

[nodejs]
symbol = " "

[package]
symbol = " "

[perl]
symbol = " "

[php]
symbol = " "

[python]
symbol = " "

[ruby]
symbol = " "

[rust]
symbol = " "

[scala]
symbol = " "

[shlvl]
symbol = " "

[swift]
symbol = "ﯣ "
```

## Bracketed Segments

This preset changes the format of all the built-in modules to show their segment
in brackets instead of using the default Starship wording ("via", "on", etc.).

Before:

![Screenshot of default Starship configuration](/presets/bracketed-segments-before.png)

After:

![Screenshot of Bracketed Segments preset](/presets/bracketed-segments-after.png)

### Configuration

```toml
[aws]
format = '\[[$symbol($profile)(\($region\))(\[$duration\])]($style)\]'

[cmake]
format = '\[[$symbol($version)]($style)\]'

[cmd_duration]
format = '\[[⏱ $duration]($style)\]'

[cobol]
format = '\[[$symbol($version)]($style)\]'

[conda]
format = '\[[$symbol$environment]($style)\]'

[crystal]
format = '\[[$symbol($version)]($style)\]'

[dart]
format = '\[[$symbol($version)]($style)\]'

[deno]
format = '\[[$symbol($version)]($style)\]'

[docker_context]
format = '\[[$symbol$context]($style)\]'

[dotnet]
format = '\[[$symbol($version)(🎯 $tfm)]($style)\]'

[elixir]
format = '\[[$symbol($version \(OTP $otp_version\))]($style)\]'

[elm]
format = '\[[$symbol($version)]($style)\]'

[erlang]
format = '\[[$symbol($version)]($style)\]'

[gcloud]
format = '\[[$symbol$account(@$domain)(\($region\))]($style)\]'

[git_branch]
format = '\[[$symbol$branch]($style)\]'

[git_status]
format = '([\[$all_status$ahead_behind\]]($style))'

[golang]
format = '\[[$symbol($version)]($style)\]'

[helm]
format = '\[[$symbol($version)]($style)\]'

[hg_branch]
format = '\[[$symbol$branch]($style)\]'

[java]
format = '\[[$symbol($version)]($style)\]'

[julia]
format = '\[[$symbol($version)]($style)\]'

[kotlin]
format = '\[[$symbol($version)]($style)\]'

[kubernetes]
format = '\[[$symbol$context( \($namespace\))]($style)\]'

[lua]
format = '\[[$symbol($version)]($style)\]'

[memory_usage]
format = '\[$symbol[$ram( | $swap)]($style)\]'

[nim]
format = '\[[$symbol($version)]($style)\]'

[nix_shell]
format = '\[[$symbol$state( \($name\))]($style)\]'

[nodejs]
format = '\[[$symbol($version)]($style)\]'

[ocaml]
format = '\[[$symbol($version)(\($switch_indicator$switch_name\))]($style)\]'

[openstack]
format = '\[[$symbol$cloud(\($project\))]($style)\]'

[package]
format = '\[[$symbol$version]($style)\]'

[perl]
format = '\[[$symbol($version)]($style)\]'

[php]
format = '\[[$symbol($version)]($style)\]'

[pulumi]
format = '\[[$symbol$stack]($style)\]'

[purescript]
format = '\[[$symbol($version)]($style)\]'

[python]
format = '\[[${symbol}${pyenv_prefix}(${version})(\($virtualenv\))]($style)\]'

[red]
format = '\[[$symbol($version)]($style)\]'

[ruby]
format = '\[[$symbol($version)]($style)\]'

[rust]
format = '\[[$symbol($version)]($style)\]'

[scala]
format = '\[[$symbol($version)]($style)\]'

[sudo]
format = '\[[as $symbol]\]'

[swift]
format = '\[[$symbol($version)]($style)\]'

[terraform]
format = '\[[$symbol$workspace]($style)\]'

[time]
format = '\[[$time]($style)\]'

[username]
format = '\[[$user]($style)\]'

[vagrant]
format = '\[[$symbol($version)]($style)\]'

[vlang]
format = '\[[$symbol($version)]($style)\]'

[zig]
format = '\[[$symbol($version)]($style)\]'
```

## Plain Text Symbols

This preset changes the symbols into plain text.
If your terminal/font could not render the NerdFonts/emojis, maybe you could try this preset!

Before (default setting with Fixedsys font):

![Screenshot of default Starship configuration with Fixedsys font](/presets/plain-text-symbols-before.png)

After (Plain Text Symbols):

![Screenshot of Plain Text Symbols preset](/presets/plain-text-symbols-after.png)

### Configuration

```toml
[character]
success_symbol = "[>](bold green)"
error_symbol = "[x](bold red)"
vicmd_symbol = "[<](bold green)"

[git_commit]
tag_symbol = " tag "

[git_status]
ahead = ">"
behind = "<"
diverged = "<>"
renamed = "r"
deleted = "x"

[aws]
symbol = "aws "

[cobol]
symbol = "cobol "

[conda]
symbol = "conda "

[crystal]
symbol = "cr "

[cmake]
symbol = "cmake "

[dart]
symbol = "dart "

[deno]
symbol = "deno "

[dotnet]
symbol = ".NET "

[directory]
read_only = " ro"

[docker_context]
symbol = "docker "

[elixir]
symbol = "exs "

[elm]
symbol = "elm "

[git_branch]
symbol = "git "

[golang]
symbol = "go "

[hg_branch]
symbol = "hg "

[java]
symbol = "java "

[julia]
symbol = "jl "

[kotlin]
symbol = "kt "

[nodejs]
symbol = "nodejs "

[memory_usage]
symbol = "memory "

[nim]
symbol = "nim "

[nix_shell]
symbol = "nix "

[ocaml]
symbol = "ml "

[package]
symbol = "pkg "

[perl]
symbol = "pl "

[php]
symbol = "php "

[pulumi]
symbol = "pulumi "

[purescript]
symbol = "purs "

[python]
symbol = "py "

[ruby]
symbol = "rb "

[rust]
symbol = "rs "

[scala]
symbol = "scala "

[sudo]
symbol = "sudo "

[swift]
symbol = "swift "
```

## Hide Runtime Versions

This preset hides the version of language runtimes. If you work in containers or virtualized environments, this one is for you!

![Screenshot of Hide Runtime Versions preset](/presets/hide-runtime-versions.png)

### Configuration

```toml
[cmake]
format = "via [$symbol]($style)"

[cobol]
format = "via [$symbol]($style)"

[crystal]
format = "via [$symbol]($style)"

[dart]
format = "via [$symbol]($style)"

[deno]
format = "via [$symbol]($style)"

[dotnet]
format = "[$symbol(🎯 $tfm )]($style)"

[elixir]
format = 'via [$symbol]($style)'

[elm]
format = 'via [$symbol]($style)'

[erlang]
format = 'via [$symbol]($style)'

[golang]
format = 'via [$symbol]($style)'

[helm]
format = 'via [$symbol]($style)'

[julia]
format = 'via [$symbol]($style)'

[kotlin]
format = 'via [$symbol]($style)'

[lua]
format = 'via [$symbol]($style)'

[nim]
format = 'via [$symbol]($style)'

[nodejs]
format = 'via [$symbol]($style)'

[ocaml]
format = 'via [$symbol(\($switch_indicator$switch_name\) )]($style)'

[perl]
format = 'via [$symbol]($style)'

[php]
format = 'via [$symbol]($style)'

[pulumi]
format = 'via [$symbol$stack]($style)'

[purescript]
format = 'via [$symbol]($style)'

[red]
format = 'via [$symbol]($style)'

[rlang]
format = 'via [$symbol]($style)'

[ruby]
format = 'via [$symbol]($style)'

[rust]
format = 'via [$symbol]($style)'

[swift]
format = 'via [$symbol]($style)'

[vagrant]
format = 'via [$symbol]($style)'

[vlang]
format = 'via [$symbol]($style)'

[zig]
format = 'via [$symbol]($style)'
```

## Pure

This preset emulates the look and behavior of [Pure](https://github.com/sindresorhus/pure).

![Screenshot of Pure preset](/presets/pure-prompt.png)

### Configuration

```toml
format = """
[╭─](bold white)$hostname$kubernetes$directory$git_branch$git_commit$git_state$git_status$docker_context$package$golang$helm$java$cmake$julia$kotlin$lua$nim$nodejs$python$ruby$rust$swift$terraform$aws$gcloud$azure $battery               
[╰─❯](bold white)"""
 
right_format = """$cmd_duration"""

# Replace the "❯" symbol in the prompt with "➜"
character]·#·The·name·of·the·module·we·are·configuring·is·"character"
success_symbol·=·"[➜](bold·green)"·#·The·"success_symbol"·segment·is·being·set·to·"➜"·with·the·color·"bold·green"

[cmd_duration]
min_time = 0
show_milliseconds = true
format = '[$duration]($style)'
style = "bold yellow"

[java]
symbol = "☕"
format = '[$duration]($style)'
style = "bold yellow"

[lua]
style = "#7FFFD4"

[cmake]
symbol = "🌕"
format = "via [${symbol} (${version} )]($style)"

[kubernetes]
format = 'context: [⎈ $context \($namespace\)](bold cyan) '
disabled = false
# [kubernetes.context_aliases]
# "dev.local.cluster.k8s" = "dev"

[memory_usage]
format = "with$symbol [${ram} ${ram_pct}( | ${swap} ${swap_pct})]($style) "
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"

[gcloud]
style = "blue"
format = '[$symbol$account(\($project\))]($style) '
symbol = "️G⅁:☁️"
[gcloud.region_aliases]
us-central1 = "uc1"
asia-northeast1 = "an1"

[aws]
format = '[$symbol$profile(\($region\))]($style) '
style = "bold yellow"
symbol = "∀⍵₷☁️☁️ "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"

[package]
format = "on [🎁 $version](208 bold) "

[docker_context]
format = "docker: [🐋 $context](blue bold)"
disabled = false

[directory]
truncation_length = 7
truncation_symbol = "…/"

[username]
style_user = "blue bold"
style_root = "red bold"
format = "user: [$user]($style) "
disabled = false
show_always = true

[time]
disabled = false
format = '[\[ $time \]]($style) '

[hostname]
ssh_only = false
format = '[$hostname](red) '
trim_at = "."
disabled = false

[status]
style = "red"
symbol = "💥 "
format = '[\[$symbol$status\]]($style) '
disabled = false

[git_branch]
always_show_remote = true
style = "bold blue"

[git_status]
ahead = "⇡🏎💨${count}"
diverged = "⇕⇡😵${ahead_count}⇣${behind_count}"
behind = "⇣😰${count}"
conflicted = "🏳 "
untracked = "🤷"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑 "

[rust]
format = "via [${symbol}(${version} )](208)($style)"

[golang]
format = "go: [🏎💨 $version](bold cyan) "

[helm]
format = "helm: [⎈ $version](bold white) "

[jobs]
symbol = "+ "
threshold = 4
format = "background [$symbol$number]($style) "

[terraform]
format = "[🏎💨 $version$workspace]($style) "

[battery]
full_symbol = "🔋"
charging_symbol = "🔌 "
discharging_symbol = "⚡️"

[[battery.display]] # "bold red" style when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]] # "bold yellow" style when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"

[[battery.display]]
threshold = 100
style = "bold green"
```
