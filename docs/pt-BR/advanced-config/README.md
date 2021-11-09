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

## Custom pre-prompt and pre-execution Commands in PowerShell

PowerShell does not have a formal preexec/precmd framework like most other shells. Because of this, it is difficult to provide fully customizable hooks in `powershell`. No entanto, Starship te oferece uma capacidade limitada de inserir suas próprias funções na processo de prompt-rendering:

Create a function named `Invoke-Starship-PreCommand`

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

You can also set a similar output with PowerShell by creating a function named `Invoke-Starship-PreCommand`.

```powershell
# edit $PROFILE
function Invoke-Starship-PreCommand {
  $host.ui.Write("`e]0; PS> $env:USERNAME@$env:COMPUTERNAME`: $pwd `a")
}

Invoke-Expression (&starship init powershell)
```

## Ativando o Prompt Direito

Alguns shells suportam um prompt no lado direito que renderiza na mesma linha do input. Starship consegue definir o conteúdo do prompt direito usando a opção `right_format`. Qualquer módulo pode ser usado no `format` é suportado o `right_format`. A variável `$all` só irá alterar os módulos que não usaram de forma explicita o `format` ou `right_format`.

Note: The right prompt is a single line following the input location. To right align modules above the input line in a multi-line prompt, see the [fill module](/config/#fill).

`right_format` is currently supported for the following shells: elvish, fish, zsh.

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

 - Um dos padrões de cores no terminal: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Você pode de forma opcional prefixar com `bright-` para obter uma versão mais brilhante/clara (ex `bright-white`).
 - Um `#` seguido por um número de seis dígitos hexadecimais. Isto especifica um [Código RGB em formato hexadecimal](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Um número entre 0-255. Este especifica um [Código de Cor ANSI 8 bits](https://i.stack.imgur.com/KTSQa.png).

Se múltiplas cores forem especificadas para foreground/background, a ultima da string que terá prioridade.
