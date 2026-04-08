# Configuração avançada

Embora o Starship seja um shell versátil, às vezes é necessário fazer mais do que editar o `starship.toml` para que ele realize certas funções. Esta página detalha algumas das técnicas de configuração avançadas utilizadas no starship.

> [!AVISO] As configurações desta seção estão sujeitas a mudanças em futuras versões do Starship.

## TransientPrompt no PowerShell

É possível substituir o prompt anteriormente impresso por uma string personalisada. Isto é útil quando todas as informações do prompt não são nescessárias sempre. Para habilitar isto, execute `Enable-TransientPrompt` na sessão do shell. Para ser permanente, adicione esta declaração no seu `$PROFILE`. A transição pode ser desativada com `Disable-TransientPrompt`.

Por padrão, o lado esquerdo da entrada é substituida por  `>`. Para personalizar isso defina uma nova função chamada `Invoke-Starship-TransientFunction`. Por exemplo, para mostrar o módulo de caractere do ` Starship's`aqui, você faria

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt e TransientRightPrompt em Cmd

Clink permite você substituir o prompt anteriormente impresso com strings personalizadas. Isto é útil quando todas as informações do prompt não são nescessárias sempre. Para habilitar isso, execute  `clink set prompt.transient <value>` onde \<value\> pode ser um dos:

- `always`: sempre substitui o prompt anterior
- `same_dir`: substitui o prompt anterior apenas se o diretório de trabalho for o mesmo
- `off`: não substitui o prompt (ou seja, desliga a transição)

Você precisa fazer isso apenas uma vez. Faça as seguintes alterações ao seu `starship.lua` para personalizar o que é exibido à esquerda e à direita:

- Por padrão, o lado esquerdo da entrada é substituida por  `>`. Para personalizar isso, defina uma nova função chamada `starship_transient_prompt_func`. Esta função recebe o prompt atual como uma string que você pode utilizar. Por exemplo, para mostrar o módulo de caractere do ` Starship's`aqui, você faria

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- Por padrão, o lado direito da entrada é vazio. Para personalizar isso, defina uma nova função chamada `starship_transient_rprompt_func`. Esta função recebe a prompt atual de como uma string que você pode utilizar. Por exemplo, para exibir o momento em que o último comando foi iniciado, você faria

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt e TransientRightPrompt no Fish

É possível substituir o prompt anteriormente impresso por uma string personalisada. Isto é útil quando todas as informações do prompt não são nescessárias sempre. Para habilitar isso, execute `enable_transience` na sessão do shell. Para torná-lo permanente, coloque esta declaração no seu `~/.config/fish/config.fish`. Transição pode ser desativada com `disable_transience`.

Observe que, no caso do Fish, o prompt transitório só será impresso se a linha de comando não estiver vazia, e sintaticamente correta.

- Por padrão, o lado esquerdo da entrada é substituído por um símbolo `❯` verde. Para personalizar isso, defina uma nova função chamada `starship_transient_prompt_func`. Por exemplo, para mostrar o módulo de caractere do ` Starship's`aqui, você faria

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

- Por padrão, o lado direito da entrada é vazio. Para personalizar isso, defina uma nova função chamada `starship_transient_rprompt_func`. Por exemplo, para exibir o momento em que o último comando foi iniciado, você faria

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## TransientPrompt e TransientRightPrompt em Bash

