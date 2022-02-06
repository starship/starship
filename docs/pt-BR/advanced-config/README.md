# Configuração avançada

Ainda que Starship seja um shell versátil, às vezes você precisará fazer algumas outras coisas além de editar o arquivo `starship.toml`. Esta página detalha algumas das técnicas de configuração avançadas utilizadas no starship.

::: atenção

As configurações nesta seção estão sujeitas a alterações em futuras versões do Starship.

:::

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

Nota: O prompt direito é uma única linha após o local de entrada. Para alinhar à direita os módulos acima a linha de entrada em um prompt de várias linhas, consulte o [módulo fill](/config/#fill).

`right_format` é atualmente compatível com os seguintes shells: elvish, fish, zsh, xonsh, cmd.

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

Starship pode definir o prompt de continuação usando a opção `continuation_prompt`. The default prompt is `"[∙](bright-black) "`.

Note: `continuation_prompt` should be set to a literal string without any variables.

Note: Continuation prompts are only available in the following shells:

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

Style strings are a list of words, separated by whitespace. The words are not case sensitive (i.e. `bold` and `BoLd` are considered the same string). Each word can be one of the following:

- `negrito`
- `itálico`
- `sublinhado`
- `escurecido`
- `invertido`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

onde `<color>` é um especificador de cor (discutido abaixo). `fg:<color>` e `<color>` atualmente fazem a mesma coisa, embora isso possa mudar no futuro. `inverted` swaps the background and foreground colors. The order of words in the string does not matter.

The `none` token overrides all other tokens in a string if it is not part of a `bg:` specifier, so that e.g. `fg:red none fg:blue` will still create a string with no styling. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

- One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
- A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- A number between 0-255. This specifies an [8-bit ANSI Color Code](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
