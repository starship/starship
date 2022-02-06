# Configuração

Para começar a configurar a starship, crie o seguinte arquivo: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Todas as configurações do starship são feitas neste arquivo [TOML](https://github.com/toml-lang/toml):

```toml
# Inserts a blank line between shell prompts
add_newline = true

# Replace the "❯" symbol in the prompt with "➜"
[character] # The name of the module we are configuring is "character"
success_symbol = "[➜](bold green)" # The "success_symbol" segment is being set to "➜" with the color "bold green"

# Disable the package module, hiding it from the prompt completely
[package]
disabled = true
```

Você pode alterar o caminho padrão do arquivo de configuração com a variável de ambiente `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

No PowerShell (Windows) você pode adicionar a seguinte linha no seu `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Logging

By default starship logs warnings and errors into a file named `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, where the session key is corresponding to a instance of your terminal. This, however can be changed using the `STARSHIP_CACHE` environment variable:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

No PowerShell (Windows) você pode adicionar a seguinte linha no seu `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Or for Cmd (Windows) would be adding this line to your `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Terminologia

**Module**: A component in the prompt giving information based on contextual information from your OS. For example, the "nodejs" module shows the version of Node.js that is currently installed on your computer, if your current directory is a Node.js project.

**Variable**: Smaller sub-components that contain information provided by the module. For example, the "version" variable in the "nodejs" module contains the current version of Node.js.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Formatação de Strings

Format strings are the format that a module prints all its variables with. Most modules have an entry called `format` that configures the display format of the module. You can use texts, variables and text groups in a format string.

#### Variável

A variable contains a `$` symbol followed by the name of the variable. The name of a variable can only contain letters, numbers and `_`.

Por exemplo:

- `$version` é uma formatação de string com uma variável chamada `version`.
- `$git_branch$git_commit` é uma formatação de string com duas variáveis chamadas `git_branch` e `git_commit`.
- `$git_branch $git_commit` Tem as duas variáveis separadas por espaço.

#### Grupo de Texto

A text group is made up of two different parts.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings). You can add texts, variables, or even nested text groups in it.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). This can be used to style the first part.

Por exemplo:

- `[on](red bold)` vai imprimir uma string `on` com texto em negrito e com a cor vermelha.
- `[⌘ $version](bold green)` vai imprimir o simbolo `⌘` seguido pela variável `version`, com o texto em negrito e na cor verde.
- `[a [b](red) c](green)` vai imprimir `a b c` com `b` vermelho, e `a` e `c` verde.

#### Estilo dos textos

Most modules in starship allow you to configure their display styles. This is done with an entry (usually called `style`) which is a string specifying the configuration. Here are some examples of style strings along with what they do. For details on the full syntax, consult the [advanced config guide](/advanced-config/).

- `"fg:green bg:blue"` deixa o texto verde com o fundo azul
- `"bg:blue fg:bright-green"` deixa o texto verde brilhante com o fundo azul
- `"bold fg:27"` deixa o texto em negrito com a cor 27 [da tabela ANSI](https://i.stack.imgur.com/KTSQa.png)
- `"underline bg:#bf5700"` deixa o texto sublinhado com o fundo laranja escuro
- `"bold italic fg:purple"` deixa o texto em negrito e itálico com a cor roxa
- `""` desabilita explicitamente todos os estilos

Note that what styling looks like will be controlled by your terminal emulator. For example, some terminal emulators will brighten the colors instead of bolding text, and some color themes use the same values for the normal and bright colors. Also, to get italic text, your terminal must support italics.

#### Formatação de String Condicional

A conditional format string wrapped in `(` and `)` will not render if all variables inside are empty.

Por exemplo:

- `(@$region)` não vai exibir nada caso a variável `region` seja `None` ou vazia, caso contrario vai exibir `@` seguido pelo valor da variável region.
- `(texto qualquer)` não vai exibir nada sempre, pois não existe variável entre os parenteses.
- Quando usar `$all` é um atalho para `\[$a$b\]`, `($all)` vai exibir nada somente quando `$a` e `$b` são `None`. Isto funciona da mesma forma que `(\[$a$b\] )`.

#### Caracteres Especiais

The following symbols have special usage in a format string and must be escaped: `$ \ [ ] ( )`.

Note that TOML has [both basic strings and literal strings](https://toml.io/en/v1.0.0#string). It is recommended to use a literal string (surrounded by single quotes) in your config. If you want to use a basic string (surrounded by double quotes), you must escape the backslash itself (i.e. use `\\`).

For example, when you want to print a `$` symbol on a new line, the following configs for `format` are equivalent:

```toml
# with basic string
format = "\n\\$"

# with multiline basic string
format = """

\\$"""

# with literal string
format = '''

\$'''
```

## Prompt de Comando

This is the list of prompt-wide configuration options.

### Opções

| Opções            | Padrão                         | Descrição                                                              |
| ----------------- | ------------------------------ | ---------------------------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | Configura o formato do prompt.                                         |
| `right_format`    | `""`                           | Veja [Ativa o prompt direito](/advanced-config/#enable-right-prompt)   |
| `scan_timeout`    | `30`                           | Tempo limite para escanear arquivos (em milissegundos).                |
| `command_timeout` | `500`                          | Tempo limite de execução de comandos pelo starship (em milissegundos). |
| `add_newline`     | `true`                         | Insere linha vazia entre os prompts do shell.                          |

### Exemplo

```toml
# ~/.config/starship.toml

# Use custom format
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

# Espera 10 milliseconds para que o starship verifique os arquivos no diretório atual.
scan_timeout = 10

# Remove a quebra de linha no início do prompt
add_newline = false
```

### Format de Prompt Padrão

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. The default is as shown:

```toml
format = "$all"

# Equivalente a
format = """
$username\
$hostname\
$localip\
$shlvl\
$singularity\
$kubernetes\
$directory\
$vcsh\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$docker_context\
$package\
$cmake\
$cobol\
$container\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$helm\
$java\
$julia\
$kotlin\
$lua\
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$pulumi\
$purescript\
$python\
$rlang\
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vlang\
$vagrant\
$zig\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
$env_var\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$shell\
$character"""
```

If you just want to extend the default format, you can use `$all`; modules you explicitly add to the format will not be duplicated. Ex.

```toml
# Move o diretório para a segunda linha
format = "$all$directory$character"
```

## AWS

The `aws` module shows the current AWS region and profile. This is based on `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env var with `~/.aws/config` file. This module also shows an expiration timer when using temporary credentials.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile is read from the `AWS_VAULT` env var and the credentials expiration date is read from the `AWS_SESSION_EXPIRATION` env var.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile is read from the `AWSUME_PROFILE` env var and the credentials expiration date is read from the `AWSUME_EXPIRATION` env var.

### Opções

| Opções              | Padrão                                                               | Descrição                                                            |
| ------------------- | -------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\])]($style)'` | O formato do módulo.                                                 |
| `symbol`            | `"☁️ "`                                                              | O símbolo usado antes de exibir o perfil atual da AWS.               |
| `region_aliases`    |                                                                      | Tabela de aleases de regiões a serem exibidas, além do nome da AWS.  |
| `style`             | `"bold yellow"`                                                      | O estilo do módulo.                                                  |
| `expiration_symbol` | `X`                                                                  | O simbolo exibido quando as credenciais temporárias estão expiradas. |
| `disabled`          | `false`                                                              | Desabilita o modulo `AWS`.                                           |

### Variáveis

| Variável  | Exemplo          | Descrição                            |
| --------- | ---------------- | ------------------------------------ |
| region    | `ap-northeast-1` | A região atual do AWS                |
| profile   | `astronauts`     | O perfil atual do AWS                |
| duration  | `2h27m20s`       | A duração temporária das credenciais |
| symbol    |                  | Espelha o valor da opção `símbolo`   |
| style\* |                  | Espelha o valor da opção `style`     |

*: This variable can only be used as a part of a style string

### Exemplos

#### Exibir tudo

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Exibir região

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$region]($style) "
style = "bold blue"
symbol = "🅰 "
[aws.region_aliases]
ap-southeast-2 = "au"
us-east-1 = "va"
```

#### Exibir perfil

```toml
# ~/.config/starship.toml

[aws]
format = "on [$symbol$profile]($style) "
style = "bold blue"
symbol = "🅰 "
```

## Azure

The `azure` module shows the current Azure Subscription. This is based on showing the name of the default subscription, as defined in the `~/.azure/azureProfile.json` file.

### Opções

| Variável   | Padrão                                   | Descrição                                      |
| ---------- | ---------------------------------------- | ---------------------------------------------- |
| `format`   | `"on [$symbol($subscription)]($style) "` | O formato que o módulo Azure será renderizado. |
| `symbol`   | `"ﴃ "`                                   | O símbolo usado no formato.                    |
| `style`    | `"blue bold"`                            | O estilo usado no formato.                     |
| `disabled` | `true`                                   | Desabilita o módulo `azure`.                   |

### Exemplo

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($subscription)]($style) "
symbol = "ﴃ "
style = "blue bold"
```

## Bateria

The `battery` module shows how charged the device's battery is and its current charging status. The module is only visible when the device's battery is below 10%.

### Opções

