# Paramètres par défaut

Voici un ensemble de configurations soumises par la communauté pour Starship. Si vous avez une configuration à partager, veuillez [proposer une PR](https://github.com/starship/starship/edit/master/docs/presets/README.md) pour mettre à jour ce fichier! 😊

## Symboles Nerd Font

Cette configuration ne modifie rien sauf les symboles utilisés pour chaque module. Si les émoticônes ne sont pas votre tasse de thé, elle pourrait retenir votre attention!

![Capture d'écran du pré-réglage Nerd Font Symbols](/presets/nerd-font-symbols.png)

### Pré-requis

- Une [Nerd Font](https://www.nerdfonts.com/) est installée et activée dans votre terminal (l'exemple utilise Fira Code Nerd Font)

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

## Segments entre crochets

Cette configuration modifie le format de tous les modules intégrés pour afficher leur segment entre crochets au lieu d'utiliser le label par défaut de Starship ("via", "sur", etc.).

Avant:

![Capture d'écran de la configuration par défaut de Starship](/presets/bracketed-segments-before.png)

Après:

![Capture d'écran de la configuration Segments entre crochets](/presets/bracketed-segments-after.png)

### Configuration

```toml
[aws]
format = '\[[$symbol($profile)(\($region\))(\[$duration\])]($style)\]'

[cmake]
format = '\[[$symbol($version)]($style)\]'

[cmd_duration]
format = "[⏱ $duration]($style)"

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

## Texte uniquement

Cette configuration change les symboles en texte brut. Si votre terminal/police ne peuvent pas afficher les émoticônes ou les symboles NerdFonts, vous pouvez peut-être essayer cette configuration !

Avant (réglage par défaut avec la police Fixedsys):

![Capture d'écran de la configuration Starship par défaut avec la police Fixedsys](/presets/plain-text-symbols-before.png)

Après (Texte uniquement):

![Capture d'écran de la configuration Texte uniquement](/presets/plain-text-symbols-after.png)

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

[swift]
symbol = "swift "
```
