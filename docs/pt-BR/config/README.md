# Configuração

Para começar a configurar a starship, crie o seguinte arquivo: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Todas as configurações do starship são feitas neste arquivo [TOML](https://github.com/toml-lang/toml):

```toml
# Insere uma quebra de linha entre os prompts do shell
add_newline = true

# Substitui o símbolo "❯" no prompt por "➜"
[character] # O nome do módulo que estamos configurando é "character"
success_symbol = "[➜](bold green)" # O segmento "success_symbol" está sendo definido como "➜" com a cor "verde em negrito"

# Desabilita o módulo do pacote, ocultando-o completamente do prompt
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

Ou para Cmd (Windows) seria adicionar esta linha no seu `starship.lua`:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\\Users\\user\\example\\non\\default\\path\\starship.toml')
```

### Logging

Por padrão, o starship registra avisos e erros em um arquivo chamado `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`, onde a chave de sessão corresponde a uma instância do seu terminal. Isso, no entanto, pode ser alterado usando a variável de ambiente `STARSHIP_CACHE`:

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

**Módulo**: Um componente no prompt que fornece informações com base em informações contextuais do seu sistema operacional. Por exemplo, o módulo "nodejs" mostra a versão do Node.js que está atualmente instalada em seu computador, se seu diretório atual for um projeto Node.js.

**Variável**: Subcomponentes menores que contêm informações fornecidas pelo módulo. Por exemplo, a variável "version" no módulo "nodejs" contém a versão atual do Node.js.

Por convenção, a maioria dos módulos tem um prefixo de cor de terminal padrão (ex., `via` em "nodejs") e um espaço vazio como sufixo.

### Formatação de Strings

As strings de formato são o formato com o qual um módulo imprime todas as suas variáveis. A maioria dos módulos tem uma entrada chamada `format` que configura o formato de exibição do módulo. Você pode usar textos, variáveis e grupos de texto em uma string de formato.

#### Variável

Uma variável contém um símbolo `$` seguido pelo nome da variável. O nome de uma variável pode conter apenas letras, números e `_`.

Por exemplo:

- `$version` é uma formatação de string com uma variável chamada `version`.
- `$git_branch$git_commit` é uma formatação de string com duas variáveis chamadas `git_branch` e `git_commit`.
- `$git_branch $git_commit` Tem as duas variáveis separadas por espaço.

#### Grupo de Texto

Um grupo de texto é composto de duas partes diferentes.

A primeira parte, que está entre um `[]`, é uma [string de formato](#format-strings). Você pode adicionar textos, variáveis ou até mesmo grupos de texto aninhados nele.

Na segunda parte, que está dentro de um `()`, está uma [string de estilo](#style-strings). Isso pode ser usado para estilizar a primeira parte.

Por exemplo:

- `[on](red bold)` vai imprimir uma string `on` com texto em negrito e com a cor vermelha.
- `[⌘ $version](bold green)` vai imprimir o simbolo `⌘` seguido pela variável `version`, com o texto em negrito e na cor verde.
- `[a [b](red) c](green)` vai imprimir `a b c` com `b` vermelho, e `a` e `c` verde.

#### Estilo dos textos

A maioria dos módulos no starship permite que você configure seus estilos de exibição. Isso é feito com uma entrada (normalmente chamada de `estilo`) que é uma string especificando a configuração. Aqui estão alguns exemplos de strings de estilo junto com o que elas fazem. Para obter detalhes sobre a sintaxe completa, consulte o [guia de configuração avançada](/advanced-config/).

- `"fg:green bg:blue"` deixa o texto verde com o fundo azul
- `"bg:blue fg:bright-green"` deixa o texto verde brilhante com o fundo azul
- `"bold fg:27"` deixa o texto em negrito com a cor 27 [da tabela ANSI](https://i.stack.imgur.com/KTSQa.png)
- `"underline bg:#bf5700"` deixa o texto sublinhado com o fundo laranja escuro
- `"bold italic fg:purple"` deixa o texto em negrito e itálico com a cor roxa
- `""` desabilita explicitamente todos os estilos

Observe que a aparência do estilo será controlada pelo emulador de terminal. Por exemplo, alguns emuladores de terminal irão clarear as cores em vez de colocar o texto em negrito, e alguns temas de cores usam os mesmos valores para as cores normais e brilhantes. Além disso, para obter texto em itálico, seu terminal deve suportar itálico.

#### Formatação de String Condicional

Uma string de formato condicional envolta de `(` e `)` não será renderizada se todas as variáveis internas estiverem vazias.

Por exemplo:

- `(@$region)` não vai exibir nada caso a variável `region` seja `None` ou vazia, caso contrario vai exibir `@` seguido pelo valor da variável region.
- `(texto qualquer)` não vai exibir nada sempre, pois não existe variável entre os parenteses.
- Quando usar `$all` é um atalho para `\[$a$b\]`, `($all)` vai exibir nada somente quando `$a` e `$b` são `None`. Isto funciona da mesma forma que `(\[$a$b\] )`.

#### Caracteres Especiais

Os seguintes símbolos têm uso especial em uma string de formato e devem ser evitados: `$ \ [ ] ( )`.

Observe que o TOML tem [strings básicas e strings literais](https://toml.io/en/v1.0.0#string). É recomendado usar uma string literal (entre aspas simples) em seu config. Se você quiser usar uma string básica (entre aspas duplas), você deve escapar da própria barra invertida (ou seja, use `\\`).

Por exemplo, quando você deseja imprimir um símbolo `$` em uma nova linha, as seguintes configurações para `format` são equivalentes:

```toml
# com string básica
format = "\n\\$"

# com string básica multilinha
format = """

\\$"""

# com string literal
format = '''

\$'''
```

## Prompt de Comando

Esta é a lista de opções de configuração em todo o prompt.

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

O padrão `format` é usado para definir o formato do prompt, se estiver vazio ou nenhum `format` for fornecido. O padrão é como mostrado:

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

Se você quiser apenas estender o formato padrão, você pode usar `$all`; os módulos que você adicionar explicitamente ao formato não serão duplicados. Ex.

```toml
# Move o diretório para a segunda linha
format = "$all$directory$character"
```

## AWS

The `aws` module shows the current AWS region and profile when credentials or a `credential_process` have been setup. Isto é baseado nas variáveis de env `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` contidas no arquivo `~/.aws/config`. Este modulo exibi também tempo de expiração de credenciais temporarias.

The module will display a profile only if its credentials are present in `~/.aws/credentials` or a `credential_process` is defined in `~/.aws/config`. Alternatively, having any of the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, or `AWS_SESSION_TOKEN` env vars defined will also suffice.

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
| `disabled`          | `false`                                                              | Desabilita o módulo `AWS`.                                           |

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
format = 'em [$symbol($profile )(\($region\) )]($style)'
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
format = "em [$symbol$region]($style) "
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
format = "em [$symbol$profile]($style) "
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
format = "em [$symbol($subscription)]($style) "
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

The `display` configuration option is used to define when the battery indicator should be shown (threshold), which symbol would be used (symbol), and what it would like (style). If no `display` is provided. O padrão é como mostrado:

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
format = "levou [$duration](bold yellow)"
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
| `disabled` | `false`                              | Desabilita o módulo `container`.          |

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
| `symbol`            | `"🔮 "`                               | O símbolo usado antes de exibir a versão do crystal.                                 |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                                  |
| `detect_extensions` | `["cr"]`                             | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["shard.yml"]`                      | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `disabled`          | `false`                              | Desabilita o módulo `crystal`.                                                       |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v0.32.1` | A versão do `crystal`              |
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
| `symbol`            | `"🎯 "`                                            | Um formato de string que representa o simbolo do Dart                                |
| `detect_extensions` | `["dart"]`                                        | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["pubspec.yaml", "pubspec.yml", "pubspec.lock"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[".dart_tool"]`                                  | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold blue"`                                     | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                           | Desabilita o módulo `dart`.                                                          |

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
| `symbol`            | `"🦕 "`                                                                  | Um formato de string que representa o simbolo do Deno                                |
| `detect_extensions` | `[]`                                                                    | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["deno.json", "deno.jsonc", "mod.ts", "mod.js", "deps.ts", "deps.js"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                                    | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"green bold"`                                                          | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                 | Desabilita o módulo `deno`.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v1.8.3` | A versão do `deno`                 |
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

| Opções              | Padrão                                             | Descrição                                                                                   |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| `truncation_length` | `3`                                                | O número de pastas pais do diretório atual que serão truncadas.                             |
| `truncate_to_repo`  | `true`                                             | Seu diretório será truncado ou não para a raiz do repositório git atual.                    |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | O formato do módulo.                                                                        |
| `style`             | `"bold cyan"`                                      | O estilo do módulo.                                                                         |
| `disabled`          | `false`                                            | Desabilita o módulo `directory`.                                                            |
| `read_only`         | `"🔒"`                                              | O simbolo que indica que o diretório atual é somente leitura.                               |
| `read_only_style`   | `"red"`                                            | O estilo para o simbolo de somente leitura.                                                 |
| `truncation_symbol` | `""`                                               | O simbolo para prefixo de caminhos truncados. ex: "…/"                                      |
| `repo_root_style`   | `None`                                             | O estilo para o repositório git root quando `truncate_to_repo` estiver definido como false. |
| `home_symbol`       | `"~"`                                              | O simbolo para indicar o diretório home.                                                    |
| `use_os_path_sep`   | `true`                                             | Use the OS specific path separator instead of always using `/` (e.g. `\` on Windows)     |

<details>
<summary>This module has a few advanced configuration options that control how the directory is displayed.</summary>

| Opções Avançadas            | Padrão | Descrição                                                                                                                                                             |
| --------------------------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substituições`             |        | Uma tabela de substituições para fazer no path.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`    | O número de caracteres para usar quando aplicado no path logico do fish shell pwd.                                                                                    |
| `use_logical_path`          | `true` | Se `true` exibe um caminho lógico originado do shell via `PWD` ou`--logical-path`. Se `false` em vez disso, exibe o caminho do filesystem com os symlinks resolvidos. |

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
| path      | `"D:/Projetos"`       | O caminho do diretório atual     |
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

| Opções              | Padrão                                                        | Descrição                                                                            |
| ------------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol$context]($style) "`                            | O formato do módulo.                                                                 |
| `symbol`            | `"🐳 "`                                                        | O simbolo usado antes de exibir a versão do contexto docker.                         |
| `only_with_files`   | `true`                                                        | Exibe somente quando houver um arquivo                                               |
| `detect_extensions` | `[]`                                                          | Quais extensões devem acionar este módulo (precisa que `only_with_files` seja true). |
| `detect_files`      | `["docker-compose.yml", "docker-compose.yaml", "Dockerfile"]` | Quais arquivos devem acionar este módulo (precisa que `only_with_files` seja true).  |
| `detect_folders`    | `[]`                                                          | Quais pastas devem acionar este módulo (precisa que `only_with_files` seja true).    |
| `style`             | `"blue bold"`                                                 | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                       | Desabilita o módulo `docker_context`.                                                |

### Variáveis

| Variável  | Exemplo        | Descrição                          |
| --------- | -------------- | ---------------------------------- |
| context   | `test_context` | O contexto atual do docker         |
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
| `symbol`            | `".NET "`                                                                                               | O símbolo usado antes de exibir a versão do dotnet.                                  |
| `heuristic`         | `true`                                                                                                  | Usa a versão de detecção rápida do starship snappy.                                  |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                    | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold blue"`                                                                                           | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                                                 | Desabilita o módulo `dotnet`.                                                        |

### Variáveis

| Variável  | Exemplo          | Descrição                          |
| --------- | ---------------- | ---------------------------------- |
| version   | `v3.1.201`       | A versão do sdk `dotnet`           |
| tfm       | `netstandard2.0` | O framework alvo do projeto atual  |
| symbol    |                  | Espelha o valor da opção `símbolo` |
| style\* |                  | Espelha o valor da opção `style`   |

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
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | O formato do módulo elixir.                                                          |
| `version_format`    | `"v${raw}"`                                                 | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | O símbolo usado antes de exibir a versão do Elixir/Erlang.                           |
| `detect_extensions` | `[]`                                                        | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["mix.exs"]`                                               | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                        | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold purple"`                                             | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                     | Desabilita o módulo `elixir`.                                                        |

### Variáveis

| Variável    | Exemplo | Descrição                          |
| ----------- | ------- | ---------------------------------- |
| version     | `v1.10` | A versão do `elixir`               |
| otp_version |         | A versão otp do `elixir`           |
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
| `symbol`            | `"🌳 "`                                             | O formato de string que representa o simbolo do Elm.                                 |
| `detect_extensions` | `["elm"]`                                          | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["elm.json", "elm-package.json", ".elm-version"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `["elm-stuff"]`                                    | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"cyan bold"`                                      | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                            | Desabilita o módulo `elm`.                                                           |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v0.19.1` | A versão do `elm`                  |
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

| Opções     | Padrão                         | Descrição                                                                      |
| ---------- | ------------------------------ | ------------------------------------------------------------------------------ |
| `symbol`   | `""`                           | O simbolo usado antes de exibir o valor da variável.                           |
| `variable` |                                | A variável de ambiente a ser exibida.                                          |
| `default`  |                                | O valor padrão para exibir quando a variável selecionada não estiver definida. |
| `format`   | `"with [$env_value]($style) "` | O formato do módulo.                                                           |
| `disabled` | `false`                        | Desabilita o módulo `env_var`.                                                 |

### Variáveis

| Variável  | Exemplo                                  | Descrição                               |
| --------- | ---------------------------------------- | --------------------------------------- |
| env_value | `Windows NT` (se _variable_ seria `$OS`) | O valor de ambiente da opção `variable` |
| symbol    |                                          | Espelha o valor da opção `símbolo`      |
| style\* | `black bold dimmed`                      | Espelha o valor da opção `style`        |

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
default = "shell desconhecido"
[env_var.USER]
default = "user desconhecido"
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
| `symbol`            | `" "`                               | O símbolo usado antes de exibir a versão do erlang.                                  |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                                  |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `disabled`          | `false`                              | Desabilita o módulo `erlang`.                                                        |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v22.1.3` | A versão do `erlang`               |
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

| Opções     | Padrão         | Descrição                               |
| ---------- | -------------- | --------------------------------------- |
| `symbol`   | `"."`          | O simbolo usado para preencher a linha. |
| `style`    | `"bold black"` | O estilo do módulo.                     |
| `disabled` | `false`        | Desabilita o módulo `fill`              |

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

| Opções            | Padrão                                                     | Descrição                                                        |
| ----------------- | ---------------------------------------------------------- | ---------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | O formato do módulo.                                             |
| `symbol`          | `"☁️  "`                                                   | O simbolo usado antes de exibir o perfil atual do GCP.           |
| `region_aliases`  |                                                            | Tabela de aliases de região para exibir além do nome do GCP.     |
| `project_aliases` |                                                            | Table of project aliases to display in addition to the GCP name. |
| `style`           | `"bold blue"`                                              | O estilo do módulo.                                              |
| `disabled`        | `false`                                                    | Disables the `gcloud` module.                                    |

### Variáveis

| Variável  | Exemplo       | Descrição                                                          |
| --------- | ------------- | ------------------------------------------------------------------ |
| region    | `us-central1` | A região atual do GCP                                              |
| account   | `foo`         | O perfil atual do GCP                                              |
| domain    | `example.com` | O perfil de domínio atual do GCP                                   |
| project   |               | O projeto atual do GCP                                             |
| active    | `default`     | O nome da configuração escrita em `~/.config/gcloud/active_config` |
| symbol    |               | Espelha o valor da opção `símbolo`                                 |
| style\* |               | Espelha o valor da opção `style`                                   |

*: This variable can only be used as a part of a style string

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

#### Display account and aliased project

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\($project\))]($style) '
[gcloud.project_aliases]
very-long-project-name = "vlpn"
```

## Git Branch

The `git_branch` module shows the active branch of the repo in your current directory.

### Opções

| Opções               | Padrão                           | Descrição                                                                                         |
| -------------------- | -------------------------------- | ------------------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                          | Exibe o nome do braço remoto, mesmo se ele for igual ao nome do braço local.                      |
| `format`             | `"on [$symbol$branch]($style) "` | O formato do módulo. Use `"$branch"` para se referir ao nome do braço atual.                      |
| `symbol`             | `" "`                           | Um formato de string que representa o simbolo do git branch.                                      |
| `style`              | `"bold purple"`                  | O estilo do módulo.                                                                               |
| `truncation_length`  | `2^63 - 1`                       | Truncates um braço do git para `N` caracteres.                                                    |
| `truncation_symbol`  | `"…"`                            | O simbolo usado para indicar que o nome braço foi truncado. Você pode usar `""` para sem simbolo. |
| `only_attached`      | `false`                          | Apenas exibe o nome do braço quando o estado não for detached `HEAD`.                             |
| `disabled`           | `false`                          | Desabilita o módulo `git_branch`.                                                                 |

### Variáveis

| Variável      | Exemplo  | Descrição                                                                                         |
| ------------- | -------- | ------------------------------------------------------------------------------------------------- |
| branch        | `master` | O nome do braço atual, retornará para `HEAD` se não tiver braço atual (e.x: git detached `HEAD`). |
| remote_name   | `origin` | O nome do remoto.                                                                                 |
| remote_branch | `master` | O nome do braço rastreado no `remote_name`.                                                       |
| symbol        |          | Espelha o valor da opção `símbolo`                                                                |
| style\*     |          | Espelha o valor da opção `style`                                                                  |

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

| Opções               | Padrão                             | Descrição                                                          |
| -------------------- | ---------------------------------- | ------------------------------------------------------------------ |
| `commit_hash_length` | `7`                                | O tamanho do git commit hash para ser exibido.                     |
| `format`             | `"[\\($hash$tag\\)]($style) "` | O formato do módulo.                                               |
| `style`              | `"bold green"`                     | O estilo do módulo.                                                |
| `only_detached`      | `true`                             | Apenas exibe o git commit hash quando o estado for detached `HEAD` |
| `tag_disabled`       | `true`                             | Desabilita a exibição da informação da tag no módulo `git_commit`. |
| `tag_symbol`         | `" 🏷 "`                            | Simbolo da tag prefixado na informação a ser exibida               |
| `disabled`           | `false`                            | Desabilita o módulo `git_commit`.                                  |

### Variáveis

| Variável  | Exemplo   | Descrição                        |
| --------- | --------- | -------------------------------- |
| hash      | `b703eb3` | A hash atual do git commit       |
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

| Opções         | Padrão                                                          | Descrição                                                                            |
| -------------- | --------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `rebase`       | `"REBASING"`                                                    | O formato de string exibida quando um `rebase` esta em progresso.                    |
| `merge`        | `"MERGING"`                                                     | O formato de string exibida quando um `merge` esta em progresso.                     |
| `revert`       | `"REVERTING"`                                                   | O formato de string exibida quando um `revert` esta em progresso.                    |
| `cherry_pick`  | `"CHERRY-PICKING"`                                              | O formato de string exibida quando um `cherry-pick` esta em progresso.               |
| `bisect`       | `"BISECTING"`                                                   | O formato de string exibida quando um `bisect` esta em progresso.                    |
| `am`           | `"AM"`                                                          | O formato de string exibida quando um `apply-mailbox` (`git am`) esta em progresso.  |
| `am_or_rebase` | `"AM/REBASE"`                                                   | O formato de string exibida quando um `apply-mailbox` or `rebase` esta em progresso. |
| `style`        | `"bold yellow"`                                                 | O estilo do módulo.                                                                  |
| `format`       | `'\([$state( $progress_current/$progress_total)]($style)\) '` | O formato do módulo.                                                                 |
| `disabled`     | `false`                                                         | Desabilita o módulo `git_state`.                                                     |

### Variáveis

| Variável         | Exemplo    | Descrição                              |
| ---------------- | ---------- | -------------------------------------- |
| state            | `REBASING` | O estado atual do repo                 |
| progress_current | `1`        | O progresso da operação atual          |
| progress_total   | `2`        | O total do progresso da operação atual |
| style\*        |            | Espelha o valor da opção `style`       |

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

| Opções               | Padrão                                                       | Descrição                                   |
| -------------------- | ------------------------------------------------------------ | ------------------------------------------- |
| `added_style`        | `"bold green"`                                               | O estilo para a contagem de adições.        |
| `deleted_style`      | `"bold red"`                                                 | O estilo para a contagem de exclusões.      |
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
| `format`            | `'([\[$all_status$ahead_behind\]]($style) )'` | O formato padrão para `git_status`  |
| `conflicted`        | `"="`                                           | Este braço tem conflitos.           |
| `ahead`             | `"⇡"`                                           | O formato do `ahead`                |
| `behind`            | `"⇣"`                                           | O formato do `behind`               |
| `diverged`          | `"⇕"`                                           | O formato do `diverged`             |
| `up_to_date`        | `""`                                            | O formato do `up_to_date`           |
| `untracked`         | `"?"`                                           | O formato do `untracked`            |
| `stashed`           | `"$"`                                           | O formato do `stashed`              |
| `modified`          | `"!"`                                           | O formato do `modified`             |
| `staged`            | `"+"`                                           | O formato do `staged`               |
| `renamed`           | `"»"`                                           | O formato do `renamed`              |
| `deleted`           | `"✘"`                                           | O formato do `deleted`              |
| `style`             | `"bold red"`                                    | O estilo do módulo.                 |
| `ignore_submodules` | `false`                                         | Ignora as alterações de submódulos. |
| `disabled`          | `false`                                         | Desabilita o módulo `git_status`.   |

### Variáveis

The following variables can be used in `format`:

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

*: This variable can only be used as a part of a style string

The following variables can be used in `diverged`:

| Variável       | Descrição                                           |
| -------------- | --------------------------------------------------- |
| `ahead_count`  | Número de commits a frente do braço de rastreamento |
| `behind_count` | Número de commits atrás do braço de rastreamento    |

The following variables can be used in `conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` and `deleted`:

| Variável | Descrição                  |
| -------- | -------------------------- |
| `count`  | Exibe o número de arquivos |

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
| `symbol`            | `"🐹 "`                                                                         | O formato da string que representa o simbolo do Go.                                  |
| `detect_extensions` | `["go"]`                                                                       | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `["Godeps"]`                                                                   | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold cyan"`                                                                  | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                        | Desabilita o módulo `golang`.                                                        |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v1.12.1` | A versão do `go`                   |
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
| `symbol`            | `"⎈ "`                               | O formato de string que representa o simbolo do Helm.                                |
| `style`             | `"bold white"`                       | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `helm`.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v3.1.1` | A versão do `helm`                 |
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

| Opções     | Padrão                      | Descrição                                                                                                                                                |
| ---------- | --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only` | `true`                      | Apenas exibe o hostname quando conectado em uma sessão SSH.                                                                                              |
| `trim_at`  | `"."`                       | String na qual vai truncar o hostname, após a primeira correspondência. `"."` vai truncar após o primeiro ponto. `""` vai desabilitar qualquer truncação |
| `format`   | `"[$hostname]($style) in "` | O formato do módulo.                                                                                                                                     |
| `style`    | `"bold dimmed green"`       | O estilo do módulo.                                                                                                                                      |
| `disabled` | `false`                     | Desabilita o módulo `hostname`.                                                                                                                          |

### Variáveis

| Variável  | Exemplo    | Descrição                        |
| --------- | ---------- | -------------------------------- |
| hostname  | `computer` | O hostname do computador         |
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
| `symbol`            | `"☕ "`                                                                                                    | Um formato de string que representa o simbolo do Java                                |
| `style`             | `"red dimmed"`                                                                                            | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                                                   | Desabilita o módulo `java`.                                                          |

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| version   | `v14`   | A versão do `java`                 |
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

| Opções             | Padrão                        | Descrição                                                                 |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------- |
| `threshold`*       | `1`                           | Exibe o número de jobs se excedido.                                       |
| `symbol_threshold` | `1`                           | Exibe `symbol` se o número de jobs for ao menos `symbol_threshold`.       |
| `number_threshold` | `2`                           | Exibe o número de jobs se o número de jobs é ao menos `number_threshold`. |
| `format`           | `"[$symbol$number]($style) "` | O formato do módulo.                                                      |
| `symbol`           | `"✦"`                         | A string usada para representar a variável `symbol`.                      |
| `style`            | `"bold blue"`                 | O estilo do módulo.                                                       |
| `disabled`         | `false`                       | Desabilita o módulo `jobs`.                                               |

*: This option is deprecated, please use the `number_threshold` and `symbol_threshold` options instead.

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| number    | `1`     | O número de jobs                   |
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
| `symbol`            | `"ஃ "`                               | O formato de string que representa o simbolo do Julia.                               |
| `style`             | `"bold purple"`                      | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `julia`.                                                         |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v1.4.0` | A versão do `julia`                |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
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

Displays the current [Kubernetes context](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) name and, if set, the namespace, user and cluster from the kubeconfig file. The namespace needs to be set in the kubeconfig file, this can be done via `kubectl config set-context starship-context --namespace astronaut`. Similarly the user and cluster can be set with `kubectl config set-context starship-context --user starship-user` and `kubectl config set-context starship-context --cluster starship-cluster`. If the `$KUBECONFIG` env var is set the module will use that if not it will use the `~/.kube/config`.

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
| context   | `starship-context`   | The current kubernetes context name      |
| namespace | `starship-namespace` | If set, the current kubernetes namespace |
| user      | `starship-user`      | If set, the current kubernetes user      |
| cluster   | `starship-cluster`   | If set, the current kubernetes cluster   |
| symbol    |                      | Espelha o valor da opção `símbolo`       |
| style\* |                      | Espelha o valor da opção `style`         |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [⛵ ($user on )($cluster in )$context \($namespace\)](dimmed green) '
disabled = false
[kubernetes.context_aliases]
"dev.local.cluster.k8s" = "dev"
".*/openshift-cluster/.*" = "openshift"
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
```

#### Regex Matching

Additional to simple aliasing, `context_aliases` also supports extended matching and renaming using regular expressions.

The regular expression must match on the entire kube context, capture groups can be referenced using `$name` and `$N` in the replacement. This is more explained in the [regex crate](https://docs.rs/regex/1.5.4/regex/struct.Regex.html#method.replace) documentation.

Long and automatically generated cluster names can be identified and shortened using regular expressions:

```toml
[kubernetes.context_aliases]
# OpenShift contexts carry the namespace and user in the kube context: `namespace/name/user`:
".*/openshift-cluster/.*" = "openshift"
# Or better, to rename every OpenShift cluster at once:
".*/(?P<var_cluster>[\\w-]+)/.*" = "$var_cluster"

# Contexts from GKE, AWS and other cloud providers usually carry additional information, like the region/zone.
# The following entry matches on the GKE format (`gke_projectname_zone_cluster-name`)
# and renames every matching kube context into a more readable format (`gke-cluster-name`):
"gke_.*_(?P<var_cluster>[\\w-]+)" = "gke-$var_cluster"
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

## Local IP

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
| `symbol`            | `"🌙 "`                               | Uma string que representa o simbolo do Lua.                                          |
| `detect_extensions` | `["lua"]`                            | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[".lua-version"]`                   | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `["lua"]`                            | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold blue"`                        | O estilo do módulo.                                                                  |
| `lua_binary`        | `"lua"`                              | Configura o binário lua que o Starship executa para pegar a versão.                  |
| `disabled`          | `false`                              | Desabilita o módulo `lua`.                                                           |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v5.4.0` | A versão do `lua`                  |
| symbol    |          | Espelha o valor da opção `símbolo` |
| style\* |          | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Uso de Memória

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

This module is disabled by default. To enable it, set `disabled` to `false` in your configuration file.

:::

### Opções

| Opções      | Padrão                                          | Descrição                                                     |
| ----------- | ----------------------------------------------- | ------------------------------------------------------------- |
| `threshold` | `75`                                            | Esconde o uso de memoria a menos que exceda esta porcentagem. |
| `format`    | `"via $symbol [${ram}( \| ${swap})]($style) "` | O formato do módulo.                                          |
| `symbol`    | `"🐏"`                                           | O simbolo usado antes de exibir o uso de memoria.             |
| `style`     | `"bold dimmed white"`                           | O estilo do módulo.                                           |
| `disabled`  | `true`                                          | Desabilita o módulo `memory_usage`.                           |

### Variáveis

| Variável         | Exemplo       | Descrição                                         |
| ---------------- | ------------- | ------------------------------------------------- |
| ram              | `31GiB/65GiB` | O uso/total de memoria RAM atual do sistema.      |
| ram_pct          | `48%`         | A porcentagem de uso atual da memoria do sistema. |
| swap\*\*     | `1GiB/4GiB`   | O tamanho atual do swap do sistema.               |
| swap_pct\*\* | `77%`         | A porcentagem atual de uso do swap.               |
| symbol           | `🐏`           | Espelha o valor da opção `símbolo`                |
| style\*        |               | Espelha o valor da opção `style`                  |

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

| Opções              | Padrão                           | Descrição                                                                               |
| ------------------- | -------------------------------- | --------------------------------------------------------------------------------------- |
| `symbol`            | `" "`                           | O simbolo usado ante do marcador hg ou nome do braço do repositório no diretório atual. |
| `style`             | `"bold purple"`                  | O estilo do módulo.                                                                     |
| `format`            | `"on [$symbol$branch]($style) "` | O formato do módulo.                                                                    |
| `truncation_length` | `2^63 - 1`                       | Trunca o nome do braço hg para `N` caracteres                                           |
| `truncation_symbol` | `"…"`                            | O simbolo usado para indicar que o nome braço foi truncado.                             |
| `disabled`          | `true`                           | Desabilita o módulo `hg_branch`.                                                        |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| branch    | `master` | O braço mercurial ativo            |
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
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo                                                                  |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | O símbolo usado antes de exibir a versão do Nim.                                     |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["nim.cfg"]`                        | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold yellow"`                      | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `nim`.                                                           |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v1.2.0` | A versão do `nimc`                 |
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

| Opções       | Padrão                                         | Descrição                                         |
| ------------ | ---------------------------------------------- | ------------------------------------------------- |
| `format`     | `'via [$symbol$state( \($name\))]($style) '` | O formato do módulo.                              |
| `symbol`     | `"❄️ "`                                        | Uma string que representa o simbolo do nix-shell. |
| `style`      | `"bold blue"`                                  | O estilo do módulo.                               |
| `impure_msg` | `"impure"`                                     | Uma string que exibe quando o shell é impuro.     |
| `pure_msg`   | `"pure"`                                       | Uma string que exibe quando o shell é puro.       |
| `disabled`   | `false`                                        | Desabilita o módulo `nix_shell`.                  |

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| state     | `pure`  | O estado do nix-shell              |
| name      | `lorri` | O nome do nix-shell                |
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

| Opções              | Padrão                               | Descrição                                                                                                |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                                     |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch`                     |
| `symbol`            | `" "`                               | Uma string que representa o simbolo do Node.js.                                                          |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Quais extensões devem ativar este módulo.                                                                |
| `detect_files`      | `["package.json", ".node-version"]`  | Quais nomes de arquivos devem ativar este módulo.                                                        |
| `detect_folders`    | `["node_modules"]`                   | Quais pastas devem ativar este módulo.                                                                   |
| `style`             | `"bold green"`                       | O estilo do módulo.                                                                                      |
| `disabled`          | `false`                              | Desabilita o módulo `nodejs`.                                                                            |
| `not_capable_style` | `bold red`                           | O estilo para o módulo quando a propriedade engine no package.json não coincide com a versão do Node.js. |

### Variáveis

| Variável  | Exemplo    | Descrição                          |
| --------- | ---------- | ---------------------------------- |
| version   | `v13.12.0` | A versão do `node`                 |
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
| `symbol`                  | `"🐫 "`                                                                     | O símbolo usado antes de exibir a versão do OCaml.                                   |
| `global_switch_indicator` | `""`                                                                       | A string usada para representar a mudança global OPAM.                               |
| `local_switch_indicator`  | `"*"`                                                                      | A string usada para representar as mudanças locais do OPAM.                          |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Quais extensões devem ativar este módulo.                                            |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Quais pastas devem ativar este módulo.                                               |
| `style`                   | `"bold yellow"`                                                            | O estilo do módulo.                                                                  |
| `disabled`                | `false`                                                                    | Desabilita o módulo `ocaml`.                                                         |

### Variáveis

| Variável         | Exemplo      | Descrição                                                        |
| ---------------- | ------------ | ---------------------------------------------------------------- |
| version          | `v4.10.0`    | A versão do `ocaml`                                              |
| switch_name      | `my-project` | O switch OPAM ativo                                              |
| switch_indicator |              | Espelha o valor do `indicator` para o switch ativo atual do OPAM |
| symbol           |              | Espelha o valor da opção `símbolo`                               |
| style\*        |              | Espelha o valor da opção `style`                                 |

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

| Opções     | Padrão                                              | Descrição                                            |
| ---------- | --------------------------------------------------- | ---------------------------------------------------- |
| `format`   | `"on [$symbol$cloud(\\($project\\))]($style) "` | O formato do módulo.                                 |
| `symbol`   | `"☁️ "`                                             | O simbolo usado para exibir o OpenStack cloud atual. |
| `style`    | `"bold yellow"`                                     | O estilo do módulo.                                  |
| `disabled` | `false`                                             | Desabilita o módulo `openstack`.                     |

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| cloud     | `corp`  | O OpenStack cloud atual            |
| project   | `dev`   | O projeto OpenStack atual          |
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

## Versionamento de Pacotes

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
| `symbol`          | `"📦 "`                            | O símbolo usado antes de exibir a versão do pacote.                                  |
| `version_format`  | `"v${raw}"`                       | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`           | `"bold 208"`                      | O estilo do módulo.                                                                  |
| `display_private` | `false`                           | Habilita a exibição da versão para os pacotes marcados como privado.                 |
| `disabled`        | `false`                           | Desabilita o módulo `package`.                                                       |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v1.0.0` | A versão do seu pacote             |
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
| `symbol`            | `"🐪 "`                                                                                                   | O símbolo usado antes de exibir a versão do Perl.                                    |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                     | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold 149"`                                                                                             | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                                                  | Desabilita o módulo `perl`.                                                          |

### Variáveis

| Variável  | Exemplo   | Descrição                          |
| --------- | --------- | ---------------------------------- |
| version   | `v5.26.1` | A versão do `perl`                 |
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
| `disabled`          | `false`                              | Desabilita o módulo `php`.                                                           |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v7.3.8` | A versão do `php`                  |
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
| `symbol`         | `" "`                                       | Uma string que é exibida antes do Pulumi stack.                                      |
| `style`          | `"bold 5"`                                   | O estilo do módulo.                                                                  |
| `disabled`       | `false`                                      | Desabilita o módulo `pulumi`.                                                        |

### Variáveis

| Variável  | Exemplo    | Descrição                          |
| --------- | ---------- | ---------------------------------- |
| version   | `v0.12.24` | A versão do `pulumi`               |
| stack     | `dev`      | A stack Pulumi atual               |
| username  | `alice`    | The current Pulumi username        |
| symbol    |            | Espelha o valor da opção `símbolo` |
| style\* |            | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

#### With Pulumi Version

```toml
# ~/.config/starship.toml

[pulumi]
format = "[🛥 ($version )$stack]($style) "
```

#### Without Pulumi version

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
| `symbol`            | `"<=> "`                       | O símbolo usado antes de exibir a versão do PureScript.                              |
| `detect_extensions` | `["purs"]`                           | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["spago.dhall"]`                    | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold white"`                       | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `purescript`.                                                    |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `0.13.5` | A versão do `purescript`           |
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

| Opções               | Padrão                                                                                                       | Descrição                                                                            |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | O formato do módulo.                                                                 |
| `version_format`     | `"v${raw}"`                                                                                                  | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`             | `"🐍 "`                                                                                                       | Uma string que representa o simbolo do Python                                        |
| `style`              | `"yellow bold"`                                                                                              | O estilo do módulo.                                                                  |
| `pyenv_version_name` | `false`                                                                                                      | Usa pyenv para pegar a versão do Python                                              |
| `pyenv_prefix`       | `pyenv`                                                                                                      | Prefixo antes da versão do pyenv, apenas usado se pyenv for usado                    |
| `python_binary`      | `["python", "python3", "python2"]`                                                                           | Configura o binário python que o Starship vai executar para obter a versão.          |
| `detect_extensions`  | `["py"]`                                                                                                     | Quais extensões devem acionar este módulo                                            |
| `detect_files`       | `[".python-version", "Pipfile", "__init__.py", "pyproject.toml", "requirements.txt", "setup.py", "tox.ini"]` | []                                                                                   |
| `detect_folders`     | `[]`                                                                                                         | Quais pastas devem ativar este módulo                                                |
| `disabled`           | `false`                                                                                                      | Desabilita o módulo `python`.                                                        |

::: tip

The `python_binary` variable accepts either a string or a list of strings. Starship will try executing each binary until it gets a result. Note you can only change the binary that Starship executes to get the version of Python not the arguments that are used.

The default values and order for `python_binary` was chosen to first identify the Python version in a virtualenv/conda environments (which currently still add a `python`, no matter if it points to `python3` or `python2`). This has the side effect that if you still have a system Python 2 installed, it may be picked up before any Python 3 (at least on Linux Distros that always symlink `/usr/bin/python` to Python 2). If you do not work with Python 2 anymore but cannot remove the system Python 2, changing this to `"python3"` will hide any Python version 2, see example below.

:::

### Variáveis

| Variável     | Exemplo         | Descrição                               |
| ------------ | --------------- | --------------------------------------- |
| version      | `"v3.8.1"`      | A versão do `python`                    |
| symbol       | `"🐍 "`          | Espelha o valor da opção `símbolo`      |
| style        | `"yellow bold"` | Espelha o valor da opção `style`        |
| pyenv_prefix | `"pyenv "`      | Espelha o valor da opção `pyenv_prefix` |
| virtualenv   | `"venv"`        | O nome atual do `virtualenv`            |

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
| `symbol`            | `"📐"`                                | Uma string que representa o simbolo do R.                                            |
| `style`             | `"blue bold"`                        | O estilo do módulo.                                                                  |
| `detect_extensions` | `["R", "Rd", "Rmd", "Rproj", "Rsx"]` | Quais extensões devem acionar este módulo                                            |
| `detect_files`      | `[".Rprofile"]`                      | []                                                                                   |
| `detect_folders`    | `[".Rproj.user"]`                    | Quais pastas devem ativar este módulo                                                |
| `disabled`          | `false`                              | Desabilita o módulo `r`.                                                             |

### Variáveis

| Variável | Exemplo       | Descrição                          |
| -------- | ------------- | ---------------------------------- |
| version  | `v4.0.5`      | A versão do `R`                    |
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
| `symbol`            | `"🔺 "`                               | Uma string que representa o simbolo do Red.                                          |
| `detect_extensions` | `["red"]`                            | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"red bold"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `red`.                                                           |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v2.5.1` | A versão do `red`                  |
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
| `symbol`            | `"💎 "`                               | Uma string que representa o simbolo do Ruby.                                         |
| `detect_extensions` | `["rb"]`                             | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `detect_variables`  | `["RUBY_VERSION", "RBENV_VERSION"]`  | Quais variáveis de ambiente devem ativar este módulo.                                |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `ruby`.                                                          |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v2.5.1` | A versão do `ruby`                 |
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
| `symbol`            | `"🦀 "`                               | Uma string que representa o simbolo do Rust                                          |
| `detect_extensions` | `["rs"]`                             | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Cargo.toml"]`                     | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `rust`.                                                          |

### Variáveis

| Variável  | Exemplo           | Descrição                          |
| --------- | ----------------- | ---------------------------------- |
| version   | `v1.43.0-nightly` | A versão do `rustc`                |
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
| `symbol`            | `"🆂 "`                                   | Uma string que representa o simbolo do Scala.                                        |
| `style`             | `"red dimmed"`                           | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                  | Desabilita o módulo `scala`.                                                         |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `2.13.5` | A versão do `scala`                |
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

| Opções                 | Padrão                    | Descrição                                               |
| ---------------------- | ------------------------- | ------------------------------------------------------- |
| `bash_indicator`       | `bsh`                     | Uma string para representar o bash.                     |
| `fish_indicator`       | `fsh`                     | Uma string usada para representar o fish.               |
| `zsh_indicator`        | `zsh`                     | Uma string usada para representar o zsh.                |
| `powershell_indicator` | `psh`                     | Uma string usada para representar o powershell.         |
| `ion_indicator`        | `ion`                     | Uma string usada para representar o ion.                |
| `elvish_indicator`     | `esh`                     | Uma string usada para representar o elvish.             |
| `tcsh_indicator`       | `tsh`                     | Uma string usada para representar o tcsh.               |
| `xonsh_indicator`      | `xsh`                     | Uma string usada para representar o xonsh.              |
| `cmd_indicator`        | `cmd`                     | A format string used to represent cmd.                  |
| `nu_indicator`         | `nu`                      | Uma string usada para representar o nu.                 |
| `unknown_indicator`    |                           | Valor padrão para exibir quando o shell é desconhecido. |
| `format`               | `"[$indicator]($style) "` | O formato do módulo.                                    |
| `style`                | `"white bold"`            | O estilo do módulo.                                     |
| `disabled`             | `true`                    | Desabilita o módulo `shell`.                            |

### Variáveis

| Variável  | Padrão | Descrição                                                     |
| --------- | ------ | ------------------------------------------------------------- |
| indicator |        | Espelha o valor do `indicator` para o shell usado atualmente. |
| style\* |        | Espelha o valor da opção `style`.                             |

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

| Opções      | Padrão                       | Descrição                                                         |
| ----------- | ---------------------------- | ----------------------------------------------------------------- |
| `threshold` | `2`                          | Limite de exibição.                                               |
| `format`    | `"[$symbol$shlvl]($style) "` | O formato do módulo.                                              |
| `symbol`    | `"↕️  "`                     | O simbolo usado para representar o `SHLVL`.                       |
| `repeat`    | `false`                      | Caso o `symbol` deva se repetir de acordo com o total do `SHLVL`. |
| `style`     | `"bold yellow"`              | O estilo do módulo.                                               |
| `disabled`  | `true`                       | Desabilita o módulo `shlvl`.                                      |

### Variáveis

| Variável  | Exemplo | Descrição                          |
| --------- | ------- | ---------------------------------- |
| shlvl     | `3`     | O valor atual do `SHLVL`           |
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

| Opções     | Padrão                           | Descrição                                   |
| ---------- | -------------------------------- | ------------------------------------------- |
| `format`   | `'[$symbol\[$env\]]($style) '` | O formato do módulo.                        |
| `symbol`   | `""`                             | Uma string exibida antes do nome da imagem. |
| `style`    | `"bold dimmed blue"`             | O estilo do módulo.                         |
| `disabled` | `false`                          | Desabilita o módulo `singularity`.          |

### Variáveis

| Variável  | Exemplo      | Descrição                          |
| --------- | ------------ | ---------------------------------- |
| env       | `centos.img` | A imagem atual do Singularity      |
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
| `symbol`                | `"✖"`                                                                                | O simbolo exibido no erro de programa                   |
| `success_symbol`        | `"✔️"`                                                                               | O simbolo exibido no sucesso de programa                |
| `not_executable_symbol` | `"🚫"`                                                                                | O simbolo exibido quando o arquivo não é executável     |
| `not_found_symbol`      | `"🔍"`                                                                                | O simbolo exibido quando o comando não é encontrado     |
| `sigint_symbol`         | `"🧱"`                                                                                | O simbolo exibido no SIGINT (Ctrl + c)                  |
| `signal_symbol`         | `"⚡"`                                                                                | O simbolo exibido em qualquer sinal                     |
| `style`                 | `"bold red"`                                                                         | O estilo do módulo.                                     |
| `recognize_signal_code` | `true`                                                                               | Habilita o mapeamento de sinais para códigos de saída   |
| `map_symbol`            | `false`                                                                              | Habilita o mapeamento de símbolos para códigos de saída |
| `pipestatus`            | `false`                                                                              | Habilita o relatório de pipestatus                      |
| `pipestatus_separator`  | `|`                                                                                  |                                                         |
| `pipestatus_format`     | `\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)` | O formato do módulo quando o comando é um pipeline      |
| `disabled`              | `true`                                                                               | Desabilita o módulo `status`.                           |

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
| symbol         |         | Espelha o valor da opção `símbolo`                                                                       |
| style\*      |         | Espelha o valor da opção `style`                                                                         |

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

| Opções          | Padrão                  | Descrição                                                                  |
| --------------- | ----------------------- | -------------------------------------------------------------------------- |
| `format`        | `[as $symbol]($style)"` | O formato do módulo                                                        |
| `symbol`        | `"🧙 "`                  | O simbolo exibido quando as credenciais estão em cache                     |
| `style`         | `"bold blue"`           | O estilo do módulo.                                                        |
| `allow_windows` | `false`                 | Desde que o Windows não tem um padrão sudo, o valor padrão é desabilitado. |
| `disabled`      | `true`                  | Desabilita o módulo `sudo`.                                                |

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
| `symbol`            | `"🐦 "`                               | Uma string que representa o simbolo do Swift                                         |
| `detect_extensions` | `["swift"]`                          | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Package.swift"]`                  | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold 202"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `swift`.                                                         |

### Variáveis

| Variável  | Exemplo  | Descrição                          |
| --------- | -------- | ---------------------------------- |
| version   | `v5.2.4` | A versão do `swift`                |
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
| `symbol`            | `"💠"`                                | Uma string que é exibida antes do workspace terraform.                               |
| `detect_extensions` | `["tf", "tfplan", "tfstate"]`        | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `[]`                                 | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[".terraform"]`                     | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold 105"`                         | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `terraform`.                                                     |

### Variáveis

| Variável  | Exemplo    | Descrição                          |
| --------- | ---------- | ---------------------------------- |
| version   | `v0.12.24` | A versão do `terraform`            |
| workspace | `default`  | O workspace atual do Terraform     |
| symbol    |            | Espelha o valor da opção `símbolo` |
| style\* |            | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

### Exemplo

#### With Terraform Version

```toml
# ~/.config/starship.toml

[terraform]
format = "[🏎💨 $version$workspace]($style) "
```

#### Without Terraform version

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

| Opções            | Padrão                  | Descrição                                                                                                                   |
| ----------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | A string de formato do módulo.                                                                                              |
| `use_12hr`        | `false`                 | Habilita a formatação de 12 horas                                                                                           |
| `time_format`     | veja abaixo             | A string [chrono format string](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html) usada para formatar a hora. |
| `style`           | `"bold yellow"`         | O estilo do módulo time                                                                                                     |
| `utc_time_offset` | `"local"`               | Define o UTC a ser usado. Intervalo de -24 &lt; x &lt; 24. Aceita floats para acomodar timezones 30/45.         |
| `disabled`        | `true`                  | Desabilita o módulo `time`.                                                                                                 |
| `time_range`      | `"-"`                   | Define o intervalo de tempo o qual o módulo será exibido. O horário deve ser especificado no formato de 24-hours            |

If `use_12hr` is `true`, then `time_format` defaults to `"%r"`. Otherwise, it defaults to `"%T"`. Manually setting `time_format` will override the `use_12hr` setting.

### Variáveis

| Variável  | Exemplo    | Descrição                        |
| --------- | ---------- | -------------------------------- |
| time      | `13:08:10` | A hora atual.                    |
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

## Nome do usuário

The `username` module shows active user's username. The module will be shown if any of the following conditions are met:

- O usuário atual é root
- O usuário atual não é o mesmo que está logado
- O usuário atual esta conectado em uma sessão SSH
- A variável `show_always` esta definida como true

::: tip

SSH connection is detected by checking environment variables `SSH_CONNECTION`, `SSH_CLIENT`, and `SSH_TTY`. If your SSH host does not set up these variables, one workaround is to set one of them with a dummy value.

:::

### Opções

| Opções        | Padrão                  | Descrição                               |
| ------------- | ----------------------- | --------------------------------------- |
| `style_root`  | `"bold red"`            | O estilo usado quando o usuário é root. |
| `style_user`  | `"bold yellow"`         | O estilo usado para usuários não root.  |
| `format`      | `"[$user]($style) in "` | O formato do módulo.                    |
| `show_always` | `false`                 | Sempre exibe o módulo `username`.       |
| `disabled`    | `false`                 | Desabilita o módulo `username`.         |

### Variáveis

| Variável | Exemplo      | Descrição                                                                              |
| -------- | ------------ | -------------------------------------------------------------------------------------- |
| `style`  | `"red bold"` | Espelha o valor da opção `style_root` quando o root esta logado e `style_user` se não. |
| `user`   | `"fulano"`   | O ID do usuário logado atualmente.                                                     |

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
| `symbol`            | `"⍱ "`                               | Um formato de string que representa o simbolo do Vagrant.                            |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["Vagrantfile"]`                    | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"cyan bold"`                        | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `vagrant`.                                                       |

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
| `symbol`            | `"V "`                                       | Um formato de string que representa o simbolo do V                                   |
| `detect_extensions` | `["v"]`                                      | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                         | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"blue bold"`                                | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                      | Desabilita o módulo `vlang`.                                                         |

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
| `symbol`   |                                  | O simbolo usado antes de exibir o nome do repositório. |
| `style`    | `"bold yellow"`                  | O estilo do módulo.                                    |
| `format`   | `"vcsh [$symbol$repo]($style) "` | O formato do módulo.                                   |
| `disabled` | `false`                          | Desabilita o módulo `vcsh`.                            |

### Variáveis

| Variável  | Exemplo                                     | Descrição                          |
| --------- | ------------------------------------------- | ---------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | O nome do repositório ativo        |
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
| `symbol`            | `"↯ "`                               | O símbolo usado antes de exibir a versão do Zig.                                     |
| `style`             | `"bold yellow"`                      | O estilo do módulo.                                                                  |
| `disabled`          | `false`                              | Desabilita o módulo `zig`.                                                           |
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

## Comandos Personalizados

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

| Opções        | Padrão                          | Descrição                                                                                                                                                                        |
| ------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     | `""`                            | O comando cuja a saída deve ser exibida. O comando será passado no stdin para o shell.                                                                                           |
| `when`        |                                 | Um comando de shell usado como condição para exibir o módulo. O módulo será exibido se o comando retornar `0` como código de status.                                             |
| `shell`       |                                 | [Veja abaixo](#custom-command-shell)                                                                                                                                             |
| `description` | `"<custom module>"`       | A descrição do módulo, isto será exibido quando executar `starship explain`.                                                                                                     |
| `files`       | `[]`                            | Os arquivos que serão buscados por correspondência no diretório atual.                                                                                                           |
| `directories` | `[]`                            | Os diretórios que serão buscados por correspondência no diretório atual.                                                                                                         |
| `extensions`  | `[]`                            | As extensões que serão buscadas por correspondência no diretório atual.                                                                                                          |
| `symbol`      | `""`                            | O simbolo usado antes de exibir a saída do comando.                                                                                                                              |
| `style`       | `"bold green"`                  | O estilo do módulo.                                                                                                                                                              |
| `format`      | `"[$symbol($output )]($style)"` | O formato do módulo.                                                                                                                                                             |
| `disabled`    | `false`                         | Desabilita este módulo `custom`.                                                                                                                                                 |
| `os`          |                                 | Nome do sistema operacional onde módulo sera exibido (unix, linux, macos, windows, ... ) [Veja os possíveis valores](https://doc.rust-lang.org/std/env/consts/constant.OS.html). |

### Variáveis

| Variável  | Descrição                          |
| --------- | ---------------------------------- |
| output    | A saída do comando no `shell`      |
| symbol    | Espelha o valor da opção `símbolo` |
| style\* | Espelha o valor da opção `style`   |

*: This variable can only be used as a part of a style string

#### Custom command shell

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
