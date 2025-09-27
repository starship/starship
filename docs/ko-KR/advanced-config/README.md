# 고급 설정

Starship은 다재다능한 셸이지만, 때로는 `starship.toml`을 수정하는 것 이상을 해야 특정 작업을 수행할 수 있습니다. 이 페이지에서는 starship에서 사용되는 몇 가지 고급 설정 기술에 대해 자세히 설명합니다.

::: warning

이 섹션의 설정은 향후 Starship 릴리스에서 변경될 수 있습니다.

:::

## PowerShell의 TransientPrompt

이전에 출력된 프롬프트를 사용자 지정 문자열로 바꿀 수 있습니다. 이는 모든 프롬프트 정보가 항상 필요하지 않은 경우에 유용합니다. 이를 활성화하려면 셸 세션에서 `Enable-TransientPrompt`를 실행하세요. 영구적으로 적용하려면 이 구문을 `$PROFILE`에 넣으세요. Transience는 `Disable-TransientPrompt`로 즉시 비활성화할 수 있습니다.

기본적으로 입력의 왼쪽은 `>`로 바뀝니다. 이를 사용자 지정하려면 `Invoke-Starship-TransientFunction`이라는 새 함수를 정의하세요. 예를 들어, 여기에 Starship의 `character` 모듈을 표시하려면 다음과 같이 하세요.

```powershell
function Invoke-Starship-TransientFunction {
  &starship module character
}

Invoke-Expression (&starship init powershell)

Enable-TransientPrompt
```

## Cmd의 TransientPrompt 및 TransientRightPrompt

Clink를 사용하면 이전에 출력된 프롬프트를 사용자 지정 문자열로 바꿀 수 있습니다. 이는 모든 프롬프트 정보가 항상 필요하지 않은 경우에 유용합니다. 이를 활성화하려면 `clink set prompt.transient <value>`를 실행하세요. 여기서 `<value>`는 다음 중 하나일 수 있습니다.

- `always`: 항상 이전 프롬프트를 바꿉니다.
- `same_dir`: 작업 디렉토리가 동일한 경우에만 이전 프롬프트를 바꿉니다.
- `off`: 프롬프트를 바꾸지 않습니다 (즉, transience를 끕니다).

이 작업은 한 번만 수행하면 됩니다. 왼쪽과 오른쪽에 표시되는 내용을 사용자 지정하려면 `starship.lua`를 다음과 같이 변경하세요.

- 기본적으로 입력의 왼쪽은 `>`로 바뀝니다. 이를 사용자 지정하려면 `starship_transient_prompt_func`라는 새 함수를 정의하세요. 이 함수는 현재 프롬프트를 문자열로 받아 활용할 수 있습니다. 예를 들어, 여기에 Starship의 `character` 모듈을 표시하려면 다음과 같이 하세요.

```lua
function starship_transient_prompt_func(prompt)
  return io.popen("starship module character"
    .." --keymap="..rl.getvariable('keymap')
  ):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

- 기본적으로 입력의 오른쪽은 비어 있습니다. 이를 사용자 지정하려면 `starship_transient_rprompt_func`라는 새 함수를 정의하세요. 이 함수는 현재 프롬프트를 문자열로 받아 활용할 수 있습니다. 예를 들어, 마지막 명령이 시작된 시간을 여기에 표시하려면 다음과 같이 하세요.

```lua
function starship_transient_rprompt_func(prompt)
  return io.popen("starship module time"):read("*a")
end
load(io.popen('starship init cmd'):read("*a"))()
```

## Fish의 TransientPrompt 및 TransientRightPrompt

이전에 출력된 프롬프트를 사용자 지정 문자열로 바꿀 수 있습니다. 이는 모든 프롬프트 정보가 항상 필요하지 않은 경우에 유용합니다. 이를 활성화하려면 셸 세션에서 `enable_transience`를 실행하세요. 영구적으로 적용하려면 이 구문을 `~/.config/fish/config.fish`에 넣으세요. Transience는 `disable_transience`로 즉시 비활성화할 수 있습니다.

Fish의 경우, transient 프롬프트는 명령줄이 비어 있지 않고 구문적으로 올바른 경우에만 출력됩니다.

- 기본적으로 입력의 왼쪽은 굵은 녹색 `❯`로 바뀝니다. 이를 사용자 지정하려면 `starship_transient_prompt_func`라는 새 함수를 정의하세요. 예를 들어, 여기에 Starship의 `character` 모듈을 표시하려면 다음과 같이 하세요.

```fish
function starship_transient_prompt_func
  starship module character