| Opções               | Padrão                            | Descrição                                                    |
| -------------------- | --------------------------------- | ------------------------------------------------------------ |
| `full_symbol`        | `" "`                            | O simbolo exibido quando a bateria estiver cheia.            |
| `charging_symbol`    | `" "`                            | O simbolo exibido quando a bateria está carregando.          |
| `discharging_symbol` | `" "`                            | O simbolo exibido quando a bateria está descarregando.       |
| `unknown_symbol`     | `" "`                            | O simbolo exibido quando o estado da bateria é desconhecido. |
| `empty_symbol`       | `" "`                            | O simbolo exibido quando o estado da bateria é vazio.        |
| `format`             | `"[$symbol$percentage]($style) "` | O formato do módulo.                                         |
| `display`            | [link](#battery-display)          | Limite de exibição e estilo para o módulo.                   |
| `disabled`           | `false`                           | Desabilita o módulo `battery`.                               |

### Exemplo

```toml
# ~/.config/starship.toml

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "
```

### Indicador de bateria

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style). If no `display` is provided. The default is as shown:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Opções

The `display` option is an array of the following table.

| Opções               | Padrão     | Descrição                                                                                          |
| -------------------- | ---------- | -------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`       | O limite superior para exibição.                                                                   |
| `style`              | `bold red` | O estilo usado para exibir quando estiver em uso.                                                  |
| `charging_symbol`    | `-`        | Simbolo opcional, mostrado quando a opção estiver em uso, o simbolo padrão é `charging_symbol`.    |
| `discharging_symbol` | `-`        | Simbolo opcional, mostrado quando a opção estiver em uso, o simbolo padrão é `discharging_symbol`. |

#### Exemplo

```toml
[[battery.display]] # "bold red" style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = "bold red"

[[battery.display]] # "bold yellow" style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = "bold yellow"
discharging_symbol = "💦"

# when capacity is over 30%, the battery indicator will not be displayed
```

## Caractere

The `character` module shows a character (usually an arrow) beside where the text is entered in your terminal.

The character will tell you whether the last command was successful or not. It can do this in two ways:

- alterando a cor (`red`/`green`)
- alterando a forma (`❯`/`✖`)

By default it only changes color. If you also want to change its shape take a look at [this example](#with-custom-error-shape).

::: atenção

`error_symbol` is not supported on nu shell.

:::

::: atenção

`vicmd_symbol` is only supported in cmd, fish and zsh.

:::

### Opções

| Opções           | Padrão              | Descrição                                                                                   |
| ---------------- | ------------------- | ------------------------------------------------------------------------------------------- |
| `format`         | `"$symbol"`         | O formato da string usado antes da entrada dos textos.                                      |
| `success_symbol` | `"[❯](bold green)"` | O formato da string usado antes da entrada de texto se o comando anterior for bem-sucedido. |
| `error_symbol`   | `"[❯](bold red)"`   | O formato de string usado antes da entrada de texto se o comando anterior tiver falhado.    |
| `vicmd_symbol`   | `"[❮](bold green)"` | O fromato de string usado antes da entrada de texto se o shell esta no vim normal mode.     |
| `disabled`       | `false`             | Desabilita o módulo `character`.                                                            |

### Variáveis

| Variável | Exemplo | Descrição                                                        |
| -------- | ------- | ---------------------------------------------------------------- |
| symbol   |         | Um espelho de `success_symbol`, `error_symbol` ou `vicmd_symbol` |

### Exemplos

#### Com formas customizadas de erro

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[✗](bold red) "
```

#### Sem formas customizadas de erro

```toml
# ~/.config/starship.toml

[character]
success_symbol = "[➜](bold green) "
error_symbol = "[➜](bold red) "
```

#### Com formas customizadas no vim

```toml
# ~/.config/starship.toml

[character]
vicmd_symbol = "[V](bold green) "
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). By default the module will be activated if any of the following conditions are met:

- O diretorio atual cotem um arquivo `CMakeLists.txt`
- O diretorio atual tem um arquivo `CMakeCache.txt`

### Opções

| Opções              | Padrão                                 | Descrição                                                                            |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`   | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                            | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"△ "`                                 | O simbolo usado antes da versão do cmake.                                            |
| `detect_extensions` | `[]`                                   | Quais extensões devem acionar este módulo                                            |
| `detect_files`      | `["CMakeLists.txt", "CMakeCache.txt"]` | []                                                                                   |
| `detect_folders`    | `[]`                                   | Quais pastas devem ativar este módulo                                                |
| `style`             | `"bold blue"`                          | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                | Desabilita o módulo `cmake`.                                                         |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v3.17.3` | A versão do cmake                  |
| symbol    |           | Espelha o valor da opção `símbolo` |
| style\* |           | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL. By default, the module will be shown if any of the following conditions are met:

- O diretório atual tem qualquer arquivo com extensão `.cob` or `.COB`
- O diretório atual tenham qualquer arquivo com extensão `.cbl` or `.CBL`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `symbol`            | `"⚙️ "`                              | O simbolo usado antes de exibir a versão do COBOL.                                   |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold blue"`                        | O estilo do módulo.                                                                  |
| `detect_extensions` | `["cbl", "cob", "CBL", "COB"]`       | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `disabled`          | `false`                              | Desabilita o módulo `cobol`.                                                         |

### Variáveis

| Variável  | Exemplo    | Descrição                          |
| --------- | ---------- | ---------------------------------- |
| version   | `v3.1.2.0` | A versão do `cobol`                |
| symbol    |            | Espelha o valor da opção `símbolo` |
| style\* |            | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

## Tempo de execução do comando

The `cmd_duration` module shows how long the last command took to execute. The module will be shown only if the command took longer than two seconds, or the `min_time` config value, if it exists.

::: warning Do not hook the DEBUG trap in Bash

If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

:::

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simply define the arrays `preexec_functions` and `precmd_functions` before running `eval $(starship init $0)`, and then proceed as normal.

### Opções

| Opções                 | Padrão                        | Descrição                                                                                                                                                         |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Duração mais curta para exibir o tempo (em milissegundos).                                                                                                        |
| `show_milliseconds`    | `false`                       | Exibir milissegundos ou invés de segundos para duração.                                                                                                           |
| `format`               | `"took [$duration]($style) "` | O formato do módulo.                                                                                                                                              |
| `style`                | `"bold yellow"`               | O estilo do módulo.                                                                                                                                               |
| `disabled`             | `false`                       | Desabilita o módulo `cmd_duration`.                                                                                                                               |
| `show_notifications`   | `false`                       | Exibi notificações no desktop quando o comando for concluído.                                                                                                     |
| `min_time_to_notify`   | `45_000`                      | Tempo minimo para notificação (em milissegundos).                                                                                                                 |
| `notification_timeout` |                               | Duration to show notification for (in milliseconds). If unset, notification timeout will be determined by daemon. Not all notification daemons honor this option. |

### Variáveis

| Variável  | Exemplo  | Descrição                                 |
| --------- | -------- | ----------------------------------------- |
| duration  | `16m40s` | O tempo que levou para executar o comando |
| style\* |          | Espelha o valor da opção `style`          |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

::: tip

This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.

:::

### Opções

