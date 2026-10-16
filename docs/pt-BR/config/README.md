# Configuração

To get started configuring starship, create the following file: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

All configuration for starship is done in this [TOML](https://github.com/toml-lang/toml) file:

```toml
# Obtém o preenchimento do editor baseado no esquema de configuração
"$schema" = 'https://starship.rs/config-schema.json'

# Insere uma linha branca entre os prompts do shell
add_newline = true

# Substitui o símbolo '❯' no prompt por  '➜'
[character] # O nome do módulo que estamos configurando é  'character'
success_symbol = '[➜](bold green)' # O 'success_symbol' é definido para  '➜' com a cor 'bold green'

# Desabilita o módulo package, escondendo completamente ele do prompt
[package]
disabled = true
```

### Configuração do Local do Arquivo

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

### Terminologia

**Module**: A component in the prompt giving information based on contextual information from your OS. Por exemplo, o "nodejs"módulo exibe a versão do Node.js que está instalada no computador, se o diretório atual for um projeto Node.js.

**Variable**: Smaller sub-components that contain information provided by the module. Por exemplo, a variável "version" no módulo "nodejs"contem a versão atual do Node.js.

By convention, most modules have a prefix of default terminal color (e.g. `via` in "nodejs") and an empty space as a suffix.

### Strings

In TOML syntax, [text values](https://toml.io/en/v1.0.0#string) are declared with `'`, `"`, `'''`, or `"""`.

Os seguintes símbolos de sintaxe do Starship têm uso especial em uma string de formatação e devem ser escapados para exibir como este caractere: `$ [ ] ( )`.

| Símbolo | Tipo                       | Notas                                                            |
| ------- | -------------------------- | ---------------------------------------------------------------- |
| `'`     | string literal             | menos escapando                                                  |
| `"`     | string                     | mais escapando                                                   |
| `'''`   | string literal multi-linha | menos escapando                                                  |
| `"""`   | string multi-linha         | mais escapantes, novas linhas em declarações podem ser ignoradas |

Por exemplo:

```toml
# literal string
format = '☺\☻ '

# regular string
format = "☺\\☻ "

# escaping Starship symbols
format = '\[\$\] '
```

Ao usar quebras de linha, declarações de várias linhas podem ser usadas.
For example, if you want to print a `$` symbol on a new line, the following values for `format` are equivalent:

```toml
# com string literal
format = '''

\$'''

# com string básica multilinha 
format = """

\\$"""

# com string básica
format = "\n\\$"
```

Em strings básicas de várias linhas, newlines podem ser usadas para formatação sem estarem presentes no valor escapado delas.

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

### Formatação de Strings

As strings de formato são o formato com o qual um módulo imprime todas as suas variáveis.
Most modules have an entry called `format` that configures the display format of the module.
Você pode usar textos, variáveis e grupos de texto em uma string de formato.

#### Variável

A variable contains a `$` symbol followed by the name of the variable.
O nome de uma variável pode conter apenas letras, números e `_`.

Por exemplo:

- `'$version'` is a format string with a variable named `version`.
- `'$git_branch$git_commit'` is a format string with two variables named `git_branch` and `git_commit`.
- `'$git_branch $git_commit'` has the two variables separated with a space.

#### Grupo de Texto

Um grupo de texto é composto de duas partes diferentes.

The first part, which is enclosed in a `[]`, is a [format string](#format-strings).
Você pode adicionar textos, variáveis ou até mesmo grupos de texto aninhados nele.

In the second part, which is enclosed in a `()`, is a [style string](#style-strings). Isso pode ser usado para estilizar a primeira parte.

Por exemplo:

- `'[on](red bold)'` will print a string `on` with bold text colored red.
- `'[⌘ $version](bold green)'` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `'[a [b](red) c](green)'` will print `a b c` with `b` red, and `a` and `c` green.

#### Estilo dos textos

A maioria dos módulos no starship permite que você configure seus estilos de exibição. This is done with an entry (usually called `style`) which is a string specifying the configuration. Aqui estão alguns exemplos de strings de estilo junto com o que elas fazem. For details on the full syntax, consult the [advanced config guide](../advanced-config/).

- `'fg:green bg:blue'` sets green text on a blue background
- `'bg:blue fg:bright-green'` sets bright green text on a blue background
- `'bold fg:27'` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `'underline bg:#bf5700'` sets underlined text on a burnt orange background
- `'bold italic fg:purple'` sets bold italic purple text
- `''` explicitly disables all styling

Observe que a aparência do estilo será controlada pelo emulador de terminal. Por exemplo, alguns emuladores de terminal irão clarear as cores em vez de colocar o texto em negrito, e alguns temas de cores usam os mesmos valores para as cores normais e brilhantes. Além disso, para obter texto em itálico, seu terminal deve suportar itálico.

#### Formatação de String Condicional

A conditional format string wrapped in `(` and `)` will not render if all variables inside are empty.

Por exemplo:

- `'(@$region)'` will show nothing if the variable `region` is `None` or empty string, otherwise `@` followed by the value of region.
- `'(some text)'` will always show nothing since there are no variables wrapped in the braces.
- When `$combined` is a shortcut for `\[$a$b\]`, `'($combined)'` will show nothing only if `$a` and `$b` are both `None`.
  This works the same as `'(\[$a$b\] )'`.

### Correspondência negativa

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. Estas receberão
listas de strings para coresponder ou não. Opções "negativas", aquelas que não tem correspondencia, são
indicadas com um caractere  '!'. The presence of _any_ negative indicator in the directory
will result in the module not being matched.

As extensões são combinadas com os dois caracteres após o último ponto em um nome de arquivo e os caracteres
após o primeiro ponto em um nome de arquivo. For example, `foo.bar.tar.gz` will be matched
against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Arquivos que o nome começa com um ponto
não são considerados ter nenhuma extensão.

Para ver como isso funciona na prática, você pode combinar TypeScript mas não arquivos MPEG Transport Stream:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Prompt de Comando

Esta é a lista de opções de configuração em todo o prompt.

### Opções

| Opções            | Padrão                         | Descrição                                                                                                                                                                                                                       |
| ----------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | Configura o formato do prompt.                                                                                                                                                                                  |
| `right_format`    | `''`                           | See [Enable Right Prompt](../advanced-config/#enable-right-prompt)                                                                                                                                                              |
| `scan_timeout`    | `30`                           | Tempo limite para escanear arquivos (em milissegundos).                                                                                                                                      |
| `command_timeout` | `500`                          | Tempo limite de execução de comandos pelo starship (em milissegundos).                                                                                                                       |
| `add_newline`     | `true`                         | Insere linha vazia entre os prompts do shell.                                                                                                                                                                   |
| `palette`         | `''`                           | Sets which color palette from `palettes` to use.                                                                                                                                                                |
| `palettes`        | `{}`                           | Collection of color palettes that assign [colors](../advanced-config/#style-strings) to user-defined names. Note que paletas de cores não podem referir-se a suas próprias definições de cores. |
| `follow_symlinks` | `true`                         | Segue links simbólicos para verificar se são diretórios; usados em módulos como o git.                                                                                                                          |

> [!TIP]
> If you have symlinks to networked filesystems, consider setting
> `follow_symlinks` to `false`.

### Exemplo

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

### Format de Prompt Padrão

The default `format` is used to define the format of the prompt, if empty or no `format` is provided. O padrão é como mostrado:

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
$nats\
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
$bun\
$c\
$cmake\
$cobol\
$cpp\
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
$gradle\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$lua\
$maven\
$mojo\
$nim\
$nodejs\
$ocaml\
$odin\
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
$xmake\
$zig\
$buf\
$guix_shell\
$nix_shell\
$conda\
$pixi\
$meson\
$spack\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
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
$container\
$netns\
$os\
$shell\
$character"""
```

If you just want to extend the default format, you can use `$all`;
modules you explicitly add to the format will not be duplicated. Ex.

```toml
# Mova o diretório para a segunda linha
format = '$all$directory$character'
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials.
The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process`, `sso_start_url`, or `sso_session` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice.
If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

When using [aws-vault](https://github.com/99designs/aws-vault) the profile
is read from the `AWS_VAULT` env var and the credentials expiration date
is read from the `AWS_SESSION_EXPIRATION` env var.

When using [awsu](https://github.com/kreuzwerker/awsu) the profile
is read from the `AWSU_PROFILE` env var.

When using [AWSume](https://awsu.me) the profile
is read from the `AWSUME_PROFILE` env var and the credentials expiration
date is read from the `AWSUME_EXPIRATION` env var.

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials`
falls back to the `x_security_token_expires` key.

When using [aws-sso-cli](https://github.com/synfinatic/aws-sso-cli) the profile
is read from the `AWS_SSO_PROFILE` env var.

### Opções

| Opções              | Padrão                                                                | Descrição                                                                                                                   |
| ------------------- | --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | O formato do módulo.                                                                                        |
| `símbolo`           | `'☁️ '`                                                               | O símbolo usado antes de exibir o perfil atual da AWS.                                                      |
| `region_aliases`    | `{}`                                                                  | Tabela de aleases de regiões a serem exibidas, além do nome da AWS.                                         |
| `profile_aliases`   | `{}`                                                                  | Tabela de apelidos de perfil a serem exibidos além do nome da AWS.                                          |
| `style`             | `'bold yellow'`                                                       | O estilo do módulo.                                                                                         |
| `expiration_symbol` | `'X'`                                                                 | O simbolo exibido quando as credenciais temporárias estão expiradas.                                        |
| `disabled`          | `false`                                                               | Disables the `AWS` module.                                                                                  |
| `force_display`     | `false`                                                               | If `true` displays info even if `credentials`, `credential_process` or `sso_start_url` have not been setup. |

### Variáveis

| Variável | Exemplo          | Descrição                            |
| -------- | ---------------- | ------------------------------------ |
| region   | `ap-northeast-1` | A região atual do AWS                |
| profile  | `astronauts`     | O perfil atual do AWS                |
| duration | `2h27m20s`       | A duração temporária das credenciais |
| symbol   |                  | Espelha o valor da opção `symbol`    |
| style\*  |                  | Espelha o valor da opção `style`     |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplos

#### Exibir tudo

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\($region\) )]($style)'
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
[aws.profile_aliases]
CompanyGroupFrobozzOnCallAccess = 'Frobozz'
```

#### Exibir região

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$region]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
```

#### Exibir perfil

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$profile]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.profile_aliases]
Enterprise_Naming_Scheme-voidstars = 'void**'
```

## Azure

The `azure` module shows the current Azure Subscription. This is based on showing the name of the default subscription or the username, as defined in the `~/.azure/azureProfile.json` file.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Variável               | Padrão                                   | Descrição                                                                                             |
| ---------------------- | ---------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `format`               | `'on [$symbol($subscription)]($style) '` | O formato que o módulo Azure será renderizado.                                        |
| `símbolo`              | `'󰠅 '`                                  | O símbolo usado no formato.                                                           |
| `style`                | `'blue bold'`                            | O estilo usado no formato.                                                            |
| `disabled`             | `true`                                   | Disables the `azure` module.                                                          |
| `subscription_aliases` | `{}`                                     | Table of subscription name aliases to display in addition to Azure subscription name. |

### Exemplos

#### Exibir Nome da Assinatura

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = '󰠅 '
style = 'blue bold'
```

#### Exibir Usuário

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "󰠅 "
style = "blue bold"
```

#### Exibir Alias do Nome da Assinatura

```toml
# ~/.config/starship.toml

[azure.subscription_aliases]
very-long-subscription-name = 'vlsn'
```

## Bateria

The `battery` module shows how charged the device's battery is and its current charging status.
O módulo é visível somente quando a bateria está abaixo de 10%.

### Opções

| Opções               | Padrão                            | Descrição                                                                    |
| -------------------- | --------------------------------- | ---------------------------------------------------------------------------- |
| `full_symbol`        | `'󰁹 '`                           | O simbolo exibido quando a bateria estiver cheia.            |
| `charging_symbol`    | `'󰂄 '`                           | O simbolo exibido quando a bateria está carregando.          |
| `discharging_symbol` | `'󰂃 '`                           | O simbolo exibido quando a bateria está descarregando.       |
| `unknown_symbol`     | `'󰂑 '`                           | O simbolo exibido quando o estado da bateria é desconhecido. |
| `empty_symbol`       | `'󰂎 '`                           | O simbolo exibido quando o estado da bateria é vazio.        |
| `format`             | `'[$symbol$percentage]($style) '` | O formato do módulo.                                         |
| `display`            | [link](#battery-display)          | Limite de exibição e estilo para o módulo.                   |
| `disabled`           | `false`                           | Disables the `battery` module.                               |

### Exemplo

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### Indicador de bateria

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style).
If no `display` is provided. O padrão é como mostrado:

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

The default value for the `charging_symbol` and `discharging_symbol` option is respectively the value of `battery`'s `charging_symbol` and `discharging_symbol` option.

#### Opções

The `display` option is an array of the following table.

| Opções               | Padrão       | Descrição                                                                                                                 |
| -------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | O limite superior para exibição.                                                                          |
| `style`              | `'red bold'` | O estilo usado para exibir quando estiver em uso.                                                         |
| `charging_symbol`    |              | Optional symbol displayed if display option is in use, defaults to battery's `charging_symbol` option.    |
| `discharging_symbol` |              | Optional symbol displayed if display option is in use, defaults to battery's `discharging_symbol` option. |

#### Exemplo

```toml
[[battery.display]] # 'bold red' style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = 'bold red'

[[battery.display]] # 'bold yellow' style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦 '

# when capacity is over 30%, the battery indicator will not be displayed
```

## Buf

The `buf` module shows the currently installed version of [Buf](https://buf.build). By default, the module is shown if the current directory contains a [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), or [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) configuration file.

### Opções

| Opções              | Padrão                                          | Descrição                                                         |
| ------------------- | ----------------------------------------------- | ----------------------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | The format for the `buf` module.                  |
| `version_format`    | `'v${raw}'`                                     | A versão formatada.                               |
| `símbolo`           | `'🐃 '`                                         | O símbolo usado antes de exibir a versão do Buf.  |
| `detect_extensions` | `[]`                                            | Quais extensões devem ativar este módulo.         |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Quais nomes de arquivos devem ativar este módulo. |
| `detect_folders`    | `[]`                                            | Quais pastas devem ativar este módulo.            |
| `style`             | `'bold blue'`                                   | O estilo do módulo.                               |
| `disabled`          | `false`                                         | Disables the `elixir` module.                     |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| `version` | `v1.0.0` | The version of `buf`              |
| `símbolo` |          | Espelha o valor da opção `symbol` |
| `style`\* |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime.
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `bun.lock` file
- The current directory contains a `bun.lockb` file
- The current directory contains a `bunfig.toml` file

### Opções

| Opções              | Padrão                                     | Descrição                                                                                                     |
| ------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`       | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                                | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🥟 '`                                    | Uma string de formato que representa o símbolo do Bun.                                        |
| `detect_extensions` | `[]`                                       | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                       | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold red'`                               | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                    | Disables the `bun` module.                                                                    |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v0.1.4` | The version of `bun`              |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

#### Customize the format

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

The `c` module shows some information about your C compiler. By default
the module will be shown if the current directory contains a `.c` or `.h`
file.

### Opções

| Opções              | Padrão                                                                        | Descrição                                                                                                     |
| ------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | A string de formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                                                   | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'C '`                                                                        | O símbolo utilizado antes de exibir os detalhes do compilador                                                 |
| `detect_extensions` | `['c', 'h']`                                                                  | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                                                          | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                                                          | Quais pastas devem ativar este módulo.                                                        |
| `comandos`          | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | Como detectar qual é o compilador                                                                             |
| `style`             | `'bold 149'`                                                                  | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                                                       | Disables the `c` module.                                                                      |

### Variáveis

| Variável | Exemplo                                | Descrição                         |
| -------- | -------------------------------------- | --------------------------------- |
| name     | clang                                  | O nome do compilador              |
| version  | 13.0.0 | A versão do compilador            |
| symbol   |                                        | Espelha o valor da opção `symbol` |
| style    |                                        | Espelha o valor da opção `style`  |

### Comandos

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship tentará executar cada comando até que obtenha um resultado no STDOUT.

If a C compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Exemplo

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## CPP

The `cpp` module shows some information about your `C++` compiler. By default,
the module will be shown if the current directory contains a `.cpp`, `.hpp`, or other `C++`-related files.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções              | Padrão                                                                           | Descrição                                                                                                     |
| ------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                     | A string de formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                                                      | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'C++ '`                                                                         | O símbolo utilizado antes de exibir os detalhes do compilador                                                 |
| `detect_extensions` | `['cpp', 'cc', 'cxx', 'c++', 'hpp', 'hh', 'hxx', 'h++', 'tcc']`                  | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                                                             | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                                                             | Quais pastas devem ativar este módulo.                                                        |
| `comandos`          | `[ [ 'c++', '--version' ], [ 'g++', '--version' ], [ 'clang++', '--version' ] ]` | Como detectar qual é o compilador                                                                             |
| `style`             | `'bold 149'`                                                                     | O estilo do módulo.                                                                           |
| `disabled`          | `true`                                                                           | Disables the `cpp` module.                                                                    |

### Variáveis

| Variável | Exemplo                                | Descrição                         |
| -------- | -------------------------------------- | --------------------------------- |
| name     | clang++                                | O nome do compilador              |
| version  | 13.0.0 | A versão do compilador            |
| symbol   |                                        | Espelha o valor da opção `symbol` |
| style    |                                        | Espelha o valor da opção `style`  |

### Comandos

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycpp', '--version']`. Starship tentará executar cada comando até que obtenha um resultado no STDOUT.

If a C++ compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Exemplo

```toml
# ~/.config/starship.toml

[cpp]
disabled = false
format = 'via [$name $version]($style)'
```

## Caractere

The `character` module shows a character (usually an arrow) beside where the text
is entered in your terminal.

O caractere vai te dizer se o ultimo comando foi bem sucedido ou não. Você pode fazer isto de duas maneiras:

- changing color (`red`/`green`)
- changing shape (`❯`/`✖`)

Por padrão ele apenas muda de cor. If you also want to change its shape take a
look at [this example](#with-custom-error-shape).

> [!WARNING]
> `vimcmd_symbol` is only supported in cmd, fish and zsh.
> `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol`
> are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

### Opções

| Opções                      | Padrão               | Descrição                                                                                                   |
| --------------------------- | -------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | O formato da string usado antes da entrada dos textos.                                      |
| `success_symbol`            | `'[❯](bold green)'`  | O formato da string usado antes da entrada de texto se o comando anterior for bem-sucedido. |
| `error_symbol`              | `'[❯](bold red)'`    | O formato de string usado antes da entrada de texto se o comando anterior tiver falhado.    |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | O fromato de string usado antes da entrada de texto se o shell esta no vim normal mode.     |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim `replace_one` mode.     |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim replace mode.           |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | The format string used before the text input if the shell is in vim visual mode.            |
| `disabled`                  | `false`              | Disables the `character` module.                                                            |

### Variáveis

| Variável | Exemplo | Descrição                                                                                                                |
| -------- | ------- | ------------------------------------------------------------------------------------------------------------------------ |
| symbol   |         | A mirror of either `success_symbol`, `error_symbol`, `vimcmd_symbol` or `vimcmd_replace_one_symbol` etc. |

### Exemplos

#### Com formas customizadas de erro

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[✗](bold red) '
```

#### Sem formas customizadas de erro

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[➜](bold red) '
```

#### Com formas customizadas no vim

```toml
# ~/.config/starship.toml

[character]
vimcmd_symbol = '[V](bold green) '
```

## CMake

The `cmake` module shows the currently installed version of [CMake](https://cmake.org/). Por padrão o módulo será ativo se qualquer das condições a seguir for atendida:

- The current directory contains a `CMakeLists.txt` file
- The current directory contains a `CMakeCache.txt` file

### Opções

| Opções              | Padrão                                 | Descrição                                                                                                     |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`   | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                            | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'△ '`                                 | O simbolo usado antes da versão do cmake.                                                     |
| `detect_extensions` | `[]`                                   | Quais extensões devem acionar este módulo                                                                     |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | []                                                        |
| `detect_folders`    | `[]`                                   | Quais pastas devem ativar este módulo                                                                         |
| `style`             | `'bold blue'`                          | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                | Disables the `cmake` module.                                                                  |

### Variáveis

| Variável | Exemplo   | Descrição                         |
| -------- | --------- | --------------------------------- |
| version  | `v3.17.3` | A versão do cmake                 |
| symbol   |           | Espelha o valor da opção `symbol` |
| style\*  |           | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

## COBOL / GNUCOBOL

The `cobol` module shows the currently installed version of COBOL.
Por padrão, o módulo será exibido se qualquer das seguintes condições for atendida:

- The current directory contains any files ending in `.cob` or `.COB`
- The current directory contains any files ending in `.cbl` or `.CBL`

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `símbolo`           | `'⚙️ '`                              | O simbolo usado antes de exibir a versão do COBOL.                                            |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold blue'`                        | O estilo do módulo.                                                                           |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `disabled`          | `false`                              | Disables the `cobol` module.                                                                  |

### Variáveis

| Variável | Exemplo    | Descrição                         |
| -------- | ---------- | --------------------------------- |
| version  | `v3.1.2.0` | The version of `cobol`            |
| symbol   |            | Espelha o valor da opção `symbol` |
| style\*  |            | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

## Tempo de execução do comando

The `cmd_duration` module shows how long the last command took to execute.
The module will be shown only if the command took longer than two seconds, or
the `min_time` config value, if it exists.

> [!WARNING]
> Do not hook the DEBUG trap in Bash
>
> If you are running Starship in `bash`, do not hook the `DEBUG` trap after running
> `eval $(starship init $0)`, or this module **will** break.

Bash users who need preexec-like functionality can use
[rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec).
Simply define the arrays `preexec_functions` and `precmd_functions` before
running `eval $(starship init $0)`, and then proceed as normal.

### Opções

| Opções                 | Padrão                        | Descrição                                                                                                                                                                                                                                                              |
| ---------------------- | ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Duração mais curta para exibir o tempo (em milissegundos).                                                                                                                                                                          |
| `show_milliseconds`    | `false`                       | Exibir milissegundos ou invés de segundos para duração.                                                                                                                                                                                                |
| `format`               | `'took [$duration]($style) '` | O formato do módulo.                                                                                                                                                                                                                                   |
| `style`                | `'bold yellow'`               | O estilo do módulo.                                                                                                                                                                                                                                    |
| `disabled`             | `false`                       | Disables the `cmd_duration` module.                                                                                                                                                                                                                    |
| `show_notifications`   | `false`                       | Exibi notificações no desktop quando o comando for concluído.                                                                                                                                                                                          |
| `min_time_to_notify`   | `45_000`                      | Tempo minimo para notificação (em milissegundos).                                                                                                                                                                                   |
| `notification_timeout` |                               | Duração para mostrar a notificação (em milissegundos). Se não estiver definido, o tempo limite de notificação será determinado pelo daemon. Nem todos os daemons de notificação aceitam essa opção. |

### Variáveis

| Variável | Exemplo  | Descrição                                 |
| -------- | -------- | ----------------------------------------- |
| duration | `16m40s` | O tempo que levou para executar o comando |
| style\*  |          | Espelha o valor da opção `style`          |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

> [!TIP]
> This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`.
> If you use [pixi](https://pixi.sh), you can disable pixi's prompt modifier by running `pixi config set shell.change-ps1 false`.

### Opções

| Opções              | Padrão                                 | Descrição                                                                                                                                                                                                                                                   |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | The number of directories the environment path should be truncated to, if the environment was created via `conda create -p [path]`. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `símbolo`           | `'🅒 '`                                | O simbolo usado antes do nome do environment.                                                                                                                                                                                               |
| `style`             | `'bold green'`                         | O estilo do módulo.                                                                                                                                                                                                                         |
| `format`            | `'via [$symbol$environment]($style) '` | O formato do módulo.                                                                                                                                                                                                                        |
| `ignore_base`       | `true`                                 | Ignores `base` environment when activated.                                                                                                                                                                                                  |
| `detect_env_vars`   | `["!PIXI_ENVIRONMENT_NAME"]`           | Which environment variable(s) should trigger this module. If it's a pixi environment, this module is not being triggered by default.                                                                     |
| `disabled`          | `false`                                | Disables the `conda` module.                                                                                                                                                                                                                |

### Variáveis

| Variável    | Exemplo      | Descrição                         |
| ----------- | ------------ | --------------------------------- |
| environment | `astronauts` | O environment atual do conda      |
| symbol      |              | Espelha o valor da opção `symbol` |
| style\*     |              | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## Container

The `container` module displays a symbol and container name, if inside a container.

### Opções

| Opções     | Padrão                             | Descrição                                         |
| ---------- | ---------------------------------- | ------------------------------------------------- |
| `símbolo`  | `'⬢'`                              | O símbolo mostrado, quando dentro de um contêiner |
| `style`    | `'bold red dimmed'`                | O estilo do módulo.               |
| `format`   | `'[$symbol \[$name\]]($style) '` | O formato do módulo.              |
| `disabled` | `false`                            | Disables the `container` module.  |

### Variáveis

| Variável | Exemplo             | Descrição                         |
| -------- | ------------------- | --------------------------------- |
| name     | `fedora-toolbox:35` | O nome do contêiner               |
| symbol   |                     | Espelha o valor da opção `symbol` |
| style\*  |                     | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `shard.yml` file
- The current directory contains a `.cr` file

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `símbolo`           | `'🔮 '`                              | O símbolo usado antes de exibir a versão do crystal.                                          |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | O estilo do módulo.                                                                           |
| `detect_extensions` | `['cr']`                             | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['shard.yml']`                      | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `disabled`          | `false`                              | Disables the `crystal` module.                                                                |

### Variáveis

| Variável | Exemplo   | Descrição                         |
| -------- | --------- | --------------------------------- |
| version  | `v0.32.1` | The version of `crystal`          |
| symbol   |           | Espelha o valor da opção `symbol` |
| style\*  |           | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers)
SDK version when you are in the root directory of your Daml project. The `sdk-version` in
the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION`
environment variable.
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `daml.yaml` file

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'Λ '`                               | A format string representing the symbol of Daml                                                               |
| `style`             | `'bold cyan'`                        | O estilo do módulo.                                                                           |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['daml.yaml']`                      | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `disabled`          | `false`                              | Disables the `daml` module.                                                                   |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v2.2.0` | The version of `daml`             |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

The `dart` module shows the currently installed version of [Dart](https://dart.dev/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a file with `.dart` extension
- The current directory contains a `.dart_tool` directory
- The current directory contains a `pubspec.yaml`, `pubspec.yml` or `pubspec.lock` file

### Opções

| Opções              | Padrão                                            | Descrição                                                                                                     |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`              | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                                       | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🎯 '`                                           | Um formato de string que representa o simbolo do Dart                                                         |
| `detect_extensions` | `['dart']`                                        | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `['.dart_tool']`                                  | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold blue'`                                     | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                           | Disables the `dart` module.                                                                   |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v2.8.4` | The version of `dart`             |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

The `deno` module shows you your currently installed version of [Deno](https://deno.land/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### Opções

| Opções              | Padrão                                                                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                                                                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🦕 '`                                                                              | Um formato de string que representa o simbolo do Deno                                                         |
| `detect_extensions` | `[]`                                                                                 | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                                                                 | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'green bold'`                                                                       | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                                                              | Disables the `deno` module.                                                                   |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v1.8.3` | The version of `deno`             |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Diretório

The `directory` module shows the path to your current directory, truncated to
three parent folders. Seu diretório será truncando na raiz do repositório git que você estiver atualmente.

When using the `fish_style_pwd_dir_length` option, instead of hiding the path that is
truncated, you will see a shortened name of each directory based on the number
you enable for the option.

For example, given `~/Dev/Nix/nixpkgs/pkgs` where `nixpkgs` is the repo root,
and the option set to `1`. You will now see `~/D/N/nixpkgs/pkgs`, whereas before
it would have been `nixpkgs/pkgs`.

### Opções

| Opções                   | Padrão                                                                                                                       | Descrição                                                                                                                                  |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `truncation_length`      | `3`                                                                                                                          | O número de pastas pais do diretório atual que serão truncadas.                                                            |
| `truncate_to_repo`       | `true`                                                                                                                       | Seu diretório será truncado ou não para a raiz do repositório git atual.                                                   |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | O formato do módulo.                                                                                                       |
| `style`                  | `'bold cyan'`                                                                                                                | O estilo do módulo.                                                                                                        |
| `disabled`               | `false`                                                                                                                      | Disables the `directory` module.                                                                                           |
| `read_only`              | `'🔒'`                                                                                                                       | O simbolo que indica que o diretório atual é somente leitura.                                                              |
| `read_only_style`        | `'red'`                                                                                                                      | O estilo para o simbolo de somente leitura.                                                                                |
| `truncation_symbol`      | `''`                                                                                                                         | O simbolo para prefixo de caminhos truncados. eg: '…/'                                                     |
| `before_repo_root_style` |                                                                                                                              | The style for the path segment above the root of the git repo. The default value is equivalent to `style`. |
| `repo_root_style`        |                                                                                                                              | O estilo para a raiz do repositório git. The default value is equivalent to `style`.                       |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | The format of a git repo when `before_repo_root_style` and `repo_root_style` is defined.                                   |
| `home_symbol`            | `'~'`                                                                                                                        | O simbolo para indicar o diretório home.                                                                                   |
| `use_os_path_sep`        | `true`                                                                                                                       | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows)   |

<details><summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Opções Avançadas            | Padrão | Descrição                                                                                                                                                                                              |
| --------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `substitutions`             |        | An Array or table of substitutions to be made to the path.                                                                                                                             |
| `fish_style_pwd_dir_length` | `0`    | O número de caracteres para usar quando aplicado no path logico do fish shell pwd.                                                                                                     |
| `use_logical_path`          | `true` | If `true` render the logical path sourced from the shell via `PWD` or `--logical-path`. If `false` instead render the physical filesystem path with symlinks resolved. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network
prefixes or development directories of Java. Note isto irá desabilita o estilo PWD do fish. It takes an array of the following
key/value pairs:

| Value   | Tipo    | Descrição                                               |
| ------- | ------- | ------------------------------------------------------- |
| `from`  | String  | The value to substitute                                 |
| `to`    | String  | The replacement for that value, if found                |
| `regex` | Boolean | (Optional) Whether `from` is a regex |

By using `regex = true`, you can use [Rust's regular expressions](https://docs.rs/regex/latest/regex/#syntax) in `from`.
For instance you can replace every slash except the first with the following:

```toml
substitutions = [
  { from = "^/", to = "<root>/", regex = true },
  { from = "/", to = " | " },
  { from = "^<root>", to = "/", regex = true },
]
```

This will replace `/var/log` to `/ | var | log`.

The old syntax still works, although it doesn't support regular expressions:

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length` interacts with the standard truncation options in a way that can be surprising at first: if it's non-zero,
the components of the path that would normally be truncated are instead displayed with that many characters. For example, the path
`/built/this/city/on/rock/and/roll`, which would normally be displayed as `rock/and/roll`, would be displayed as
`/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with
a single character. For `fish_style_pwd_dir_length = 2`, it would be `/bu/th/ci/on/rock/and/roll`.

</details>

### Variáveis

| Variável | Exemplo               | Descrição                        |
| -------- | --------------------- | -------------------------------- |
| path     | `'D:/Projetos'`       | O caminho do diretório atual     |
| style\*  | `'black bold dimmed'` | Espelha o valor da opção `style` |

\*: Esta variável só pode ser usada como parte de uma string de estilo

<details><summary>The git repos have additional variables.</summary>

Let us consider the path `/path/to/home/git_repo/src/lib`

| Variável                                                   | Exemplo                 | Descrição                                           |
| ---------------------------------------------------------- | ----------------------- | --------------------------------------------------- |
| before_root_path | `'/caminho/para/home/'` | O caminho antes do caminho do diretório raiz do git |
| repo_root                             | `'git_repo'`            | O nome do diretório raiz do git                     |
| path                                                       | `'/src/lib'`            | O caminho restante                                  |
| style                                                      | `'black bold dimmed'`   | Espelha o valor da opção `style`                    |
| repo_root_style  | `'underline white'`     | Estilo para o nome do diretório raiz do git         |

</details>

### Exemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Direnv

The `direnv` module shows the status of the current rc file if one is present. The status includes the path to the rc file, whether it is loaded, and whether it has been allowed by `direnv`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções              | Padrão                                 | Descrição                                                                                  |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------ |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | O formato do módulo.                                                       |
| `símbolo`           | `'direnv '`                            | The symbol used before displaying the direnv context.                      |
| `style`             | `'bold orange'`                        | O estilo do módulo.                                                        |
| `disabled`          | `true`                                 | Disables the `direnv` module.                                              |
| `detect_extensions` | `[]`                                   | Quais extensões devem ativar este módulo.                                  |
| `detect_files`      | `['.envrc']`                           | Quais nomes de arquivos devem ativar este módulo.                          |
| `detect_folders`    | `[]`                                   | Quais pastas devem ativar este módulo.                                     |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | Quais variáveis de ambiente devem ativar este módulo.                      |
| `allowed_msg`       | `'allowed'`                            | The message displayed when an rc file is allowed.                          |
| `not_allowed_msg`   | `'not allowed'`                        | The message displayed when an rc file is not_allowed. |
| `denied_msg`        | `'denied'`                             | The message displayed when an rc file is denied.                           |
| `loaded_msg`        | `'loaded'`                             | The message displayed when an rc file is loaded.                           |
| `unloaded_msg`      | `'not loaded'`                         | The message displayed when an rc file is not loaded.                       |

### Variáveis

| Variável                     | Exemplo             | Descrição                                               |
| ---------------------------- | ------------------- | ------------------------------------------------------- |
| loaded                       | `loaded`            | Whether the current rc file is loaded.  |
| allowed                      | `denied`            | Whether the current rc file is allowed. |
| rc_path | `/home/test/.envrc` | The current rc file path.               |
| symbol                       |                     | Espelha o valor da opção `symbol`.      |
| style\*                      | `red bold`          | Espelha o valor da opção `style`.       |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```

## Docker Context

The `docker_context` module shows the currently active
[Docker context](https://docs.docker.com/engine/context/working-with-contexts/)
if it's not set to `default` or `desktop-linux`, or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or
`DOCKER_CONTEXT` environment variables are set (as they are meant to override
the context in use).

### Opções

| Opções              | Padrão                                                                                       | Descrição                                                                                                            |
| ------------------- | -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$context]($style) '`                                                           | O formato do módulo.                                                                                 |
| `símbolo`           | `'🐳 '`                                                                                      | O simbolo usado antes de exibir a versão do contexto docker.                                         |
| `only_with_files`   | `true`                                                                                       | Exibe somente quando houver um arquivo                                                                               |
| `detect_extensions` | `[]`                                                                                         | Which extensions should trigger this module (needs `only_with_files` to be true). |
| `detect_files`      | `['compose.yml', 'compose.yaml', 'docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Which filenames should trigger this module (needs `only_with_files` to be true).  |
| `detect_folders`    | `[]`                                                                                         | Which folders should trigger this module (needs `only_with_files` to be true).    |
| `style`             | `'blue bold'`                                                                                | O estilo do módulo.                                                                                  |
| `disabled`          | `false`                                                                                      | Disables the `docker_context` module.                                                                |

### Variáveis

| Variável | Exemplo        | Descrição                         |
| -------- | -------------- | --------------------------------- |
| context  | `test_context` | O contexto atual do docker        |
| symbol   |                | Espelha o valor da opção `symbol` |
| style\*  |                | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

The `dotnet` module shows the relevant version of the [.NET Core SDK](https://dotnet.microsoft.com/) for the current directory. Se o SDK foi predefinido no diretório atual, a versão será exibida. Caso contrário, o módulo
exibe a versão mais recente instalada do SDK.

Por padrão, este módulo só será exibido no seu prompt quando um ou mais dos
seguintes arquivos estiverem presentes no diretório atual:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

Você também precisará do SDK do .NET Core instalado para usá-lo corretamente.

Internamente, este módulo usa um mecanismo próprio para detecção da versão. Typically it is twice as fast
as running `dotnet --version`, but it may show an incorrect version if your .NET project has an
unusual directory layout. If accuracy is more important than speed, you can disable the mechanism by
setting `heuristic = false` in the module options.

The module will also show the Target Framework Moniker
(<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>)
when there is a `.csproj` file in the current directory.

### Opções

| Opções              | Padrão                                                                                                  | Descrição                                                                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                          | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                                                                                             | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'.NET '`                                                                                               | O símbolo usado na frente da versão do dotnet.                                                |
| `heuristic`         | `true`                                                                                                  | Usa a detecção de versão rápida para manter o starship ligeiro e hábil.                       |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                                                                                    | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold blue'`                                                                                           | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                                                                                 | Disables the `dotnet` module.                                                                 |

### Variáveis

| Variável | Exemplo          | Descrição                                         |
| -------- | ---------------- | ------------------------------------------------- |
| version  | `v3.1.201`       | The version of `dotnet` sdk                       |
| tfm      | `netstandard2.0` | O Target Framework Moniker usado no projeto atual |
| symbol   |                  | Espelha o valor da opção `symbol`                 |
| style\*  |                  | Espelha o valor da opção `style`                  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

The `elixir` module shows the currently installed version of [Elixir](https://elixir-lang.org/) and [Erlang/OTP](https://erlang.org/doc/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `mix.exs` file.

### Opções

| Opções              | Padrão                                                      | Descrição                                                                                                     |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | O formato do módulo elixir.                                                                   |
| `version_format`    | `'v${raw}'`                                                 | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'💧 '`                                                     | O símbolo usado na frente da versão do Elixir ou Erlang.                                      |
| `detect_extensions` | `[]`                                                        | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['mix.exs']`                                               | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                                        | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold purple'`                                             | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                                     | Disables the `elixir` module.                                                                 |

### Variáveis

| Variável                         | Exemplo | Descrição                         |
| -------------------------------- | ------- | --------------------------------- |
| version                          | `v1.10` | The version of `elixir`           |
| otp_version |         | The otp version of `elixir`       |
| symbol                           |         | Espelha o valor da opção `symbol` |
| style\*                          |         | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

The `elm` module shows the currently installed version of [Elm](https://elm-lang.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `elm.json` file
- The current directory contains a `elm-package.json` file
- The current directory contains a `.elm-version` file
- The current directory contains a `elm-stuff` folder
- The current directory contains `*.elm` files

### Opções

| Opções              | Padrão                                             | Descrição                                                                                                     |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`               | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                                        | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🌳 '`                                            | O formato de string que representa o simbolo do Elm.                                          |
| `detect_extensions` | `['elm']`                                          | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `['elm-stuff']`                                    | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'cyan bold'`                                      | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                            | Disables the `elm` module.                                                                    |

### Variáveis

| Variável | Exemplo   | Descrição                         |
| -------- | --------- | --------------------------------- |
| version  | `v0.19.1` | The version of `elm`              |
| symbol   |           | Espelha o valor da opção `symbol` |
| style\*  |           | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## Variáveis de Ambiente

The `env_var` module displays the current value of a selected environment variables.
O módulo vai exibir somente se algumas das condições a seguir for atendida:

- The `variable` configuration option matches an existing environment variable
- The `variable` configuration option is not defined, but the `default` configuration option is

> [!TIP]
> The order in which env_var modules are shown can be individually set by including
> `${env_var.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`).
> By default, the `env_var` module will simply show all env_var modules in the order they were defined.

> [!TIP]
> Multiple environmental variables can be displayed by using a `.`. (see example)
> If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.
>
> Exemplo: a configuração a seguir irá mostrar o valor da variável de ambiente USER
>
> ```toml
> # ~/.config/starship.toml
>
> [env_var.USER]
> default = 'unknown user'
> ```

### Opções

| Opções      | Padrão                                | Descrição                                                                                    |
| ----------- | ------------------------------------- | -------------------------------------------------------------------------------------------- |
| `símbolo`   | `""`                                  | O símbolo usado antes de exibir o valor da variável.                         |
| `variáveis` |                                       | A variável de ambiente a ser exibida.                                        |
| `padrão`    |                                       | O valor padrão a ser exibido quando a variável selecionada não for definida. |
| `format`    | `"with [$symbol$env_value]($style) "` | O formato do módulo.                                                         |
| `descrição` | `"<env_var module>"`                  | The description of the module that is shown when running `starship explain`. |
| `disabled`  | `false`                               | Disables the `env_var` module.                                               |
| `style`     | `"black bold dimmed"`                 | O estilo do módulo.                                                          |

### Variáveis

| Variável                       | Exemplo                                                        | Descrição                                  |
| ------------------------------ | -------------------------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | The environment value of option `variable` |
| symbol                         |                                                                | Espelha o valor da opção `symbol`          |
| style\*                        |                                                                | Espelha o valor da opção `style`           |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = 'SHELL'
default = 'unknown shell'
```

Exibindo múltiplas variáveis de ambiente:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = 'SHELL'
default = 'unknown shell'
[env_var.USER]
default = 'unknown user'
```

## Erlang

The `erlang` module shows the currently installed version of [Erlang/OTP](https://erlang.org/doc/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `rebar.config` file.
- The current directory contains a `erlang.mk` file.

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `' '`                               | O símbolo usado antes de exibir a versão do erlang.                                           |
| `style`             | `'bold red'`                         | O estilo do módulo.                                                                           |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `disabled`          | `false`                              | Disables the `erlang` module.                                                                 |

### Variáveis

| Variável | Exemplo   | Descrição                         |
| -------- | --------- | --------------------------------- |
| version  | `v22.1.3` | The version of `erlang`           |
| symbol   |           | Espelha o valor da opção `symbol` |
| style\*  |           | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Fennel

The `fennel` module shows the currently installed version of [Fennel](https://fennel-lang.org).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a file with the `.fnl` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🧅 '`                              | The symbol used before displaying the version of fennel.                                      |
| `style`             | `'bold green'`                       | O estilo do módulo.                                                                           |
| `detect_extensions` | `['fnl']`                            | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `disabled`          | `false`                              | Disables the `fennel` module.                                                                 |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v1.2.1` | The version of `fennel`           |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```

## Fill

The `fill` module fills any extra space on the line with a symbol. If multiple `fill` modules are
present in a line they will split the space evenly between them. Isto é útil para alinhar outros módulos.

### Opções

| Opções     | Padrão         | Descrição                                               |
| ---------- | -------------- | ------------------------------------------------------- |
| `símbolo`  | `'.'`          | O simbolo usado para preencher a linha. |
| `style`    | `'bold black'` | O estilo do módulo.                     |
| `disabled` | `false`        | Disables the `fill` module                              |

### Exemplo

```toml
# ~/.config/starship.toml
format = 'AA $fill BB $fill CC'

[fill]
symbol = '-'
style = 'bold green'
```

Produz um prompt parecido com:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Fortran

The `fortran` module shows the current compiler version of Fortran.

### Opções

| Opções              | Padrão                                                                                                                      | Descrição                                                                                                     |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `símbolo`           | `' '`                                                                                                                      | The symbol used before displaying the version of Fortran.                                     |
| `format`            | `'via [$symbol($version )]($style)'`                                                                                        | O formato do módulo.                                                                          |
| `version_format`    | `'${raw}'`                                                                                                                  | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold purple'`                                                                                                             | O estilo do módulo.                                                                           |
| `detect_extensions` | `['f', 'F', 'for', 'FOR', 'ftn', 'FTN', 'f77', 'F77', 'f90', 'F90', 'f95', 'F95','f03', 'F03', 'f08', 'F08', 'f18', 'F18']` | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['fpm.toml']`                                                                                                              | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                                                                                                        | Quais pastas devem ativar este módulo.                                                        |
| `comandos`          | `[ [ 'gfortran', '--version' ], [ 'flang', '--version' ], [ 'flang-new', '--version' ] ]`                                   | Como detectar qual é o compilador                                                                             |
| `disabled`          | `false`                                                                                                                     | Disables the `fortran` module.                                                                |

### Variáveis

| Variável | Exemplo  | Descrição                           |
| -------- | -------- | ----------------------------------- |
| name     | gfortran | O nome do compilador                |
| version  | `14.2.0` | The version of the Fortran compiler |
| symbol   |          | Espelha o valor da opção `symbol`   |
| style\*  |          | Espelha o valor da opção `style`    |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Comandos

The `commands` option accepts a list of commands to determine the compiler version and name.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['myfortran', '--version']`. Starship tentará executar cada comando até que obtenha um resultado no STDOUT.

If a Fortran compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/).

## Fossil Branch

The `fossil_branch` module shows the name of the active branch of the check-out in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções              | Padrão                           | Descrição                                                                                                                   |
| ------------------- | -------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | O formato do módulo. Use `'$branch'` to refer to the current branch name.                   |
| `símbolo`           | `' '`                           | The symbol used before the branch name of the check-out in your current directory.                          |
| `style`             | `'bold purple'`                  | O estilo do módulo.                                                                                         |
| `truncation_length` | `2^63 - 1`                       | Truncates a Fossil branch name to `N` graphemes                                                                             |
| `truncation_symbol` | `'…'`                            | O simbolo usado para indicar que o nome braço foi truncado. You can use `''` for no symbol. |
| `disabled`          | `true`                           | Disables the `fossil_branch` module.                                                                        |

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| branch   | `trunk` | The active Fossil branch          |
| symbol   |         | Espelha o valor da opção `symbol` |
| style\*  |         | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```

## Fossil Metrics

The `fossil_metrics` module will show the number of added and deleted lines in the check-out in your current directory. At least v2.14 (2021-01-20) of Fossil is required.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções               | Padrão                                                       | Descrição                                                   |
| -------------------- | ------------------------------------------------------------ | ----------------------------------------------------------- |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | O formato do módulo.                        |
| `added_style`        | `'bold green'`                                               | O estilo para a contagem de adições.        |
| `deleted_style`      | `'bold red'`                                                 | O estilo para a contagem de exclusões.      |
| `only_nonzero_diffs` | `true`                                                       | Exibe apenas o status para itens alterados. |
| `disabled`           | `true`                                                       | Disables the `fossil_metrics` module.       |

### Variáveis

| Variável                             | Exemplo | Descrição                                   |
| ------------------------------------ | ------- | ------------------------------------------- |
| added                                | `1`     | O número atual de linhas adicionadas        |
| deleted                              | `2`     | O número atual de linhas excluidas          |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[fossil_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI.
This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.
The `CLOUDSDK_CORE_PROJECT` and `CLOUDSDK_COMPUTE_REGION` environment variables, when set, override the `project` and `region` values from the active configuration, mirroring the behavior of `gcloud` itself.

When the module is enabled it will always be active, unless `detect_env_vars` has
been set in which case the module will only be active when one of the
environment variables has been set.

### Opções

| Opções            | Padrão                                                     | Descrição                                                                           |
| ----------------- | ---------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | O formato do módulo.                                                |
| `símbolo`         | `'☁️  '`                                                   | O simbolo usado antes de exibir o perfil atual do GCP.              |
| `region_aliases`  | `{}`                                                       | Tabela de aliases de região para exibir além do nome do GCP.        |
| `project_aliases` | `{}`                                                       | Tabela de apelidos do projeto a serem exibidos além do nome do GCP. |
| `detect_env_vars` | `[]`                                                       | Which environmental variables should trigger this module                            |
| `style`           | `'bold blue'`                                              | O estilo do módulo.                                                 |
| `disabled`        | `false`                                                    | Disables the `gcloud` module.                                       |

### Variáveis

| Variável | Exemplo       | Descrição                                                          |
| -------- | ------------- | ------------------------------------------------------------------ |
| region   | `us-central1` | A região atual do GCP                                              |
| account  | `foo`         | O perfil atual do GCP                                              |
| domain   | `example.com` | O perfil de domínio atual do GCP                                   |
| project  |               | O projeto atual do GCP                                             |
| active   | `padrão`      | The active config name written in `~/.config/gcloud/active_config` |
| symbol   |               | Espelha o valor da opção `symbol`                                  |
| style\*  |               | Espelha o valor da opção `style`                                   |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplos

#### Exibe conta e projeto

```toml
# ~/.config/starship.toml

[gcloud]
format = 'em [$symbol$account(@$domain)(\($project\))]($style) '
```

#### Exibe apenas o nome da configuração ativa

```toml
# ~/.config/starship.toml

[gcloud]
format = '[$symbol$active]($style) '
style = 'bold yellow'
```

#### Exibir conta e região

```toml
# ~/.config/starship.toml

[gcloud]
symbol = '️🇬️ '
[gcloud.region_aliases]
us-central1 = 'uc1'
asia-northeast1 = 'an1'
```

#### Exibir conta e projeto apelidado

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = 'vlpn'
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Opções

| Opções               | Padrão                                            | Descrição                                                                                                                   |
| -------------------- | ------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Exibe o nome do braço remoto, mesmo se ele for igual ao nome do braço local.                                |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | O formato do módulo. Use `'$branch'` to refer to the current branch name.                   |
| `símbolo`            | `' '`                                            | Um formato de string que representa o simbolo do git branch.                                                |
| `style`              | `'bold purple'`                                   | O estilo do módulo.                                                                                         |
| `truncation_length`  | `2^63 - 1`                                        | Truncates a git branch to `N` graphemes.                                                                    |
| `truncation_symbol`  | `'…'`                                             | O simbolo usado para indicar que o nome braço foi truncado. You can use `''` for no symbol. |
| `only_attached`      | `false`                                           | Only show the branch name when not in a detached `HEAD` state.                                              |
| `ignore_branches`    | `[]`                                              | Uma lista de nomes para evitar a exibição. Useful for 'master' or 'main'.                   |
| `ignore_bare_repo`   | `false`                                           | Do not show when in a bare repo.                                                                            |
| `disabled`           | `false`                                           | Disables the `git_branch` module.                                                                           |

### Variáveis

| Variável                           | Exemplo  | Descrição                                                                                                                                                                 |
| ---------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| branch                             | `master` | The current branch name, falls back to `HEAD` if there's no current branch (e.g. git detached `HEAD`). |
| remote_name   | `origin` | O nome do remoto.                                                                                                                                         |
| remote_branch | `master` | The name of the branch tracked on `remote_name`.                                                                                                          |
| symbol                             |          | Espelha o valor da opção `symbol`                                                                                                                                         |
| style\*                            |          | Espelha o valor da opção `style`                                                                                                                                          |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[git_branch]
symbol = '🌱 '
truncation_length = 4
truncation_symbol = ''
ignore_branches = ['master', 'main']
```

## Git commit

The `git_commit` module shows the current commit hash and also the tag (if any) of the repo in your current directory.

### Opções

| Opções               | Padrão                         | Descrição                                                                                                            |
| -------------------- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| `commit_hash_length` | `7`                            | O tamanho do git commit hash para ser exibido.                                                       |
| `format`             | `'[\($hash$tag\)]($style) '` | O formato do módulo.                                                                                 |
| `style`              | `'bold green'`                 | O estilo do módulo.                                                                                  |
| `only_detached`      | `true`                         | Only show git commit hash when in detached `HEAD` state                                                              |
| `tag_disabled`       | `true`                         | Disables showing tag info in `git_commit` module.                                                    |
| `tag_max_candidates` | `0`                            | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `' 🏷  '`                      | Simbolo da tag prefixado na informação a ser exibida                                                                 |
| `disabled`           | `false`                        | Disables the `git_commit` module.                                                                    |

### Variáveis

| Variável | Exemplo   | Descrição                                                    |
| -------- | --------- | ------------------------------------------------------------ |
| hash     | `b703eb3` | A hash atual do git commit                                   |
| tag      | `v1.0.0`  | The tag name if showing tag info is enabled. |
| style\*  |           | Espelha o valor da opção `style`                             |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## Git State

The `git_state` module will show in directories which are part of a git
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc. Se houver informação de progresso (e.x: REBASING 3/10). esta informação será exibida também.

### Opções

| Opções         | Padrão                                                          | Descrição                                                                                                       |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                    | A format string displayed when a `rebase` is in progress.                                       |
| `merge`        | `'MERGING'`                                                     | A format string displayed when a `merge` is in progress.                                        |
| `revert`       | `'REVERTING'`                                                   | A format string displayed when a `revert` is in progress.                                       |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | A format string displayed when a `cherry-pick` is in progress.                                  |
| `bisect`       | `'BISECTING'`                                                   | A format string displayed when a `bisect` is in progress.                                       |
| `am`           | `'AM'`                                                          | A format string displayed when an `apply-mailbox` (`git am`) is in progress. |
| `am_or_rebase` | `'AM/REBASE'`                                                   | A format string displayed when an ambiguous `apply-mailbox` or `rebase` is in progress.         |
| `style`        | `'bold yellow'`                                                 | O estilo do módulo.                                                                             |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | O formato do módulo.                                                                            |
| `disabled`     | `false`                                                         | Disables the `git_state` module.                                                                |

### Variáveis

| Variável                              | Exemplo    | Descrição                              |
| ------------------------------------- | ---------- | -------------------------------------- |
| state                                 | `REBASING` | O estado atual do repo                 |
| progress_current | `1`        | O progresso da operação atual          |
| progress_total   | `2`        | O total do progresso da operação atual |
| style\*                               |            | Espelha o valor da opção `style`       |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Git Metrics

The `git_metrics` module will show the number of added and deleted lines in
the current git repository.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções               | Padrão                                                       | Descrição                                                   |
| -------------------- | ------------------------------------------------------------ | ----------------------------------------------------------- |
| `added_style`        | `'bold green'`                                               | O estilo para a contagem de adições.        |
| `deleted_style`      | `'bold red'`                                                 | O estilo para a contagem de exclusões.      |
| `only_nonzero_diffs` | `true`                                                       | Exibe apenas o status para itens alterados. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | O formato do módulo.                        |
| `disabled`           | `true`                                                       | Disables the `git_metrics` module.          |
| `ignore_submodules`  | `false`                                                      | Ignora as alterações de submódulos                          |

### Variáveis

| Variável                             | Exemplo | Descrição                                   |
| ------------------------------------ | ------- | ------------------------------------------- |
| added                                | `1`     | O número atual de linhas adicionadas        |
| deleted                              | `2`     | O número atual de linhas excluidas          |
| added_style\*   |         | Mirrors the value of option `added_style`   |
| deleted_style\* |         | Mirrors the value of option `deleted_style` |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git Status

The `git_status` module shows symbols representing the state of the repo in your
current directory.

> [!TIP]
> The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment.
> You can disable the module or use the `windows_starship` option to use a Windows-native Starship executable to compute `git_status` for those paths.

### Opções

| Opções                 | Padrão                                          | Descrição                                                                                                                                      |
| ---------------------- | ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`               | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status`                                                                                                            |
| `conflicted`           | `'='`                                           | The format shown when this branch has merge conflicts.                                                                         |
| `ahead`                | `'⇡'`                                           | The format shown when this branch is ahead of the branch being tracked.                                                        |
| `behind`               | `'⇣'`                                           | The format shown when this branch is behind the branch being tracked.                                                          |
| `diverged`             | `'⇕'`                                           | The format shown when this branch has diverged from the branch being tracked.                                                  |
| `up_to_date`           | `''`                                            | The format shown when this branch is up to date with the branch being tracked.                                                 |
| `untracked`            | `'?'`                                           | The format shown when there are untracked files in the working directory.                                                      |
| `stashed`              | `'\$'`                                         | The format shown when a stash exists for the local repository.                                                                 |
| `modified`             | `'!'`                                           | The format shown when there are file modifications in the working directory.                                                   |
| `staged`               | `'+'`                                           | The format shown when a new file has been added to the staging area.                                                           |
| `renamed`              | `'»'`                                           | The format shown when a renamed file has been added to the staging area.                                                       |
| `deleted`              | `'✘'`                                           | The format shown when a file's deletion has been added to the staging area.                                                    |
| `typechanged`          | `""`                                            | The format shown when a file's type has been changed in the staging area.                                                      |
| `style`                | `'bold red'`                                    | O estilo do módulo.                                                                                                            |
| `ignore_submodules`    | `false`                                         | Ignora as alterações de submódulos.                                                                                            |
| `worktree_added`       | `""`                                            | The format shown when a new file has been added in the working directory.                                                      |
| `worktree_deleted`     | `""`                                            | The format shown when a file has been deleted in the working directory.                                                        |
| `worktree_modified`    | `""`                                            | The format shown when a file has been modified in the working directory.                                                       |
| `worktree_typechanged` | `""`                                            | The format shown when a file's type has been changed in the working directory.                                                 |
| `index_added`          | `""`                                            | The format shown when a new file has been added to the staging area.                                                           |
| `index_deleted`        | `""`                                            | The format shown when a file has been deleted from the staging area.                                                           |
| `index_modified`       | `""`                                            | The format shown when a file has been modified in the staging area.                                                            |
| `index_typechanged`    | `""`                                            | The format shown when a file's type has been changed in the staging area.                                                      |
| `disabled`             | `false`                                         | Disables the `git_status` module.                                                                                              |
| `windows_starship`     |                                                 | Use this (Linux) path to a Windows Starship executable to render `git_status` when on Windows paths in WSL. |
| `use_git_executable`   | `false`                                         | Do not use `gitoxide` for computing the status, but use the `git` executable instead.                                          |

### Variáveis

The following variables can be used in `format`:

| Variável               | Descrição                                                                                                                     |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `all_status`           | Shortcut for `$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked`.                     |
| `ahead_behind`         | Displays `diverged`, `ahead`, `behind` or `up_to_date` format string based on the current status of the repo. |
| `conflicted`           | Displays `conflicted` when this branch has merge conflicts.                                                   |
| `untracked`            | Displays `untracked` when there are untracked files in the working directory.                                 |
| `stashed`              | Displays `stashed` when a stash exists for the local repository.                                              |
| `modified`             | Displays `modified` when there are file modifications in the working directory.                               |
| `staged`               | Displays `staged` when a new file has been added to the staging area.                                         |
| `renamed`              | Displays `renamed` when a renamed file has been added to the staging area.                                    |
| `deleted`              | Displays `deleted` when a file's deletion has been added to the staging area.                                 |
| `typechanged`          | Displays `typechanged` when a file's type has been changed in the staging area.                               |
| `worktree_added`       | Displays `worktree_added` when a new file has been added in the working directory.                            |
| `worktree_deleted`     | Displays `worktree_deleted` when a file's been deleted in the working directory.                              |
| `worktree_modified`    | Displays `worktree_modified` when a file's been modified in the working directory.                            |
| `worktree_typechanged` | Displays `worktree_typechanged` when a file's type has been changed in the working directory.                 |
| `index_added`          | Displays `index_added` when a new file has been added to the staging area.                                    |
| `index_deleted`        | Displays `index_deleted` when a file has been deleted from the staging area.                                  |
| `index_modified`       | Displays `index_modified` when a file has been modified in the staging area.                                  |
| `index_typechanged`    | Displays `index_typechanged` when a file's type has been changed in the staging area.                         |
| style\*                | Espelha o valor da opção `style`                                                                                              |

\*: Esta variável só pode ser usada como parte de uma string de estilo

The following variables can be used in `diverged`:

| Variável       | Descrição                                           |
| -------------- | --------------------------------------------------- |
| `ahead_count`  | Número de commits a frente do braço de rastreamento |
| `behind_count` | Número de commits atrás do braço de rastreamento    |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed`, `deleted`, `typechanged`, `worktree_added`, `worktree_deleted`, `worktree_modified`, `worktree_typechanged`, `index_added`, `index_deleted`, `index_modified`, and `index_typechanged`:

| Variável | Descrição                  |
| -------- | -------------------------- |
| `count`  | Exibe o número de arquivos |

### Exemplo

```toml
# ~/.config/starship.toml

[git_status]
conflicted = '🏳'
ahead = '🏎💨'
behind = '😰'
diverged = '😵'
up_to_date = '✓'
untracked = '🤷'
stashed = '📦'
modified = '📝'
staged = '[++\($count\)](green)'
renamed = '👅'
deleted = '🗑'
```

Exibe o count a frente/atrás do braço que esta sendo rastreado

```toml
# ~/.config/starship.toml

[git_status]
ahead = '⇡${count}'
diverged = '⇕⇡${ahead_count}⇣${behind_count}'
behind = '⇣${count}'
```

Use o executável do Windows Starship em caminhos do Windows em WSL

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/nomedousuario/scoop/apps/starship/current/starship.exe'
```

## Gleam

The `gleam` module shows the currently installed version of [Gleam](https://gleam.run/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `gleam.toml` file
- The current directory contains a file with the `.gleam` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'⭐ '`                               | A format string representing the symbol of Gleam.                                             |
| `detect_extensions` | `['gleam']`                          | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['gleam.toml']`                     | Quais nomes de arquivos devem ativar este módulo.                                             |
| `style`             | `'bold #FFAFF3'`                     | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `gleam` module.                                                                  |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v1.0.0` | The version of `gleam`            |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[gleam]
format = 'via [⭐ $version](bold red) '
```

## Go

The `golang` module shows the currently installed version of [Go](https://golang.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `go.work` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Opções

| Opções              | Padrão                                                                                    | Descrição                                                                                                                                  |
| ------------------- | ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | O formato do módulo.                                                                                                       |
| `version_format`    | `'v${raw}'`                                                                               | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch`                              |
| `símbolo`           | `'🐹 '`                                                                                   | O formato da string que representa o simbolo do Go.                                                                        |
| `detect_extensions` | `['go']`                                                                                  | Quais extensões devem ativar este módulo.                                                                                  |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Quais nomes de arquivos devem ativar este módulo.                                                                          |
| `detect_folders`    | `['Godeps']`                                                                              | Quais pastas devem ativar este módulo.                                                                                     |
| `style`             | `'bold cyan'`                                                                             | O estilo do módulo.                                                                                                        |
| `not_capable_style` | `'bold red'`                                                                              | The style for the module when the go directive in the go.mod file does not match the installed Go version. |
| `disabled`          | `false`                                                                                   | Disables the `golang` module.                                                                                              |

### Variáveis

| Variável                         | Exemplo   | Descrição                                                                                                                                                                   |
| -------------------------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version                          | `v1.12.1` | The version of `go`                                                                                                                                                         |
| mod_version | `1.16`    | `go` version requirement as set in the go directive of `go.mod`. Will only show if the version requirement does not match the `go` version. |
| symbol                           |           | Espelha o valor da opção `symbol`                                                                                                                                           |
| style\*                          |           | Espelha o valor da opção `style`                                                                                                                                            |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

### Using `mod_version`

```toml
# ~/.config/starship.toml

[golang]
format = 'via [$symbol($version )($mod_version )]($style)'
```

## Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment.
The module will be shown when inside a guix-shell environment.

### Opções

| Opções     | Padrão                     | Descrição                                                              |
| ---------- | -------------------------- | ---------------------------------------------------------------------- |
| `format`   | `'via [$symbol]($style) '` | O formato do módulo.                                   |
| `símbolo`  | `'🐃 '`                    | A format string representing the symbol of guix-shell. |
| `style`    | `'yellow bold'`            | O estilo do módulo.                                    |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| symbol   |         | Espelha o valor da opção `symbol` |
| style\*  |         | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

The `gradle` module shows the version of the [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html)
currently used in the project directory.

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `gradle/wrapper/gradle-wrapper.properties` directory.
- The current directory contains a file ending with `.gradle` or `.gradle.kts`.

The `gradle` module is only able to read your Gradle Wrapper version from your config file, we don't execute your wrapper, because of the security concerns.

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🅶 '`                              | A format string representing the symbol of Gradle.                                            |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `['gradle']`                         | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold bright-cyan'`                 | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `gradle` module.                                                                 |
| `recursive`         | `false`                              | Enables recursive finding for the `gradle` directory.                                         |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v7.5.1` | The version of `gradle`           |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

## Haskell

The `haskell` module finds the current selected GHC version and/or the selected Stack snapshot.

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `stack.yaml` file
- The current directory contains any `.hs`, `.cabal`, or `.hs-boot` file

### Opções

| Opções              | Padrão                               | Descrição                                                         |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                              |
| `símbolo`           | `'λ '`                               | Uma string de formato que representa o símbolo de Haskell         |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Quais extensões devem ativar este módulo.         |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | Quais nomes de arquivos devem ativar este módulo. |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.            |
| `style`             | `'bold purple'`                      | O estilo do módulo.                               |
| `disabled`          | `false`                              | Disables the `haskell` module.                    |

### Variáveis

| Variável                           | Exemplo     | Descrição                                                                               |
| ---------------------------------- | ----------- | --------------------------------------------------------------------------------------- |
| version                            |             | `ghc_version` or `snapshot` depending on whether the current project is a Stack project |
| snapshot                           | `lts-18.12` | Snapshot do Stack selecionado                                                           |
| ghc\_version | `9.2.1`     | Versão do GHC instalada                                                                 |
| symbol                             |             | Espelha o valor da opção `symbol`                                                       |
| style\*                            |             | Espelha o valor da opção `style`                                                        |

\*: Esta variável só pode ser usada como parte de uma string de estilo

## Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- The current directory contains a `.haxelib` or a `haxe_libraries` directory
- The current directory contains a file with the `.hx` or `.hxml` extension

### Opções

| Opções              | Padrão                                                                                          | Descrição                                                                                                     |
| ------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                            | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                                                                                     | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | Quais pastas devem ativar este módulo.                                                        |
| `símbolo`           | `'⌘ '`                                                                                          | A format string representing the symbol of Haxe.                                              |
| `style`             | `'bold fg:202'`                                                                                 | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                                                                         | Disables the `haxe` module.                                                                   |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v4.2.5` | The version of `haxe`             |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `helmfile.yaml` file
- The current directory contains a `Chart.yaml` file

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `símbolo`           | `'⎈ '`                               | O formato de string que representa o simbolo do Helm.                                         |
| `style`             | `'bold white'`                       | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `helm` module.                                                                   |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v3.1.1` | The version of `helm`             |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## Hostname

The `hostname` module shows the system hostname.

### Opções

| Opções            | Padrão                                 | Descrição                                                                                                                                                                                      |
| ----------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`        | `true`                                 | Apenas exibe o hostname quando conectado em uma sessão SSH.                                                                                                                    |
| `ssh_symbol`      | `'🌐 '`                                | Uma formatação de string que representa o símbolo quando conectado à sessão SSH.                                                                                               |
| `trim_at`         | `'.'`                                  | String na qual vai truncar o hostname, após a primeira correspondência. `'.'` will stop after the first dot. `''` will disable any truncation. |
| `detect_env_vars` | `[]`                                   | Which environment variable(s) should trigger this module.                                                                                                   |
| `format`          | `'[$ssh_symbol$hostname]($style) in '` | O formato do módulo.                                                                                                                                                           |
| `style`           | `'bold dimmed green'`                  | O estilo do módulo.                                                                                                                                                            |
| `disabled`        | `false`                                | Disables the `hostname` module.                                                                                                                                                |
| `aliases`         | `{}`                                   | Translate system hostnames to something else. If `trim_at` is specified, only the first part will be matched and replaced.                                     |

### Variáveis

| Variável                        | Exemplo    | Descrição                                                      |
| ------------------------------- | ---------- | -------------------------------------------------------------- |
| hostname                        | `computer` | O hostname do computador                                       |
| style\*                         |            | Espelha o valor da opção `style`                               |
| ssh_symbol | `'🌏 '`    | O símbolo a ser representado quando conectado à uma sessão SSH |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplos

#### Always show the hostname

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

#### Hide the hostname in remote tmux sessions

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
detect_env_vars = ['!TMUX', 'SSH_CONNECTION']
disabled = false
```

#### Replace the hostname with a nickname

```toml
# ~/.config/starship.toml
[hostname]
aliases = { "Max's MacBook Pro" = "home" }
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot`, or `.sdkmanrc` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Opções

| Opções              | Padrão                                                                                                                | Descrição                                                                                                     |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                                                                                                           | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                                                                                                  | Quais pastas devem ativar este módulo.                                                        |
| `símbolo`           | `'☕ '`                                                                                                                | Um formato de string que representa o simbolo do Java                                                         |
| `style`             | `'red dimmed'`                                                                                                        | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                                                                                               | Disables the `java` module.                                                                   |

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| version  | `v14`   | The version of `java`             |
| symbol   |         | Espelha o valor da opção `symbol` |
| style\*  |         | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## Jobs

The `jobs` module shows the current number of jobs running.
O módulo vai ser exibido apenas se existir jobs em segundo plano sendo executados.
The module will show the number of jobs running if there are at least
2 jobs, or more than the `number_threshold` config value, if it exists.
The module will show a symbol if there is at least 1 job, or more than the
`symbol_threshold` config value, if it exists. You can set both values
to 0 in order to _always_ show the symbol and number of jobs, even if there are
0 jobs running.

A funcionalidade padrão é:

- 0 jobs -> Nada é exibido.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

> [!WARNING]
> This module is not supported on tcsh.

> [!WARNING]
> The `threshold` option is deprecated, but if you want to use it,
> the module will show the number of jobs running if there is more than 1 job, or
> more than the `threshold` config value, if it exists. If `threshold` is set to 0,
> then the module will also show when there are 0 jobs running.

### Opções

| Opções             | Padrão                        | Descrição                                                                                |
| ------------------ | ----------------------------- | ---------------------------------------------------------------------------------------- |
| `threshold`\*      | `1`                           | Exibe o número de jobs se excedido.                                      |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | O formato do módulo.                                                     |
| `símbolo`          | `'✦'`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `'bold blue'`                 | O estilo do módulo.                                                      |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |

\*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| number   | `1`     | O número de jobs                  |
| symbol   |         | Espelha o valor da opção `symbol` |
| style\*  |         | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplos

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
symbol_threshold = 0
```

#### Changing process grouping behavior in fish

When using the Fish shell, Starship counts **job groups** instead of individual process IDs by default. This prevents overcounting when a pipeline has multiple processes but only one suspended group. To revert to the legacy PID-based counting, please add the following to your shell config:

```fish
set -g __starship_fish_use_job_groups "false"
```

## Julia

The `julia` module shows the currently installed version of [Julia](https://julialang.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['jl']`                             | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `símbolo`           | `'ஃ '`                               | O formato de string que representa o simbolo do Julia.                                        |
| `style`             | `'bold purple'`                      | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `julia` module.                                                                  |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v1.4.0` | The version of `julia`            |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `.kt` or a `.kts` file

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['kt', 'kts']`                      | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `símbolo`           | `'🅺 '`                              | O formato de string que representa o simbolo do Kotlin.                                       |
| `style`             | `'bold blue'`                        | O estilo do módulo.                                                                           |
| `kotlin_binary`     | `'kotlin'`                           | Configura o binário do kotlin que o Starship executa para obter a versão.                     |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                                 |

### Variáveis

| Variável | Exemplo   | Descrição                         |
| -------- | --------- | --------------------------------- |
| version  | `v1.4.21` | The version of `kotlin`           |
| symbol   |           | Espelha o valor da opção `symbol` |
| style\*  |           | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[kotlin]
symbol = '🅺 '
```

```toml
# ~/.config/starship.toml

[kotlin]
# Uses the Kotlin Compiler binary to get the installed version
kotlin_binary = 'kotlinc'
```

## Kubernetes

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file.
The namespace needs to be set in the kubeconfig file, this can be done via
`kubectl config set-context starship-context --namespace astronaut`.
Similarly, the user and cluster can be set with `kubectl config set-context starship-context --user starship-user`
and `kubectl config set-context starship-context --cluster starship-cluster`.
If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.
>
> When the module is enabled it will always be active, unless any of
> `detect_env_vars`, `detect_extensions`, `detect_files` or `detect_folders` have
> been set in which case the module will only be active in directories that match
> those conditions or one of the environmental variables has been set.

### Opções

> [!WARNING]
> The `context_aliases` and `user_aliases` options are deprecated. Use `contexts` and the corresponding `context_alias`
> and `user_alias` options instead.

| Opções              | Padrão                                               | Descrição                                                                     |
| ------------------- | ---------------------------------------------------- | ----------------------------------------------------------------------------- |
| `símbolo`           | `'☸ '`                                               | Uma string que representa o simbolo exibido antes do Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | O formato do módulo.                                          |
| `style`             | `'cyan bold'`                                        | O estilo do módulo.                                           |
| `context_aliases`\* | `{}`                                                 | Tabela de aliases de contexto para exibir.                    |
| `user_aliases`\*    | `{}`                                                 | Table of user aliases to display.                             |
| `detect_extensions` | `[]`                                                 | Quais extensões devem ativar este módulo.                     |
| `detect_files`      | `[]`                                                 | Quais nomes de arquivos devem ativar este módulo.             |
| `detect_folders`    | `[]`                                                 | Quais pastas devem ativar este módulo.                        |
| `detect_env_vars`   | `[]`                                                 | Which environmental variables should trigger this module                      |
| `contexts`          | `[]`                                                 | Customized styles and symbols for specific contexts.          |
| `disabled`          | `true`                                               | Disables the `kubernetes` module.                             |

\*: This option is deprecated, please add `contexts` with the corresponding `context_alias` and `user_alias` options instead.

To customize the style of the module for specific environments, use the following configuration as
part of the `contexts` list:

| Variável          | Descrição                                                                                                                |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `context_pattern` | **Required** Regular expression to match current Kubernetes context name.                                |
| `user_pattern`    | Regular expression to match current Kubernetes user name.                                                |
| `context_alias`   | Context alias to display instead of the full context name.                                               |
| `user_alias`      | User alias to display instead of the full user name.                                                     |
| `style`           | The style for the module when using this context. If not set, will use module's style.   |
| `símbolo`         | The symbol for the module when using this context. If not set, will use module's symbol. |

Note that all regular expression are anchored with `^<pattern>$` and so must match the whole string. The `*_pattern`
regular expressions may contain capture groups, which can be referenced in the corresponding alias via `$name` and `$N`
(see example below and the
[rust Regex::replace() documentation](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)).

### Variáveis

| Variável  | Exemplo              | Descrição                                   |
| --------- | -------------------- | ------------------------------------------- |
| context   | `starship-context`   | O nome atual do kubernetes context          |
| namespace | `starship-namespace` | Se definido o namespace atual do kubernetes |
| user      | `starship-user`      | Se definido, o usuário atual do kubernetes  |
| cluster   | `starship-cluster`   | Se definido, o cluster atual do kubernetes  |
| symbol    |                      | Espelha o valor da opção `symbol`           |
| style\*   |                      | Espelha o valor da opção `style`            |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
contexts = [
  { context_pattern = "dev.local.cluster.k8s", style = "green", symbol = "💔 " },
]
```

Only show the module in directories that contain a `k8s` file.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Kubernetes Context specific config

The `contexts` configuration option is used to customise what the current Kubernetes context name looks
like (style and symbol) if the name matches the defined regular expression.

```toml
# ~/.config/starship.toml

[[kubernetes.contexts]]
# "bold red" style + default symbol when Kubernetes current context name equals "production" *and* the current user
# equals "admin_user"
context_pattern = "production"
user_pattern = "admin_user"
style = "bold red"
context_alias = "prod"
user_alias = "admin"

[[kubernetes.contexts]]
# "green" style + a different symbol when Kubernetes current context name contains openshift
context_pattern = ".*openshift.*"
style = "green"
symbol = "💔 "
context_alias = "openshift"

[[kubernetes.contexts]]
# Using capture groups
# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
context_pattern = "gke_.*_(?P<cluster>[\\w-]+)"
context_alias = "gke-$cluster"
```

## Quebra de Linha

The `line_break` module separates the prompt into two lines.

### Opções

| Opções     | Padrão  | Descrição                                                                          |
| ---------- | ------- | ---------------------------------------------------------------------------------- |
| `disabled` | `false` | Disables the `line_break` module, making the prompt a single line. |

### Exemplo

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## IP Local

The `localip` module shows the IPv4 address of the primary network interface.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções     | Padrão                    | Descrição                                                                      |
| ---------- | ------------------------- | ------------------------------------------------------------------------------ |
| `ssh_only` | `true`                    | Apenas mostre o endereço IP quando conectado a uma sessão SSH. |
| `format`   | `'[$localipv4]($style) '` | O formato do módulo.                                           |
| `style`    | `'bold yellow'`           | O estilo do módulo.                                            |
| `disabled` | `true`                    | Disables the `localip` module.                                 |

### Variáveis

| Variável  | Exemplo                                                      | Descrição                        |
| --------- | ------------------------------------------------------------ | -------------------------------- |
| localipv4 | 192.168.1.13 | Contém o endereço IPv4 principal |
| style\*   |                                                              | Espelha o valor da opção `style` |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

The `lua` module shows the currently installed version of [Lua](http://www.lua.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🌙 '`                              | Uma string que representa o simbolo do Lua.                                                   |
| `detect_extensions` | `['lua']`                            | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['.lua-version']`                   | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `['lua']`                            | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold blue'`                        | O estilo do módulo.                                                                           |
| `lua_binary`        | `'lua'`                              | Configura o binário lua que o Starship executa para pegar a versão.                           |
| `disabled`          | `false`                              | Disables the `lua` module.                                                                    |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v5.4.0` | The version of `lua`              |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## Maven

The `maven` module indicates the presence of a Maven project in the current directory. If the [Maven Wrapper](https://maven.apache.org/wrapper/) is enabled, the Maven version will be parsed from `.mvn/wrapper/maven-wrapper.properties` and shown.

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `pom.xml` file.
- The current directory contains a `.mvn/wrapper/maven-wrapper.properties` file.

If you use an alternate POM syntax (for example `pom.hocon`), add its filename to `detect_files`.

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🅼 '`                              | A format string representing the symbol of Maven.                                             |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['pom.xml']`                        | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `['.mvn']`                           | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold bright-cyan'`                 | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `maven` module.                                                                  |
| `recursive`         | `false`                              | Enables recursive finding for the `.mvn` directory.                                           |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v3.2.0` | The version of `maven`            |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

## Uso de Memória

The `memory_usage` module shows current system memory and swap usage.

Por padrão o uso do swap é exibido se o total de swap do sistema é diferente de zero.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções      | Padrão                                         | Descrição                                                                     |
| ----------- | ---------------------------------------------- | ----------------------------------------------------------------------------- |
| `threshold` | `75`                                           | Esconde o uso de memoria a menos que exceda esta porcentagem. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | O formato do módulo.                                          |
| `símbolo`   | `'🐏'`                                         | O simbolo usado antes de exibir o uso de memoria.             |
| `style`     | `'bold dimmed white'`                          | O estilo do módulo.                                           |
| `disabled`  | `true`                                         | Disables the `memory_usage` module.                           |

### Variáveis

| Variável                          | Exemplo       | Descrição                                                         |
| --------------------------------- | ------------- | ----------------------------------------------------------------- |
| ram                               | `31GiB/65GiB` | O uso/total de memoria RAM atual do sistema.      |
| ram_pct      | `48%`         | A porcentagem de uso atual da memoria do sistema. |
| swap\*\*                          | `1GiB/4GiB`   | O tamanho atual do swap do sistema.               |
| swap_pct\*\* | `77%`         | A porcentagem atual de uso do swap.               |
| symbol                            | `🐏`          | Espelha o valor da opção `symbol`                                 |
| style\*                           |               | Espelha o valor da opção `style`                                  |

\*: Esta variável só pode ser usada como parte de uma string de estilo
\*\*: As informações do arquivo SWAP são exibidas apenas se detectadas no sistema atual

### Exemplo

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = ' '
style = 'bold dimmed green'
```

## Meson

The `meson` module shows the current Meson developer environment status.

By default the Meson project name is displayed, if `$MESON_DEVENV` is set.

### Opções

| Opções              | Padrão                             | Descrição                                                                                                                 |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                                |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | O formato do módulo.                                                                                      |
| `símbolo`           | `'⬢ '`                             | The symbol used before displaying the project name.                                                       |
| `style`             | `'blue bold'`                      | O estilo do módulo.                                                                                       |
| `disabled`          | `false`                            | Disables the `meson` module.                                                                              |

### Variáveis

| Variável | Exemplo    | Descrição                         |
| -------- | ---------- | --------------------------------- |
| project  | `starship` | The current Meson project name    |
| symbol   | `🐏`       | Espelha o valor da opção `symbol` |
| style\*  |            | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = '--'
symbol = ' '
style = 'bold dimmed green'
```

## Mercurial Branch

The `hg_branch` module shows the active branch and topic of the repo in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções              | Padrão                                    | Descrição                                                                                               |
| ------------------- | ----------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| `símbolo`           | `' '`                                    | O simbolo usado ante do marcador hg ou nome do braço do repositório no diretório atual. |
| `style`             | `'bold purple'`                           | O estilo do módulo.                                                                     |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | O formato do módulo.                                                                    |
| `truncation_length` | `2^63 - 1`                                | Truncates the hg branch / topic name to `N` graphemes                                                   |
| `truncation_symbol` | `'…'`                                     | O simbolo usado para indicar que o nome braço foi truncado.                             |
| `disabled`          | `true`                                    | Disables the `hg_branch` module.                                                        |

### Variáveis

| Variável | Exemplo   | Descrição                         |
| -------- | --------- | --------------------------------- |
| branch   | `master`  | O braço mercurial ativo           |
| topic    | `feature` | The active mercurial topic        |
| symbol   |           | Espelha o valor da opção `symbol` |
| style\*  |           | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Mercurial State

The `hg_state` module will show in directories which are part of a mercurial
repository, and where there is an operation in progress, such as: _REBASING_,
_BISECTING_, etc.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções       | Padrão                      | Descrição                                                                     |
| ------------ | --------------------------- | ----------------------------------------------------------------------------- |
| `merge`      | `'MERGING'`                 | A format string displayed when a `merge` is in progress.      |
| `rebase`     | `'REBASING'`                | A format string displayed when a `rebase` is in progress.     |
| `update`     | `'UPDATING'`                | A format string displayed when a `update` is in progress.     |
| `bisect`     | `'BISECTING'`               | A format string displayed when a `bisect` is in progress.     |
| `shelve`     | `'SHELVING'`                | A format string displayed when a `shelve` is in progress.     |
| `graft`      | `'GRAFTING'`                | A format string displayed when a `graft` is in progress.      |
| `transplant` | `'TRANSPLANTING'`           | A format string displayed when a `transplant` is in progress. |
| `histedit`   | `'HISTEDITING'`             | A format string displayed when a `histedit` is in progress.   |
| `style`      | `'bold yellow'`             | O estilo do módulo.                                           |
| `format`     | `'\([$state]($style)\) '` | O formato do módulo.                                          |
| `disabled`   | `true`                      | Disables the `hg_state` module.                               |

### Variáveis

| Variável                              | Exemplo    | Descrição                              |
| ------------------------------------- | ---------- | -------------------------------------- |
| state                                 | `REBASING` | O estado atual do repo                 |
| progress_current | `1`        | O progresso da operação atual          |
| progress_total   | `2`        | O total do progresso da operação atual |
| style\*                               |            | Espelha o valor da opção `style`       |

\*: Esta variável só pode ser usada como parte de uma string de estilo

## Mise

The `mise` module shows the current mise health as reported by running `mise doctor`.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções              | Padrão                                                               | Descrição                                                         |
| ------------------- | -------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `símbolo`           | `'mise '`                                                            | The symbol used before displaying _mise_ health.  |
| `style`             | `'bold purple'`                                                      | O estilo do módulo.                               |
| `format`            | `'on [$symbol$health]($style) '`                                     | O formato do módulo.                              |
| `detect_extensions` | `[]`                                                                 | Quais extensões devem ativar este módulo.         |
| `detect_files`      | `['mise.toml', 'mise.local.toml', '.mise.toml', '.mise.local.toml']` | Quais nomes de arquivos devem ativar este módulo. |
| `detect_folders`    | `['.mise']`                                                          | Quais pastas devem ativar este módulo.            |
| `healthy_symbol`    | `healthy`                                                            | The message displayed when _mise_ is healthy.     |
| `unhealthy_symbol`  | `unhealthy`                                                          | The message displayed when _mise_ is unhealthy.   |
| `disabled`          | `true`                                                               | Disables the `mise` module.                       |

### Variáveis

| Variável | Exemplo   | Descrição                         |
| -------- | --------- | --------------------------------- |
| health   | `healthy` | The health of _mise_              |
| symbol   |           | Espelha o valor da opção `symbol` |
| style\*  |           | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[mise]
health = 'ready'
```

## Mojo

The `mojo` module shows the current version of [Mojo programming language](https://www.modular.com/mojo) installed

### Opções

| Opções              | Padrão                                | Descrição                                                              |
| ------------------- | ------------------------------------- | ---------------------------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'` | O formato do módulo.                                   |
| `símbolo`           | `'🔥 '`                               | The symbol used before displaying the version of Mojo. |
| `style`             | `'bold 208'`                          | O estilo do módulo.                                    |
| `disabled`          | `false`                               | Disables the `mojo` module.                            |
| `detect_extensions` | `['mojo', '🔥']`                      | Quais extensões devem ativar este módulo.              |
| `detect_files`      | `[]`                                  | Quais nomes de arquivos devem ativar este módulo.      |
| `detect_folders`    | `[]`                                  | Quais pastas devem ativar este módulo.                 |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `24.4.0` | The version of `mojo`             |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[mojo]
format = 'via [mojo ($version )($hash )]($style)'
```

## NATS

The `nats` module shows the name of the current [NATS](https://nats.io) context.

### Opções

| Opções     | Padrão                     | Descrição                                                                                       |
| ---------- | -------------------------- | ----------------------------------------------------------------------------------------------- |
| `símbolo`  | `'✉️ '`                    | The symbol used before the NATS context (defaults to empty). |
| `style`    | `'bold purple'`            | O estilo do módulo.                                                             |
| `format`   | `'[$symbol$name]($style)'` | O formato do módulo.                                                            |
| `disabled` | `false`                    | Disables the `nats` module.                                                     |

### Variáveis

| Variável | Exemplo     | Descrição                         |
| -------- | ----------- | --------------------------------- |
| name     | `localhost` | The name of the NATS context      |
| symbol   |             | Espelha o valor da opção `symbol` |
| style\*  |             | Espelha o valor da opção `style`  |

### Exemplo

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```

## Network Namespace

The `netns` module shows the current network namespace.
This uses `ip netns identify` to get the network namespace, so only network namespaces mounted at `/var/run/netns` will be detected.

### Opções

| Opções     | Padrão                            | Descrição                                                                                            |
| ---------- | --------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `format`   | `'[$symbol \[$name\]]($style)'` | O formato do módulo.                                                                 |
| `símbolo`  | `'🛜 '`                           | The symbol used before the network namespace (defaults to empty). |
| `style`    | `'blue bold dimmed'`              | O estilo do módulo.                                                                  |
| `disabled` | `false`                           | Disables the `netns` module.                                                         |

### Variáveis

| Variável | Exemplo    | Descrição                                 |
| -------- | ---------- | ----------------------------------------- |
| name     | `my-netns` | The name of the current network namespace |
| symbol   |            | Espelha o valor da opção `symbol`         |
| style\*  |            | Espelha o valor da opção `style`          |

### Exemplo

```toml
# ~/.config/starship.toml

[netns]
style = 'bold yellow'
symbol = '🌐 '
```

## Nim

The `nim` module shows the currently installed version of [Nim](https://nim-lang.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo                                                                                           |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'👑 '`                              | O símbolo usado antes de exibir a versão do Nim.                                              |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['nim.cfg']`                        | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold yellow'`                      | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `nim` module.                                                                    |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v1.2.0` | The version of `nimc`             |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

The `nix_shell` module shows the [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) environment.
O módulo vai exibir quando estiver dentro de um ambiente nix-shell.

### Opções

| Opções        | Padrão                                         | Descrição                                                                             |
| ------------- | ---------------------------------------------- | ------------------------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | O formato do módulo.                                                  |
| `símbolo`     | `'❄️ '`                                        | Uma string que representa o simbolo do nix-shell.                     |
| `style`       | `'bold blue'`                                  | O estilo do módulo.                                                   |
| `impure_msg`  | `'impure'`                                     | Uma string que exibe quando o shell é impuro.                         |
| `pure_msg`    | `'pure'`                                       | Uma string que exibe quando o shell é puro.                           |
| `unknown_msg` | `''`                                           | A format string shown when it is unknown if the shell is pure/impure. |
| `disabled`    | `false`                                        | Disables the `nix_shell` module.                                      |
| `heuristic`   | `false`                                        | Attempts to detect new `nix shell`-style shells with a heuristic.     |

### Variáveis

| Variável | Exemplo | Descrição                                                                                        |
| -------- | ------- | ------------------------------------------------------------------------------------------------ |
| state    | `pure`  | O estado do nix-shell                                                                            |
| name     | `lorri` | O nome do nix-shell                                                                              |
| level    | `1`     | The depth level of the nix-shell (Only when using [Lix](https://lix.systems)) |
| symbol   |         | Espelha o valor da opção `symbol`                                                                |
| style\*  |         | Espelha o valor da opção `style`                                                                 |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = '[impure shell](bold red)'
pure_msg = '[pure shell](bold green)'
unknown_msg = '[unknown shell](bold yellow)'
format = 'via [☃️ $state( \($name\))](bold blue) '
```

## Node.js

The `nodejs` module shows the currently installed version of [Node.js](https://nodejs.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts`, `.mts` or `.cts` extension

Additionally, the module will be hidden by default if the directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `bunfig.toml`, `bun.lock`, or `bun.lockb` file, overriding the above conditions.

### Opções

| Opções              | Padrão                                        | Descrição                                                                                                                                                |
| ------------------- | --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | O formato do módulo.                                                                                                                     |
| `version_format`    | `'v${raw}'`                                   | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch`                                            |
| `símbolo`           | `' '`                                        | Uma string que representa o simbolo do Node.js.                                                                          |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | Quais extensões devem ativar este módulo.                                                                                                |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | Quais nomes de arquivos devem ativar este módulo.                                                                                        |
| `detect_folders`    | `['node_modules']`                            | Quais pastas devem ativar este módulo.                                                                                                   |
| `style`             | `'bold green'`                                | O estilo do módulo.                                                                                                                      |
| `disabled`          | `false`                                       | Disables the `nodejs` module.                                                                                                            |
| `not_capable_style` | `'bold red'`                                  | O estilo para o módulo quando a propriedade engine no package.json não coincide com a versão do Node.js. |

### Variáveis

| Variável                             | Exemplo    | Descrição                                                                                                                                                                                 |
| ------------------------------------ | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version                              | `v13.12.0` | The version of `node`                                                                                                                                                                     |
| engines_version | `>=12.0.0` | `node` version requirement as set in the engines property of `package.json`. Will only show if the version requirement does not match the `node` version. |
| symbol                               |            | Espelha o valor da opção `symbol`                                                                                                                                                         |
| style\*                              |            | Espelha o valor da opção `style`                                                                                                                                                          |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Opções

| Opções                    | Padrão                                                                     | Descrição                                                                                                     |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | A string de formato do módulo.                                                                |
| `version_format`          | `'v${raw}'`                                                                | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`                 | `'🐫 '`                                                                    | O símbolo usado antes de exibir a versão do OCaml.                                            |
| `global_switch_indicator` | `''`                                                                       | A string usada para representar a mudança global OPAM.                                        |
| `local_switch_indicator`  | `'*'`                                                                      | A string usada para representar as mudanças locais do OPAM.                                   |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                       | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                    | Quais pastas devem ativar este módulo.                                                        |
| `style`                   | `'bold yellow'`                                                            | O estilo do módulo.                                                                           |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                                                  |

### Variáveis

| Variável                              | Exemplo      | Descrição                                                         |
| ------------------------------------- | ------------ | ----------------------------------------------------------------- |
| version                               | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | O switch OPAM ativo                                               |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol                                |              | Espelha o valor da opção `symbol`                                 |
| style\*                               |              | Espelha o valor da opção `style`                                  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Odin

The `odin` module shows the currently installed version of [Odin](https://odin-lang.org/). By default the module will be shown if the current directory contains a `.odin` file.

### Opções

| Opções              | Padrão                               | Descrição                                                              |
| ------------------- | ------------------------------------ | ---------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                   |
| `show_commit`       | `false`                              | Shows the commit as part of the version.               |
| `símbolo`           | `'Ø '`                               | The symbol used before displaying the version of Odin. |
| `style`             | `'bold bright-blue'`                 | O estilo do módulo.                                    |
| `disabled`          | `false`                              | Disables the `odin` module.                            |
| `detect_extensions` | `['odin']`                           | Quais extensões devem ativar este módulo.              |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.      |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                 |

### Variáveis

| Variável | Exemplo       | Descrição                         |
| -------- | ------------- | --------------------------------- |
| version  | `dev-2024-03` | The version of `odin`             |
| symbol   |               | Espelha o valor da opção `symbol` |
| style\*  |               | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[odin]
format = 'via [󰹩 ($version )]($style)'
show_commit = true
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool.
By default the module will be shown if the current directory contains a `.rego` file.

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🪖  '`                             | A format string representing the symbol of OPA.                                               |
| `detect_extensions` | `['rego']`                           | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold blue'`                        | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `opa` module.                                                                    |

### Variáveis

| Variável | Exemplo   | Descrição                         |
| -------- | --------- | --------------------------------- |
| version  | `v0.44.0` | The version of `opa`              |
| symbol   |           | Espelha o valor da opção `symbol` |
| style\*  |           | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

The `openstack` module shows the current OpenStack cloud and project. The module
only active when the `OS_CLOUD` env var is set, in which case it will read
`clouds.yaml` file from any of the [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files).
para buscar o projeto atual em uso.

### Opções

| Opções     | Padrão                                          | Descrição                                                            |
| ---------- | ----------------------------------------------- | -------------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | O formato do módulo.                                 |
| `símbolo`  | `'☁️ '`                                         | O simbolo usado para exibir o OpenStack cloud atual. |
| `style`    | `'bold yellow'`                                 | O estilo do módulo.                                  |
| `disabled` | `false`                                         | Disables the `openstack` module.                     |

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| cloud    | `corp`  | O OpenStack cloud atual           |
| project  | `dev`   | O projeto OpenStack atual         |
| symbol   |         | Espelha o valor da opção `symbol` |
| style\*  |         | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```

## OS

The `os` module shows the current operating system.
OS information is detected via the [os_info](https://lib.rs/crates/os_info) crate.

> [!WARNING]
> The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções     | Padrão                | Descrição                                                              |
| ---------- | --------------------- | ---------------------------------------------------------------------- |
| `format`   | `'[$symbol]($style)'` | O formato do módulo.                                   |
| `style`    | `'bold white'`        | O estilo do módulo.                                    |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type.
Operating system types not defined by your configuration use the default symbols table below.
All operating systems currently supported by the module are listed below.
If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# This is the default symbols table.
[os.symbols]
AIX = "➿ "
Alpaquita = "🔔 "
AlmaLinux = "💠 "
Alpine = "🏔️ "
ALTLinux = "Ⓐ "
Amazon = "🙂 "
Android = "🤖 "
AOSC = "🐱 "
Arch = "🎗️ "
Artix = "🎗️ "
Bluefin = "🐟 "
CachyOS = "🎗️ "
CentOS = "💠 "
Debian = "🌀 "
DragonFly = "🐉 "
Elementary = "🍏 "
Emscripten = "🔗 "
EndeavourOS = "🚀 "
Fedora = "🎩 "
FreeBSD = "😈 "
Garuda = "🦅 "
Gentoo = "🗜️ "
HardenedBSD = "🛡️ "
Illumos = "🐦 "
Ios = "📱 "
InstantOS = "⏲️ "
Kali = "🐉 "
Linux = "🐧 "
Mabox = "📦 "
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
Nobara = "🎩 "
OpenBSD = "🐡 "
OpenCloudOS = "☁️ "
openEuler = "🦉 "
openSUSE = "🦎 "
OracleLinux = "🦴 "
PikaOS = "🐤 "
Pop = "🍭 "
Raspbian = "🍓 "
Redhat = "🎩 "
RedHatEnterprise = "🎩 "
RockyLinux = "💠 "
Redox = "🧪 "
Solus = "⛵ "
SUSE = "🦎 "
Ubuntu = "🎯 "
Ultramarine = "🔷 "
Unknown = "❓ "
Uos = "🐲 "
Void = " "
Windows = "🪟 "
Zorin = "🔹 "
```

### Variáveis

| Variável | Exemplo      | Descrição                                                          |
| -------- | ------------ | ------------------------------------------------------------------ |
| symbol   | `🎗️`        | The current operating system symbol from advanced option `symbols` |
| name     | `Arch Linux` | The current operating system name                                  |
| tipo     | `Arch`       | The current operating system type                                  |
| codename |              | The current operating system codename, if applicable               |
| edition  |              | The current operating system edition, if applicable                |
| version  |              | The current operating system version, if applicable                |
| style\*  |              | Espelha o valor da opção `style`                                   |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[os]
format = "on [($name )]($style)"
style = "bold blue"
disabled = false

[os.symbols]
Windows = " "
Arch = "Arch is the best! "
```

## Versionamento de Pacotes

The `package` module is shown when the current directory is the repository for a
package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`,
`poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `galaxy`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present
  in the current directory
- [**JSR**](https://jsr.io/) – The `jsr` package version is extracted from the `jsr.json`/`jsr.jsonc` or `deno.json`/`deno.jsonc` present in the current directory
- [**Cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**Nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**Poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present
  in the current directory
- [**Python**](https://www.python.org) - The `python` package version is extracted from a [PEP 621](https://peps.python.org/pep-0621/) compliant `pyproject.toml` or a `setup.cfg` present in the current directory
- [**Composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present
  in the current directory
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present in the current directory
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present in the current directory
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present in the current directory
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present in the current directory
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present in the current directory
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present in the current directory
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present in the current directory
- [**Galaxy**](https://galaxy.ansible.com/) - The `galaxy` package version is extracted from the `galaxy.yml` present in the current directory
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present in the current directory
- [**SBT**](https://scala-sbt.org) - The `sbt` package version is extracted from the `build.sbt` present in the current directory
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) - The `dart` package version is extracted from the `pubspec.yaml` present in the current directory

> ⚠️ The version being shown is that of the package whose source code is in your
> current directory, not your package manager.

### Opções

| Opções            | Padrão                            | Descrição                                                                                                     |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`          | `'is [$symbol$version]($style) '` | O formato do módulo.                                                                          |
| `símbolo`         | `'📦 '`                           | O símbolo usado antes de exibir a versão do pacote.                                           |
| `version_format`  | `'v${raw}'`                       | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`           | `'bold 208'`                      | O estilo do módulo.                                                                           |
| `display_private` | `false`                           | Habilita a exibição da versão para os pacotes marcados como privado.                          |
| `disabled`        | `false`                           | Disables the `package` module.                                                                |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v1.0.0` | A versão do seu pacote            |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[package]
format = 'via [🎁 $version](208 bold) '
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Opções

| Opções              | Padrão                                                                                                   | Descrição                                                                                                     |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | A string de formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                                                                              | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🐪 '`                                                                                                  | O símbolo usado antes de exibir a versão do Perl.                                             |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                                                                                     | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold 149'`                                                                                             | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                                                   |

### Variáveis

| Variável | Exemplo   | Descrição                         |
| -------- | --------- | --------------------------------- |
| version  | `v5.26.1` | The version of `perl`             |
| symbol   |           | Espelha o valor da opção `symbol` |
| style\*  |           | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🐘 '`                              | The symbol used before displaying the version of PHP.                                         |
| `detect_extensions` | `['php']`                            | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['composer.json', '.php-version']`  | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'147 bold'`                         | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `php` module.                                                                    |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v7.3.8` | The version of `php`              |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Pijul Channel

The `pijul_channel` module shows the active channel of the repo in your current directory.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções              | Padrão                            | Descrição                                                                                            |
| ------------------- | --------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `símbolo`           | `' '`                            | The symbol used before the pijul channel name of the repo in your current directory. |
| `style`             | `'bold purple'`                   | O estilo do módulo.                                                                  |
| `format`            | `'on [$symbol$channel]($style) '` | O formato do módulo.                                                                 |
| `truncation_length` | `2^63 - 1`                        | Truncates the pijul channel name to `N` graphemes                                                    |
| `truncation_symbol` | `'…'`                             | O simbolo usado para indicar que o nome braço foi truncado.                          |
| `disabled`          | `true`                            | Disables the `pijul` module.                                                         |

## Pixi

The `pixi` module shows the installed [pixi](https://pixi.sh) version as well as the activated
environment and project name, if `$PIXI_ENVIRONMENT_NAME` is set.

> [!TIP]
> This does not suppress pixi's own prompt modifier, you may want to run `pixi config set shell.change-ps1 false`.

### Opções

| Opções                     | Padrão                                                    | Descrição                                                                                                                      |
| -------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `format`                   | `'via [$symbol($version )(\($environment\) )]($style)'` | O formato do módulo.                                                                                           |
| `version_format`           | `'v${raw}'`                                               | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch`. |
| `símbolo`                  | `'🧚 '`                                                   | O simbolo usado antes do nome do environment.                                                                  |
| `style`                    | `'yellow bold'`                                           | O estilo do módulo.                                                                                            |
| `show_default_environment` | `true`                                                    | Whether to indicate that the `default` environment of your project is activated.                               |
| `pixi_binary`              | `['pixi']`                                                | Configures the pixi binary that Starship should execute when getting the version.                              |
| `detect_extensions`        | `[]`                                                      | Quais extensões devem ativar este módulo.                                                                      |
| `detect_files`             | `['pixi.toml']`                                           | Quais nomes de arquivos devem ativar este módulo.                                                              |
| `detect_folders`           | `[]`                                                      | Quais pastas devem ativar este módulo.                                                                         |
| `disabled`                 | `false`                                                   | Disables the `pixi` module.                                                                                    |

### Variáveis

| Variável                          | Exemplo      | Descrição                         |
| --------------------------------- | ------------ | --------------------------------- |
| version                           | `v0.33.0`    | The version of `pixi`             |
| environment                       | `py311`      | The current pixi environment      |
| project_name | `my-project` | The current pixi project name     |
| symbol                            |              | Espelha o valor da opção `symbol` |
| style                             |              | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml

[pixi]
format = '[$symbol$environment](yellow) '
```

## Pulumi

The `pulumi` module shows the current username, selected [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/), and version.

> [!TIP]
> By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms).
> If you still want to enable it, [follow the example shown below](#with-pulumi-version).

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains either `Pulumi.yaml` or `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Opções

| Opções           | Padrão                                       | Descrição                                                                                                     |
| ---------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | A string de formato do módulo.                                                                |
| `version_format` | `'v${raw}'`                                  | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`        | `' '`                                       | Uma string que é exibida antes do Pulumi stack.                                               |
| `style`          | `'bold 5'`                                   | O estilo do módulo.                                                                           |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.                                |
| `disabled`       | `false`                                      | Disables the `pulumi` module.                                                                 |

### Variáveis

| Variável | Exemplo    | Descrição                         |
| -------- | ---------- | --------------------------------- |
| version  | `v0.12.24` | The version of `pulumi`           |
| stack    | `dev`      | A stack Pulumi atual              |
| username | `alice`    | O nome de usuário Pulumi atual    |
| symbol   |            | Espelha o valor da opção `symbol` |
| style\*  |            | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

#### Com a versão do Pulumi

```toml
# ~/.config/starship.toml

[pulumi]
format = '[🛥 ($version )$stack]($style) '
```

#### Sem a versão do Pulumi

```toml
# ~/.config/starship.toml
[pulumi]
symbol = '🛥 '
format = '[$symbol$stack]($style) '
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version.
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `spago.dhall` file
- The current directory contains a `spago.yaml` file
- The current directory contains a `spago.lock` file
- The current directory contains a file with the `.purs` extension

### Opções

| Opções              | Padrão                                        | Descrição                                                                                                     |
| ------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                                   | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'<=> '`                                      | O símbolo usado antes de exibir a versão do PureScript.                                       |
| `detect_extensions` | `['purs']`                                    | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['spago.dhall', 'spago.yaml', 'spago.lock']` | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                          | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold white'`                                | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                       | Disables the `purescript` module.                                                             |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `0.13.5` | The version of `purescript`       |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```

## Python

The `python` module shows the currently installed version of [Python](https://www.python.org/) and the
current [Python virtual environment](https://docs.python.org/tutorial/venv.html) if one is activated.

If `pyenv_version_name` is set to `true`, it will display the pyenv version
name. Otherwise, it will display the version number from `python --version`.

Por padrão, o módulo será exibido se qualquer das seguintes condições for atendida:

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
- The current directory contains a file with the `.py` extension.
- The current directory contains a file with the `.ipynb` extension.
- Um ambiente virtual está atualmente ativo

### Opções

| Opções               | Padrão                                                                                                       | Descrição                                                                                                     |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | O formato do módulo.                                                                          |
| `version_format`     | `'v${raw}'`                                                                                                  | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`            | `'🐍 '`                                                                                                      | Uma string que representa o simbolo do Python                                                                 |
| `style`              | `'yellow bold'`                                                                                              | O estilo do módulo.                                                                           |
| `pyenv_version_name` | `false`                                                                                                      | Usa pyenv para pegar a versão do Python                                                                       |
| `pyenv_prefix`       | `'pyenv'`                                                                                                    | Prefixo antes da versão do pyenv, apenas usado se pyenv for usado                                             |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configures the python binaries that Starship should execute when getting the version.         |
| `detect_extensions`  | `['py', 'ipynb']`                                                                                            | Quais extensões devem acionar este módulo                                                                     |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | []                                                        |
| `detect_folders`     | `[]`                                                                                                         | Quais pastas devem ativar este módulo                                                                         |
| `generic_venv_names` | `[]`                                                                                                         | Which venv names should be replaced with the parent directory name.                           |
| `disabled`           | `false`                                                                                                      | Disables the `python` module.                                                                 |

> [!TIP]
> The `python_binary` variable accepts either:
>
> - a string (e.g. `'python3'`),
> - a list of strings (e.g. `['python', 'python3']`)
> - a list of lists of strings, representing commands with optional arguments (e.g.
>   `[['mise', 'exec', '--', 'python'], ['python3']]`)
>
> Starship will try executing each configured command until it gets a result.
> Note you can only change the binary that Starship executes to get the version
> of Python not the arguments that are used.
>
> The default values and order for `python_binary` was chosen to first identify
> the Python version in a virtualenv/conda environments (which currently still
> add a `python`, no matter if it points to `python3` or `python2`). This has the
> side effect that if you still have a system Python 2 installed, it may be
> picked up before any Python 3 (at least on Linux Distros that always symlink
> `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but
> cannot remove the system Python 2, changing this to `'python3'` will hide any
> Python version 2, see example below.

### Variáveis

| Variável                          | Exemplo         | Descrição                                                                   |
| --------------------------------- | --------------- | --------------------------------------------------------------------------- |
| version                           | `'v3.8.1'`      | The version of `python`                                                     |
| symbol                            | `'🐍 '`         | Espelha o valor da opção `symbol`                                           |
| style                             | `'yellow bold'` | Espelha o valor da opção `style`                                            |
| pyenv_prefix | `'pyenv '`      | Mirrors the value of option `pyenv_prefix`                                  |
| virtualenv                        | `'venv'`        | The current `virtualenv` name or the parent if matches `generic_venv_names` |

### Exemplo

```toml
# ~/.config/starship.toml

[python]
symbol = '👾 '
pyenv_version_name = true
```

```toml
# ~/.config/starship.toml

[python]
# Only use the `python3` binary to get the version.
python_binary = 'python3'
```

```toml
# ~/.config/starship.toml

[python]
# Use `mise` to get the version.
python_binary = [['mise', 'exec', '--', 'python']]
```

```toml
# ~/.config/starship.toml

[python]
# Potentially dangerous: `uv` can run any binary at `.venv/bin/python` without interaction
python_binary = [['uv', 'run', '--no-python-downloads', '--no-project', 'python']]
```

```toml
# ~/.config/starship.toml

[python]
# Não acione para arquivos com a extensão py
detect_extensions = []
```

## Quarto

The `quarto` module shows the current installed version of Quarto used in a project.

Por padrão, o módulo será exibido se qualquer das seguintes condições for atendida:

- The current directory contains a `_quarto.yml` file
- The current directory contains any `*.qmd` file

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'⨁ '`                               | A format string representing the symbol of Quarto                                                             |
| `style`             | `'bold #75AADB'`                     | O estilo do módulo.                                                                           |
| `detect_extensions` | `['.qmd']`                           | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['_quarto.yml']`                    | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `disabled`          | `false`                              | Disables the `quarto` module.                                                                 |

### Variáveis

| Variável | Exemplo   | Descrição                         |
| -------- | --------- | --------------------------------- |
| version  | `1.4.549` | The version of `quarto`           |
| symbol   |           | Espelha o valor da opção `symbol` |
| style\*  |           | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

## R

The `rlang` module shows the currently installed version of [R](https://www.r-project.org/). O módulo será mostrado se
qualquer uma das seguintes condições for atendida:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'📐'`                               | Uma string que representa o simbolo do R.                                                     |
| `style`             | `'blue bold'`                        | O estilo do módulo.                                                                           |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Quais extensões devem acionar este módulo                                                                     |
| `detect_files`      | `['.Rprofile']`                      | []                                                        |
| `detect_folders`    | `['.Rproj.user']`                    | Quais pastas devem ativar este módulo                                                                         |
| `disabled`          | `false`                              | Disables the `r` module.                                                                      |

### Variáveis

| Variável | Exemplo       | Descrição                         |
| -------- | ------------- | --------------------------------- |
| version  | `v4.0.5`      | The version of `R`                |
| symbol   |               | Espelha o valor da opção `symbol` |
| style    | `'blue bold'` | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Opções

| Opções              | Padrão                                           | Descrição                                                                                                     |
| ------------------- | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | A string de formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                      | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🦋 '`                                          | The symbol used before displaying the version of Raku                                                         |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['META6.json']`                                 | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                             | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold 149'`                                     | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                          | Disables the `raku` module.                                                                   |

### Variáveis

| Variável                        | Exemplo | Descrição                            |
| ------------------------------- | ------- | ------------------------------------ |
| version                         | `v6.d`  | The version of `raku`                |
| vm_version | `moar`  | The version of VM `raku` is built on |
| symbol                          |         | Espelha o valor da opção `symbol`    |
| style\*                         |         | Espelha o valor da opção `style`     |

### Exemplo

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/).
O módulo será mostrado se alguma das seguintes condições for atendida:

- The current directory contains a file with `.red` or `.reds` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🔺 '`                              | Uma string que representa o simbolo do Red.                                                   |
| `detect_extensions` | `['red']`                            | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'red bold'`                         | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `red` module.                                                                    |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v2.5.1` | The version of `red`              |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/).
O módulo será mostrado se alguma das seguintes condições for atendida:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file
- The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set

Starship gets the current Ruby version by running `ruby -v`.

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'💎 '`                              | Uma string que representa o simbolo do Ruby.                                                  |
| `detect_extensions` | `['rb']`                             | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Quais variáveis de ambiente devem ativar este módulo.                                         |
| `style`             | `'bold red'`                         | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `ruby` module.                                                                   |

### Variáveis

| Variável | Exemplo  | Descrição                                                   |
| -------- | -------- | ----------------------------------------------------------- |
| version  | `v2.5.1` | The version of `ruby`                                       |
| symbol   |          | Espelha o valor da opção `symbol`                           |
| style\*  |          | Espelha o valor da opção `style`                            |
| gemset   | `test`   | Optional, gets the current RVM gemset name. |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/).
O módulo será mostrado se alguma das seguintes condições for atendida:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🦀 '`                              | Uma string que representa o simbolo do Rust                                                                   |
| `detect_extensions` | `['rs']`                             | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['Cargo.toml']`                     | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold red'`                         | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `rust` module.                                                                   |

### Variáveis

| Variável  | Exemplo           | Descrição                                    |
| --------- | ----------------- | -------------------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`                       |
| numver    | `1.51.0`          | The numeric component of the `rustc` version |
| toolchain | `beta`            | A versão do toolchain                        |
| symbol    |                   | Espelha o valor da opção `symbol`            |
| style\*   |                   | Espelha o valor da opção `style`             |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Opções

| Opções              | Padrão                                   | Descrição                                                                                                     |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                              | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['sbt', 'scala']`                       | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `['.metals']`                            | Quais pastas devem ativar este módulo.                                                        |
| `símbolo`           | `'🆂 '`                                  | Uma string que representa o simbolo do Scala.                                                 |
| `style`             | `'red dimmed'`                           | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                  | Disables the `scala` module.                                                                  |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `2.13.5` | The version of `scala`            |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```

## Shell

The `shell` module shows an indicator for currently used shell.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções                 | Padrão                    | Descrição                                                                                                                              |
| ---------------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `bash_indicator`       | `'bsh'`                   | Uma string para representar o bash.                                                                                    |
| `fish_indicator`       | `'fsh'`                   | Uma string usada para representar o fish.                                                                              |
| `zsh_indicator`        | `'zsh'`                   | Uma string usada para representar o zsh.                                                                               |
| `powershell_indicator` | `'psh'`                   | Uma string usada para representar o powershell.                                                                        |
| `pwsh_indicator`       |                           | A format string used to represent pwsh. The default value mirrors the value of `powershell_indicator`. |
| `ion_indicator`        | `'ion'`                   | Uma string usada para representar o ion.                                                                               |
| `elvish_indicator`     | `'esh'`                   | Uma string usada para representar o elvish.                                                                            |
| `tcsh_indicator`       | `'tsh'`                   | Uma string usada para representar o tcsh.                                                                              |
| `xonsh_indicator`      | `'xsh'`                   | Uma string usada para representar o xonsh.                                                                             |
| `cmd_indicator`        | `'cmd'`                   | Uma string usada para representar o cmd.                                                                               |
| `nu_indicator`         | `'nu'`                    | Uma string usada para representar o nu.                                                                                |
| `unknown_indicator`    | `''`                      | Valor padrão para exibir quando o shell é desconhecido.                                                                |
| `format`               | `'[$indicator]($style) '` | O formato do módulo.                                                                                                   |
| `style`                | `'white bold'`            | O estilo do módulo.                                                                                                    |
| `disabled`             | `true`                    | Disables the `shell` module.                                                                                           |

### Variáveis

| Variável  | Padrão | Descrição                                                                  |
| --------- | ------ | -------------------------------------------------------------------------- |
| indicator |        | Mirrors the value of `indicator` for currently used shell. |
| style\*   |        | Espelha o valor da opção `style`.                          |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplos

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = '󰈺 '
powershell_indicator = '_'
unknown_indicator = 'mystery shell'
style = 'cyan bold'
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is
set to a number and meets or exceeds the specified threshold.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções          | Padrão                       | Descrição                                                                     |
| --------------- | ---------------------------- | ----------------------------------------------------------------------------- |
| `threshold`     | `2`                          | Limite de exibição.                                           |
| `format`        | `'[$symbol$shlvl]($style) '` | O formato do módulo.                                          |
| `símbolo`       | `'↕️  '`                     | The symbol used to represent the `SHLVL`.                     |
| `repeat`        | `false`                      | Causes `symbol` to be repeated by the current `SHLVL` amount. |
| `repeat_offset` | `0`                          | Decrements number of times `symbol` is repeated by the offset value           |
| `style`         | `'bold yellow'`              | O estilo do módulo.                                           |
| `disabled`      | `true`                       | Disables the `shlvl` module.                                  |

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| shlvl    | `3`     | The current value of `SHLVL`      |
| symbol   |         | Espelha o valor da opção `symbol` |
| style\*  |         | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

Using `repeat` and `repeat_offset` along with `character` module, one can get
prompt like `❯❯❯` where last character is colored appropriately for return
status code and preceding characters are provided by `shlvl`.

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '[$symbol]($style)'
repeat = true
symbol = '❯'
repeat_offset = 1
```

## Singularidade

The `singularity` module shows the current [Singularity](https://sylabs.io/singularity/) image, if inside a container
and `$SINGULARITY_NAME` is set.

### Opções

| Opções     | Padrão                           | Descrição                                                   |
| ---------- | -------------------------------- | ----------------------------------------------------------- |
| `format`   | `'[$symbol\[$env\]]($style) '` | O formato do módulo.                        |
| `símbolo`  | `''`                             | Uma string exibida antes do nome da imagem. |
| `style`    | `'bold dimmed blue'`             | O estilo do módulo.                         |
| `disabled` | `false`                          | Disables the `singularity` module.          |

### Variáveis

| Variável | Exemplo      | Descrição                         |
| -------- | ------------ | --------------------------------- |
| env      | `centos.img` | A imagem atual do Singularity     |
| symbol   |              | Espelha o valor da opção `symbol` |
| style\*  |              | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Solidity

The `solidity` module shows the currently installed version of [Solidity](https://soliditylang.org/)
The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.sol` extension

### Opções

| Opções              | Padrão                                                       | Descrição                                                                                                     |
| ------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                         | O formato do módulo.                                                                          |
| `version_format`    | `'v${major}.${minor}.${patch}'`                              | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'S '`                                                       | A format string representing the symbol of Solidity                                                           |
| \`compiler          | ['solc'] | The default compiler for Solidity.                                                            |
| `detect_extensions` | `['sol']`                                                    | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                                         | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                                         | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold blue'`                                                | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                                      | Disables this module.                                                                         |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v0.8.1` | The version of `solidity`         |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml
[solidity]
format = "via [S $version](blue bold)"
```

## Spack

The `spack` module shows the current [Spack](https://spack.readthedocs.io/en/latest/) environment, if `$SPACK_ENV` is set.

### Opções

| Opções              | Padrão                                 | Descrição                                                                                                                                                                                             |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | O número de diretórios para os quais o caminho do ambiente deve ser truncado. `0` means no truncation. Also see the [`directory`](#directory) module. |
| `símbolo`           | `'🅢  '`                               | O simbolo usado antes do nome do environment.                                                                                                                                         |
| `style`             | `'bold blue'`                          | O estilo do módulo.                                                                                                                                                                   |
| `format`            | `'via [$symbol$environment]($style) '` | O formato do módulo.                                                                                                                                                                  |
| `disabled`          | `false`                                | Disables the `spack` module.                                                                                                                                                          |

### Variáveis

| Variável    | Exemplo      | Descrição                         |
| ----------- | ------------ | --------------------------------- |
| environment | `astronauts` | O ambiente spack atual            |
| symbol      |              | Espelha o valor da opção `symbol` |
| style\*     |              | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Status

The `status` module displays the exit code of the previous command.
If $success_symbol is empty (default), the module will be shown only if the exit code is not `0`.
O código de status será convertido em um inteiro de 32 bits signed.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções                      | Padrão                                                                           | Descrição                                                                                            |
| --------------------------- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                    | O formato do módulo                                                                                  |
| `símbolo`                   | `'❌'`                                                                            | O simbolo exibido no erro de programa                                                                |
| `success_symbol`            | `''`                                                                             | O simbolo exibido no sucesso de programa                                                             |
| `not_executable_symbol`     | `'🚫'`                                                                           | O simbolo exibido quando o arquivo não é executável                                                  |
| `not_found_symbol`          | `'🔍'`                                                                           | O simbolo exibido quando o comando não é encontrado                                                  |
| `sigint_symbol`             | `'🧱'`                                                                           | O simbolo exibido no SIGINT (Ctrl + c)                                            |
| `signal_symbol`             | `'⚡'`                                                                            | O simbolo exibido em qualquer sinal                                                                  |
| `style`                     | `'bold red'`                                                                     | O estilo do módulo.                                                                  |
| `success_style`             |                                                                                  | The style used on program success (defaults to `style` if unset). |
| `failure_style`             |                                                                                  | The style used on program failure (defaults to `style` if unset). |
| `recognize_signal_code`     | `true`                                                                           | Habilita o mapeamento de sinais para códigos de saída                                                |
| `map_symbol`                | `false`                                                                          | Habilita o mapeamento de símbolos para códigos de saída                                              |
| `pipestatus`                | `false`                                                                          | Habilita o relatório de pipestatus                                                                   |
| `pipestatus_separator`      | <code>&vert;</code>                                          | O símbolo usado para separar segmentos de pipestatus (suporta formatação)         |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style) '` | O formato do módulo quando o comando é um pipeline                                                   |
| `pipestatus_segment_format` |                                                                                  | When specified, replaces `format` when formatting pipestatus segments                                |
| `disabled`                  | `true`                                                                           | Disables the `status` module.                                                        |

### Variáveis

| Variável                            | Exemplo | Descrição                                                                                                                     |
| ----------------------------------- | ------- | ----------------------------------------------------------------------------------------------------------------------------- |
| status                              | `127`   | O codígo de saída do último comando                                                                                           |
| hex_status     | `0x7F`  | O codígo de saída do último comando em hex                                                                                    |
| int                                 | `127`   | O codígo de saída do último comando                                                                                           |
| common_meaning | `ERROR` | Significa que o código não é um sinal                                                                                         |
| signal_number  | `9`     | Número do sinal correspondente ao código de saída, apenas se sinalizado                                                       |
| signal_name    | `KILL`  | Nome do sinal correspondente ao código de saída, apenas se for sinalizado                                                     |
| maybe_int      | `7`     | Contém o código de saída quando nenhum significado for encontrado                                                             |
| pipestatus                          |         | Exibição do pipeline de programas com os códigos de saída, este é apenas disponível no pipestatus_format |
| symbol                              |         | Espelha o valor da opção `symbol`                                                                                             |
| style\*                             |         | Mirrors the value of option `success_style` on program success and `failure_style` otherwise                                  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[status]
style = 'bg:blue'
symbol = '🔴 '
success_symbol = '🟢 SUCCESS'
format = '[\[$symbol$common_meaning$signal_name$maybe_int\]]($style) '
map_symbol = true
disabled = false
```

## Sudo

The `sudo` module displays if sudo credentials are currently cached.
O módulo vai ser exibido somente se as credenciais estiverem em cache.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções          | Padrão                   | Descrição                                                                                  |
| --------------- | ------------------------ | ------------------------------------------------------------------------------------------ |
| `format`        | `'[as $symbol]($style)'` | O formato do módulo                                                                        |
| `símbolo`       | `'🧙 '`                  | O simbolo exibido quando as credenciais estão em cache                                     |
| `style`         | `'bold blue'`            | O estilo do módulo.                                                        |
| `allow_windows` | `false`                  | Desde que o Windows não tem um padrão sudo, o valor padrão é desabilitado. |
| `disabled`      | `true`                   | Disables the `sudo` module.                                                |

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| symbol   |         | Espelha o valor da opção `symbol` |
| style\*  |         | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[sudo]
style = 'bold green'
symbol = '👩‍💻 '
disabled = false
```

```toml
# No windows
# $HOME\.starship\config.toml

[sudo]
allow_windows = true
disabled = false
```

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/).
O módulo será mostrado se alguma das seguintes condições for atendida:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'🐦 '`                              | Uma string que representa o simbolo do Swift                                                                  |
| `detect_extensions` | `['swift']`                          | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['Package.swift']`                  | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold 202'`                         | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `swift` module.                                                                  |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v5.2.4` | The version of `swift`            |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[swift]
format = 'via [🏎  $version](red bold)'
```

## Terraform

The `terraform` module shows the currently selected [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) and version.
It supports both Hashicorp Terraform and OpenTofu for version detection.

> [!TIP]
> By default the Terraform/OpenTofu version is not shown, since this is slow for current versions when a lot of plugins are in use.
> If you still want to enable it, [follow the example shown below](#with-terraform-version).

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf`, `.tfplan` or `.tfstate` extensions

### Opções

| Opções              | Padrão                                                  | Descrição                                                                                                     |
| ------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$workspace]($style) '`                    | A string de formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                             | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'💠'`                                                  | Uma string que é exibida antes do workspace terraform.                                        |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`                           | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                                    | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `['.terraform']`                                        | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'bold 105'`                                            | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                                 | Disables the `terraform` module.                                                              |
| `comandos`          | `[ [ 'terraform', 'version' ], [ 'tofu', 'version' ] ]` | How to detect what the Terraform version is.                                                  |

### Variáveis

| Variável  | Exemplo    | Descrição                         |
| --------- | ---------- | --------------------------------- |
| version   | `v0.12.24` | The version of `terraform`        |
| workspace | `padrão`   | O workspace atual do Terraform    |
| symbol    |            | Espelha o valor da opção `symbol` |
| style\*   |            | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

#### Com a versão do Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$version $workspace]($style) '
```

#### Sem a versão do Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$workspace]($style) '
```

## Horário

The `time` module shows the current **local** time.
The `format` configuration value is used by the [`jiff`](https://crates.io/crates/jiff) crate to control how the time is displayed. Take a look [at the jiff strftime docs](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html) to see what options are available.

> [!TIP]
> This module is disabled by default.
> To enable it, set `disabled` to `false` in your configuration file.

### Opções

| Opções            | Padrão                  | Descrição                                                                                                                                                                                                                        |
| ----------------- | ----------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | A string de formato do módulo.                                                                                                                                                                                   |
| `use_12hr`        | `false`                 | Habilita a formatação de 12 horas                                                                                                                                                                                                |
| `time_format`     | veja abaixo             | The [jiff format string](https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html) used to format the time.                                                                                                       |
| `style`           | `'bold yellow'`         | O estilo do módulo time                                                                                                                                                                                                          |
| `utc_time_offset` | `'local'`               | Define o UTC a ser usado. Either an IANA time zone name or a range from -24 &lt; x &lt; 24. Aceita floats para acomodar timezones 30/45. |
| `disabled`        | `true`                  | Disables the `time` module.                                                                                                                                                                                      |
| `time_range`      | `'-'`                   | Define o intervalo de tempo o qual o módulo será exibido. O horário deve ser especificado no formato de 24-hours                                                                                                 |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`.
Manually setting `time_format` will override the `use_12hr` setting.

### Variáveis

| Variável | Exemplo    | Descrição                        |
| -------- | ---------- | -------------------------------- |
| time     | `13:08:10` | A hora atual.    |
| style\*  |            | Espelha o valor da opção `style` |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

#### With UTC offset

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = '%T'
utc_time_offset = '-5'
time_range = '10:00:00-14:00:00'
```

#### With Timezone name

```toml
# ~/.config/starship.toml

[time]
disabled = false
time_format = '%T'
utc_time_offset = 'Europe/Berlin'
```

## Typst

The `typst` module shows the current installed version of Typst used in a project.

Por padrão, o módulo será exibido se qualquer das seguintes condições for atendida:

- The current directory contains a `template.typ` file
- The current directory contains any `*.typ` file

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'t '`                               | A format string representing the symbol of Typst                                                              |
| `style`             | `'bold #0093A7'`                     | O estilo do módulo.                                                                           |
| `detect_extensions` | `['.typ']`                           | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['template.typ']`                   | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `disabled`          | `false`                              | Disables the `typst` module.                                                                  |

### Variáveis

| Variável                           | Exemplo  | Descrição                                                            |
| ---------------------------------- | -------- | -------------------------------------------------------------------- |
| version                            | `v0.9.0` | The version of `typst`, alias for typst_version |
| typst_version | `padrão` | The current Typst version                                            |
| symbol                             |          | Espelha o valor da opção `symbol`                                    |
| style\*                            |          | Espelha o valor da opção `style`                                     |

\*: Esta variável só pode ser usada como parte de uma string de estilo

## Nome do usuário

The `username` module shows active user's username.
O módulo será mostrado se alguma das seguintes condições for atendida:

- O usuário atual é root/admin
- O usuário atual não é o mesmo que está logado
- O usuário atual esta conectado em uma sessão SSH
- The variable `show_always` is set to true
- The array `detect_env_vars` contains at least the name of one environment variable, that is set

> [!TIP]
> SSH connection is detected by checking environment variables
> `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. Se seu host SSH não tem estas variáveis configuradas, uma alternativa é definir uma delas com valor burro.

### Opções

| Opções            | Padrão                  | Descrição                                                                                    |
| ----------------- | ----------------------- | -------------------------------------------------------------------------------------------- |
| `style_root`      | `'bold red'`            | O estilo usado quando o usuário é root/admin.                                |
| `style_user`      | `'bold yellow'`         | O estilo usado para usuários não root.                                       |
| `detect_env_vars` | `[]`                    | Which environment variable(s) should trigger this module. |
| `format`          | `'[$user]($style) in '` | O formato do módulo.                                                         |
| `show_always`     | `false`                 | Always shows the `username` module.                                          |
| `disabled`        | `false`                 | Disables the `username` module.                                              |
| `aliases`         | `{}`                    | Translate system usernames to something else.                                |

### Variáveis

| Variável | Exemplo      | Descrição                                                                                                   |
| -------- | ------------ | ----------------------------------------------------------------------------------------------------------- |
| `style`  | `'red bold'` | Mirrors the value of option `style_root` when root is logged in and `style_user` otherwise. |
| `user`   | `'fulano'`   | O ID do usuário logado atualmente.                                                          |

### Exemplo

#### Always show the username

```toml
# ~/.config/starship.toml

[username]
style_user = 'white bold'
style_root = 'black bold'
format = 'user: [$user]($style) '
disabled = false
show_always = true
aliases = { "corpuser034g" = "matchai" }
```

## Vagrant

The `vagrant` module shows the currently installed version of [Vagrant](https://www.vagrantup.com/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `Vagrantfile` file

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'⍱ '`                               | Um formato de string que representa o simbolo do Vagrant.                                     |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['Vagrantfile']`                    | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'cyan bold'`                        | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                                                |

### Variáveis

| Variável | Exemplo          | Descrição                         |
| -------- | ---------------- | --------------------------------- |
| version  | `Vagrant 2.2.10` | The version of `Vagrant`          |
| symbol   |                  | Espelha o valor da opção `symbol` |
| style\*  |                  | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

The `vlang` module shows you your currently installed version of [V](https://vlang.io/).
Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Opções

| Opções              | Padrão                                       | Descrição                                                                                                     |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`         | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                                  | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'V '`                                       | Um formato de string que representa o simbolo do V                                                            |
| `detect_extensions` | `['v']`                                      | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                         | Quais pastas devem ativar este módulo.                                                        |
| `style`             | `'blue bold'`                                | O estilo do módulo.                                                                           |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                                                  |

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| version  | `v0.2`  | The version of `v`                |
| symbol   |         | Espelha o valor da opção `symbol` |
| style\*  |         | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCS

> Note the module is enabled by default but **not** included in the default list because that would be a breaking change.
> Additionally, the exact format of the module may change in the future, for example to handle right-aligned prompt.

The `vcs` module displays the current active Version Control System (VCS).
The module will be shown only if a configured VCS is currently in use.

### Opções

| Opções           | Padrão                                                      | Descrição                                                             |
| ---------------- | ----------------------------------------------------------- | --------------------------------------------------------------------- |
| `order`          | `["git", "hg", "pijul", "fossil"]`                          | The order in which to search VCSes.                   |
| `fossil_modules` | `"$fossil_branch$fossil_metrics"`                           | Modules to show when a Fossil repository is found.    |
| `git_modules`    | `"$git_branch$git_commit$git_state$git_metrics$git_status"` | Modules to show when a Git repository is found.       |
| `hg_modules`     | `"$hg_branch$hg_state"`                                     | Modules to show when a Mercurial repository is found. |
| `pijul_modules`  | `"$pijul_channel"`                                          | Modules to show when a Pijul repository is found.     |
| `disabled`       | `false`                                                     | Disables the `vcs` module.                            |

### Exemplo

```toml
# ~/.config/starship.toml

[vcs]
# Will look for Git then Pijul if not found but not for other VCSes at all
order = [
  "git",
  "pijul",
]
# Any module (except `$vcs` itself to avoid infinite loops) can be included here
git_modules = "$git_branch${custom.foo}"

# See documentation for custom modules
[custom.foo]
command = 'echo foo'
detect_files = ['foo']
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'
```

## VCSH

The `vcsh` module displays the current active [VCSH](https://github.com/RichiH/vcsh) repository.
O módulo vai ser exibido apenas se um repositório estiver em uso.

### Opções

| Opções     | Padrão                           | Descrição                                                              |
| ---------- | -------------------------------- | ---------------------------------------------------------------------- |
| `símbolo`  | `''`                             | O simbolo usado antes de exibir o nome do repositório. |
| `style`    | `'bold yellow'`                  | O estilo do módulo.                                    |
| `format`   | `'vcsh [$symbol$repo]($style) '` | O formato do módulo.                                   |
| `disabled` | `false`                          | Disables the `vcsh` module.                            |

### Variáveis

| Variável | Exemplo                                     | Descrição                         |
| -------- | ------------------------------------------- | --------------------------------- |
| repo     | `dotfiles` if in a VCSH repo named dotfiles | O nome do repositório ativo       |
| symbol   |                                             | Espelha o valor da opção `symbol` |
| style\*  | `black bold dimmed`                         | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## XMake

The `xmake` module shows the currently installed version of [XMake](https://xmake.io/). Por padrão o módulo será ativo se qualquer das condições a seguir for atendida:

- The current directory contains a `xmake.lua` file

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'△ '`                               | O simbolo usado antes da versão do cmake.                                                     |
| `detect_extensions` | `[]`                                 | Quais extensões devem acionar este módulo                                                                     |
| `detect_files`      | `['xmake.lua']`                      | []                                                        |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo                                                                         |
| `style`             | `'bold green'`                       | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `xmake` module.                                                                  |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v2.9.5` | The version of xmake              |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

## Zig

By default the `zig` module shows the currently installed version of [Zig](https://ziglang.org/).
O módulo será mostrado se alguma das seguintes condições for atendida:

- The current directory contains a `.zig` file

### Opções

| Opções              | Padrão                               | Descrição                                                                                                     |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                          |
| `version_format`    | `'v${raw}'`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `símbolo`           | `'↯ '`                               | O símbolo usado antes de exibir a versão do Zig.                                              |
| `style`             | `'bold yellow'`                      | O estilo do módulo.                                                                           |
| `disabled`          | `false`                              | Disables the `zig` module.                                                                    |
| `detect_extensions` | `['zig']`                            | Quais extensões devem ativar este módulo.                                                     |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                             |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                                        |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v0.6.0` | The version of `zig`              |
| symbol   |          | Espelha o valor da opção `symbol` |
| style\*  |          | Espelha o valor da opção `style`  |

\*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```

## Comandos Personalizados

The `custom` modules show the output of some arbitrary commands.

Esses módulos serão mostrados se alguma das seguintes condições for atendida:

- The current directory contains a file whose name is in `detect_files`
- The current directory contains a directory whose name is in `detect_folders`
- The current directory contains a file whose extension is in `detect_extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matches with `os` field if defined.

> [!TIP]
> Multiple custom modules can be defined by using a `.`.

> [!TIP]
> The order in which custom modules are shown can be individually set by including
> `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`).
> By default, the `custom` module will simply show all custom modules in the order they were defined.

> [!TIP]
> [Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules.
> Se você tem um exemplo interessante que não esta coberto lá, sinta-se livre para compartilha-lo!

> [!WARNING]
> If `unsafe_no_escape` is enabled or prior to starship v1.20 command output is printed unescaped to the prompt.
>
> Seja qual for a saída o comando irá gerar uma saída sem modificações no prompt. This means if the output
> contains shell-specific interpretable sequences, they could be interpreted on display.
> Depending on the shell, this can mean that e.g. strings enclosed by backticks are executed by the shell.
> Such sequences are usually shell specific, e.g. you can write a command module that writes bash sequences,
> e.g. `\h`, but this module will not work in a fish or zsh shell.
>
> Format strings can also contain shell specific prompt sequences, e.g.
> [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html),
> [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

### Opções

| Opções              | Padrão                          | Descrição                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------------- | ------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | O comando cuja a saída deve ser exibida. O comando será passado no stdin para o shell.                                                                                                                                                                                                                                                                              |
| `when`              | `false`                         | Either a boolean value (`true` or `false`, without quotes) or a string shell command used as a condition to show the module. In case of a string, the module will be shown if the `shell` returns a `0` status code from executing it.                                                                                                           |
| `require_repo`      | `false`                         | If `true`, the module will only be shown in paths containing a (git) repository. Esta opção, por si só, não é uma condição de exibição suficiente na ausência de outras opções.                                                                                                                                                                  |
| `shell`             |                                 | [Veja abaixo](#custom-command-shell)                                                                                                                                                                                                                                                                                                                                                                |
| `descrição`         | `'<custom module>'`             | The description of the module that is shown when running `starship explain`.                                                                                                                                                                                                                                                                                                        |
| `unsafe_no_escape`  | `false`                         | When set, command output is not escaped of characters that could be interpreted by the shell.                                                                                                                                                                                                                                                                                       |
| `detect_files`      | `[]`                            | Os arquivos que serão buscados por correspondência no diretório atual.                                                                                                                                                                                                                                                                                                              |
| `detect_folders`    | `[]`                            | Os diretórios que serão buscados por correspondência no diretório atual.                                                                                                                                                                                                                                                                                                            |
| `detect_extensions` | `[]`                            | As extensões que serão buscadas por correspondência no diretório atual.                                                                                                                                                                                                                                                                                                             |
| `símbolo`           | `''`                            | O simbolo usado antes de exibir a saída do comando.                                                                                                                                                                                                                                                                                                                                 |
| `style`             | `'bold green'`                  | O estilo do módulo.                                                                                                                                                                                                                                                                                                                                                                 |
| `format`            | `'[$symbol($output )]($style)'` | O formato do módulo.                                                                                                                                                                                                                                                                                                                                                                |
| `disabled`          | `false`                         | Disables this `custom` module.                                                                                                                                                                                                                                                                                                                                                      |
| `os`                |                                 | Nome do sistema operacional onde módulo sera exibido (unix, linux, macos, windows, ... ) [See possible values](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                                       |
| `use_stdin`         |                                 | Um valor booleano opcional que substitui se os comandos devem ser encaminhados para o shell por meio da entrada padrão ou como um argumento. Se a entrada padrão não definida for usada por padrão, a menos que o shell não a suporte (cmd, nushell). Configurar isso desativa a manipulação de argumentos específicos do shell. |
| `ignore_timeout`    | `false`                         | Ignore global `command_timeout` setting and keep running external commands, no matter how long they take.                                                                                                                                                                                                                                                                           |

### Variáveis

| Variável | Descrição                              |
| -------- | -------------------------------------- |
| output   | The output of `command` run in `shell` |
| symbol   | Espelha o valor da opção `symbol`      |
| style\*  | Espelha o valor da opção `style`       |

\*: Esta variável só pode ser usada como parte de uma string de estilo

#### Comandos personalizados de shell

`shell` accepts a non-empty list of strings, where:

- A primeira string é o caminho para o shell que executará o comando.
- Outros argumentos que serão passados para o shell.

If unset, it will fallback to STARSHIP_SHELL and then to 'sh' on Linux, and 'cmd /C' on Windows.

The `command` (and `when`, if applicable) will be passed in on stdin.

If `shell` is not given or only contains one element and Starship detects PowerShell will be used,
the following arguments will automatically be added: `-NoProfile -Command -`.
If `shell` is not given or only contains one element and Starship detects Cmd will be used,
the following argument will automatically be added: `/C` and `stdin` will be set to `false`.
If `shell` is not given or only contains one element and Starship detects Nushell will be used,
the following arguments will automatically be added: `-c` and `stdin` will be set to `false`.
Este comportamento pode ser evitado passando explicitamente argumento para o shell, ex.

```toml
shell = ['pwsh', '-Command', '-']
```

> [!WARNING]
> Make sure your custom shell configuration exits gracefully
>
> If you set a custom command, make sure that the default Shell used by starship
> will properly execute the command with a graceful exit (via the `shell`
> option).
>
> For example, PowerShell requires the `-Command` parameter to execute a one
> liner. Omitindo este parâmetro pode ser que o starship entre em um loop recursivo, onde o shell tente carrega um ambiente completo de perfil com o próprio starship novamente e portanto execute novamente o comando, entrando em um loop sem fim.
>
> Parameters similar to `-NoProfile` in PowerShell are recommended for other
> shells as well to avoid extra loading time of a custom profile on every
> starship invocation.
>
> Detecção automática de shell e adição de parâmetros estão sendo implementados atualmente, mas é possível que nem todas as shells sejam cobertas.
> [Please open an issue](https://github.com/starship/starship/issues/new/choose)
> with shell details and starship configuration if you hit such scenario.

### Exemplo

```toml
# ~/.config/starship.toml

[custom.foo]
command = 'echo foo' # shows output of command
detect_files = ['foo'] # can specify filters but wildcards are not supported
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'

[custom.time]
command = 'time /T'
detect_extensions = ['pst'] # filters *.pst files
shell = ['pwsh.exe', '-NoProfile', '-Command', '-']

[custom.time-as-arg]
command = 'time /T'
detect_extensions = ['pst'] # filters *.pst files
shell = ['pwsh.exe', '-NoProfile', '-Command']
use_stdin = false
```