end
starship init fish | source
enable_transience
```

- 기본적으로 입력의 오른쪽은 비어 있습니다. 이를 사용자 지정하려면 `starship_transient_rprompt_func`라는 새 함수를 정의하세요. 예를 들어, 마지막 명령이 시작된 시간을 여기에 표시하려면 다음과 같이 하세요.

```fish
function starship_transient_rprompt_func
  starship module time
end
starship init fish | source
enable_transience
```

## Bash의 TransientPrompt 및 TransientRightPrompt

v0.4 이상의 [Ble.sh](https://github.com/akinomyoga/ble.sh) 프레임워크를 사용하면 이전에 출력된 프롬프트를 사용자 지정 문자열로 바꿀 수 있습니다. 이는 모든 프롬프트 정보가 항상 필요하지 않은 경우에 유용합니다. 이를 활성화하려면 `~/.bashrc`에 `bleopt prompt_ps1_transient=<value>`를 넣으세요.

여기서 `<value>`는 `always`, `same-dir`, `trim`의 콜론으로 구분된 목록입니다. `prompt_ps1_final`이 비어 있고 `prompt_ps1_transient` 옵션에 비어 있지 않은 `<value>`가 있으면, 현재 명령줄을 떠날 때 `PS1`으로 지정된 프롬프트가 지워집니다. `<value>`에 `trim` 필드가 포함된 경우, 여러 줄 `PS1`의 마지막 줄만 보존되고 다른 줄은 지워집니다. 그렇지 않으면, `PS1=`이 지정된 것처럼 명령줄이 다시 그려집니다. `<value>`에 `same-dir` 필드가 포함되어 있고 현재 작업 디렉토리가 이전 명령줄의 최종 디렉토리와 다른 경우, 이 `prompt_ps1_transient` 옵션은 무시됩니다.

왼쪽과 오른쪽에 표시되는 내용을 사용자 지정하려면 `~/.blerc`(또는 `~/.config/blesh/init.sh`)를 다음과 같이 변경하세요.

- 입력의 왼쪽이 무엇으로 바뀔지 사용자 지정하려면 `prompt_ps1_final` Ble.sh 옵션을 설정하세요. 예를 들어, 여기에 Starship의 `character` 모듈을 표시하려면 다음과 같이 하세요.

```bash
bleopt prompt_ps1_final='$(starship module character)'
```

- 입력의 오른쪽이 무엇으로 바뀔지 사용자 지정하려면 `prompt_rps1_final` Ble.sh 옵션을 설정하세요. 예를 들어, 마지막 명령이 시작된 시간을 여기에 표시하려면 다음과 같이 하세요.

```bash
bleopt prompt_rps1_final='$(starship module time)'
```

## Cmd의 사용자 지정 pre-prompt 및 pre-execution 명령어

Clink는 Cmd 셸에서 pre-prompt 및 pre-exec 명령을 실행하기 위한 매우 유연한 API를 제공합니다. Starship과 함께 사용하기는 매우 간단합니다. 필요에 따라 `starship.lua` 파일을 다음과 같이 변경하세요.

- 프롬프트가 그려지기 직전에 사용자 지정 함수를 실행하려면 `starship_preprompt_user_func`라는 새 함수를 정의하세요. 이 함수는 현재 프롬프트를 문자열로 받아 활용할 수 있습니다. 예를 들어, 프롬프트 앞에 로켓을 그리려면 다음과 같이 하세요.

```lua
function starship_preprompt_user_func(prompt)
  print("🚀")
end

load(io.popen('starship init cmd'):read("*a"))()
```

- 명령이 실행되기 직전에 사용자 지정 함수를 실행하려면 `starship_precmd_user_func`라는 새 함수를 정의하세요. 이 함수는 현재 명령줄을 문자열로 받아 활용할 수 있습니다. 예를 들어, 실행될 명령을 출력하려면 다음과 같이 하세요.

```lua
function starship_precmd_user_func(line)
  print("Executing: "..line)
end