| Opções              | Padrão                                 | Descrição                                                                                                                                                                                                  |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | O número de diretórios do envirionment path deve ser truncado, se o environment foi criado via `conda create -p [path]`. `0` quer dizer sem truncação. Também consulte o módulo [`directory`](#directory). |
| `symbol`            | `"🅒 "`                                 | O simbolo usado antes do nome do environment.                                                                                                                                                              |
| `style`             | `"bold green"`                         | O estilo do módulo.                                                                                                                                                                                        |
| `format`            | `"via [$symbol$environment]($style) "` | O formato do módulo.                                                                                                                                                                                       |
| `ignore_base`       | `true`                                 | Ignora o environment `base` quando ativado.                                                                                                                                                                |
| `disabled`          | `false`                                | Desabilita o módulo `conda`.                                                                                                                                                                               |

### Variáveis

| Variável    | Exemplo      | Descrição                          |
| ----------- | ------------ | ---------------------------------- |
| environment | `astronauts` | O environment atual do conda       |
| symbol      |              | Espelha o valor da opção `símbolo` |
| style\*   |              | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Container

The `container` module displays a symbol and container name, if inside a container.

### Opções

| Opções     | Padrão                               | Descrição                                 |
| ---------- | ------------------------------------ | ----------------------------------------- |
| `symbol`   | `"⬢"`                                | The symbol shown, when inside a container |
| `style`    | `"bold red dimmed"`                  | O estilo do módulo.                       |
| `format`   | "[$symbol \\[$name\\]]($style) " | O formato do módulo.                      |
| `disabled` | `false`                              | Disables the `container` module.          |

### Variáveis

| Variável  | Exemplo             | Descrição                          |
| --------- | ------------------- | ---------------------------------- |
| name      | `fedora-toolbox:35` | The name of the container          |
| symbol    |                     | Espelha o valor da opção `símbolo` |
| style\* |                     | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[container]
format = "[$symbol \\[$name\\]]($style) "
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `shard.yml`
- O diretório atual contem um arquivo `.cr`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `symbol`            | `"🔮 "`                               | The symbol used before displaying the version of crystal.                            |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                                  |
| `detect_extensions` | `["cr"]`                             | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["shard.yml"]`                      | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `disabled`          | `false`                              | Disables the `crystal` module.                                                       |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v0.32.1` | The version of `crystal`           |
| symbol    |           | Espelha o valor da opção `símbolo` |
| style\* |           | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem algum arquivo com extensão `.dart`
- O diretório atual contem um diretório `.dart_tool`
- O diretório atual contem um arquivo `pubspec.yaml`, `pubspec.yml` ou `pubspec.lock`

### Opções

| Opções              | Padrão                                            | Descrição                                                                            |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`              | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                                       | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🎯 "`                                            | A format string representing the symbol of Dart                                      |
| `detect_extensions` | `["dart"]`                                        | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[".dart_tool"]`                                  | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold blue"`                                     | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                           | Disables the `dart` module.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v2.8.4` | The version of `dart`              |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tenha um arquivo `deno.json`, `deno.jsonc`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js`

### Opções

| Opções              | Padrão                                                                  | Descrição                                                                            |
| ------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                    | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                                                             | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                                                  | A format string representing the symbol of Deno                                      |
| `detect_extensions` | `[]`                                                                    | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["deno.json", "deno.jsonc", "mod.ts", "mod.js", "deps.ts", "deps.js"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                                    | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"green bold"`                                                          | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                 | Disables the `deno` module.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v1.8.3` | The version of `deno`              |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

### Exemplo

```toml
# ~/.config/starship.toml

[deno]
format = "via [🦕 $version](green bold) "
```

## Diretório

The `directory` module shows the path to your current directory, truncated to three parent folders. Your directory will also be truncated to the root of the git repo that you're currently in.

When using the fish style pwd option, instead of hiding the path that is truncated, you will see a shortened name of each directory based on the number you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root, and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before it would have been `nixpkgs/pkgs`.

### Opções

| Opções              | Padrão                                             | Descrição                                                                               |
| ------------------- | -------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                | The number of parent folders that the current directory should be truncated to.         |
| `truncate_to_repo`  | `true`                                             | Whether or not to truncate to the root of the git repo that you're currently in.        |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | O formato do módulo.                                                                    |
| `style`             | `"bold cyan"`                                      | O estilo do módulo.                                                                     |
| `disabled`          | `false`                                            | Disables the `directory` module.                                                        |
| `read_only`         | `"🔒"`                                              | The symbol indicating current directory is read only.                                   |
| `read_only_style`   | `"red"`                                            | The style for the read only symbol.                                                     |
| `truncation_symbol` | `""`                                               | The symbol to prefix to truncated paths. eg: "…/"                                       |
| `repo_root_style`   | `None`                                             | The style for the root of the git repo when `truncate_to_repo` option is set to false.  |
| `home_symbol`       | `"~"`                                              | The symbol indicating home directory.                                                   |
| `use_os_path_sep`   | `true`                                             | Use the OS specific path seperator instead of always using `/` (e.g. `\` on Windows) |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Advanced Option             | Padrão | Descrição                                                                                                                                                              |
| --------------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |        | A table of substitutions to be made to the path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`    | The number of characters to use when applying fish shell pwd path logic.                                                                                               |
| `use_logical_path`          | `true` | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories (i.e. Java). Note that this will disable the fish style PWD.

```toml
[directory.substitutions]
"/Volumes/network/path" = "/net"
"src/com/long/java/path" = "mypath"
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero, the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variáveis

| Variável  | Exemplo               | Descrição                        |
| --------- | --------------------- | -------------------------------- |
| path      | `"D:/Projetos"`       | The current directory path       |
| style\* | `"black bold dimmed"` | Espelha o valor da opção `style` |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default` or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or `DOCKER_CONTEXT` environment variables are set (as they are meant to override the context in use).

### Opções

| Opções              | Padrão                                                        | Descrição                                                                         |
| ------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$context]($style) "`                            | O formato do módulo.                                                              |
| `symbol`            | `"🐳 "`                                                        | The symbol used before displaying the Docker context.                             |
| `only_with_files`   | `true`                                                        | Only show when there's a match                                                    |
| `detect_extensions` | `[]`                                                          | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                          | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `"blue bold"`                                                 | O estilo do módulo.                                                               |
| `disabled`          | `false`                                                       | Disables the `docker_context` module.                                             |

### Variáveis

| Variável  | Exemplo        | Descrição                          |
| --------- | -------------- | ---------------------------------- |
| context   | `test_context` | The current docker context         |
| symbol    |                | Espelha o valor da opção `símbolo` |
| style\* |                | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. If the SDK has been pinned in the current directory, the pinned version is shown. Otherwise the module shows the latest installed version of the SDK.

By default this module will only be shown in your prompt when one or more of the following files are present in the current directory:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

You'll also need the .NET Core SDK installed in order to use it correctly.

Internally, this module uses its own mechanism for version detection. Typically it is twice as fast as running `dotnet --version`, but it may show an incorrect version if your .NET project has an unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) when there is a csproj file in the current directory.

### Opções

| Opções              | Padrão                                                                                                  | Descrição                                                                            |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                                                                                             | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | The symbol used before displaying the version of dotnet.                             |
| `heuristic`         | `true`                                                                                                  | Use faster version detection to keep starship snappy.                                |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                    | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold blue"`                                                                                           | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                                                 | Disables the `dotnet` module.                                                        |

### Variáveis

| Variável  | Exemplo          | Descrição                                                          |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | The version of `dotnet` sdk                                        |
| tfm       | `netstandard2.0` | The Target Framework Moniker that the current project is targeting |
| symbol    |                  | Espelha o valor da opção `símbolo`                                 |
| style\* |                  | Espelha o valor da opção `style`                                   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `mix.exs`.

### Opções

| Opções              | Padrão                                                      | Descrição                                                                            |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | The format for the module elixir.                                                    |
| `version_format`    | `"v${raw}"`                                                 | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | The symbol used before displaying the version of Elixir/Erlang.                      |
| `detect_extensions` | `[]`                                                        | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["mix.exs"]`                                               | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                        | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold purple"`                                             | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                                        |

### Variáveis

| Variável    | Exemplo | Descrição                          |
| ----------- | ------- | ---------------------------------- |
| version     | `v1.10` | The version of `elixir`            |
| otp_version |         | The otp version of `elixir`        |
| symbol      |         | Espelha o valor da opção `símbolo` |
| style\*   |         | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `elm.json`
- O diretório atual contem o arquivo `elm-package.json`
- O diretório atual contem um arquivo `.elm-version`
- O diretório atual contem uma pasta `elm-stuff`
- O diretório contem arquivos `*.elm`

### Opções

| Opções              | Padrão                                             | Descrição                                                                            |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`               | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                                        | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌳 "`                                             | A format string representing the symbol of Elm.                                      |
| `detect_extensions` | `["elm"]`                                          | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `["elm-stuff"]`                                    | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"cyan bold"`                                      | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                            | Disables the `elm` module.                                                           |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v0.19.1` | The version of `elm`               |
| symbol    |           | Espelha o valor da opção `símbolo` |
| style\* |           | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[elm]
format = "via [ $version](cyan bold) "
```

## Variáveis de Ambiente

The `env_var` module displays the current value of a selected environment variables. The module will be shown only if any of the following conditions are met:

- A opção de configuração da `variable` corresponde a uma variável existente
- A configuração `variable` não está definida, mas a `default` está

::: tip Multiple environmental variables can be displayed by using a `.`. (see example) If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.

Example: following configuration will display value of USER environment variable

```toml
# ~/.config/starship.toml

[env_var.USER]
default = "unknown user"
```

:::

### Opções

| Opções     | Padrão                         | Descrição                                                                    |
| ---------- | ------------------------------ | ---------------------------------------------------------------------------- |
| `symbol`   | `""`                           | The symbol used before displaying the variable value.                        |
| `variable` |                                | The environment variable to be displayed.                                    |
| `default`  |                                | The default value to be displayed when the selected variable is not defined. |
| `format`   | `"with [$env_value]($style) "` | O formato do módulo.                                                         |
| `disabled` | `false`                        | Disables the `env_var` module.                                               |

### Variáveis

| Variável  | Exemplo                                  | Descrição                                  |
| --------- | ---------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (se _variable_ seria `$OS`) | The environment value of option `variable` |
| symbol    |                                          | Espelha o valor da opção `símbolo`         |
| style\* | `black bold dimmed`                      | Espelha o valor da opção `style`           |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "shell desconhecido"
```

Displaying multiple environmental variables:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = "SHELL"
default = "unknown shell"
[env_var.USER]
default = "unknown user"
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `rebar.config`.
- O diretório atual contem um arquivo `erlang.mk`.

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | The symbol used before displaying the version of erlang.                             |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                                  |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `disabled`          | `false`                              | Disables the `erlang` module.                                                        |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v22.1.3` | The version of `erlang`            |
| symbol    |           | Espelha o valor da opção `símbolo` |
| style\* |           | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are present in a line they will split the space evenly between them. This is useful for aligning other modules.

### Opções

| Opções     | Padrão         | Descrição                         |
| ---------- | -------------- | --------------------------------- |
| `symbol`   | `"."`          | The symbol used to fill the line. |
| `style`    | `"bold black"` | O estilo do módulo.               |
| `disabled` | `false`        | Disables the `fill` module        |

### Exemplo

```toml
# ~/.config/starship.toml
format = "AA $fill BB $fill CC"

[fill]
symbol = "-"
style = "bold green"
```

Produces a prompt that looks like:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Opções

| Opções           | Padrão                                                     | Descrição                                                       |
| ---------------- | ---------------------------------------------------------- | --------------------------------------------------------------- |
| `format`         | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | O formato do módulo.                                            |
| `symbol`         | `"☁️  "`                                                   | The symbol used before displaying the current GCP profile.      |
| `region_aliases` |                                                            | Table of region aliases to display in addition to the GCP name. |
| `style`          | `"bold blue"`                                              | O estilo do módulo.                                             |
| `disabled`       | `false`                                                    | Disables the `gcloud` module.                                   |

### Variáveis

| Variável  | Exemplo       | Descrição                                                          |
| --------- | ------------- | ------------------------------------------------------------------ |
| region    | `us-central1` | The current GCP region                                             |
| account   | `foo`         | The current GCP profile                                            |
| domain    | `example.com` | The current GCP profile domain                                     |
| project   |               | The current GCP project                                            |
| active    | `default`     | The active config name written in `~/.config/gcloud/active_config` |
| symbol    |               | Espelha o valor da opção `símbolo`                                 |
| style\* |               | Espelha o valor da opção `style`                                   |

*: This variable can only be used as a part of a style string

### Exemplos

#### Exibe conta e projeto

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
```

#### Exibe apenas o nome da configuração ativa

```toml
# ~/.config/starship.toml

[gcloud]
format = "[$symbol$active]($style) "
style = "bold yellow"
```

#### Exibir conta e região

```toml
# ~/.config/starship.toml

[gcloud]
symbol = "️🇬️ "
[gcloud.region_aliases]
us-central1 = "uc1"
asia-northeast1 = "an1"
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Opções

| Opções               | Padrão                           | Descrição                                                                                |
| -------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Shows the remote tracking branch name, even if it is equal to the local branch name.     |
| `format`             | `"on [$symbol$branch]($style) "` | O formato do módulo. Use `"$branch"` to refer to the current branch name.                |
| `symbol`             | `" "`                           | A format string representing the symbol of git branch.                                   |
| `style`              | `"bold purple"`                  | O estilo do módulo.                                                                      |
| `truncation_length`  | `2^63 - 1`                       | Truncates a git branch to `N` graphemes.                                                 |
| `truncation_symbol`  | `"…"`                            | The symbol used to indicate a branch name was truncated. You can use `""` for no symbol. |
| `only_attached`      | `false`                          | Only show the branch name when not in a detached `HEAD` state.                           |
| `disabled`           | `false`                          | Disables the `git_branch` module.                                                        |

### Variáveis

| Variável      | Exemplo  | Descrição                                                                                              |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | The remote name.                                                                                       |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                       |
| symbol        |          | Espelha o valor da opção `símbolo`                                                                     |
| style\*     |          | Espelha o valor da opção `style`                                                                       |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Opções

| Opções               | Padrão                             | Descrição                                               |
| -------------------- | ---------------------------------- | ------------------------------------------------------- |
| `commit_hash_length` | `7`                                | The length of the displayed git commit hash.            |
| `format`             | `"[\\($hash$tag\\)]($style) "` | O formato do módulo.                                    |
| `style`              | `"bold green"`                     | O estilo do módulo.                                     |
| `only_detached`      | `true`                             | Only show git commit hash when in detached `HEAD` state |
| `tag_disabled`       | `true`                             | Disables showing tag info in `git_commit` module.       |
| `tag_symbol`         | `" 🏷 "`                            | Tag symbol prefixing the info shown                     |
| `disabled`           | `false`                            | Disables the `git_commit` module.                       |

### Variáveis

| Variável  | Exemplo   | Descrição                        |
| --------- | --------- | -------------------------------- |
| hash      | `b703eb3` | The current git commit hash      |
| style\* |           | Espelha o valor da opção `style` |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

The `git_state` module will show in directories which are part of a git repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc. If there is progress information (e.g., REBASING 3/10), that information will be shown too.

### Opções

| Opções         | Padrão                                                          | Descrição                                                                               |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `"REBASING"`                                                    | A format string displayed when a `rebase` is in progress.                               |
| `merge`        | `"MERGING"`                                                     | A format string displayed when a `merge` is in progress.                                |
| `revert`       | `"REVERTING"`                                                   | A format string displayed when a `revert` is in progress.                               |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | A format string displayed when a `cherry-pick` is in progress.                          |
| `bisect`       | `"BISECTING"`                                                   | A format string displayed when a `bisect` is in progress.                               |
| `am`           | `"AM"`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress.            |
| `am_or_rebase` | `"AM/REBASE"`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress. |
| `style`        | `"bold yellow"`                                                 | O estilo do módulo.                                                                     |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | O formato do módulo.                                                                    |
| `disabled`     | `false`                                                         | Disables the `git_state` module.                                                        |

### Variáveis

| Variável         | Exemplo    | Descrição                        |
| ---------------- | ---------- | -------------------------------- |
| state            | `REBASING` | The current state of the repo    |
| progress_current | `1`        | The current operation progress   |
| progress_total   | `2`        | The total operation progress     |
| style\*        |            | Espelha o valor da opção `style` |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in the current git repository.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opções

| Opções               | Padrão                                                       | Descrição                             |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `"bold green"`                                               | The style for the added count.        |
| `deleted_style`      | `"bold red"`                                                 | The style for the deleted count.      |
| `only_nonzero_diffs` | `true`                                                       | Render status only for changed items. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | O formato do módulo.                  |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.    |

### Variáveis

| Variável          | Exemplo | Descrição                                   |
| ----------------- | ------- | ------------------------------------------- |
| added             | `1`     | The current number of added lines           |
| deleted           | `2`     | The current number of deleted lines         |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = "bold blue"
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your current directory.

### Opções

| Opções              | Padrão                                          | Descrição                           |
| ------------------- | ----------------------------------------------- | ----------------------------------- |
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status` |
| `conflicted`        | `"="`                                           | This branch has merge conflicts.    |
| `ahead`             | `"⇡"`                                           | The format of `ahead`               |
| `behind`            | `"⇣"`                                           | The format of `behind`              |
| `diverged`          | `"⇕"`                                           | The format of `diverged`            |
| `up_to_date`        | `""`                                            | The format of `up_to_date`          |
| `untracked`         | `"?"`                                           | The format of `untracked`           |
| `stashed`           | `"$"`                                           | The format of `stashed`             |
| `modified`          | `"!"`                                           | The format of `modified`            |
| `staged`            | `"+"`                                           | The format of `staged`              |
| `renamed`           | `"»"`                                           | The format of `renamed`             |
| `deleted`           | `"✘"`                                           | The format of `deleted`             |
| `style`             | `"bold red"`                                    | O estilo do módulo.                 |
| `ignore_submodules` | `false`                                         | Ignore changes to submodules.       |
| `disabled`          | `false`                                         | Disables the `git_status` module.   |

### Variáveis

The following variables can be used in `format`:

| Variável       | Descrição                                                                                                     |
| -------------- | ------------------------------------------------------------------------------------------------------------- |
| `all_status`   | Shortcut for`$conflicted$stashed$deleted$renamed$modified$staged$untracked`                                   |
| `ahead_behind` | Displays `diverged`, `ahead`, `behind` or `up_to_date` format string based on the current status of the repo. |
| `conflicted`   | Displays `conflicted` when this branch has merge conflicts.                                                   |
| `untracked`    | Displays `untracked` when there are untracked files in the working directory.                                 |
| `stashed`      | Displays `stashed` when a stash exists for the local repository.                                              |
| `modified`     | Displays `modified` when there are file modifications in the working directory.                               |
| `staged`       | Displays `staged` when a new file has been added to the staging area.                                         |
| `renamed`      | Displays `renamed` when a renamed file has been added to the staging area.                                    |
| `deleted`      | Displays `deleted` when a file's deletion has been added to the staging area.                                 |
| style\*      | Espelha o valor da opção `style`                                                                              |

*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| Variável       | Descrição                                      |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | Number of commits ahead of the tracking branch |
| `behind_count` | Number of commits behind the tracking branch   |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Variável | Descrição                |
| -------- | ------------------------ |
| `count`  | Show the number of files |

### Exemplo

```toml
# ~/.config/starship.toml

[git_status]
conflicted = "🏳"
ahead = "🏎💨"
behind = "😰"
diverged = "😵"
up_to_date = "✓"
untracked = "🤷"
stashed = "📦"
modified = "📝"
staged = '[++\($count\)](green)'
renamed = "👅"
deleted = "🗑"
```

Show ahead/behind count of the branch being tracked

```toml
# ~/.config/starship.toml

[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `go.mod`
- O diretório atual contem um arquivo `go.sum`
- O diretório atual contem um arquivo `glide.yaml`
- O diretório atual contem um arquivo `Gopkg.yml`
- O diretório atual contém um arquivo `Gopkg.lock`
- O diretório atual contem um arquivo `.go-version`
- O diretório atual contem um diretório `Godeps`
- O diretório atual contem arquivos com a extensão `.go`

### Opções

| Opções              | Padrão                                                                         | Descrição                                                                            |
| ------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                           | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                                                                    | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                         | A format string representing the symbol of Go.                                       |
| `detect_extensions` | `["go"]`                                                                       | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `["Godeps"]`                                                                   | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold cyan"`                                                                  | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                        | Disables the `golang` module.                                                        |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v1.12.1` | The version of `go`                |
| symbol    |           | Espelha o valor da opção `símbolo` |
| style\* |           | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `helmfile.yaml`
- O diretório atual contem um arquivo `Chart.yaml`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm.                                     |
| `style`             | `"bold white"`                       | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `helm` module.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v3.1.1` | The version of `helm`              |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[helm]
format = "via [⎈ $version](bold white) "
```

## Hostname

The `hostname` module shows the system hostname.

### Opções

| Opções     | Padrão                      | Descrição                                                                                                                            |
| ---------- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `ssh_only` | `true`                      | Only show hostname when connected to an SSH session.                                                                                 |
| `trim_at`  | `"."`                       | String that the hostname is cut off at, after the first match. `"."` will stop after the first dot. `""` will disable any truncation |
| `format`   | `"[$hostname]($style) in "` | O formato do módulo.                                                                                                                 |
| `style`    | `"bold dimmed green"`       | O estilo do módulo.                                                                                                                  |
| `disabled` | `false`                     | Disables the `hostname` module.                                                                                                      |

### Variáveis

| Variável  | Exemplo    | Descrição                        |
| --------- | ---------- | -------------------------------- |
| hostname  | `computer` | The hostname of the computer     |
| style\* |            | Espelha o valor da opção `style` |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contenha algum dos arquivos `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot`
- O diretório atual contenha arquivos com as extensões `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc`

### Opções

| Opções              | Padrão                                                                                                    | Descrição                                                                            |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                                                                                               | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                      | Quais pastas devem ativar este módulo.                                               |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java                                      |
| `style`             | `"red dimmed"`                                                                                            | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                                                   | Disables the `java` module.                                                          |

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| version   | `v14`   | The version of `java`              |
| symbol    |         | Espelha o valor da opção `símbolo` |
| style\* |         | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there are at least 2 jobs, or more than the `number_threshold` config value, if it exists. The module will show a symbol if there is at least 1 job, or more than the `symbol_threshold` config value, if it exists. You can set both values to 0 in order to _always_ show the symbol and number of jobs, even if there are 0 jobs running.

The default functionality is:

- 0 jobs -> Nada é exibido.
- 1 job -> `symbol` é exibido.
- 2 jobs or more -> `symbol` + `number` é exibido.

::: atenção

This module is not supported on tcsh and nu.

:::

::: atenção

The `threshold` option is deprecated, but if you want to use it, the module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

:::

### Opções

| Opções             | Padrão                        | Descrição                                                                |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `"[$symbol$number]($style) "` | O formato do módulo.                                                     |
| `symbol`           | `"✦"`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `"bold blue"`                 | O estilo do módulo.                                                      |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| number    | `1`     | The number of jobs                 |
| symbol    |         | Espelha o valor da opção `símbolo` |
| style\* |         | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[jobs]
symbol = "+ "
number_threshold = 4
symbol_threshold = 0
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `Project.toml`
- O diretório atual contem um arquivo `Manifest.toml`
- O diretório atual contem arquivos com a extensão `.jl`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia.                                    |
| `style`             | `"bold purple"`                      | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `julia` module.                                                         |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v1.4.0` | The version of `julia`             |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## localip

The `localip` module shows the IPv4 address of the primary network interface.

### Opções

| Opções     | Padrão                    | Descrição                                              |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | Only show IP address when connected to an SSH session. |
| `format`   | `"[$localipv4]($style) "` | O formato do módulo.                                   |
| `style`    | `"bold yellow"`           | O estilo do módulo.                                    |
| `disabled` | `true`                    | Disables the `localip` module.                         |

### Variáveis

| Variável  | Exemplo      | Descrição                         |
| --------- | ------------ | --------------------------------- |
| localipv4 | 192.168.1.13 | Contains the primary IPv4 address |
| style\* |              | Espelha o valor da opção `style`  |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = "@[$localipv4](bold red) "
disabled = false
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `.kt` ou um arquivo `.kts`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["kt", "kts"]`                      | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                                   |
| `style`             | `"bold blue"`                        | O estilo do módulo.                                                                  |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version.        |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                        |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`            |
| symbol    |           | Espelha o valor da opção `símbolo` |
| style\* |           | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[kotlin]
symbol = "🅺 "
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = "kotlinc"
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-cluster --namespace astronaut`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opções

| Opções            | Padrão                                               | Descrição                                                             |
| ----------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| `symbol`          | `"☸ "`                                               | A format string representing the symbol displayed before the Cluster. |
| `format`          | `'[$symbol$context( \($namespace\))]($style) in '` | O formato do módulo.                                                  |
| `style`           | `"cyan bold"`                                        | O estilo do módulo.                                                   |
| `context_aliases` |                                                      | Table of context aliases to display.                                  |
| `disabled`        | `true`                                               | Disables the `kubernetes` module.                                     |

### Variáveis

| Variável  | Exemplo              | Descrição                                |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-cluster`   | The current kubernetes context           |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| symbol    |                      | Espelha o valor da opção `símbolo`       |
| style\* |                      | Espelha o valor da opção `style`         |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ $context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
".*/openshift-cluster/.*" = "openshift"
"gke_.*_(?P<cluster>[\\w-]+)" = "gke-$cluster"
```

#### Correspondência Regex

Additional to simple aliasing, `context_aliases` also supports extended matching and renaming using regular expressions.

The regular expression must match on the entire kube context, capture groups can be referenced using `$name` and `$N` in the replacement. This is more explained in the [regex crate](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace) documentation.

Long and automatically generated cluster names can be identified and shortened using regular expressions:

```toml
[kubernetes.context_aliases]
# OpenShift contexts carry the namespace and user in the kube context: `namespace/name/user`:
".*/openshift-cluster/.*" = "openshift"
# Or better, to rename every OpenShift cluster at once:
".*/(?P<cluster>[\\w-]+)/.*" = "$cluster"

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
"gke_.*_(?P<cluster>[\\w-]+)" = "gke-$cluster"
```

## Line Break

The `line_break` module separates the prompt into two lines.

### Opções

| Opções     | Padrão  | Descrição                                                          |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### Exemplo

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contém um arquivo `.lua-version`
- O diretório atual contém um diretório `lua`
- O diretório atual tem um arquivo com a extensão `.lua`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                                      |
| `detect_extensions` | `["lua"]`                            | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[".lua-version"]`                   | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `["lua"]`                            | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold blue"`                        | O estilo do módulo.                                                                  |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version.           |
| `disabled`          | `false`                              | Disables the `lua` module.                                                           |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v5.4.0` | The version of `lua`               |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Memory Usage

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opções

| Opções      | Padrão                                          | Descrição                                                |
| ----------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | Hide the memory usage unless it exceeds this percentage. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | O formato do módulo.                                     |
| `symbol`    | `"🐏"`                                           | The symbol used before displaying the memory usage.      |
| `style`     | `"bold dimmed white"`                           | O estilo do módulo.                                      |
| `disabled`  | `true`                                          | Disables the `memory_usage` module.                      |

### Variáveis

| Variável         | Exemplo       | Descrição                                                          |
| ---------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | The usage/total RAM of the current system memory.                  |
| ram_pct          | `48%`         | The percentage of the current system memory.                       |
| swap\*\*     | `1GiB/4GiB`   | The swap memory size of the current system swap memory file.       |
| swap_pct\*\* | `77%`         | The swap memory percentage of the current system swap memory file. |
| symbol           | `🐏`           | Espelha o valor da opção `símbolo`                                 |
| style\*        |               | Espelha o valor da opção `style`                                   |

*: This variable can only be used as a part of a style string *\*: The SWAP file information is only displayed if detected on the current system

### Exemplo

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = " "
style = "bold dimmed green"
```

## Mercurial Branch

The `hg_branch` module shows the active branch of the repo in your current directory.

### Opções

| Opções              | Padrão                           | Descrição                                                                                    |
| ------------------- | -------------------------------- | -------------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | The symbol used before the hg bookmark or branch name of the repo in your current directory. |
| `style`             | `"bold purple"`                  | O estilo do módulo.                                                                          |
| `format`            | `"on [$symbol$branch]($style) "` | O formato do módulo.                                                                         |
| `truncation_length` | `2^63 - 1`                       | Truncates the hg branch name to `N` graphemes                                                |
| `truncation_symbol` | `"…"`                            | The symbol used to indicate a branch name was truncated.                                     |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| branch    | `master` | The active mercurial branch        |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[hg_branch]
format = "on [🌱 $branch](bold purple)"
truncation_length = 4
truncation_symbol = ""
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `nim.cfg`
- O diretório atual tenha um arquivo com a extensão `.nim`
- O diretório atual tenha um arquivo com a extensão `.nims`
- O diretório atual tenha um arquivo com a extensão `.nimble`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | The format for the module                                                            |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim.                                |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["nim.cfg"]`                        | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold yellow"`                      | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `nim` module.                                                           |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v1.2.0` | The version of `nimc`              |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[nim]
style = "yellow"
symbol = "🎣 "
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment. The module will be shown when inside a nix-shell environment.

### Opções

| Opções       | Padrão                                         | Descrição                                             |
| ------------ | ---------------------------------------------- | ----------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | O formato do módulo.                                  |
| `symbol`     | `"❄️ "`                                        | A format string representing the symbol of nix-shell. |
| `style`      | `"bold blue"`                                  | O estilo do módulo.                                   |
| `impure_msg` | `"impure"`                                     | A format string shown when the shell is impure.       |
| `pure_msg`   | `"pure"`                                       | A format string shown when the shell is pure.         |
| `disabled`   | `false`                                        | Disables the `nix_shell` module.                      |

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| state     | `pure`  | The state of the nix-shell         |
| name      | `lorri` | The name of the nix-shell          |
| symbol    |         | Espelha o valor da opção `símbolo` |
| style\* |         | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = "[impure shell](bold red)"
pure_msg = "[pure shell](bold green)"
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem o arquivo `package.json`
- O diretório atual tenha um arquivo `.node-version`
- O diretório atual tenha um arquivo`.nvmrc`
- O diretório atual tenha um diretório `node_modules`
- O diretório atual tenha um arquivo com a extensão `.js`, `.mjs` or `.cjs`
- O diretório atual tenha um arquivo com a extensão `.ts`

### Opções

| Opções              | Padrão                               | Descrição                                                                                             |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                                  |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch`                  |
| `symbol`            | `" "`                               | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Quais extensões devem ativar este módulo.                                                             |
| `detect_files`      | `["package.json", ".node-version"]`  | Quais nomes de arquivos devem ativar este módulo.                                                     |
| `detect_folders`    | `["node_modules"]`                   | Quais pastas devem ativar este módulo.                                                                |
| `style`             | `"bold green"`                       | O estilo do módulo.                                                                                   |
| `disabled`          | `false`                              | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variáveis

| Variável  | Exemplo    | Descrição                          |
| --------- | ---------- | ---------------------------------- |
| version   | `v13.12.0` | The version of `node`              |
| symbol    |            | Espelha o valor da opção `símbolo` |
| style\* |            | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contenha um arquivo com a extensão`.opam` ou um diretório `_opam`
- O diretório atual tenha um diretório `esy.lock`
- O diretório atual tenha um arquivo `dune` or `dune-project`
- O diretório atual tenha um arquivo `jbuild` or `jbuild-ignore`
- O diretório tenha um arquivo `.merlin`
- O diretório atual tenha um arquivo com a extensão `.ml`, `.mli`, `.re` ou `.rei`

### Opções

| Opções                    | Padrão                                                                     | Descrição                                                                            |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | A string de formato do módulo.                                                       |
| `version_format`          | `"v${raw}"`                                                                | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml.                              |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch.                              |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.                               |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Quais extensões devem ativar este módulo.                                            |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Quais pastas devem ativar este módulo.                                               |
| `style`                   | `"bold yellow"`                                                            | O estilo do módulo.                                                                  |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                                         |

### Variáveis

| Variável         | Exemplo      | Descrição                                                         |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Espelha o valor da opção `símbolo`                                |
| style\*        |              | Espelha o valor da opção `style`                                  |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[ocaml]
format = "via [🐪 $version]($style) "
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module only active when the `OS_CLOUD` env var is set, in which case it will read `clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). to fetch the current project in use.

### Opções

| Opções     | Padrão                                              | Descrição                                                      |
| ---------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | O formato do módulo.                                           |
| `symbol`   | `"☁️ "`                                             | The symbol used before displaying the current OpenStack cloud. |
| `style`    | `"bold yellow"`                                     | O estilo do módulo.                                            |
| `disabled` | `false`                                             | Disables the `openstack` module.                               |

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| cloud     | `corp`  | The current OpenStack cloud        |
| project   | `dev`   | The current OpenStack project      |
| symbol    |         | Espelha o valor da opção `símbolo` |
| style\* |         | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – O versionamento de pacotes `npm` é extraído do `package.json` presente no diretório atual
- [**Cargo**](https://doc.rust-lang.org/cargo/) – O versionamento de pacotes `cargo`é extraído do arquivo `Cargo.toml` presente no diretório atual
- [**Nimble**](https://github.com/nim-lang/nimble) - O versionamento de pacotes `nimble` é extraído do arquivo `*.nimble` presente no diretório atual com o comando`nimble dump`
- [**Poetry**](https://python-poetry.org/) – O versionamento de pacotes `poetry` é extraído do arquivo `pyproject.toml` presente no diretório atual
- [**Python**](https://www.python.org) - O versionamento de pacotes `python` é extraído do arquivo `setup.cfg` presente no diretório atual
- [**Composer**](https://getcomposer.org/) – O versionamento de pacotes `composer` é extraído do arquivo`composer.json` presente no diretório atual
- [**Gradle**](https://gradle.org/) – O versionamento de pacotes `gradle` é extraído do arquivo `build.gradle` presente no diretório atual
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - O versionamento de pacotes é extraído do arquivo `Project.toml` presente no diretório atual
- [**Mix**](https://hexdocs.pm/mix/) - O versionamento de pacotes `mix`é extraído do arquivo `mix.exs` presente no diretório atual
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - O versionamento do gráfico `helm` é extraído do arquivo `Chart.yaml` presente no diretório atual
- [**Maven**](https://maven.apache.org/) - O versionamento de pacotes `maven` é extraído do arquivo`pom.xml` presente no diretório atual
- [**Meson**](https://mesonbuild.com/) - O versionamento de pacotes `meson` é extraído do arquivo`meson.build` presente no diretório atual
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - O versionamento de pacotes `shards` é extraído do arquivo `shard.yml` presente no diretório atual
- [**V**](https://vlang.io) - O versionamento de pacotes `vlang` é extraido do arquivo`v.mod` presente no diretório atual
- [**SBT**](https://scala-sbt.org) - O versionamento de pacotes `sbt` pé extraído do arquivo `build.sbt` presente no diretório atual
- [**Dart**](https://pub.dev/) - O versionamento de pacotes `dart` pé extraído do arquivo `pubspec.yaml` presente no diretório atual

> ⚠️ A versão exibida é a que esta presente no código fonte do diretório atual e não do gerenciador de pacotes.

### Opções

| Opções            | Padrão                            | Descrição                                                                            |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------------------ |
| `format`          | `"is [$symbol$version]($style) "` | O formato do módulo.                                                                 |
| `symbol`          | `"📦 "`                            | The symbol used before displaying the version the package.                           |
| `version_format`  | `"v${raw}"`                       | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`           | `"bold 208"`                      | O estilo do módulo.                                                                  |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.                            |
| `disabled`        | `false`                           | Disables the `package` module.                                                       |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v1.0.0` | The version of your package        |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tanha um aquivo `Makefile.PL` ou `Build.PL`
- O diretório atual tenha um arquivo `cpanfile` ou `cpanfile.snapshot`
- O diretório atual tenha um arquivo `META.json` ou `META.yml`
- O diretório atual tenha um arquivo `.perl-version`
- O diretório atual tenha um `.pl`, `.pm` ou `.pod`

### Opções

| Opções              | Padrão                                                                                                   | Descrição                                                                            |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | A string de formato do módulo.                                                       |
| `version_format`    | `"v${raw}"`                                                                                              | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl                                |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                     | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold 149"`                                                                                             | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                                          |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v5.26.1` | The version of `perl`              |
| symbol    |           | Espelha o valor da opção `símbolo` |
| style\* |           | Espelha o valor da opção `style`   |

### Exemplo

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `composer.json`
- O diretório atual tenha um arquivo `.php-version`
- O diretório atual tenha um arquivo com extensão `.php`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | The symbol used before displaying the version of PHP.                                |
| `detect_extensions` | `["php"]`                            | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["composer.json", ".php-version"]`  | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"147 bold"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `php` module.                                                           |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v7.3.8` | The version of `php`               |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

::: tip

By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms). If you still want to enable it, [follow the example shown below](#with-pulumi-version).

:::

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tenha um arquivo `Pulumi.yaml` ou `Pulumi.yml`
- Um diretório pai que tenha um arquivo `Pulumi.yaml` ou `Pulumi.yml`

### Opções

| Opções           | Padrão                                       | Descrição                                                                            |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`         | `"via [$symbol($username@)$stack]($style) "` | A string de formato do módulo.                                                       |
| `version_format` | `"v${raw}"`                                  | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `" "`                                       | A format string shown before the Pulumi stack.                                       |
| `style`          | `"bold 5"`                                   | O estilo do módulo.                                                                  |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                                        |

### Variáveis

| Variável  | Exemplo    | Descrição                          |
| --------- | ---------- | ---------------------------------- |
| version   | `v0.12.24` | The version of `pulumi`            |
| stack     | `dev`      | The current Pulumi stack           |
| username  | `alice`    | The current Pulumi username        |
| symbol    |            | Espelha o valor da opção `símbolo` |
| style\* |            | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

#### Com a versão do Pulumi

```toml
# ~/.config/starship.toml

[pulumi]
format = "[🛥 ($version )$stack]($style) "
```

#### Sem a versão do Pulumi

```toml
# ~/.config/starship.toml
[pulumi]
symbol = "🛥 "
format = "[$symbol$stack]($style) "
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `spago.dhall`
- O diretório atual tenha um arquivo com a extensão `.purs`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript.                         |
| `detect_extensions` | `["purs"]`                           | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["spago.dhall"]`                    | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold white"`                       | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `purescript` module.                                                    |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `0.13.5` | The version of `purescript`        |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[purescript]
format = "via [$symbol$version](bold white)"
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version name. Otherwise, it will display the version number from `python --version`.

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tenha um arquivo `.python-version`
- O diretório atual tenha um arquivo `Pipfile`
- O diretório atual tenha um arquivo `__init__.py`
- O diretório atual contem um arquivo `pyproject.toml`
- O diretório atual contem um arquivo `requirements.txt`
- O diretório atual contem um arquivo `setup.py`
- O diretório atual contem um arquivo `tox.ini`
- O diretório atual tenha um arquivo com a extensão `.py`.
- Um ambiente virtual está atualmente ativo

### Opções

| Opções               | Padrão                                                                                                       | Descrição                                                                              |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | O formato do módulo.                                                                   |
| `version_format`     | `"v${raw}"`                                                                                                  | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch`   |
| `symbol`             | `"🐍 "`                                                                                                       | A format string representing the symbol of Python                                      |
| `style`              | `"yellow bold"`                                                                                              | O estilo do módulo.                                                                    |
| `pyenv_version_name` | `false`                                                                                                      | Use pyenv to get Python version                                                        |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefix before pyenv version display, only used if pyenv is used                        |
| `python_binary`      | `["python", "python3", "python2"]`                                                                           | Configures the python binaries that Starship should executes when getting the version. |
| `detect_extensions`  | `["py"]`                                                                                                     | Quais extensões devem acionar este módulo                                              |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | []                                                                                     |
| `detect_folders`     | `[]`                                                                                                         | Quais pastas devem ativar este módulo                                                  |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                          |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Variáveis

| Variável     | Exemplo         | Descrição                                  |
| ------------ | --------------- | ------------------------------------------ |
| version      | `"v3.8.1"`      | The version of `python`                    |
| symbol       | `"🐍 "`          | Espelha o valor da opção `símbolo`         |
| style        | `"yellow bold"` | Espelha o valor da opção `style`           |
| pyenv_prefix | `"pyenv "`      | Mirrors the value of option `pyenv_prefix` |
| virtualenv   | `"venv"`        | The current `virtualenv` name              |

### Exemplo

```toml
# ~/.config/starship.toml

[python]
symbol = "👾 "
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = "python3"
```

```toml
# ~/.config/starship.toml

[python]
# Don't trigger for files with the py extension
detect_extensions = []
```

```toml
# ~/.config/starship.toml

[python]
# Display the version of python from inside a local venv.
#
# Note this will only work when the venv is inside the project and it will only
# work in the directory that contains the venv dir but maybe this is ok?
python_binary = ["./venv/bin/python", "python", "python3", "python2"]
```

## R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). The module will be shown if any of the following conditions are met:

- O diretório atual tenha um arquivo com a extensão `.R`.
- O diretório atual tenha um arquivo com a extensão `.Rd`.
- O diretório atual tenha um arquivo com a extensão `.Rmd`.
- O diretório atual tenha um arquivo com a extensão `.Rproj`.
- O diretório atual tenha um arquivo com a extensão `.Rsx`.
- O diretório atual tenha um arquivo `.Rprofile`
- O diretório atual tenha uma pasta `.Rpoj.user`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"📐"`                                | A format string representing the symbol of R.                                        |
| `style`             | `"blue bold"`                        | O estilo do módulo.                                                                  |
| `detect_extensions` | `["R", "Rd", "Rmd", "Rproj", "Rsx"]` | Quais extensões devem acionar este módulo                                            |
| `detect_files`      | `[".Rprofile"]`                      | []                                                                                   |
| `detect_folders`    | `[".Rproj.user"]`                    | Quais pastas devem ativar este módulo                                                |
| `disabled`          | `false`                              | Disables the `r` module.                                                             |

### Variáveis

| Variável | Exemplo       | Descrição                          |
| -------- | ------------- | ---------------------------------- |
| version  | `v4.0.5`      | The version of `R`                 |
| symbol   |               | Espelha o valor da opção `símbolo` |
| style    | `"blue bold"` | Espelha o valor da opção `style`   |

### Exemplo

```toml
# ~/.config/starship.toml

[rlang]
format = "with [📐 $version](blue bold) "
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). The module will be shown if any of the following conditions are met:

- O diretório atual contenha um arquivo com a extensão `.red` or `.reds`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.                                      |
| `detect_extensions` | `["red"]`                            | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"red bold"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `red` module.                                                           |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v2.5.1` | The version of `red`               |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). The module will be shown if any of the following conditions are met:

- O diretório atual tenha um arquivo `Gemfile`
- O diretório atual contém um arquivo `.ruby-version`
- O diretório atual contem um arquivo `.rb`
- As variáveis de ambiente `RUBY_VERSION` ou `RBENV_VERSION` estão definidas

Starship gets the current Ruby version by running `ruby -v`.

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                                     |
| `detect_extensions` | `["rb"]`                             | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `detect_variables`  | `["RUBY_VERSION", "RBENV_VERSION"]`  | Which environment variables should trigger this module.                              |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `ruby` module.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v2.5.1` | The version of `ruby`              |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). The module will be shown if any of the following conditions are met:

- O diretório atual contem um arquivo `Cargo.toml`
- O diretório atual tenha um arquivo com a extensão `.rs`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust                                      |
| `detect_extensions` | `["rs"]`                             | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Cargo.toml"]`                     | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `rust` module.                                                          |

### Variáveis

| Variável  | Exemplo           | Descrição                          |
| --------- | ----------------- | ---------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`             |
| symbol    |                   | Espelha o valor da opção `símbolo` |
| style\* |                   | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `build.sbt`, `.scalaenv` ou `.sbtenv`
- O diretório atual tenha um arquivo com a extensão `.scala` ou `.sbt`
- O diretório atual tenha um diretório chamado `.metals`

### Opções

| Opções              | Padrão                                   | Descrição                                                                            |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [${symbol}(${version} )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                              | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[".metals"]`                            | Quais pastas devem ativar este módulo.                                               |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.                                    |
| `style`             | `"red dimmed"`                           | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                  | Disables the `scala` module.                                                         |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `2.13.5` | The version of `scala`             |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opções