O framework [Ble.sh](https://github.com/akinomyoga/ble.sh) na versão 0.4 ou superior permite substituir o prompt exibido anteriormente por strings personalizadas. Isso é útil em casos onde todas as informações do prompt não são sempre necessárias. Para habilitar isso, coloque em `~/.bashrc` `bleopt prompt_ps1_transient=<value>`:

O \<value\> aqui é uma lista separada por dois pontos de `always`, `same-dir` e `trim`. Quando `prompt_ps1_final` está vazio e a opção `prompt_ps1_transient` tem um \ não vazio <value\>, o prompt especificado por `PS1` é apagado ao sair da linha de comando atual. Se \<value\> contiver um campo `trim`, apenas a última linha da multilinha `PS1`  será preservada e as outras linhas serão apagadas. Caso contrário, a linha de comando será redesenhada como se `PS1=` fosse especificado. Quando um campo `same-dir` está contido em \<value\> e o diretório atual é diferente do diretório final da linha de comando anterior, esta opção `prompt_ps1_transient` é ignorada.

Faça as seguintes alterações no seu `~/.blerc` (ou em `~/.config/blesh/init.sh`) para personalizar o que é exibido à esquerda e à direita:

- Para personalizar o que o lado esquerdo da entrada será substituído, configure a opção `prompt_ps1_final` Ble.sh. Por exemplo, para exibir o módulo `caractere` do Starship aqui, você deve fazer

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- Para personalizar o que o lado direito de entrada será substituído, configure a opção `prompt_rps1_final` Ble.sh. Por exemplo, para exibir o momento em que o último comando foi iniciado, você deve fazer

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Comandos personalizados de pré-prompt e pré-execução no Cmd

O Clink fornece APIs extremamente flexíveis para executar comandos pré-prompt e pré-execução em Cmd shell. É bastante simples de usar com o Starship. Faça as seguintes alterações no seu arquivo `starship.lua` conforme suas necessidades:

- Para executar uma função personalizada logo antes do prompt ser inicializado, defina um novo função chamada `starship_preprompt_user_func`. Esta função recebe o prompt atual como uma string que você pode utilizar. Por exemplo, para exibir um foguete antes do prompt, você faria

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- Para executar uma função personalizada logo antes de um comando ser executado, defina um novo função chamada `starship_precmd_user_func`. Esta função recebe a linha de comando atual como uma string que você pode utilizar. Por exemplo, para imprimir o comando que está prestes a ser executado, você faria

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Comandos personalizados de pré-prompt e pré-execução no Bash

Bash não possui uma estrutura formal pré-prompt/pré-execução como a maioria dos outros shells. Por causa disso, é difícil fornecer ganchos totalmente personalizáveis no `bash`. No entanto, Starship te oferece uma capacidade limitada de inserir suas próprias funções na processo de prompt-rendering:

- Para executar uma função personalizada logo antes de o prompt ser inicializado, define uma nova função e, em seguida, atribui seu nome a `starship_precmd_user_func`. Por exemplo, para exibir um foguete antes do prompt, você faria

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Para executar uma função personalizada logo antes de um comando ser executado, você pode usar o [`DEBUG` mecanismo de captura](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). No entanto, você **deve** capturar o sinal DEBUG _antes_ de inicializar o Starship! Starship pode preservar o valor da captura do DEBUG, mas se a captura for substituída após a inicialização do starship, algumas funcionalidades serão interrompidas.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Captura o DEBUG *antes* de executar a nave estelar
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Comandos personalizados de pré-prompt e pré-execução no PowerShell

PowerShell não possui uma estrutura formal pré-prompt/pré-execução como a maioria dos outros shells. Por causa disso, é difícil fornecer ganchos totalmente personalizáveis no `powershell`. No entanto, Starship te oferece uma capacidade limitada de inserir suas próprias funções na processo de prompt-rendering:

Crie uma função chamada `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Alterar Título da Janela

Alguns prompts do shell alterarão automaticamente o título da janela para você (ex., para refletir no seu diretório de trabalho). Fish ainda faz isso por padrão. Starship não faz isso, mas é bastante simples adicionar essa funcionalidade para `bash`, `zsh`, `cmd` ou `powershell`.

Primeiro, defina uma função de mudança de título da janela (idêntica em bash e zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

Você pode usar variáveis para personalizar este título (`$USER`, `$HOSTNAME` e `$PWD` são escolhas populares).

No `bash`, defina esta função como a função precmd da nave estelar:

```bash
starship_precmd_user_func="set_win_title"
```

No `zsh`, adicione isso ao array `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

Se você gostar do resultado, adicione estas linhas ao seu arquivo de configuração do shell (`~/.bashrc` ou `~/.zshrc`) para torná-lo permanente.

Por exemplo, se você deseja exibir seu diretório atual no título da guia do terminal, adicione o seguinte trecho ao seu `~/.bashrc` ou `~/.zshrc`:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

Para Cmd, você pode alterar o título da janela usando a função `starship_preprompt_user_func`.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

Você também pode definir uma saída semelhante com o PowerShell criando uma função chamada `Invoke-Starship-PreCommand`.

```powershell
# editar $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Ativando o Prompt Direito

Alguns shells suportam um prompt direito que é renderizado na mesma linha que a entrada. Starship pode definir o conteúdo do prompt correto usando a opção `right_format`. Qualquer módulo que pode ser usado no `format` também é compatível com `right_format`. A variável `$all` conterá apenas módulos não usado explicitamente em `format` ou `right_format`.

Nota: O prompt direito é uma única linha após o local de entrada. Para alinhar à direita os módulos acima da linha de entrada em um prompt de várias linhas, consulte o [módulo`fill`](../config/#fill).

`right_format` é atualmente suportado para os seguintes shells: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Nota: O framework [Ble.sh](https://github.com/akinomyoga/ble.sh) na versão 0.4 ou superior deve estar instalado para utilizar o prompt direito no bash.

### Exemplo

```toml
# ~/.config/starship.toml

# Um prompt mínimo à esquerda
format = """$character"""

# movw o restante do prompt para a direita
right_format = """$all"""
```

Produz um prompt como o seguinte:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## Prompt de Continuação

Alguns shells suportam um prompt de continuação junto com o prompt normal. Esse prompt é renderizado em vez do prompt normal quando o usuário insere uma instrução incompleta (como um único parêntese esquerdo ou aspas).

Starship pode definir o prompt de continuação usando a opção `continuation_prompt`. O prompt padrão é  `'[∙](bright-black) '`.

Nota: `continuation_prompt` deve ser definido como uma string literal sem nenhuma variável.

Nota: os prompts de continuação estão disponíveis apenas nos seguintes shells:

- `bash`
- `zsh`
- `PowerShell`

### Exemplo

```toml
# ~/.config/starship.toml

# Um prompt de continuação que exibe duas setas preenchidas
continuation_prompt = '▶▶ '
```

## Linha de estado para Claude Code

Starship suporta a exibição de uma linha de estado personalizada quando executando dentro do Claude Code, a linha de comando da Anthropic's para programação interativa com Claude. Essa linha de estado fornece informação em tempo real sobre sua sessão Claude, incluindo o modelo sendo usado, o uso da janela de contexto e custos da sessão.

Para mais informações sobre o recurso de linha de estado do Claude Code, consulte a [documentação da linha de estado do Claude Code](https://code.claude.com/docs/en/statusline).

### Configuração

Para usar o Starship como sua linha de estado do Claude Code:

1. Run `/statusline` in Claude Code and ask it to configure Starship, or manually add the following to your `.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "starship statusline claude-code"
  }
}
```

2. Customize the statusline appearance in your `~/.config/starship.toml` (see [Configuration](#configuration) below)

### Overview

When invoked with `starship statusline claude-code`, Starship receives Claude Code session data via stdin and renders a statusline using a dedicated profile named `claude-code`.

The profile includes three specialized modules:

- `claude_model`: Displays the current Claude model being used
- `claude_context`: Shows context window usage with a visual gauge
- `claude_cost`: Displays session cost and statistics

The default profile format is:

```toml
[profiles]
claude-code = "$claude_model$git_branch$claude_context$claude_cost"
```

### Configuração

You can customize the Claude Code statusline by modifying the `claude-code` profile and individual module configurations in your `~/.config/starship.toml`:

```toml
# ~/.config/starship.toml

