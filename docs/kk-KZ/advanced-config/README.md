# Кеңейтілген баптаулар (Advanced Configuration)

Starship өте әмбебап қабықша промпты болғанымен, кейбір арнайы міндеттерді орындау үшін `starship.toml` файлын өңдеуден де күрделірек қадамдар қажет болуы мүмкін. Бұл бетте Starship-те қолданылатын бірнеше кеңейтілген баптау тәсілдері егжей-тегжейлі сипатталған.

> [!WARNING]
> Осы бөлімдегі конфигурациялар Starship-тің болашақ шығарылымдарында өзгеруі мүмкін.

## PowerShell-де өтпелі промптты (TransientPrompt) баптау

Алдыңғы енгізілген пәрмендердің промптын арнайы қысқа жолмен ауыстыруға болады. Бұл терминалдағы барлық ақпарат үнемі қажет болмайтын жағдайларда экранды таза ұстау үшін өте пайдалы. Мұны қосу үшін қабықша сессиясында `Enable-TransientPrompt` пәрменін орындаңыз. Оны тұрақты ету үшін бұл жолды `$PROFILE` файлыңызға қосыңыз. Өтпелі режимді кез келген уақытта `Disable-TransientPrompt` пәрменімен өшіруге болады.

Әдепкі бойынша енгізу жолының сол жағы `>` таңбасымен ауыстырылады. Мұны баптау үшін `Invoke-Starship-TransientFunction` атты жаңа функция анықтаңыз. Мысалы, мұнда Starship-тің `character` модулін көрсету үшін:

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## Cmd-де өтпелі промпт пен оң жақ өтпелі промпт (TransientPrompt және TransientRightPrompt)

Clink құралы алдыңғы шығарылған промптты өзіңіз көрсеткен жолдармен ауыстыруға мүмкіндік береді. Мұны қосу үшін `clink set prompt.transient <value>` пәрменін орындаңыз, мұндағы \<value\> келесілердің бірі болуы мүмкін:

- `always`: алдыңғы промптты әрқашан ауыстыру
- `same_dir`: тек жұмыс каталогы өзгермеген жағдайда ғана алдыңғы промптты ауыстыру
- `off`: промптты ауыстырмау (яғни өтпелі режимді өшіру)

Бұл баптауды тек бір рет орындау жеткілікті. Сол және оң жақта не көрсетілетінін баптау үшін `starship.lua` файлыңызға келесі өзгерістерді енгізіңіз:

- Әдепкі бойынша енгізу жолының сол жағы `>` таңбасымен ауыстырылады. Мұны өзгерту үшін `starship_transient_prompt_func` атты жаңа функция анықтаңыз. Бұл функция ағымдағы промптты жол (string) ретінде қабылдайды. Мысалы, мұнда Starship-тің `character` модулін шығару үшін:

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- Әдепкі бойынша енгізу жолының оң жағы бос болады. Мұны баптау үшін `starship_transient_rprompt_func` функциясын анықтаңыз. Мысалы, соңғы пәрменнің қашан басталған уақытын шығару үшін:

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## Fish-те өтпелі промпт және оң жақ өтпелі промпт

Алдыңғы промптты арнайы қысқа жолмен ауыстыруға болады. Мұны ағымдағы сессияда қосу үшін `enable_transience` пәрменін орындаңыз. Оны тұрақты ету үшін бұл жолды `~/.config/fish/config.fish` файлына қосыңыз. Өтпелі режимді `disable_transience` пәрменімен қайта өшіруге болады.

Fish қабықшасында өтпелі промпт тек командалық жол бос болмаған және синтаксистік тұрғыдан дұрыс болған жағдайда ғана шығарылатынын ескеріңіз.

- Әдепкі бойынша енгізу жолының сол жағы қалың жасыл `❯` таңбасымен ауыстырылады. Мұны өзгерту үшін `starship_transient_prompt_func` функциясын анықтаңыз:

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- Әдепкі бойынша оң жағы бос болады. Мұны баптау үшін `starship_transient_rprompt_func` функциясын анықтаңыз:

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## Bash-те өтпелі промпт және оң жақ өтпелі промпт