| Opções                 | Padrão                    | Descrição                                                    |
| ---------------------- | ------------------------- | ------------------------------------------------------------ |
| `bash_indicator`       | `bsh`                     | A format string used to represent bash.                      |
| `fish_indicator`       | `fsh`                     | A format string used to represent fish.                      |
| `zsh_indicator`        | `zsh`                     | A format string used to represent zsh.                       |
| `powershell_indicator` | `psh`                     | A format string used to represent powershell.                |
| `ion_indicator`        | `ion`                     | A format string used to represent ion.                       |
| `elvish_indicator`     | `esh`                     | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `tsh`                     | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `xsh`                     | A format string used to represent xonsh.                     |
| `cmd_indicator`        | `cmd`                     | A format string used to represent cmd.                       |
| `nu_indicator`         | `nu`                      | A format string used to represent nu.                        |
| `unknown_indicator`    |                           | The default value to be displayed when the shell is unknown. |
| `format`               | `"[$indicator]($style) "` | O formato do módulo.                                         |
| `style`                | `"white bold"`            | O estilo do módulo.                                          |
| `disabled`             | `true`                    | Disables the `shell` module.                                 |

### Variáveis

| Variável  | Padrão | Descrição                                                  |
| --------- | ------ | ---------------------------------------------------------- |
| indicator |        | Mirrors the value of `indicator` for currently used shell. |
| style\* |        | Mirrors the value of option `style`.                       |