load(io.popen('starship init cmd'):read("*a"))()
```

## Bash의 사용자 지정 pre-prompt 및 pre-execution 명령어

Bash는 대부분의 다른 셸과 같은 공식적인 preexec/precmd 프레임워크가 없습니다. 이 때문에 `bash`에서 완전히 사용자 지정 가능한 훅을 제공하기 어렵습니다. 그러나 Starship은 프롬프트 렌더링 절차에 자신의 함수를 삽입할 수 있는 제한된 기능을 제공합니다.

- 프롬프트가 그려지기 직전에 사용자 지정 함수를 실행하려면 새 함수를 정의한 다음 그 이름을 `starship_precmd_user_func`에 할당하세요. 예를 들어, 프롬프트 앞에 로켓을 그리려면 다음과 같이 하세요.

```bash
function blastoff(){
    echo "🚀"
}
starship_precmd_user_func="blastoff"
```

- 명령이 실행되기 직전에 사용자 지정 함수를 실행하려면 [`DEBUG` 트랩 메커니즘](https://jichu4n.com/posts/debug-trap-and-prompt_command-in-bash/)을 사용할 수 있습니다. 그러나 Starship을 초기화하기 _전에_ DEBUG 신호를 트랩해야 합니다! Starship은 DEBUG 트랩의 값을 보존할 수 있지만, starship이 시작된 후 트랩이 덮어쓰여지면 일부 기능이 깨집니다.

```bash
function blastoff(){
    echo "🚀"
}
trap blastoff DEBUG     # starship을 실행하기 *전에* DEBUG를 트랩합니다
set -o functrace
eval $(starship init bash)
set +o functrace
```

## PowerShell의 사용자 지정 pre-prompt 및 pre-execution 명령어

PowerShell은 대부분의 다른 셸과 같은 공식적인 preexec/precmd 프레임워크가 없습니다. 이 때문에 `powershell`에서 완전히 사용자 지정 가능한 훅을 제공하기 어렵습니다. 그러나 Starship은 프롬프트 렌더링 절차에 자신의 함수를 삽입할 수 있는 제한된 기능을 제공합니다.

`Invoke-Starship-PreCommand`라는 이름의 함수를 만드세요.

```powershell
function Invoke-Starship-PreCommand {
    $host.ui.Write("🚀")
}
```

## 창 제목 변경

일부 셸 프롬프트는 자동으로 창 제목을 변경합니다 (예: 작업 디렉토리를 반영하도록). Fish는 기본적으로 그렇게 합니다. Starship은 이 기능을 수행하지 않지만, `bash`, `zsh`, `cmd` 또는 `powershell`에 이 기능을 추가하는 것은 매우 간단합니다.

먼저, 창 제목 변경 함수를 정의합니다 (bash와 zsh에서 동일).

```bash
function set_win_title(){
    echo -ne "]0; 여기에_창_제목_입력 "
}
```

변수를 사용하여 이 제목을 사용자 지정할 수 있습니다 (`$USER`, `$HOSTNAME`, `$PWD`가 많이 사용됩니다).

`bash`에서는 이 함수를 precmd starship 함수로 설정합니다.

```bash
starship_precmd_user_func="set_win_title"
```

`zsh`에서는 `precmd_functions` 배열에 이것을 추가합니다.

```bash
precmd_functions+=(set_win_title)
```

결과가 마음에 들면, 이 줄들을 셸 설정 파일 (`~/.bashrc` 또는 `~/.zshrc`)에 추가하여 영구적으로 만드세요.

예를 들어, 터미널 탭 제목에 현재 디렉토리를 표시하려면 다음 스니펫을 `~/.bashrc` 또는 `~/.zshrc`에 추가하세요.

```bash
function set_win_title(){
    echo -ne "]0; $(basename "$PWD") "
}
starship_precmd_user_func="set_win_title"
```

Cmd의 경우, `starship_preprompt_user_func` 함수를 사용하여 창 제목을 변경할 수 있습니다.

```lua
function starship_preprompt_user_func(prompt)
  console.settitle(os.getenv('USERNAME').."@"..os.getenv('COMPUTERNAME')..": "..os.getcwd())
end

load(io.popen('starship init cmd'):read("*a"))()
```

`Invoke-Starship-PreCommand`라는 이름의 함수를 만들어 PowerShell에서도 비슷한 출력을 설정할 수 있습니다.

```powershell
# $PROFILE 편집
function Invoke-Starship-PreCommand {
  $host.ui.RawUI.WindowTitle = "$env:USERNAME@$env:COMPUTERNAME`: $pwd `a"
}

Invoke-Expression (&starship init powershell)
```

## 오른쪽 프롬프트 활성화

일부 셸은 입력과 같은 줄에 렌더링되는 오른쪽 프롬프트를 지원합니다. Starship에서는 `right_format` 옵션을 사용하여 오른쪽 프롬프트의 내용을 설정할 수 있습니다. `format`에서 사용할 수 있는 모든 모듈은 `right_format`에서도 지원됩니다. `$all` 변수는 `format` 또는 `right_format`에서 명시적으로 사용하지 않는 모듈만 포함합니다.

알림: 오른쪽 프롬프트는 입력 위치에 따라 한 줄로 표시됩니다. 여러 줄 프롬프트에서 입력 줄 위에 모듈을 오른쪽 정렬하려면 [`fill` 모듈](../config/#fill)을 참조하세요.

`right_format`은 현재 다음 셸에서 지원됩니다: elvish, fish, zsh, xonsh, cmd, nushell, bash.

참고: bash에서 오른쪽 프롬프트를 사용하려면 v0.4 이상의 [Ble.sh](https://github.com/akinomyoga/ble.sh) 프레임워크를 설치해야 합니다.

### 예시

```toml
# ~/.config/starship.toml

