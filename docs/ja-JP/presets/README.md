# 準備するもの

以下は、Starship用にコミュニティに提供された構成プリセットのコレクションです。 共有するプリセットがある場合は、このファイルを更新して[PR](https://github.com/starship/starship/edit/master/docs/presets/README.md)を送信してください！ 😊

## Nerd Font Symbols

このプリセットは、各モジュールに使用されるシンボル以外は何も変更しません。 絵文字が気に食わない場合、これはあなたの目を引くかもしれません！

![Nerd Font Symbolsプリセットのスクリーンショット](/presets/nerd-font-symbols.png)

### 必要なもの

- 端末にインストールされ、有効になっている[Nerd Font](https://www.nerdfonts.com/)（例ではFira Code Nerd Fontを使用)

### 設定

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

このプリセットは付属のモジュール全てのフォーマットを変更し、デフォルトのStarshipの表示("via", "on", etc.)から、それぞれの情報を括弧で区分けするように変更します。

変更前:

![Starshipのデフォルト設定のスクリーンショット](/presets/bracketed-segments-before.png)

変更後:

![Bracketed Segmentsプリセットのスクリーンショット](/presets/bracketed-segments-after.png)

### 設定

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

このプリセットは記号をプレーンテキストに変更します。 あなたのターミナルのフォントでNerdFontsまたは絵文字が使えない場合は、これを試してみるといいかもしれません！

変更前 (デフォルトの設定でFixedsysフォント):

![Fixedsysフォントを用いたStarshipのデフォルト設定のスクリーンショット](/presets/plain-text-symbols-before.png)

変更後 (プレーンテキストの記号):

![Plain Text Symbolsプリセットのスクリーンショット](/presets/plain-text-symbols-after.png)

### 設定

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

### 設定

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

### 設定

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
