# Configuração

Para começar a configurar a starship, crie o seguinte arquivo: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Todas as configurações do starship são feitas neste arquivo [TOML](https://github.com/toml-lang/toml):

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

Você pode alterar o caminho padrão do arquivo de configuração com a variável de ambiente `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

No PowerShell (Windows) você pode adicionar a seguinte linha no seu `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Ou para Cmd (Windows) seria adicionar esta linha no seu `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Logging

Por padrão o starship grava logs de erros e warnings dentro de um arquivo chamado `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, onde a session key corresponde a instancia do seu terminal. Isto, no entanto pode ser alterado usando a variável de ambiente `STARSHIP_CACHE`:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

No PowerShell (Windows) você pode adicionar a seguinte linha no seu `$PROFILE`:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Ou para Cmd (Windows) seria adicionar esta linha no seu `starship.lua`:

```lua
os.setenv('STARSHIP_CACHE', 'C:\\Users\\user\\AppData\\Local\\Temp')
```

### Terminologia

**Módulo**: Um componente no prompt que fornece informações baseado no contexto do seu SO. Por exemplo, o "nodejs"módulo exibe a versão do Node.js que está instalada no computador, se o diretório atual for um projeto Node.js.

**Variável**: Um pequeno subcomponente que contem informações fornecidas pelo módulo. Por exemplo, a variável "version" no módulo "nodejs"contem a versão atual do Node.js.

Por convenção, a maioria dos módulos tem um prefixo de cor (e.x. `via` no "nodejs") e um espaço vazio para sufixo.

### Strings

In TOML syntax, [text values](https://toml.io/en/v1.0.0#string) are declared with `'`, `"`, `'''`, or `"""`.

The following Starship syntax symbols have special usage in a format string and must be escaped to display as that character: `$ [ ] ( )`.

| Symbol | Type                      | Notes                                                  |
| ------ | ------------------------- | ------------------------------------------------------ |
| `'`    | literal string            | less escaping                                          |
| `"`    | string                    | more escaping                                          |
| `'''`  | multi-line literal string | less escaping                                          |
| `"""`  | multi-line string         | more escaping, newlines in declarations can be ignored |

Por exemplo:

```toml
# literal string
format = '☺\☻ '

# regular string
format = "☺\\☻ "

# escaping Starship symbols
format = '\[\$\] '
```

When using line breaks, multi-line declarations can be used. For example, if you want to print a `$` symbol on a new line, the following values for `format` are equivalent:

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

### Formatação de Strings

As strings de formato são o formato com o qual um módulo imprime todas as suas variáveis. A maioria dos módulos tem uma entrada chamada `format` que configura o formato de exibição do módulo. Você pode usar textos, variáveis e grupos de texto em uma string de formato.

#### Variável

Uma variável contém um símbolo `$` seguido pelo nome da variável. O nome de uma variável pode conter apenas letras, números e `_`.

Por exemplo:

- `'$version'` is a format string with a variable named `version`.
- `'$git_branch$git_commit'` is a format string with two variables named `git_branch` and `git_commit`.
- `'$git_branch $git_commit'` has the two variables separated with a space.

#### Grupo de Texto

Um grupo de texto é composto de duas partes diferentes.

A primeira parte, que está entre um `[]`, é uma [string de formato](#format-strings). Você pode adicionar textos, variáveis ou até mesmo grupos de texto aninhados nele.

Na segunda parte, que está dentro de um `()`, está uma [string de estilo](#style-strings). Isso pode ser usado para estilizar a primeira parte.

Por exemplo:

- `'[on](red bold)'` will print a string `on` with bold text colored red.
- `'[⌘ $version](bold green)'` will print a symbol `⌘` followed by the content of variable `version`, with bold text colored green.
- `'[a [b](red) c](green)'` will print `a b c` with `b` red, and `a` and `c` green.

#### Estilo dos textos

A maioria dos módulos no starship permite que você configure seus estilos de exibição. Isso é feito com uma entrada (normalmente chamada de `estilo`) que é uma string especificando a configuração. Aqui estão alguns exemplos de strings de estilo junto com o que elas fazem. Para obter detalhes sobre a sintaxe completa, consulte o [guia de configuração avançada](/advanced-config/).

- `'fg:green bg:blue'` sets green text on a blue background
- `'bg:blue fg:bright-green'` sets bright green text on a blue background
- `'bold fg:27'` sets bold text with [ANSI color](https://i.stack.imgur.com/KTSQa.png) 27
- `'underline bg:#bf5700'` sets underlined text on a burnt orange background
- `'bold italic fg:purple'` sets bold italic purple text
- `''` explicitly disables all styling

Observe que a aparência do estilo será controlada pelo emulador de terminal. Por exemplo, alguns emuladores de terminal irão clarear as cores em vez de colocar o texto em negrito, e alguns temas de cores usam os mesmos valores para as cores normais e brilhantes. Além disso, para obter texto em itálico, seu terminal deve suportar itálico.

#### Formatação de String Condicional

Uma string de formato condicional envolta de `(` e `)` não será renderizada se todas as variáveis internas estiverem vazias.

Por exemplo:

- `'(@$region)'` will show nothing if the variable `region` is `None` or empty string, otherwise `@` followed by the value of region.
- `'(some text)'` will always show nothing since there are no variables wrapped in the braces.
- When `$combined` is a shortcut for `\[$a$b\]`, `'($combined)'` will show nothing only if `$a` and `$b` are both `None`. This works the same as `'(\[$a$b\] )'`.

### Negative matching

Many modules have `detect_extensions`, `detect_files`, and `detect_folders` variables. These take lists of strings to match or not match. "Negative" options, those which should not be matched, are indicated with a leading '!' character. The presence of _any_ negative indicator in the directory will result in the module not being matched.

Extensions are matched against both the characters after the last dot in a filename, and the characters after the first dot in a filename. For example, `foo.bar.tar.gz` will be matched against `bar.tar.gz` and `gz` in the `detect_extensions` variable. Files whose name begins with a dot are not considered to have extensions at all.

To see how this works in practice, you could match TypeScript but not MPEG Transport Stream files thus:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Prompt de Comando

Esta é a lista de opções de configuração em todo o prompt.

### Opções

| Opções            | Padrão                         | Descrição                                                                                                                                                                        |
| ----------------- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | Configura o formato do prompt.                                                                                                                                                   |
| `right_format`    | `''`                           | Veja [Ativa o prompt direito](/advanced-config/#enable-right-prompt)                                                                                                             |
| `scan_timeout`    | `30`                           | Tempo limite para escanear arquivos (em milissegundos).                                                                                                                          |
| `command_timeout` | `500`                          | Tempo limite de execução de comandos pelo starship (em milissegundos).                                                                                                           |
| `add_newline`     | `true`                         | Insere linha vazia entre os prompts do shell.                                                                                                                                    |
| `palette`         | `''`                           | Sets which color palette from `palettes` to use.                                                                                                                                 |
| `palettes`        | `{}`                           | Collection of color palettes that assign [colors](/advanced-config/#style-strings) to user-defined names. Note that color palettes cannot reference their own color definitions. |

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

O padrão `format` é usado para definir o formato do prompt, se estiver vazio ou nenhum `format` for fornecido. O padrão é como mostrado:

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
$directory\
$vcsh\
$fossil_branch\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$pijul_channel\
$docker_context\
$package\
$c\
$cmake\
$cobol\
$daml\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$fennel\
$golang\
$guix_shell\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$gradle\
$lua\
$nim\
$nodejs\
$ocaml\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$raku\
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
$buf\
$nix_shell\
$conda\
$meson\
$spack\
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
$os\
$container\
$shell\
$character"""
```

Se você quiser apenas estender o formato padrão, você pode usar `$all`; os módulos que você adicionar explicitamente ao formato não serão duplicados. Ex.

```toml
# Move the directory to the second line
format = '$all$directory$character'
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials. The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process` or `sso_start_url` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice. If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

Quando usar [aws-vault](https://github.com/99designs/aws-vault), o perfil é lido da variável de ambiente `AWS_VAULT` e o tempo de expiração de credenciais é lido da variável de ambiente `AWS_SESSION_EXPIRATION`.

Quando usar [awsu](https://github.com/kreuzwerker/awsu) o perfil é lido da varável de env `AWSU_PROFILE`.

Quando usar [AWSume](https://awsu.me) o perfil é lido da variável `AWSUME_PROFILE` e o tempo de expiração de credenciais é lida da variável de env `AWSUME_EXPIRATION`.

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials` falls back to the `x_security_token_expires` key.

### Opções

| Opções              | Padrão                                                                | Descrição                                                                                                                     |
| ------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | O formato do módulo.                                                                                                          |
| `symbol`            | `'☁️ '`                                                               | O símbolo usado antes de exibir o perfil atual da AWS.                                                                        |
| `region_aliases`    | `{}`                                                                  | Tabela de aleases de regiões a serem exibidas, além do nome da AWS.                                                           |
| `profile_aliases`   | `{}`                                                                  | Tabela de apelidos de perfil a serem exibidos além do nome da AWS.                                                            |
| `style`             | `'bold yellow'`                                                       | O estilo do módulo.                                                                                                           |
| `expiration_symbol` | `X`                                                                   | O simbolo exibido quando as credenciais temporárias estão expiradas.                                                          |
| `disabled`          | `false`                                                               | Desabilita o módulo `AWS`.                                                                                                    |
| `force_display`     | `false`                                                               | Se `true` exibe as informações mesmo que `credentials`, `credential_process` ou `sso_start_url` não tenham sido configurados. |

### Variáveis

| Variável  | Exemplo          | Descrição                            |
| --------- | ---------------- | ------------------------------------ |
| region    | `ap-northeast-1` | A região atual do AWS                |
| profile   | `astronauts`     | O perfil atual do AWS                |
| duration  | `2h27m20s`       | A duração temporária das credenciais |
| symbol    |                  | Espelha o valor da opção `symbol`    |
| style\* |                  | Espelha o valor da opção `style`     |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

O módulo `azure` exibe a assinatura Azure atual. This is based on showing the name of the default subscription or the username, as defined in the `~/.azure/azureProfile.json` file.

### Opções

| Variável   | Padrão                                   | Descrição                                      |
| ---------- | ---------------------------------------- | ---------------------------------------------- |
| `format`   | `'on [$symbol($subscription)]($style) '` | O formato que o módulo Azure será renderizado. |
| `symbol`   | `'ﴃ '`                                   | O símbolo usado no formato.                    |
| `style`    | `'blue bold'`                            | O estilo usado no formato.                     |
| `disabled` | `true`                                   | Desabilita o módulo `azure`.                   |

### Exemplos

#### Display Subscription Name

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = 'ﴃ '
style = 'blue bold'
```

#### Display Username

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "em [$symbol($username)]($style) "
symbol = "ﴃ "
style = "blue bold"
```

## Bateria

O módulo `battery` exibe o quanto a bateria do dispositivo está carregada e o estado atual de carregamento. O módulo é visível somente quando a bateria está abaixo de 10%.

### Opções

| Opções               | Padrão                            | Descrição                                                    |
| -------------------- | --------------------------------- | ------------------------------------------------------------ |
| `full_symbol`        | `' '`                            | O simbolo exibido quando a bateria estiver cheia.            |
| `charging_symbol`    | `' '`                            | O simbolo exibido quando a bateria está carregando.          |
| `discharging_symbol` | `' '`                            | O simbolo exibido quando a bateria está descarregando.       |
| `unknown_symbol`     | `' '`                            | O simbolo exibido quando o estado da bateria é desconhecido. |
| `empty_symbol`       | `' '`                            | O simbolo exibido quando o estado da bateria é vazio.        |
| `format`             | `'[$symbol$percentage]($style) '` | O formato do módulo.                                         |
| `display`            | [link](#battery-display)          | Limite de exibição e estilo para o módulo.                   |
| `disabled`           | `false`                           | Desabilita o módulo `battery`.                               |

### Exemplo

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### Indicador de bateria

A configuração `display` é usada para definir quando o indicador de bateria deve ser exibido (threshold), qual deve ser o simbolo(symbol) e como você gostaria de exibir (style). Se nenhum `display` for fornecido. O padrão é como mostrado:

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

O valor padrão das opções `charging_symbol` e `discharging_symbol`é respectivamente o valor das opções `battery`'s `charging_symbol` e `discharging_symbol`.

#### Opções

A opção `display` é um array da seguinte tabela.

| Opções               | Padrão       | Descrição                                                                                          |
| -------------------- | ------------ | -------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | O limite superior para exibição.                                                                   |
| `style`              | `'red bold'` | O estilo usado para exibir quando estiver em uso.                                                  |
| `charging_symbol`    |              | Simbolo opcional, mostrado quando a opção estiver em uso, o simbolo padrão é `charging_symbol`.    |
| `discharging_symbol` |              | Simbolo opcional, mostrado quando a opção estiver em uso, o simbolo padrão é `discharging_symbol`. |

#### Exemplo

```toml
[[battery.display]] # 'bold red' style and discharging_symbol when capacity is between 0% and 10%
threshold = 10
style = 'bold red'

[[battery.display]] # 'bold yellow' style and 💦 symbol when capacity is between 10% and 30%
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦'

# when capacity is over 30%, the battery indicator will not be displayed
```

## Buf

O módulo `buf` mostra a versão instalada do [Buf](https://buf.build). Por padrão, o módulo é mostrado se todas as seguintes condições forem atendidas:

- A CLI [`buf`](https://github.com/bufbuild/buf) está instalada.
- O atual diretório contém um [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml) ou arquivo de configuração [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml).

### Opções

| Opções              | Padrão                                          | Descrição                                         |
| ------------------- | ----------------------------------------------- | ------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | O formato do módulo `buf`.                        |
| `version_format`    | `'v${raw}'`                                     | A versão formatada.                               |
| `symbol`            | `'🐃 '`                                          | O símbolo usado antes de exibir a versão do Buf.  |
| `detect_extensions` | `[]`                                            | Quais extensões devem ativar este módulo.         |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | Quais nomes de arquivos devem ativar este módulo. |
| `detect_folders`    | `[]`                                            | Quais pastas devem ativar este módulo.            |
| `style`             | `'bold blue'`                                   | O estilo do módulo.                               |
| `disabled`          | `false`                                         | Desabilita o módulo `elixir`.                     |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| `version` | `v1.0.0` | A versão do `buf`                 |
| `symbol`  |          | Espelha o valor da opção `symbol` |
| `style`*  |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

The `bun` module shows the currently installed version of the [bun](https://bun.sh) JavaScript runtime. Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `bun.lockb`
- O diretório atual conter um arquivo `bunfig.toml`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🍞 '`                               | A format string representing the symbol of Bun.                                     |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['bun.lockb', 'bunfig.toml']`       | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold red'`                         | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Disables the `bun` module.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v0.1.4` | The version of `bun`              |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

O módulo `c` mostra algumas informações sobre o seu compilador de C. Por padrão o módulo será exibido se o diretório atual contém um arquivo `.c` ou `.h`.

### Opções

| Opções              | Padrão                                                                      | Descrição                                                                           |
| ------------------- | --------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                | A string de formato do módulo.                                                      |
| `version_format`    | `'v${raw}'`                                                                 | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'C '`                                                                      | O símbolo utilizado antes de exibir os detalhes do compilador                       |
| `detect_extensions` | `['c', 'h']`                                                                | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                                                        | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                                                        | Quais pastas devem ativar este módulo.                                              |
| `commands`          | [ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ] | Como detectar qual é o compilador                                                   |
| `style`             | `'bold 149'`                                                                | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                                     | Desabilita o módulo `c`.                                                            |

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| name     | clang   | O nome do compilador              |
| version  | 13.0.0  | A versão do compilador            |
| symbol   |         | Espelha o valor da opção `symbol` |
| style    |         | Espelha o valor da opção `style`  |

Note que `version` não está no formato padrão.

### Comandos

A opção `commands` aceita uma lista de comandos para determinar a versão e o nome do compilador.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship tentará executar cada comando até que obtenha um resultado no STDOUT.

Se um compilador C não é suportado por este módulo, você pode solicitá-lo [criando uma issue no GitHub](https://github.com/starship/starship/).

### Exemplo

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## Caractere

O módulo `character` exibe um caracter (normalmente uma seta) ao lado de onde o texto começa a ser inserido no terminal.

O caractere vai te dizer se o ultimo comando foi bem sucedido ou não. Você pode fazer isto de duas maneiras:

- alterando a cor (`red`/`green`)
- alterando a forma (`❯`/`✖`)

Por padrão ele apenas muda de cor. Se você deseja alterar o formato de uma olhada [neste exemplo](#with-custom-error-shape).

::: warning

`vimcmd_symbol` is only supported in cmd, fish and zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol` are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

:::

### Opções

| Opções                      | Padrão               | Descrição                                                                                   |
| --------------------------- | -------------------- | ------------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | O formato da string usado antes da entrada dos textos.                                      |
| `success_symbol`            | `'[❯](bold green)'`  | O formato da string usado antes da entrada de texto se o comando anterior for bem-sucedido. |
| `error_symbol`              | `'[❯](bold red)'`    | O formato de string usado antes da entrada de texto se o comando anterior tiver falhado.    |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | O fromato de string usado antes da entrada de texto se o shell esta no vim normal mode.     |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim `replace_one` mode.     |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | The format string used before the text input if the shell is in vim replace mode.           |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | The format string used before the text input if the shell is in vim visual mode.            |
| `disabled`                  | `false`              | Desabilita o módulo `character`.                                                            |

### Variáveis

| Variável | Exemplo | Descrição                                                                                                |
| -------- | ------- | -------------------------------------------------------------------------------------------------------- |
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
vicmd_symbol = '[V](bold green) '
```

## CMake

O módulo `cmake` exibe a versão instalada do [CMake](https://cmake.org/). Por padrão o módulo será ativo se qualquer das condições a seguir for atendida:

- O diretorio atual cotem um arquivo `CMakeLists.txt`
- O diretorio atual tem um arquivo `CMakeCache.txt`

### Opções

| Opções              | Padrão                                 | Descrição                                                                            |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`   | O formato do módulo.                                                                 |
| `version_format`    | `'v${raw}'`                            | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'△ '`                                 | O simbolo usado antes da versão do cmake.                                            |
| `detect_extensions` | `[]`                                   | Quais extensões devem acionar este módulo                                            |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | []                                                                                   |
| `detect_folders`    | `[]`                                   | Quais pastas devem ativar este módulo                                                |
| `style`             | `'bold blue'`                          | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                | Desabilita o módulo `cmake`.                                                         |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v3.17.3` | A versão do cmake                 |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

## COBOL / GNUCOBOL

O módulo `cobol` exibe a versão instalada atual do COBOL. Por padrão, o módulo será exibido se qualquer das seguintes condições for atendida:

- O diretório atual tem qualquer arquivo com extensão `.cob` or `.COB`
- O diretório atual tenham qualquer arquivo com extensão `.cbl` or `.CBL`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `symbol`            | `'⚙️ '`                              | O simbolo usado antes de exibir a versão do COBOL.                                  |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold blue'`                        | O estilo do módulo.                                                                 |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `disabled`          | `false`                              | Desabilita o módulo `cobol`.                                                        |

### Variáveis

| Variável  | Exemplo    | Descrição                         |
| --------- | ---------- | --------------------------------- |
| version   | `v3.1.2.0` | A versão do `cobol`               |
| symbol    |            | Espelha o valor da opção `symbol` |
| style\* |            | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

## Tempo de execução do comando

O módulo `cmd_duration` exibi o tempo que o ultimo comando levou para executar. O módulo vai exibir somente se o comando levar mais de dois segundos, ou o valor de configuração `min_time` existir.

::: warning Não utilize o DEBUG-trap no Bash

Se você esta rodando o Starship no `bash`, você não deve ativar a armadilha `DEBUG` após rodar `eval $(starship init $0)`, ou este módulo **vai** quebrar.

:::

Usuários do bash que precisam de funções pre-executadas podem usar [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simplesmente defina os arrays `preexec_functions` e `precmd_functions` antes de rodar `eval $(starship init $0)`, e depois pode proceder normalmente.

### Opções

| Opções                 | Padrão                        | Descrição                                                                                                                                                                                           |
| ---------------------- | ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | Duração mais curta para exibir o tempo (em milissegundos).                                                                                                                                          |
| `show_milliseconds`    | `false`                       | Exibir milissegundos ou invés de segundos para duração.                                                                                                                                             |
| `format`               | `'took [$duration]($style) '` | O formato do módulo.                                                                                                                                                                                |
| `style`                | `'bold yellow'`               | O estilo do módulo.                                                                                                                                                                                 |
| `disabled`             | `false`                       | Desabilita o módulo `cmd_duration`.                                                                                                                                                                 |
| `show_notifications`   | `false`                       | Exibi notificações no desktop quando o comando for concluído.                                                                                                                                       |
| `min_time_to_notify`   | `45_000`                      | Tempo minimo para notificação (em milissegundos).                                                                                                                                                   |
| `notification_timeout` |                               | Duração para mostrar a notificação (em milissegundos). Se não estiver definido, o tempo limite de notificação será determinado pelo daemon. Nem todos os daemons de notificação aceitam essa opção. |

### Variáveis

| Variável  | Exemplo  | Descrição                                 |
| --------- | -------- | ----------------------------------------- |
| duration  | `16m40s` | O tempo que levou para executar o comando |
| style\* |          | Espelha o valor da opção `style`          |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

O módulo `conda` exibe o ambiente atual do [Conda](https://docs.conda.io/en/latest/), se o `$CONDA_DEFAULT_ENV` estiver definido.

::: tip

Isso não suprime o modificador de prompt do conda, você pode executar `conda config --set changeps1 False`.

:::

### Opções

| Opções              | Padrão                                 | Descrição                                                                                                                                                                                                  |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | O número de diretórios do envirionment path deve ser truncado, se o environment foi criado via `conda create -p [path]`. `0` quer dizer sem truncação. Também consulte o módulo [`directory`](#directory). |
| `symbol`            | `'🅒 '`                                 | O simbolo usado antes do nome do environment.                                                                                                                                                              |
| `style`             | `'bold green'`                         | O estilo do módulo.                                                                                                                                                                                        |
| `format`            | `'via [$symbol$environment]($style) '` | O formato do módulo.                                                                                                                                                                                       |
| `ignore_base`       | `true`                                 | Ignora o environment `base` quando ativado.                                                                                                                                                                |
| `disabled`          | `false`                                | Desabilita o módulo `conda`.                                                                                                                                                                               |

### Variáveis

| Variável    | Exemplo      | Descrição                         |
| ----------- | ------------ | --------------------------------- |
| environment | `astronauts` | O environment atual do conda      |
| symbol      |              | Espelha o valor da opção `symbol` |
| style\*   |              | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## Container

O módulo `container` exibe um símbolo e nome do contêiner, se dentro de um container.

### Opções

| Opções     | Padrão                             | Descrição                                         |
| ---------- | ---------------------------------- | ------------------------------------------------- |
| `symbol`   | `'⬢'`                              | O símbolo mostrado, quando dentro de um contêiner |
| `style`    | `'bold red dimmed'`                | O estilo do módulo.                               |
| `format`   | `'[$symbol \[$name\]]($style) '` | O formato do módulo.                              |
| `disabled` | `false`                            | Desabilita o módulo `container`.                  |

### Variáveis

| Variável  | Exemplo             | Descrição                         |
| --------- | ------------------- | --------------------------------- |
| name      | `fedora-toolbox:35` | O nome do contêiner               |
| symbol    |                     | Espelha o valor da opção `symbol` |
| style\* |                     | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

O módulo `crystal` exibe a versão instalada atual do [Crystal](https://crystal-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `shard.yml`
- O diretório atual contem um arquivo `.cr`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `symbol`            | `'🔮 '`                               | O símbolo usado antes de exibir a versão do crystal.                                |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | O estilo do módulo.                                                                 |
| `detect_extensions` | `['cr']`                             | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['shard.yml']`                      | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `disabled`          | `false`                              | Desabilita o módulo `crystal`.                                                      |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v0.32.1` | A versão do `crystal`             |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

The `daml` module shows the currently used [Daml](https://www.digitalasset.com/developers) SDK version when you are in the root directory of your Daml project. The `sdk-version` in the `daml.yaml` file will be used, unless it's overridden by the `DAML_SDK_VERSION` environment variable. Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `daml.yaml`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                 |
| `version_format`    | `'v${raw}'`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'Λ '`                               | A format string representing the symbol of Daml                                      |
| `style`             | `'bold cyan'`                        | O estilo do módulo.                                                                  |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `['daml.yaml']`                      | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `disabled`          | `false`                              | Disables the `daml` module.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.2.0` | The version of `daml`             |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

O módulo `dart` exibe a versão atual instalada do [Dart](https://dart.dev/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem algum arquivo com extensão `.dart`
- O diretório atual contem um diretório `.dart_tool`
- O diretório atual contem um arquivo `pubspec.yaml`, `pubspec.yml` ou `pubspec.lock`

### Opções

| Opções              | Padrão                                            | Descrição                                                                            |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`              | O formato do módulo.                                                                 |
| `version_format`    | `'v${raw}'`                                       | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🎯 '`                                            | Um formato de string que representa o simbolo do Dart                                |
| `detect_extensions` | `['dart']`                                        | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `['.dart_tool']`                                  | Quais pastas devem ativar este módulo.                                               |
| `style`             | `'bold blue'`                                     | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                           | Desabilita o módulo `dart`.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.8.4` | The version of `dart`             |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

O módulo `deno` exibe a versão instalada atual do [Deno](https://deno.land/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tenha um arquivo `deno.json`, `deno.jsonc`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js`

### Opções

| Opções              | Padrão                                                                  | Descrição                                                                            |
| ------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                    | O formato do módulo.                                                                 |
| `version_format`    | `'v${raw}'`                                                             | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦕 '`                                                                  | Um formato de string que representa o simbolo do Deno                                |
| `detect_extensions` | `[]`                                                                    | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                                    | Quais pastas devem ativar este módulo.                                               |
| `style`             | `'green bold'`                                                          | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                 | Desabilita o módulo `deno`.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.8.3` | A versão do `deno`                |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## Diretório

O módulo `directory` exibe o caminho do diretório atual, truncando as três pastas pai. Seu diretório será truncando na raiz do repositório git que você estiver atualmente.

Quando usar a opção de estilo fish pwd, ao invés de esconder o caminho que é truncado, você vai ver um nome encurtado de cada diretório baseado no número que você habilitar para a opção.

Por exemplo, dado `~/Dev/Nix/nixpkgs/pkgs` onde `nixpkgs` é o repositório raiz e a opção esta definida para `1`. Você verá `~/D/N/nixpkgs/pkgs`, enquanto antes seria `nixpkgs/pkgs`.

### Opções

| Opções                   | Padrão                                                                                                                       | Descrição                                                                                                              |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | O número de pastas pais do diretório atual que serão truncadas.                                                        |
| `truncate_to_repo`       | `true`                                                                                                                       | Seu diretório será truncado ou não para a raiz do repositório git atual.                                               |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | O formato do módulo.                                                                                                   |
| `style`                  | `'bold cyan'`                                                                                                                | O estilo do módulo.                                                                                                    |
| `disabled`               | `false`                                                                                                                      | Desabilita o módulo `directory`.                                                                                       |
| `read_only`              | `'🔒'`                                                                                                                        | O simbolo que indica que o diretório atual é somente leitura.                                                          |
| `read_only_style`        | `'red'`                                                                                                                      | O estilo para o simbolo de somente leitura.                                                                            |
| `truncation_symbol`      | `''`                                                                                                                         | O simbolo para prefixo de caminhos truncados. eg: '…/'                                                                 |
| `before_repo_root_style` |                                                                                                                              | The style for the path segment above the root of the git repo. O valor padrão é equivalente a `style`.                 |
| `repo_root_style`        |                                                                                                                              | O estilo para a raiz do repositório git. O valor padrão é equivalente a `style`.                                       |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | The format of a git repo when `before_repo_root_style` and `repo_root_style` is defined.                               |
| `home_symbol`            | `'~'`                                                                                                                        | O simbolo para indicar o diretório home.                                                                               |
| `use_os_path_sep`        | `true`                                                                                                                       | Use o separador de caminho específico do sistema opracional em vez de sempre usar `/` (por exemplo, `\` no Windows) |

<details>
<summary>Este módulo tem algumas configurações avançadas que controlam como o diretório é exibido.</summary>

| Opções Avançadas            | Padrão | Descrição                                                                                                                                                             |
| --------------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substituições`             |        | Uma tabela de substituições para fazer no path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`    | O número de caracteres para usar quando aplicado no path logico do fish shell pwd.                                                                                    |
| `use_logical_path`          | `true` | Se `true` exibe um caminho lógico originado do shell via `PWD` ou`--logical-path`. Se `false` em vez disso, exibe o caminho do filesystem com os symlinks resolvidos. |

`substitutions` aceita você definir substituições arbitrarias para strings literais que ocorra no path, por exemplo prefixos de rede longos ou diretórios de desenvolvimento (ex:. Java). Note isto irá desabilita o estilo PWD do fish.

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length` interage com as opções de truncação padrão de uma forma que pode suprimir no começo: se não for zero, os componentes do path que normalmente seriam truncados são exibidos com todos caracteres. Por exemplo, o caminho `/built/this/city/on/rock/and/roll`, que normalmente seria exibido como`rock/and/roll`, seria exibido como `/b/t/c/o/rock/and/roll` com`fish_style_pwd_dir_length = 1`--os componentes de path que normalmente seriam removidos são exibidos com um único caractere. Para `fish_style_pwd_dir_length = 2`, seria `/bu/th/ci/on/rock/and/roll`.

</details>

### Variáveis

| Variável  | Exemplo               | Descrição                        |
| --------- | --------------------- | -------------------------------- |
| path      | `'D:/Projetos'`       | O caminho do diretório atual     |
| style\* | `'black bold dimmed'` | Espelha o valor da opção `style` |

*: Esta variável só pode ser usada como parte de uma string de estilo

<details>
<summary>Os repositórios git têm variáveis adicionais.</summary>

Vamos considerar o caminho `/caminho/para/home/git_repo/src/lib`

| Variável           | Exemplo                 | Descrição                                           |
| ------------------ | ----------------------- | --------------------------------------------------- |
| before_root_path | `'/caminho/para/home/'` | O caminho antes do caminho do diretório raiz do git |
| repo_root          | `'git_repo'`            | O nome do diretório raiz do git                     |
| path               | `'/src/lib'`            | O caminho restante                                  |
| style              | `'black bold dimmed'`   | Espelha o valor da opção `style`                    |
| repo_root_style  | `'underline white'`     | Estilo para o nome do diretório raiz do git         |

</details>

### Exemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Docker Context

O módulo `docker_context` exibe o [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) ativo atualmente se não estiver definido como `default` ou se as variáveis de ambiente `DOCKER_MACHINE_NAME`, `DOCKER_HOST` ou `DOCKER_CONTEXT` estiverem definidas (iram sobrescrever o contexto atual).

### Opções

| Opções              | Padrão                                                        | Descrição                                                                            |
| ------------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol$context]($style) '`                            | O formato do módulo.                                                                 |
| `symbol`            | `'🐳 '`                                                        | O simbolo usado antes de exibir a versão do contexto docker.                         |
| `only_with_files`   | `true`                                                        | Exibe somente quando houver um arquivo                                               |
| `detect_extensions` | `[]`                                                          | Quais extensões devem acionar este módulo (precisa que `only_with_files` seja true). |
| `detect_files`      | `['docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Quais arquivos devem acionar este módulo (precisa que `only_with_files` seja true).  |
| `detect_folders`    | `[]`                                                          | Quais pastas devem acionar este módulo (precisa que `only_with_files` seja true).    |
| `style`             | `'blue bold'`                                                 | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                       | Desabilita o módulo `docker_context`.                                                |

### Variáveis

| Variável  | Exemplo        | Descrição                         |
| --------- | -------------- | --------------------------------- |
| context   | `test_context` | O contexto atual do docker        |
| symbol    |                | Espelha o valor da opção `symbol` |
| style\* |                | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

O módulo `dotnet` exibe a versão relevante do [.NET Core SDK](https://dotnet.microsoft.com/) para o directório atual. Se o SDK foi predefinido no diretório atual, a versão será exibida. Caso contrário, o módulo exibe a versão mais recente instalada do SDK.

Por padrão, este módulo só será exibido no seu prompt quando um ou mais dos seguintes arquivos estiverem presentes no diretório atual:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

Você também precisará do SDK do .NET Core instalado para usá-lo corretamente.

Internamente, este módulo usa um mecanismo próprio para detecção da versão. Geralmente é duas vezes mais rápido que executar `dotnet --version`, mas pode exibir uma versão errada se o seu projeto .NET tiver o layout de diretórios incomum. Se precisão for mais importante que velocidade, você pode desabilitar o mecanismo definindo `heuristic = false` nas opções do módulo.

O módulo também mostrará o Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>) quando houver um arquivo `.csproj` no diretório atual.

### Opções

| Opções              | Padrão                                                                                                  | Descrição                                                                           |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                           | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                                                                             | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'.NET '`                                                                                               | O símbolo usado na frente da versão do dotnet.                                      |
| `heuristic`         | `true`                                                                                                  | Usa a detecção de versão rápida para manter o starship ligeiro e hábil.             |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                                                                                    | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold blue'`                                                                                           | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                                                                 | Desabilita o módulo `dotnet`.                                                       |

### Variáveis

| Variável  | Exemplo          | Descrição                                         |
| --------- | ---------------- | ------------------------------------------------- |
| version   | `v3.1.201`       | A versão do `dotnet`                              |
| tfm       | `netstandard2.0` | O Target Framework Moniker usado no projeto atual |
| symbol    |                  | Espelha o valor da opção `symbol`                 |
| style\* |                  | Espelha o valor da opção `style`                  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

O módulo `elixir` exibe a versão instalada do [Elixir](https://elixir-lang.org/) e [Erlang/OTP](https://erlang.org/doc/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `mix.exs`.

### Opções

| Opções              | Padrão                                                      | Descrição                                                                           |
| ------------------- | ----------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | O formato do módulo elixir.                                                         |
| `version_format`    | `'v${raw}'`                                                 | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💧 '`                                                      | O símbolo usado na frente da versão do Elixir ou Erlang.                            |
| `detect_extensions` | `[]`                                                        | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['mix.exs']`                                               | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                                        | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold purple'`                                             | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                     | Desabilita o módulo `elixir`.                                                       |

### Variáveis

| Variável    | Exemplo | Descrição                         |
| ----------- | ------- | --------------------------------- |
| version     | `v1.10` | A versão do `elixir`              |
| otp_version |         | A versão otp do `elixir`          |
| symbol      |         | Espelha o valor da opção `symbol` |
| style\*   |         | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

O módulo `elm` exibe a versão instalada do [Elm](https://elm-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `elm.json`
- O diretório atual contem o arquivo `elm-package.json`
- O diretório atual contem um arquivo `.elm-version`
- O diretório atual contem uma pasta `elm-stuff`
- O diretório atual contém arquivos `*.elm`

### Opções

| Opções              | Padrão                                             | Descrição                                                                           |
| ------------------- | -------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`               | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                        | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🌳 '`                                             | O formato de string que representa o simbolo do Elm.                                |
| `detect_extensions` | `['elm']`                                          | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `['elm-stuff']`                                    | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'cyan bold'`                                      | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                            | Desabilita o módulo `elm`.                                                          |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v0.19.1` | A versão do `elm`                 |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## Variáveis de Ambiente

O módulo `env_var` exibe o valor atual de uma variável de ambiente selecionada. O módulo vai exibir somente se algumas das condições a seguir for atendida:

- A opção de configuração da `variable` corresponde a uma variável existente
- A configuração `variable` não está definida, mas a `default` está

::: tip

The order in which env_var modules are shown can be individually set by including `${env_var.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `env_var` module will simply show all env_var modules in the order they were defined.

:::

::: tip

Múltiplas variáveis de ambiente podem ser exibidas usando um `.`. (Veja o exemplo) se a configuração `variable` não é definida, o módulo irá exibir o valor da variável após o caractere `.`.

Exemplo: a configuração a seguir irá mostrar o valor da variável de ambiente USER

```toml
# ~/.config/starship.toml

[env_var.USER]
default = 'unknown user'
```

:::

### Opções

| Opções        | Padrão                         | Descrição                                                                    |
| ------------- | ------------------------------ | ---------------------------------------------------------------------------- |
| `symbol`      | `""`                           | O símbolo usado antes de exibir o valor da variável.                         |
| `variable`    |                                | A variável de ambiente a ser exibida.                                        |
| `default`     |                                | O valor padrão a ser exibido quando a variável selecionada não for definida. |
| `format`      | `"with [$env_value]($style) "` | O formato do módulo.                                                         |
| `description` | `"<env_var module>"`     | A descrição do módulo, isto será exibido quando executar `starship explain`. |
| `disabled`    | `false`                        | Desabilita o módulo `env_var`.                                               |

### Variáveis

| Variável  | Exemplo                                   | Descrição                               |
| --------- | ----------------------------------------- | --------------------------------------- |
| env_value | `Windows NT` (se a variável __ for `$OS`) | O valor de ambiente da opção `variable` |
| symbol    |                                           | Espelha o valor da opção `symbol`       |
| style\* | `black bold dimmed`                       | Espelha o valor da opção `style`        |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

O módulo `erlang` mostra a versão atualmente instalada do [Erlang/OTP](https://erlang.org/doc/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contém um arquivo `rebar.config`.
- O diretório atual contém um arquivo `erlang.mk`.

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `' '`                               | O símbolo usado antes de exibir a versão do erlang.                                 |
| `style`             | `'bold red'`                         | O estilo do módulo.                                                                 |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `disabled`          | `false`                              | Desabilita o módulo `erlang`.                                                       |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v22.1.3` | A versão do `erlang`              |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Fennel

The `fennel` module shows the currently installed version of [Fennel](https://fennel-lang.org). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a file with the `.fnl` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🧅 '`                               | The symbol used before displaying the version of fennel.                            |
| `style`             | `'bold green'`                       | O estilo do módulo.                                                                 |
| `detect_extensions` | `[fnl]`                              | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `disabled`          | `false`                              | Disables the `fennel` module.                                                       |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.2.1` | The version of `fennel`           |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```

## Fill

O módulo `fill` preenche qualquer espaço vazio da linha com um simbolo. Se múltiplos módulos `fill` estiverem presentes em uma linha, eles irão dividir o espaço entre eles. Isto é útil para alinhar outros módulos.

### Opções

| Opções     | Padrão         | Descrição                               |
| ---------- | -------------- | --------------------------------------- |
| `symbol`   | `'.'`          | O simbolo usado para preencher a linha. |
| `style`    | `'bold black'` | O estilo do módulo.                     |
| `disabled` | `false`        | Desabilita o módulo `fill`              |

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

## Fossil Branch

The `fossil_branch` module shows the name of the active branch of the check-out in your current directory.

### Opções

| Opções              | Padrão                           | Descrição                                                                                   |
| ------------------- | -------------------------------- | ------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | O formato do módulo. Use `'$branch'` to refer to the current branch name.                   |
| `symbol`            | `' '`                           | The symbol used before the branch name of the check-out in your current directory.          |
| `style`             | `'bold purple'`                  | O estilo do módulo.                                                                         |
| `truncation_length` | `2^63 - 1`                       | Truncates a Fossil branch name to `N` graphemes                                             |
| `truncation_symbol` | `'…'`                            | O simbolo usado para indicar que o nome braço foi truncado. You can use `''` for no symbol. |
| `disabled`          | `true`                           | Disables the `fossil_branch` module.                                                        |

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| branch    | `trunk` | The active Fossil branch          |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```

## Google Cloud (`gcloud`)

O módulo `gcloud` exibe a configuração atual para a CLI do [`gcloud`](https://cloud.google.com/sdk/gcloud). Isto é baseadp mp arquivo `~/.config/gcloud/active_config` e no arquivo`~/.config/gcloud/configurations/config_{CONFIG NAME}` e a env var `CLOUDSDK_CONFIG`.

### Opções

| Opções            | Padrão                                                     | Descrição                                                           |
| ----------------- | ---------------------------------------------------------- | ------------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | O formato do módulo.                                                |
| `symbol`          | `'☁️  '`                                                   | O simbolo usado antes de exibir o perfil atual do GCP.              |
| `region_aliases`  | `{}`                                                       | Tabela de aliases de região para exibir além do nome do GCP.        |
| `project_aliases` | `{}`                                                       | Tabela de apelidos do projeto a serem exibidos além do nome do GCP. |
| `style`           | `'bold blue'`                                              | O estilo do módulo.                                                 |
| `disabled`        | `false`                                                    | Desabilita o módulo `gcloud`.                                       |

### Variáveis

| Variável  | Exemplo       | Descrição                                                          |
| --------- | ------------- | ------------------------------------------------------------------ |
| region    | `us-central1` | A região atual do GCP                                              |
| account   | `foo`         | O perfil atual do GCP                                              |
| domain    | `example.com` | O perfil de domínio atual do GCP                                   |
| project   |               | O projeto atual do GCP                                             |
| active    | `default`     | O nome da configuração escrita em `~/.config/gcloud/active_config` |
| symbol    |               | Espelha o valor da opção `symbol`                                  |
| style\* |               | Espelha o valor da opção `style`                                   |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

O módulo `git_branch` exibe o branch ativo do repositório no diretório atual.

### Opções

| Opções               | Padrão                                            | Descrição                                                                                   |
| -------------------- | ------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | Exibe o nome do braço remoto, mesmo se ele for igual ao nome do braço local.                |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | O formato do módulo. Use `'$branch'` to refer to the current branch name.                   |
| `symbol`             | `' '`                                            | Um formato de string que representa o simbolo do git branch.                                |
| `style`              | `'bold purple'`                                   | O estilo do módulo.                                                                         |
| `truncation_length`  | `2^63 - 1`                                        | Truncates um braço do git para `N` caracteres.                                              |
| `truncation_symbol`  | `'…'`                                             | O simbolo usado para indicar que o nome braço foi truncado. You can use `''` for no symbol. |
| `only_attached`      | `false`                                           | Apenas exibe o nome do braço quando o estado não for detached `HEAD`.                       |
| `ignore_branches`    | `[]`                                              | Uma lista de nomes para evitar a exibição. Useful for 'master' or 'main'.                   |
| `disabled`           | `false`                                           | Desabilita o módulo `git_branch`.                                                           |

### Variáveis

| Variável      | Exemplo  | Descrição                                                                                         |
| ------------- | -------- | ------------------------------------------------------------------------------------------------- |
| branch        | `master` | O nome do braço atual, retornará para `HEAD` se não tiver braço atual (e.x: git detached `HEAD`). |
| remote_name   | `origin` | O nome do remoto.                                                                                 |
| remote_branch | `master` | O nome do braço rastreado no `remote_name`.                                                       |
| symbol        |          | Espelha o valor da opção `symbol`                                                                 |
| style\*     |          | Espelha o valor da opção `style`                                                                  |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

O módulo `git_commit` exibe o hash do comiit atual e também a tag (se existir) do repositório no diretório atual.

### Opções

| Opções               | Padrão                         | Descrição                                                                            |
| -------------------- | ------------------------------ | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                            | O tamanho do git commit hash para ser exibido.                                       |
| `format`             | `'[\($hash$tag\)]($style) '` | O formato do módulo.                                                                 |
| `style`              | `'bold green'`                 | O estilo do módulo.                                                                  |
| `only_detached`      | `true`                         | Apenas exibe o git commit hash quando o estado for detached `HEAD`                   |
| `tag_disabled`       | `true`                         | Desabilita a exibição da informação da tag no módulo `git_commit`.                   |
| `tag_max_candidates` | `0`                            | How many commits to consider for tag display. The default only allows exact matches. |
| `tag_symbol`         | `' 🏷 '`                        | Simbolo da tag prefixado na informação a ser exibida                                 |
| `disabled`           | `false`                        | Desabilita o módulo `git_commit`.                                                    |

### Variáveis

| Variável  | Exemplo   | Descrição                                    |
| --------- | --------- | -------------------------------------------- |
| hash      | `b703eb3` | A hash atual do git commit                   |
| tag       | `v1.0.0`  | The tag name if showing tag info is enabled. |
| style\* |           | Espelha o valor da opção `style`             |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## Git State

O módulo `git_state` vai exibir nos diretorios que fazem parte de um repositorio git e onde existe uma operação em progresso, como: _REBASING_, _BISECTING_, etc. Se houver informação de progresso (e.x: REBASING 3/10). esta informação será exibida também.

### Opções

| Opções         | Padrão                                                          | Descrição                                                                            |
| -------------- | --------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `rebase`       | `'REBASING'`                                                    | O formato de string exibida quando um `rebase` esta em progresso.                    |
| `merge`        | `'MERGING'`                                                     | O formato de string exibida quando um `merge` esta em progresso.                     |
| `revert`       | `'REVERTING'`                                                   | O formato de string exibida quando um `revert` esta em progresso.                    |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | O formato de string exibida quando um `cherry-pick` esta em progresso.               |
| `bisect`       | `'BISECTING'`                                                   | O formato de string exibida quando um `bisect` esta em progresso.                    |
| `am`           | `'AM'`                                                          | O formato de string exibida quando um `apply-mailbox` (`git am`) esta em progresso.  |
| `am_or_rebase` | `'AM/REBASE'`                                                   | O formato de string exibida quando um `apply-mailbox` or `rebase` esta em progresso. |
| `style`        | `'bold yellow'`                                                 | O estilo do módulo.                                                                  |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | O formato do módulo.                                                                 |
| `disabled`     | `false`                                                         | Desabilita o módulo `git_state`.                                                     |

### Variáveis

| Variável         | Exemplo    | Descrição                              |
| ---------------- | ---------- | -------------------------------------- |
| state            | `REBASING` | O estado atual do repo                 |
| progress_current | `1`        | O progresso da operação atual          |
| progress_total   | `2`        | O total do progresso da operação atual |
| style\*        |            | Espelha o valor da opção `style`       |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Git Metrics

O módulo `git_metrics` vai exibir o número de adições e exclusões de linhas no repositório git atual.

::: tip

Este módulo é desabilitado por padrão. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

:::

### Opções

| Opções               | Padrão                                                       | Descrição                                   |
| -------------------- | ------------------------------------------------------------ | ------------------------------------------- |
| `added_style`        | `'bold green'`                                               | O estilo para a contagem de adições.        |
| `deleted_style`      | `'bold red'`                                                 | O estilo para a contagem de exclusões.      |
| `only_nonzero_diffs` | `true`                                                       | Exibe apenas o status para itens alterados. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | O formato do módulo.                        |
| `disabled`           | `true`                                                       | Desabilita o módulo `git_metrics`.          |

### Variáveis

| Variável          | Exemplo | Descrição                               |
| ----------------- | ------- | --------------------------------------- |
| added             | `1`     | O número atual de linhas adicionadas    |
| deleted           | `2`     | O número atual de linhas excluidas      |
| added_style\*   |         | Espelha o valor da opção `added_style`  |
| deleted_style\* |         | Espelha o valor da opção`deleted_style` |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git Status

O módulo `git_status` exibe o simbolo que representa o estado do repositório no diretório atual.

::: tip

O módulo Git Status é muito lento nos diretórios do Windows (por exemplo, em `/mnt/c/`) quando em um ambiente WSL. Você pode desabilitar o módulo ou usar a opção `windows_starship` para usar um executável Starship nativo do Windows para calcular o `git_status` para esses caminhos.

:::

### Opções

| Opções              | Padrão                                          | Descrição                                                                                                                               |
| ------------------- | ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | O formato padrão para `git_status`                                                                                                      |
| `conflicted`        | `'='`                                           | Este braço tem conflitos.                                                                                                               |
| `ahead`             | `'⇡'`                                           | O formato do `ahead`                                                                                                                    |
| `behind`            | `'⇣'`                                           | O formato do `behind`                                                                                                                   |
| `diverged`          | `'⇕'`                                           | O formato do `diverged`                                                                                                                 |
| `up_to_date`        | `''`                                            | O formato do `up_to_date`                                                                                                               |
| `untracked`         | `'?'`                                           | O formato do `untracked`                                                                                                                |
| `stashed`           | `'$'`                                           | O formato do `stashed`                                                                                                                  |
| `modified`          | `'!'`                                           | O formato do `modified`                                                                                                                 |
| `staged`            | `'+'`                                           | O formato do `staged`                                                                                                                   |
| `renamed`           | `'»'`                                           | O formato do `renamed`                                                                                                                  |
| `deleted`           | `'✘'`                                           | O formato do `deleted`                                                                                                                  |
| `style`             | `'bold red'`                                    | O estilo do módulo.                                                                                                                     |
| `ignore_submodules` | `false`                                         | Ignora as alterações de submódulos.                                                                                                     |
| `disabled`          | `false`                                         | Desabilita o módulo `git_status`.                                                                                                       |
| `windows_starship`  |                                                 | Use este caminho (Linux) para um executável do Windows Starship renderizar o `git_status` quando estiver em caminhos do Windows no WSL. |

### Variáveis

As variáveis a seguir podem ser usadas no `format`:

| Variável       | Descrição                                                                                                  |
| -------------- | ---------------------------------------------------------------------------------------------------------- |
| `all_status`   | Atalhos para `$conflicted$stashed$deleted$renamed$modified$staged$untracked`                               |
| `ahead_behind` | Exibe `diverged`, `ahead`, `behind` or `up_to_date` conforme o formato da string do status do repositório. |
| `conflicted`   | Exibe `conflicted` quando este braço tenha conflitos no merge.                                             |
| `untracked`    | Exibe `untracked` quando há arquivos não rastreados no diretório atual.                                    |
| `stashed`      | Exibe `stashed` quando um stash existe para o repositório local.                                           |
| `modified`     | Exibe `modified` quando um arquivo tenha modificações for adicionado na área de staging.                   |
| `staged`       | Exibe `staged` quando um arquivo novo for adicionado na área de staging.                                   |
| `renamed`      | Exibe `renamed` quando um arquivo renomeado for adicionado na área de staging.                             |
| `deleted`      | Exibe `deleted` quando um arquivo deletado for adicionado na área de staging.                              |
| style\*      | Espelha o valor da opção `style`                                                                           |

*: Esta variável só pode ser usada como parte de uma string de estilo

As variáveis a seguir podem ser usadas em `diverged`:

| Variável       | Descrição                                           |
| -------------- | --------------------------------------------------- |
| `ahead_count`  | Número de commits a frente do braço de rastreamento |
| `behind_count` | Número de commits atrás do braço de rastreamento    |

As variaveis a seguir podem ser usadas em `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` e `deleted`:

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

## Go

O módulo `golang` exibe a versão instalada atual do [Go](https://golang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `go.mod`
- O diretório atual conter um arquivo `go.sum`
- O diretório atual conter um arquivo `go.work`
- O diretório atual conter um arquivo `glide.yaml`
- O diretório atual contem um arquivo `Gopkg.yml`
- O diretório atual contém um arquivo `Gopkg.lock`
- O diretório atual contem um arquivo `.go-version`
- O diretório atual contem um diretório `Godeps`
- O diretório atual contem arquivos com a extensão `.go`

### Opções

| Opções              | Padrão                                                                                    | Descrição                                                                           |
| ------------------- | ----------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                                                               | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐹 '`                                                                                    | O formato da string que representa o simbolo do Go.                                 |
| `detect_extensions` | `['go']`                                                                                  | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `['Godeps']`                                                                              | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold cyan'`                                                                             | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                                                   | Desabilita o módulo `golang`.                                                       |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v1.12.1` | A versão do `go`                  |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[golang]
format = 'via [🏎💨 $version](bold cyan) '
```

## Guix-shell

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment. The module will be shown when inside a guix-shell environment.

### Opções

| Opções     | Padrão                     | Descrição                                              |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `'via [$symbol]($style) '` | O formato do módulo.                                   |
| `symbol`   | `"🐃 "`                     | A format string representing the symbol of guix-shell. |
| `style`    | `"yellow bold"`            | O estilo do módulo.                                    |
| `disabled` | `false`                    | Disables the `guix_shell` module.                      |

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

The `gradle` module shows the version of the [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) currently used in the project directory.

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `gradle/wrapper/gradle-wrapper.properties` directory.
- The current directory contains a file ending with `.gradle` or `.gradle.kts`.

The `gradle` module is only able to read your Gradle Wrapper version from your config file, we don't execute your wrapper, because of the security concerns.

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                |
| `version_format`    | `"v${raw}"`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🅶 "`                               | A format string representing the symbol of Gradle.                                  |
| `detect_extensions` | `["gradle", "gradle.kts"]`           | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `["gradle"]`                         | Quais pastas devem ativar este módulo.                                              |
| `style`             | `"bold bright-cyan"`                 | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Disables the `gradle` module.                                                       |
| `recursive`         | `false`                              | Enables recursive finding for the `gradle` directory.                               |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v7.5.1` | The version of `gradle`           |
| symbol   |          | Espelha o valor da opção `symbol` |
| style*   |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

## Haskell

O módulo `haskell` encontra a versão atual do GHC selecionada e/ou o snapshot do Stack selecionado.

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `stack.yaml`
- O diretório atual contém qualquer arquivo `.hs`, `.cabal`, ou `.hs-boot`

### Opções

| Opções              | Padrão                               | Descrição                                                 |
| ------------------- | ------------------------------------ | --------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                      |
| `symbol`            | `'λ '`                               | Uma string de formato que representa o símbolo de Haskell |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | Quais extensões devem ativar este módulo.                 |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | Quais nomes de arquivos devem ativar este módulo.         |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                    |
| `style`             | `'bold purple'`                      | O estilo do módulo.                                       |
| `disabled`          | `false`                              | Desativa o módulo `haskell`.                              |

### Variáveis

| Variável       | Exemplo     | Descrição                                                                      |
| -------------- | ----------- | ------------------------------------------------------------------------------ |
| version        |             | `ghc_version` ou o `snapshot` dependendo se o projeto atual é um projeto Stack |
| snapshot       | `lts-18.12` | Snapshot do Stack selecionado                                                  |
| ghc\_version | `9.2.1`     | Versão do GHC instalada                                                        |
| symbol         |             | Espelha o valor da opção `symbol`                                              |
| style\*      |             | Espelha o valor da opção `style`                                               |

*: Esta variável só pode ser usada como parte de uma string de estilo

## Haxe

The `haxe` module shows the currently installed version of [Haxe](https://haxe.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` or `.haxerc` file
- The current directory contains a `.haxelib` or a `haxe_libraries` directory
- The current directory contains a file with the `.hx` or `.hxml` extension

### Opções

| Opções              | Padrão                                                                                          | Descrição                                                                           |
| ------------------- | ----------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                            | O formato do módulo.                                                                |
| `version_format`    | `"v${raw}"`                                                                                     | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["hx", "hxml"]`                                                                                | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `["project.xml", "Project.xml", "application.xml", "haxelib.json", "hxformat.json", ".haxerc"]` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[".haxelib", "haxe_libraries"]`                                                                | Quais pastas devem ativar este módulo.                                              |
| `symbol`            | `"⌘ "`                                                                                          | O formato de string que representa o simbolo do Helm.                               |
| `style`             | `"bold fg:202"`                                                                                 | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                                                         | Disables the `haxe` module.                                                         |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v4.2.5` | The version of `haxe`             |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

O módulo `helm` exibe a versão atual instalada do [Helm](https://helm.sh/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `helmfile.yaml`
- O diretório atual contem um arquivo `Chart.yaml`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `symbol`            | `'⎈ '`                               | O formato de string que representa o simbolo do Helm.                               |
| `style`             | `'bold white'`                       | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `helm`.                                                         |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v3.1.1` | A versão do `helm`                |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## Hostname

O módulo `hostname` exibe o nome do hostname.

### Opções

| Opções       | Padrão                                 | Descrição                                                                                                                                     |
| ------------ | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`   | `true`                                 | Apenas exibe o hostname quando conectado em uma sessão SSH.                                                                                   |
| `ssh_symbol` | `'🌐 '`                                 | Uma formatação de string que representa o símbolo quando conectado à sessão SSH.                                                              |
| `trim_at`    | `'.'`                                  | String na qual vai truncar o hostname, após a primeira correspondência. `'.'` will stop after the first dot. `''` will disable any truncation |
| `format`     | `'[$ssh_symbol$hostname]($style) in '` | O formato do módulo.                                                                                                                          |
| `style`      | `'bold dimmed green'`                  | O estilo do módulo.                                                                                                                           |
| `disabled`   | `false`                                | Desabilita o módulo `hostname`.                                                                                                               |

### Variáveis

| Variável   | Exemplo    | Descrição                                                      |
| ---------- | ---------- | -------------------------------------------------------------- |
| hostname   | `computer` | O hostname do computador                                       |
| style\*  |            | Espelha o valor da opção `style`                               |
| ssh_symbol | `'🌏 '`     | O símbolo a ser representado quando conectado à uma sessão SSH |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

## Java

O módulo `java` exibe o versão atual instalada do [Java](https://www.oracle.com/java/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, or `build.boot` file
- O diretório atual contenha arquivos com as extensões `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc`

### Opções

| Opções              | Padrão                                                                                                   | Descrição                                                                           |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                 | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                                                                              | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                     | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot']` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                                                                                     | Quais pastas devem ativar este módulo.                                              |
| `symbol`            | `'☕ '`                                                                                                   | Um formato de string que representa o simbolo do Java                               |
| `style`             | `'red dimmed'`                                                                                           | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                                                                  | Desabilita o módulo `java`.                                                         |

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| version   | `v14`   | A versão do `java`                |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## Jobs

O módulo `jobs` exibe o número atual de jobs rodando. O módulo vai ser exibido apenas se existir jobs em segundo plano sendo executados. O módulo vai exibir o número de jobs rodando se ao menos tiver 2 jobs ou mais que o valor da configuração `number_threshold`, se existir. O módulo vai exibir um simbolo se tiver ao menos 1 job ou mais, se o valor da configuração `symbol_threshold` existir. Você pode setar os dois valores para 0 para _sempre_ exibir o simbolo e número de jobs, mesmo que seja 0 os jobs em execução.

A funcionalidade padrão é:

- 0 jobs -> Nada é exibido.
- 1 job -> `symbol` é exibido.
- 2 jobs or more -> `symbol` + `number` é exibido.

::: atenção

Este módulo não é suportado em tcsh e nu.

:::

::: atenção

A opção `threshold` está obsoleta, mas se você quiser usa-la, o módulo vai exibir o numero de jobs rodando se for maior que 1 ou maior que o valor configurado na `threshold`, se ele existir. Se o valor `threshold` for definido como 0, então o módulo vai exibir quando tiver 0 jobs rodando.

:::

### Opções

| Opções             | Padrão                        | Descrição                                                                 |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------- |
| `threshold`*       | `1`                           | Exibe o número de jobs se excedido.                                       |
| `symbol_threshold` | `1`                           | Exibe `symbol` se o número de jobs for ao menos `symbol_threshold`.       |
| `number_threshold` | `2`                           | Exibe o número de jobs se o número de jobs é ao menos `number_threshold`. |
| `format`           | `'[$symbol$number]($style) '` | O formato do módulo.                                                      |
| `symbol`           | `'✦'`                         | A string usada para representar a variável `symbol`.                      |
| `style`            | `'bold blue'`                 | O estilo do módulo.                                                       |
| `disabled`         | `false`                       | Desabilita o módulo `jobs`.                                               |

*: Esta opção está obsoleta, por favor use o `number_threshold` e `symbol_threshold` em vez disso.

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| number    | `1`     | O número de jobs                  |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
symbol_threshold = 0
```

## Julia

O módulo `julia` exibe a versão atual instalada do [Julia](https://julialang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `Project.toml`
- O diretório atual contem um arquivo `Manifest.toml`
- O diretório atual contem arquivos com a extensão `.jl`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['jl']`                             | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `symbol`            | `'ஃ '`                               | O formato de string que representa o simbolo do Julia.                              |
| `style`             | `'bold purple'`                      | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `julia`.                                                        |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.4.0` | A versão do `julia`               |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

O módulo `kotlin` exibie a versão atual instalada do [Kotlin](https://kotlinlang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `.kt` ou um arquivo `.kts`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['kt', 'kts']`                      | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `symbol`            | `'🅺 '`                               | O formato de string que representa o simbolo do Kotlin.                             |
| `style`             | `'bold blue'`                        | O estilo do módulo.                                                                 |
| `kotlin_binary`     | `'kotlin'`                           | Configura o binário do kotlin que o Starship executa para obter a versão.           |
| `disabled`          | `false`                              | Desabilita o módulo `kotlin`.                                                       |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v1.4.21` | A versão do `kotlin`              |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

Exibe o nome atual do [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) e, se definido, o namespace, usuário e cluster do arquivo kubeconfig. O namespace precisa ser definido no arquivo kubeconfig, isso pode ser feito via `kubectl config set-context starship-context --namespace astronaut`. Da mesma forma, o usuário e o cluster podem ser definidos com `kubectl config set-context starship-context --user starship-user` e `kubectl config set-context starship-context --cluster starship-cluster`. Se a env var `$KUBECONFIG` estiver definida o módulo vai usa-la ao invés de usar o `~/.kube/config`.

::: tip

Este módulo é desabilitado por padrão. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

When the module is enabled it will always be active, unless any of `detect_extensions`, `detect_files` or `detect_folders` have been set in which case the module will only be active in directories that match those conditions.

:::

### Opções

| Opções              | Padrão                                               | Descrição                                                     |
| ------------------- | ---------------------------------------------------- | ------------------------------------------------------------- |
| `symbol`            | `'☸ '`                                               | Uma string que representa o simbolo exibido antes do Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | O formato do módulo.                                          |
| `style`             | `'cyan bold'`                                        | O estilo do módulo.                                           |
| `context_aliases`   | `{}`                                                 | Tabela de aliases de contexto para exibir.                    |
| `user_aliases`      | `{}`                                                 | Table of user aliases to display.                             |
| `detect_extensions` | `[]`                                                 | Quais extensões devem ativar este módulo.                     |
| `detect_files`      | `[]`                                                 | Quais nomes de arquivos devem ativar este módulo.             |
| `detect_folders`    | `[]`                                                 | Quais pastas devem ativar este módulo.                        |
| `disabled`          | `true`                                               | Desabilita o módulo `kubernetes`.                             |

### Variáveis

| Variável  | Exemplo              | Descrição                                   |
| --------- | -------------------- | ------------------------------------------- |
| context   | `starship-context`   | O nome atual do kubernetes context          |
| namespace | `starship-namespace` | Se definido o namespace atual do kubernetes |
| user      | `starship-user`      | Se definido, o usuário atual do kubernetes  |
| cluster   | `starship-cluster`   | Se definido, o cluster atual do kubernetes  |
| symbol    |                      | Espelha o valor da opção `symbol`           |
| style\* |                      | Espelha o valor da opção `style`            |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
'dev.local.cluster.k8s' = 'dev'
'.*/openshift-cluster/.*' = 'openshift'
'gke_.*_(?P<var_cluster>[\\w-]+)' = 'gke-$var_cluster'
[kubernetes.user_aliases]
'dev.local.cluster.k8s' = 'dev'
'root/.*' = 'root'
```

Only show the module in directories that contain a `k8s` file.

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Correspondência Regex

Additional to simple aliasing, `context_aliases` and `user_aliases` also supports extended matching and renaming using regular expressions.

A expressão regular deve coincidir com todo o contexto kube, Grupos de captura podem ser referenciados usando `$name` e `$N` na substituição. Isto esta mais explicado na documentação do [regex crate](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace).

Nomes longos de clusters gerados automaticamente podem ser encurtados usando expressão regular:

```toml
[kubernetes.context_aliases]
# OpenShift contexts carry the namespace and user in the kube context: `namespace/name/user`:
'.*/openshift-cluster/.*' = 'openshift'
# Or better, to rename every OpenShift cluster at once:
'.*/(?P<var_cluster>[\\w-]+)/.*' = '$var_cluster'

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
'gke_.*_(?P<var_cluster>[\\w-]+)' = 'gke-$var_cluster'
```

## Quebra de Linha

O módulo `line_break` separa o prompt em duas linhas.

### Opções

| Opções     | Padrão  | Descrição                                                                           |
| ---------- | ------- | ----------------------------------------------------------------------------------- |
| `disabled` | `false` | Desabilita o módulo `line_break`, fazendo com que o prompt seja em uma única linha. |

### Exemplo

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## IP Local

O módulo `localip` mostra o endereço IPv4 da interface primária de rede.

### Opções

| Opções     | Padrão                    | Descrição                                                      |
| ---------- | ------------------------- | -------------------------------------------------------------- |
| `ssh_only` | `true`                    | Apenas mostre o endereço IP quando conectado a uma sessão SSH. |
| `format`   | `'[$localipv4]($style) '` | O formato do módulo.                                           |
| `style`    | `'bold yellow'`           | O estilo do módulo.                                            |
| `disabled` | `true`                    | Desabilita o módulo `localip`.                                 |

### Variáveis

| Variável  | Exemplo      | Descrição                        |
| --------- | ------------ | -------------------------------- |
| localipv4 | 192.168.1.13 | Contém o endereço IPv4 principal |
| style\* |              | Espelha o valor da opção `style` |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

O módulo `lua` exibe a versão atual instalada do [Lua](http://www.lua.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contém um arquivo `.lua-version`
- O diretório atual contém um diretório `lua`
- O diretório atual tem um arquivo com a extensão `.lua`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🌙 '`                               | Uma string que representa o simbolo do Lua.                                         |
| `detect_extensions` | `['lua']`                            | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['.lua-version']`                   | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `['lua']`                            | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold blue'`                        | O estilo do módulo.                                                                 |
| `lua_binary`        | `'lua'`                              | Configura o binário lua que o Starship executa para pegar a versão.                 |
| `disabled`          | `false`                              | Desabilita o módulo `lua`.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v5.4.0` | A versão do `lua`                 |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[lua]
format = 'via [🌕 $version](bold blue) '
```

## Uso de Memória

O módulo `memory_usage` mostra a memória atual do sistema e o uso de troca.

Por padrão o uso do swap é exibido se o total de swap do sistema é diferente de zero.

::: tip

Este módulo é desabilitado por padrão. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

:::

### Opções

| Opções      | Padrão                                          | Descrição                                                     |
| ----------- | ----------------------------------------------- | ------------------------------------------------------------- |
| `threshold` | `75`                                            | Esconde o uso de memoria a menos que exceda esta porcentagem. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | O formato do módulo.                                          |
| `symbol`    | `'🐏'`                                           | O simbolo usado antes de exibir o uso de memoria.             |
| `style`     | `'bold dimmed white'`                           | O estilo do módulo.                                           |
| `disabled`  | `true`                                          | Desabilita o módulo `memory_usage`.                           |

### Variáveis

| Variável         | Exemplo       | Descrição                                         |
| ---------------- | ------------- | ------------------------------------------------- |
| ram              | `31GiB/65GiB` | O uso/total de memoria RAM atual do sistema.      |
| ram_pct          | `48%`         | A porcentagem de uso atual da memoria do sistema. |
| swap\*\*     | `1GiB/4GiB`   | O tamanho atual do swap do sistema.               |
| swap_pct\*\* | `77%`         | A porcentagem atual de uso do swap.               |
| symbol           | `🐏`           | Espelha o valor da opção `symbol`                 |
| style\*        |               | Espelha o valor da opção `style`                  |

*: Esta variável só pode ser usada como parte de uma string de estilo *\*: As informações do arquivo SWAP são exibidas apenas se detectadas no sistema atual

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

| Opções              | Padrão                             | Descrição                                                                                 |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | Truncates a project name to `N` graphemes.                                                |
| `truncation_symbol` | `'…'`                              | The symbol used to indicate a project name was truncated. You can use `''` for no symbol. |
| `format`            | `'via [$symbol$project]($style) '` | O formato do módulo.                                                                      |
| `symbol`            | `'⬢ '`                             | The symbol used before displaying the project name.                                       |
| `style`             | `'blue bold'`                      | O estilo do módulo.                                                                       |
| `disabled`          | `false`                            | Disables the `meson` module.                                                              |

### Variáveis

| Variável  | Exemplo    | Descrição                         |
| --------- | ---------- | --------------------------------- |
| project   | `starship` | The current Meson project name    |
| symbol    | `🐏`        | Espelha o valor da opção `symbol` |
| style\* |            | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

### Opções

| Opções              | Padrão                                    | Descrição                                                                               |
| ------------------- | ----------------------------------------- | --------------------------------------------------------------------------------------- |
| `symbol`            | `' '`                                    | O simbolo usado ante do marcador hg ou nome do braço do repositório no diretório atual. |
| `style`             | `'bold purple'`                           | O estilo do módulo.                                                                     |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | O formato do módulo.                                                                    |
| `truncation_length` | `2^63 - 1`                                | Truncates the hg branch / topic name to `N` graphemes                                   |
| `truncation_symbol` | `'…'`                                     | O simbolo usado para indicar que o nome braço foi truncado.                             |
| `disabled`          | `true`                                    | Desabilita o módulo `hg_branch`.                                                        |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| branch    | `master`  | O braço mercurial ativo           |
| topic     | `feature` | The active mercurial topic        |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Nim

O módulo `nim` exibe a versão atual instalada do [Nim](https://nim-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `nim.cfg`
- O diretório atual tenha um arquivo com a extensão `.nim`
- O diretório atual tenha um arquivo com a extensão `.nims`
- O diretório atual tenha um arquivo com a extensão `.nimble`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo                                                                 |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'👑 '`                               | O símbolo usado antes de exibir a versão do Nim.                                    |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['nim.cfg']`                        | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold yellow'`                      | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `nim`.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.2.0` | A versão do `nimc`                |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

O módulo `nix_shell` exibe o ambiente [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html). O módulo vai exibir quando estiver dentro de um ambiente nix-shell.

### Opções

| Opções        | Padrão                                         | Descrição                                                             |
| ------------- | ---------------------------------------------- | --------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \($name\))]($style) '` | O formato do módulo.                                                  |
| `symbol`      | `'❄️ '`                                        | Uma string que representa o simbolo do nix-shell.                     |
| `style`       | `'bold blue'`                                  | O estilo do módulo.                                                   |
| `impure_msg`  | `'impure'`                                     | Uma string que exibe quando o shell é impuro.                         |
| `pure_msg`    | `'pure'`                                       | Uma string que exibe quando o shell é puro.                           |
| `unknown_msg` | `''`                                           | A format string shown when it is unknown if the shell is pure/impure. |
| `disabled`    | `false`                                        | Desabilita o módulo `nix_shell`.                                      |
| `heuristic`   | `false`                                        | Attempts to detect new `nix shell`-style shells with a heuristic.     |

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| state     | `pure`  | O estado do nix-shell             |
| name      | `lorri` | O nome do nix-shell               |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

O módulo `nodejs` exibe a versão atual instalada do [Node.js](https://nodejs.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `package.json`
- O diretório atual tenha um arquivo `.node-version`
- O diretório atual tenha um arquivo`.nvmrc`
- O diretório atual tenha um diretório `node_modules`
- O diretório atual tenha um arquivo com a extensão `.js`, `.mjs` or `.cjs`
- O diretório atual contém um arquivo com a extensão `.ts`, `.mts` ou `.cts`

### Opções

| Opções              | Padrão                                     | Descrição                                                                                                |
| ------------------- | ------------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`       | O formato do módulo.                                                                                     |
| `version_format`    | `'v${raw}'`                                | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch`                      |
| `symbol`            | `' '`                                     | Uma string que representa o simbolo do Node.js.                                                          |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']` | Quais extensões devem ativar este módulo.                                                                |
| `detect_files`      | `['package.json', '.node-version']`        | Quais nomes de arquivos devem ativar este módulo.                                                        |
| `detect_folders`    | `['node_modules']`                         | Quais pastas devem ativar este módulo.                                                                   |
| `style`             | `'bold green'`                             | O estilo do módulo.                                                                                      |
| `disabled`          | `false`                                    | Desabilita o módulo `nodejs`.                                                                            |
| `not_capable_style` | `bold red`                                 | O estilo para o módulo quando a propriedade engine no package.json não coincide com a versão do Node.js. |

### Variáveis

| Variável  | Exemplo    | Descrição                         |
| --------- | ---------- | --------------------------------- |
| version   | `v13.12.0` | A versão do `node`                |
| symbol    |            | Espelha o valor da opção `symbol` |
| style\* |            | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

O módulo `ocaml` exibe a versão atual instalada do [OCaml](https://ocaml.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contenha um arquivo com a extensão`.opam` ou um diretório `_opam`
- O diretório atual tenha um diretório `esy.lock`
- O diretório atual tenha um arquivo `dune` or `dune-project`
- O diretório atual tenha um arquivo `jbuild` or `jbuild-ignore`
- O diretório tenha um arquivo `.merlin`
- O diretório atual tenha um arquivo com a extensão `.ml`, `.mli`, `.re` ou `.rei`

### Opções

| Opções                    | Padrão                                                                     | Descrição                                                                           |
| ------------------------- | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`                  | `'via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)'` | A string de formato do módulo.                                                      |
| `version_format`          | `'v${raw}'`                                                                | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `'🐫 '`                                                                     | O símbolo usado antes de exibir a versão do OCaml.                                  |
| `global_switch_indicator` | `''`                                                                       | A string usada para representar a mudança global OPAM.                              |
| `local_switch_indicator`  | `'*'`                                                                      | A string usada para representar as mudanças locais do OPAM.                         |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                       | Quais extensões devem ativar este módulo.                                           |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                    | Quais pastas devem ativar este módulo.                                              |
| `style`                   | `'bold yellow'`                                                            | O estilo do módulo.                                                                 |
| `disabled`                | `false`                                                                    | Desabilita o módulo `ocaml`.                                                        |

### Variáveis

| Variável         | Exemplo      | Descrição                                                        |
| ---------------- | ------------ | ---------------------------------------------------------------- |
| version          | `v4.10.0`    | A versão do `ocaml`                                              |
| switch_name      | `my-project` | O switch OPAM ativo                                              |
| switch_indicator |              | Espelha o valor do `indicator` para o switch ativo atual do OPAM |
| symbol           |              | Espelha o valor da opção `symbol`                                |
| style\*        |              | Espelha o valor da opção `style`                                 |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Open Policy Agent

The `opa` module shows the currently installed version of the OPA tool. By default the module will be shown if the current directory contains a `.rego` file.

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🪖  '`                              | A format string representing the symbol of OPA.                                     |
| `detect_extensions` | `['rego']`                           | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold blue'`                        | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Disables the `opa` module.                                                          |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v0.44.0` | The version of `opa`              |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

O módulo `openstack` exibe o OpenStack cloud e projeto atual. O módulo apenas ativa quando a env var `OS_CLOUD` esta definida, neste caso ele vai ler o arquivo `clouds.yaml` de qualquer um dos [default locations](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files). para buscar o projeto atual em uso.

### Opções

| Opções     | Padrão                                          | Descrição                                            |
| ---------- | ----------------------------------------------- | ---------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\($project\))]($style) '` | O formato do módulo.                                 |
| `symbol`   | `'☁️ '`                                         | O simbolo usado para exibir o OpenStack cloud atual. |
| `style`    | `'bold yellow'`                                 | O estilo do módulo.                                  |
| `disabled` | `false`                                         | Desabilita o módulo `openstack`.                     |

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| cloud     | `corp`  | O OpenStack cloud atual           |
| project   | `dev`   | O projeto OpenStack atual         |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```

## OS

The `os` module shows the current operating system. OS information is detected via the [os_info](https://lib.rs/crates/os_info) crate.

::: atenção

The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

:::

::: tip

Este módulo é desabilitado por padrão. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

:::

### Opções

| Opções     | Padrão                | Descrição                                              |
| ---------- | --------------------- | ------------------------------------------------------ |
| `format`   | `"[$symbol]($style)"` | O formato do módulo.                                   |
| `style`    | `"bold white"`        | O estilo do módulo.                                    |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type. Operating system types not defined by your configuration use the default symbols table below. All operating systems currently supported by the module are listed below. If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

```toml
# This is the default symbols table.
[os.symbols]
Alpine = "🏔️ "
Amazon = "🙂 "
Android = "🤖 "
Arch = "🎗️ "
CentOS = "💠 "
Debian = "🌀 "
DragonFly = "🐉 "
Emscripten = "🔗 "
EndeavourOS = "🚀 "
Fedora = "🎩 "
FreeBSD = "😈 "
Garuda = "🦅 "
Gentoo = "🗜️ "
HardenedBSD = "🛡️ "
Illumos = "🐦 "
Linux = "🐧 "
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
OpenBSD = "🐡 "
openSUSE = "🦎 "
OracleLinux = "🦴 "
Pop = "🍭 "
Raspbian = "🍓 "
Redhat = "🎩 "
RedHatEnterprise = "🎩 "
Redox = "🧪 "
Solus = "⛵ "
SUSE = "🦎 "
Ubuntu = "🎯 "
Unknown = "❓ "
Windows = "🪟 "
```

### Variáveis

| Variável  | Exemplo      | Descrição                                                          |
| --------- | ------------ | ------------------------------------------------------------------ |
| symbol    | `🎗️`         | The current operating system symbol from advanced option `symbols` |
| name      | `Arch Linux` | The current operating system name                                  |
| type      | `Arch`       | The current operating system type                                  |
| codename  |              | The current operating system codename, if applicable               |
| edition   |              | The current operating system edition, if applicable                |
| version   |              | The current operating system version, if applicable                |
| style\* |              | Espelha o valor da opção `style`                                   |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

O módulo `package` é mostrado quando o diretório atual é o repositório de um pacote e mostra sua versão atual. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – O versionamento de pacotes `npm` é extraído do `package.json` presente no diretório atual
- [**Cargo**](https://doc.rust-lang.org/cargo/) – O versionamento de pacotes `cargo`é extraído do arquivo `Cargo.toml` presente no diretório atual
- [**Nimble**](https://github.com/nim-lang/nimble) - O versionamento de pacotes `nimble` é extraído do arquivo `*.nimble` presente no diretório atual com o comando`nimble dump`
- [**Poetry**](https://python-poetry.org/) – O versionamento de pacotes `poetry` é extraído do arquivo `pyproject.toml` presente no diretório atual
- [**Python**](https://www.python.org) - O versionamento de pacotes `python` é extraída de um `pyproject.toml` compatível com [PEP 621](https://peps.python.org/pep-0621/) ou um `setup.cfg` presente no diretório atual
- [**Composer**](https://getcomposer.org/) – O versionamento de pacotes `composer` é extraído do arquivo`composer.json` presente no diretório atual
- [**Gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present in the current directory
- [**Julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present in the current directory
- [**Mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present in the current directory
- [**Helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present in the current directory
- [**Maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present in the current directory
- [**Meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present in the current directory
- [**Shards**](https://crystal-lang.org/reference/the_shards_command/index.html) - The `shards` package version is extracted from the `shard.yml` present in the current directory
- [**V**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present in the current directory
- [**SBT**](https://scala-sbt.org) - O versionamento de pacotes `sbt` pé extraído do arquivo `build.sbt` presente no diretório atual
- [**Daml**](https://www.digitalasset.com/developers) - The `daml` package version is extracted from the `daml.yaml` present in the current directory
- [**Dart**](https://pub.dev/) - O versionamento de pacotes `dart` pé extraído do arquivo `pubspec.yaml` presente no diretório atual

> ⚠️ A versão exibida é a que esta presente no código fonte do diretório atual e não do gerenciador de pacotes.

### Opções

| Opções            | Padrão                            | Descrição                                                                           |
| ----------------- | --------------------------------- | ----------------------------------------------------------------------------------- |
| `format`          | `'is [$symbol$version]($style) '` | O formato do módulo.                                                                |
| `symbol`          | `'📦 '`                            | O símbolo usado antes de exibir a versão do pacote.                                 |
| `version_format`  | `'v${raw}'`                       | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`           | `'bold 208'`                      | O estilo do módulo.                                                                 |
| `display_private` | `false`                           | Habilita a exibição da versão para os pacotes marcados como privado.                |
| `disabled`        | `false`                           | Desabilita o módulo `package`.                                                      |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.0.0` | A versão do seu pacote            |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[package]
format = 'via [🎁 $version](208 bold) '
```

## Perl

O módulo `perl` exibe a versão atual instalada do [Perl](https://www.perl.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tanha um aquivo `Makefile.PL` ou `Build.PL`
- O diretório atual tenha um arquivo `cpanfile` ou `cpanfile.snapshot`
- O diretório atual tenha um arquivo `META.json` ou `META.yml`
- O diretório atual tenha um arquivo `.perl-version`
- O diretório atual tenha um `.pl`, `.pm` ou `.pod`

### Opções

| Opções              | Padrão                                                                                                   | Descrição                                                                           |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                                     | A string de formato do módulo.                                                      |
| `version_format`    | `'v${raw}'`                                                                                              | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐪 '`                                                                                                   | O símbolo usado antes de exibir a versão do Perl.                                   |
| `detect_extensions` | `['pl', 'pm', 'pod']`                                                                                    | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['Makefile.PL', 'Build.PL', 'cpanfile', 'cpanfile.snapshot', 'META.json', 'META.yml', '.perl-version']` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                                                                                     | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold 149'`                                                                                             | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                                                                  | Desabilita o módulo `perl`.                                                         |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v5.26.1` | A versão do `perl`                |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml

[perl]
format = 'via [🦪 $version]($style) '
```

## PHP

O módulo `php` mostra a versão atualmente instalada do [PHP](https://www.php.net/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `composer.json`
- O diretório atual tenha um arquivo `.php-version`
- O diretório atual tenha um arquivo com extensão `.php`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐘 '`                               | The symbol used before displaying the version of PHP.                               |
| `detect_extensions` | `['php']`                            | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['composer.json', '.php-version']`  | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'147 bold'`                         | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `php`.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v7.3.8` | A versão do `php`                 |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[php]
format = 'via [🔹 $version](147 bold) '
```

## Pijul Channel

The `pijul_channel` module shows the active channel of the repo in your current directory.

### Opções

| Opções              | Padrão                            | Descrição                                                                            |
| ------------------- | --------------------------------- | ------------------------------------------------------------------------------------ |
| `symbol`            | `' '`                            | The symbol used before the pijul channel name of the repo in your current directory. |
| `style`             | `'bold purple'`                   | O estilo do módulo.                                                                  |
| `format`            | `'on [$symbol$channel]($style) '` | O formato do módulo.                                                                 |
| `truncation_length` | `2^63 - 1`                        | Truncates the pijul channel name to `N` graphemes                                    |
| `truncation_symbol` | `'…'`                             | O simbolo usado para indicar que o nome braço foi truncado.                          |
| `disabled`          | `true`                            | Disables the `pijul` module.                                                         |

## Pulumi

O módulo `pulumi` mostra o nome de usuário atual, a [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/) selcionada e a versão.

::: tip

Por padrão a versão do Pulumi não é exibida, pois leva um tempo maior para ser carregada. Em média a maioria dos plugins leva (~70ms). Se você ainda deseja ativa-la, [siga o exemplo abaixo](#with-pulumi-version).

:::

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tenha um arquivo `Pulumi.yaml` ou `Pulumi.yml`
- A parent directory contains either `Pulumi.yaml` or `Pulumi.yml` unless `search_upwards` is set to `false`

### Opções

| Opções           | Padrão                                       | Descrição                                                                           |
| ---------------- | -------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`         | `'via [$symbol($username@)$stack]($style) '` | A string de formato do módulo.                                                      |
| `version_format` | `'v${raw}'`                                  | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`         | `' '`                                       | Uma string que é exibida antes do Pulumi stack.                                     |
| `style`          | `'bold 5'`                                   | O estilo do módulo.                                                                 |
| `search_upwards` | `true`                                       | Enable discovery of pulumi config files in parent directories.                      |
| `disabled`       | `false`                                      | Desabilita o módulo `pulumi`.                                                       |

### Variáveis

| Variável        | Exemplo    | Descrição                         |
| --------------- | ---------- | --------------------------------- |
| version         | `v0.12.24` | A versão do `pulumi`              |
| stack           | `dev`      | A stack Pulumi atual              |
| nome do usuário | `alice`    | O nome de usuário Pulumi atual    |
| symbol          |            | Espelha o valor da opção `symbol` |
| style\*       |            | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

O módulo `purescript` exibe a versão atual instalada do [PureScript](https://www.purescript.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `spago.dhall`
- O diretório atual tenha um arquivo com a extensão `.purs`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'<=> '`                       | O símbolo usado antes de exibir a versão do PureScript.                             |
| `detect_extensions` | `['purs']`                           | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['spago.dhall']`                    | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold white'`                       | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `purescript`.                                                   |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `0.13.5` | A versão do `purescript`          |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[purescript]
format = 'via [$symbol$version](bold white)'
```

## Python

O módulo `python` exibe a versão atual instalada do [Python](https://www.python.org/) e o [Python virtual environment](https://docs.python.org/tutorial/venv.html) se algum estiver ativo.

Se o `pyenv_version_name` estiver definido como `true`, será exibido o nome da versão do pyenv. Caso contrario, ele exibirá o número da versão do `python --version`.

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tenha um arquivo `.python-version`
- O diretório atual tenha um arquivo `Pipfile`
- O diretório atual tenha um arquivo `__init__.py`
- O diretório atual conter um arquivo `pyproject.toml`
- O diretório atual conter um arquivo `requirements.txt`
- O diretório atual conter um arquivo `setup.py`
- O diretório atual conter um arquivo `tox.ini`
- O diretório atual tenha um arquivo com a extensão `.py`.
- Um ambiente virtual está atualmente ativo

### Opções

| Opções               | Padrão                                                                                                       | Descrição                                                                           |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | O formato do módulo.                                                                |
| `version_format`     | `'v${raw}'`                                                                                                  | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`             | `'🐍 '`                                                                                                       | Uma string que representa o simbolo do Python                                       |
| `style`              | `'yellow bold'`                                                                                              | O estilo do módulo.                                                                 |
| `pyenv_version_name` | `false`                                                                                                      | Usa pyenv para pegar a versão do Python                                             |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefixo antes da versão do pyenv, apenas usado se pyenv for usado                   |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configura o binário python que o Starship vai executar para obter a versão.         |
| `detect_extensions`  | `['py']`                                                                                                     | Quais extensões devem acionar este módulo                                           |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | []                                                                                  |
| `detect_folders`     | `[]`                                                                                                         | Quais pastas devem ativar este módulo                                               |
| `disabled`           | `false`                                                                                                      | Desabilita o módulo `python`.                                                       |

::: tip

A variável `python_binary` aceita uma string ou uma lista de strings. O Starship vai tentar executar cada binário até obter um resultado. Note que você pode apenas alterar o binário em que o Starship executa para pegar a versão do Python não os argumentos que são utilizados.

O valor padrão e a ordem para o `python_binary` foram escolhidos para identificar primeiro uma versão do Python em um ambiente virtualenv/conda (que atualmente adiciona um `python`, não importa se ele aponta pro `python3` ou `python2`). Este tem efeito de lado, se você tenha um sistema com Python 2 instalado, pode ser usado antes de qualquer Python 3 (pelo menos nas Distros Linux que sempre fazem um symlink `/usr/bin/python` para Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `'python3'` will hide any Python version 2, see example below.

:::

### Variáveis

| Variável     | Exemplo         | Descrição                               |
| ------------ | --------------- | --------------------------------------- |
| version      | `'v3.8.1'`      | A versão do `python`                    |
| symbol       | `'🐍 '`          | Espelha o valor da opção `symbol`       |
| style        | `'yellow bold'` | Espelha o valor da opção `style`        |
| pyenv_prefix | `'pyenv '`      | Espelha o valor da opção `pyenv_prefix` |
| virtualenv   | `'venv'`        | O nome atual do `virtualenv`            |

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
# Use apenas o binário `python3` para obter a versão.
python_binary = 'python3'
```

```toml
# ~/.config/starship.toml

[python]
# Não acione para arquivos com a extensão py
detect_extensions = []
```

```toml
# ~/.config/starship.toml

[python]
# Exibe a versão do python de dentro de um venv local.
#
# Note que isso só funcionará quando o venv estiver dentro do projeto e só
# funcionará apenas no diretório que contém o diretório venv, mas talvez isso seja ok?
python_binary = ['./venv/bin/python', 'python', 'python3', 'python2']
```

## R

O módulo `rlang` mostra a versão atualmente instalada do [R](https://www.r-project.org/). O módulo será mostrado se qualquer uma das seguintes condições for atendida:

- O diretório atual tenha um arquivo com a extensão `.R`.
- O diretório atual tenha um arquivo com a extensão `.Rd`.
- O diretório atual tenha um arquivo com a extensão `.Rmd`.
- O diretório atual tenha um arquivo com a extensão `.Rproj`.
- O diretório atual tenha um arquivo com a extensão `.Rsx`.
- O diretório atual tenha um arquivo `.Rprofile`
- O diretório atual tenha uma pasta `.Rpoj.user`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'📐'`                                | Uma string que representa o simbolo do R.                                           |
| `style`             | `'blue bold'`                        | O estilo do módulo.                                                                 |
| `detect_extensions` | `['R', 'Rd', 'Rmd', 'Rproj', 'Rsx']` | Quais extensões devem acionar este módulo                                           |
| `detect_files`      | `['.Rprofile']`                      | []                                                                                  |
| `detect_folders`    | `['.Rproj.user']`                    | Quais pastas devem ativar este módulo                                               |
| `disabled`          | `false`                              | Desabilita o módulo `r`.                                                            |

### Variáveis

| Variável | Exemplo       | Descrição                         |
| -------- | ------------- | --------------------------------- |
| version  | `v4.0.5`      | A versão do `R`                   |
| symbol   |               | Espelha o valor da opção `symbol` |
| style    | `'blue bold'` | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml

[rlang]
format = 'with [📐 $version](blue bold) '
```

## Raku

The `raku` module shows the currently installed version of [Raku](https://www.raku.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `META6.json` file
- The current directory contains a `.p6`, `.pm6`, `.raku`, `.rakumod` or `.pod6`

### Opções

| Opções              | Padrão                                           | Descrição                                                                           |
| ------------------- | ------------------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version-$vm_version )]($style)'` | A string de formato do módulo.                                                      |
| `version_format`    | `'v${raw}'`                                      | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦋 '`                                           | The symbol used before displaying the version of Raku                               |
| `detect_extensions` | `['p6', 'pm6', 'pod6', 'raku', 'rakumod']`       | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['META6.json']`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                             | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold 149'`                                     | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                          | Disables the `raku` module.                                                         |

### Variáveis

| Variável   | Exemplo | Descrição                            |
| ---------- | ------- | ------------------------------------ |
| version    | `v6.d`  | The version of `raku`                |
| vm_version | `moar`  | The version of VM `raku` is built on |
| symbol     |         | Espelha o valor da opção `symbol`    |
| style\*  |         | Espelha o valor da opção `style`     |

### Exemplo

```toml
# ~/.config/starship.toml

[raku]
format = 'via [🦪 $version]($style) '
```

## Red

Por padrão o módulo `red` exibe a versão atual instalada do [Red](https://www.red-lang.org/). O módulo será mostrado se alguma das seguintes condições for atendida:

- O diretório atual contenha um arquivo com a extensão `.red` or `.reds`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🔺 '`                               | Uma string que representa o simbolo do Red.                                         |
| `detect_extensions` | `['red']`                            | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'red bold'`                         | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `red`.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.5.1` | A versão do `red`                 |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[red]
symbol = '🔴 '
```

## Ruby

Por padrão o módulo `ruby` vai exibir a versão atual instalada do [Ruby](https://www.ruby-lang.org/). O módulo será mostrado se alguma das seguintes condições for atendida:

- O diretório atual tenha um arquivo `Gemfile`
- O diretório atual contém um arquivo `.ruby-version`
- O diretório atual contem um arquivo `.rb`
- As variáveis de ambiente `RUBY_VERSION` ou `RBENV_VERSION` estão definidas

O Starship pega a versão atual do Ruby rodando `ruby -v`.

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💎 '`                               | Uma string que representa o simbolo do Ruby.                                        |
| `detect_extensions` | `['rb']`                             | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['Gemfile', '.ruby-version']`       | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `detect_variables`  | `['RUBY_VERSION', 'RBENV_VERSION']`  | Quais variáveis de ambiente devem ativar este módulo.                               |
| `style`             | `'bold red'`                         | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `ruby`.                                                         |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.5.1` | A versão do `ruby`                |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[ruby]
symbol = '🔺 '
```

## Rust

Por padrão o módulo `rust` vai exibir a versão atual instalada do [Rust](https://www.rust-lang.org/). O módulo será mostrado se alguma das seguintes condições for atendida:

- O diretório atual contem um arquivo `Cargo.toml`
- O diretório atual tenha um arquivo com a extensão `.rs`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦀 '`                               | Uma string que representa o simbolo do Rust                                         |
| `detect_extensions` | `['rs']`                             | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['Cargo.toml']`                     | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold red'`                         | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `rust`.                                                         |

### Variáveis

| Variável  | Exemplo           | Descrição                               |
| --------- | ----------------- | --------------------------------------- |
| version   | `v1.43.0-nightly` | A versão do `rustc`                     |
| numver    | `1.51.0`          | O componente numérico da versão `rustc` |
| toolchain | `beta`            | A versão do toolchain                   |
| symbol    |                   | Espelha o valor da opção `symbol`       |
| style\* |                   | Espelha o valor da opção `style`        |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[rust]
format = 'via [⚙️ $version](red bold)'
```

## Scala

O módulo `scala` exibe a versão atual instalada do [Scala](https://www.scala-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `build.sbt`, `.scalaenv` ou `.sbtenv`
- O diretório atual tenha um arquivo com a extensão `.scala` ou `.sbt`
- O diretório atual tenha um diretório chamado `.metals`

### Opções

| Opções              | Padrão                                   | Descrição                                                                           |
| ------------------- | ---------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                              | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['sbt', 'scala']`                       | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['.scalaenv', '.sbtenv', 'build.sbt']`  | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `['.metals']`                            | Quais pastas devem ativar este módulo.                                              |
| `symbol`            | `'🆂 '`                                   | Uma string que representa o simbolo do Scala.                                       |
| `style`             | `'red dimmed'`                           | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                  | Desabilita o módulo `scala`.                                                        |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `2.13.5` | A versão do `scala`               |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[scala]
symbol = '🌟 '
```

## Shell

O módulo de `shell` exibe um indicador para o shell que esta sendo usado.

::: tip

Este módulo é desabilitado por padrão. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

:::

### Opções

| Opções                 | Padrão                    | Descrição                                               |
| ---------------------- | ------------------------- | ------------------------------------------------------- |
| `bash_indicator`       | `'bsh'`                   | Uma string para representar o bash.                     |
| `fish_indicator`       | `'fsh'`                   | Uma string usada para representar o fish.               |
| `zsh_indicator`        | `'zsh'`                   | Uma string usada para representar o zsh.                |
| `powershell_indicator` | `'psh'`                   | Uma string usada para representar o powershell.         |
| `ion_indicator`        | `'ion'`                   | Uma string usada para representar o ion.                |
| `elvish_indicator`     | `'esh'`                   | Uma string usada para representar o elvish.             |
| `tcsh_indicator`       | `'tsh'`                   | Uma string usada para representar o tcsh.               |
| `xonsh_indicator`      | `'xsh'`                   | Uma string usada para representar o xonsh.              |
| `cmd_indicator`        | `'cmd'`                   | Uma string usada para representar o cmd.                |
| `nu_indicator`         | `'nu'`                    | Uma string usada para representar o nu.                 |
| `unknown_indicator`    | `''`                      | Valor padrão para exibir quando o shell é desconhecido. |
| `format`               | `'[$indicator]($style) '` | O formato do módulo.                                    |
| `style`                | `'white bold'`            | O estilo do módulo.                                     |
| `disabled`             | `true`                    | Desabilita o módulo `shell`.                            |

### Variáveis

| Variável  | Padrão | Descrição                                                     |
| --------- | ------ | ------------------------------------------------------------- |
| indicator |        | Espelha o valor do `indicator` para o shell usado atualmente. |
| style\* |        | Espelha o valor da opção `style`.                             |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplos

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ''
powershell_indicator = '_'
unknown_indicator = 'mystery shell'
style = 'cyan bold'
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is set to a number and meets or exceeds the specified threshold.

### Opções

| Opções      | Padrão                       | Descrição                                                         |
| ----------- | ---------------------------- | ----------------------------------------------------------------- |
| `threshold` | `2`                          | Limite de exibição.                                               |
| `format`    | `'[$symbol$shlvl]($style) '` | O formato do módulo.                                              |
| `symbol`    | `'↕️  '`                     | O simbolo usado para representar o `SHLVL`.                       |
| `repeat`    | `false`                      | Caso o `symbol` deva se repetir de acordo com o total do `SHLVL`. |
| `style`     | `'bold yellow'`              | O estilo do módulo.                                               |
| `disabled`  | `true`                       | Desabilita o módulo `shlvl`.                                      |

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| shlvl     | `3`     | O valor atual do `SHLVL`          |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[shlvl]
disabled = false
format = '$shlvl level(s) down'
threshold = 3
```

## Singularidade

O módulo `singularity` exibe a imagem atual do [Singularity](https://sylabs.io/singularity/), se dentro de um contêiner e definido `$SINGULARITY_NAME`.

### Opções

| Opções     | Padrão                           | Descrição                                   |
| ---------- | -------------------------------- | ------------------------------------------- |
| `format`   | `'[$symbol\[$env\]]($style) '` | O formato do módulo.                        |
| `symbol`   | `''`                             | Uma string exibida antes do nome da imagem. |
| `style`    | `'bold dimmed blue'`             | O estilo do módulo.                         |
| `disabled` | `false`                          | Desabilita o módulo `singularity`.          |

### Variáveis

| Variável  | Exemplo      | Descrição                         |
| --------- | ------------ | --------------------------------- |
| env       | `centos.img` | A imagem atual do Singularity     |
| symbol    |              | Espelha o valor da opção `symbol` |
| style\* |              | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Spack

O módulo `spack` mostra o ambiente [Spack](https://spack.readthedocs.io/en/latest/) atual, se o `$SPACK_ENV` estiver definido.

### Opções

| Opções              | Padrão                                 | Descrição                                                                                                                                                       |
| ------------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | O número de diretórios para os quais o caminho do ambiente deve ser truncado. `0` quer dizer sem truncação. Também consulte o módulo [`directory`](#directory). |
| `symbol`            | `'🅢  '`                                | O simbolo usado antes do nome do environment.                                                                                                                   |
| `style`             | `'bold blue'`                          | O estilo do módulo.                                                                                                                                             |
| `format`            | `'via [$symbol$environment]($style) '` | O formato do módulo.                                                                                                                                            |
| `disabled`          | `false`                                | Desabilita o módulo `spack`.                                                                                                                                    |

### Variáveis

| Variável    | Exemplo      | Descrição                         |
| ----------- | ------------ | --------------------------------- |
| environment | `astronauts` | O ambiente spack atual            |
| symbol      |              | Espelha o valor da opção `symbol` |
| style\*   |              | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[spack]
format = '[$symbol$environment](dimmed blue) '
```

## Status

O módulo `status` exibe o código de saída do comando anterior. Se o $success_symbol estiver vazio (padrão), o módulo será exibido apenas se o código de saída não for `0.`. O código de status será convertido em um inteiro de 32 bits signed.

::: tip

Este módulo é desabilitado por padrão. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

:::

### Opções

| Opções                      | Padrão                                                                             | Descrição                                                             |
| --------------------------- | ---------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                      | O formato do módulo                                                   |
| `symbol`                    | `'❌'`                                                                              | O simbolo exibido no erro de programa                                 |
| `success_symbol`            | `''`                                                                               | O simbolo exibido no sucesso de programa                              |
| `not_executable_symbol`     | `'🚫'`                                                                              | O simbolo exibido quando o arquivo não é executável                   |
| `not_found_symbol`          | `'🔍'`                                                                              | O simbolo exibido quando o comando não é encontrado                   |
| `sigint_symbol`             | `'🧱'`                                                                              | O simbolo exibido no SIGINT (Ctrl + c)                                |
| `signal_symbol`             | `'⚡'`                                                                              | O simbolo exibido em qualquer sinal                                   |
| `style`                     | `'bold red'`                                                                       | O estilo do módulo.                                                   |
| `recognize_signal_code`     | `true`                                                                             | Habilita o mapeamento de sinais para códigos de saída                 |
| `map_symbol`                | `false`                                                                            | Habilita o mapeamento de símbolos para códigos de saída               |
| `pipestatus`                | `false`                                                                            | Habilita o relatório de pipestatus                                    |
| `pipestatus_separator`      | <code>&vert;</code>                                                          | The symbol used to separate pipestatus segments (supports formatting) |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style)'` | O formato do módulo quando o comando é um pipeline                    |
| `pipestatus_segment_format` |                                                                                    | When specified, replaces `format` when formatting pipestatus segments |
| `disabled`                  | `true`                                                                             | Desabilita o módulo `status`.                                         |

### Variáveis

| Variável       | Exemplo | Descrição                                                                                                |
| -------------- | ------- | -------------------------------------------------------------------------------------------------------- |
| status         | `127`   | O codígo de saída do último comando                                                                      |
| hex_status     | `0x7F`  | O codígo de saída do último comando em hex                                                               |
| int            | `127`   | O codígo de saída do último comando                                                                      |
| common_meaning | `ERROR` | Significa que o código não é um sinal                                                                    |
| signal_number  | `9`     | Número do sinal correspondente ao código de saída, apenas se sinalizado                                  |
| signal_name    | `KILL`  | Nome do sinal correspondente ao código de saída, apenas se for sinalizado                                |
| maybe_int      | `7`     | Contém o código de saída quando nenhum significado for encontrado                                        |
| pipestatus     |         | Exibição do pipeline de programas com os códigos de saída, este é apenas disponível no pipestatus_format |
| symbol         |         | Espelha o valor da opção `symbol`                                                                        |
| style\*      |         | Espelha o valor da opção `style`                                                                         |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

O módulo `sudo` é exibido se uma credencial sudo estiver em cache. O módulo vai ser exibido somente se as credenciais estiverem em cache.

::: tip

Este módulo é desabilitado por padrão. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

:::

### Opções

| Opções          | Padrão                   | Descrição                                                                  |
| --------------- | ------------------------ | -------------------------------------------------------------------------- |
| `format`        | `'[as $symbol]($style)'` | O formato do módulo                                                        |
| `symbol`        | `'🧙 '`                   | O simbolo exibido quando as credenciais estão em cache                     |
| `style`         | `'bold blue'`            | O estilo do módulo.                                                        |
| `allow_windows` | `false`                  | Desde que o Windows não tem um padrão sudo, o valor padrão é desabilitado. |
| `disabled`      | `true`                   | Desabilita o módulo `sudo`.                                                |

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

Por padrão o módulo `swift` vai exibir a versão atual instalada do [Swift](https://swift.org/). O módulo será mostrado se alguma das seguintes condições for atendida:

- O diretório atual tenha um arquivo `Package.swift`
- O diretório atual tenha um arquivo com a extensão `.swift`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🐦 '`                               | Uma string que representa o simbolo do Swift                                        |
| `detect_extensions` | `['swift']`                          | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['Package.swift']`                  | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold 202'`                         | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `swift`.                                                        |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v5.2.4` | A versão do `swift`               |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[swift]
format = 'via [🏎  $version](red bold)'
```

## Terraform

O módulo `terraform` exibe o [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) selecionado e sua versão.

::: tip

Por padrão a versão do Terraform não é exibida, desde que é lento recuperar a versão atual quando muitos plugins estão em uso. Se você deseja habilitar,, [Siga o exemplo abaixo](#with-terraform-version).

:::

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tenha uma pasta `.terraform`
- O diretório atual tenha arquivos com as extensões `.tf`, `.tfplan` or `.tfstate`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$workspace]($style) '` | A string de formato do módulo.                                                      |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💠'`                                | Uma string que é exibida antes do workspace terraform.                              |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`        | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `['.terraform']`                     | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold 105'`                         | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `terraform`.                                                    |

### Variáveis

| Variável  | Exemplo    | Descrição                         |
| --------- | ---------- | --------------------------------- |
| version   | `v0.12.24` | A versão do `terraform`           |
| workspace | `default`  | O workspace atual do Terraform    |
| symbol    |            | Espelha o valor da opção `symbol` |
| style\* |            | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

#### Com a versão do Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = '[🏎💨 $version$workspace]($style) '
```

#### Sem a versão do Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = '[🏎💨 $workspace]($style) '
```

## Horário

O módulo `time` exibe a hora **local** atual. A configuração de `format` é usada pelo [`chrono`](https://crates.io/crates/chrono) para controlar qual hora é exibida. Dê uma olhada na [documentação do chrono strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) para ver quais opções estão disponíveis.

::: tip

Este módulo é desabilitado por padrão. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

:::

### Opções

| Opções            | Padrão                  | Descrição                                                                                                                   |
| ----------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `'at [$time]($style) '` | A string de formato do módulo.                                                                                              |
| `use_12hr`        | `false`                 | Habilita a formatação de 12 horas                                                                                           |
| `time_format`     | veja abaixo             | A string [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) usada para formatar a hora. |
| `style`           | `'bold yellow'`         | O estilo do módulo time                                                                                                     |
| `utc_time_offset` | `'local'`               | Define o UTC a ser usado. Intervalo de -24 &lt; x &lt; 24. Aceita floats para acomodar timezones 30/45.         |
| `disabled`        | `true`                  | Desabilita o módulo `time`.                                                                                                 |
| `time_range`      | `'-'`                   | Define o intervalo de tempo o qual o módulo será exibido. O horário deve ser especificado no formato de 24-hours            |

If `use_12hr` is `true`, then `time_format` defaults to `'%r'`. Otherwise, it defaults to `'%T'`. Ajustes manuais no `time_format` irão sobrescrever a configuração `use_12hr`.

### Variáveis

| Variável  | Exemplo    | Descrição                        |
| --------- | ---------- | -------------------------------- |
| time      | `13:08:10` | A hora atual.                    |
| style\* |            | Espelha o valor da opção `style` |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[time]
disabled = false
format = '🕙[\[ $time \]]($style) '
time_format = '%T'
utc_time_offset = '-5'
time_range = '10:00:00-14:00:00'
```

## Nome do usuário

O módulo `username` mostra o nome de usuário do usuário ativo. O módulo será mostrado se alguma das seguintes condições for atendida:

- O usuário atual é root/admin
- O usuário atual não é o mesmo que está logado
- O usuário atual esta conectado em uma sessão SSH
- A variável `show_always` esta definida como true

::: tip

Conexões SSH são detectadas checando as variáveis de ambiente `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. Se seu host SSH não tem estas variáveis configuradas, uma alternativa é definir uma delas com valor burro.

:::

### Opções

| Opções        | Padrão                  | Descrição                                     |
| ------------- | ----------------------- | --------------------------------------------- |
| `style_root`  | `'bold red'`            | O estilo usado quando o usuário é root/admin. |
| `style_user`  | `'bold yellow'`         | O estilo usado para usuários não root.        |
| `format`      | `'[$user]($style) in '` | O formato do módulo.                          |
| `show_always` | `false`                 | Sempre exibe o módulo `username`.             |
| `disabled`    | `false`                 | Desabilita o módulo `username`.               |

### Variáveis

| Variável | Exemplo      | Descrição                                                                              |
| -------- | ------------ | -------------------------------------------------------------------------------------- |
| `style`  | `'red bold'` | Espelha o valor da opção `style_root` quando o root esta logado e `style_user` se não. |
| `user`   | `'fulano'`   | O ID do usuário logado atualmente.                                                     |

### Exemplo

```toml
# ~/.config/starship.toml

[username]
style_user = 'white bold'
style_root = 'black bold'
format = 'user: [$user]($style) '
disabled = false
show_always = true
```

## Vagrant

O módulo `vagrant` exibe a versão atual instalada do [Vagrant](https://www.vagrantup.com/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `Vagrantfile`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⍱ '`                               | Um formato de string que representa o simbolo do Vagrant.                           |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['Vagrantfile']`                    | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'cyan bold'`                        | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `vagrant`.                                                      |

### Variáveis

| Variável  | Exemplo          | Descrição                         |
| --------- | ---------------- | --------------------------------- |
| version   | `Vagrant 2.2.10` | A versão do `Vagrant`             |
| symbol    |                  | Espelha o valor da opção `symbol` |
| style\* |                  | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

O módulo `vlang` exibe a versão atual instalada do [V](https://vlang.io/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual cotem qualquer arquivo com a extensão `.v`
- O diretório atual contem um arquivo `v.mod`, `vpkg.json` ou `.vpkg-lock.json`

### Opções

| Opções              | Padrão                                       | Descrição                                                                           |
| ------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`         | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                  | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'V '`                                       | Um formato de string que representa o simbolo do V                                  |
| `detect_extensions` | `['v']`                                      | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                         | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'blue bold'`                                | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                      | Desabilita o módulo `vlang`.                                                        |

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| version   | `v0.2`  | A versão do `v`                   |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCSH

O módulo `vcsh` exibe o repositório [VCSH](https://github.com/RichiH/vcsh) atual ativo. O módulo vai ser exibido apenas se um repositório estiver em uso.

### Opções

| Opções     | Padrão                           | Descrição                                              |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `symbol`   | `''`                             | O simbolo usado antes de exibir o nome do repositório. |
| `style`    | `'bold yellow'`                  | O estilo do módulo.                                    |
| `format`   | `'vcsh [$symbol$repo]($style) '` | O formato do módulo.                                   |
| `disabled` | `false`                          | Desabilita o módulo `vcsh`.                            |

### Variáveis

| Variável  | Exemplo                                     | Descrição                         |
| --------- | ------------------------------------------- | --------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | O nome do repositório ativo       |
| symbol    |                                             | Espelha o valor da opção `symbol` |
| style\* | `black bold dimmed`                         | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## Zig

Por padrão, o módulo `zig` mostra a versão atualmente instalada do [Zig](https://ziglang.org/). O módulo será mostrado se alguma das seguintes condições for atendida:

- O diretório atual contém arquivo com a extensão `.zig`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'↯ '`                               | O símbolo usado antes de exibir a versão do Zig.                                    |
| `style`             | `'bold yellow'`                      | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Desabilita o módulo `zig`.                                                          |
| `detect_extensions` | `['zig']`                            | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v0.6.0` | A versão do `zig`                 |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```

## Comandos Personalizados

Os módulos `custom` mostram a saída de alguns comandos arbitrários.

Esses módulos serão mostrados se alguma das seguintes condições for atendida:

- O diretório atual contém um arquivo cujo nome está em `detect_files`
- O diretório atual contém um diretório cujo nome está em `detect_folders`
- O diretório atual contém um arquivo cuja extensão está em `detect_extensions`
- O comando `when` retorna 0
- The current Operating System (std::env::consts::OS) matches with `os` field if defined.

::: tip

Vários módulos personalizados podem ser definidos usando um `.`.

:::

::: tip

A ordem na qual os módulos personalizados são mostrados pode ser definida individualmente incluindo `${custom.foo}` no `format` de nível superior (como inclui um ponto, você precisa usar `${...}`). Por padrão, o módulo `custom` mostrará todos os módulos personalizados na ordem em que eles foram definidos.

:::

::: tip

A [Issue #1252](https://github.com/starship/starship/discussions/1252) contém exemplos de módulos personalizados. Se você tem um exemplo interessante que não esta coberto lá, sinta-se livre para compartilha-lo!

:::

::: warning A saída do comando é exibido unescaped no prompt

Seja qual for a saída o comando irá gerar uma saída sem modificações no prompt. Isto siguinifica que a saída tenha sequencias especiais estas serão interpretadas pelo seu shell, elas serão expandidas quando exibidas. Estas sequencias especiais são especificas da shell, ex: você pode escrever um módulo de comando que escreve uma sequencia de bash, ex:. `\h`, mas este módulo não irá funcionar no shell fish ou zsh.

Strings de formatação também podem conter sequencias de prompt especificas de shell, ex: [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

:::

### Opções

| Opções              | Padrão                          | Descrição                                                                                                                                                                                                                                                                                                                        |
| ------------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | O comando cuja a saída deve ser exibida. O comando será passado no stdin para o shell.                                                                                                                                                                                                                                           |
| `when`              | `false`                         | Ou um valor booleano (`true` ou `false`, sem aspas) ou um comando shell de string usado como condição para mostrar o módulo. No caso de uma string, o módulo será mostrado se o comando retornar um código de status `0`.                                                                                                        |
| `shell`             |                                 | [Veja abaixo](#custom-command-shell)                                                                                                                                                                                                                                                                                             |
| `description`       | `'<custom module>'`       | A descrição do módulo, isto será exibido quando executar `starship explain`.                                                                                                                                                                                                                                                     |
| `detect_files`      | `[]`                            | Os arquivos que serão buscados por correspondência no diretório atual.                                                                                                                                                                                                                                                           |
| `detect_folders`    | `[]`                            | Os diretórios que serão buscados por correspondência no diretório atual.                                                                                                                                                                                                                                                         |
| `detect_extensions` | `[]`                            | As extensões que serão buscadas por correspondência no diretório atual.                                                                                                                                                                                                                                                          |
| `symbol`            | `''`                            | O simbolo usado antes de exibir a saída do comando.                                                                                                                                                                                                                                                                              |
| `style`             | `'bold green'`                  | O estilo do módulo.                                                                                                                                                                                                                                                                                                              |
| `format`            | `'[$symbol($output )]($style)'` | O formato do módulo.                                                                                                                                                                                                                                                                                                             |
| `disabled`          | `false`                         | Desabilita este módulo `custom`.                                                                                                                                                                                                                                                                                                 |
| `os`                |                                 | Nome do sistema operacional onde módulo sera exibido (unix, linux, macos, windows, ... ) [Veja os possíveis valores](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                                                                                 |
| `use_stdin`         |                                 | Um valor booleano opcional que substitui se os comandos devem ser encaminhados para o shell por meio da entrada padrão ou como um argumento. Se a entrada padrão não definida for usada por padrão, a menos que o shell não a suporte (cmd, nushell). Configurar isso desativa a manipulação de argumentos específicos do shell. |
| `ignore_timeout`    | `false`                         | Ignora a configuração global do `command_timeout` e continua executando comandos externos, não importa quanto tempo eles demorem.                                                                                                                                                                                                |

### Variáveis

| Variável  | Descrição                         |
| --------- | --------------------------------- |
| output    | A saída do comando no `shell`     |
| symbol    | Espelha o valor da opção `symbol` |
| style\* | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

#### Comandos personalizados de shell

`shell` aceita uma lista não vazia de string, onde:

- A primeira string é o caminho para o shell que executará o comando.
- Outros argumentos que serão passados para o shell.

If unset, it will fallback to STARSHIP_SHELL and then to 'sh' on Linux, and 'cmd /C' on Windows.

O `command` será passado no stdin.

Se o `shell` não for dado ou apenas conter um elemento e o Starship detectar PowerShell ele será usado, os seguintes argumentos serão automaticamente adicionados: `-NoProfile -Command -`. Se `shell` não for fornecido ou contiver apenas um elemento e o Starship detectar que o Cmd será usado, o seguinte argumento será adicionado automaticamente: `/C` e `stdin` serão definidos como `false`. Se `shell` não for fornecido ou contiver apenas um elemento e o Starship detectar que o Nushell será usado, os seguintes argumentos serão adicionados automaticamente: `-c` e `stdin` serão definidos como `false`. Este comportamento pode ser evitado passando explicitamente argumento para o shell, ex.

```toml
shell = ['pwsh', '-Command', '-']
```

::: warning Tenha certeza que tenha uma configuração de shell

Se você definir um comando personalizado, tenha certeza que o shell padrão usado pelo starship execute este comando com uma saída graciosa (via the `shell` option).

Por exemplo, PowerShell requer o parâmetro `-Command` para executar um liner. Omitindo este parâmetro pode ser que o starship entre em um loop recursivo, onde o shell tente carrega um ambiente completo de perfil com o próprio starship novamente e portanto execute novamente o comando, entrando em um loop sem fim.

Parâmetros similares ao `-NoProfile` no PowerShell são recomendados para outras shells, bem como para evitar um tempo de carregamento maior toda vez que o starship for chamado.

Detecção automática de shell e adição de parâmetros estão sendo implementados atualmente, mas é possível que nem todas as shells sejam cobertas. [Por favor abra uma issue](https://github.com/starship/starship/issues/new/choose) com detalhes do shell e configurações do starship se você acertar algum cenário.

:::

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
