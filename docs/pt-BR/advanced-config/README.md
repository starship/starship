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

## Comandos personalizados de pre-prompt e pre-execution no Bash

O Bash não possui uma estrutura formal para os hooks preexec/precmd como a maioria dos outros shells. Por esse motivo, é difícil fornecer hooks totalmente customizáveis no `bash`. No entanto, Starship te oferece uma capacidade limitada de inserir suas próprias funções na processo de prompt-rendering:

- Para rodar uma função personalizada antes do prompt iniciar, defina uma nova função e atribua seu nome para `starship_precmd_user_func`. Por exemplo, para desenhar um foguete antes do prompt iniciar você faria

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Para rodar uma função personalizada antes que um comando seja executado, você pode usar [`DEBUG` como mecanismo de armadilha](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). No entanto, você **deve** prender o sinal de DEBUG _antes_ de inicializar o Starship! O Starship consegue preservar o valor da armadilha DEBUG, mas se a armadilha for substituída depois do starship iniciar, algumas funções iram quebrar.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Captura o DEBUG *antes* de executar a nave estelar
set -o functrace
eval $(starship init bash)
set +o functrace
```

## Comandos personalizados de pre-prompt e pre-execution no PowerShell

O PowerShell não possui uma estrutura formal para os hooks preexec/precmd como a maioria dos outros shells. Por esse motivo, é difícil fornecer hooks totalmente customizáveis no `powershell`. No entanto, Starship te oferece uma capacidade limitada de inserir suas próprias funções na processo de prompt-rendering:

Crie uma função chamada `Invoke-Starship-PreCommand`

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Altera o título da janela

Alguns shell prompts iram alterar o titulo da janela automaticamente para você (e.x: para espelhar o diretório atual). Fish faz isso por padrão. Starship não faz isso, mas é bastante simples adicionar essa funcionalidade para `bash`, `zsh`, `cmd` ou `powershell`.

Primeiro, defina uma função de alteração de titulo de janela (é o mesmo para bash e zsh):

```bash
function set_win_title(){
    echo -ne "\033]0; YOUR_WINDOW_TITLE_HERE \007"
}
```

Você pode usar variáveis para customizar o titulo (`$USER`, `$HOSTNAME`, e `$PWD` são escolhas populares).

No `bash`, defina esta função como a precedente da função starship:

```bash
starship_precmd_user_func="set_win_title"
```

No `zsh`, adicione no array `precmd_functions`:

```bash
precmd_functions+=(set_win_title)
```

Se você gostar do resultado, adicione esta linha ao seu arquivo de configuração de shell (`~/.bashrc` or `~/.zshrc`) para torna-lo permanente.

Por exemplo, se você quiser exibir seu diretório atual no seu titulo de aba do terminal, adicione o seguinte snippet ao seu `~/.bashrc` ou `~/.zshrc`:

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
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (&starship init powershell)
```

## Ativando o Prompt Direito

Alguns shells suportam um prompt no lado direito que renderiza na mesma linha do input. Starship consegue definir o conteúdo do prompt direito usando a opção `right_format`. Qualquer módulo pode ser usado no `format` é suportado o `right_format`. A variável `$all` só irá alterar os módulos que não usaram de forma explicita o `format` ou `right_format`.

Nota: O prompt direito é uma linha única seguindo o local de entrada. To right align modules above the input line in a multi-line prompt, see the [`fill` module](/config/#fill).

`right_format` é atualmente compatível com os seguintes shells: elvish, fish, zsh, xonsh, cmd.

### Exemplo

```toml
# ~/.config/starship.toml

# Um prompt esquerdo minimo 
format = """$character"""

# Move o resto do prompt para direita
right_format = """$all"""
```

Gera um prompt parecido com o seguinte:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## Prompt de Continução

Algumas shells suportam prompt de continuação juntamento com o prompt normal. Este prompt é renderizado em vez do prompt normal quando o usuário insere um comando incompleto (Como um parentese ou aspas a esquerda).

O Starship consegue definir o prompt de continuação usando a opção `continuation_prompt`. O prompt padrão é `"[∙](bright-black) "`.

Nota: `continuation_prompt` deve ser definido como uma string literal sem variáveis.

Nota: Prompt de continuação são disponíveis apenas nos shells a seguir:

- `bash`
- `zsh`
- `PowerShell`

### Exemplo

```toml
#~/.config/starship.toml

# Um prompt de continuação que mostra duas setas
continuation_prompt = "▶▶"
```

## Estilo dos textos

Estilo de strings são uma lista de palavras, separadas por espaço. As palavras não são case sensitive (ou seja `bold` e `BoLd` são consideradas iguais). Cada palavra pode ser uma das seguintes:

- `bold`
- `italic`
- `underline`
- `dimmed`
- `inverted`
- `bg:<color>`
- `fg:<color>`
- `<color>`
- `none`

onde `<color>` é uma especialista de cores (discutido abaixo). `fg:<color>` e `<color>` atualmente fazem a mesma coisa, isto deve mudar no futuro. `inverted` troca as cores de background e foreground. A ordem de palavras na string não importa.

O token `none` substitui todos os outros tokens na string se ele não fizer parte de um `bg:` especificado que seja, por exemplo `fg:red none fg:blue` ainda criará uma string sem estilo. `bg:none` define a cor padrão de background então `fg:red bg:none` é equivalente a `red` ou `fg:red` e `bg:green fg:red bg:none` é equivalente a `fg:red` ou`red`. Pode se transformar em um erro ao usar `none` em um conjunto de outros tokens no futuro.

Um especialista em cores pode ser um dos seguintes:

- Uma das cores padrão do terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Você pode, opcionalmente, prefixar esses com `bright-` para obter a versão brilhante/clara (por exemplo, `bright-white`).
- Um `#` seguido por um número de seis dígitos hexadecimais. Isto especifica um [Código RGB em formato hexadecimal](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- Um número entre 0-255. Este especifica um [Código de Cor ANSI 8 bits](https://i.stack.imgur.com/KTSQa.png).

Se múltiplas cores forem especificadas para foreground/background, a ultima da string que terá prioridade.
