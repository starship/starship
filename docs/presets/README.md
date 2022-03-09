a# Presets

Here is a collection of community-submitted configuration presets for Starship.
If you have a preset to share, please [submit a PR](https://github.com/starship/starship/edit/master/docs/presets/README.md) updating this file! 😊

To get details on how to use a preset, simply click on the image.

## [Nerd Font Symbols](./nerd-font)

This preset changes the symbols for each module to use Nerd Font symbols.

[![Screenshot of Nerd Font Symbols preset](/presets/img/nerd-font-symbols.png "Click to view Nerd Font Symbols preset")](./nerd-font)

- A [Nerd Font](https://www.nerdfonts.com/) installed and enabled in your terminal (the example uses Fira Code Nerd Font)

### Configuration

```toml
[aws]
symbol = "  "

[buf]
symbol = " "

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

[![Screenshot of Bracketed Segments preset](/presets/img/bracketed-segments.png "Click to view Bracketed Segments preset")](./bracketed-segments)

## [Plain Text Symbols](./plain-text)

This preset changes the symbols for each module into plain text. Great if you
don't have access to Unicode.

[![Screenshot of Plain Text Symbols preset](/presets/img/plain-text-symbols.png "Click to view Plain Text Symbols preset")](./plain-text)

### Configuration

```toml
[aws]
format = '\[[$symbol($profile)(\($region\))(\[$duration\])]($style)\]'

[buf]
format = '\[[$symbol($version)]($style)\]'

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

[![Screenshot of Hide Runtime Versions preset](/presets/img/no-runtime-versions.png "Click to view No Runtime Versions preset")](./no-runtimes)

## [Pure Prompt](./pure-preset)

This preset emulates the look and behavior of [Pure](https://github.com/sindresorhus/pure).

[![Screenshot of Pure preset](/presets/img/pure-preset.png "Click to view Pure Prompt preset")](./pure-preset)