*: This variable can only be used as a part of a style string

### Exemplos

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "mystery shell"
style = "cyan bold"
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Opções

| Opções      | Padrão                       | Descrição                                                     |
| ----------- | ---------------------------- | ------------------------------------------------------------- |
| `threshold` | `2`                          | Display threshold.                                            |
| `format`    | `"[$symbol$shlvl]($style) "` | O formato do módulo.                                          |
| `symbol`    | `"↕️  "`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`    | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `style`     | `"bold yellow"`              | O estilo do módulo.                                           |
| `disabled`  | `true`                       | Disables the `shlvl` module.                                  |

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| shlvl     | `3`     | The current value of `SHLVL`       |
| symbol    |         | Espelha o valor da opção `símbolo` |
| style\* |         | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = "$shlvl level(s) down"
threshold = 3
```

## Singularidade

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container and `$SINGULARITY_NAME` is set.

### Opções

| Opções     | Padrão                           | Descrição                                        |
| ---------- | -------------------------------- | ------------------------------------------------ |
| `format`   | `'[$symbol\[$env\]]($style) '` | O formato do módulo.                             |
| `symbol`   | `""`                             | A format string displayed before the image name. |
| `style`    | `"bold dimmed blue"`             | O estilo do módulo.                              |
| `disabled` | `false`                          | Disables the `singularity` module.               |

