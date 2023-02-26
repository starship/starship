# Perguntas Frequentes

## Qual é a configuração usada no GIF de demonstração?

- **Emulador de Terminal**: [iTerm2](https://iterm2.com/)
  - **Tema**: Minimal
  - **Esquema de Cores**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **Fonte**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **Shell**: [Fish Shell](https://fishshell.com/)
  - **Configuração**: [matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **Prompt**: [Starship](https://starship.rs/)

## Como eu consigo obter autocompletar de comando, conforme mostrado no GIF de demonstração?

O suporte de autocompletar, é provido pelo shell que você escolher. No caso da demonstração, é utilizado o [Fish Shell](https://fishshell.com/), que prove autocompletar como padrão. Se você usa Z Shell (zsh), Eu sugiro que você dê uma olhada no [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## De forma geral `format` e `<module>.disabled` fazem a mesma coisa?

Sim, ambos podem ser usados para desativar os mulos no prompt. Se seu objetivo geral é desativar modulos, `<module>.disabled` é o mais recomendado pelas seguintes razões:

- Desabilitar módulos é mais explícito do que omiti-lo através do `format`
- Modulos recém-criados serão adicionados quando o Starship for atualizado

## As documentações dizem que o Starship é cross-shell. Porque minha shell preferida não é suportada?

A forma em que o Starship foi construído, faz com que ele seja compatível com qualquer shell. O binário do starship é sem estado e shell agnóstico, então se o seu shell suporta customização de prompt e expansão de shell, Starship pode ser utilizado.

Aqui está um pequeno exemplo de como o Starship funciona com o bash:

```sh
# Recupera o status do último comando executado
STATUS=$?

# Recupera o número de jobs que estão rodando.
NUM_JOBS=$(jobs -p | wc -l)

# Define a saída do prompt para ´starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

A [implementação do Bash](https://github.com/starship/starship/blob/master/src/init/starship.bash) embutida no Starship é um pouco mais complexa para aceitar recursos avançados como o [Módulo de duração de comando](https://starship.rs/config/#command-duration) e para garantir isto o Starship é compatível com configurações pre-instaladas.

Para uma lista de flags aceitos pelo `starship prompt`, use o seguinte comando:

```sh
starship prompt --help
```

O prompt vai usar o contexto fornecido, mas nenhuma flag é obrigatória.

## Como faço para rodar o Starship em distribuições Linux com versões antigas do glibc?

Se você está tendo um erro como "_version 'GLIBC_2.18' not found (required by starship)_" quando usa o binário prebuilt (por exemplo, no CentOS 6 ou 7), você pode usar um binário compilado com `musl` em vez do `glibc`:

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## Porque eu vejo alertas de `Executing command "..." timed out.`?

O Starship executa comandos diferentes para recuperar as informações para exibir no prompt, por exemplo a versão de um programa ou o status atual do git. Para ter certeza de que o starship não travou durante uma execução destes comandos nos definimos um limite de tempo, se um comando ultrapassar este limite o starship vai parar a execução do comando e exibe o alerta acima, esse é um comportamento esperado. Este limite de tempo é configurado usando o [`command_timeout`key](/config/#prompt) então se você quiser você pode aumentar este limite. Você pode também seguir os passos para debugar para ver qual comando esta demorando e se você pode otimizar ele. Finalmente você pode definir a variável de ambiente `STARSHIP_LOG` para `error` para esconder estes alertas.

## Eu vejo símbolos que não entendo ou não esperado, o que isso significa?

Se você vê símbolos que não reconhece você pode usar `starship explain` para exibir os módulos que estão sendo mostrados no momento.

## O Starship esta fazendo algo inesperado, como eu posso debugar?

Você pode ativar os logs de debug usando a variavel de ambiente `STARSHIP_LOG`. Este logs podem ser bastantes verboso então é bastante útil usar o comando `module` se você esta tentando debugar um modulo em particular, por exemplo, se você esta tentando debugar o modulo `rust` você pode executar o seguinte comando para rastrear logs e saídas do modulo.

```sh
env STARSHIP_LOG=trace starship module rust
```

Se o starship começa a ficar lento você pode tentar usar o comando `timings` para ver se tem um modulo ou comando para culpar.

```sh
env STARSHIP_LOG=trace starship timings
```

Isto vai retornar o rastreamento do log e um detalhamento de todos os módulos que levam mais que 1ms para executar ou produzir alguma saída.

Finally if you find a bug you can use the `bug-report` command to create a GitHub issue.

```sh
starship bug-report
```

## Por que não consigo visualizar um simbolo glifo no meu prompt?

A causa mais comum é a configuração incorreta do sistema. Algumas distribuições Linux em particular não vem com suporte de fontes pronto para uso. Você deve conferir os pontos abaixo:

- Sua localização está configurada como UTF-8, como por exemplo `de_DE.UTF-8` ou `ja_JP.UTF-8`. Se `LC_ALL` não estiver configurado como UTF-8, [você deve mudar](https://www.tecmint.com/set-system-locales-in-linux/).
- Você tem uma fonte de emoji instalda. A maioria dos sistemas vem com uma fonte de emoji instalada como padrão, mas alguns não (principalmente o Arch Linux). Você pode instalar uma em seu sistema, através do gerenciador de pacotes-[noto emoji](https://www.google.com/get/noto/help/emoji/) é uma escolha popular.
- Você está usando uma [Nerd Font](https://www.nerdfonts.com/).

Para testar seu sistema, execute o comando abaixo em um terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

A primeira linha deve produzir um [emoji de cobra](https://emojipedia.org/snake/), enquanto a segunda linha deve produzir um [um simbolo de bifurcação (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

Se um dos símbolos acima falhar seu sistema ainda está configurado de forma errada. Infelizmente, obter a configuração de fontes correta as vezes é difícil. Usuários no Discord podem te ajudar. Se os dois símbolos acima exibirem de forma correta, mas você ainda continua sem visualizar no Starship, [registre um erro!](https://github.com/starship/starship/issues/new/choose)

## Como eu desinstalo o Starship?

O Starship é tão fácil de desinstalar tão como é para instalar.

1. Remova qualquer linha da configuração do seu shell (ex: `~/.bashrc`) usada para iniciar o Starship.
1. Delete o binário do Starship.

Se o Starship foi instalando usando algum gerenciador de pacotes, por favor consulte as documentações do mesmo para instruções de desinstalação.

Se o Starship foi instalado usando o script de instalação, o comando abaixo irá remover o binário:

```sh
# Localiza e exclui o binário do starship
sh -c 'rm "$(command -v 'starship')"'
```
