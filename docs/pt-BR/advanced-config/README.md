# Configuração avançada

Ainda que Starship seja um shell versátil, às vezes você precisará fazer algumas outras coisas além de editar o arquivo `starship.toml`. Esta página detalha algumas das técnicas de configuração avançadas utilizadas no starship.

::: atenção

As configurações nesta seção estão sujeitas a alterações em futuras versões do Starship.

:::

## TransientPrompt no PowerShell

It is possible to replace the previous-printed prompt with a custom string. This is useful in cases where all the prompt information is not always needed. To enable this, run `Enable-TransientPrompt` in the shell session. To make it permanent, put this statement in your `$PROFILE`. Transience can be disabled on-the-fly with `Disable-TransientPrompt`.

By default, the left side of input gets replaced with `>`. To customize this, define a new function called `Invoke-Starship-TransientFunction`. For example, to display Starship's `character` module here, you would do

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## TransientPrompt and TransientRightPrompt in Cmd

Clink allows you to replace the previous-printed prompt with custom strings. This is useful in cases where all the prompt information is not always needed. To enable this, run `clink set prompt.transient <value>` where \<value\> can be one of:

- `always`: always replace the previous prompt
- `same_dir`: replace the previous prompt only if the working directory is same
- `off`: do not replace the prompt (i.e. turn off transience)

You need to do this only once. Make the following changes to your `starship.lua` to customize what gets displayed on the left and on the right:

- By default, the left side of input gets replaced with `>`. To customize this, define a new function called `starship_transient_prompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display Starship's `character` module here, you would do

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. This function receives the current prompt as a string that you can utilize. For example, to display the time at which the last command was started here, you would do

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## TransientPrompt and TransientRightPrompt in Fish

It is possible to replace the previous-printed prompt with a custom string. This is useful in cases where all the prompt information is not always needed. To enable this, run `enable_transience` in the shell session. To make it permanent, put this statement in your `~/.config/fish/config.fish`. Transience can be disabled on-the-fly with `disable_transience`.

Note that in case of Fish, the transient prompt is only printed if the commandline is non-empty, and syntactically correct.

- By default, the left side of input gets replaced with a bold-green `❯`. To customize this, define a new function called `starship_transient_prompt_func`. For example, to display Starship's `character` module here, you would do

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- By default, the right side of input is empty. To customize this, define a new function called `starship_transient_rprompt_func`. For example, to display the time at which the last command was started here, you would do

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
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
   $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (& starship init powershell)
```

## Ativando o Prompt Direito

Alguns shells suportam um prompt direito que é renderizado na mesma linha que a entrada. Starship pode definir o conteúdo do prompt correto usando a opção `right_format`. Qualquer módulo que pode ser usado no `format` também é compatível com `right_format`. A variável `$all` conterá apenas módulos não usado explicitamente em `format` ou `right_format`.

Nota: O prompt direito é uma única linha após o local de entrada. Para alinhar módulos à direita acima da linha de entrada em um prompt de várias linhas, consulte o [módulo `fill`](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh, xonsh, cmd, nushell.

Note: Nushell 0.71.0 or later is required

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

Starship pode definir o prompt de continuação usando a opção `continuation_prompt`. O prompt padrão é `"[∙](bright-black) "`.

Nota: `continuation_prompt` deve ser definido como uma string literal sem nenhuma variável.

Nota: os prompts de continuação estão disponíveis apenas nos seguintes shells:

- `bash`
- `zsh`
- `PowerShell`

### Exemplo

```toml
# ~/.config/starship.toml

# Um prompt de continuação que exibe duas setas preenchidas
continuation_prompt = "▶▶"
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

onde `<color>` é um especificador de cor (discutido abaixo). `fg:<color>` e `<color>` atualmente fazem a mesma coisa, embora isso possa mudar no futuro. `inverted` troca as cores de fundo e primeiro plano. A ordem das palavras na string não importa.

O token `none` substitui todos os outros tokens em uma string se não fizer parte de um especificador `bg:`, de modo que, ex., `fg:red none fg:blue` ainda criará uma string sem estilo. `bg:none` define o plano de fundo para a cor padrão para que `fg:red bg:none` seja equivalente a `red` ou `fg:red` e `bg:green fg:red bg:none` também é equivalente a `fg:red` ou `red`. Pode ser um erro usar `none` em conjunto com outros tokens no futuro.

Um especificador de cor pode ser um dos seguintes:

- Uma das cores padrão do terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Você pode, opcionalmente, prefixar esses com `bright-` para obter a versão brilhante/clara (por exemplo, `bright-white`).
- Um `#` seguido por um número hexadecimal de seis dígitos. Especifica um [Código hexadecimal de cor RGB](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Um número entre 0-255. Especifica um [Código de cores ANSI de 8 bits](https://i.stack.imgur.com/KTSQa.png).

Se várias cores forem especificadas para primeiro plano/plano de fundo, a última na string terá prioridade.

Not every style string will be displayed correctly by every terminal. In particular, the following known quirks exist:

- Many terminals disable support for `blink` by default
- `hidden` is [not supported on iTerm](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` is not supported by the default macOS Terminal.app