### Variáveis

| Variável  | Exemplo      | Descrição                          |
| --------- | ------------ | ---------------------------------- |
| env       | `centos.img` | The current Singularity image      |
| symbol    |              | Espelha o valor da opção `símbolo` |
| style\* |              | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`. The status code will cast to a signed 32-bit integer.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

::: warning This module is not supported on nu shell. :::

### Opções

| Opções                  | Padrão                                                                               | Descrição                                               |
| ----------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "`                                                        | O formato do módulo                                     |
| `symbol`                | `"✖"`                                                                                | The symbol displayed on program error                   |
| `success_symbol`        | `"✔️"`                                                                               | The symbol displayed on program success                 |
| `not_executable_symbol` | `"🚫"`                                                                                | The symbol displayed when file isn't executable         |
| `not_found_symbol`      | `"🔍"`                                                                                | The symbol displayed when the command can't be found    |
| `sigint_symbol`         | `"🧱"`                                                                                | The symbol displayed on SIGINT (Ctrl + c)               |
| `signal_symbol`         | `"⚡"`                                                                                | The symbol displayed on any signal                      |
| `style`                 | `"bold red"`                                                                         | O estilo do módulo.                                     |
| `recognize_signal_code` | `true`                                                                               | Enable signal mapping from exit code                    |
| `map_symbol`            | `false`                                                                              | Enable symbols mapping from exit code                   |
| `pipestatus`            | `false`                                                                              | Enable pipestatus reporting                             |
| `pipestatus_separator`  | `|`                                                                                  |                                                         |
| `pipestatus_format`     | `\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)` | The format of the module when the command is a pipeline |
| `disabled`              | `true`                                                                               | Disables the `status` module.                           |

### Variáveis

