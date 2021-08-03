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

O token `none` substitui todos os outros tokens na string se ele não fizer parte de um `bg:` especificado que seja, por exemplo `fg:red none fg:blue` ainda criará uma string sem estilo. `bg:none` sets the background to the default color so `fg:red bg:none` is equivalent to `red` or `fg:red` and `bg:green fg:red bg:none` is also equivalent to `fg:red` or `red`. It may become an error to use `none` in conjunction with other tokens in the future.

A color specifier can be one of the following:

 - One of the standard terminal colors: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. You can optionally prefix these with `bright-` to get the bright version (e.g. `bright-white`).
 - A `#` followed by a six-digit hexadecimal number. This specifies an [RGB color hex code](https://www.w3schools.com/colors/colors_hexadecimal.asp).
 - Um número entre 0-255. Este especifica um [Código de Cor ANSI 8 bits](https://i.stack.imgur.com/KTSQa.png).

If multiple colors are specified for foreground/background, the last one in the string will take priority.
