# Configuração

Para começar a configurar a starship, crie o seguinte arquivo: `~/.config/starship.toml`.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Todas as configurações do starship são feitas neste arquivo [TOML](https://github.com/toml-lang/toml):

```toml
#Insere uma linha vazia entre os prompts
add_newline = true

# Substitui o simbolo "❯" do prompt por "➜"
[character]                            # O nome do módulo que nos estamos configurando é "character"
success_symbol = "[➜](bold green)"     # O seguimento do "success_symbol" é alterado para  "➜" com a cor  "bold green"

# Desabilita o módulo de package, ocultando completamente do prompt
[package]
disabled = true
```

Você pode alterar o caminho padrão do arquivo de configuração com a variável de ambiente `STARSHIP_CONFIG`:

```sh
export STARSHIP_CONFIG=~/.starship/config.toml
```

No PowerShell (Windows) você pode adicionar a seguinte linha no seu `$PROFILE`:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\.starship\config.toml"
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

### Terminologia

**Módulo**: Um componente no prompt que fornece informações baseado no contexto do seu SO. Por exemplo, o "nodejs"módulo exibe a versão do Node.js que está instalada no computador, se o diretório atual for um projeto Node.js.

**Variável**: Um pequeno subcomponente que contem informações fornecidas pelo módulo. Por exemplo, a variável "version" no módulo "nodejs"contem a versão atual do Node.js.

Por convenção, a maioria dos módulos tem um prefixo de cor (e.x. `via` no "nodejs") e um espaço vazio para sufixo.

### Formatação de Strings

Formatar uma string é a forma de como o módulo ira imprimir suas variáveis. A maioria dos módulos tem uma entrada chamada `format` que configura o formato que o módulo é exibido. Você pode usar textos, variáveis e grupo de textos em uma formatação de string.

#### Variável

Uma variável contem um simbolo `$` seguido pelo nome da variável. O nome da variável contem apenas letras, números e `_`.

Por exemplo:

- `$version` é uma formatação de string com uma variável chamada `version`.
- `$git_branch$git_commit` é uma formatação de string com duas variáveis chamadas `git_branch` e `git_commit`.
- `$git_branch $git_commit` Tem as duas variáveis separadas por espaço.

#### Grupo de Texto

Um grupo de texto é composto por duas partes diferentes.

A primeira parte, é contida em um `[]`, é uma [formatação de string](#format-strings). Você pode adicionar textos, variáveis ou até mesmos grupo de textos aninhados.

Na segunda parte, é composta por um `()`, é uma [estilização de string](#style-strings). Este pode ser usado para estilizar a primeira parte.

Por exemplo:

- `[on](red bold)` vai imprimir uma string `on` com texto em negrito e com a cor vermelha.
- `[⌘ $version](bold green)` vai imprimir o simbolo `⌘` seguido pela variável `version`, com o texto em negrito e na cor verde.
- `[a [b](red) c](green)` vai imprimir `a b c` com `b` vermelho, e `a` e `c` verde.

#### Estilo dos textos

A maioria dos módulos do starship permite que você configure o estilo de exibição dos textos. Isso é feito através de um parâmetro (geralmente chamado `style`) que é uma string especificando a configuração. Aqui estão alguns exemplos de strings de estilo e o que elas fazem. Para detalhes sobre a sintaxe completa, consulte o [guia de configurações avançadas](/advanced-config/).

- `"fg:green bg:blue"` deixa o texto verde com o fundo azul
- `"bg:blue fg:bright-green"` deixa o texto verde brilhante com o fundo azul
- `"bold fg:27"` deixa o texto em negrito com a cor 27 [da tabela ANSI](https://i.stack.imgur.com/KTSQa.png)
- `"underline bg:#bf5700"` deixa o texto sublinhado com o fundo laranja escuro
- `"bold italic fg:purple"` deixa o texto em negrito e itálico com a cor roxa
- `""` desabilita explicitamente todos os estilos

Note que a aparência do estilo será controlado pelo seu terminal. Por exemplo, alguns terminais deixarão as cores mais brilhantes ao invés de deixar o texto em negrito, ou alguns temas podem usar as mesmas cores para cores brilhantes e normais. Além disso, para textos em itálico, o terminal precisa ter suporte.

#### Formatação de String Condicional

Uma formatação condicional de string é envolto por `(` e `)` não vai ser exibido caso a variável dentro esteja vazia.

Por exemplo:

- `(@$region)` não vai exibir nada caso a variável `region` seja `None` ou vazia, caso contrario vai exibir `@` seguido pelo valor da variável region.
- `(texto qualquer)` não vai exibir nada sempre, pois não existe variável entre os parenteses.
- Quando usar `$all` é um atalho para `\[$a$b\]`, `($all)` vai exibir nada somente quando `$a` e `$b` são `None`. Isto funciona da mesma forma que `(\[$a$b\] )`.

#### Caractere de escape

Os símbolos a seguir tem uso especial em uma formatação de string. Se você quer imprimir os símbolos a seguir você deve adicionar uma barra invertida (`\`).

- \$
- \\
- [
- ]
- (
- )

Observe que `toml` tem [sua própria sintaxe de escape](https://github.com/toml-lang/toml#user-content-string). É recomendado usar uma string literal (`''`) no seu arquivo de configuração. Se você quer usar uma string básica (`""`), preste atenção ao usar a barra invertida `\`.

Por exemplo, quando você quer imprimir um simbolo `$` em uma nova linha, as configurações de `format` a seguir são equivalentes:

```toml
# com string básica
format = "\n\\$"

# com múltiplas linhas de string básica
format = """

\\$"""

# com string literal
format = '''

\$'''
```

## Prompt de Comando

Está é a lista de opções de configuração de prompt.

### Opções

| Opções            | Padrão                         | Descrição                                                              |
| ----------------- | ------------------------------ | ---------------------------------------------------------------------- |
| `format`          | [link](#default-prompt-format) | Configura o formato do prompt.                                         |
| `scan_timeout`    | `30`                           | Tempo limite para escanear arquivos (em milissegundos).                |
| `command_timeout` | `500`                          | Tempo limite de execução de comandos pelo starship (em milissegundos). |
| `add_newline`     | `true`                         | Insere linha vazia entre os prompts do shell.                          |

### Exemplo

```toml
# ~/.config/starship.toml

# Usa um format customizado
format = """
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) """

#Espera 10 milissegundos para que o starship check os arquivos do diretório atual.
scan_timeout = 10

# Desabilita uma nova linha no inicio do prompt
add_newline = false
```

### Format de Prompt Padrão

O `formato` padrão é usado para definir o formato do prompt, se um valor vazio ou não `formatado` for informado. Os valores padrão são os seguintes:

```toml
format = "$all"

# Que é equivalente a
format = """
$username\
$hostname\
$shlvl\
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
$nim\
$nodejs\
$ocaml\
$perl\
$php\
$purescript\
$python\
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
$env_var\
$crystal\
$custom\
$cmd_duration\
$line_break\
$lua\
$jobs\
$battery\
$time\
$status\
$shell\
$character"""
```

## AWS

O módulo `aws` exibi a região e perfil atual do AWS. Isto é baseado nas variáveis de env `AWS_REGION`, `AWS_DEFAULT_REGION`, and `AWS_PROFILE` contidas no arquivo `~/.aws/config`. Este modulo exibi também tempo de expiração de credenciais temporarias.

Quando usar [aws-vault](https://github.com/99designs/aws-vault) o perfil é lido da variável `AWS_VAULT` e o tempo de expiração de credenciais é lida da variável de env `AWS_SESSION_EXPIRATION`.

Quando usar [awsu](https://github.com/kreuzwerker/awsu) o perfil é lido da varável de env `AWSU_PROFILE`.

Quando usar [AWSume](https://awsu.me) o perfil é lido da variável `AWSUME_PROFILE` e o tempo de expiração de credenciais é lida da variável de env `AWSUME_EXPIRATION`.

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

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

## Bateria

O módulo `battery` exibe o quanto a bateria do dispositivo está carregada e o estado atual de carregamento. O módulo é visível somente quando a bateria está abaixo de 10%.

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

A configuração `display` é usada para definir quando o indicador de bateria deve ser exibido (threshold), qual deve ser o simbolo(symbol) e como você gostaria de exibir (style). Se nenhum `display` for fornecido. Os valores padrão são os seguintes:

```toml
[[battery.display]]
threshold = 10
style = "bold red"
```

O valor padrão das opções `charging_symbol` e `discharging_symbol`é respectivamente o valor das opções `battery`'s `charging_symbol` e `discharging_symbol`.

#### Opções

A opção `display` é um array da seguinte tabela.

| Opções               | Padrão     | Descrição                                                                                          |
| -------------------- | ---------- | -------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`       | O limite superior para exibição.                                                                   |
| `style`              | `bold red` | O estilo usado para exibir quando estiver em uso.                                                  |
| `charging_symbol`    | `-`        | Simbolo opcional, mostrado quando a opção estiver em uso, o simbolo padrão é `charging_symbol`.    |
| `discharging_symbol` | `-`        | Simbolo opcional, mostrado quando a opção estiver em uso, o simbolo padrão é `discharging_symbol`. |

#### Exemplo

```toml
[[battery.display]]  # "bold red" é estilo e o simbolo discharging_symbol serão usados quando a capacidade estiver entre 0% e 10%
threshold = 10
style = "bold red"

[[battery.display]]  # "bold yellow" é estilo e o simbolo 💦 serão usados quando a capacidade estiver entre 10% e 30%
threshold = 30
style = "bold yellow"
discharging_symbol = 💦

# Quando a capcidade for maior que  30%, o indicador de bateria não sera mostrado

```

## Caractere

O módulo `character` exibe um caracter (normalmente uma seta) ao lado de onde o texto começa a ser inserido no terminal.

O caractere vai te dizer se o ultimo comando foi bem sucedido ou não. Você pode fazer isto de duas maneiras:

- alterando a cor (`red`/`green`)
- alterando a forma (`❯`/`✖`)

Por padrão ele apenas muda de cor. Se você deseja alterar o formato de uma olhada [neste exemplo](#with-custom-error-shape).

::: warning

`error_symbol` não é suportado no elvish e nu shell.

:::

::: warning

`vicmd_symbol` é suportado apenas no fish e zsh.

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

O módulo `cmake` exibe a versão instalada do [CMake](https://cmake.org/). Por padrão o módulo será ativo se qualquer das condições a seguir for atendida:

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

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v3.17.3` | A versão do cmake                 |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

## Tempo de execução do comando

O módulo `cmd_duration` exibi o tempo que o ultimo comando levou para executar. O módulo vai exibir somente se o comando levar mais de dois segundos, ou o valor de configuração `min_time` existir.

::: warning Não utilize o DEBUG-trap no Bash

Se você esta rodando o Starship no `bash`, você não deve ativar a armadilha `DEBUG` após rodar `eval $(starship init $0)`, ou este módulo **vai** quebrar.

:::

Usuários do bash que precisam de funções pre-executadas podem usar [rcaloras's bash_preexec framework](https://github.com/rcaloras/bash-preexec). Simplesmente defina os arrays `preexec_functions` e `precmd_functions` antes de rodar `eval $(starship init $0)`, e depois pode proceder normalmente.

### Opções

| Opções               | Padrão                        | Descrição                                                     |
| -------------------- | ----------------------------- | ------------------------------------------------------------- |
| `min_time`           | `2_000`                       | Duração mais curta para exibir o tempo (em milissegundos).    |
| `show_milliseconds`  | `false`                       | Exibir milissegundos ou invés de segundos para duração.       |
| `format`             | `"took [$duration]($style) "` | O formato do módulo.                                          |
| `style`              | `"bold yellow"`               | O estilo do módulo.                                           |
| `disabled`           | `false`                       | Desabilita o módulo `cmd_duration`.                           |
| `show_notifications` | `false`                       | Exibi notificações no desktop quando o comando for concluído. |
| `min_time_to_notify` | `45_000`                      | Tempo minimo para notificação (em milissegundos).             |

::: tip

Para exibir notificações requer que o starship seja construído com suporte para `rust-notify`. Você consegue verificar se seu starship suporta notificações rodando `STARSHIP_LOG=debug starship module cmd_duration -d 60000` quando `show_notifications` é definido como `true`.

:::

### Variáveis

| Variável  | Exemplo  | Descrição                                 |
| --------- | -------- | ----------------------------------------- |
| duration  | `16m40s` | O tempo que levou para executar o comando |
| style\* |          | Espelha o valor da opção `style`          |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = "underwent [$duration](bold yellow)"
```

## Conda

O módulo `conda` exibe o environment atual do conda, se `$CONDA_DEFAULT_ENV` é definido.

::: tip

Isso não suprime o modificador de prompt do conda, você pode executar `conda config --set changeps1 False`.

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

| Variável    | Exemplo      | Descrição                         |
| ----------- | ------------ | --------------------------------- |
| environment | `astronauts` | O environment atual do conda      |
| symbol      |              | Espelha o valor da opção `symbol` |
| style\*   |              | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[conda]
format = "[$symbol$environment](dimmed green) "
```

## Crystal

O módulo `crystal` exibe a versão instalada atual do [Crystal](https://crystal-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `shard.yml`
- O diretório atual contem um arquivo `.cr`

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `symbol`            | `"🔮 "`                               | O simbolo usado antes de exibir a versão do crystal.                                 |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                                  |
| `detect_extensions` | `["cr"]`                             | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["shard.yml"]`                      | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `disabled`          | `false`                              | Desabilita o módulo `crystal`.                                                       |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v0.32.1` | A versão do `crystal`             |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[crystal]
format = "via [✨ $version](bold blue) "
```

## Dart

O módulo `dart` exibe a versão atual instalada do [Dart](https://dart.dev/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

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

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.8.4` | The version of `dart`             |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[dart]
format = "via [🔰 $version](bold red) "
```

## Deno

O módulo `deno` exibe a versão instalada atual do [Deno](https://deno.land/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:
- O diretório contem um arquivo `mod.ts`, `mod.js`, `deps.ts` ou `deps.js`

### Opções

| Opções              | Padrão                                       | Descrição                                                                            |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"`         | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                                  | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦕 "`                                       | Um formato de string que representa o simbolo do Deno                                |
| `detect_extensions` | `[]`                                         | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["mod.ts", "mod.js", "deps.ts", "deps.js"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                         | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"green bold"`                               | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                      | Desabilita o módulo `deno`.                                                          |

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
format = "via [🦕 $version](green bold) "
```

## Diretório

O módulo `directory` exibe o caminho do diretório atual, truncando as três pastas pai. Seu diretório será truncando na raiz do repositório git que você estiver atualmente.

Quando usar a opção de estilo fish pwd, ao invés de esconder o caminho que é truncado, você vai ver um nome encurtado de cada diretório baseado no número que você habilitar para a opção.

Por exemplo, dado `~/Dev/Nix/nixpkgs/pkgs` onde `nixpkgs` é o repositório raiz e a opção esta definida para `1`. Você verá `~/D/N/nixpkgs/pkgs`, enquanto antes seria `nixpkgs/pkgs`.

### Opções

| Opções              | Padrão                                             | Descrição                                                                |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------ |
| `truncation_length` | `3`                                                | O número de pastas pais do diretório atual que serão truncadas.          |
| `truncate_to_repo`  | `true`                                             | Seu diretório será truncado ou não para a raiz do repositório git atual. |
| `format`            | `"[$path]($style)[$read_only]($read_only_style) "` | O formato do módulo.                                                     |
| `style`             | `"bold cyan"`                                      | O estilo do módulo.                                                      |
| `disabled`          | `false`                                            | Desabilita o módulo `directory`.                                         |
| `read_only`         | `"🔒"`                                              | O simbolo que indica que o diretório atual é somente leitura.            |
| `read_only_style`   | `"red"`                                            | O estilo para o simbolo de somente leitura.                              |
| `truncation_symbol` | `""`                                               | O simbolo para prefixo de caminhos truncados. ex: "…/"                   |
| `home_symbol`       | `"~"`                                              | O simbolo para indicar o diretório home.                                 |

<details>
<summary>Este módulo tem algumas configurações avançadas que controlam como o diretório é exibido.</summary>

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
| path      | `"D:/Projects"`       | O caminho do diretório atual     |
| style\* | `"black bold dimmed"` | Espelha o valor da opção `style` |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = "…/"
```

## Docker Context

O módulo `docker_context` exibe o [Docker context](https://docs.docker.com/engine/context/working-with-contexts/) atual se não estiver definido como `default` ou se as variáveis de ambiente `DOCKER_HOST` ou `DOCKER_CONTEXT` estiverem definidas (se destinam a sobrescrever o contexto atual).

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

| Variável  | Exemplo        | Descrição                         |
| --------- | -------------- | --------------------------------- |
| context   | `test_context` | O contexto atual do docker        |
| symbol    |                | Espelha o valor da opção `symbol` |
| style\* |                | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[docker_context]
format = "via [🐋 $context](blue bold)"
```

## Dotnet

O módulo `dotnet` exibe a versão relevante do [.NET Core SDK](https://dotnet.microsoft.com/) para a pasta atual. Se o SDK foi fixado na pasta atual, a versão será exibida. Caso contrario será exibida a ultima versão instalada do SDK.

Por padrão o módulo vai apenas exibir no seu prompt quando um ou mais dos seguintes arquivos estiverem presente no diretório:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

Você também precisa do .NET Core SDK instalado para usá-lo corretamente.

Internamente, este módulo usa seu próprio mecanismo de detecção de versão. Normalmente é duas vezes mais rápido que executar `dotnet --version`, mas pode exibir uma versão errado se o projeto .NET tiver o layout de diretório incomum. Se a precisão é mais importante que velocidade, você pode desabilitar o mecanismo definindo `heuristic = false` nas opções do modulo.

O módulo também vai exibir o Target Framework Moniker (<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-framework-versions>) quando o diretório atual tiver o arquivo csproj.

### Opções

| Opções              | Padrão                                                                                                  | Descrição                                                                            |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )(🎯 $tfm )]($style)"`                                                           | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                                                                                             | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `".NET "`                                                                                               | O simbolo usado antes de exibir a versão do dotnet.                                  |
| `heuristic`         | `true`                                                                                                  | Usa a versão de detecção rápida do starship snappy.                                  |
| `detect_extensions` | `["csproj", "fsproj", "xproj"]`                                                                         | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["global.json", "project.json", "Directory.Build.props", "Directory.Build.targets", "Packages.props"]` | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                                                                    | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold blue"`                                                                                           | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                                                                 | Desabilita o módulo `dotnet`.                                                        |

### Variáveis

| Variável  | Exemplo          | Descrição                         |
| --------- | ---------------- | --------------------------------- |
| version   | `v3.1.201`       | A versão do sdk `dotnet`          |
| tfm       | `netstandard2.0` | O framework alvo do projeto atual |
| symbol    |                  | Espelha o valor da opção `symbol` |
| style\* |                  | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[dotnet]
symbol = "🥅 "
style = "green"
heuristic = false
```

## Elixir

O módulo `elixir` exibe a versão instalada do [Elixir](https://elixir-lang.org/) e [Erlang/OTP](https://erlang.org/doc/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `mix.exs`.

### Opções

| Opções              | Padrão                                                      | Descrição                                                                            |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | O formato do módulo elixir.                                                          |
| `version_format`    | `"v${raw}"`                                                 | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💧 "`                                                      | O simbolo usado antes de exibir a versão do Elixir/Erlang.                           |
| `detect_extensions` | `[]`                                                        | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["mix.exs"]`                                               | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                                        | Quais pastas devem ativar este módulo.                                               |
| `style`             | `"bold purple"`                                             | O estilo do módulo.                                                                  |
| `disabled`          | `false`                                                     | Desabilita o módulo `elixir`.                                                        |

### Variáveis

| Variável    | Exemplo | Descrição                         |
| ----------- | ------- | --------------------------------- |
| version     | `v1.10` | A versão do `elixir`              |
| otp_version |         | A versão otp do `elixir`          |
| symbol      |         | Espelha o valor da opção `symbol` |
| style\*   |         | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[elixir]
symbol = "🔮 "
```

## Elm

O módulo `elm` exibe a versão instalada do [Elm](https://elm-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

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

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v0.19.1` | A versão do `elm`                 |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
[elm]
format = "via [ $version](cyan bold) "
```

## Variáveis de Ambiente

O módulo `env_var` exibe o valor atual de uma variável de ambiente selecionada. O módulo vai exibir somente se algumas das condições a seguir for atendida:

- A opção de configuração da `variable` corresponde a uma variável existente
- A configuração `variable` não está definida, mas a `default` está


::: tip Varias variáveis podem ser exibidas usando um `.`. (Veja o exemplo) se a configuração `variable` não é definida, o módulo irá exibir o valor da variável após o caractere `.`.

Exemplo: a configuração a seguir irá mostrar o valor da variável de ambiente USER
```toml
# ~/.config/starship.toml

[env_var.USER]
default = "unknown user"
```
:::

### Opções

| Opções     | Padrão                         | Descrição                                                                      |
| ---------- | ------------------------------ | ------------------------------------------------------------------------------ |
| `symbol`   |                                | O simbolo usado antes de exibir o valor da variável.                           |
| `variable` |                                | A variável de ambiente a ser exibida.                                          |
| `default`  |                                | O valor padrão para exibir quando a variável selecionada não estiver definida. |
| `format`   | `"with [$env_value]($style) "` | O formato do módulo.                                                           |
| `disabled` | `false`                        | Desabilita o módulo `env_var`.                                                 |

### Variáveis

| Variável  | Exemplo                                     | Descrição                               |
| --------- | ------------------------------------------- | --------------------------------------- |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | O valor de ambiente da opção `variable` |
| symbol    |                                             | Espelha o valor da opção `symbol`       |
| style\* | `black bold dimmed`                         | Espelha o valor da opção `style`        |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[env_var]
variable = "SHELL"
default = "unknown shell"
```

Exibindo múltiplas variáveis de ambiente:
```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = "SHELL"
default = "unknown shell"
[env_var.USER]
default = "unknown user"
```

## Erlang

O módulo de `erlang` exibe a versão atual instalada do [Erlang/OTP](https://erlang.org/doc/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- O diretório atual contem um arquivo `rebar.config`.
- O diretório atual contem um arquivo `erlang.mk`.

### Opções

| Opções              | Padrão                               | Descrição                                                                            |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------------------ |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                 |
| `version_format`    | `"v${raw}"`                          | O formato da versão. As variáveis disponíveis são `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `" "`                               | O simbolo usado antes de exibir a versão do erlang.                                  |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                                  |
| `detect_extensions` | `[]`                                 | Quais extensões devem ativar este módulo.                                            |
| `detect_files`      | `["rebar.config", "elang.mk"]`       | Quais nomes de arquivos devem ativar este módulo.                                    |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                               |
| `disabled`          | `false`                              | Desabilita o módulo `erlang`.                                                        |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v22.1.3` | A versão do `erlang`              |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[erlang]
format = "via [e $version](bold red) "
```

## Google Cloud (`gcloud`)

The `gcloud` module shows the current configuration for [`gcloud`](https://cloud.google.com/sdk/gcloud) CLI. This is based on the `~/.config/gcloud/active_config` file and the `~/.config/gcloud/configurations/config_{CONFIG NAME}` file and the `CLOUDSDK_CONFIG` env var.

### Opções

| Opções           | Padrão                                                     | Descrição                                                    |
| ---------------- | ---------------------------------------------------------- | ------------------------------------------------------------ |
| `format`         | `'on [$symbol$account(@$domain)(\($region\))]($style) '` | O formato do módulo.                                         |
| `symbol`         | `"☁️  "`                                                   | O simbolo usado antes de exibir o perfil atual do GCP.       |
| `region_aliases` |                                                            | Tabela de aliases de região para exibir além do nome do GCP. |
| `style`          | `"bold blue"`                                              | O estilo do módulo.                                          |
| `disabled`       | `false`                                                    | Desabilita o módulo `gcloud`.                                |

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

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

O módulo `git_branch` exibe o branch ativo do repositório no diretório atual.

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
| symbol        |          | Espelha o valor da opção `symbol`                                                                 |
| style\*     |          | Espelha o valor da opção `style`                                                                  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[git_branch]
symbol = "🌱 "
truncation_length = 4
truncation_symbol = ""
```

## Git commit

O módulo `git_commit` exibe o hash do comiit atual e também a tag (se existir) do repositório no diretório atual.

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

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = "🔖 "
```

## Git State

O módulo `git_state` vai exibir nos diretorios que fazem parte de um repositorio git e onde existe uma operação em progresso, como: _REBASING_, _BISECTING_, etc. Se houver informação de progresso (e.x: REBASING 3/10). esta informação será exibida também.

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

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[git_state]
format = '[\($state( $progress_current of $progress_total)\)]($style) '
cherry_pick = "[🍒 PICKING](bold red)"
```

## Git Metrics

O módulo `git_metrics` vai exibir o número de adições e exclusões de linhas no repositório git atual.

::: tip

Este módulo é desabilitado por padrão. Para ativa-lo, defina `disabled` para `false` no seu arquivo de configuração.

:::

### Opções

| Opções          | Padrão                                                   | Descrição                              |
| --------------- | -------------------------------------------------------- | -------------------------------------- |
| `added_style`   | `"bold green"`                                           | O estilo para a contagem de adições.   |
| `deleted_style` | `"bold red"`                                             | O estilo para a contagem de exclusões. |
| `format`        | `'[+$added]($added_style) [-$deleted]($deleted_style) '` | O formato do módulo.                   |
| `disabled`      | `true`                                                   | Desabilita o módulo `git_metrics`.     |

### Variáveis

| Variável          | Exemplo | Descrição                               |
| ----------------- | ------- | --------------------------------------- |
| added             | `1`     | O número atual de linhas adicionadas    |
| deleted           | `2`     | O número atual de linhas excluidas      |
| added_style\*   |         | Espelha o valor da opção `added_style`  |
| deleted_style\* |         | Espelha o valor da opção`deleted_style` |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

| Opções       | Padrão                                          | Descrição                           |
| ------------ | ----------------------------------------------- | ----------------------------------- |
| `format`     | `'([\[$all_status$ahead_behind\]]($style) )'` | The default format for `git_status` |
| `conflicted` | `"="`                                           | This branch has merge conflicts.    |
| `ahead`      | `"⇡"`                                           | The format of `ahead`               |
| `behind`     | `"⇣"`                                           | The format of `behind`              |
| `diverged`   | `"⇕"`                                           | The format of `diverged`            |
| `up_to_date` | `""`                                            | The format of `up_to_date`          |
| `untracked`  | `"?"`                                           | The format of `untracked`           |
| `stashed`    | `"$"`                                           | The format of `stashed`             |
| `modified`   | `"!"`                                           | The format of `modified`            |
| `staged`     | `"+"`                                           | The format of `staged`              |
| `renamed`    | `"»"`                                           | The format of `renamed`             |
| `deleted`    | `"✘"`                                           | The format of `deleted`             |
| `style`      | `"bold red"`                                    | O estilo do módulo.                 |
| `disabled`   | `false`                                         | Disables the `git_status` module.   |

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

\*: Essa variável só pode ser usada como parte de uma string de estilo

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
untracked = "🤷‍"
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

## Golang

The `golang` module shows the currently installed version of [Golang](https://golang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `go.mod` file
- The current directory contains a `go.sum` file
- The current directory contains a `glide.yaml` file
- The current directory contains a `Gopkg.yml` file
- The current directory contains a `Gopkg.lock` file
- The current directory contains a `.go-version` file
- The current directory contains a `Godeps` directory
- The current directory contains a file with the `.go` extension

### Opções

| Opções              | Padrão                                                                         | Descrição                                                                 |
| ------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                           | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                                                                    | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐹 "`                                                                         | A format string representing the symbol of Go.                            |
| `detect_extensions` | `["go"]`                                                                       | Which extensions should trigger this module.                              |
| `detect_files`      | `["go.mod", "go.sum", "glide.yaml", "Gopkg.yml", "Gopkg.lock", ".go-version"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `["Godeps"]`                                                                   | Which folders should trigger this module.                                 |
| `style`             | `"bold cyan"`                                                                  | O estilo do módulo.                                                       |
| `disabled`          | `false`                                                                        | Disables the `golang` module.                                             |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v1.12.1` | The version of `go`               |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[golang]
format = "via [🏎💨 $version](bold cyan) "
```

## Helm

The `helm` module shows the currently installed version of [Helm](https://helm.sh/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `helmfile.yaml` file
- The current directory contains a `Chart.yaml` file

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["helmfile.yaml", "Chart.yaml"]`    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                    |
| `symbol`            | `"⎈ "`                               | A format string representing the symbol of Helm.                          |
| `style`             | `"bold white"`                       | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `helm` module.                                               |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v3.1.1` | The version of `helm`             |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format =  "on [$hostname](bold red) "
trim_at = ".companyname.com"
disabled = false
```

## Java

The `java` module shows the currently installed version of [Java](https://www.oracle.com/java/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `.deps.edn`, `project.clj`, or `build.boot` file
- The current directory contains a file with the `.java`, `.class`, `.gradle`, `.jar`, `.clj`, or `.cljc` extension

### Opções

| Opções              | Padrão                                                                                                    | Descrição                                                                 |
| ------------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"`                                                                  | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                                                                                               | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["java", "class", "gradle", "jar", "cljs", "cljc"]`                                                      | Which extensions should trigger this module.                              |
| `detect_files`      | `["pom.xml", "build.gradle.kts", "build.sbt", ".java-version", ".deps.edn", "project.clj", "build.boot"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                      | Quais pastas devem ativar este módulo.                                    |
| `symbol`            | `"☕ "`                                                                                                    | A format string representing the symbol of Java                           |
| `style`             | `"red dimmed"`                                                                                            | O estilo do módulo.                                                       |
| `disabled`          | `false`                                                                                                   | Disables the `java` module.                                               |

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| version   | `v14`   | The version of `java`             |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[java]
symbol = "🌟 "
```

## Jobs

The `jobs` module shows the current number of jobs running. The module will be shown only if there are background jobs running. The module will show the number of jobs running if there are at least 2 jobs, or more than the `number_threshold` config value, if it exists. The module will show a symbol if there is at least 1 job, or more than the `symbol_threshold` config value, if it exists. You can set both values to 0 in order to *always* show the symbol and number of jobs, even if there are 0 jobs running.

The default functionality is:

- 0 jobs -> Nothing is shown.
- 1 job -> `symbol` is shown.
- 2 jobs or more -> `symbol` + `number` are shown.

::: warning

This module is not supported on tcsh and nu.

:::

::: warning

The `threshold` option is deprecated, but if you want to use it, the module will show the number of jobs running if there is more than 1 job, or more than the `threshold` config value, if it exists. If `threshold` is set to 0, then the module will also show when there are 0 jobs running.

:::

### Opções

| Opções             | Padrão                        | Descrição                                                                |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`\*    | `1`                           | Show number of jobs if exceeded.                                         |
| `symbol_threshold` | `1`                           | Show `symbol` if the job count is at least `symbol_threshold`.           |
| `number_threshold` | `2`                           | Show the number of jobs if the job count is at least `number_threshold`. |
| `format`           | `"[$symbol$number]($style) "` | O formato do módulo.                                                     |
| `symbol`           | `"✦"`                         | The string used to represent the `symbol` variable.                      |
| `style`            | `"bold blue"`                 | O estilo do módulo.                                                      |
| `disabled`         | `false`                       | Disables the `jobs` module.                                              |
 \*: This option is deprecated, please use the 

`number_threshold` and `symbol_threshold` options instead.

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| number    | `1`     | The number of jobs                |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

- The current directory contains a `Project.toml` file
- The current directory contains a `Manifest.toml` file
- The current directory contains a file with the `.jl` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["jl"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Project.toml", "Manifest.toml"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                    |
| `symbol`            | `"ஃ "`                               | A format string representing the symbol of Julia.                         |
| `style`             | `"bold purple"`                      | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `julia` module.                                              |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.4.0` | The version of `julia`            |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[julia]
symbol = "∴ "
```

## Kotlin

The `kotlin` module shows the currently installed version of [Kotlin](https://kotlinlang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `.kt` or a `.kts` file

### Opções

| Opções              | Padrão                               | Descrição                                                                     |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                          |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`     |
| `detect_extensions` | `["kt", "kts"]`                      | Which extensions should trigger this module.                                  |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                                   |
| `detect_folders`    | `[]`                                 | Quais pastas devem ativar este módulo.                                        |
| `symbol`            | `"🅺 "`                               | A format string representing the symbol of Kotlin.                            |
| `style`             | `"bold blue"`                        | O estilo do módulo.                                                           |
| `kotlin_binary`     | `"kotlin"`                           | Configures the kotlin binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `kotlin` module.                                                 |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v1.4.21` | The version of `kotlin`           |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

Este módulo é desabilitado por padrão. Para ativa-lo, defina `disabled` para `false` no seu arquivo de configuração.

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
| symbol    |                      | Espelha o valor da opção `symbol`        |
| style\* |                      | Espelha o valor da opção `style`         |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

#### Regex Matching

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

## Quebra de linha

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

- The current directory contains a `.lua-version` file
- The current directory contains a `lua` directory
- The current directory contains a file with the `.lua` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                  |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                       |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`  |
| `symbol`            | `"🌙 "`                               | A format string representing the symbol of Lua.                            |
| `detect_extensions` | `["lua"]`                            | Which extensions should trigger this module.                               |
| `detect_files`      | `[".lua-version"]`                   | Which filenames should trigger this module.                                |
| `detect_folders`    | `["lua"]`                            | Which folders should trigger this module.                                  |
| `style`             | `"bold blue"`                        | O estilo do módulo.                                                        |
| `lua_binary`        | `"lua"`                              | Configures the lua binary that Starship executes when getting the version. |
| `disabled`          | `false`                              | Disables the `lua` module.                                                 |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v5.4.0` | The version of `lua`              |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[lua]
format = "via [🌕 $version](bold blue) "
```

## Uso de memória

The `memory_usage` module shows current system memory and swap usage.

By default the swap usage is displayed if the total system swap is non-zero.

::: tip

Este módulo é desabilitado por padrão. Para ativa-lo, defina `disabled` para `false` no seu arquivo de configuração.

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
| symbol           | `🐏`           | Espelha o valor da opção `symbol`                                  |
| style\*        |               | Espelha o valor da opção `style`                                   |

\*: This variable can only be used as a part of a style string \*\*: The SWAP file information is only displayed if detected on the current system

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
| `truncation_symbol` | `"…"`                            | O simbolo usado para indicar que o nome braço foi truncado.                                  |
| `disabled`          | `true`                           | Disables the `hg_branch` module.                                                             |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| branch    | `master` | The active mercurial branch       |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

- The current directory contains a `nim.cfg` file
- The current directory contains a file with the `.nim` extension
- The current directory contains a file with the `.nims` extension
- The current directory contains a file with the `.nimble` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo                                                       |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"👑 "`                               | The symbol used before displaying the version of Nim.                     |
| `detect_extensions` | `["nim", "nims", "nimble"]`          | Which extensions should trigger this module.                              |
| `detect_files`      | `["nim.cfg"]`                        | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold yellow"`                      | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `nim` module.                                                |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.2.0` | The version of `nimc`             |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| state     | `pure`  | The state of the nix-shell        |
| name      | `lorri` | The name of the nix-shell         |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

- The current directory contains a `package.json` file
- The current directory contains a `.node-version` file
- The current directory contains a `.nvmrc` file
- The current directory contains a `node_modules` directory
- The current directory contains a file with the `.js`, `.mjs` or `.cjs` extension
- The current directory contains a file with the `.ts` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                                             |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                                                  |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch`                             |
| `symbol`            | `" "`                               | A format string representing the symbol of Node.js.                                                   |
| `detect_extensions` | `["js", "mjs", "cjs", "ts"]`         | Which extensions should trigger this module.                                                          |
| `detect_files`      | `["package.json", ".node-version"]`  | Which filenames should trigger this module.                                                           |
| `detect_folders`    | `["node_modules"]`                   | Which folders should trigger this module.                                                             |
| `style`             | `"bold green"`                       | O estilo do módulo.                                                                                   |
| `disabled`          | `false`                              | Disables the `nodejs` module.                                                                         |
| `not_capable_style` | `bold red`                           | The style for the module when an engines property in package.json does not match the Node.js version. |

### Variáveis

| Variável  | Exemplo    | Descrição                         |
| --------- | ---------- | --------------------------------- |
| version   | `v13.12.0` | The version of `node`             |
| symbol    |            | Espelha o valor da opção `symbol` |
| style\* |            | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[nodejs]
format = "via [🤖 $version](bold green) "
```

## OCaml

The `ocaml` module shows the currently installed version of [OCaml](https://ocaml.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a file with `.opam` extension or `_opam` directory
- The current directory contains a `esy.lock` directory
- The current directory contains a `dune` or `dune-project` file
- The current directory contains a `jbuild` or `jbuild-ignore` file
- The current directory contains a `.merlin` file
- The current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension

### Opções

| Opções                    | Padrão                                                                     | Descrição                                                                 |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`                  | `"via [$symbol($version )(\($switch_indicator$switch_name\) )]($style)"` | The format string for the module.                                         |
| `version_format`          | `"v${raw}"`                                                                | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`                  | `"🐫 "`                                                                     | The symbol used before displaying the version of OCaml.                   |
| `global_switch_indicator` | `""`                                                                       | The format string used to represent global OPAM switch.                   |
| `local_switch_indicator`  | `"*"`                                                                      | The format string used to represent local OPAM switch.                    |
| `detect_extensions`       | `["opam", "ml", "mli", "re", "rei"]`                                       | Which extensions should trigger this module.                              |
| `detect_files`            | `["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"]`           | Which filenames should trigger this module.                               |
| `detect_folders`          | `["_opam", "esy.lock"]`                                                    | Which folders should trigger this module.                                 |
| `style`                   | `"bold yellow"`                                                            | O estilo do módulo.                                                       |
| `disabled`                | `false`                                                                    | Disables the `ocaml` module.                                              |

### Variáveis

| Variável         | Exemplo      | Descrição                                                         |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | The version of `ocaml`                                            |
| switch_name      | `my-project` | The active OPAM switch                                            |
| switch_indicator |              | Mirrors the value of `indicator` for currently active OPAM switch |
| symbol           |              | Espelha o valor da opção `symbol`                                 |
| style\*        |              | Espelha o valor da opção `style`                                  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| cloud     | `corp`  | The current OpenStack cloud       |
| project   | `dev`   | The current OpenStack project     |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[openstack]
format = "on [$symbol$cloud(\\($project\\))]($style) "
style = "bold yellow"
symbol = "☁️ "
```

## Package Version

The `package` module is shown when the current directory is the repository for a package, and shows its current version. The module currently supports `npm`, `nimble`, `cargo`, `poetry`, `composer`, `gradle`, `julia`, `mix` and `helm` packages.

- [**npm**](https://docs.npmjs.com/cli/commands/npm) – The `npm` package version is extracted from the `package.json` present in the current directory
- [**cargo**](https://doc.rust-lang.org/cargo/) – The `cargo` package version is extracted from the `Cargo.toml` present in the current directory
- [**nimble**](https://github.com/nim-lang/nimble) - The `nimble` package version is extracted from the `*.nimble` file present in the current directory with the `nimble dump` command
- [**poetry**](https://python-poetry.org/) – The `poetry` package version is extracted from the `pyproject.toml` present in the current directory
- [**python**](https://www.python.org) - The `python` package version is extracted from the `setup.cfg` present in the current directory
- [**composer**](https://getcomposer.org/) – The `composer` package version is extracted from the `composer.json` present in the current directory
- [**gradle**](https://gradle.org/) – The `gradle` package version is extracted from the `build.gradle` present
- [**julia**](https://docs.julialang.org/en/v1/stdlib/Pkg/) - The package version is extracted from the `Project.toml` present
- [**mix**](https://hexdocs.pm/mix/) - The `mix` package version is extracted from the `mix.exs` present
- [**helm**](https://helm.sh/docs/helm/helm_package/) - The `helm` chart version is extracted from the `Chart.yaml` present
- [**maven**](https://maven.apache.org/) - The `maven` package version is extracted from the `pom.xml` present
- [**meson**](https://mesonbuild.com/) - The `meson` package version is extracted from the `meson.build` present
- [**vlang**](https://vlang.io) - The `vlang` package version is extracted from the `v.mod` present

> ⚠️ The version being shown is that of the package whose source code is in your current directory, not your package manager.

### Opções

| Opções            | Padrão                            | Descrição                                                                 |
| ----------------- | --------------------------------- | ------------------------------------------------------------------------- |
| `format`          | `"is [$symbol$version]($style) "` | O formato do módulo.                                                      |
| `symbol`          | `"📦 "`                            | The symbol used before displaying the version the package.                |
| `version_format`  | `"v${raw}"`                       | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `style`           | `"bold 208"`                      | O estilo do módulo.                                                       |
| `display_private` | `false`                           | Enable displaying version for packages marked as private.                 |
| `disabled`        | `false`                           | Disables the `package` module.                                            |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v1.0.0` | The version of your package       |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[package]
format = "via [🎁 $version](208 bold) "
```

## Perl

The `perl` module shows the currently installed version of [Perl](https://www.perl.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `Makefile.PL` or `Build.PL` file
- The current directory contains a `cpanfile` or `cpanfile.snapshot` file
- The current directory contains a `META.json` file or `META.yml` file
- The current directory contains a `.perl-version` file
- The current directory contains a `.pl`, `.pm` or `.pod`

### Opções

| Opções              | Padrão                                                                                                   | Descrição                                                                 |
| ------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`                                                                     | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                                                                                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐪 "`                                                                                                   | The symbol used before displaying the version of Perl                     |
| `detect_extensions` | `["pl", "pm", "pod"]`                                                                                    | Which extensions should trigger this module.                              |
| `detect_files`      | `["Makefile.PL", "Build.PL", "cpanfile", "cpanfile.snapshot", "META.json", "META.yml", ".perl-version"]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                                                                                     | Which folders should trigger this module.                                 |
| `style`             | `"bold 149"`                                                                                             | O estilo do módulo.                                                       |
| `disabled`          | `false`                                                                                                  | Disables the `perl` module.                                               |

### Variáveis

| Variável  | Exemplo   | Descrição                         |
| --------- | --------- | --------------------------------- |
| version   | `v5.26.1` | The version of `perl`             |
| symbol    |           | Espelha o valor da opção `symbol` |
| style\* |           | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml

[perl]
format = "via [🦪 $version]($style) "
```

## PHP

The `php` module shows the currently installed version of [PHP](https://www.php.net/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `composer.json` file
- The current directory contains a `.php-version` file
- The current directory contains a `.php` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐘 "`                               | The symbol used before displaying the version of PHP.                     |
| `detect_extensions` | `["php"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `["composer.json", ".php-version"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"147 bold"`                         | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `php` module.                                                |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v7.3.8` | The version of `php`              |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[php]
format = "via [🔹 $version](147 bold) "
```

## PureScript

The `purescript` module shows the currently installed version of [PureScript](https://www.purescript.org/) version. Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `spago.dhall` file
- The current directory contains a file with the `.purs` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"<=> "`                       | The symbol used before displaying the version of PureScript.              |
| `detect_extensions` | `["purs"]`                           | Which extensions should trigger this module.                              |
| `detect_files`      | `["spago.dhall"]`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold white"`                       | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `purescript` module.                                         |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `0.13.5` | The version of `purescript`       |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

- The current directory contains a `.python-version` file
- The current directory contains a `Pipfile` file
- The current directory contains a `__init__.py` file
- The current directory contains a `pyproject.toml` file
- The current directory contains a `requirements.txt` file
- The current directory contains a `setup.py` file
- The current directory contains a `tox.ini` file
- The current directory contains a file with the `.py` extension.
- A virtual environment is currently activated

### Opções

| Opções               | Padrão                                                                                                       | Descrição                                                                              |
| -------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- |
| `format`             | `'via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)'`                                  | O formato do módulo.                                                                   |
| `version_format`     | `"v${raw}"`                                                                                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch`              |
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
| symbol       | `"🐍 "`          | Espelha o valor da opção `symbol`          |
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

The `rlang` module shows the currently installed version of R. The module will be shown if any of the following conditions are met:

- The current directory contains a file with the `.R` extension.
- The current directory contains a file with the `.Rd` extension.
- The current directory contains a file with the `.Rmd` extension.
- The current directory contains a file with the `.Rproj` extension.
- The current directory contains a file with the `.Rsx` extension.
- The current directory contains a `.Rprofile` file
- The current directory contains a `.Rproj.user` folder

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"📐"`                                | A format string representing the symbol of R.                             |
| `style`             | `"blue bold"`                        | O estilo do módulo.                                                       |
| `detect_extensions` | `["R", "Rd", "Rmd", "Rproj", "Rsx"]` | Quais extensões devem acionar este módulo                                 |
| `detect_files`      | `[".Rprofile"]`                      | []                                                                        |
| `detect_folders`    | `[".Rproj.user"]`                    | Quais pastas devem ativar este módulo                                     |
| `disabled`          | `false`                              | Disables the `r` module.                                                  |

### Variáveis

| Variável | Exemplo       | Descrição                         |
| -------- | ------------- | --------------------------------- |
| version  | `v4.0.5`      | The version of `R`                |
| symbol   |               | Espelha o valor da opção `symbol` |
| style    | `"blue bold"` | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml

[rlang]
format = "with [📐 $version](blue bold) "
```

## Red

By default the `red` module shows the currently installed version of [Red](https://www.red-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a file with `.red` or `.reds` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🔺 "`                               | A format string representing the symbol of Red.                           |
| `detect_extensions` | `["red"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"red bold"`                         | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `red` module.                                                |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.5.1` | The version of `red`              |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[red]
symbol = "🔴 "
```

## Ruby

By default the `ruby` module shows the currently installed version of [Ruby](https://www.ruby-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Gemfile` file
- The current directory contains a `.ruby-version` file
- The current directory contains a `.rb` file

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💎 "`                               | A format string representing the symbol of Ruby.                          |
| `detect_extensions` | `["rb"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Gemfile", ".ruby-version"]`       | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `ruby` module.                                               |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v2.5.1` | The version of `ruby`             |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[ruby]
symbol = "🔺 "
```

## Rust

By default the `rust` module shows the currently installed version of [Rust](https://www.rust-lang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Cargo.toml` file
- The current directory contains a file with the `.rs` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🦀 "`                               | A format string representing the symbol of Rust                           |
| `detect_extensions` | `["rs"]`                             | Which extensions should trigger this module.                              |
| `detect_files`      | `["Cargo.toml"]`                     | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold red"`                         | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `rust` module.                                               |

### Variáveis

| Variável  | Exemplo           | Descrição                         |
| --------- | ----------------- | --------------------------------- |
| version   | `v1.43.0-nightly` | The version of `rustc`            |
| symbol    |                   | Espelha o valor da opção `symbol` |
| style\* |                   | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[rust]
format = "via [⚙️ $version](red bold)"
```

## Scala

The `scala` module shows the currently installed version of [Scala](https://www.scala-lang.org/). Por padrão o módulo vai exibir se uma das condições a seguir for atendida:

- The current directory contains a `build.sbt`, `.scalaenv` or `.sbtenv` file
- The current directory contains a file with the `.scala` or `.sbt` extension
- The current directory contains a directory named `.metals`

### Opções

| Opções              | Padrão                                   | Descrição                                                                 |
| ------------------- | ---------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [${symbol}(${version} )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                              | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `detect_extensions` | `["sbt", "scala"]`                       | Which extensions should trigger this module.                              |
| `detect_files`      | `[".scalaenv", ".sbtenv", "build.sbt"]`  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".metals"]`                            | Quais pastas devem ativar este módulo.                                    |
| `symbol`            | `"🆂 "`                                   | A format string representing the symbol of Scala.                         |
| `style`             | `"red dimmed"`                           | O estilo do módulo.                                                       |
| `disabled`          | `false`                                  | Disables the `scala` module.                                              |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `2.13.5` | The version of `scala`            |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[scala]
symbol = "🌟 "
```

## Shell

The `shell` module shows an indicator for currently used shell.

::: tip

Este módulo é desabilitado por padrão. Para ativa-lo, defina `disabled` para `false` no seu arquivo de configuração.

:::

### Opções

| Opções                 | Padrão       | Descrição                                                    |
| ---------------------- | ------------ | ------------------------------------------------------------ |
| `bash_indicator`       | `bsh`        | Uma string para representar o bash.                          |
| `fish_indicator`       | `fsh`        | Uma string usada para representar o fish.                    |
| `zsh_indicator`        | `zsh`        | A format string used to represent zsh.                       |
| `powershell_indicator` | `psh`        | A format string used to represent powershell.                |
| `ion_indicator`        | `ion`        | A format string used to represent ion.                       |
| `elvish_indicator`     | `esh`        | A format string used to represent elvish.                    |
| `tcsh_indicator`       | `tsh`        | A format string used to represent tcsh.                      |
| `xonsh_indicator`      | `xsh`        | A format string used to represent xonsh.                     |
| `unknown_indicator`    |              | The default value to be displayed when the shell is unknown. |
| `format`               | `$indicator` | O formato do módulo.                                         |
| `disabled`             | `true`       | Disables the `shell` module.                                 |

### Variáveis

| Variável  | Padrão | Descrição                                                  |
| --------- | ------ | ---------------------------------------------------------- |
| indicator |        | Mirrors the value of `indicator` for currently used shell. |

### Exemplos

```toml
# ~/.config/starship.toml

[shell]
fish_indicator = ""
powershell_indicator = "_"
unknown_indicator = "mystery shell"
disabled = false
```

## SHLVL

The `shlvl` module shows the current `SHLVL` ("shell level") environment variable, if it is set to a number and meets or exceeds the specified threshold.

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

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| shlvl     | `3`     | The current value of `SHLVL`      |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

| Variável  | Exemplo      | Descrição                         |
| --------- | ------------ | --------------------------------- |
| env       | `centos.img` | The current Singularity image     |
| symbol    |              | Espelha o valor da opção `symbol` |
| style\* |              | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[singularity]
format = '[📦 \[$env\]]($style) '
```

## Status

The `status` module displays the exit code of the previous command. The module will be shown only if the exit code is not `0`.

::: tip

Este módulo é desabilitado por padrão. Para ativa-lo, defina `disabled` para `false` no seu arquivo de configuração.

:::

::: warning This module is not supported on elvish and nu shell. :::

### Opções

| Opções                  | Padrão                                                                               | Descrição                                               |
| ----------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------- |
| `format`                | `"[$symbol$status]($style) "`                                                        | The format of the module                                |
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
| `pipestatus_separator`  | `|`                                                                                  | The symbol that separate in pipe program exit codes     |
| `pipestatus_format`     | `\\[$pipestatus\\] => [$symbol$common_meaning$signal_name$maybe_int]($style)` | The format of the module when the command is a pipeline |
| `disabled`              | `true`                                                                               | Disables the `status` module.                           |

### Variáveis

| Variável       | Exemplo | Descrição                                                                                   |
| -------------- | ------- | ------------------------------------------------------------------------------------------- |
| status         | `127`   | The exit code of the last command                                                           |
| int            | `127`   | The exit code of the last command                                                           |
| common_meaning | `ERROR` | Meaning of the code if not a signal                                                         |
| signal_number  | `9`     | Signal number corresponding to the exit code, only if signalled                             |
| signal_name    | `KILL`  | Name of the signal corresponding to the exit code, only if signalled                        |
| maybe_int      | `7`     | Contains the exit code number when no meaning has been found                                |
| pipestatus     |         | Rendering of in pipeline programs's exit codes, this is only available in pipestatus_format |
| symbol         |         | Espelha o valor da opção `symbol`                                                           |
| style\*      |         | Espelha o valor da opção `style`                                                            |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

## Swift

By default the `swift` module shows the currently installed version of [Swift](https://swift.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `Package.swift` file
- The current directory contains a file with the `.swift` extension

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"🐦 "`                               | A format string representing the symbol of Swift                          |
| `detect_extensions` | `["swift"]`                          | Which extensions should trigger this module.                              |
| `detect_files`      | `["Package.swift"]`                  | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"bold 202"`                         | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `swift` module.                                              |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v5.2.4` | The version of `swift`            |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

- The current directory contains a `.terraform` folder
- Current directory contains a file with the `.tf` or `.hcl` extensions

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol$workspace]($style) "` | The format string for the module.                                         |
| `version_format`    | `"v${raw}"`                          | A versão formatada. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"💠"`                                | A format string shown before the terraform workspace.                     |
| `detect_extensions` | `["tf", "hcl"]`                      | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[".terraform"]`                     | Which folders should trigger this module.                                 |
| `style`             | `"bold 105"`                         | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `terraform` module.                                          |

### Variáveis

| Variável  | Exemplo    | Descrição                         |
| --------- | ---------- | --------------------------------- |
| version   | `v0.12.24` | The version of `terraform`        |
| workspace | `default`  | The current Terraform workspace   |
| symbol    |            | Espelha o valor da opção `symbol` |
| style\* |            | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

Este módulo é desabilitado por padrão. Para ativa-lo, defina `disabled` para `false` no seu arquivo de configuração.

:::

### Opções

| Opções            | Padrão                  | Descrição                                                                                                                          |
| ----------------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | `"at [$time]($style) "` | The format string for the module.                                                                                                  |
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
| horário   | `13:08:10` | The current time.                |
| style\* |            | Espelha o valor da opção `style` |

\*: Essa variável só pode ser usada como parte de uma string de estilo

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

- The current user is root
- The current user isn't the same as the one that is logged in
- The user is currently connected as an SSH session
- The variable `show_always` is set to true

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

- The current directory contains a `Vagrantfile` file

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"⍱ "`                               | A format string representing the symbol of Vagrant.                       |
| `detect_extensions` | `[]`                                 | Which extensions should trigger this module.                              |
| `detect_files`      | `["Vagrantfile"]`                    | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |
| `style`             | `"cyan bold"`                        | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `vagrant` module.                                            |

### Variáveis

| Variável  | Exemplo          | Descrição                         |
| --------- | ---------------- | --------------------------------- |
| version   | `Vagrant 2.2.10` | The version of `Vagrant`          |
| symbol    |                  | Espelha o valor da opção `symbol` |
| style\* |                  | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[vagrant]
format = "via [⍱ $version](bold white) "
```

## VLang

The `vlang` module shows you your currently installed version of V. By default the module will be shown if any of the following conditions are met:
- The current directory contains a file with `.v` extension
- The current directory contains a `v.mod`, `vpkg.json` or `.vpkg-lock.json` file

### Opções

| Opções              | Padrão                                       | Descrição                                                                 |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"`         | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                                  | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"V "`                                       | A format string representing the symbol of V                              |
| `detect_extensions` | `["v"]`                                      | Which extensions should trigger this module.                              |
| `detect_files`      | `["v.mod", "vpkg.json", ".vpkg-lock.json" ]` | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                         | Which folders should trigger this module.                                 |
| `style`             | `"blue bold"`                                | O estilo do módulo.                                                       |
| `disabled`          | `false`                                      | Disables the `vlang` module.                                              |

### Variáveis

| Variável  | Exemplo | Descrição                         |
| --------- | ------- | --------------------------------- |
| version   | `v0.2`  | The version of `v`                |
| symbol    |         | Espelha o valor da opção `symbol` |
| style\* |         | Espelha o valor da opção `style`  |

### Exemplo

```toml
# ~/.config/starship.toml
[v]
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

| Variável  | Exemplo                                     | Descrição                         |
| --------- | ------------------------------------------- | --------------------------------- |
| repo      | `dotfiles` if in a VCSH repo named dotfiles | O nome do repositório ativo       |
| symbol    |                                             | Espelha o valor da opção `symbol` |
| style\* | `black bold dimmed`                         | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[vcsh]
format = "[🆅 $repo](bold blue) "
```

## Zig

By default the the `zig` module shows the currently installed version of [Zig](https://ziglang.org/). The module will be shown if any of the following conditions are met:

- The current directory contains a `.zig` file

### Opções

| Opções              | Padrão                               | Descrição                                                                 |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------- |
| `format`            | `"via [$symbol($version )]($style)"` | O formato do módulo.                                                      |
| `version_format`    | `"v${raw}"`                          | The version format. Available vars are `raw`, `major`, `minor`, & `patch` |
| `symbol`            | `"↯ "`                               | The symbol used before displaying the version of Zig.                     |
| `style`             | `"bold yellow"`                      | O estilo do módulo.                                                       |
| `disabled`          | `false`                              | Disables the `zig` module.                                                |
| `detect_extensions` | `["zig"]`                            | Which extensions should trigger this module.                              |
| `detect_files`      | `[]`                                 | Which filenames should trigger this module.                               |
| `detect_folders`    | `[]`                                 | Which folders should trigger this module.                                 |

### Variáveis

| Variável  | Exemplo  | Descrição                         |
| --------- | -------- | --------------------------------- |
| version   | `v0.6.0` | The version of `zig`              |
| symbol    |          | Espelha o valor da opção `symbol` |
| style\* |          | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

### Exemplo

```toml
# ~/.config/starship.toml

[zig]
symbol = "⚡️ "
```

## Comandos Personalizados

The `custom` modules show the output of some arbitrary commands.

These modules will be shown if any of the following conditions are met:

- The current directory contains a file whose name is in `files`
- The current directory contains a directory whose name is in `directories`
- The current directory contains a file whose extension is in `extensions`
- The `when` command returns 0
- The current Operating System (std::env::consts::OS) matchs with `os` field if defined.

::: tip

Multiple custom modules can be defined by using a `.`.

:::

::: tip

The order in which custom modules are shown can be individually set by including `${custom.foo}` in the top level `format` (as it includes a dot, you need to use `${...}`). By default, the `custom` module will simply show all custom modules in the order they were defined.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252) contains examples of custom modules. If you have an interesting example not covered there, feel free to share it there!

:::

### Opções

| Opções        | Padrão                          | Descrição                                                                                                                                                                        |
| ------------- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`     |                                 | The command whose output should be printed. The command will be passed on stdin to the shell.                                                                                    |
| `when`        |                                 | A shell command used as a condition to show the module. The module will be shown if the command returns a `0` status code.                                                       |
| `shell`       |                                 | [See below](#custom-command-shell)                                                                                                                                               |
| `descrição`   | `"<custom module>"`       | The description of the module that is shown when running `starship explain`.                                                                                                     |
| `files`       | `[]`                            | The files that will be searched in the working directory for a match.                                                                                                            |
| `directories` | `[]`                            | The directories that will be searched in the working directory for a match.                                                                                                      |
| `extensions`  | `[]`                            | The extensions that will be searched in the working directory for a match.                                                                                                       |
| `symbol`      | `""`                            | The symbol used before displaying the command output.                                                                                                                            |
| `style`       | `"bold green"`                  | O estilo do módulo.                                                                                                                                                              |
| `format`      | `"[$symbol($output )]($style)"` | O formato do módulo.                                                                                                                                                             |
| `disabled`    | `false`                         | Desabilita este módulo `custom`.                                                                                                                                                 |
| `os`          |                                 | Nome do sistema operacional onde módulo sera exibido (unix, linux, macos, windows, ... ) [Veja os possíveis valores](https://doc.rust-lang.org/std/env/consts/constant.OS.html). |

### Variáveis

| Variável  | Descrição                         |
| --------- | --------------------------------- |
| output    | A saída do comando no `shell`     |
| symbol    | Espelha o valor da opção `symbol` |
| style\* | Espelha o valor da opção `style`  |

\*: Essa variável só pode ser usada como parte de uma string de estilo

#### Custom command shell

`shell` accepts a non-empty list of strings, where:

- The first string is the path to the shell to use to execute the command.
- Other following arguments are passed to the shell.

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
command = "echo foo"  # shows output of command
files = ["foo"]       # can specify filters
when = """ test "$HOME" == "$PWD" """
format = " transcending [$output]($style)"

[custom.time]
command = "time /T"
files = ["*.pst"]
shell = ["pwsh.exe", "-NoProfile", "-Command", "-"]
```