| Variável       | Exemplo | Descrição                                                                                   |
| -------------- | ------- | ------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                           |
| hex_status     | `0x7F`  | The exit code of the last command in hex                                                    |
| int            | `127`   | The exit code of the last command                                                           |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                         |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                             |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                        |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                |
| pipestatus     |         | Rendering of in pipeline programs's exit codes, this is only available in pipestatus_format |
| symbol         |         | Espelha o valor da opção `símbolo`                                                          |
| style\*      |         | Espelha o valor da opção `style`                                                            |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[status]
style = "bg:blue"
symbol = "🔴"
format = '[\[$symbol $common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

The `sudo` module displays if sudo credentials are currently cached. The module will only be shown if credentials are cached.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opções

| Opções          | Padrão                  | Descrição                                               |
| --------------- | ----------------------- | ------------------------------------------------------- |
| `format`        | `[as $symbol]($style)"` | O formato do módulo                                     |
| `symbol`        | `"🧙 "`                  | The symbol displayed when credentials are cached        |
| `style`         | `"bold blue"`           | O estilo do módulo.                                     |
| `allow_windows` | `false`                 | Since windows has no default sudo, default is disabled. |
| `disabled`      | `true`                  | Disables the `sudo` module.                             |

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| symbol    |         | Espelha o valor da opção `símbolo` |
| style\* |         | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[sudo]
style = "bold green"
symbol = "👩‍💻 "
disabled = false
```

```toml
# On windows
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). The module will be shown if any of the following conditions are met:

- O diretório atual tenha um arquivo `Package.swift`
- O diretório atual tenha um arquivo com a extensão `.swift`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift                                     |
| `detect_extensions` | `["swift"]`                          | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Package.swift"]`                  | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold 202"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `swift` module.                                                         |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v5.2.4` | The version of `swift`             |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[swift]
format = "via [🏎  $version](red bold)"
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.

::: tip

By default the Terraform version is not shown, since this is slow for current versions of Terraform when a lot of plugins are in use. If you still want to enable it, [follow the example shown below](#with-terraform-version).

:::

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tenha uma pasta `.terraform`
- O diretório atual tenha arquivos com as extensões `.tf`, `.tfplan` or `.tfstate`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol$workspace]($style) "` | A string de formato do módulo.                                                       |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.                                |
| `detect_extensions` | `["tf", "tfplan", "tfstate"]`        | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[".terraform"]`                     | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold 105"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `terraform` module.                                                     |

### Variáveis

| Variável  | Exemplo    | Descrição                          |
| --------- | ---------- | ---------------------------------- |
| version   | `v0.12.24` | The version of `terraform`         |
| workspace | `default`  | The current Terraform workspace    |
| symbol    |            | Espelha o valor da opção `símbolo` |
| style\* |            | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

#### Com a versão do Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Sem a versão do Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $workspace]($style) "
```

## Horário

The `time` module shows the current **local** time. The `format` configuration value is used by the [`chrono`](https://crates.io/crates/chrono) crate to control how the time is displayed. Take a look [at the chrono strftime docs](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) to see what options are available.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opções

| Opções            | Padrão                  | Descrição                                                                                                                          |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | A string de formato do módulo.                                                                                                     |
| `use_12hr`        | `false`                 | Enables 12 hour formatting                                                                                                         |
| `time_format`     | see below               | The [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) used to format the time.                |
| `style`           | `"bold yellow"`         | The style for the module time                                                                                                      |
| `utc_time_offset` | `"local"`               | Sets the UTC offset to use. Range from -24 &lt; x &lt; 24. Allows floats to accommodate 30/45 minute timezone offsets. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                        |
| `time_range`      | `"-"`                   | Sets the time range during which the module will be shown. Times must be specified in 24-hours format                              |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Variáveis

| Variável  | Exemplo    | Descrição                        |
| --------- | ---------- | -------------------------------- |
| time      | `13:08:10` | The current time.                |
| style\* |            | Espelha o valor da opção `style` |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = "%T"
utc_time_offset = "-5"
time_range = "10:00:00-14:00:00"
```

## Username

The `username` module shows active user's username. The module will be shown if any of the following conditions are met:

- O usuário atual é root
- O usuário atual não é o mesmo que está logado
- O usuário atual esta conectado em uma sessão SSH
- A variável `show_always` esta definida como true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Opções

| Opções        | Padrão                  | Descrição                             |
| ------------- | ----------------------- | ------------------------------------- |
| `style_root`  | `"bold red"`            | The style used when the user is root. |
| `style_user`  | `"bold yellow"`         | The style used for non-root users.    |
| `format`      | `"[$user]($style) in "` | O formato do módulo.                  |
| `show_always` | `false`                 | Always shows the `username` module.   |
| `disabled`    | `false`                 | Disables the `username` module.       |

### Variáveis

| Variável | Exemplo      | Descrição                                                                                   |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `"red bold"` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `"matchai"`  | The currently logged-in user ID.                                                            |

### Exemplo

```toml
# ~/.config/starship.toml

[username]
style_user = "white bold"
style_root = "black bold"
format = "user: [$user]($style) "
disabled = false
show_always = true
```

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `Vagrantfile`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.                                  |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Vagrantfile"]`                    | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"cyan bold"`                        | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                                       |

### Variáveis

| Variável  | Exemplo          | Descrição                          |
| --------- | ---------------- | ---------------------------------- |
| version   | `Vagrant 2.2.10` | A versão do `Vagrant`              |
| symbol    |                  | Espelha o valor da opção `símbolo` |
| style\* |                  | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual cotem qualquer arquivo com a extensão `.v`
- O diretório atual contem um arquivo `v.mod`, `vpkg.json` ou `.vpkg-lock.json`

### Opções

| Opções              | Padrão                                       | Descrição                                                                            |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`         | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                                  | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"V "`                                       | A format string representing the symbol of V                                         |
| `detect_extensions` | `["v"]`                                      | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                         | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"blue bold"`                                | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                                         |

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| version   | `v0.2`  | A versão do `v`                    |
| symbol    |         | Espelha o valor da opção `símbolo` |
| style\* |         | Espelha o valor da opção `style`   |

### Exemplo

```toml
# ~/.config/starship.toml
[vlang]
format = "via [V $version](blue bold) "
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository. The module will be shown only if a repository is currently in use.

### Opções

| Opções     | Padrão                           | Descrição                                              |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   |                                  | The symbol used before displaying the repository name. |
| `style`    | `"bold yellow"`                  | O estilo do módulo.                                    |
| `format`   | `"vcsh [$symbol$repo]($style) "` | O formato do módulo.                                   |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variáveis

| Variável  | Exemplo                                     | Descrição                          |
| --------- | ------------------------------------------- | ---------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | The active repository name         |
| symbol    |                                             | Espelha o valor da opção `símbolo` |
| style\* | `black bold dimmed`                         | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). The module will be shown if any of the following conditions are met:

- O diretório atual contém arquivo com a extensão `.zig`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig.                                |
| `style`             | `"bold yellow"`                      | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Disables the `zig` module.                                                           |
| `detect_extensions` | `["zig"]`                            | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v0.6.0` | A versão do `zig`                 |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Custom commands

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- O diretório atual tenha um arquivo cujo o nome esta em `files`
- O diretório atual tenha um diretório cujo o nome esta em `directories`
- O diretório atual tenha um arquivo com extensão que esteja em `extensions`
- O comando `when` retorna 0
- O sistema operacional (std::env::consts::OS) corresponde com o `os` se definido.

::: tip

Multiple custom modules can be defined by using a `.`.

:::

::: tip

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

::: warning Command output is printed unescaped to the prompt

Whatever output the command generates is printed unmodified in the prompt. This means if the output contains special sequences that are interpreted by your shell they will be expanded when displayed. These special sequences are shell specific, e.g. you can write a command module that writes bash sequences, e.g. `\h`, but this module will not work in a fish or zsh shell.

Format strings can also contain shell specific prompt sequences, e.g. [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

:::

### Opções

| Opções        | Padrão                          | Descrição                                                                                                                                                                     |
| ------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | `""`                            | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                 |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code.                                                    |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                                                                            |
| `description` | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                                                                                  |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                         |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                   |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                    |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                                                                         |
| `style`       | `"bold green"`                  | O estilo do módulo.                                                                                                                                                           |
| `format`      | `"[$symbol($output )]($style)"` | O formato do módulo.                                                                                                                                                          |
| `disabled`    | `false`                         | Disables this `custom` module.                                                                                                                                                |
| `os`          |                                 | Operating System name on which the module will be shown (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html). |

### Variáveis

| Variável  | Descrição                              |
| --------- | -------------------------------------- |
| output    | The output of shell command in `shell` |
| symbol    | Espelha o valor da opção `símbolo`     |
| style\* | Espelha o valor da opção `style`       |

*: This variable can only be used as a part of a style string

#### Comandos personalizados de shell

`shell` accepts a non-empty list of strings, where:

- A primeira string é o caminho para o shell que executará o comando.
- Outros argumentos que serão passados para o shell.

If unset, it will fallback to STARSHIP_SHELL and then to "sh" on Linux, and "cmd /C" on Windows.

The `command` will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used, the following arguments will automatically be added: `-NoProfile -Command -`. This behavior can be avoided by explicitly passing arguments to the shell, e.g.

```toml
shell = ["pwsh", "-Command", "-"]
```

::: warning Make sure your custom shell configuration exits gracefully

If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).

For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.

Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.

Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

:::

### Exemplo

```toml
# ~/.config/starship.toml

[custom.foo]
command = "echo foo" # shows output of command
files = ["foo"] # can specify filters but wildcards are not supported
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
extensions = ["pst"] # filters *.pst files
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
