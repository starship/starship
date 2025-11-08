# 자주 묻는 질문

## 데모 GIF에는 어떤 구성을 사용했나요?

- **터미널 에뮬레이터**: [iTerm2](https://iterm2.com/)
  - **테마**: Minimal
  - **배색**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **폰트**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **셸**: [Fish Shell](https://fishshell.com/)
  - **구성**: [matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **프롬프트**: [Starship](https://starship.rs/)

## 데모 GIF에 표시된 명령 완성 기능을 어떻게 얻을 수 있나요?

완성 지원 또는 자동 완성은 선택한 셸에서 제공합니다. 데모의 경우 [Fish Shell](https://fishshell.com/)로 데모를 진행했으며, Fish Shell은 기본적으로 완성을 제공합니다. Z Shell(zsh)을 사용하는 경우 [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions)를 살펴보는 것이 좋습니다.

## 최상위 `format`과 `<module>.disabled`는 동일한 기능을 하나요?

예, 둘 다 프롬프트에서 모듈을 비활성화하는 데 사용할 수 있습니다. 모듈을 비활성화하는 것이 유일한 계획이라면 다음과 같은 이유로 `<module>.disabled`가 선호되는 방법입니다.

- 모듈 비활성화는 최상위 `format`에서 생략하는 것보다 더 명시적입니다.
- 새로 생성된 모듈은 Starship이 업데이트됨에 따라 프롬프트에 추가됩니다.

## The docs say Starship is cross-shell. Why isn't my preferred shell supported?

Starship이 구축된 방식으로는 사실상 모든 셸에 대한 지원을 추가할 수 있어야 합니다. starship 바이너리는 상태 비저장이며 셸에 구애받지 않으므로, 셸이 프롬프트 사용자 지정을 지원하고 셸 확장을 지원하는 한 Starship을 사용할 수 있습니다.

다음은 Starship이 bash에서 작동하는 작은 예시입니다.

```sh
# Get the status code from the last command executed
STATUS=$?

# Get the number of jobs running.
NUM_JOBS=$(jobs -p | wc -l)

# Set the prompt to the output of `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

Starship에 내장된 [Bash 구현](https://github.com/starship/starship/blob/master/src/init/starship.bash)은 [명령 지속 시간 모듈](https://starship.rs/config/#command-duration)과 같은 고급 기능을 허용하고 Starship이 사전 설치된 Bash 구성과 호환되도록 하기 위해 약간 더 복잡합니다.

`starship 프롬프트`에서 지원하는 모든 플래그 값을 보려면 아래 명령어를 사용하세요:

```sh
starship prompt --help
```

프롬프트는 제공된 만큼의 컨텍스트를 사용하지만, "필수" 플래그는 없습니다.

## 오래된 버전의 glibc가 있는 Linux 배포판에서 Starship을 어떻게 실행하나요?

미리 빌드된 바이너리를 실행할 때 (예를 들어 CentOS 6 혹은 7에서) "_version 'GLIBC_2.18' not found (required by starship)_" 같은 오류가 보인다면, `glibc` 대신 `musl`로 컴파일된 바이너리 파일을 사용하세요.

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## 왜 `Executing command "..." timed out.` 경고가 뜨나요?

Starship은 프롬프트에 표시할 정보를 얻기 위해 다양한 명령을 실행합니다. 예를 들어 프로그램 버전이나 현재 git 상태를 가져옵니다. starship이 이러한 명령을 실행하는 동안 멈추지 않도록 시간 제한을 설정합니다. 명령이 이 제한보다 오래 걸리면 starship은 명령 실행을 중지하고 위 경고를 출력합니다. 이는 예상된 동작입니다. 이 시간 제한은 [`command_timeout` 키](../config/#prompt)를 사용하여 구성할 수 있으므로 원하는 경우 시간 제한을 늘릴 수 있습니다. 또한 아래 디버깅 단계를 따라 어떤 명령이 느린지 확인하고 최적화할 수 있는지 확인할 수 있습니다. 마지막으로 이러한 경고를 숨기려면 `STARSHIP_LOG` 환경 변수를 `error`로 설정할 수 있습니다.

## 이해할 수 없거나 예상치 못한 기호가 보이는데 무슨 뜻인가요?

인식할 수 없는 기호가 보이면 `starship explain`을 사용하여 현재 표시되는 모듈을 설명할 수 있습니다.

## Starship이 예상치 못한 동작을 하는데, 어떻게 디버깅할 수 있나요?

`STARSHIP_LOG` 환경 변수를 사용하여 디버그 로그를 활성화할 수 있습니다. 이 로그는 매우 자세할 수 있으므로 특정 모듈을 디버깅하려는 경우 `module` 명령을 사용하는 것이 유용합니다. 예를 들어 `rust` 모듈을 디버깅하려는 경우 다음 명령을 실행하여 모듈의 추적 로그 및 출력을 얻을 수 있습니다.

```sh
env STARSHIP_LOG=trace starship module rust
```

starship이 느리다면 `timings` 명령을 사용하여 특정 모듈이나 명령이 원인인지 확인할 수 있습니다.

```sh
env STARSHIP_LOG=trace starship timings
```

이것은 추적 로그와 실행하는 데 1ms 이상 걸렸거나 일부 출력을 생성한 모든 모듈의 분석을 출력합니다.

마지막으로 버그를 발견하면 `bug-report` 명령을 사용하여 GitHub 이슈를 생성할 수 있습니다.

```sh
starship bug-report
```

## 프롬프트에 글리프 기호가 보이지 않는 이유는 무엇인가요?

가장 흔한 원인은 시스템 설정 오류입니다. 특히 일부 Linux 배포판은 기본적으로 글꼴 지원이 제공되지 않습니다. 다음 사항을 확인해야 합니다.

- 로케일이 `de_DE.UTF-8` 또는 `ja_JP.UTF-8`과 같은 UTF-8 값으로 설정되어 있는지 확인하세요. `LC_ALL`이 UTF-8 값이 아닌 경우 [변경해야 합니다](https://www.tecmint.com/set-system-locales-in-linux/).
- You have an emoji font installed. 대부분의 시스템에는 기본적으로 이모지 글꼴이 제공되지만, 일부(특히 Arch Linux)는 그렇지 않습니다. 일반적으로 시스템의 패키지 관리자를 통해 설치할 수 있습니다. [noto emoji](https://www.google.com/get/noto/help/emoji/)는 인기 있는 선택입니다.
- [Nerd Font](https://www.nerdfonts.com/)를 사용하고 있는지 확인하세요.

시스템을 테스트하려면 터미널에서 다음 명령을 실행하세요.

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

첫 번째 줄은 [뱀 이모지](https://emojipedia.org/snake/)를 생성해야 하고, 두 번째 줄은 [파워라인 브랜치 기호(e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs)를 생성해야 합니다.

두 기호 중 하나라도 올바르게 표시되지 않으면 시스템이 여전히 잘못 구성된 것입니다. 안타깝게도 글꼴 구성을 올바르게 설정하는 것은 때때로 어렵습니다. Discord의 사용자들이 도움을 줄 수 있습니다. 두 기호 모두 올바르게 표시되지만 starship에서 여전히 보이지 않는다면 [버그 보고서를 제출하세요!](https://github.com/starship/starship/issues/new/choose)

## Starship을 어떻게 삭제하나요?

Starship은 처음 설치하는 것만큼이나 쉽게 제거할 수 있습니다.

1. 셸 설정 파일 (예시: `~/.bashrc`) 에서 Starship 초기화에 사용되는 모든 줄을 제거하세요.
1. Starship 바이너리 파일을 제거하세요.

Starship을 패키지 매니저로 설치하였다면 해당 패키지 매니저의 제거 지침 문서를 참조해 주세요.

Starship을 설치 스크립트로 설치하였다면 바이너리 파일 제거를 위해 아래 명령어를 실행하세요:

```sh
# starship 바이너리 파일을 찾고 제거합니다. sh -c 'rm "$(command -v 'starship')"'
```

## `sudo` 없이 Starship을 어떻게 설치하나요?

셸 설치 스크립트(`https://starship.rs/install.sh`)는 대상 설치 디렉토리가 현재 사용자가 쓸 수 없는 경우에만 `sudo`를 사용하려고 시도합니다. 예를 들어, `curl -sS https://starship.rs/install.sh | sh -s -- -b ~/.local/bin`은 설치 스크립트의 `-b` 명령줄 옵션을 사용하여 설치 디렉토리를 `~/.local/bin`으로 설정합니다. 기본 설치 디렉토리는 `$BIN_DIR` 환경 변수의 값 또는 `$BIN_DIR`이 설정되지 않은 경우 `/usr/local/bin`입니다. 대신 설치 디렉토리를 사용자가 쓸 수 있는 디렉토리로 설정하면 `sudo` 없이 starship을 설치할 수 있습니다.

Starship의 비대화형 설치의 경우 확인을 건너뛰려면 `-y` 옵션을 추가하는 것을 잊지 마세요. 지원되는 모든 설치 옵션 목록은 설치 스크립트의 소스를 확인하세요.

패키지 관리자를 사용하는 경우 `sudo`를 사용하거나 사용하지 않고 설치하는 방법에 대한 패키지 관리자 문서를 참조하세요.