# Customize the claude-code profile
[profiles]
claude-code = "$claude_model$claude_context$claude_cost"

# Configure individual modules
[claude_model]
format = "[$symbol$model]($style) "
symbol = "🤖 "
style = "bold blue"

[claude_context]
format = "[$gauge $percentage]($style) "
gauge_width = 10

[claude_cost]
format = "[$symbol$cost]($style) "
symbol = "💰 "
```

### Claude Model

The `claude_model` module displays the current Claude model being used in the session.

#### Opções

| Opções          | Padrão                       | Descrição                                                                                 |
| --------------- | ---------------------------- | ----------------------------------------------------------------------------------------- |
| `format`        | `'[$symbol$model]($style) '` | O formato do módulo.                                                                      |
| `symbol`        | `'🤖 '`                       | The symbol shown before the model name.                                                   |
| `style`         | `'bold blue'`                | O estilo do módulo.                                                                       |
| `model_aliases` | `{}`                         | Map of model IDs or display names to shorter aliases. Checks ID first, then display name. |
| `disabled`      | `false`                      | Disables the `claude_model` module.                                                       |

#### Variáveis

| Variável  | Exemplo             | Descrição                             |
| --------- | ------------------- | ------------------------------------- |
| model     | `Claude 3.5 Sonnet` | The display name of the current model |
| model_id  | `claude-3-5-sonnet` | The model ID                          |
| symbol    |                     | Espelha o valor da opção `symbol`     |
| style\* |                     | Espelha o valor da opção `style`      |

\*: Essa variável só pode ser usada como parte de uma string de estilo

#### Exemplos

```toml
# ~/.config/starship.toml

