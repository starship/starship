# Configuração avançada

Ainda que Starship se`ja um shell versátil, às vezes você precisará fazer algumas outras coisas além de editar o arquivo <code>starship.toml`. Esta página detalha algumas das configurações mais avançadas usadas em starship.

::: warning

As configurações nesta seção estão sujeitas a alterações em futuras versões do Starship.

:::

## Comandos personalizados de pre-prompt e pre-execution no Bash

O Bash não possui uma estrutura formal para os hooks preexec/precmd como a maioria dos outros shells. Por esse motivo, é difícil fornecer hooks totalmente customizáveis no `bash`. No entanto, Starship te oferece uma capacidade limitada de inserir suas próprias funções na processo de prompt-rendering:

- Para rodar uma função personalizada antes do prompt iniciar, defina uma nova função e atribua seu nome para `starship_precmd_user_func`. Por exemplo, para desenhar um foguete antes do prompt iniciar você faria

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Para rodar uma função personalizada antes que um comando seja executado, você pode usar [`DEBUG` como mecanismo de armadilha](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/). No entanto, você **deve** prender o sinal de DEBUG *antes* de inicializar o Starship! O Starship consegue preservar o valor da armadilha DEBUG, mas se a armadilha for substituída depois do starship iniciar, algumas funções iram quebrar.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Trap Inicia o debug antes de iniciar o starship
eval $(starship init bash)
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

Alguns shell prompts iram alterar o titulo da janela automaticamente para você (e.x: para espelhar o diretório atual). Fish faz isso por padrão. Starship não faz isso, mas é bastante simples adicionar esta funcionalidade ao `bash` ou `zsh`.

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

Nota: O prompt direito é uma linha única seguindo o local de entrada. Para alinhar à direita os módulos acima a linha de entrada em um prompt de várias linhas, consulte o [módulo fill](/config/#fill).

`right_format` é atualmente suportado para os seguintes shells: elvish, fish, zsh, xonsh.

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

Strings de estilo são uma lista de palavras, separadas por espaço. As palavras não são case sensitive ( ou seja `bold` e `BoLd` são considerados iguais). Cada palavra pode ser as seguintes:

  - `bold`
  - `italic`
  - `underline`
  - `dimmed`
  - `inverted`
  - `bg:<color>`
  - `fg:<color>`
  - `<color>`
  - `none`

onde `<color>` é uma cor especifica (explicado abaixo). `fg:<color>` e `<color>` atualmente fazem a mesma coisa, isto deve mudar no futuro. `inverted` troca a cor de background pela cor do foreground. A ordem das palavras não importa.

O token `none` sobrescreve qualquer outro token na string se ele não for part de um `bg:` especifico, então ex: `fg:red none fg:blue` será criado uma string sem estilo. 0>bg:none</code> define o background com a cor padrão então `fg:red bg:none` é equivalente a `red` ou `fg:red` e `bg:green fg:red bg:none` é também equivalente a `fg:red` ou `red`. No futuro pode se tornar um erro usar `none` junto de outros tokens.

Um especialista em cores pode ser os seguintes:

 - Uma das cores padrões do terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Você pode optar por adicionar o prefixo `bright-` para conseguir a versão mais clara (ex: `bright-white`).
 - Um `#` seguido por seis dígitos hexadecimais. Isto especifica um [Código RGB em hexa](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Um número entre 0-255. Este especifica um [Código de cor ANSI de 8-bit](https://i.stack.imgur.com/KTSQa.png).

Se múltiplas cores forem especificadas para foreground/background, o ultimo da string será o prioritário.