# 간결한 왼쪽 프롬프트
format = """$character"""

# 프롬프트의 나머지를 오른쪽으로 옮기기
right_format = """$all"""
```

위 설정은 아래와 같은 프롬프트를 출력합니다:

```
▶                                   starship on  rprompt [!] is 📦 v0.57.0 via 🦀 v1.54.0 took 17s
```

## 연속 프롬프트

일부 셸은 일반 프롬프트와 함께 연속 프롬프트를 지원합니다. 이 프롬프트는 사용자가 불완전한 문장(예: 여는 괄호나 따옴표 하나)을 입력했을 때 일반 프롬프트 대신 렌더링됩니다.

Starship은 `continuation_prompt` 옵션을 사용하여 연속 프롬프트를 설정할 수 있습니다. 기본 프롬프트는 `'[∙](bright-black) '`입니다.

참고: `continuation_prompt`는 변수 없이 리터럴 문자열로 설정해야 합니다.

참고: 연속 프롬프트는 다음 셸에서만 사용할 수 있습니다.

- `bash`
- `zsh`
- `Powershell`

### 예시

```toml
# ~/.config/starship.toml

# 채워진 화살표 두 개를 표시하는 연속 프롬프트
continuation_prompt = '▶▶ '
```

## 스타일 문자열

스타일 문자열은 공백으로 구분된 단어 목록입니다. 단어는 대소문자를 구분하지 않습니다 (즉, `bold`와 `BoLd`는 동일한 문자열로 간주됩니다). 각 단어는 다음 중 하나가 될 수 있습니다:

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

`<color>` 부분은 색상 지정자입니다 (아래에 후술). 현재, `fg:<color>` 와 `<color>`는 동일한 동작을 하지만 차후에 바뀔 수 있습니다. `<color>`는 또한 `prev_fg` 또는 `prev_bg`로 설정할 수 있으며, 이는 사용 가능한 경우 이전 항목의 전경 또는 배경색으로 평가되거나 그렇지 않으면 `none`으로 평가됩니다. `inverted`는 배경 색과 전경 색을 서로 바꿉니다. 문자열의 단어 순서는 중요하지 않습니다.

`none` 토큰은 `bg:` 지정자의 일부가 아닌 경우 문자열의 다른 모든 토큰을 재정의하므로, 예를 들어 `fg:red none fg:blue`는 스타일이 없는 문자열을 만듭니다. `bg:none`은 배경을 기본 색상으로 설정하므로 `fg:red bg:none`은 `red` 또는 `fg:red`와 동일하며 `bg:green fg:red bg:none`도 `fg:red` 또는 `red`와 동일합니다. 향후 다른 토큰과 함께 `none`을 사용하는 것은 오류가 발생할 수 있습니다.

색상 지정자는 다음 중 하나가 될 수 있습니다:

- 표준 터미널 색상: `black`, `red`, `green`, `blue`, `yellow`, `purple`, `cyan`, `white`. 앞에 `bright-`를 붙여 밝게 만들 수도 있습니다 (예시: `bright-white`).
- `#` 다음의 여섯 자리 16진수 숫자. 이는 [RGB 색상 16진수 코드](https://www.w3schools.com/colors/colors_hexadecimal.asp)입니다.
- 0~255 사이의 숫자. 이는 [8비트 ANSI 색상 코드](https://i.stack.imgur.com/KTSQa.png)입니다.

배경에 여러 색상이 지정된 경우, 문자열의 마지막 색상이 우선 순위를 갖습니다.

모든 스타일 문자열이 모든 터미널에서 올바르게 표시되는 것은 아닙니다. 특히 다음과 같은 알려진 문제점이 있습니다.

- 많은 터미널이 기본적으로 `blink` 지원을 비활성화합니다.
- `hidden`은 [iTerm에서 지원되지 않습니다](https://gitlab.com/gnachman/iterm2/-/issues/4564).
- `strikethrough`는 기본 macOS Terminal.app에서 지원되지 않습니다.