# Basic customization
[claude_model]
format = "on [$symbol$model]($style) "
symbol = "🧠 "
style = "bold cyan"

# Using model aliases for vendor-specific model names
# You can alias by model ID or display name
[claude_model.model_aliases]
# Alias by vendor model ID (e.g. AWS Bedrock)
"global.anthropic.claude-sonnet-4-5-20250929-v1:0" = "Sonnet 4.5"
# Alias by display name
"Claude Sonnet 4.5 (Vendor Proxy)" = "Sonnet"
```

### Claude Context

The `claude_context` module displays context window usage as a percentage and visual gauge. The style automatically changes based on configurable thresholds.

#### Opções

| Opções                 | Padrão                            | Descrição                                          |
| ---------------------- | --------------------------------- | -------------------------------------------------- |
| `format`               | `'[$gauge $percentage]($style) '` | O formato do módulo.                               |
| `symbol`               | `''`                              | The symbol shown before the gauge.                 |
| `gauge_width`          | `5`                               | The width of the gauge in characters.              |
| `gauge_full_symbol`    | `'█'`                             | The symbol used for filled segments of the gauge.  |
| `gauge_partial_symbol` | `'▒'`                             | The symbol used for partial segments of the gauge. |
| `gauge_empty_symbol`   | `'░'`                             | The symbol used for empty segments of the gauge.   |
| `display`              | [veja abaixo](#display)           | Threshold and style configurations.                |
| `disabled`             | `false`                           | Disables the `claude_context` module.              |

##### Display

The `display` option is an array of objects that define thresholds and styles for different usage levels. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Opções      | Padrão       | Descrição                                                                |
| ----------- | ------------ | ------------------------------------------------------------------------ |
| `threshold` | `0.0`        | The minimum context windows usage percentage to match this configuration |
| `style`     | `bold green` | The value of `style` if this display configuration is matched            |
| `hidden`    | `false`      | Hide this module if this the configuration is matched.                   |

```toml
[[claude_context.display]]
threshold = 0
hidden = true

[[claude_context.display]]
threshold = 30
style = "bold green"

[[claude_context.display]]
threshold = 60
style = "bold yellow"

[[claude_context.display]]
threshold = 80
style = "bold red"
```

#### Variáveis

| Variável                     | Exemplo | Descrição                                             |
| ---------------------------- | ------- | ----------------------------------------------------- |
| gauge                        | `██▒░░` | Visual representation of context usage                |
| percentage                   | `65%`   | Context usage as a percentage                         |
| input_tokens                 | `45.2k` | Total input tokens in conversation                    |
| output_tokens                | `12.3k` | Total output tokens in conversation                   |
| curr_input_tokens          | `5.1k`  | Input tokens from most recent API call                |
| curr_output_tokens         | `1.2k`  | Output tokens from most recent API call               |
| curr_cache_creation_tokens | `1.5k`  | Cache creation tokens from most recent API call       |
| curr_cache_read_tokens     | `23.4k` | Cache read tokens from most recent API call           |
| total_tokens                 | `200k`  | Total context window size                             |
| symbol                       |         | Espelha o valor da opção `symbol`                     |
| style\*                    |         | Mirrors the style from the matching display threshold |

\*: Essa variável só pode ser usada como parte de uma string de estilo

#### Exemplos

**Minimal gauge-only display**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$gauge]($style) "
gauge_width = 10
```

**Detailed token information**

```toml
# ~/.config/starship.toml

[claude_context]
format = "[$percentage ($input_tokens in / $output_tokens out)]($style) "
```

**Custom gauge symbols**

```toml
# ~/.config/starship.toml

[claude_context]
gauge_full_symbol = "▰"
gauge_partial_symbol = ""
gauge_empty_symbol = "▱"
gauge_width = 10
format = "[$gauge]($style) "
```

**Custom thresholds**

```toml
# ~/.config/starship.toml

[[claude_context.display]]
threshold = 0
style = "bold green"

[[claude_context.display]]
threshold = 50
style = "bold yellow"

[[claude_context.display]]
threshold = 75
style = "bold orange"

[[claude_context.display]]
threshold = 90
style = "bold red"
```

### Claude Cost

The `claude_cost` module displays the total cost of the current Claude Code session in USD. Like `claude_context`, it supports threshold-based styling.

#### Opções

