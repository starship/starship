# Configuração

Para começar a configurar a starship, crie o seguinte arquivo: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Todas as configurações do starship são feitas neste arquivo [TOML](https://github.com/toml-lang/toml):

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

Por padrão os logs de avisos e erros do starship estão em um arquivo chamado `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, onde a session key é correspondente a uma instancia do terminal. Isto, no entanto pode ser alterado usando a variável de ambiente `STARSHIP_CACHE`:

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

Na sintaxe TOML, [valores de texto](https://toml.io/en/v1.0.0#string) são declarados com `'`, `"`, `'''`, ou `"""`.

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

Ao usar quebras de linha, declarações de várias linhas podem ser usadas. Por exemplo, se você quiser imprimir um símbolo `$` em uma nova linha, os seguintes valores para o `format` são equivalentes:

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

As strings de formato são o formato com o qual um módulo imprime todas as suas variáveis. A maioria dos módulos tem uma entrada chamada `format` que configura o formato de exibição do módulo. Você pode usar textos, variáveis e grupos de texto em uma string de formato.

#### Variável

Uma variável contém um símbolo `$` seguido pelo nome da variável. O nome de uma variável pode conter apenas letras, números e `_`.

Por exemplo:

- `'$version'` é uma string de formato com uma variável chamada `version`.
- `'$git_branch$git_commit'` é uma string de formato com duas variáveis chamadas `git_branch` e `git_commit`.
- `'$git_branch $git_commit'` tem as duas variáveis separadas por um espaço.

#### Grupo de Texto

Um grupo de texto é composto de duas partes diferentes.

A primeira parte, que está entre um `[]`, é uma [string de formato](#format-strings). Você pode adicionar textos, variáveis ou até mesmo grupos de texto aninhados nele.

Na segunda parte, que está dentro de um `()`, está uma [string de estilo](#style-strings). Isso pode ser usado para estilizar a primeira parte.

Por exemplo:

- `'[on](red bold)'` irá imprimir uma string `em` com texto em negrito vermelho.
- `'[⌘ $version](bold green)'` imprimirá um símbolo  `⌘` seguido pelo conteúdo da `version`, com texto negrito verde.
- `'[a [b](red) c](green)'` imprimirá  `a b c`  com `b` vermelhor, `a` e `c` verdes.

#### Estilo dos textos

A maioria dos módulos no starship permite que você configure seus estilos de exibição. Isso é feito com uma entrada (normalmente chamada de `estilo`) que é uma string especificando a configuração. Aqui estão alguns exemplos de strings de estilo junto com o que elas fazem. Para detalhes da sintaxe completa, consulte o [ guia de configurações avançadas ](../advanced-config/).

- `"fg:green bg:blue"` define o texto para verde e o fundo azul
- `"bg:blue fg:bright-green"` define o texto para verde brilhante e o fundo azul
- `"bold fg:27"` define o texto para negrito com a cor 27 [da tabela ANSI](https://i.stack.imgur.com/KTSQa.png)
- `"underline bg:#bf5700"` define o texto para sublinhado com o fundo laranja escuro
- `"bold italic fg:purple"` define o texto para negrito e itálico com a cor roxa
- `"` explicitamente desabilita todos os estilos

Observe que a aparência do estilo será controlada pelo emulador de terminal. Por exemplo, alguns emuladores de terminal irão clarear as cores em vez de colocar o texto em negrito, e alguns temas de cores usam os mesmos valores para as cores normais e brilhantes. Além disso, para obter texto em itálico, seu terminal deve suportar itálico.

#### Formatação de String Condicional

Uma string de formato condicional envolta de `(` e `)` não será renderizada se todas as variáveis internas estiverem vazias.

Por exemplo:

- `(@$region)` não vai exibir nada caso a variável `region` seja `None` ou vazia, caso contrario vai exibir `@` seguido pelo valor da variável region.
- `(texto qualquer)` não vai exibir nada sempre, pois não existe variável entre os parenteses.
- Quando usar `$combined` é um atalho para `\[$a$b\]`, `($combined)` só vai exibir algo se  `$a` e `$b` são `None`. Isto funciona da mesma forma que `(\[$a$b\] )`.

### Correspondência negativa

Muitos módulos têm variáveis `detect_extensions`,  `detect_files`, e `detect_folders`. Estas receberão listas de strings para coresponder ou não. Opções "negativas", aquelas que não tem correspondencia, são indicadas com um caractere  '!'. A presença de _varios_ indicadores negativos no diretório resultara que o módulo não sera correspondido.

As extensões são combinadas com os dois caracteres após o último ponto em um nome de arquivo e os caracteres após o primeiro ponto em um nome de arquivo. Por exemplo, `foo.bar.tar.gz` vai ser comparada com  `bar.tar.gz` e `gz` na `detect_extensions` variavel. Arquivos que o nome começa com um ponto não são considerados ter nenhuma extensão.

Para ver como isso funciona na prática, você pode combinar TypeScript mas não arquivos MPEG Transport Stream:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## Prompt de Comando

Esta é a lista de opções de configuração em todo o prompt.

### Opções

| Opções            | Padrão                         | Descrição                                                                                                                                                                                       |
| ----------------- | ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | Configura o formato do prompt.                                                                                                                                                                  |
| `right_format`    | `''`                           | See [Enable Right Prompt](../advanced-config/#enable-right-prompt)                                                                                                                              |
| `scan_timeout`    | `30`                           | Tempo limite para escanear arquivos (em milissegundos).                                                                                                                                         |
| `command_timeout` | `500`                          | Tempo limite de execução de comandos pelo starship (em milissegundos).                                                                                                                          |
| `add_newline`     | `true`                         | Insere linha vazia entre os prompts do shell.                                                                                                                                                   |
| `palette`         | `''`                           | Define qual a paleta de cores de `palettes` será usada.                                                                                                                                         |
| `palettes`        | `{}`                           | Collection of color palettes that assign [colors](../advanced-config/#style-strings) to user-defined names. Note que paletas de cores não podem referir-se a suas próprias definições de cores. |
| `follow_symlinks` | `true`                         | Follows symlinks to check if they're directories; used in modules such as git.                                                                                                                  |

> [!TIP] If you have symlinks to networked filesystems, consider setting `follow_symlinks` to `false`.

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

Se você quiser apenas estender o formato padrão, você pode usar `$all`; os módulos que você adicionar explicitamente ao formato não serão duplicados. Ex.

```toml
# Mova o diretório para a segunda linha
format = '$all$directory$character'
```

## AWS

The `aws` module shows the current AWS region and profile and an expiration timer when using temporary credentials. The output of the module uses the `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` env vars and the `~/.aws/config` and `~/.aws/credentials` files as required.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or if a `credential_process`, `sso_start_url`, or `sso_session` are defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice. If the option `force_display` is set to `true`, all available information will be displayed even if no credentials per the conditions above are detected.

Quando usar [aws-vault](https://github.com/99designs/aws-vault), o perfil é lido da variável de ambiente `AWS_VAULT` e o tempo de expiração de credenciais é lido da variável de ambiente `AWS_SESSION_EXPIRATION`.

Quando usar [awsu](https://github.com/kreuzwerker/awsu) o perfil é lido da varável de env `AWSU_PROFILE`.

Quando usar [AWSume](https://awsu.me) o perfil é lido da variável `AWSUME_PROFILE` e o tempo de expiração de credenciais é lida da variável de env `AWSUME_EXPIRATION`.

When using [saml2aws](https://github.com/Versent/saml2aws) the expiration information obtained from `~/.aws/credentials` falls back to the `x_security_token_expires` key.

When using [aws-sso-cli](https://github.com/synfinatic/aws-sso-cli) the profile is read from the `AWS_SSO_PROFILE` env var.

### Opções

| Opções              | Padrão                                                                | Descrição                                                                                                                     |
| ------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\($region\) )(\[$duration\] )]($style)'` | O formato do módulo.                                                                                                          |
| `symbol`            | `'☁️ '`                                                               | O símbolo usado antes de exibir o perfil atual da AWS.                                                                        |
| `region_aliases`    | `{}`                                                                  | Tabela de aleases de regiões a serem exibidas, além do nome da AWS.                                                           |
| `profile_aliases`   | `{}`                                                                  | Tabela de apelidos de perfil a serem exibidos além do nome da AWS.                                                            |
| `style`             | `'bold yellow'`                                                       | O estilo do módulo.                                                                                                           |
| `expiration_symbol` | `'X'`                                                                 | O simbolo exibido quando as credenciais temporárias estão expiradas.                                                          |
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

O módulo `azure` exibe a assinatura Azure atual. Isto é baseado na exibição do nome da assinatura padrão ou no nome do usuário, como definido no arquivo `~/.azure/azureProfile.json`.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Variável               | Padrão                                   | Descrição                                                                             |
| ---------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------- |
| `format`               | `'on [$symbol($subscription)]($style) '` | O formato que o módulo Azure será renderizado.                                        |
| `symbol`               | `'󰠅 '`                                   | O símbolo usado no formato.                                                           |
| `style`                | `'blue bold'`                            | O estilo usado no formato.                                                            |
| `disabled`             | `true`                                   | Desabilita o módulo `azure`.                                                          |
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

O módulo `battery` exibe o quanto a bateria do dispositivo está carregada e o estado atual de carregamento. O módulo é visível somente quando a bateria está abaixo de 10%.

### Opções

| Opções               | Padrão                            | Descrição                                                    |
| -------------------- | --------------------------------- | ------------------------------------------------------------ |
| `full_symbol`        | `'󰁹 '`                            | O simbolo exibido quando a bateria estiver cheia.            |
| `charging_symbol`    | `'󰂄 '`                            | O simbolo exibido quando a bateria está carregando.          |
| `discharging_symbol` | `'󰂃 '`                            | O simbolo exibido quando a bateria está descarregando.       |
| `unknown_symbol`     | `'󰂑 '`                            | O simbolo exibido quando o estado da bateria é desconhecido. |
| `empty_symbol`       | `'󰂎 '`                            | O simbolo exibido quando o estado da bateria é vazio.        |
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
discharging_symbol = '💦 '

# when capacity is over 30%, the battery indicator will not be displayed
```

## Buf

O módulo `buf` mostra a versão instalada do [Buf](https://buf.build). By default, the module is shown if the current directory contains a [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml), or [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) configuration file.

### Opções

| Opções              | Padrão                                | Descrição                                         |
| ------------------- | ------------------------------------- | ------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'` | O formato do módulo `buf`.                        |
| `version_format`    | `'v${raw}'`                           | A versão formatada.                               |
| `symbol`            | `'🐃 '`                                | O símbolo usado antes de exibir a versão do Buf.  |
| `detect_extensions` | `[]`                                  | Quais extensões devem ativar este módulo.         |
| `detect_files`      | `[]`                                  | Quais nomes de arquivos devem ativar este módulo. |
| `detect_folders`    | `[]`                                  | Quais pastas devem ativar este módulo.            |
| `style`             | `'bold blue'`                         | O estilo do módulo.                               |
| `disabled`          | `false`                               | Desabilita o módulo `elixir`.                     |

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

O módulo `bun` mostra a versão atualmente instalada do [bun](https://bun.sh) runtime do JavaScript. Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `bun.lock`
- O diretório atual conter um arquivo `bun.lockb`
- O diretório atual conter um arquivo `bunfig.toml`

### Opções

| Opções              | Padrão                                     | Descrição                                                                           |
| ------------------- | ------------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`       | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🥟 '`                                     | Uma string de formato que representa o símbolo do Bun.                              |
| `detect_extensions` | `[]`                                       | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                       | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold red'`                               | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                    | Desativa o módulo `bun`.                                                            |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v0.1.4` | A versão do `bun`                 |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

#### Customize the format

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

O módulo `c` mostra algumas informações sobre o seu compilador de C. Por padrão o módulo será exibido se o diretório atual contém um arquivo `.c` ou `.h`.

### Opções

| Opções              | Padrão                                                                        | Descrição                                                                           |
| ------------------- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | A string de formato do módulo.                                                      |
| `version_format`    | `'v${raw}'`                                                                   | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'C '`                                                                        | O símbolo utilizado antes de exibir os detalhes do compilador                       |
| `detect_extensions` | `['c', 'h']`                                                                  | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                                                          | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                                                          | Quais pastas devem ativar este módulo.                                              |
| `commands`          | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | Como detectar qual é o compilador                                                   |
| `style`             | `'bold 149'`                                                                  | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                                       | Desabilita o módulo `c`.                                                            |

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| name     | clang   | O nome do compilador              |
| version  | 13.0.0  | A versão do compilador            |
| symbol   |         | Espelha o valor da opção `symbol` |
| style    |         | Espelha o valor da opção `style`  |

### Comandos

A opção `commands` aceita uma lista de comandos para determinar a versão e o nome do compilador.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['mycc', '--version']`. Starship tentará executar cada comando até que obtenha um resultado no STDOUT.

If a C compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/issues/new/choose).

### Exemplo

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## CPP

The `cpp` module shows some information about your `C++` compiler. By default, the module will be shown if the current directory contains a `.cpp`, `.hpp`, or other `C++`-related files.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções              | Padrão                                                                           | Descrição                                                                            |
| ------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                     | A string de formato do módulo.                                                       |
| `version_format`    | `'v${raw}'`                                                                      | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'C++ '`                                                                         | O símbolo utilizado antes de exibir os detalhes do compilador                        |
| `detect_extensions` | `['cpp', 'cc', 'cxx', 'c++', 'hpp', 'hh', 'hxx', 'h++', 'tcc']`                  | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[]`                                                                             | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                                             | Quais pastas devem ativar este módulo.                                               |
| `commands`          | `[ [ 'c++', '--version' ], [ 'g++', '--version' ], [ 'clang++', '--version' ] ]` | Como detectar qual é o compilador                                                    |
| `style`             | `'bold 149'`                                                                     | O estilo do módulo.                                                                  |
| `disabled`          | `true`                                                                           | Disables the `cpp` module.                                                           |

### Variáveis

| Variável | Exemplo | Descrição                         |
| -------- | ------- | --------------------------------- |
| name     | clang++ | O nome do compilador              |
| version  | 13.0.0  | A versão do compilador            |
| symbol   |         | Espelha o valor da opção `symbol` |
| style    |         | Espelha o valor da opção `style`  |

### Comandos

A opção `commands` aceita uma lista de comandos para determinar a versão e o nome do compilador.

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

O módulo `character` exibe um caracter (normalmente uma seta) ao lado de onde o texto começa a ser inserido no terminal.

O caractere vai te dizer se o ultimo comando foi bem sucedido ou não. Você pode fazer isto de duas maneiras:

- alterando a cor (`red`/`green`)
- alterando a forma (`❯`/`✖`)

Por padrão ele apenas muda de cor. Se você deseja alterar o formato de uma olhada [neste exemplo](#with-custom-error-shape).

> [!WARNING] `vimcmd_symbol` is only supported in cmd, fish and zsh. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol`, and `vimcmd_visual_symbol` are only supported in fish due to [upstream issues with mode detection in zsh](https://github.com/starship/starship/issues/625#issuecomment-732454148).

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

| Variável | Exemplo | Descrição                                                                                           |
| -------- | ------- | --------------------------------------------------------------------------------------------------- |
| symbol   |         | Um espelho de `success_symbol`, `error_symbol`, `vimcmd_symbol` ou `vimcmd_replace_one_symbol` etc. |

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

O módulo `cmake` exibe a versão instalada do [CMake](https://cmake.org/). Por padrão o módulo será ativo se qualquer das condições a seguir for atendida:

- O diretorio atual cotem um arquivo `CMakeLists.txt`
- O diretorio atual tem um arquivo `CMakeCache.txt`

### Opções

| Opções              | Padrão                                 | Descrição                                                                           |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`   | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                            | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'△ '`                                 | O simbolo usado antes da versão do cmake.                                           |
| `detect_extensions` | `[]`                                   | Quais extensões devem acionar este módulo                                           |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | []                                                                                  |
| `detect_folders`    | `[]`                                   | Quais pastas devem ativar este módulo                                               |
| `style`             | `'bold blue'`                          | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                | Desabilita o módulo `cmake`.                                                        |

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

> [!WARNING] Do not hook the DEBUG trap in Bash
> 
> If you are running Starship in `bash`, do not hook the `DEBUG` trap after running `eval $(starship init $0)`, or this module **will** break.

Bash users who need preexec-like functionality can use [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simplesmente defina os arrays `preexec_functions` e `precmd_functions` antes de rodar `eval $(starship init $0)`, e depois pode proceder normalmente.

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

The `conda` module shows the current [Conda](https://docs.conda.io/en/latest/) environment, if `$CONDA_DEFAULT_ENV` is set.

> [!TIP] This does not suppress conda's own prompt modifier, you may want to run `conda config --set changeps1 False`. If you use [pixi](https://pixi.sh), you can disable pixi's prompt modifier by running `pixi config set shell.change-ps1 false`.

### Opções

| Opções              | Padrão                                 | Descrição                                                                                                                                                                                                  |
| ------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | O número de diretórios do envirionment path deve ser truncado, se o environment foi criado via `conda create -p [path]`. `0` quer dizer sem truncação. Também consulte o módulo [`directory`](#directory). |
| `symbol`            | `'🅒 '`                                 | O simbolo usado antes do nome do environment.                                                                                                                                                              |
| `style`             | `'bold green'`                         | O estilo do módulo.                                                                                                                                                                                        |
| `format`            | `'via [$symbol$environment]($style) '` | O formato do módulo.                                                                                                                                                                                       |
| `ignore_base`       | `true`                                 | Ignora o environment `base` quando ativado.                                                                                                                                                                |
| `detect_env_vars`   | `["!PIXI_ENVIRONMENT_NAME"]`           | Which environment variable(s) should trigger this module. If it's a pixi environment, this module is not being triggered by default.                                                                       |
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

The `crystal` module shows the currently installed version of [Crystal](https://crystal-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `shard.yml`
- O diretório atual contem um arquivo `.cr`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `symbol`            | `'🔮 '`                               | O símbolo usado antes de exibir a versão do crystal.                                 |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                 |
| `version_format`    | `'v${raw}'`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold red'`                         | O estilo do módulo.                                                                  |
| `detect_extensions` | `['cr']`                             | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `['shard.yml']`                      | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `disabled`          | `false`                              | Desabilita o módulo `crystal`.                                                       |

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

The `dart` module shows the currently installed version of [Dart](https://dart.dev/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

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

The `deno` module shows you your currently installed version of [Deno](https://deno.land/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` or `deps.js` file

### Opções

| Opções              | Padrão                                                                               | Descrição                                                                           |
| ------------------- | ------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                                                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🦕 '`                                                                               | Um formato de string que representa o simbolo do Deno                               |
| `detect_extensions` | `[]`                                                                                 | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                                                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'green bold'`                                                                       | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                                              | Desabilita o módulo `deno`.                                                         |

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

Ao usar a opção `fish_style_pwd_dir_length`, em vez de esconder o caminho que é truncado, você verá um nome encurtado de cada diretório com base no número que você habilitar para a opção.

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
| `substituições`             |        | An Array or table of substitutions to be made to the path.                                                                                                            |
| `fish_style_pwd_dir_length` | `0`    | O número de caracteres para usar quando aplicado no path logico do fish shell pwd.                                                                                    |
| `use_logical_path`          | `true` | Se `true` exibe um caminho lógico originado do shell via `PWD` ou`--logical-path`. Se `false` em vez disso, exibe o caminho do filesystem com os symlinks resolvidos. |

`substitutions` allows you to define arbitrary replacements for literal strings that occur in the path, for example long network prefixes or development directories of Java. Note isto irá desabilita o estilo PWD do fish. It takes an array of the following key/value pairs:

| Value   | Tipo    | Descrição                                |
| ------- | ------- | ---------------------------------------- |
| `from`  | String  | The value to substitute                  |
| `to`    | String  | The replacement for that value, if found |
| `regex` | Boolean | (Optional) Whether `from` is a regex     |

By using `regex = true`, you can use [Rust's regular expressions](https://docs.rs/regex/latest/regex/#syntax) in `from`. For instance you can replace every slash except the first with the following:

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

`fish_style_pwd_dir_length` interage com as opções de truncação padrão de uma forma que pode suprimir no começo: se não for zero, os componentes do path que normalmente seriam truncados são exibidos com todos caracteres. For example, the path `/built/this/city/on/rock/and/roll`, which would normally be displayed as `rock/and/roll`, would be displayed as `/b/t/c/o/rock/and/roll` with `fish_style_pwd_dir_length = 1`--the path components that would normally be removed are displayed with a single character. Para `fish_style_pwd_dir_length = 2`, seria `/bu/th/ci/on/rock/and/roll`.

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

## Direnv

The `direnv` module shows the status of the current rc file if one is present. The status includes the path to the rc file, whether it is loaded, and whether it has been allowed by `direnv`.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções              | Padrão                                 | Descrição                                             |
| ------------------- | -------------------------------------- | ----------------------------------------------------- |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | O formato do módulo.                                  |
| `symbol`            | `'direnv '`                            | The symbol used before displaying the direnv context. |
| `style`             | `'bold orange'`                        | O estilo do módulo.                                   |
| `disabled`          | `true`                                 | Disables the `direnv` module.                         |
| `detect_extensions` | `[]`                                   | Quais extensões devem ativar este módulo.             |
| `detect_files`      | `['.envrc']`                           | Quais nomes de arquivos devem ativar este módulo.     |
| `detect_folders`    | `[]`                                   | Quais pastas devem ativar este módulo.                |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | Quais variáveis de ambiente devem ativar este módulo. |
| `allowed_msg`       | `'allowed'`                            | The message displayed when an rc file is allowed.     |
| `not_allowed_msg`   | `'not allowed'`                        | The message displayed when an rc file is not_allowed. |
| `denied_msg`        | `'denied'`                             | The message displayed when an rc file is denied.      |
| `loaded_msg`        | `'loaded'`                             | The message displayed when an rc file is loaded.      |
| `unloaded_msg`      | `'not loaded'`                         | The message displayed when an rc file is not loaded.  |

### Variáveis

| Variável  | Exemplo             | Descrição                               |
| --------- | ------------------- | --------------------------------------- |
| loaded    | `loaded`            | Whether the current rc file is loaded.  |
| allowed   | `denied`            | Whether the current rc file is allowed. |
| rc_path   | `/home/test/.envrc` | The current rc file path.               |
| symbol    |                     | Espelha o valor da opção `symbol`.      |
| style\* | `red bold`          | Espelha o valor da opção `style`.       |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```

## Docker Context

The `docker_context` module shows the currently active [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) if it's not set to `default` or `desktop-linux`, or if the `DOCKER_MACHINE_NAME`, `DOCKER_HOST` or `DOCKER_CONTEXT` environment variables are set (as they are meant to override the context in use).

### Opções

| Opções              | Padrão                                                                                       | Descrição                                                                            |
| ------------------- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol$context]($style) '`                                                           | O formato do módulo.                                                                 |
| `symbol`            | `'🐳 '`                                                                                       | O simbolo usado antes de exibir a versão do contexto docker.                         |
| `only_with_files`   | `true`                                                                                       | Exibe somente quando houver um arquivo                                               |
| `detect_extensions` | `[]`                                                                                         | Quais extensões devem acionar este módulo (precisa que `only_with_files` seja true). |
| `detect_files`      | `['compose.yml', 'compose.yaml', 'docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | Quais arquivos devem acionar este módulo (precisa que `only_with_files` seja true).  |
| `detect_folders`    | `[]`                                                                                         | Quais pastas devem acionar este módulo (precisa que `only_with_files` seja true).    |
| `style`             | `'blue bold'`                                                                                | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                                      | Desabilita o módulo `docker_context`.                                                |

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

> [!TIP] The order in which env_var modules are shown can be individually set by including `${env_var.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `env_var` module will simply show all env_var modules in the order they were defined.

> [!TIP] Multiple environmental variables can be displayed by using a `.`. (see example) If the `variable` configuration option is not set, the module will display value of variable under the name of text after the `.` character.
> 
> Exemplo: a configuração a seguir irá mostrar o valor da variável de ambiente USER
> 
> ```toml
> 
> # ~/.config/starship.toml
> 
> [env_var.USER] default = 'unknown user' ```

### Opções

| Opções        | Padrão                                | Descrição                                                                    |
| ------------- | ------------------------------------- | ---------------------------------------------------------------------------- |
| `symbol`      | `""`                                  | O símbolo usado antes de exibir o valor da variável.                         |
| `variable`    |                                       | A variável de ambiente a ser exibida.                                        |
| `default`     |                                       | O valor padrão a ser exibido quando a variável selecionada não for definida. |
| `format`      | `"with [$symbol$env_value]($style) "` | O formato do módulo.                                                         |
| `description` | `"<env_var module>"`            | A descrição do módulo, isto será exibido quando executar `starship explain`. |
| `disabled`    | `false`                               | Desabilita o módulo `env_var`.                                               |
| `style`       | `"black bold dimmed"`                 | O estilo do módulo.                                                          |

### Variáveis

| Variável  | Exemplo                                   | Descrição                               |
| --------- | ----------------------------------------- | --------------------------------------- |
| env_value | `Windows NT` (se a variável __ for `$OS`) | O valor de ambiente da opção `variable` |
| symbol    |                                           | Espelha o valor da opção `symbol`       |
| style\* |                                           | Espelha o valor da opção `style`        |

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
| `detect_extensions` | `['fnl']`                            | Quais extensões devem ativar este módulo.                                           |
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

## Fortran

The `fortran` module shows the current compiler version of Fortran.

### Opções

| Opções              | Padrão                                                                                                                      | Descrição                                                                           |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `symbol`            | `' '`                                                                                                                      | The symbol used before displaying the version of Fortran.                           |
| `format`            | `'via [$symbol($version )]($style)'`                                                                                        | O formato do módulo.                                                                |
| `version_format`    | `'${raw}'`                                                                                                                  | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`             | `'bold purple'`                                                                                                             | O estilo do módulo.                                                                 |
| `detect_extensions` | `['f', 'F', 'for', 'FOR', 'ftn', 'FTN', 'f77', 'F77', 'f90', 'F90', 'f95', 'F95','f03', 'F03', 'f08', 'F08', 'f18', 'F18']` | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['fpm.toml']`                                                                                                              | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                                                                                                        | Quais pastas devem ativar este módulo.                                              |
| `commands`          | `[ [ 'gfortran', '--version' ], [ 'flang', '--version' ], [ 'flang-new', '--version' ] ]`                                   | Como detectar qual é o compilador                                                   |
| `disabled`          | `false`                                                                                                                     | Disables the `fortran` module.                                                      |

### Variáveis

| Variável  | Exemplo  | Descrição                           |
| --------- | -------- | ----------------------------------- |
| name      | gfortran | O nome do compilador                |
| version   | `14.2.0` | The version of the Fortran compiler |
| symbol    |          | Espelha o valor da opção `symbol`   |
| style\* |          | Espelha o valor da opção `style`    |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Comandos

A opção `commands` aceita uma lista de comandos para determinar a versão e o nome do compilador.

Each command is represented as a list of the executable name, followed by its arguments, usually something like `['myfortran', '--version']`. Starship tentará executar cada comando até que obtenha um resultado no STDOUT.

If a Fortran compiler is not supported by this module, you can request it by [raising an issue on GitHub](https://github.com/starship/starship/).

## Fossil Branch

The `fossil_branch` module shows the name of the active branch of the check-out in your current directory.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

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

## Fossil Metrics

The `fossil_metrics` module will show the number of added and deleted lines in the check-out in your current directory. At least v2.14 (2021-01-20) of Fossil is required.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções               | Padrão                                                       | Descrição                                   |
| -------------------- | ------------------------------------------------------------ | ------------------------------------------- |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | O formato do módulo.                        |
| `added_style`        | `'bold green'`                                               | O estilo para a contagem de adições.        |
| `deleted_style`      | `'bold red'`                                                 | O estilo para a contagem de exclusões.      |
| `only_nonzero_diffs` | `true`                                                       | Exibe apenas o status para itens alterados. |
| `disabled`           | `true`                                                       | Disables the `fossil_metrics` module.       |

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

[fossil_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Google Cloud (`gcloud`)

O módulo `gcloud` exibe a configuração atual para a CLI do [`gcloud`](https://cloud.google.com/sdk/gcloud). Isto é baseadp mp arquivo `~/.config/gcloud/active_config` e no arquivo`~/.config/gcloud/configurations/config_{CONFIG NAME}` e a env var `CLOUDSDK_CONFIG`.

When the module is enabled it will always be active, unless `detect_env_vars` has been set in which case the module will only be active when one of the environment variables has been set.

### Opções

| Opções            | Padrão                                                     | Descrição                                                           |
| ----------------- | ---------------------------------------------------------- | ------------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | O formato do módulo.                                                |
| `symbol`          | `'☁️  '`                                                   | O simbolo usado antes de exibir o perfil atual do GCP.              |
| `region_aliases`  | `{}`                                                       | Tabela de aliases de região para exibir além do nome do GCP.        |
| `project_aliases` | `{}`                                                       | Tabela de apelidos do projeto a serem exibidos além do nome do GCP. |
| `detect_env_vars` | `[]`                                                       | Which environmental variables should trigger this module            |
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
| `ignore_bare_repo`   | `false`                                           | Do not show when in a bare repo.                                                            |
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
| `tag_symbol`         | `' 🏷  '`                       | Simbolo da tag prefixado na informação a ser exibida                                 |
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

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções               | Padrão                                                       | Descrição                                   |
| -------------------- | ------------------------------------------------------------ | ------------------------------------------- |
| `added_style`        | `'bold green'`                                               | O estilo para a contagem de adições.        |
| `deleted_style`      | `'bold red'`                                                 | O estilo para a contagem de exclusões.      |
| `only_nonzero_diffs` | `true`                                                       | Exibe apenas o status para itens alterados. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | O formato do módulo.                        |
| `disabled`           | `true`                                                       | Desabilita o módulo `git_metrics`.          |
| `ignore_submodules`  | `false`                                                      | Ignora as alterações de submódulos          |

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

> [!TIP] The Git Status module is very slow in Windows directories (for example under `/mnt/c/`) when in a WSL environment. Você pode desabilitar o módulo ou usar a opção `windows_starship` para usar um executável Starship nativo do Windows para calcular o `git_status` para esses caminhos.

### Opções

| Opções                 | Padrão                                          | Descrição                                                                                                                               |
| ---------------------- | ----------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `format`               | `'([\[$all_status$ahead_behind\]]($style) )'` | O formato padrão para `git_status`                                                                                                      |
| `conflicted`           | `'='`                                           | The format shown when this branch has merge conflicts.                                                                                  |
| `ahead`                | `'⇡'`                                           | The format shown when this branch is ahead of the branch being tracked.                                                                 |
| `behind`               | `'⇣'`                                           | The format shown when this branch is behind the branch being tracked.                                                                   |
| `diverged`             | `'⇕'`                                           | The format shown when this branch has diverged from the branch being tracked.                                                           |
| `up_to_date`           | `''`                                            | The format shown when this branch is up to date with the branch being tracked.                                                          |
| `untracked`            | `'?'`                                           | The format shown when there are untracked files in the working directory.                                                               |
| `stashed`              | `'\$'`                                         | The format shown when a stash exists for the local repository.                                                                          |
| `modified`             | `'!'`                                           | The format shown when there are file modifications in the working directory.                                                            |
| `staged`               | `'+'`                                           | The format shown when a new file has been added to the staging area.                                                                    |
| `renamed`              | `'»'`                                           | The format shown when a renamed file has been added to the staging area.                                                                |
| `deleted`              | `'✘'`                                           | The format shown when a file's deletion has been added to the staging area.                                                             |
| `typechanged`          | `""`                                            | The format shown when a file's type has been changed in the staging area.                                                               |
| `style`                | `'bold red'`                                    | O estilo do módulo.                                                                                                                     |
| `ignore_submodules`    | `false`                                         | Ignora as alterações de submódulos.                                                                                                     |
| `worktree_added`       | `""`                                            | The format shown when a new file has been added in the working directory.                                                               |
| `worktree_deleted`     | `""`                                            | The format shown when a file has been deleted in the working directory.                                                                 |
| `worktree_modified`    | `""`                                            | The format shown when a file has been modified in the working directory.                                                                |
| `worktree_typechanged` | `""`                                            | The format shown when a file's type has been changed in the working directory.                                                          |
| `index_added`          | `""`                                            | The format shown when a new file has been added to the staging area.                                                                    |
| `index_deleted`        | `""`                                            | The format shown when a file has been deleted from the staging area.                                                                    |
| `index_modified`       | `""`                                            | The format shown when a file has been modified in the staging area.                                                                     |
| `index_typechanged`    | `""`                                            | The format shown when a file's type has been changed in the staging area.                                                               |
| `disabled`             | `false`                                         | Desabilita o módulo `git_status`.                                                                                                       |
| `windows_starship`     |                                                 | Use este caminho (Linux) para um executável do Windows Starship renderizar o `git_status` quando estiver em caminhos do Windows no WSL. |
| `use_git_executable`   | `false`                                         | Do not use `gitoxide` for computing the status, but use the `git` executable instead.                                                   |

### Variáveis

As variáveis a seguir podem ser usadas no `format`:

| Variável               | Descrição                                                                                                  |
| ---------------------- | ---------------------------------------------------------------------------------------------------------- |
| `all_status`           | Shortcut for `$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked`.                  |
| `ahead_behind`         | Exibe `diverged`, `ahead`, `behind` or `up_to_date` conforme o formato da string do status do repositório. |
| `conflicted`           | Exibe `conflicted` quando este braço tenha conflitos no merge.                                             |
| `untracked`            | Exibe `untracked` quando há arquivos não rastreados no diretório atual.                                    |
| `stashed`              | Exibe `stashed` quando um stash existe para o repositório local.                                           |
| `modified`             | Exibe `modified` quando um arquivo tenha modificações for adicionado na área de staging.                   |
| `staged`               | Exibe `staged` quando um arquivo novo for adicionado na área de staging.                                   |
| `renamed`              | Exibe `renamed` quando um arquivo renomeado for adicionado na área de staging.                             |
| `deleted`              | Exibe `deleted` quando um arquivo deletado for adicionado na área de staging.                              |
| `typechanged`          | Displays `typechanged` when a file's type has been changed in the staging area.                            |
| `worktree_added`       | Displays `worktree_added` when a new file has been added in the working directory.                         |
| `worktree_deleted`     | Displays `worktree_deleted` when a file's been deleted in the working directory.                           |
| `worktree_modified`    | Displays `worktree_modified` when a file's been modified in the working directory.                         |
| `worktree_typechanged` | Displays `worktree_typechanged` when a file's type has been changed in the working directory.              |
| `index_added`          | Displays `index_added` when a new file has been added to the staging area.                                 |
| `index_deleted`        | Displays `index_deleted` when a file has been deleted from the staging area.                               |
| `index_modified`       | Displays `index_modified` when a file has been modified in the staging area.                               |
| `index_typechanged`    | Displays `index_typechanged` when a file's type has been changed in the staging area.                      |
| style\*              | Espelha o valor da opção `style`                                                                           |

*: Esta variável só pode ser usada como parte de uma string de estilo

As variáveis a seguir podem ser usadas em `diverged`:

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

The `gleam` module shows the currently installed version of [Gleam](https://gleam.run/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `gleam.toml`
- The current directory contains a file with the `.gleam` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⭐ '`                               | A format string representing the symbol of Gleam.                                   |
| `detect_extensions` | `['gleam']`                          | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['gleam.toml']`                     | Quais nomes de arquivos devem ativar este módulo.                                   |
| `style`             | `'bold #FFAFF3'`                     | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Disables the `gleam` module.                                                        |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.0.0` | The version of `gleam`            |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[gleam]
format = 'via [⭐ $version](bold red) '
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

| Opções              | Padrão                                                                                    | Descrição                                                                                                  |
| ------------------- | ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | O formato do módulo.                                                                                       |
| `version_format`    | `'v${raw}'`                                                                               | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch`                        |
| `symbol`            | `'🐹 '`                                                                                    | O formato da string que representa o simbolo do Go.                                                        |
| `detect_extensions` | `['go']`                                                                                  | Quais extensões devem ativar este módulo.                                                                  |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | Quais nomes de arquivos devem ativar este módulo.                                                          |
| `detect_folders`    | `['Godeps']`                                                                              | Quais pastas devem ativar este módulo.                                                                     |
| `style`             | `'bold cyan'`                                                                             | O estilo do módulo.                                                                                        |
| `not_capable_style` | `'bold red'`                                                                              | The style for the module when the go directive in the go.mod file does not match the installed Go version. |
| `disabled`          | `false`                                                                                   | Desabilita o módulo `golang`.                                                                              |

### Variáveis

| Variável    | Exemplo   | Descrição                                                                                                                                   |
| ----------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| version     | `v1.12.1` | A versão do `go`                                                                                                                            |
| mod_version | `1.16`    | `go` version requirement as set in the go directive of `go.mod`. Will only show if the version requirement does not match the `go` version. |
| symbol      |           | Espelha o valor da opção `symbol`                                                                                                           |
| style\*   |           | Espelha o valor da opção `style`                                                                                                            |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

The `guix_shell` module shows the [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) environment. The module will be shown when inside a guix-shell environment.

### Opções

| Opções     | Padrão                     | Descrição                                              |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `'via [$symbol]($style) '` | O formato do módulo.                                   |
| `symbol`   | `'🐃 '`                     | A format string representing the symbol of guix-shell. |
| `style`    | `'yellow bold'`            | O estilo do módulo.                                    |
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
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🅶 '`                               | A format string representing the symbol of Gradle.                                  |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `['gradle']`                         | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold bright-cyan'`                 | O estilo do módulo.                                                                 |
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
| `format`            | `'via [$symbol($version )]($style)'`                                                            | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                                                                     | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | Quais pastas devem ativar este módulo.                                              |
| `symbol`            | `'⌘ '`                                                                                          | A format string representing the symbol of Haxe.                                    |
| `style`             | `'bold fg:202'`                                                                                 | O estilo do módulo.                                                                 |
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

| Opções            | Padrão                                 | Descrição                                                                                                                                      |
| ----------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`        | `true`                                 | Apenas exibe o hostname quando conectado em uma sessão SSH.                                                                                    |
| `ssh_symbol`      | `'🌐 '`                                 | Uma formatação de string que representa o símbolo quando conectado à sessão SSH.                                                               |
| `trim_at`         | `'.'`                                  | String na qual vai truncar o hostname, após a primeira correspondência. `'.'` will stop after the first dot. `''` will disable any truncation. |
| `detect_env_vars` | `[]`                                   | Which environment variable(s) should trigger this module.                                                                                      |
| `format`          | `'[$ssh_symbol$hostname]($style) in '` | O formato do módulo.                                                                                                                           |
| `style`           | `'bold dimmed green'`                  | O estilo do módulo.                                                                                                                            |
| `disabled`        | `false`                                | Desabilita o módulo `hostname`.                                                                                                                |
| `aliases`         | `{}`                                   | Translate system hostnames to something else. If `trim_at` is specified, only the first part will be matched and replaced.                     |

### Variáveis

| Variável   | Exemplo    | Descrição                                                      |
| ---------- | ---------- | -------------------------------------------------------------- |
| hostname   | `computer` | O hostname do computador                                       |
| style\*  |            | Espelha o valor da opção `style`                               |
| ssh_symbol | `'🌏 '`     | O símbolo a ser representado quando conectado à uma sessão SSH |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

O módulo `java` exibe o versão atual instalada do [Java](https://www.oracle.com/java/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot`, or `.sdkmanrc` file
- O diretório atual contenha arquivos com as extensões `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc`

### Opções

| Opções              | Padrão                                                                                                                | Descrição                                                                           |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                                                                                           | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                                                                                                  | Quais pastas devem ativar este módulo.                                              |
| `symbol`            | `'☕ '`                                                                                                                | Um formato de string que representa o simbolo do Java                               |
| `style`             | `'red dimmed'`                                                                                                        | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                                                                               | Desabilita o módulo `java`.                                                         |

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

> [!WARNING] This module is not supported on tcsh.

> [!WARNING] The `threshold` option is deprecated, but if you want to use it, the module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

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

Exibe o nome atual do [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) e, se definido, o namespace, usuário e cluster do arquivo kubeconfig. O namespace precisa ser definido no arquivo kubeconfig, isso pode ser feito via `kubectl config set-context starship-context --namespace astronaut`. Similarly, the user and cluster can be set with `kubectl config set-context starship-context --user starship-user` and `kubectl config set-context starship-context --cluster starship-cluster`. Se a env var `$KUBECONFIG` estiver definida o módulo vai usa-la ao invés de usar o `~/.kube/config`.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.
> 
> When the module is enabled it will always be active, unless any of `detect_env_vars`, `detect_extensions`, `detect_files` or `detect_folders` have been set in which case the module will only be active in directories that match those conditions or one of the environmental variables has been set.

### Opções

> [!WARNING] The `context_aliases` and `user_aliases` options are deprecated. Use `contexts` and the corresponding `context_alias` and `user_alias` options instead.

| Opções              | Padrão                                               | Descrição                                                     |
| ------------------- | ---------------------------------------------------- | ------------------------------------------------------------- |
| `symbol`            | `'☸ '`                                               | Uma string que representa o simbolo exibido antes do Cluster. |
| `format`            | `'[$symbol$context( \($namespace\))]($style) in '` | O formato do módulo.                                          |
| `style`             | `'cyan bold'`                                        | O estilo do módulo.                                           |
| `context_aliases`*  | `{}`                                                 | Tabela de aliases de contexto para exibir.                    |
| `user_aliases`*     | `{}`                                                 | Table of user aliases to display.                             |
| `detect_extensions` | `[]`                                                 | Quais extensões devem ativar este módulo.                     |
| `detect_files`      | `[]`                                                 | Quais nomes de arquivos devem ativar este módulo.             |
| `detect_folders`    | `[]`                                                 | Quais pastas devem ativar este módulo.                        |
| `detect_env_vars`   | `[]`                                                 | Which environmental variables should trigger this module      |
| `contexts`          | `[]`                                                 | Customized styles and symbols for specific contexts.          |
| `disabled`          | `true`                                               | Desabilita o módulo `kubernetes`.                             |

*: This option is deprecated, please add `contexts` with the corresponding `context_alias` and `user_alias` options instead.

To customize the style of the module for specific environments, use the following configuration as part of the `contexts` list:

| Variável          | Descrição                                                                                |
| ----------------- | ---------------------------------------------------------------------------------------- |
| `context_pattern` | **Required** Regular expression to match current Kubernetes context name.                |
| `user_pattern`    | Regular expression to match current Kubernetes user name.                                |
| `context_alias`   | Context alias to display instead of the full context name.                               |
| `user_alias`      | User alias to display instead of the full user name.                                     |
| `style`           | The style for the module when using this context. If not set, will use module's style.   |
| `symbol`          | The symbol for the module when using this context. If not set, will use module's symbol. |

Note that all regular expression are anchored with `^<pattern>$` and so must match the whole string. The `*_pattern` regular expressions may contain capture groups, which can be referenced in the corresponding alias via `$name` and `$N` (see example below and the [rust Regex::replace() documentation](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace)).

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

The `contexts` configuration option is used to customise what the current Kubernetes context name looks like (style and symbol) if the name matches the defined regular expression.

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

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

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

## Maven

The `maven` module indicates the presence of a Maven project in the current directory. If the [Maven Wrapper](https://maven.apache.org/wrapper/) is enabled, the Maven version will be parsed from `.mvn/wrapper/maven-wrapper.properties` and shown.

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual conter um arquivo `pom.xml`.
- The current directory contains a `.mvn/wrapper/maven-wrapper.properties` file.

If you use an alternate POM syntax (for example `pom.hocon`), add its filename to `detect_files`.

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'🅼 '`                               | A format string representing the symbol of Maven.                                   |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['pom.xml']`                        | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `['.mvn']`                           | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold bright-cyan'`                 | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Disables the `maven` module.                                                        |
| `recursive`         | `false`                              | Enables recursive finding for the `.mvn` directory.                                 |

### Variáveis

| Variável | Exemplo  | Descrição                         |
| -------- | -------- | --------------------------------- |
| version  | `v3.2.0` | The version of `maven`            |
| symbol   |          | Espelha o valor da opção `symbol` |
| style*   |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

## Uso de Memória

O módulo `memory_usage` mostra a memória atual do sistema e o uso de troca.

Por padrão o uso do swap é exibido se o total de swap do sistema é diferente de zero.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

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

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

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

## Mercurial State

The `hg_state` module will show in directories which are part of a mercurial repository, and where there is an operation in progress, such as: _REBASING_, _BISECTING_, etc.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções       | Padrão                      | Descrição                                                         |
| ------------ | --------------------------- | ----------------------------------------------------------------- |
| `merge`      | `'MERGING'`                 | O formato de string exibida quando um `merge` esta em progresso.  |
| `rebase`     | `'REBASING'`                | O formato de string exibida quando um `rebase` esta em progresso. |
| `update`     | `'UPDATING'`                | A format string displayed when a `update` is in progress.         |
| `bisect`     | `'BISECTING'`               | O formato de string exibida quando um `bisect` esta em progresso. |
| `shelve`     | `'SHELVING'`                | A format string displayed when a `shelve` is in progress.         |
| `graft`      | `'GRAFTING'`                | A format string displayed when a `graft` is in progress.          |
| `transplant` | `'TRANSPLANTING'`           | A format string displayed when a `transplant` is in progress.     |
| `histedit`   | `'HISTEDITING'`             | A format string displayed when a `histedit` is in progress.       |
| `style`      | `'bold yellow'`             | O estilo do módulo.                                               |
| `format`     | `'\([$state]($style)\) '` | O formato do módulo.                                              |
| `disabled`   | `true`                      | Disables the `hg_state` module.                                   |

### Variáveis

| Variável         | Exemplo    | Descrição                              |
| ---------------- | ---------- | -------------------------------------- |
| state            | `REBASING` | O estado atual do repo                 |
| progress_current | `1`        | O progresso da operação atual          |
| progress_total   | `2`        | O total do progresso da operação atual |
| style\*        |            | Espelha o valor da opção `style`       |

*: Esta variável só pode ser usada como parte de uma string de estilo

## Mise

The `mise` module shows the current mise health as reported by running `mise doctor`.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções              | Padrão                                                               | Descrição                                         |
| ------------------- | -------------------------------------------------------------------- | ------------------------------------------------- |
| `symbol`            | `'mise '`                                                            | The symbol used before displaying _mise_ health.  |
| `style`             | `'bold purple'`                                                      | O estilo do módulo.                               |
| `format`            | `'on [$symbol$health]($style) '`                                     | O formato do módulo.                              |
| `detect_extensions` | `[]`                                                                 | Quais extensões devem ativar este módulo.         |
| `detect_files`      | `['mise.toml', 'mise.local.toml', '.mise.toml', '.mise.local.toml']` | Quais nomes de arquivos devem ativar este módulo. |
| `detect_folders`    | `['.mise']`                                                          | Quais pastas devem ativar este módulo.            |
| `healthy_symbol`    | `healthy`                                                            | The message displayed when _mise_ is healthy.     |
| `unhealthy_symbol`  | `unhealthy`                                                          | The message displayed when _mise_ is unhealthy.   |
| `disabled`          | `true`                                                               | Disables the `mise` module.                       |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| health    | `healthy` | The health of _mise_              |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[mise]
health = 'ready'
```

## Mojo

The `mojo` module shows the current version of [Mojo programming language](https://www.modular.com/mojo) installed

### Opções

| Opções              | Padrão                                | Descrição                                              |
| ------------------- | ------------------------------------- | ------------------------------------------------------ |
| `format`            | `'with [$symbol($version )]($style)'` | O formato do módulo.                                   |
| `symbol`            | `'🔥 '`                                | The symbol used before displaying the version of Mojo. |
| `style`             | `'bold 208'`                          | O estilo do módulo.                                    |
| `disabled`          | `false`                               | Disables the `mojo` module.                            |
| `detect_extensions` | `['mojo', '🔥']`                       | Quais extensões devem ativar este módulo.              |
| `detect_files`      | `[]`                                  | Quais nomes de arquivos devem ativar este módulo.      |
| `detect_folders`    | `[]`                                  | Quais pastas devem ativar este módulo.                 |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `24.4.0` | The version of `mojo`             |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[mojo]
format = 'via [mojo ($version )($hash )]($style)'
```

## NATS

The `nats` module shows the name of the current [NATS](https://nats.io) context.

### Opções

| Opções     | Padrão                     | Descrição                                                    |
| ---------- | -------------------------- | ------------------------------------------------------------ |
| `symbol`   | `'✉️ '`                    | The symbol used before the NATS context (defaults to empty). |
| `style`    | `'bold purple'`            | O estilo do módulo.                                          |
| `format`   | `'[$symbol$name]($style)'` | O formato do módulo.                                         |
| `disabled` | `false`                    | Disables the `nats` module.                                  |

### Variáveis

| Variável  | Exemplo     | Descrição                         |
| --------- | ----------- | --------------------------------- |
| name      | `localhost` | The name of the NATS context      |
| symbol    |             | Espelha o valor da opção `symbol` |
| style\* |             | Espelha o valor da opção `style`  |

### Exemplo

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```

## Network Namespace

The `netns` module shows the current network namespace. This uses `ip netns identify` to get the network namespace, so only network namespaces mounted at `/var/run/netns` will be detected.

### Opções

| Opções     | Padrão                            | Descrição                                                         |
| ---------- | --------------------------------- | ----------------------------------------------------------------- |
| `format`   | `'[$symbol \[$name\]]($style)'` | O formato do módulo.                                              |
| `symbol`   | `'🛜 '`                            | The symbol used before the network namespace (defaults to empty). |
| `style`    | `'blue bold dimmed'`              | O estilo do módulo.                                               |
| `disabled` | `false`                           | Disables the `netns` module.                                      |

### Variáveis

| Variável  | Exemplo    | Descrição                                 |
| --------- | ---------- | ----------------------------------------- |
| name      | `my-netns` | The name of the current network namespace |
| symbol    |            | Espelha o valor da opção `symbol`         |
| style\* |            | Espelha o valor da opção `style`          |

### Exemplo

```toml
# ~/.config/starship.toml

[netns]
style = 'bold yellow'
symbol = '🌐 '
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

Additionally, the module will be hidden by default if the directory contains a `bunfig.toml`, `bun.lock`, or `bun.lockb` file, overriding the above conditions.

### Opções

| Opções              | Padrão                                        | Descrição                                                                                                |
| ------------------- | --------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | O formato do módulo.                                                                                     |
| `version_format`    | `'v${raw}'`                                   | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch`                      |
| `symbol`            | `' '`                                        | Uma string que representa o simbolo do Node.js.                                                          |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | Quais extensões devem ativar este módulo.                                                                |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | Quais nomes de arquivos devem ativar este módulo.                                                        |
| `detect_folders`    | `['node_modules']`                            | Quais pastas devem ativar este módulo.                                                                   |
| `style`             | `'bold green'`                                | O estilo do módulo.                                                                                      |
| `disabled`          | `false`                                       | Desabilita o módulo `nodejs`.                                                                            |
| `not_capable_style` | `'bold red'`                                  | O estilo para o módulo quando a propriedade engine no package.json não coincide com a versão do Node.js. |

### Variáveis

| Variável        | Exemplo       | Descrição                                                                                                                                                 |
| --------------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version         | `v13.12.0`    | A versão do `node`                                                                                                                                        |
| engines_version | `>=12.0.0` | `node` version requirement as set in the engines property of `package.json`. Will only show if the version requirement does not match the `node` version. |
| symbol          |               | Espelha o valor da opção `symbol`                                                                                                                         |
| style\*       |               | Espelha o valor da opção `style`                                                                                                                          |

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

## Odin

The `odin` module shows the currently installed version of [Odin](https://odin-lang.org/). By default the module will be shown if the current directory contains a `.odin` file.

### Opções

| Opções              | Padrão                               | Descrição                                              |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                   |
| `show_commit`       | `false`                              | Shows the commit as part of the version.               |
| `symbol`            | `'Ø '`                               | The symbol used before displaying the version of Odin. |
| `style`             | `'bold bright-blue'`                 | O estilo do módulo.                                    |
| `disabled`          | `false`                              | Disables the `odin` module.                            |
| `detect_extensions` | `['odin']`                           | Quais extensões devem ativar este módulo.              |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.      |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                 |

### Variáveis

| Variável  | Exemplo       | Descrição                         |
| --------- | ------------- | --------------------------------- |
| version   | `dev-2024-03` | The version of `odin`             |
| symbol    |               | Espelha o valor da opção `symbol` |
| style\* |               | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[odin]
format = 'via [󰹩 ($version )]($style)'
show_commit = true
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

> [!WARNING] The [os_info](https://lib.rs/crates/os_info) crate used by this module is known to be inaccurate on some systems.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções     | Padrão                | Descrição                                              |
| ---------- | --------------------- | ------------------------------------------------------ |
| `format`   | `'[$symbol]($style)'` | O formato do módulo.                                   |
| `style`    | `'bold white'`        | O estilo do módulo.                                    |
| `disabled` | `true`                | Disables the `os` module.                              |
| `symbols`  |                       | A table that maps each operating system to its symbol. |

`symbols` allows you to define arbitrary symbols to display for each operating system type. Operating system types not defined by your configuration use the default symbols table below. All operating systems currently supported by the module are listed below. If you would like an operating system to be added, feel free to open a [feature request](https://github.com/starship/starship/issues/new/choose).

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

| Variável  | Exemplo      | Descrição                                                          |
| --------- | ------------ | ------------------------------------------------------------------ |
| symbol    | `🎗️`         | The current operating system symbol from advanced option `symbols` |
| name      | `Arch Linux` | The current operating system name                                  |
| tipo      | `Arch`       | The current operating system type                                  |
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

O módulo `package` é mostrado quando o diretório atual é o repositório de um pacote e mostra sua versão atual. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `python`, `composer`, `gradle`, `julia`, `mix`, `helm`, `shards`, `galaxy`, `daml` and `dart` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – O versionamento de pacotes `npm` é extraído do `package.json` presente no diretório atual
- [**JSR**](https://jsr.io/) – The `jsr` package version is extracted from the `jsr.json`/`jsr.jsonc` or `deno.json`/`deno.jsonc` present in the current directory
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
- [**Galaxy**](https://galaxy.ansible.com/) - The `galaxy` package version is extracted from the `galaxy.yml` present in the current directory
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

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções              | Padrão                            | Descrição                                                                            |
| ------------------- | --------------------------------- | ------------------------------------------------------------------------------------ |
| `symbol`            | `' '`                            | The symbol used before the pijul channel name of the repo in your current directory. |
| `style`             | `'bold purple'`                   | O estilo do módulo.                                                                  |
| `format`            | `'on [$symbol$channel]($style) '` | O formato do módulo.                                                                 |
| `truncation_length` | `2^63 - 1`                        | Truncates the pijul channel name to `N` graphemes                                    |
| `truncation_symbol` | `'…'`                             | O simbolo usado para indicar que o nome braço foi truncado.                          |
| `disabled`          | `true`                            | Disables the `pijul` module.                                                         |

## Pixi

The `pixi` module shows the installed [pixi](https://pixi.sh) version as well as the activated environment, if `$PIXI_ENVIRONMENT_NAME` is set.

> [!TIP] This does not suppress pixi's own prompt modifier, you may want to run `pixi config set shell.change-ps1 false`.

### Opções

| Opções                     | Padrão                                                    | Descrição                                                                         |
| -------------------------- | --------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`                   | `'via [$symbol($version )(\($environment\) )]($style)'` | O formato do módulo.                                                              |
| `version_format`           | `'v${raw}'`                                               | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch`.        |
| `symbol`                   | `'🧚 '`                                                    | O simbolo usado antes do nome do environment.                                     |
| `style`                    | `'yellow bold'`                                           | O estilo do módulo.                                                               |
| `show_default_environment` | `true`                                                    | Whether to indicate that the `default` environment of your project is activated.  |
| `pixi_binary`              | `['pixi']`                                                | Configures the pixi binary that Starship should execute when getting the version. |
| `detect_extensions`        | `[]`                                                      | Quais extensões devem ativar este módulo.                                         |
| `detect_files`             | `['pixi.toml']`                                           | Quais nomes de arquivos devem ativar este módulo.                                 |
| `detect_folders`           | `[]`                                                      | Quais pastas devem ativar este módulo.                                            |
| `disabled`                 | `false`                                                   | Disables the `pixi` module.                                                       |

### Variáveis

| Variável    | Exemplo   | Descrição                         |
| ----------- | --------- | --------------------------------- |
| version     | `v0.33.0` | The version of `pixi`             |
| environment | `py311`   | The current pixi environment      |
| symbol      |           | Espelha o valor da opção `symbol` |
| style       |           | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml

[pixi]
format = '[$symbol$environment](yellow) '
```

## Pulumi

O módulo `pulumi` mostra o nome de usuário atual, a [Pulumi Stack](https://www.pulumi.com/docs/intro/concepts/stack/) selcionada e a versão.

> [!TIP] By default the Pulumi version is not shown, since it takes an order of magnitude longer to load then most plugins (~70ms). Se você ainda deseja ativa-la, [siga o exemplo abaixo](#with-pulumi-version).

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

| Variável  | Exemplo    | Descrição                         |
| --------- | ---------- | --------------------------------- |
| version   | `v0.12.24` | A versão do `pulumi`              |
| stack     | `dev`      | A stack Pulumi atual              |
| username  | `alice`    | O nome de usuário Pulumi atual    |
| symbol    |            | Espelha o valor da opção `symbol` |
| style\* |            | Espelha o valor da opção `style`  |

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
- O diretório atual conter um arquivo `spago.yaml`
- O diretório atual conter um arquivo `spago.lock`
- O diretório atual tenha um arquivo com a extensão `.purs`

### Opções

| Opções              | Padrão                                        | Descrição                                                                           |
| ------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                                   | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'<=> '`                                | O símbolo usado antes de exibir a versão do PureScript.                             |
| `detect_extensions` | `['purs']`                                    | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['spago.dhall', 'spago.yaml', 'spago.lock']` | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                          | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold white'`                                | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                       | Desabilita o módulo `purescript`.                                                   |

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

Por padrão, o módulo será exibido se qualquer das seguintes condições for atendida:

- O diretório atual tenha um arquivo `.python-version`
- O diretório atual tenha um arquivo `Pipfile`
- O diretório atual tenha um arquivo `__init__.py`
- O diretório atual conter um arquivo `pyproject.toml`
- O diretório atual conter um arquivo `requirements.txt`
- O diretório atual conter um arquivo `setup.py`
- O diretório atual conter um arquivo `tox.ini`
- O diretório atual tenha um arquivo com a extensão `.py`.
- The current directory contains a file with the `.ipynb` extension.
- Um ambiente virtual está atualmente ativo

### Opções

| Opções               | Padrão                                                                                                       | Descrição                                                                             |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | O formato do módulo.                                                                  |
| `version_format`     | `'v${raw}'`                                                                                                  | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch`   |
| `symbol`             | `'🐍 '`                                                                                                       | Uma string que representa o simbolo do Python                                         |
| `style`              | `'yellow bold'`                                                                                              | O estilo do módulo.                                                                   |
| `pyenv_version_name` | `false`                                                                                                      | Usa pyenv para pegar a versão do Python                                               |
| `pyenv_prefix`       | `'pyenv'`                                                                                                    | Prefixo antes da versão do pyenv, apenas usado se pyenv for usado                     |
| `python_binary`      | `['python', 'python3', 'python2']`                                                                           | Configures the python binaries that Starship should execute when getting the version. |
| `detect_extensions`  | `['py', 'ipynb']`                                                                                            | Quais extensões devem acionar este módulo                                             |
| `detect_files`       | `['.python-version', 'Pipfile', '__init__.py', 'pyproject.toml', 'requirements.txt', 'setup.py', 'tox.ini']` | []                                                                                    |
| `detect_folders`     | `[]`                                                                                                         | Quais pastas devem ativar este módulo                                                 |
| `generic_venv_names` | `[]`                                                                                                         | Which venv names should be replaced with the parent directory name.                   |
| `disabled`           | `false`                                                                                                      | Desabilita o módulo `python`.                                                         |

> [!TIP] The `python_binary` variable accepts either a string or a list of strings. O Starship vai tentar executar cada binário até obter um resultado. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.
> 
> The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `'python3'` will hide any Python version 2, see example below.

### Variáveis

| Variável     | Exemplo         | Descrição                                                                   |
| ------------ | --------------- | --------------------------------------------------------------------------- |
| version      | `'v3.8.1'`      | A versão do `python`                                                        |
| symbol       | `'🐍 '`          | Espelha o valor da opção `symbol`                                           |
| style        | `'yellow bold'` | Espelha o valor da opção `style`                                            |
| pyenv_prefix | `'pyenv '`      | Espelha o valor da opção `pyenv_prefix`                                     |
| virtualenv   | `'venv'`        | The current `virtualenv` name or the parent if matches `generic_venv_names` |

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

## Quarto

The `quarto` module shows the current installed version of Quarto used in a project.

Por padrão, o módulo será exibido se qualquer das seguintes condições for atendida:

- The current directory contains a `_quarto.yml` file
- The current directory contains any `*.qmd` file

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'⨁ '`                               | A format string representing the symbol of Quarto                                   |
| `style`             | `'bold #75AADB'`                     | O estilo do módulo.                                                                 |
| `detect_extensions` | `['.qmd']`                           | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['_quarto.yml']`                    | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `disabled`          | `false`                              | Disables the `quarto` module.                                                       |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `1.4.549` | The version of `quarto`           |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

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

| Variável  | Exemplo  | Descrição                                   |
| --------- | -------- | ------------------------------------------- |
| version   | `v2.5.1` | A versão do `ruby`                          |
| symbol    |          | Espelha o valor da opção `symbol`           |
| style\* |          | Espelha o valor da opção `style`            |
| gemset    | `test`   | Optional, gets the current RVM gemset name. |

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

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções                 | Padrão                    | Descrição                                                                                              |
| ---------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------ |
| `bash_indicator`       | `'bsh'`                   | Uma string para representar o bash.                                                                    |
| `fish_indicator`       | `'fsh'`                   | Uma string usada para representar o fish.                                                              |
| `zsh_indicator`        | `'zsh'`                   | Uma string usada para representar o zsh.                                                               |
| `powershell_indicator` | `'psh'`                   | Uma string usada para representar o powershell.                                                        |
| `pwsh_indicator`       |                           | A format string used to represent pwsh. The default value mirrors the value of `powershell_indicator`. |
| `ion_indicator`        | `'ion'`                   | Uma string usada para representar o ion.                                                               |
| `elvish_indicator`     | `'esh'`                   | Uma string usada para representar o elvish.                                                            |
| `tcsh_indicator`       | `'tsh'`                   | Uma string usada para representar o tcsh.                                                              |
| `xonsh_indicator`      | `'xsh'`                   | Uma string usada para representar o xonsh.                                                             |
| `cmd_indicator`        | `'cmd'`                   | Uma string usada para representar o cmd.                                                               |
| `nu_indicator`         | `'nu'`                    | Uma string usada para representar o nu.                                                                |
| `unknown_indicator`    | `''`                      | Valor padrão para exibir quando o shell é desconhecido.                                                |
| `format`               | `'[$indicator]($style) '` | O formato do módulo.                                                                                   |
| `style`                | `'white bold'`            | O estilo do módulo.                                                                                    |
| `disabled`             | `true`                    | Desabilita o módulo `shell`.                                                                           |

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
fish_indicator = '󰈺 '
powershell_indicator = '_'
unknown_indicator = 'mystery shell'
style = 'cyan bold'
disabled = false
```

## SHLVL

The `shlvl` module shows the current [`SHLVL`](https://tldp.org/LDP/abs/html/internalvariables.html#SHLVLREF) ('shell level') environment variable, if it is set to a number and meets or exceeds the specified threshold.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções          | Padrão                       | Descrição                                                           |
| --------------- | ---------------------------- | ------------------------------------------------------------------- |
| `threshold`     | `2`                          | Limite de exibição.                                                 |
| `format`        | `'[$symbol$shlvl]($style) '` | O formato do módulo.                                                |
| `symbol`        | `'↕️  '`                     | O simbolo usado para representar o `SHLVL`.                         |
| `repeat`        | `false`                      | Caso o `symbol` deva se repetir de acordo com o total do `SHLVL`.   |
| `repeat_offset` | `0`                          | Decrements number of times `symbol` is repeated by the offset value |
| `style`         | `'bold yellow'`              | O estilo do módulo.                                                 |
| `disabled`      | `true`                       | Desabilita o módulo `shlvl`.                                        |

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

Using `repeat` and `repeat_offset` along with `character` module, one can get prompt like `❯❯❯` where last character is colored appropriately for return status code and preceding characters are provided by `shlvl`.

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

## Solidity

The `solidity` module shows the currently installed version of [Solidity](https://soliditylang.org/) The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.sol` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${major}.${minor}.${patch}'`      | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'S '`                               | A format string representing the symbol of Solidity                                 |
| `compiler          | ['solc']                             | The default compiler for Solidity.                                                  |
| `detect_extensions` | `['sol']`                            | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold blue'`                        | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Disables this module.                                                               |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v0.8.1` | The version of `solidity`         |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml
[solidity]
format = "via [S $version](blue bold)"
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

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

### Opções

| Opções                      | Padrão                                                                              | Descrição                                                                               |
| --------------------------- | ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `format`                    | `'[$symbol$status]($style) '`                                                       | O formato do módulo                                                                     |
| `symbol`                    | `'❌'`                                                                               | O simbolo exibido no erro de programa                                                   |
| `success_symbol`            | `''`                                                                                | O simbolo exibido no sucesso de programa                                                |
| `not_executable_symbol`     | `'🚫'`                                                                               | O simbolo exibido quando o arquivo não é executável                                     |
| `not_found_symbol`          | `'🔍'`                                                                               | O simbolo exibido quando o comando não é encontrado                                     |
| `sigint_symbol`             | `'🧱'`                                                                               | O simbolo exibido no SIGINT (Ctrl + c)                                                  |
| `signal_symbol`             | `'⚡'`                                                                               | O simbolo exibido em qualquer sinal                                                     |
| `style`                     | `'bold red'`                                                                        | O estilo do módulo.                                                                     |
| `success_style`             |                                                                                     | The style used on program success (defaults to `style` if unset).                       |
| `failure_style`             |                                                                                     | The style used on program failure (defaults to `style` if unset).                       |
| `recognize_signal_code`     | `true`                                                                              | Habilita o mapeamento de sinais para códigos de saída                                   |
| `map_symbol`                | `false`                                                                             | Habilita o mapeamento de símbolos para códigos de saída                                 |
| `pipestatus`                | `false`                                                                             | Habilita o relatório de pipestatus                                                      |
| `pipestatus_separator`      | <code>&vert;</code>                                                           | O símbolo usado para separar segmentos de pipestatus (suporta formatação)               |
| `pipestatus_format`         | `'\[$pipestatus\] => [$symbol$common_meaning$signal_name$maybe_int]($style) '` | O formato do módulo quando o comando é um pipeline                                      |
| `pipestatus_segment_format` |                                                                                     | Quando especificado, substitui o  `format` quando ha formatação de segmentos pipestatus |
| `disabled`                  | `true`                                                                              | Desabilita o módulo `status`.                                                           |

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
| style\*      |         | Mirrors the value of option `success_style` on program success and `failure_style` otherwise             |

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

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

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

O módulo `terraform` exibe o [Terraform workspace](https://www.terraform.io/docs/language/state/workspaces.html) selecionado e sua versão. It supports both Hashicorp Terraform and OpenTofu for version detection.

> [!TIP] By default the Terraform/OpenTofu version is not shown, since this is slow for current versions when a lot of plugins are in use. Se você deseja habilitar,, [Siga o exemplo abaixo](#with-terraform-version).

Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual tenha uma pasta `.terraform`
- O diretório atual tenha arquivos com as extensões `.tf`, `.tfplan` or `.tfstate`

### Opções

| Opções              | Padrão                                                  | Descrição                                                                           |
| ------------------- | ------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$workspace]($style) '`                    | A string de formato do módulo.                                                      |
| `version_format`    | `'v${raw}'`                                             | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'💠'`                                                   | Uma string que é exibida antes do workspace terraform.                              |
| `detect_extensions` | `['tf', 'tfplan', 'tfstate']`                           | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `[]`                                                    | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `['.terraform']`                                        | Quais pastas devem ativar este módulo.                                              |
| `style`             | `'bold 105'`                                            | O estilo do módulo.                                                                 |
| `disabled`          | `false`                                                 | Desabilita o módulo `terraform`.                                                    |
| `commands`          | `[ [ 'terraform', 'version' ], [ 'tofu', 'version' ] ]` | How to detect what the Terraform version is.                                        |

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
format = 'via [$symbol$version $workspace]($style) '
```

#### Sem a versão do Terraform

```toml
# ~/.config/starship.toml

[terraform]
format = 'via [$symbol$workspace]($style) '
```

## Horário

O módulo `time` exibe a hora **local** atual. A configuração de `format` é usada pelo [`chrono`](https://crates.io/crates/chrono) para controlar qual hora é exibida. Dê uma olhada na [documentação do chrono strftime](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) para ver quais opções estão disponíveis.

> [!TIP] This module is disabled by default. Para habilitar, defina `disabled` para `false` no seu arquivo de configuração.

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

## Typst

The `typst` module shows the current installed version of Typst used in a project.

Por padrão, o módulo será exibido se qualquer das seguintes condições for atendida:

- O diretório atual conter um arquivo `template.typ`
- The current directory contains any `*.typ` file

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'t '`                               | A format string representing the symbol of Typst                                    |
| `style`             | `'bold #0093A7'`                     | O estilo do módulo.                                                                 |
| `detect_extensions` | `['.typ']`                           | Quais extensões devem ativar este módulo.                                           |
| `detect_files`      | `['template.typ']`                   | Quais nomes de arquivos devem ativar este módulo.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                              |
| `disabled`          | `false`                              | Disables the `typst` module.                                                        |

### Variáveis

| Variável      | Exemplo   | Descrição                                       |
| ------------- | --------- | ----------------------------------------------- |
| version       | `v0.9.0`  | The version of `typst`, alias for typst_version |
| typst_version | `default` | The current Typst version                       |
| symbol        |           | Espelha o valor da opção `symbol`               |
| style\*     |           | Espelha o valor da opção `style`                |

*: Esta variável só pode ser usada como parte de uma string de estilo

## Nome do usuário

O módulo `username` mostra o nome de usuário do usuário ativo. O módulo será mostrado se alguma das seguintes condições for atendida:

- O usuário atual é root/admin
- O usuário atual não é o mesmo que está logado
- O usuário atual esta conectado em uma sessão SSH
- A variável `show_always` esta definida como true
- The array `detect_env_vars` contains at least the name of one environment variable, that is set

> [!TIP] SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

### Opções

| Opções            | Padrão                  | Descrição                                                 |
| ----------------- | ----------------------- | --------------------------------------------------------- |
| `style_root`      | `'bold red'`            | O estilo usado quando o usuário é root/admin.             |
| `style_user`      | `'bold yellow'`         | O estilo usado para usuários não root.                    |
| `detect_env_vars` | `[]`                    | Which environment variable(s) should trigger this module. |
| `format`          | `'[$user]($style) in '` | O formato do módulo.                                      |
| `show_always`     | `false`                 | Sempre exibe o módulo `username`.                         |
| `disabled`        | `false`                 | Desabilita o módulo `username`.                           |
| `aliases`         | `{}`                    | Translate system usernames to something else.             |

### Variáveis

| Variável | Exemplo      | Descrição                                                                              |
| -------- | ------------ | -------------------------------------------------------------------------------------- |
| `style`  | `'red bold'` | Espelha o valor da opção `style_root` quando o root esta logado e `style_user` se não. |
| `user`   | `'fulano'`   | O ID do usuário logado atualmente.                                                     |

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

## VCS

> Note the module is enabled by default but **not** included in the default list because that would be a breaking change. Additionally, the exact format of the module may change in the future, for example to handle right-aligned prompt.

The `vcs` module displays the current active Version Control System (VCS). The module will be shown only if a configured VCS is currently in use.

### Opções

| Opções           | Padrão                                                      | Descrição                                             |
| ---------------- | ----------------------------------------------------------- | ----------------------------------------------------- |
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

## XMake

The `xmake` module shows the currently installed version of [XMake](https://xmake.io/). Por padrão o módulo será ativo se qualquer das condições a seguir for atendida:

- O diretório atual conter um arquivo `xmake.lua`

### Opções

| Opções              | Padrão                               | Descrição                                                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | O formato do módulo.                                                                |
| `version_format`    | `'v${raw}'`                          | A versão formatada. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `'△ '`                               | O simbolo usado antes da versão do cmake.                                           |
| `detect_extensions` | `[]`                                 | Quais extensões devem acionar este módulo                                           |
| `detect_files`      | `['xmake.lua']`                      | []                                                                                  |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo                                               |
| `style`             | `'bold green'`                       | O estilo do módulo.                                                                 |
| `disabled`          | `false`                              | Disables the `xmake` module.                                                        |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.9.5` | The version of xmake              |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

*: Esta variável só pode ser usada como parte de uma string de estilo

## Zig

By default the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). O módulo será mostrado se alguma das seguintes condições for atendida:

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

> [!TIP] Multiple custom modules can be defined by using a `.`.

> [!TIP] The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). Por padrão, o módulo `custom` mostrará todos os módulos personalizados na ordem em que eles foram definidos.

> [!TIP] [Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. Se você tem um exemplo interessante que não esta coberto lá, sinta-se livre para compartilha-lo!

> [!WARNING] If `unsafe_no_escape` is enabled or prior to starship v1.20 command output is printed unescaped to the prompt.
> 
> Seja qual for a saída o comando irá gerar uma saída sem modificações no prompt. This means if the output contains shell-specific interpretable sequences, they could be interpreted on display. Depending on the shell, this can mean that e.g. strings enclosed by backticks are executed by the shell. Such sequences are usually shell specific, e.g. you can write a command module that writes bash sequences, e.g. `\h`, but this module will not work in a fish or zsh shell.
> 
> Format strings can also contain shell specific prompt sequences, e.g. [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html).

### Opções

| Opções              | Padrão                          | Descrição                                                                                                                                                                                                                                                                                                                        |
| ------------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | O comando cuja a saída deve ser exibida. O comando será passado no stdin para o shell.                                                                                                                                                                                                                                           |
| `when`              | `false`                         | Ou um valor booleano (`true` ou `false`, sem aspas) ou um comando shell de string usado como condição para mostrar o módulo. In case of a string, the module will be shown if the `shell` returns a `0` status code from executing it.                                                                                           |
| `require_repo`      | `false`                         | Se `true`, o módulo será exibido apenas em caminhos que contenham um repositório (git). Esta opção, por si só, não é uma condição de exibição suficiente na ausência de outras opções.                                                                                                                                           |
| `shell`             |                                 | [Veja abaixo](#custom-command-shell)                                                                                                                                                                                                                                                                                             |
| `description`       | `'<custom module>'`       | A descrição do módulo, isto será exibido quando executar `starship explain`.                                                                                                                                                                                                                                                     |
| `unsafe_no_escape`  | `false`                         | When set, command output is not escaped of characters that could be interpreted by the shell.                                                                                                                                                                                                                                    |
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

| Variável  | Descrição                              |
| --------- | -------------------------------------- |
| output    | The output of `command` run in `shell` |
| symbol    | Espelha o valor da opção `symbol`      |
| style\* | Espelha o valor da opção `style`       |

*: Esta variável só pode ser usada como parte de uma string de estilo

#### Comandos personalizados de shell

`shell` aceita uma lista não vazia de string, onde:

- A primeira string é o caminho para o shell que executará o comando.
- Outros argumentos que serão passados para o shell.

If unset, it will fallback to STARSHIP_SHELL and then to 'sh' on Linux, and 'cmd /C' on Windows.

The `command` (and `when`, if applicable) will be passed in on stdin.

Se o `shell` não for dado ou apenas conter um elemento e o Starship detectar PowerShell ele será usado, os seguintes argumentos serão automaticamente adicionados: `-NoProfile -Command -`. Se `shell` não for fornecido ou contiver apenas um elemento e o Starship detectar que o Cmd será usado, o seguinte argumento será adicionado automaticamente: `/C` e `stdin` serão definidos como `false`. Se `shell` não for fornecido ou contiver apenas um elemento e o Starship detectar que o Nushell será usado, os seguintes argumentos serão adicionados automaticamente: `-c` e `stdin` serão definidos como `false`. Este comportamento pode ser evitado passando explicitamente argumento para o shell, ex.

```toml
shell = ['pwsh', '-Command', '-']
```

> [!WARNING] Make sure your custom shell configuration exits gracefully
> 
> If you set a custom command, make sure that the default Shell used by starship will properly execute the command with a graceful exit (via the `shell` option).
> 
> For example, PowerShell requires the `-Command` parameter to execute a one liner. Omitting this parameter might throw starship into a recursive loop where the shell might try to load a full profile environment with starship itself again and hence re-execute the custom command, getting into a never ending loop.
> 
> Parameters similar to `-NoProfile` in PowerShell are recommended for other shells as well to avoid extra loading time of a custom profile on every starship invocation.
> 
> Automatic detection of shells and proper parameters addition are currently implemented, but it's possible that not all shells are covered. [Please open an issue](https://github.com/starship/starship/issues/new/choose) with shell details and starship configuration if you hit such scenario.

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