v0.4 немесе одан жоғары нұсқадағы [Ble.sh](https://github.com/akinomyoga/ble.sh) фреймворкі алдыңғы басып шығарылған промптты өзгертуге мүмкіндік береді. Мұны қосу үшін `~/.bashrc` файлына `bleopt prompt_ps1_transient=<value>` жолын жазыңыз:

Мұндағы \<value\> — қос нүктемен бөлінген `always`, `same-dir` және `trim` тізімі.
`prompt_ps1_final` бос болған кезде және `prompt_ps1_transient` бос емес мәнге ие болса, `PS1` арқылы көрсетілген промпт командалық жолдан шыққан кезде өшіріледі. Егер мәнде `trim` өрісі болса, көп жолды `PS1` промптының тек соңғы жолы ғана сақталады, ал басқа жолдар жойылады.

Сол және оң жақта не көрсетілетінін баптау үшін `~/.blerc` (немесе `~/.config/blesh/init.sh`) файлына өзгерістер енгізіңіз:

- Сол жақты өзгерту үшін `prompt_ps1_final` Ble.sh опциясын баптаңыз. Мысалы, Starship-тің `character` модулін көрсету үшін:

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- Оң жақты өзгерту үшін `prompt_rps1_final` Ble.sh опциясын баптаңыз. Мысалы, уақытты көрсету үшін:

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Cmd-де промптқа дейінгі және орындалуға дейінгі арнайы командалар

Clink Cmd қабықшасында промптқа дейінгі (pre-prompt) және пәрменді орындау алдындағы (pre-exec) командаларды орындау үшін өте икемді API ұсынады. Қалауыңызға қарай `starship.lua` файлына келесі өзгерістерді енгізіңіз:

- Промпт шығарылмас бұрын функцияны іске қосу үшін `starship_preprompt_user_func` функциясын анықтаңыз. Мысалы, промпт алдында зымыран белгісін шығару үшін:

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- Пәрмен орындалмас бұрын функцияны іске қосу үшін `starship_precmd_user_func` функциясын анықтаңыз:

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Bash-те промптқа дейінгі және орындалу алдындағы арнайы командалар

Bash-та басқа қабықшалар сияқты ресми preexec/precmd фреймворкі жоқ. Соған қарамастан, Starship сізге промптты құру процесіне өз функцияларыңызды қосуға шектеулі мүмкіндік береді:

- Промпт салынбас бұрын функцияны іске қосу үшін жаңа функция анықтап, оның атауын `starship_precmd_user_func` айнымалысына тағайындаңыз:

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- Пәрмен орындалмас бұрын функцияны шақыру үшін [`DEBUG` тұзақ (trap) механизмін](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/) пайдалана аласыз. Алайда DEBUG сигналын міндетті түрде Starship-ті инициализациялаудан **бұрын** ұстау қажет!

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # Starship-ті іске қоспас бұрын DEBUG тұзағын орнату
set -o functrace
eval $(starship init bash)
set +o functrace
```

## PowerShell-де арнайы командаларды баптау

PowerShell-де өзіңіздің арнайы функцияңызды іске қосу үшін `Invoke-Starship-PreCommand` функциясын жасаңыз:

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## Терезе тақырыбын өзгерту (Change Window Title)

Кейбір қабықшалар терезе тақырыбын автоматты түрде өзгертеді (мысалы, ағымдағы жұмыс каталогын көрсету үшін). Fish тіпті мұны әдепкіде жасайды. Starship мұны автоматты түрде жасамайды, бірақ бұл функционалды `bash`, `zsh`, `cmd` немесе `powershell` қабықшаларына оңай қосуға болады.

Алдымен терезе тақырыбын өзгерту функциясын анықтаңыз (bash және zsh-та бірдей):

```bash
function set_win_title(){
    echo -ne "\033]0; ОРНАТАТЫН_ТЕРЕЗЕ_ТАҚЫРЫБЫ \007"
}
```

Тақырыпты баптау үшін айнымалыларды қолдануға болады (`$USER`, `$HOSTNAME` және `$PWD` танымал таңдаулар болып табылады).

`bash` қабықшасында бұл функцияны starship-тің precmd функциясы ретінде орнатыңыз:

```bash
starship_precmd_user_func="set_win_title"
```

`zsh` ішінде оны `precmd_functions` массивіне қосыңыз:

```bash
precmd_functions+=(set_win_title)
```

Мысалы, терминал қойындысында ағымдағы каталогтың атауын көрсету үшін `~/.bashrc` немесе `~/.zshrc` файлына келесіні қосыңыз:

```bash
function set_win_title(){
    echo -ne "\033]0; $(basename "$PWD") \007"
}
starship_precmd_user_func="set_win_title"
```

Cmd үшін терезе тақырыбын `starship_preprompt_user_func` функциясы арқылы өзгертуге болады:

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

PowerShell үшін `$PROFILE` ішінде `Invoke-Starship-PreCommand` функциясын қолдануға болады:

```powershell
# $PROFILE өңдеу
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## Оң жақ промптты қосу (Enable Right Prompt)

Кейбір қабықшалар енгізу жолымен бір сызықта орналасатын оң жақ промптты қолдайды. Starship оң жақ промпт мазмұнын `right_format` параметрі арқылы орната алады. `format` ішінде қолданылатын кез келген модуль `right_format` ішінде де қолдау табады. Ал `$all` айнымалысы тек `format` немесе `right_format` ішінде ашық түрде көрсетілмеген модульдерді ғана қамтиды.

Ескертпе: Оң жақ промпт — енгізу орнынан кейінгі бір жолдық элемент. Көп жолдық промптта енгізу жолының үстіндегі модульдерді оңға туралау үшін [`fill` модулін](../config/#fill) қараңыз.

`right_format` қазіргі уақытта келесі қабықшаларда қолдау тапқан: elvish, fish, zsh, xonsh, cmd, nushell, bash.

Ескертпе: Bash ішінде оң жақ промптты қолдану үшін [Ble.sh](https://github.com/akinomyoga/ble.sh) фреймворкінің v0.4 немесе одан жоғары нұсқасы орнатылуы тиіс.

### Мысал

```toml
# ~/.config/starship.toml

# Ықшам сол жақ промпт
format = """$character"""

# Қалған барлық модульдерді оң жаққа жылжыту
right_format = """$all"""
```

Нәтижесі келесідей көрінеді:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

`zsh` (v5.0.5+) қолданған кезде, қабықша оң жақ промптқа әдепкі бос орын қосады. Бұл Starship-тің `$fill` модулін қолданған кезде туралау ақауларын тудыруы мүмкін. Бұл бос орынды алып тастау үшін `.zshrc` файлыңызға мынаны қосыңыз:

```zsh
ZLE_RPROMPT_INDENT=0
```

## Жалғастыру промпты (Continuation Prompt)

Кейбір қабықшалар қалыпты промптпен қатар жалғастыру промптын қолдайды. Бұл промпт пайдаланушы толық емес пәрменді (мысалы, жабылмаған жақша немесе тырнақша) енгізген кезде қалыпты промпттың орнына шығарылады.

Starship жалғастыру промптын `continuation_prompt` опциясы арқылы орната алады. Әдепкі промпт: `'[∙](bright-black) '`.

Ескертпе: `continuation_prompt` ешқандай айнымалыларсыз тікелей мәтіндік жол ретінде орнатылуы керек.

Ескертпе: Жалғастыру промпты тек мына қабықшаларда қолжетімді:

- `bash`
- `zsh`
- `PowerShell`

### Мысал

```toml
# ~/.config/starship.toml

# Екі боялған көрсеткіні көрсететін жалғастыру промпты
continuation_prompt = '▶▶ '
```

## Claude Code үшін күй жолы (Statusline for Claude Code)

Starship Anthropic-тің интерактивті бағдарламалауға арналған CLI құралы — Claude Code ішінде орындалғанда арнайы күй жолын (statusline) көрсетуді қолдайды. Бұл күй жолы нақты уақыт режимінде Claude сессиясы туралы ақпаратты (қолданылатын үлгі, мәнмәтін терезесінің қолданылуы және сессия құны) көрсетеді.

Толық ақпарат алу үшін [Claude Code statusline құжаттамасын](https://code.claude.com/docs/en/statusline) қараңыз.

### Баптау

Starship-ті Claude Code күй жолы ретінде пайдалану үшін:

1. Claude Code ішінде `/statusline` командасын орындап, оған Starship-ті баптауды сұраңыз немесе `.claude/settings.json` файлына келесіні қолмен қосыңыз:

```json
{
  "statusLine": {
    "type": "command",
    "command": "starship statusline claude-code"
  }
}
```

2. `~/.config/starship.toml` файлыңызда күй жолының сыртқы көрінісін баптаңыз.

### Жалпы шолу

`starship statusline claude-code` арқылы шақырылған кезде, Starship stdin арқылы Claude Code сессиясының деректерін қабылдайды және `claude-code` атты арнайы профильді пайдаланып күй жолын шығарады.

Бұл профиль үш арнайы модульді қамтиды:

- `claude_model`: Ағымдағы пайдаланылып жатқан Claude үлгісін көрсетеді
- `claude_context`: Мәнмәтін терезесінің (context window) қолданылуын визуалды шкаламен көрсетеді
- `claude_cost`: Сессияның құны мен статистикасын көрсетеді

Әдепкі профиль пішімі:

```toml
[profiles]
claude-code = "$claude_model$git_branch$claude_context$claude_cost"
```

### Конфигурация мысалы

```toml
# ~/.config/starship.toml

# claude-code профилін баптау
[profiles]
claude-code = "$claude_model$claude_context$claude_cost"

# Жеке модульдерді конфигурациялау
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

## Стиль жолдары (Style Strings)

Стиль жолдары бос орындармен бөлінген сөздер тізімінен тұрады. Сөздер регистрге сезімтал емес (яғни `bold` және `BoLd` бірдей деп есептеледі). Әр сөз келесілердің бірі болуы мүмкін:

- `bold` (қалың)
- `italic` (курсив)
- `underline` (асты сызылған)
- `dimmed` (күңгірт)
- `inverted` (түстері ауыстырылған)
- `blink` (жыпылықтайтын)
- `hidden` (жасырын)
- `strikethrough` (сызылып тасталған)
- `bg:<color>` (фон түсі)
- `fg:<color>` (мәтін түсі)
- `<color>` (түс)
- `none` (стильсіз)

мұндағы `<color>` — түс идентификаторы. Қазіргі уақытта `fg:<color>` және `<color>` бірдей әрекет етеді.
`<color>` мәнін `prev_fg` немесе `prev_bg` ретінде орнатуға болады, бұл қолжетімді болса алдыңғы элементтің мәтін немесе фон түсіне, әйтпесе `none` мәніне теңеседі.
`inverted` фон мен мәтін түстерінің орындарын ауыстырады. Жолдағы сөздердің реті маңызды емес.

`none` токені `bg:` құрамында болмаса, жолдағы барлық басқа токендерді жоққа шығарады (мысалы, `fg:red none fg:blue` стильсіз жол жасайды). `bg:none` фонды әдепкі түске орнатады, сондықтан `fg:red bg:none` тіркесі `red` немесе `fg:red` мәніне баламалы.

Түс анықтауышы келесілердің бірі болуы мүмкін:

- Терминалдың стандартты түстерінің бірі: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. Ашық нұсқасын алу үшін оларға `bright-` префиксін қосуға болады (мысалы, `bright-white`).
- `#` таңбасынан кейінгі алты таңбалы он алтылық сан — [RGB түс коды](https://www.w3schools.com/colors/colors_hexadecimal.asp).
- 0 мен 255 аралығындағы сан — [8-биттік ANSI түс коды](https://i.stack.imgur.com/KTSQa.png).

Егер мәтін немесе фон үшін бірнеше түс көрсетілсе, жолдағы ең соңғысы басымдыққа ие болады.

Кейбір терминалдардың шектеулері:
- Көптеген терминалдарда `blink` әдепкіде өшірулі болады.
- `hidden` опциясына [iTerm қолдау көрсетпейді](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough` macOS стандартты Terminal.app қолданбасында қолдау таппаған.