| Opções     | Padrão                             | Descrição                           |
| ---------- | ---------------------------------- | ----------------------------------- |
| `format`   | `'[$symbol(\\$$cost)]($style) '` | O formato do módulo.                |
| `symbol`   | `'💰 '`                             | The symbol shown before the cost.   |
| `display`  | [veja abaixo](#display-1)          | Threshold and style configurations. |
| `disabled` | `false`                            | Disables the `claude_cost` module.  |

##### Display

The `display` option is an array of objects that define cost thresholds and styles. The module uses the style from the highest matching threshold or hides the module if `hidden` is `true`.

| Opções      | Padrão       | Descrição                                                     |
| ----------- | ------------ | ------------------------------------------------------------- |
| `threshold` | `0.0`        | The minimum cost in USD to match this configuration           |
| `style`     | `bold green` | The value of `style` if this display configuration is matched |
| `hidden`    | `false`      | Hide this module if this configuration is matched.            |

**Default configuration:**

```toml
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 1.0
style = "bold yellow"

[[claude_cost.display]]
threshold = 5.0
style = "bold red"
```

#### Variáveis

| Variável      | Exemplo  | Descrição                                             |
| ------------- | -------- | ----------------------------------------------------- |
| cost          | `1.23`   | Total session cost in USD (formatted to 2 decimals)   |
| duration      | `1m 30s` | Total session duration                                |
| api_duration  | `45s`    | Total API call duration                               |
| lines_added   | `1.2k`   | Total lines of code added                             |
| lines_removed | `500`    | Total lines of code removed                           |
| symbol        |          | Espelha o valor da opção `symbol`                     |
| style\*     |          | Mirrors the style from the matching display threshold |

\*: Essa variável só pode ser usada como parte de uma string de estilo

#### Exemplos

```toml
# ~/.config/starship.toml

# Cost with code change statistics
[claude_cost]
format = "[$symbol$cost (+$lines_added -$lines_removed)]($style) "

# Hide module until cost exceeds $0.10
[[claude_cost.display]]
threshold = 0.0
hidden = true

[[claude_cost.display]]
threshold = 0.10
style = "bold yellow"

[[claude_cost.display]]
threshold = 2.0
style = "bold red"

# Show duration information
[claude_cost]
format = "[$symbol$cost ($duration)]($style) "
```

## Estilo dos textos

As strings de estilo são uma lista de palavras, separadas por espaços em branco. As palavras não diferenciam maiúsculas de minúsculas (ou seja, `bold` e `BoLd` são considerados a mesma string). Cada palavra pode ser as seguintes:

- `bold`
- `italic`
- `underline`
- `dimmed`
- `inverted`
- `blink`
- `hidden`
- `strikethrough`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

onde `<color>` é um especificador de cor (discutido abaixo). `fg:<color>` e `<color>` atualmente fazem a mesma coisa, embora isso possa mudar no futuro. `<color>` também pode ser configurado para `prev_fg` ou `prev_bg`, que avaliam a cor de primeiro plano ou de fundo do item anterior, respectivamente, se disponível, ou `none` caso contrário. `inverted` troca as cores de fundo e primeiro plano. A ordem das palavras na string não importa.

O token `none` substitui todos os outros tokens em uma string se não fizer parte de um especificador `bg:`, de modo que, ex., `fg:red none fg:blue` ainda criará uma string sem estilo. `bg:none` define o plano de fundo para a cor padrão para que `fg:red bg:none` seja equivalente a `red` ou `fg:red` e `bg:green fg:red bg:none` também é equivalente a `fg:red` ou `red`. Pode ser um erro usar `none` em conjunto com outros tokens no futuro.

Um especificador de cor pode ser um dos seguintes:

- Uma das cores padrão do terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Você pode, opcionalmente, prefixar esses com `bright-` para obter a versão brilhante/clara (por exemplo, `bright-white`).
- Um `#` seguido por um número hexadecimal de seis dígitos. Especifica um [Código hexadecimal de cor RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Um número entre 0-255. Especifica um [Código de cores ANSI de 8 bits](https://i.stack.imgur.com/KTSQa.png).

Se várias cores forem especificadas para primeiro plano/plano de fundo, a última na string terá prioridade.

Nem todas os estilos de string serão exibidos corretamente em todos terminais. Em particular, existem os seguintes erros conhecidos:

- Muitos terminais desabilitam por padrão o suporte ao `blink`.
- `hidden` não é [ suportado no iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` não é suportado por padrão no aplicativo de terminal do macOS.
