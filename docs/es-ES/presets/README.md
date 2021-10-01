# Ajustes por defecto

Aquí hay una colección de ajustes por defecto creados por la comunidad de Starship. ¡Si quieres compartir un ajuste por defecto, por favor, [envía un PR](https://github.com/starship/starship/edit/master/docs/presets/README.md) actualizando este archivo! 😊

## Símbolos de Nerd Font

Este ajuste por defecto no modifica nada excepto los símbolos usados para cada módulo. Si los emojis no son lo tuyo, ¡esto podría llamar tu atención!

![Captura de pantalla de los ajustes por defecto de los Símbolos de Nerd Font](/presets/nerd-font-symbols.png)

### Prerequisitos

- Una [Nerd Font](https://www.nerdfonts.com/) instalada y habilitada en tu terminal (el ejemplo usa Fira Code Nerd Font)

### Configuración

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

## Segmentos entre Corchetes

Este ajuste cambia el formato de todos los módulos incorporados para mostrar su segmento entre corchetes en lugar de usar la redacción predeterminada de Starship ("via", "on", etc.).

Antes:

![Captura de pantalla de la configuración por defecto de Starship](/presets/bracketed-segments-before.png)

Después:

![Captura de pantalla de los ajustes predeterminados de los Segmentos entre Corchetes](/presets/bracketed-segments-after.png)

### Configuración

```toml
[aws]
format = '\[[$symbol($profile)(\($region\))(\[$duration\])]($style)\]'

[cmake]
format = '\[[$symbol($version)]($style)\]'

[cmd_duration]
format = '\[[⏱ $duration ]($style)\]'

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

## Símbolos de texto sin formato

Este ajuste preestablecido cambia los símbolos a texto sin formato. Si tu terminal/fuente no pudo renderizar el NerdFonts/emojis, ¡tal vez podría probar este ajuste preestablecido!

Antes (configuración por defecto con Fuente Fixedsys):

![Captura de pantalla de configuración por defecto de Starship con fuente Fixedsys](/presets/plain-text-symbols-before.png)

Después (Símbolos de texto sin formato):

![Captura de pantalla de los ajustes predeterminados de los Símbolos de Nerd Font](/presets/plain-text-symbols-after.png)

### Configuración

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

## Ocultar versiones en tiempo de ejecución

Esta configuración predeterminada oculta la versión de los tiempos de ejecución del idioma. Si trabajas en contenedores o entornos virtualizados, ¡ésto es para ti!

![Captura de pantalla ajustes de Ocultar versiones de tiempo de ejecución](/presets/hide-runtime-versions.png)

### Configuración

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

### Configuración

```toml
format = """
$username\
$hostname\
$directory\
$git_branch\
$git_state\
$git_status\
$cmd_duration\
$line_break\
$python\
$character"""

[directory]
style = "blue"

[character]
success_symbol = "[❯](purple)"
error_symbol = "[❯](red)"
vicmd_symbol = "[❮](green)"

[git_branch]
format = "[$branch]($style)"
style = "bright-black"

[git_status]
format = "[[(*$conflicted$untracked$modified$staged$renamed$deleted)](218) ($ahead_behind$stashed)]($style)"
style = "cyan"
conflicted = "​"
untracked = "​"
modified = "​"
staged = "​"
renamed = "​"
deleted = "​"
stashed = "≡"

[git_state]
format = '\([$state( $progress_current/$progress_total)]($style)\) '
style = "bright-black"

[cmd_duration]
format = "[$duration]($style) "
style = "yellow"

[python]
format = "[$virtualenv]($style) "
style = "bright-black"
```
