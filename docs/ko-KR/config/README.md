# 설정

Starship을 설정하려면, `~/.config/starship.toml` 경로에 파일을 만드세요.

```sh
mkdir -p ~/.config && touch ~/.config/starship.toml
```

Starship의 모든 설정은 이 [TOML](https://github.com/toml-lang/toml) 파일에서 할 수 있습니다.

```toml
# 설정 스키마에 따른 에디터 자동 완성 가져오기
"$schema" = 'https://starship.rs/config-schema.json'

# 셸 프롬프트 사이에 빈 줄 추가하기
add_newline = true

# 프롬프트의 '❯' 심볼을  '➜' 로 대체하기
[character] # 설정할 모듈의 이름은 'character'
success_symbol = '[➜](bold green)' # 'success_symbol' 세그먼트를 'bold green' 색상의 '➜' 로 설정

# package 모듈을 비활성화하고 프롬프트에서 완전히 숨겨버리기
[package]
disabled = true
```

### 설정 파일 경로

`STARSHIP_CONFIG` 환경 변수를 사용하여 기본 설정 파일 위치를 변경할 수 있습니다:

```sh
export STARSHIP_CONFIG=~/example/non/default/path/starship.toml
```

PowerShell (Windows)에서는 `$PROFILE`에 다음 줄을 추가하는 것과 동일합니다:

```powershell
$ENV:STARSHIP_CONFIG = "$HOME\example\non\default\path\starship.toml"
```

Cmd (Windows)에서는 `starship.lua`에 다음 줄을 추가하는 것과 동일합니다:

```lua
os.setenv('STARSHIP_CONFIG', 'C:\Users\user\example\non\default\path\starship.toml')
```

### 로그

기본적으로 starship은 `~/.cache/starship/session_${STARSHIP_SESSION_KEY}.log`라는 파일에 경고 및 오류를 기록합니다. 여기서 세션 키는 터미널 인스턴스에 해당합니다. 그러나 `STARSHIP_CACHE` 환경 변수를 사용하여 이를 변경할 수 있습니다:

```sh
export STARSHIP_CACHE=~/.starship/cache
```

PowerShell (Windows)에서는 `$PROFILE`에 다음 줄을 추가하는 것과 동일합니다:

```powershell
$ENV:STARSHIP_CACHE = "$HOME\AppData\Local\Temp"
```

Cmd (Windows)에서는 `starship.lua`에 다음 줄을 추가하는 것과 동일합니다:

```lua
os.setenv('STARSHIP_CACHE', 'C:\Users\user\AppData\Local\Temp')
```

### 용어

**모듈**: OS의 배경 정보를 기반으로 정보를 제공하는 프롬프트의 구성 요소입니다. 예를 들어, "nodejs" 모듈은 현재 디렉토리가 Node.js 프로젝트 디렉토리라면 컴퓨터에 현재 설치되어 있는 Node.js 버전을 보여줍니다.

**변수**: 모듈에서 제공하는 정보를 포함하는 더 작은 하위 구성 요소입니다. 예를 들어, "nodejs" 모듈의 "version" 변수에는 현재 Node.js 버전이 포함됩니다.

관례적으로 대부분의 모듈은 기본 터미널 색상 접두사 (예: "nodejs"의 `via`)와 빈 공간을 접미사로 가집니다.

### 문자열

TOML 문법에서는 [텍스트 값](https://toml.io/en/v1.0.0#string)을 '`, `"`, `'''`, 그리고 `"""`으로 지정합니다.

다음 Starship 구문 기호는 형식 문자열에서 특별한 용도로 사용되며 해당 문자로 표시하려면 이스케이프해야 합니다: `$ [ ] ( )`.

| 기호    | 종류                        | 비고                                                     |
| ----- | ------------------------- | ------------------------------------------------------ |
| `'`   | 리터럴 문자열                   | 적은 이스케이프                                          |
| `"`   | 문자열                    | 더 많은 이스케이프                                          |
| `'''` | 여러 줄 리터럴 문자열           | 적은 이스케이프                                          |
| `"""` | 여러 줄 문자열              | 더 많은 이스케이프, 선언의 새 줄은 무시될 수 있습니다 |

예를 들어:

```toml
# literal string
format = '☺\☻ '

# regular string
format = "☺\\☻ "

# escaping Starship symbols
format = '\\\[\\$\\\] '
```

줄 바꿈을 사용할 때 여러 줄 선언을 사용할 수 있습니다. 예를 들어, 새 줄에 `$` 기호를 인쇄하려면 `format`에 대한 다음 값이 동일합니다:

```toml
# with literal string
format = '''

\$"''

# with multiline basic string
format = """

\\\$"""

# with basic string
format = "\n\\\$"
```

여러 줄 기본 문자열에서 새 줄은 이스케이프하여 값에 존재하지 않고도 서식 지정에 사용할 수 있습니다.

```toml
format = """
line1\
line1\
line1
line2\
line2\
line2
"""
```

### 형식 문자열

형식 문자열은 모듈이 모든 변수를 인쇄하는 형식입니다. 대부분의 모듈에는 모듈의 표시 형식을 구성하는 `format`이라는 항목이 있습니다. 형식 문자열에서 텍스트, 변수 및 텍스트 그룹을 사용할 수 있습니다.

#### 변수

변수에는 `$` 기호와 변수 이름이 포함됩니다. 변수 이름은 문자, 숫자 및 `_`만 포함할 수 있습니다.

예를 들어:

- `'$version'`은 `version`이라는 변수가 있는 형식 문자열입니다.
- `'$git_branch$git_commit'`은 `git_branch` 및 `git_commit`이라는 두 변수가 있는 형식 문자열입니다.
- `'$git_branch $git_commit'`은 두 변수가 공백으로 구분됩니다.

#### 텍스트 그룹

텍스트 그룹은 두 가지 다른 부분으로 구성됩니다.

`[]`로 묶인 첫 번째 부분은 [형식 문자열](#format-strings)입니다. 텍스트, 변수 또는 중첩된 텍스트 그룹을 추가할 수 있습니다.

`()`로 묶인 두 번째 부분은 [스타일 문자열](#style-strings)입니다. 이는 첫 번째 부분의 스타일을 지정하는 데 사용할 수 있습니다.

예를 들어:

- `'[on](red bold)'`는 빨간색 굵은 글씨로 `on` 문자열을 인쇄합니다.
- `'[⌘ $version](bold green)'`은 `⌘` 기호 뒤에 `version` 변수의 내용이 녹색 굵은 글씨로 인쇄됩니다.
- `'[a [b](red) c](green)'`은 `b`는 빨간색, `a`와 `c`는 녹색으로 `a b c`를 인쇄합니다.

#### 스타일 문자열

Starship의 대부분의 모듈에 표시 스타일을 설정할 수 있습니다. 이는 설정을 지정하는 문자열인 항목(일반적으로 `style`이라고 함)으로 수행됩니다. 다음은 스타일 문자열과 그 기능의 몇 가지 예입니다. 전체 구문에 대한 자세한 내용은 [고급 설정 가이드](../advanced-config/)를 참조하세요.

- `'fg:green bg:blue'`는 파란색 배경에 녹색 텍스트를 설정합니다.
- `'bg:blue fg:bright-green'`은 파란색 배경에 밝은 녹색 텍스트를 설정합니다.
- `'bold fg:27'`은 [ANSI 색상](https://i.stack.imgur.com/KTSQa.png) 27로 굵은 텍스트를 설정합니다.
- `'underline bg:#bf5700'`은 불타는 주황색 배경에 밑줄이 있는 텍스트를 설정합니다.
- `'bold italic fg:purple'`은 굵은 이탤릭체 보라색 텍스트를 설정합니다.
- `''`는 모든 스타일을 명시적으로 비활성화합니다.

스타일이 어떻게 보이는지는 터미널 에뮬레이터에 따라 달라집니다. 예를 들어, 일부 터미널 에뮬레이터는 텍스트를 굵게 표시하는 대신 색상을 밝게 하고, 일부 색상 테마는 일반 색상과 밝은 색상에 동일한 값을 사용합니다. 또한 이탤릭체 텍스트를 얻으려면 터미널이 이탤릭체를 지원해야 합니다.

#### 조건부 형식 문자열

`()`로 묶인 조건부 형식 문자열은 내부의 모든 변수가 비어 있으면 렌더링되지 않습니다.

예를 들어:

- `'(@$region)'`은 `region` 변수가 `None`이거나 빈 문자열이면 아무것도 표시하지 않고, 그렇지 않으면 `@` 뒤에 region 값이 표시됩니다.
- `'(some text)'`는 괄호 안에 변수가 없으므로 항상 아무것도 표시하지 않습니다.
- `$combined`가 `\[$a$b\]`의 약어일 때, `'($combined)'`는 `$a`와 `$b`가 모두 `None`인 경우에만 아무것도 표시하지 않습니다. 이는 `'(\[$a$b\] )'`와 동일하게 작동합니다.

### 부정 일치

많은 모듈에는 `detect_extensions`, `detect_files` 및 `detect_folders` 변수가 있습니다. 이들은 일치시키거나 일치시키지 않을 문자열 목록을 사용합니다. 일치시키지 않아야 하는 "부정" 옵션은 선행 '!' 문자로 표시됩니다. 디렉토리에 _어떤_ 부정 표시기가 존재하면 모듈이 일치하지 않습니다.

확장자는 파일 이름의 마지막 점 뒤 문자열과 파일 이름의 첫 번째 점 뒤 문자열 모두에 대해 일치됩니다. 예를 들어, `foo.bar.tar.gz`는 `detect_extensions` 변수에서 `bar.tar.gz` 및 `gz`와 일치됩니다. 이름이 점으로 시작하는 파일은 확장자가 없는 것으로 간주됩니다.

이것이 실제로 어떻게 작동하는지 보려면 TypeScript는 일치시키지만 MPEG 전송 스트림 파일은 일치시키지 않을 수 있습니다:

```toml
detect_extensions = ['ts', '!video.ts', '!audio.ts']
```

## 프롬프트

다음은 프롬프트 전체 설정 옵션 목록입니다.

### 옵션

| 옵션            | 기본값                        | 설명                                                                                                                                                                        |
| --------------- | ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `format`          | [링크](#default-prompt-format) | 프롬프트의 형식을 구성합니다.                                                                                                                                                |
| `right_format`    | `''`                           | [오른쪽 프롬프트 활성화](../advanced-config/#enable-right-prompt) 참조                                                                                                                 |
| `scan_timeout`    | `30`                           | starship이 파일을 스캔하는 시간 초과 (밀리초).                                                                                                                              |
| `command_timeout` | `500`                          | starship이 실행하는 명령의 시간 초과 (밀리초).                                                                                                                       |
| `add_newline`     | `true`                         | 셸 프롬프트 사이에 빈 줄을 삽입합니다.                                                                                                                                          |
| `palette`         | `''`                           | `palettes`에서 사용할 색상 팔레트를 설정합니다.                                                                                                                                   |
| `palettes`        | `{}`                           | 사용자 정의 이름에 [색상](../advanced-config/#style-strings)을 할당하는 색상 팔레트 모음입니다. 색상 팔레트는 자체 색상 정의를 참조할 수 없습니다. |
| `follow_symlinks` | `true`                         | 심볼릭 링크를 따라가 디렉토리인지 확인합니다. git과 같은 모듈에서 사용됩니다.                                                                                                     |

::: tip

네트워크 파일 시스템에 대한 심볼릭 링크가 있는 경우 `follow_symlinks`를 `false`로 설정하는 것을 고려하세요.

:::

### 예시

```toml
# ~/.config/starship.toml

# 사용자 지정 형식 사용
format = '''
[┌───────────────────>](bold green)
[│](bold green)$directory$rust$package
[└─>](bold green) '''

# starship이 현재 디렉토리 아래의 파일을 확인하는 데 10밀리초를 기다립니다.
scan_timeout = 10

# 프롬프트 시작 부분의 빈 줄 비활성화
add_newline = false

# 'foo'를 사용자 지정 색상 팔레트로 설정
palette = 'foo'

# 사용자 지정 색상 정의
[palettes.foo]
# 기존 색상 덮어쓰기
blue = '21'
# 새 색상 정의
mustard = '#af8700'
```

### 기본 프롬프트 형식

기본 `format`은 `format`이 비어 있거나 제공되지 않은 경우 프롬프트의 형식을 정의하는 데 사용됩니다. 기본값은 다음과 같습니다:

```toml
format = '$all'

# 다음은 다음과 동일합니다.
format = """
$username\
$hostname\
$localip\
$shlvl\
$singularity\
$kubernetes\
$directory\
$vcsh\
$fossil_branch\
$fossil_metrics\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$hg_branch\
$pijul_channel\
$docker_context\
$package\
$c\
$cmake\
$cobol\
$daml\
$dart\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$fennel\
$gleam\
$golang\
$guix_shell\
$haskell\
$haxe\
$helm\
$java\
$julia\
$kotlin\
$gradle\
$lua\
$nim\
$nodejs\
$ocaml\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$quarto\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$solidity\
$swift\
$terraform\
$typst\
$vlang\
$vagrant\
$zig\
$buf\
$nix_shell\
$conda\
$meson\
$spack\
$memory_usage\
$aws\
$gcloud\
$openstack\
$azure\
$nats\
$direnv\
$env_var\
$mise\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$os\
$container\
$netns\
$shell\
$character"""
```

기본 형식을 확장하려면 `$all`을 사용할 수 있습니다. 형식에 명시적으로 추가하는 모듈은 중복되지 않습니다. 예:

```toml
# 디렉토리를 두 번째 줄로 이동
format = '$all$directory$character'
```

## AWS

`aws` 모듈은 임시 자격 증명을 사용할 때 현재 AWS 리전 및 프로필과 만료 타이머를 표시합니다. 모듈의 출력은 `AWS_REGION`, `AWS_DEFAULT_REGION` 및 `AWS_PROFILE` 환경 변수와 `~/.aws/config` 및 `~/.aws/credentials` 파일을 필요에 따라 사용합니다.

모듈은 자격 증명이 `~/.aws/credentials`에 있거나 `~/.aws/config`에 `credential_process`, `sso_start_url` 또는 `sso_session`이 정의된 경우에만 프로필을 표시합니다. 또는 `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` 또는 `AWS_SESSION_TOKEN` 환경 변수 중 하나가 정의된 경우에도 충분합니다. `force_display` 옵션이 `true`로 설정되면 위 조건에 따라 자격 증명이 감지되지 않아도 사용 가능한 모든 정보가 표시됩니다.

[aws-vault](https://github.com/99designs/aws-vault)를 사용할 때 프로필은 `AWS_VAULT` 환경 변수에서 읽고 자격 증명 만료 날짜는 `AWS_SESSION_EXPIRATION` 환경 변수에서 읽습니다.

[awsu](https://github.com/kreuzwerker/awsu)를 사용할 때 프로필은 `AWSU_PROFILE` 환경 변수에서 읽습니다.

[AWSume](https://awsu.me)를 사용할 때 프로필은 `AWSUME_PROFILE` 환경 변수에서 읽고 자격 증명 만료 날짜는 `AWSUME_EXPIRATION` 환경 변수에서 읽습니다.

[saml2aws](https://github.com/Versent/saml2aws)를 사용할 때 `~/.aws/credentials`에서 얻은 만료 정보는 `x_security_token_expires` 키로 대체됩니다.

[aws-sso-cli](https://github.com/synfinatic/aws-sso-cli)를 사용할 때 프로필은 `AWS_SSO_PROFILE` 환경 변수에서 읽습니다.

### 옵션

| 옵션              | 기본값                                                               | 설명                                                                                                 |
| ------------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol($profile )(\'($region\) )(\[$duration\] )]($style)'` | 모듈의 형식입니다.                                                                                  |
| `기호`                | `'☁️ '`                                                               | 현재 AWS 프로필을 표시하기 전에 사용되는 기호입니다.                                                  |
| `region_aliases`    | `{}`                                                                  | AWS 이름 외에 표시할 리전 별칭 테이블입니다.                                             |
| `profile_aliases`   | `{}`                                                                  | AWS 이름 외에 표시할 프로필 별칭 테이블입니다.                                            |
| `style`             | `'bold yellow'`                                                       | 모듈의 스타일입니다.                                                                                   |
| `expiration_symbol` | `'X'`                                                                 | 임시 자격 증명이 만료되었을 때 표시되는 기호입니다.                                           |
| `disabled`          | `false`                                                               | `AWS` 모듈을 비활성화합니다.                                                                                  |
| `force_display`     | `false`                                                               | `true`인 경우 `credentials`, `credential_process` 또는 `sso_start_url`이 설정되지 않아도 정보를 표시합니다. |

### 변수

| 변수      | 예시               | 설명                                 |
| --------- | ---------------- | ------------------------------------------- |
| region    | `ap-northeast-1` | 현재 AWS 리전                      |
| profile   | `astronauts`     | 현재 AWS 프로필                     |
| duration  | `2h27m20s`       | 임시 자격 증명 유효 기간 |
| 기호        |                  | `symbol` 옵션의 값을 반영합니다.        |
| style* |                  | `style` 옵션의 값을 반영합니다.         |

*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

#### 모든 항목 표시

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol($profile )(\'($region\) )]($style)'
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
[aws.profile_aliases]
CompanyGroupFrobozzOnCallAccess = 'Frobozz'
```

#### 리전 표시

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$region]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.region_aliases]
ap-southeast-2 = 'au'
us-east-1 = 'va'
```

#### 프로필 표시

```toml
# ~/.config/starship.toml

[aws]
format = 'on [$symbol$profile]($style) '
style = 'bold blue'
symbol = '🅰 '
[aws.profile_aliases]
Enterprise_Naming_Scheme-voidstars = 'void**'
```

## Azure

`azure` 모듈은 현재 Azure 구독을 표시합니다. 이는 `~/.azure/azureProfile.json` 파일에 정의된 기본 구독 이름 또는 사용자 이름을 표시하는 것을 기반으로 합니다.

### 옵션

| 변수               | 기본값                                  | 설명                                                                           |
| ---------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------- |
| `format`               | `'on [$symbol($subscription)]($style) '` | Azure 모듈을 렌더링하기 위한 형식입니다.                                            |
| `기호`                   | `'󰠅 '`                                   | 형식에 사용되는 기호입니다.                                                        |
| `style`                | `'blue bold'`                            | 형식에 사용되는 스타일입니다.                                                         |
| `disabled`             | `true`                                   | `azure` 모듈을 비활성화합니다.                                                          |
| `subscription_aliases` | `{}`                                     | Azure 구독 이름 외에 표시할 구독 이름 별칭 테이블입니다. |

### 예시

#### 구독 이름 표시

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = 'on [$symbol($subscription)]($style) '
symbol = '󰠅 '
style = 'blue bold'
```

#### 사용자 이름 표시

```toml
# ~/.config/starship.toml

[azure]
disabled = false
format = "on [$symbol($username)]($style) "
symbol = "󰠅 "
style = "blue bold"
```

#### 구독 이름 별칭 표시

```toml
# ~/.config/starship.toml

[azure.subscription_aliases]
very-long-subscription-name = 'vlsn'
```

## 배터리

`battery` 모듈은 장치의 배터리 충전량과 현재 충전 상태를 표시합니다. 이 모듈은 장치의 배터리가 10% 미만일 때만 표시됩니다.

### 옵션

| 옵션               | 기본값                           | 설명                                         |
| -------------------- | --------------------------------- | -------------------------------------------------- |
| `full_symbol`        | `'󰁹 '`                            | 배터리가 가득 찼을 때 표시되는 기호입니다.          |
| `charging_symbol`    | `'󰂄 '`                            | 배터리가 충전 중일 때 표시되는 기호입니다.      |
| `discharging_symbol` | `'󰂃 '`                            | 배터리가 방전 중일 때 표시되는 기호입니다.   |
| `unknown_symbol`     | `'󰁽 '`                            | 배터리 상태를 알 수 없을 때 표시되는 기호입니다. |
| `empty_symbol`       | `'󰂎 '`                            | 배터리 상태가 비어 있을 때 표시되는 기호입니다.   |
| `format`             | `'[$symbol$percentage]($style) '` | 모듈의 형식입니다.                          |
| `display`            | [링크](#battery-display)          | 모듈의 표시 임계값 및 스타일입니다.         |
| `disabled`           | `false`                           | `battery` 모듈을 비활성화합니다.                      |

### 예시

```toml
# ~/.config/starship.toml

[battery]
full_symbol = '🔋 '
charging_symbol = '⚡️ '
discharging_symbol = '💀 '
```

### 배터리 표시

`display` 설정 옵션은 배터리 표시기를 언제 표시할지 (임계값), 어떤 기호를 사용할지 (기호), 그리고 어떻게 보일지 (스타일)를 정의하는 데 사용됩니다. `display`가 제공되지 않으면 기본값은 다음과 같습니다:

```toml
[[battery.display]]
threshold = 10
style = 'bold red'
```

`charging_symbol` 및 `discharging_symbol` 옵션의 기본값은 각각 `battery`의 `charging_symbol` 및 `discharging_symbol` 옵션의 값입니다.

#### 옵션

`display` 옵션은 다음 테이블의 배열입니다.

| 옵션               | 기본값      | 설명                                                                                               |
| -------------------- | ------------ | --------------------------------------------------------------------------------------------------------- |
| `threshold`          | `10`         | 표시 옵션의 상한입니다.                                                                   |
| `style`              | `'red bold'` | 표시 옵션이 사용 중일 때 사용되는 스타일입니다.                                                           |
| `charging_symbol`    |              | 표시 옵션이 사용 중일 때 표시되는 선택적 기호이며, 기본값은 배터리의 `charging_symbol` 옵션입니다.    |
| `discharging_symbol` |              | 표시 옵션이 사용 중일 때 표시되는 선택적 기호이며, 기본값은 배터리의 `discharging_symbol` 옵션입니다. |

#### 예시

```toml
[[battery.display]] # 용량이 0%에서 10% 사이일 때 'bold red' 스타일 및 discharging_symbol
threshold = 10
style = 'bold red'

[[battery.display]] # 용량이 10%에서 30% 사이일 때 'bold yellow' 스타일 및 💦 기호
threshold = 30
style = 'bold yellow'
discharging_symbol = '💦 '

# 용량이 30%를 초과하면 배터리 표시기가 표시되지 않습니다.
```

## Buf

`buf` 모듈은 현재 설치된 [Buf](https://buf.build) 버전을 표시합니다. 기본적으로 현재 디렉토리에 [`buf.yaml`](https://docs.buf.build/configuration/v1/buf-yaml), [`buf.gen.yaml`](https://docs.buf.build/configuration/v1/buf-gen-yaml) 또는 [`buf.work.yaml`](https://docs.buf.build/configuration/v1/buf-work-yaml) 설정 파일이 포함되어 있으면 모듈이 표시됩니다.

### 옵션

| 옵션              | 기본값                                         | 설명                                           |
| ------------------- | ----------------------------------------------- | ----------------------------------------------------- |
| `format`            | `'with [$symbol($version )]($style)'`           | `buf` 모듈의 형식입니다.                      |
| `version_format`    | `'v${raw}'`                                     | 버전 형식입니다.                                   |
| `기호`                | `'🐃 '`                                          | Buf 버전을 표시하기 전에 사용되는 기호입니다. |
| `detect_extensions` | `[]`                                            | 이 모듈을 트리거해야 하는 확장자입니다.          |
| `detect_files`      | `['buf.yaml', 'buf.gen.yaml', 'buf.work.yaml']` | 이 모듈을 트리거해야 하는 파일 이름입니다.           |
| `detect_folders`    | `[]`                                            | 이 모듈을 트리거해야 하는 폴더입니다.            |
| `style`             | `'bold blue'`                                   | 모듈의 스타일입니다.                             |
| `disabled`          | `false`                                         | `elixir` 모듈을 비활성화합니다.                         |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| `version` | `v1.0.0` | `buf`의 버전                 |
| `기호`      |          | `symbol` 옵션의 값을 반영합니다. |
| `style`*  |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[buf]
symbol = '🦬 '
```

## Bun

`bun` 모듈은 현재 설치된 [bun](https://bun.sh) JavaScript 런타임 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `bun.lock` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `bun.lockb` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `bunfig.toml` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                                    | 설명                                                               |
| ------------------- | ------------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`       | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                                | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'🥟 '`                                     | Bun의 기호를 나타내는 형식 문자열입니다.                           |
| `detect_extensions` | `[]`                                       | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['bun.lock', 'bun.lockb', 'bunfig.toml']` | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                       | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `style`             | `'bold red'`                               | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                    | `bun` 모듈을 비활성화합니다.                                                |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v0.1.4` | `bun`의 버전                 |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

#### 형식 사용자 지정

```toml
# ~/.config/starship.toml

[bun]
format = 'via [🍔 $version](bold green) '
```

## C

`c` 모듈은 C 컴파일러에 대한 일부 정보를 표시합니다. 기본적으로 현재 디렉토리에 `.c` 또는 `.h` 파일이 포함되어 있으면 모듈이 표시됩니다.

### 옵션

| 옵션              | 기본값                                                                       | 설명                                                               |
| ------------------- | ----------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                  | 모듈의 형식 문자열입니다.                                         |
| `version_format`    | `'v${raw}'`                                                                   | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'C '`                                                                        | 컴파일러 세부 정보를 표시하기 전에 사용되는 기호입니다.                    |
| `detect_extensions` | `['c', 'h']`                                                                  | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `[]`                                                                          | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                                                          | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `commands`          | `[ [ 'cc', '--version' ], [ 'gcc', '--version' ], [ 'clang', '--version' ] ]` | 컴파일러가 무엇인지 감지하는 방법입니다.                                        |
| `style`             | `'bold 149'`                                                                  | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                                                       | `c` 모듈을 비활성화합니다.                                                  |

### 변수

| 변수   | 예시     | 설명                                 |
| -------- | -------- | ------------------------------------ |
| name     | clang  | 컴파일러의 이름             |
| version  | 13.0.0 | 컴파일러의 버전          |
| 기호       |        | `symbol` 옵션의 값을 반영합니다. |
| style    |        | `style` 옵션의 값을 반영합니다.  |

참고: `version`은 기본 형식에 없습니다.

### 명령어

`commands` 옵션은 컴파일러 버전과 이름을 결정하는 명령 목록을 허용합니다.

각 명령은 실행 파일 이름과 그 뒤에 인수가 오는 목록으로 표현되며, 일반적으로 `['mycc', '--version']`과 같습니다. Starship은 STDOUT에서 결과를 얻을 때까지 각 명령을 실행하려고 시도합니다.

이 모듈에서 C 컴파일러가 지원되지 않는 경우 [GitHub에 이슈를 제기](https://github.com/starship/starship/)하여 요청할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[c]
format = 'via [$name $version]($style)'
```

## CPP

`cpp` 모듈은 C++ 컴파일러에 대한 일부 정보를 표시합니다. 기본적으로 현재 디렉토리에 `.cpp`, `.hpp` 또는 기타 C++ 관련 파일이 포함되어 있으면 모듈이 표시됩니다.

### 옵션

| 옵션              | 기본값                                                                          | 설명                                                               |
| ------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version(-$name) )]($style)'`                                     | 모듈의 형식 문자열입니다.                                         |
| `version_format`    | `'v${raw}'`                                                                      | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'C++ '`                                                                         | 컴파일러 세부 정보를 표시하기 전에 사용되는 기호입니다.                    |
| `detect_extensions` | `['cpp', 'cc', 'cxx', 'c++', 'hpp', 'hh', 'hxx', 'h++', 'tcc']`                  | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `[]`                                                                             | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                                                             | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `commands`          | `[ [ 'c++', '--version' ], [ 'g++', '--version' ], [ 'clang++', '--version' ] ]` | 컴파일러가 무엇인지 감지하는 방법입니다.                                        |
| `style`             | `'bold 149'`                                                                     | 모듈의 스타일입니다.                                                 |
| `disabled`          | `true`                                                                           | `cpp` 모듈을 비활성화합니다.                                                |

### 변수

| 변수   | 예시      | 설명                                 |
| -------- | ------- | ------------------------------------ |
| name     | clang++ | 컴파일러의 이름             |
| version  | 13.0.0  | 컴파일러의 버전          |
| 기호       |         | `symbol` 옵션의 값을 반영합니다. |
| style    |         | `style` 옵션의 값을 반영합니다.  |

참고: `version`은 기본 형식에 없습니다.

### 명령어

`commands` 옵션은 컴파일러 버전과 이름을 결정하는 명령 목록을 허용합니다.

각 명령은 실행 파일 이름과 그 뒤에 인수가 오는 목록으로 표현되며, 일반적으로 `['mycpp', '--version']`과 같습니다. Starship은 STDOUT에서 결과를 얻을 때까지 각 명령을 실행하려고 시도합니다.

이 모듈에서 C++ 컴파일러가 지원되지 않는 경우 [GitHub에 이슈를 제기](https://github.com/starship/starship/)하여 요청할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[cpp]
disabled = false
format = 'via [$name $version]($style)'
```

## 문자

`character` 모듈은 터미널에 텍스트가 입력되는 위치 옆에 문자(일반적으로 화살표)를 표시합니다.

이 문자는 마지막 명령이 성공했는지 여부를 알려줍니다. 이는 두 가지 방법으로 수행할 수 있습니다:

- 색상 변경 (`red`/`green`)
- 모양 변경 (`❯`/`✖`)

기본적으로 색상만 변경됩니다. 모양도 변경하려면 [이 예시](#with-custom-error-shape)를 참조하세요.

::: warning

`vimcmd_symbol`은 cmd, fish 및 zsh에서만 지원됩니다. `vimcmd_replace_one_symbol`, `vimcmd_replace_symbol` 및 `vimcmd_visual_symbol`은 zsh의 모드 감지 관련 [상위 문제](https://github.com/starship/starship/issues/625#issuecomment-732454148)로 인해 fish에서만 지원됩니다.

:::

### 옵션

| 옵션                      | 기본값              | 설명                                                                             |
| --------------------------- | -------------------- | --------------------------------------------------------------------------------------- |
| `format`                    | `'$symbol '`         | 텍스트 입력 전에 사용되는 형식 문자열입니다.                                           |
| `success_symbol`            | `'[❯](bold green)'`  | 이전 명령이 성공했을 때 텍스트 입력 전에 사용되는 형식 문자열입니다.         |
| `error_symbol`              | `'[❯](bold red)'`    | 이전 명령이 실패했을 때 텍스트 입력 전에 사용되는 형식 문자열입니다.            |
| `vimcmd_symbol`             | `'[❮](bold green)'`  | 셸이 vim 일반 모드일 때 텍스트 입력 전에 사용되는 형식 문자열입니다.        |
| `vimcmd_replace_one_symbol` | `'[❮](bold purple)'` | 셸이 vim `replace_one` 모드일 때 텍스트 입력 전에 사용되는 형식 문자열입니다. |
| `vimcmd_replace_symbol`     | `'[❮](bold purple)'` | 셸이 vim replace 모드일 때 텍스트 입력 전에 사용되는 형식 문자열입니다.       |
| `vimcmd_visual_symbol`      | `'[❮](bold yellow)'` | 셸이 vim visual 모드일 때 텍스트 입력 전에 사용되는 형식 문자열입니다.        |
| `disabled`                  | `false`              | `character` 모듈을 비활성화합니다.                                                        |

### 변수

| 변수   | 예시 | 설명                                                                                              |
| -------- | -------- | ---------------------------------------------------------------------------------------------------------------- |
| `기호`       |    | `success_symbol`, `error_symbol`, `vimcmd_symbol` 또는 `vimcmd_replace_one_symbol` 등의 값을 반영합니다. |

### 예시

#### 사용자 지정 오류 모양 사용

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[✗](bold red) '
```

#### 사용자 지정 오류 모양 미사용

```toml
# ~/.config/starship.toml

[character]
success_symbol = '[➜](bold green) '
error_symbol = '[➜](bold red) '
```

#### 사용자 지정 vim 모양 사용

```toml
# ~/.config/starship.toml

[character]
vimcmd_symbol = '[V](bold green) '
```

## CMake

`cmake` 모듈은 현재 설치된 [CMake](https://cmake.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 활성화됩니다:

- 현재 디렉토리에 `CMakeLists.txt` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `CMakeCache.txt` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                                | 설명                                                               |
| ------------------- | -------------------------------------- | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`   | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                            | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'△ '`                                 | cmake 버전을 표시하기 전에 사용되는 기호입니다.                              |
| `detect_extensions` | `[]`                                   | 이 모듈을 트리거해야 하는 확장자입니다.                               |
| `detect_files`      | `['CMakeLists.txt', 'CMakeCache.txt']` | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                   | 이 모듈을 트리거해야 하는 폴더입니다.                                  |
| `style`             | `'bold blue'`                          | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                | `cmake` 모듈을 비활성화합니다.                                              |

### 변수

| 변수      | 예시        | 설명                                 |
| --------- | --------- | ------------------------------------ |
| version   | `v3.17.3` | cmake의 버전                 |
| 기호        |           | `symbol` 옵션의 값을 반영합니다. |
| style* |           | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

## COBOL / GNUCOBOL

`cobol` 모듈은 현재 설치된 COBOL 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `.cob` 또는 `.COB`로 끝나는 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.cbl` 또는 `.CBL`로 끝나는 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `기호`                | `'⚙️ '`                              | COBOL 버전을 표시하기 전에 사용되는 기호입니다.                   |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `style`             | `'bold blue'`                        | 모듈의 스타일입니다.                                                 |
| `detect_extensions` | `['cbl', 'cob', 'CBL', 'COB']`       | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `[]`                                 | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `disabled`          | `false`                              | `cobol` 모듈을 비활성화합니다.                                              |

### 변수

| 변수      | 예시         | 설명                                 |
| --------- | ---------- | ------------------------------------ |
| version   | `v3.1.2.0` | `cobol`의 버전               |
| 기호        |            | `symbol` 옵션의 값을 반영합니다. |
| style* |            | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

## 명령 지속 시간

`cmd_duration` 모듈은 마지막 명령이 실행되는 데 걸린 시간을 표시합니다. 이 모듈은 명령이 2초보다 오래 걸렸거나 `min_time` 설정 값이 존재하는 경우에만 표시됩니다.

::: warning

Bash에서 DEBUG 트랩을 후킹하지 마세요

`bash`에서 Starship을 실행하는 경우 `eval $(starship init $0)`을 실행한 후 `DEBUG` 트랩을 후킹하지 마세요. 그렇지 않으면 이 모듈이 **작동하지 않습니다**.

:::

preexec와 유사한 기능이 필요한 Bash 사용자는 [rcaloras의 bash_preexec 프레임워크](https://github.com/rcaloras/bash-preexec)를 사용할 수 있습니다. `eval $(starship init $0)`을 실행하기 전에 `preexec_functions` 및 `precmd_functions` 배열을 정의한 다음 정상적으로 진행하면 됩니다.

### 옵션

| 옵션                 | 기본값                       | 설명                                                                                                                                                       |
| ---------------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `min_time`             | `2_000`                       | 시간을 표시할 최소 지속 시간 (밀리초).                                                                                                             |
| `show_milliseconds`    | `false`                       | 지속 시간에 초 외에 밀리초를 표시합니다.                                                                                                        |
| `format`               | `'took [$duration]($style) '` | 모듈의 형식입니다.                                                                                                                                        |
| `style`                | `'bold yellow'`               | 모듈의 스타일입니다.                                                                                                                                         |
| `disabled`             | `false`                       | `cmd_duration` 모듈을 비활성화합니다.                                                                                                                               |
| `show_notifications`   | `false`                       | 명령이 완료될 때 데스크톱 알림을 표시합니다.                                                                                                                |
| `min_time_to_notify`   | `45_000`                      | 알림을 표시할 최소 지속 시간 (밀리초).                                                                                                             |
| `notification_timeout` |                               | 알림을 표시할 지속 시간 (밀리초). 설정되지 않은 경우 알림 시간 초과는 데몬에 의해 결정됩니다. 모든 알림 데몬이 이 옵션을 따르는 것은 아닙니다. |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | --------------------------------------- |
| duration  | `16m40s` | 명령 실행에 걸린 시간 |
| style* |          | `style` 옵션의 값을 반영합니다.     |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[cmd_duration]
min_time = 500
format = 'underwent [$duration](bold yellow)'
```

## Conda

`conda` 모듈은 `$CONDA_DEFAULT_ENV`가 설정된 경우 현재 [Conda](https://docs.conda.io/en/latest/) 환경을 표시합니다.

::: tip

이것은 conda 자체의 프롬프트 수정자를 억제하지 않습니다. `conda config --set changeps1 False`를 실행할 수 있습니다. [pixi](https://pixi.sh)를 사용하는 경우 `pixi config set change-ps1 false`를 실행하여 pixi의 프롬프트 수정자를 비활성화할 수 있습니다.

:::

### 옵션

| 옵션              | 기본값                                | 설명                                                                                                                                                                                                 |
| ------------------- | -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `truncation_length` | `1`                                    | 환경이 `conda create -p [path]`를 통해 생성된 경우 환경 경로가 잘릴 디렉토리 수입니다. `0`은 자르지 않음을 의미합니다. [`directory`](#directory) 모듈도 참조하세요. |
| `기호`                | `'🅒 '`                                 | 환경 이름 앞에 사용되는 기호입니다.                                                                                                                                                                |
| `style`             | `'bold green'`                         | 모듈의 스타일입니다.                                                                                                                                                                                   |
| `format`            | `'via [$symbol$environment]($style) '` | 모듈의 형식입니다.                                                                                                                                                                                  |
| `ignore_base`       | `true`                                 | 활성화될 때 `base` 환경을 무시합니다.                                                                                                                                                                  |
| `detect_env_vars`   | `["!PIXI_ENVIRONMENT_NAME"]`           | 이 모듈을 트리거해야 하는 환경 변수입니다. pixi 환경인 경우 이 모듈은 기본적으로 트리거되지 않습니다.                                                                        |
| `disabled`          | `false`                                | `conda` 모듈을 비활성화합니다.                                                                                                                                                                                |

### 변수

| 변수        | 예시           | 설명                                 |
| ----------- | ------------ | ------------------------------------ |
| environment | `astronauts` | 현재 conda 환경        |
| 기호          |              | `symbol` 옵션의 값을 반영합니다. |
| style*   |              | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[conda]
format = '[$symbol$environment](dimmed green) '
```

## 컨테이너

`container` 모듈은 컨테이너 내부에 있을 경우 기호와 컨테이너 이름을 표시합니다.

### 옵션

| 옵션     | 기본값                            | 설명                               |
| ---------- | ---------------------------------- | ----------------------------------------- |
| `기호`       | `'⬢'`                              | 컨테이너 내부에 있을 때 표시되는 기호입니다. |
| `style`    | `'bold red dimmed'`                | 모듈의 스타일입니다.                 |
| `format`   | `'[$symbol \[$name\]]($style) '` | 모듈의 형식입니다.                |
| `disabled` | `false`                            | `container` 모듈을 비활성화합니다.          |

### 변수

| 변수      | 예시                  | 설명                                 |
| --------- | ------------------- | ------------------------------------ |
| name      | `fedora-toolbox:35` | 컨테이너의 이름            |
| 기호        |                     | `symbol` 옵션의 값을 반영합니다. |
| style* |                     | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[container]
format = '[$symbol \[$name\]]($style) '
```

## Crystal

`crystal` 모듈은 현재 설치된 [Crystal](https://crystal-lang.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `shard.yml` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.cr` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `기호`                | `'🔮 '`                               | crystal 버전을 표시하기 전에 사용되는 기호입니다.                 |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `style`             | `'bold red'`                         | 모듈의 스타일입니다.                                                 |
| `detect_extensions` | `['cr']`                             | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['shard.yml']`                      | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `disabled`          | `false`                              | `crystal` 모듈을 비활성화합니다.                                            |

### 변수

| 변수      | 예시        | 설명                                 |
| --------- | --------- | ------------------------------------ |
| version   | `v0.32.1` | `crystal`의 버전             |
| 기호        |           | `symbol` 옵션의 값을 반영합니다. |
| style* |           | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[crystal]
format = 'via [✨ $version](bold blue) '
```

## Daml

`daml` 모듈은 Daml 프로젝트의 루트 디렉토리에 있을 때 현재 사용 중인 [Daml](https://www.digitalasset.com/developers) SDK 버전을 표시합니다. `daml.yaml` 파일의 `sdk-version`은 `DAML_SDK_VERSION` 환경 변수에 의해 재정의되지 않는 한 사용됩니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `daml.yaml` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'Λ '`                               | Daml의 기호를 나타내는 형식 문자열입니다.                           |
| `style`             | `'bold cyan'`                        | 모듈의 스타일입니다.                                                 |
| `detect_extensions` | `[]`                                 | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['daml.yaml']`                      | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `disabled`          | `false`                              | `daml` 모듈을 비활성화합니다.                                               |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v2.2.0` | `daml`의 버전                |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[daml]
format = 'via [D $version](bold bright-green) '
```

## Dart

`dart` 모듈은 현재 설치된 [Dart](https://dart.dev/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `.dart` 확장자를 가진 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.dart_tool` 디렉토리가 포함되어 있습니다.
- 현재 디렉토리에 `pubspec.yaml`, `pubspec.yml` 또는 `pubspec.lock` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                                           | 설명                                                               |
| ------------------- | ------------------------------------------------- | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`              | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                                       | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'🎯 '`                                            | Dart의 기호를 나타내는 형식 문자열입니다.                           |
| `detect_extensions` | `['dart']`                                        | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['pubspec.yaml', 'pubspec.yml', 'pubspec.lock']` | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `['.dart_tool']`                                  | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `style`             | `'bold blue'`                                     | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                           | `dart` 모듈을 비활성화합니다.                                               |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v2.8.4` | `dart`의 버전                |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[dart]
format = 'via [🔰 $version](bold red) '
```

## Deno

`deno` 모듈은 현재 설치된 [Deno](https://deno.land/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `deno.json`, `deno.jsonc`, `deno.lock`, `mod.ts`, `mod.js`, `deps.ts` 또는 `deps.js` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                                                                              | 설명                                                               |
| ------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                 | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                                                                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'🦕 '`                                                                               | Deno의 기호를 나타내는 형식 문자열입니다.                           |
| `detect_extensions` | `[]`                                                                                 | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['deno.json', 'deno.jsonc', 'deno.lock', 'mod.ts', 'mod.js', 'deps.ts', 'deps.js']` | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                                                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `style`             | `'green bold'`                                                                       | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                                                              | `deno` 모듈을 비활성화합니다.                                               |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v1.8.3` | `deno`의 버전                |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[deno]
format = 'via [🦕 $version](green bold) '
```

## 디렉토리

`directory` 모듈은 현재 디렉토리의 경로를 세 개의 상위 폴더로 잘라 표시합니다. 또한 현재 있는 git 리포지토리의 루트로 디렉토리가 잘립니다.

`fish_style_pwd_dir_length` 옵션을 사용하면 잘린 경로를 숨기는 대신, 옵션에 대해 활성화한 숫자를 기반으로 각 디렉토리의 단축된 이름을 볼 수 있습니다.

예를 들어, `~/Dev/Nix/nixpkgs/pkgs`에서 `nixpkgs`가 리포지토리 루트이고 옵션이 `1`로 설정된 경우, 이전에는 `nixpkgs/pkgs`였던 것이 이제 `~/D/N/nixpkgs/pkgs`로 표시됩니다. 즉, 일반적으로 제거되는 경로 구성 요소가 단일 문자로 표시됩니다. `fish_style_pwd_dir_length = 2`일 때는 `/bu/th/ci/on/rock/and/roll`이 됩니다.

### 옵션

| 옵션                   | 기본값                                                                                                                      | 설명                                                                                                |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `truncation_length`      | `3`                                                                                                                          | 현재 디렉토리가 잘릴 상위 폴더 수입니다.                            |
| `truncate_to_repo`       | `true`                                                                                                                       | 현재 있는 git 리포지토리의 루트로 자를지 여부입니다.                           |
| `format`                 | `'[$path]($style)[$read_only]($read_only_style) '`                                                                           | 모듈의 형식입니다.                                                                                 |
| `style`                  | `'bold cyan'`                                                                                                                | 모듈의 스타일입니다.                                                                                  |
| `disabled`               | `false`                                                                                                                      | `directory` 모듈을 비활성화합니다.                                                                           |
| `read_only`              | `'🔒'`                                                                                                                        | 현재 디렉토리가 읽기 전용임을 나타내는 기호입니다.                                                      |
| `read_only_style`        | `'red'`                                                                                                                      | 읽기 전용 기호의 스타일입니다.                                                                        |
| `truncation_symbol`      | `''`                                                                                                                         | 잘린 경로 앞에 붙일 기호입니다. 예: '…/'                                                          |
| `before_repo_root_style` |                                                                                                                              | git 리포지토리 루트 위의 경로 세그먼트 스타일입니다. 기본값은 `style`과 동일합니다. |
| `repo_root_style`        |                                                                                                                              | git 리포지토리 루트의 스타일입니다. 기본값은 `style`과 동일합니다.                        |
| `repo_root_format`       | `'[$before_root_path]($before_repo_root_style)[$repo_root]($repo_root_style)[$path]($style)[$read_only]($read_only_style) '` | `before_repo_root_style` 및 `repo_root_style`이 정의된 경우 git 리포지토리의 형식입니다.                   |
| `home_symbol`            | `'~'`                                                                                                                        | 홈 디렉토리를 나타내는 기호입니다.                                                                      |
| `use_os_path_sep`        | `true`                                                                                                                       | 항상 `/` 대신 OS별 경로 구분 기호를 사용합니다 (예: Windows의 `").                    |

<details>
<summary>이 모듈에는 디렉토리가 표시되는 방식을 제어하는 몇 가지 고급 설정 옵션이 있습니다.</summary>

| 고급 옵션             | 기본값 | 설명                                                                                                                                                            |
| --------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `substitutions`             |         | 경로에서 발생하는 리터럴 문자열에 대한 대체 테이블입니다.                                                                                                                       |
| `fish_style_pwd_dir_length` | `0`     | fish 셸 pwd 경로 논리를 적용할 때 사용할 문자 수입니다.                                                                                               |
| `use_logical_path`          | `true`  | `true`인 경우 셸을 통해 `PWD` 또는 `--logical-path`에서 가져온 논리 경로를 렌더링합니다. `false`인 경우 심볼릭 링크가 해결된 물리적 파일 시스템 경로를 렌더링합니다. |

`substitutions`를 사용하면 경로에 나타나는 리터럴 문자열(예: 긴 네트워크 접두사 또는 Java 개발 디렉토리)에 대한 임의의 대체를 정의할 수 있습니다. 이 경우 fish 스타일 PWD가 비활성화됩니다.

```toml
[directory.substitutions]
'/Volumes/network/path' = '/net'
'src/com/long/java/path' = 'mypath'
```

`fish_style_pwd_dir_length`는 표준 자르기 옵션과 상호 작용하여 처음에는 놀라울 수 있습니다. 0이 아닌 경우 일반적으로 잘리는 경로 구성 요소는 해당 문자 수만큼 표시됩니다. 예를 들어, 일반적으로 `rock/and/roll`로 표시되는 `/built/this/city/on/rock/and/roll` 경로는 `fish_style_pwd_dir_length = 1`일 때 `/b/t/c/o/rock/and/roll`로 표시됩니다. 즉, 일반적으로 제거되는 경로 구성 요소가 단일 문자로 표시됩니다. `fish_style_pwd_dir_length = 2`일 때는 `/bu/th/ci/on/rock/and/roll`이 됩니다.

</details>

### 변수

| 변수      | 예시                    | 설명                                 |
| --------- | --------------------- | ----------------------------------- |
| path      | `'D:/Projects'`       | 현재 디렉토리 경로          |
| style* | `'black bold dimmed'` | `style` 옵션의 값을 반영합니다. |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

<details>
<summary>git 리포지토리에는 추가 변수가 있습니다.</summary>

`/path/to/home/git_repo/src/lib` 경로를 고려해 봅시다.

| 변수               | 예시                    | 설명                                 |
| ------------------ | --------------------- | --------------------------------------- |
| before_root_path | `'/path/to/home/'`    | git 루트 디렉토리 경로 이전 경로 |
| repo_root          | `'git_repo'`          | git 루트 디렉토리 이름             |
| path               | `'/src/lib'`          | 나머지 경로                      |
| style              | `'black bold dimmed'` | `style` 옵션의 값을 반영합니다.     |
| repo_root_style  | `'underline white'`   | git 루트 디렉토리 이름의 스타일       |

</details>

### 예시

```toml
# ~/.config/starship.toml

[directory]
truncation_length = 8
truncation_symbol = '…/'
```

## Direnv

`direnv` 모듈은 현재 rc 파일이 존재할 경우 해당 파일의 상태를 표시합니다. 상태에는 rc 파일의 경로, 로드 여부, `direnv`에 의해 허용되었는지 여부가 포함됩니다.

### 옵션

| 옵션              | 기본값                                | 설명                                             |
| ------------------- | -------------------------------------- | ------------------------------------------------------- |
| `format`            | `'[$symbol$loaded/$allowed]($style) '` | 모듈의 형식입니다.                              |
| `기호`                | `'direnv '`                            | direnv 컨텍스트를 표시하기 전에 사용되는 기호입니다.   |
| `style`             | `'bold orange'`                        | 모듈의 스타일입니다.                               |
| `disabled`          | `true`                                 | `direnv` 모듈을 비활성화합니다.                           |
| `detect_extensions` | `[]`                                   | 이 모듈을 트리거해야 하는 확장자입니다.            |
| `detect_files`      | `['.envrc']`                           | 이 모듈을 트리거해야 하는 파일 이름입니다.             |
| `detect_folders`    | `[]`                                   | 이 모듈을 트리거해야 하는 폴더입니다.               |
| `detect_env_vars`   | `['DIRENV_FILE']`                      | 이 모듈을 트리거해야 하는 환경 변수입니다. |
| `allowed_msg`       | `'allowed'`                            | rc 파일이 허용되었을 때 표시되는 메시지입니다.       |
| `not_allowed_msg`   | `'not allowed'`                        | rc 파일이 허용되지 않았을 때 표시되는 메시지입니다.   |
| `denied_msg`        | `'denied'`                             | rc 파일이 거부되었을 때 표시되는 메시지입니다.        |
| `loaded_msg`        | `'loaded'`                             | rc 파일이 로드되었을 때 표시되는 메시지입니다.        |
| `unloaded_msg`      | `'not loaded'`                         | rc 파일이 로드되지 않았을 때 표시되는 메시지입니다.    |

### 변수

| 변수      | 예시                  | 설명                                 |
| --------- | ------------------- | --------------------------------------- |
| loaded    | `loaded`            | 현재 rc 파일이 로드되었는지 여부입니다.  |
| allowed   | `denied`            | 현재 rc 파일이 허용되었는지 여부입니다. |
| rc_path   | `'/home/test/.envrc'` | 현재 rc 파일 경로입니다.               |
| 기호        |                     | `symbol` 옵션의 값을 반영합니다.   |
| style* | `red bold`          | `style` 옵션의 값을 반영합니다.    |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[direnv]
disabled = false
```

## Docker 컨텍스트

`docker_context` 모듈은 현재 활성 [Docker 컨텍스트](https://docs.docker.com/engine/context/working-with-contexts/)를 표시합니다. `default` 또는 `desktop-linux`로 설정되지 않았거나 `DOCKER_MACHINE_NAME`, `DOCKER_HOST` 또는 `DOCKER_CONTEXT` 환경 변수가 설정된 경우 (사용 중인 컨텍스트를 재정의하기 위한 것임) 표시됩니다.

### 옵션

| 옵션              | 기본값                                                       | 설명                                                                       |
| ------------------- | ------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol$context]($style) '`                            | 모듈의 형식입니다.                                                        |
| `기호`                | `'🐳 '`                                                        | Docker 컨텍스트를 표시하기 전에 사용되는 기호입니다.                             |
| `only_with_files`   | `true`                                                        | 일치하는 항목이 있을 때만 표시합니다.                                                    |
| `detect_extensions` | `[]`                                                          | 이 모듈을 트리거해야 하는 확장자입니다 (`only_with_files`가 true여야 함). |
| `detect_files`      | `['docker-compose.yml', 'docker-compose.yaml', 'Dockerfile']` | 이 모듈을 트리거해야 하는 파일 이름입니다 (`only_with_files`가 true여야 함).  |
| `detect_folders`    | `[]`                                                          | 이 모듈을 트리거해야 하는 폴더입니다 (`only_with_files`가 true여야 함).    |
| `style`             | `'blue bold'`                                                 | 모듈의 스타일입니다.                                                         |
| `disabled`          | `false`                                                       | `docker_context` 모듈을 비활성화합니다.                                             |

### 변수

| 변수      | 예시             | 설명                                 |
| --------- | -------------- | ------------------------------------ |
| context   | `test_context` | 현재 docker 컨텍스트           |
| 기호        |                | `symbol` 옵션의 값을 반영합니다. |
| style* |                | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[docker_context]
format = 'via [🐋 $context](blue bold)'
```

## Dotnet

`dotnet` 모듈은 현재 디렉토리에 대한 관련 [.NET Core SDK](https://dotnet.microsoft.com/) 버전을 표시합니다. SDK가 현재 디렉토리에 고정되어 있으면 고정된 버전이 표시됩니다. 그렇지 않으면 모듈은 SDK의 최신 설치 버전을 표시합니다.

기본적으로 이 모듈은 다음 파일 중 하나 이상이 현재 디렉토리에 있을 때만 프롬프트에 표시됩니다:

- `global.json`
- `project.json`
- `Directory.Build.props`
- `Directory.Build.targets`
- `Packages.props`
- `*.csproj`
- `*.fsproj`
- `*.xproj`

또한 올바르게 사용하려면 .NET Core SDK가 설치되어 있어야 합니다.

내부적으로 이 모듈은 자체 버전 감지 메커니즘을 사용합니다. 일반적으로 `dotnet --version`을 실행하는 것보다 두 배 빠르지만, .NET 프로젝트의 디렉토리 구조가 특이한 경우 잘못된 버전이 표시될 수 있습니다. 정확성이 속도보다 중요한 경우 모듈 옵션에서 `heuristic = false`로 설정하여 메커니즘을 비활성화할 수 있습니다.

이 모듈은 현재 디렉토리에 `.csproj` 파일이 있을 때 대상 프레임워크 모니커(<https://docs.microsoft.com/en-us/dotnet/standard/frameworks#supported-target-frameworks>)도 표시합니다.

### 옵션

| 옵션              | 기본값                                                                                                 | 설명                                                               |
| ------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )(🎯 $tfm )]($style)'`                                                           | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                                                                                             | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'.NET '`                                                                                               | dotnet 버전을 표시하기 전에 사용되는 기호입니다.                  |
| `heuristic`         | `true`                                                                                                  | starship을 빠르게 유지하기 위해 더 빠른 버전 감지를 사용합니다.                     |
| `detect_extensions` | `['csproj', 'fsproj', 'xproj']`                                                                         | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['global.json', 'project.json', 'Directory.Build.props', 'Directory.Build.targets', 'Packages.props']` | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                                                                                    | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `style`             | `'bold blue'`                                                                                           | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                                                                                 | `dotnet` 모듈을 비활성화합니다.                                             |

### 변수

| 변수      | 예시               | 설명                                                        |
| --------- | ---------------- | ------------------------------------------------------------------ |
| version   | `v3.1.201`       | `dotnet` sdk의 버전                                        |
| tfm       | `netstandard2.0` | 현재 프로젝트가 대상으로 하는 대상 프레임워크 모니커 |
| 기호        |                  | `symbol` 옵션의 값을 반영합니다.                               |
| style* |                  | `style` 옵션의 값을 반영합니다.                                |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[dotnet]
symbol = '🥅 '
style = 'green'
heuristic = false
```

## Elixir

`elixir` 모듈은 현재 설치된 [Elixir](https://elixir-lang.org/) 및 [Erlang/OTP](https://erlang.org/doc/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `mix.exs` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                                                     | 설명                                                               |
| ------------------- | ----------------------------------------------------------- | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version \(OTP $otp_version\) )]($style)'` | 모듈 elixir의 형식입니다.                                         |
| `version_format`    | `'v${raw}'`                                                 | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'💧 '`                                                      | Elixir/Erlang 버전을 표시하기 전에 사용되는 기호입니다.           |
| `detect_extensions` | `[]`                                                        | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['mix.exs']`                                               | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                                        | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `style`             | `'bold purple'`                                             | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                                     | `elixir` 모듈을 비활성화합니다.                                             |

### 변수

| 변수        | 예시      | 설명                                 |
| ----------- | ------- | ------------------------------------ |
| version     | `v1.10` | `elixir`의 버전              |
| otp_version |         | `elixir`의 otp 버전          |
| 기호          |         | `symbol` 옵션의 값을 반영합니다. |
| style*   |         | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[elixir]
symbol = '🔮 '
```

## Elm

`elm` 모듈은 현재 설치된 [Elm](https://elm-lang.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `elm.json` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `elm-package.json` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.elm-version` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `elm-stuff` 폴더가 포함되어 있습니다.
- 현재 디렉토리에 `*.elm` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                                            | 설명                                                               |
| ------------------- | -------------------------------------------------- | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`               | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                                        | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'🌳 '`                                             | Elm의 기호를 나타내는 형식 문자열입니다.                           |
| `detect_extensions` | `['elm']`                                          | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['elm.json', 'elm-package.json', '.elm-version']` | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `['elm-stuff']`                                    | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `style`             | `'cyan bold'`                                      | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                            | `elm` 모듈을 비활성화합니다.                                                |

### 변수

| 변수      | 예시        | 설명                                 |
| --------- | --------- | ------------------------------------ |
| version   | `v0.19.1` | `elm`의 버전                |
| 기호        |           | `symbol` 옵션의 값을 반영합니다. |
| style* |           | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[elm]
format = 'via [ $version](cyan bold) '
```

## 환경 변수

`env_var` 모듈은 선택한 환경 변수의 현재 값을 표시합니다. 이 모듈은 다음 조건 중 하나라도 충족될 때만 표시됩니다:

- `variable` 설정 옵션이 기존 환경 변수와 일치합니다.
- `variable` 설정 옵션이 정의되지 않았지만 `default` 설정 옵션이 정의되어 있습니다.

::: tip

`env_var` 모듈이 표시되는 순서는 최상위 `format`에 `${env_var.foo}`를 포함하여 개별적으로 설정할 수 있습니다 (점(.)이 포함되어 있으므로 `${...}`를 사용해야 합니다). 기본적으로 `env_var` 모듈은 정의된 순서대로 모든 `env_var` 모듈을 표시합니다.

:::

::: tip

`.`을 사용하여 여러 환경 변수를 표시할 수 있습니다 (예시 참조). `variable` 설정 옵션이 설정되지 않은 경우, 모듈은 `.` 문자 뒤의 텍스트 이름 아래에 변수 값을 표시합니다.

예시: 다음 설정은 USER 환경 변수의 값을 표시합니다.

```toml
# ~/.config/starship.toml

[env_var.USER]
default = 'unknown user'
```

:::

### 옵션

| 옵션        | 기본값                        | 설명                                                                  |
| ------------- | ------------------------------ | ---------------------------------------------------------------------------- |
| `기호`          | `""`                           | 변수 값을 표시하기 전에 사용되는 기호입니다.                        |
| `variable`    |                                | 표시할 환경 변수입니다.                                    |
| `default`     |                                | 선택한 변수가 정의되지 않았을 때 표시할 기본값입니다. |
| `format`      | `"with [$env_value]($style) "` | 모듈의 형식입니다.                                                   |
| `description` | `'<custom module>'`       | `starship explain`을 실행할 때 표시되는 모듈 설명입니다. |
| `disabled`    | `false`                        | `env_var` 모듈을 비활성화합니다.                                               |

### 변수

| 변수      | 예시                                          | 설명                                 |
| --------- | ------------------------------------------- | ------------------------------------------ |
| env_value | `Windows NT` (if _variable_ would be `$OS`) | `variable` 옵션의 환경 값 |
| 기호        |                                             | `symbol` 옵션의 값을 반영합니다.       |
| style* | `black bold dimmed`                         | `style` 옵션의 값을 반영합니다.        |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[env_var]
variable = 'SHELL'
default = 'unknown shell'
```

여러 환경 변수 표시:

```toml
# ~/.config/starship.toml

[env_var.SHELL]
variable = 'SHELL'
default = 'unknown shell'
[env_var.USER]
default = 'unknown user'
```

## Erlang

`erlang` 모듈은 현재 설치된 [Erlang/OTP](https://erlang.org/doc/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `rebar.config` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `erlang.mk` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `' '`                               | erlang 버전을 표시하기 전에 사용되는 기호입니다.                  |
| `style`             | `'bold red'`                         | 모듈의 스타일입니다.                                                 |
| `detect_extensions` | `[]`                                 | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['rebar.config', 'elang.mk']`       | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `disabled`          | `false`                              | `erlang` 모듈을 비활성화합니다.                                             |

### 변수

| 변수      | 예시        | 설명                                 |
| --------- | --------- | ------------------------------------ |
| version   | `v22.1.3` | `erlang`의 버전              |
| 기호        |           | `symbol` 옵션의 값을 반영합니다. |
| style* |           | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[erlang]
format = 'via [e $version](bold red) '
```

## Fennel

`fennel` 모듈은 현재 설치된 [Fennel](https://fennel-lang.org) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `.fnl` 확장자를 가진 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'🧅 '`                               | fennel 버전을 표시하기 전에 사용되는 기호입니다.                  |
| `style`             | `'bold green'`                       | 모듈의 스타일입니다.                                                 |
| `detect_extensions` | `['fnl']`                            | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `[]`                                 | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `disabled`          | `false`                              | `fennel` 모듈을 비활성화합니다.                                             |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v1.2.1` | `fennel`의 버전              |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[fennel]
symbol = '⫰ '
```

## 채우기

`fill` 모듈은 줄의 추가 공간을 기호로 채웁니다. 한 줄에 여러 `fill` 모듈이 있는 경우 공간을 균등하게 나눕니다. 이는 다른 모듈을 정렬하는 데 유용합니다.

### 옵션

| 옵션     | 기본값        | 설명                               |
| ---------- | -------------- | --------------------------------- |
| `기호`       | `'.'`          | 줄을 채우는 데 사용되는 기호입니다. |
| `style`    | `'bold black'` | 모듈의 스타일입니다.         |
| `disabled` | `false`        | `fill` 모듈을 비활성화합니다.        |

### 예시

```toml
# ~/.config/starship.toml
format = 'AA $fill BB $fill CC'

[fill]
symbol = '-'
style = 'bold green'
```

다음과 같은 프롬프트를 생성합니다:

```
AA -------------------------------------------- BB -------------------------------------------- CC
```

## Fossil 브랜치

`fossil_branch` 모듈은 현재 디렉토리의 체크아웃에 있는 활성 브랜치 이름을 표시합니다.

### 옵션

| 옵션               | 기본값                          | 설명                                                                              |
| ------------------- | -------------------------------- | ---------------------------------------------------------------------------------------- |
| `format`            | `'on [$symbol$branch]($style) '` | 모듈의 형식입니다. 현재 브랜치 이름을 참조하려면 `'$branch'`를 사용하세요.          |
| `기호`                | `' '`                           | 현재 디렉토리의 체크아웃에 있는 브랜치 이름 앞에 사용되는 기호입니다.       |
| `style`             | `'bold purple'`                  | 모듈의 스타일입니다.                                                                |
| `truncation_length` | `2^63 - 1`                       | Fossil 브랜치 이름을 `N` 그래프로 자릅니다.                                          |
| `truncation_symbol` | `'…'`                            | 브랜치 이름이 잘렸음을 나타내는 데 사용되는 기호입니다. 기호가 없는 경우 `''`를 사용할 수 있습니다. |
| `disabled`          | `true`                           | `fossil_branch` 모듈을 비활성화합니다.                                                     |

### 변수

| 변수      | 예시      | 설명                                 |
| --------- | ------- | ------------------------------------ |
| branch    | `trunk` | 활성 Fossil 브랜치             |
| 기호        |         | `symbol` 옵션의 값을 반영합니다. |
| style* |         | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[fossil_branch]
symbol = '🦎 '
truncation_length = 4
truncation_symbol = ''
```

## Fossil 메트릭

`fossil_metrics` 모듈은 현재 디렉토리의 체크아웃에 추가 및 삭제된 줄 수를 표시합니다. Fossil v2.14 (2021-01-20) 이상이 필요합니다.

### 옵션

| 옵션               | 기본값                                                      | 설명                                 |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | 모듈의 형식입니다.            |
| `added_style`        | `'bold green'`                                               | 추가된 수의 스타일입니다.        |
| `deleted_style`      | `'bold red'`                                                 | 삭제된 수의 스타일입니다.      |
| `only_nonzero_diffs` | `true`                                                       | 변경된 항목에 대해서만 상태를 렌더링합니다. |
| `disabled`           | `true`                                                       | `fossil_metrics` 모듈을 비활성화합니다. |

### 변수

| 변수          | 예시  | 설명                                 |
| ----------------- | --- | ------------------------------------------- |
| added             | `1` | 현재 추가된 줄 수           |
| deleted           | `2` | 현재 삭제된 줄 수         |
| added_style*   |     | `added_style` 옵션의 값을 반영합니다.   |
| deleted_style* |     | `deleted_style` 옵션의 값을 반영합니다. |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[fossil_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Google Cloud (`gcloud`)

`gcloud` 모듈은 `gcloud` CLI의 현재 구성을 표시합니다. 이는 `~/.config/gcloud/active_config` 파일과 `~/.config/gcloud/configurations/config_{CONFIG NAME}` 파일 및 `CLOUDSDK_CONFIG` 환경 변수를 기반으로 합니다.

모듈이 활성화되면 항상 활성화됩니다. 단, `detect_env_vars`가 설정된 경우에는 환경 변수 중 하나가 설정되었을 때만 모듈이 활성화됩니다.

### 옵션

| 옵션            | 기본값                                                    | 설명                                                      |
| ----------------- | ---------------------------------------------------------- | ---------------------------------------------------------------- |
| `format`          | `'on [$symbol$account(@$domain)(\'($region\)))]($style) '` | 모듈의 형식입니다.                                       |
| `기호`              | `'☁️  '`                                                   | 현재 GCP 프로필을 표시하기 전에 사용되는 기호입니다.       |
| `region_aliases`  | `{}`                                                       | GCP 이름 외에 표시할 리전 별칭 테이블입니다.  |
| `project_aliases` | `{}`                                                       | GCP 이름 외에 표시할 프로젝트 별칭 테이블입니다. |
| `detect_env_vars` | `[]`                                                       | 이 모듈을 트리거해야 하는 환경 변수입니다.         |
| `style`           | `'bold blue'`                                              | 모듈의 스타일입니다.                                       |
| `disabled`        | `false`                                                    | `gcloud` 모듈을 비활성화합니다.                                    |

### 변수

| 변수      | 예시            | 설명                                                        |
| --------- | ------------- | ------------------------------------------------------------------ |
| region    | `us-central1` | 현재 GCP 리전                                             |
| account   | `foo`         | 현재 GCP 프로필                                            |
| domain    | `example.com` | 현재 GCP 프로필 도메인                                     |
| project   |               | 현재 GCP 프로젝트                                            |
| active    | `default`     | `~/.config/gcloud/active_config`에 기록된 활성 구성 이름 |
| 기호        |               | `symbol` 옵션의 값을 반영합니다.                               |
| style* |               | `style` 옵션의 값을 반영합니다.                                |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

#### 계정 및 프로젝트 표시

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\'($project\)))]($style) '
```

#### 활성 구성 이름만 표시

```toml
# ~/.config/starship.toml

[gcloud]
format = '[$symbol$active]($style) '
style = 'bold yellow'
```

#### 계정 및 별칭 리전 표시

```toml
# ~/.config/starship.toml

[gcloud]
symbol = '️🇬️ '
[gcloud.region_aliases]
us-central1 = 'uc1'
asia-northeast1 = 'an1'
```

#### 계정 및 별칭 프로젝트 표시

```toml
# ~/.config/starship.toml

[gcloud]
format = 'on [$symbol$account(@$domain)(\'($project\)))]($style) '
[gcloud.project_aliases]
very-long-project-name = 'vlpn'
```

## Git 브랜치

`git_branch` 모듈은 현재 디렉토리의 리포지토리에 있는 활성 브랜치를 표시합니다.

### 옵션

| 옵션               | 기본값                                           | 설명                                                                              |
| -------------------- | ------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| `always_show_remote` | `false`                                           | 로컬 브랜치 이름과 같더라도 원격 추적 브랜치 이름을 표시합니다.     |
| `format`             | `'on [$symbol$branch(:$remote_branch)]($style) '` | 모듈의 형식입니다. 현재 브랜치 이름을 참조하려면 `'$branch'`를 사용하세요.          |
| `기호`                 | `' '`                                            | git 브랜치의 기호를 나타내는 형식 문자열입니다.                                   |
| `style`              | `'bold purple'`                                   | 모듈의 스타일입니다.                                                                |
| `truncation_length`  | `2^63 - 1`                                        | git 브랜치를 `N` 그래프로 자릅니다.                                          |
| `truncation_symbol`  | `'…'`                                             | 브랜치 이름이 잘렸음을 나타내는 데 사용되는 기호입니다. 기호가 없는 경우 `''`를 사용할 수 있습니다. |
| `only_attached`      | `false`                                           | 분리된 `HEAD` 상태가 아닐 때만 브랜치 이름을 표시합니다.                           |
| `ignore_branches`    | `[]`                                              | 표시하지 않을 이름 목록입니다. 'master' 또는 'main'에 유용합니다.                      |
| `disabled`           | `false`                                           | `git_branch` 모듈을 비활성화합니다.                                                        |

### 변수

| 변수          | 예시       | 설명                                                                                            |
| ------------- | -------- | ------------------------------------------------------------------------------------------------------ |
| branch        | `master` | 현재 브랜치 이름이며, 현재 브랜치가 없는 경우 (`git detached HEAD` 등) `HEAD`로 대체됩니다. |
| remote_name   | `origin` | 원격 이름입니다.                                                                                       |
| remote_branch | `master` | `remote_name`에서 추적되는 브랜치 이름입니다.                                                       |
| 기호            |          | `symbol` 옵션의 값을 반영합니다.                                                                   |
| style*     |          | `style` 옵션의 값을 반영합니다.                                                                    |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[git_branch]
symbol = '🌱 '
truncation_length = 4
truncation_symbol = ''
ignore_branches = ['master', 'main']
```

## Git 커밋

`git_commit` 모듈은 현재 디렉토리의 리포지토리에 있는 현재 커밋 해시와 태그(있는 경우)를 표시합니다.

### 옵션

| 옵션               | 기본값                        | 설명                                                                          |
| -------------------- | ------------------------------ | ------------------------------------------------------------------------------------ |
| `commit_hash_length` | `7`                            | 표시되는 git 커밋 해시의 길이입니다.                                         |
| `format`             | `'[\]($hash$tag\]($style) '` | 모듈의 형식입니다.                                                           |
| `style`              | `'bold green'`                 | 모듈의 스타일입니다.                                                            |
| `only_detached`      | `true`                         | 분리된 `HEAD` 상태일 때만 git 커밋 해시를 표시합니다.                              |
| `tag_disabled`       | `true`                         | `git_commit` 모듈에서 태그 정보 표시를 비활성화합니다.                                    |
| `tag_max_candidates` | `0`                            | 태그 표시에 고려할 커밋 수입니다. 기본값은 정확히 일치하는 항목만 허용합니다. |
| `tag_symbol`         | `' 🏷  '`                       | 표시되는 정보 앞에 붙는 태그 기호입니다.                                                  |
| `disabled`           | `false`                        | `git_commit` 모듈을 비활성화합니다.                                                    |

### 변수

| 변수      | 예시        | 설명                                 |
| --------- | --------- | -------------------------------------------- |
| hash      | `b703eb3` | 현재 git 커밋 해시                  |
| tag       | `v1.0.0`  | 태그 정보 표시가 활성화된 경우 태그 이름입니다. |
| style* |           | `style` 옵션의 값을 반영합니다.          |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[git_commit]
commit_hash_length = 4
tag_symbol = '🔖 '
```

## Git 상태

`git_state` 모듈은 git 리포지토리의 일부인 디렉토리에서 _REBASING_, _BISECTING_ 등과 같은 작업이 진행 중일 때 표시됩니다. 진행 정보(예: REBASING 3/10)가 있으면 해당 정보도 표시됩니다.

### 옵션

| 옵션         | 기본값                                                         | 설명                                                                             |
| -------------- | --------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| `rebase`       | `'REBASING'`                                                    | `rebase`가 진행 중일 때 표시되는 형식 문자열입니다.                               |
| `merge`        | `'MERGING'`                                                     | `merge`가 진행 중일 때 표시되는 형식 문자열입니다.                                |
| `revert`       | `'REVERTING'`                                                   | `revert`가 진행 중일 때 표시되는 형식 문자열입니다.                               |
| `cherry_pick`  | `'CHERRY-PICKING'`                                              | `cherry-pick`이 진행 중일 때 표시되는 형식 문자열입니다.                          |
| `bisect`       | `'BISECTING'`                                                   | `bisect`가 진행 중일 때 표시되는 형식 문자열입니다.                               |
| `am`           | `'AM'`                                                          | `apply-mailbox` (`git am`)가 진행 중일 때 표시되는 형식 문자열입니다.            |
| `am_or_rebase` | `'AM/REBASE'`                                                   | 모호한 `apply-mailbox` 또는 `rebase`가 진행 중일 때 표시되는 형식 문자열입니다. |
| `style`        | `'bold yellow'`                                                 | 모듈의 스타일입니다.                                                               |
| `format`       | `'\[$state( $progress_current/$progress_total)]($style)\) '` | 모듈의 형식입니다.                                                              |
| `disabled`     | `false`                                                         | `git_state` 모듈을 비활성화합니다.                                                        |

### 변수

| 변수             | 예시         | 설명                                 |
| ---------------- | ---------- | ----------------------------------- |
| state            | `REBASING` | 리포지토리의 현재 상태       |
| progress_current | `1`        | 현재 작업 진행 상황      |
| progress_total   | `2`        | 총 작업 진행 상황        |
| style*        |            | `style` 옵션의 값을 반영합니다. |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[git_state]
format = '[\]($state( $progress_current of $progress_total)]($style)\) '
cherry_pick = '[🍒 PICKING](bold red)'
```

## Git 메트릭

`git_metrics` 모듈은 현재 git 리포지토리에 추가 및 삭제된 줄 수를 표시합니다.

::: tip

이 모듈은 기본적으로 비활성화되어 있습니다. 활성화하려면 설정 파일에서 `disabled`를 `false`로 설정하세요.

:::

### 옵션

| 옵션               | 기본값                                                      | 설명                                 |
| -------------------- | ------------------------------------------------------------ | ------------------------------------- |
| `added_style`        | `'bold green'`                                               | 추가된 수의 스타일입니다.        |
| `deleted_style`      | `'bold red'`                                                 | 삭제된 수의 스타일입니다.      |
| `only_nonzero_diffs` | `true`                                                       | 변경된 항목에 대해서만 상태를 렌더링합니다. |
| `format`             | `'([+$added]($added_style) )([-$deleted]($deleted_style) )'` | 모듈의 형식입니다.            |
| `disabled`           | `true`                                                       | `git_metrics` 모듈을 비활성화합니다.    |
| `ignore_submodules`  | `false`                                                      | 서브모듈 변경 사항을 무시합니다.          |

### 변수

| 변수          | 예시  | 설명                                 |
| ---------------- | --- | ------------------------------------------- |
| added             | `1` | 현재 추가된 줄 수           |
| deleted           | `2` | 현재 삭제된 줄 수         |
| added_style*   |     | `added_style` 옵션의 값을 반영합니다.   |
| deleted_style* |     | `deleted_style` 옵션의 값을 반영합니다. |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[git_metrics]
added_style = 'bold blue'
format = '[+$added]($added_style)/[-$deleted]($deleted_style) '
```

## Git 상태

`git_status` 모듈은 현재 디렉토리의 리포지토리 상태를 나타내는 기호를 표시합니다.

::: tip

WSL 환경에서 Windows 디렉토리(예: `/mnt/c/` 아래)에서는 Git Status 모듈이 매우 느립니다. 모듈을 비활성화하거나 `windows_starship` 옵션을 사용하여 Windows 네이티브 Starship 실행 파일을 사용하여 해당 경로에 대한 `git_status`를 계산할 수 있습니다.

:::

### 옵션

| 옵션               | 기본값                                         | 설명                                                                                                 |
| -------------------- | ----------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| `format`             | `'([\[$all_status$ahead_behind\]]($style) )'` | `git_status`의 기본 형식입니다.                                                                         |
| `conflicted`         | `'='`                                           | 이 브랜치에 병합 충돌이 있습니다.                                                                            |
| `ahead`              | `'⇡'`                                           | `ahead`의 형식입니다.                                                                                       |
| `behind`             | `'⇣'`                                           | `behind`의 형식입니다.                                                                                      |
| `diverged`           | `'⇕'`                                           | `diverged`의 형식입니다.                                                                                    |
| `up_to_date`         | `''`                                            | `up_to_date`의 형식입니다.                                                                                  |
| `untracked`          | `'?'`                                           | `untracked`의 형식입니다.                                                                                   |
| `stashed`            | `'$'`                                           | `stashed`의 형식입니다.                                                                                     |
| `modified`           | `'!'`                                           | `modified`의 형식입니다.                                                                                    |
| `staged`             | `'+'`                                           | `staged`의 형식입니다.                                                                                      |
| `renamed`            | `'»'`                                           | `renamed`의 형식입니다.                                                                                     |
| `deleted`            | `'✘'`                                           | `deleted`의 형식입니다.                                                                                     |
| `typechanged`        | `""`                                            | `typechanged`의 형식입니다.                                                                                 |
| `style`              | `'bold red'`                                    | 모듈의 스타일입니다.                                                                                   |
| `ignore_submodules`  | `false`                                         | 서브모듈 변경 사항을 무시합니다.                                                                               |
| `disabled`           | `false`                                         | `git_status` 모듈을 비활성화합니다.                                                                           |
| `windows_starship`   |                                                 | WSL의 Windows 경로에서 `git_status`를 렌더링할 때 Windows Starship 실행 파일의 (Linux) 경로를 사용합니다. |
| `use_git_executable` | `false`                                         | 상태 계산에 `gitoxide`를 사용하지 않고 `git` 실행 파일을 사용합니다.                       |

### 변수

`format`에서 다음 변수를 사용할 수 있습니다:

| 변수           | 설명                                                                                                   |
| -------------- | ------------------------------------------------------------------------------------------------------------- |
| `all_status`   | `$conflicted$stashed$deleted$renamed$modified$typechanged$staged$untracked`의 약어                       |
| `ahead_behind` | 리포지토리의 현재 상태에 따라 `diverged`, `ahead`, `behind` 또는 `up_to_date` 형식 문자열을 표시합니다. |
| `conflicted`   | 이 브랜치에 병합 충돌이 있을 때 `conflicted`를 표시합니다.                                                   |
| `untracked`    | 작업 디렉토리에 추적되지 않은 파일이 있을 때 `untracked`를 표시합니다.                                 |
| `stashed`      | 로컬 리포지토리에 스태시가 존재할 때 `stashed`를 표시합니다.                                              |
| `modified`     | 작업 디렉토리에 파일 수정 사항이 있을 때 `modified`를 표시합니다.                               |
| `staged`       | 새 파일이 스테이징 영역에 추가되었을 때 `staged`를 표시합니다.                                         |
| `renamed`      | 이름이 변경된 파일이 스테이징 영역에 추가되었을 때 `renamed`를 표시합니다.                                    |
| `deleted`      | 파일 삭제가 스테이징 영역에 추가되었을 때 `deleted`를 표시합니다.                                 |
| `typechanged`  | 파일 유형이 스테이징 영역에서 변경되었을 때 `typechanged`를 표시합니다.                               |
| style*      | `style` 옵션의 값을 반영합니다.                                                                           |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

`diverged`에서 다음 변수를 사용할 수 있습니다:

| 변수       | 설명                                    |
| -------------- | ---------------------------------------------- |
| `ahead_count`  | 추적 브랜치보다 앞선 커밋 수 |
| `behind_count` | 추적 브랜치보다 뒤처진 커밋 수   |

`conflicted`, `ahead`, `behind`, `untracked`, `stashed`, `modified`, `staged`, `renamed` 및 `deleted`에서 다음 변수를 사용할 수 있습니다:

| 변수   | 설명              |
| -------- | ------------------------ |
| `count`  | 파일 수를 표시합니다. |

### 예시

```toml
# ~/.config/starship.toml

[git_status]
conflicted = '🏳'
ahead = '🏎💨'
behind = '😰'
diverged = '😵'
up_to_date = '✓'
untracked = '🤷'
stashed = '📦'
modified = '📝'
staged = '[++\($count\)](green)'
renamed = '👅'
deleted = '🗑'
```

추적 중인 브랜치의 앞/뒤 커밋 수 표시

```toml
# ~/.config/starship.toml

[git_status]
ahead = '⇡${count}'
diverged = '⇕⇡${ahead_count}⇣${behind_count}'
behind = '⇣${count}'
```

WSL의 Windows 경로에서 `git_status`를 렌더링할 때 Windows Starship 실행 파일을 사용합니다.

```toml
# ~/.config/starship.toml

[git_status]
windows_starship = '/mnt/c/Users/username/scoop/apps/starship/current/starship.exe'
```

## Gleam

`gleam` 모듈은 현재 설치된 [Gleam](https://gleam.run/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `gleam.toml` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.gleam` 확장자를 가진 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'⭐ '`                               | Gleam의 기호를 나타내는 형식 문자열입니다.                         |
| `detect_extensions` | `['gleam']`                          | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['gleam.toml']`                     | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `style`             | `'bold #FFAFF3'`                     | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                              | `gleam` 모듈을 비활성화합니다.                                              |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v1.0.0` | `gleam`의 버전               |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[gleam]
format = 'via [⭐ $version](bold red) '
```

## Go

`golang` 모듈은 현재 설치된 [Go](https://golang.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `go.mod` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `go.sum` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `go.work` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `glide.yaml` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `Gopkg.yml` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `Gopkg.lock` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.go-version` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `Godeps` 디렉토리가 포함되어 있습니다.
- 현재 디렉토리에 `.go` 확장자를 가진 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                                                                                   | 설명                                                                                                |
| ------------------- | ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`                                                      | 모듈의 형식입니다.                                                                                 |
| `version_format`    | `'v${raw}'`                                                                               | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다.                                  |
| `기호`                | `'🐹 '`                                                                                    | Go의 기호를 나타내는 형식 문자열입니다.                                                             |
| `detect_extensions` | `['go']`                                                                                  | 이 모듈을 트리거해야 하는 확장자입니다.                                                               |
| `detect_files`      | `['go.mod', 'go.sum', 'go.work', 'glide.yaml', 'Gopkg.yml', 'Gopkg.lock', '.go-version']` | 이 모듈을 트리거해야 하는 파일 이름입니다.                                                                |
| `detect_folders`    | `['Godeps']`                                                                              | 이 모듈을 트리거해야 하는 폴더입니다.                                                                  |
| `style`             | `'bold cyan'`                                                                             | 모듈의 스타일입니다.                                                                                  |
| `not_capable_style` | `'bold red'`                                                                              | go.mod 파일의 go 지시문이 설치된 Go 버전과 일치하지 않을 때 모듈의 스타일입니다. |
| `disabled`          | `false`                                                                                   | `golang` 모듈을 비활성화합니다.                                                                              |

### 변수

| 변수    | 예시        | 설명                                                                                                                                                        |
| --------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version     | `v1.12.1` | `go`의 버전                                                                                                                         |
| mod_version | `1.16`    | `go.mod`의 go 지시문에 설정된 `go` 버전 요구 사항입니다. `go` 버전과 일치하지 않는 경우에만 표시됩니다. |
| 기호          |           | `symbol` 옵션의 값을 반영합니다.                                                                                                        |
| style*   |           | `style` 옵션의 값을 반영합니다.                                                                                                         |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### `mod_version` 사용

```toml
# ~/.config/starship.toml

[golang]
format = 'via [$symbol($version )]($mod_version )]($style)'
```

## Guix-shell

`guix_shell` 모듈은 [guix-shell](https://guix.gnu.org/manual/devel/en/html_node/Invoking-guix-shell.html) 환경을 표시합니다. guix-shell 환경 내부에 있을 때 모듈이 표시됩니다.

### 옵션

| 옵션     | 기본값                    | 설명                                            |
| ---------- | -------------------------- | ------------------------------------------------------ |
| `format`   | `'via [$symbol]($style) '` | 모듈의 형식입니다.                             |
| `기호`       | `'🐃 '`                     | guix-shell의 기호를 나타내는 형식 문자열입니다. |
| `style`    | `'yellow bold'`            | 모듈의 스타일입니다.                              |
| `disabled` | `false`                    | `guix_shell` 모듈을 비활성화합니다.                      |

### 변수

| 변수      | 예시 | 설명                                 |
| --------- | -- | ------------------------------------ |
| 기호        |    | `symbol` 옵션의 값을 반영합니다. |
| style* |    | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[guix_shell]
disabled = true
format = 'via [🐂](yellow bold) '
```

## Gradle

`gradle` 모듈은 프로젝트 디렉토리에서 현재 사용 중인 [Gradle Wrapper](https://docs.gradle.org/current/userguide/gradle_wrapper.html) 버전을 표시합니다.

기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `gradle/wrapper/gradle-wrapper.properties` 디렉토리가 포함되어 있습니다.
- 현재 디렉토리에 `.gradle` 또는 `.gradle.kts`로 끝나는 파일이 포함되어 있습니다.

`gradle` 모듈은 보안상의 이유로 Wrapper를 실행하지 않고 설정 파일에서 Gradle Wrapper 버전을 읽을 수만 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'🅶 '`                               | Gradle의 기호를 나타내는 형식 문자열입니다.                        |
| `detect_extensions` | `['gradle', 'gradle.kts']`           | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `[]`                                 | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `['gradle']`                         | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `style`             | `'bold bright-cyan'`                 | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                              | `gradle` 모듈을 비활성화합니다.                                             |
| `recursive`         | `false`                              | `gradle` 디렉토리에 대한 재귀적 검색을 활성화합니다.                     |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v7.5.1` | `gradle`의 버전              |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

## Haskell

`haskell` 모듈은 현재 선택된 GHC 버전 및/또는 선택된 Stack 스냅샷을 찾습니다.

기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `stack.yaml` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.hs`, `.cabal` 또는 `.hs-boot` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                         |
| ------------------- | ------------------------------------ | -------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                         |
| `기호`                | `'λ '`                               | Haskell의 기호를 나타내는 형식 문자열입니다. |
| `detect_extensions` | `['hs', 'cabal', 'hs-boot']`         | 이 모듈을 트리거해야 하는 확장자입니다.       |
| `detect_files`      | `['stack.yaml', 'cabal.project']`    | 이 모듈을 트리거해야 하는 파일 이름입니다.        |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.          |
| `style`             | `'bold purple'`                      | 모듈의 스타일입니다.                          |
| `disabled`          | `false`                              | `haskell` 모듈을 비활성화합니다.                     |

### 변수

| 변수       | 예시          | 설명                                                                             |
| ---------- | ----------- | --------------------------------------------------------------------------------------- |
| version        |             | 현재 프로젝트가 Stack 프로젝트인지 여부에 따라 `ghc_version` 또는 `snapshot`입니다. |
| snapshot       | `lts-18.12` | 현재 선택된 Stack 스냅샷                                                       |
| ghc_version | `9.2.1`     | 현재 설치된 GHC 버전                                                         |
| 기호             |             | `symbol` 옵션의 값을 반영합니다.                                                    |
| style*      |             | `style` 옵션의 값을 반영합니다.                                                     |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

## Haxe

`haxe` 모듈은 현재 설치된 [Haxe](https://haxe.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `project.xml`, `Project.xml`, `application.xml`, `haxelib.json`, `hxformat.json` 또는 `.haxerc` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.haxelib` 또는 `haxe_libraries` 디렉토리가 포함되어 있습니다.
- 현재 디렉토리에 `.hx` 또는 `.hxml` 확장자를 가진 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                                                                                         | 설명                                                               |
| ------------------- | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`                                                            | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                                                                                    | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `detect_extensions` | `['hx', 'hxml']`                                                                                | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['project.xml', 'Project.xml', 'application.xml', 'haxelib.json', 'hxformat.json', '.haxerc']` | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `['.haxelib', 'haxe_libraries']`                                                                | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `기호`                | `'⌘ '`                                                                                          | Haxe의 기호를 나타내는 형식 문자열입니다.                          |
| `style`             | `'bold fg:202'`                                                                                 | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                                                                         | `haxe` 모듈을 비활성화합니다.                                               |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v4.2.5` | `haxe`의 버전                |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[haxe]
format = "via [⌘ $version](bold fg:202) "
```

## Helm

`helm` 모듈은 현재 설치된 [Helm](https://helm.sh/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `helmfile.yaml` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `Chart.yaml` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `detect_extensions` | `[]`                                 | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['helmfile.yaml', 'Chart.yaml']`    | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `기호`                | `'⎈ '`                               | Helm의 기호를 나타내는 형식 문자열입니다.                          |
| `style`             | `'bold white'`                       | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                              | `helm` 모듈을 비활성화합니다.                                               |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v3.1.1` | `helm`의 버전                |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[helm]
format = 'via [⎈ $version](bold white) '
```

## 호스트 이름

`hostname` 모듈은 시스템 호스트 이름을 표시합니다.

### 옵션

| 옵션            | 기본값                                | 설명                                                                                                                           |
| ----------------- | -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `ssh_only`        | `true`                                 | SSH 세션에 연결된 경우에만 호스트 이름을 표시합니다.                                                                                  |
| `ssh_symbol`      | `'🌐 '`                                 | SSH 세션에 연결되었을 때를 나타내는 형식 문자열입니다.                                                                |
| `trim_at`         | `'.'`                                  | 호스트 이름이 잘리는 문자열입니다. 첫 번째 일치 후 잘립니다. `'.'`는 첫 번째 점 이후를 중지합니다. `''`는 모든 자르기를 비활성화합니다. |
| `detect_env_vars` | `[]`                                   | 이 모듈을 트리거해야 하는 환경 변수입니다.                                                                             |
| `format`          | `'on [$ssh_symbol$hostname]($style) in '` | 모듈의 형식입니다.                                                                                                            |
| `style`           | `'bold dimmed green'`                  | 모듈의 스타일입니다.                                                                                                             |
| `disabled`        | `false`                                | `hostname` 모듈을 비활성화합니다.                                                                        |
| `aliases`         | `{}`                                   | 시스템 호스트 이름을 다른 이름으로 변환합니다. `trim_at`이 지정된 경우 첫 번째 부분만 일치하고 대체됩니다.            |

### 변수

| 변수   | 예시         | 설명                                           |
| -------- | ---------- | ----------------------------------------------------- |
| hostname   | `computer` | 컴퓨터의 호스트 이름                          |
| style*  |            | `style` 옵션의 값을 반영합니다.                   |
| ssh_symbol | `'🌏 '`     | SSH 세션에 연결되었을 때를 나타내는 기호 |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

#### 항상 호스트 이름 표시

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
format = '[$ssh_symbol](bold blue) on [$hostname](bold red) '
trim_at = '.companyname.com'
disabled = false
```

#### 원격 tmux 세션에서 호스트 이름 숨기기

```toml
# ~/.config/starship.toml

[hostname]
ssh_only = false
detect_env_vars = ['!TMUX', 'SSH_CONNECTION']
disabled = false
```

#### 호스트 이름을 별명으로 바꾸기

```toml
# ~/.config/starship.toml
[hostname]
alias = { "Max's MacBook Pro" = "home" }
```

## Java

`java` 모듈은 현재 설치된 [Java](https://www.oracle.com/java/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `pom.xml`, `build.gradle.kts`, `build.sbt`, `.java-version`, `deps.edn`, `project.clj`, `build.boot` 또는 `.sdkmanrc` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.java`, `.class`, `.gradle`, `.jar`, `.clj` 또는 `.cljc` 확장자를 가진 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                                                                                                               | 설명                                                               |
| ------------------- | --------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `format`            | `'via [${symbol}(${version} )]($style)'`                                                                              | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                                                                                                           | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `detect_extensions` | `['java', 'class', 'gradle', 'jar', 'cljs', 'cljc']`                                                                  | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['pom.xml', 'build.gradle.kts', 'build.sbt', '.java-version', 'deps.edn', 'project.clj', 'build.boot', '.sdkmanrc']` | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                                                                                                  | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `기호`                | `'☕ '`                                                                                                                | Java의 기호를 나타내는 형식 문자열입니다.                           |
| `style`             | `'red dimmed'`                                                                                                        | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                                                                                               | `java` 모듈을 비활성화합니다.                                               |

### 변수

| 변수      | 예시    | 설명                                 |
| --------- | ----- | ------------------------------------ |
| version   | `v14` | `java`의 버전                |
| 기호        |       | `symbol` 옵션의 값을 반영합니다. |
| style* |       | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[java]
symbol = '🌟 '
```

## 작업

`jobs` 모듈은 현재 실행 중인 작업 수를 표시합니다. 이 모듈은 백그라운드 작업이 실행 중인 경우에만 표시됩니다. 모듈은 작업이 2개 이상이거나 `number_threshold` 설정 값보다 많으면 작업 수를 표시합니다. 모듈은 작업이 1개 이상이거나 `symbol_threshold` 설정 값보다 많으면 기호를 표시합니다. 두 값 모두 0으로 설정하면 작업 수가 0개라도 기호와 작업 수를 항상 표시할 수 있습니다.

기본 기능은 다음과 같습니다:

- 작업 0개 -> 아무것도 표시되지 않습니다.
- 작업 1개 -> `symbol`이 표시됩니다.
- 작업 2개 이상 -> `symbol` + `number`가 표시됩니다.

::: warning

이 모듈은 tcsh 및 nu에서는 지원되지 않습니다.

:::

::: warning

`threshold` 옵션은 더 이상 사용되지 않지만 사용하려면 모듈이 작업 수가 1개보다 많거나 `threshold` 설정 값이 존재하는 경우보다 많으면 작업 수를 표시합니다. `threshold`가 0으로 설정되면 모듈은 작업 수가 0개일 때도 표시됩니다.

:::

### 옵션

| 옵션             | 기본값                       | 설명                                                              |
| ------------------ | ----------------------------- | ------------------------------------------------------------------------ |
| `threshold`*       | `1`                           | 표시할 작업 수의 임계값입니다.                                         |
| `symbol_threshold` | `1`                           | 작업 수가 `symbol_threshold` 이상이면 `symbol`을 표시합니다.           |
| `number_threshold` | `2`                           | 작업 수가 `number_threshold` 이상이면 작업 수를 표시합니다.             |
| `format`           | `'[$symbol$number]($style) '` | 모듈의 형식입니다.                                               |
| `기호`               | `'✦'`                         | `symbol` 변수를 나타내는 데 사용되는 문자열입니다.                      |
| `style`            | `'bold blue'`                 | 모듈의 스타일입니다.                                                |
| `disabled`         | `false`                       | `jobs` 모듈을 비활성화합니다.                                              |

*: 이 옵션은 더 이상 사용되지 않습니다. 대신 `number_threshold` 및 `symbol_threshold` 옵션을 사용하세요.

### 변수

| 변수      | 예시  | 설명                                 |
| --------- | --- | ------------------------------------ |
| number    | `1` | 작업 수                   |
| 기호        |     | `symbol` 옵션의 값을 반영합니다. |
| style* |     | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[jobs]
symbol = '+ '
number_threshold = 4
 symbol_threshold = 0
```

## Julia

`julia` 모듈은 현재 설치된 [Julia](https://julialang.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `Project.toml` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `Manifest.toml` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.jl` 확장자를 가진 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'ஃ '`                               | Julia의 기호를 나타내는 형식 문자열입니다.                         |
| `style`             | `'bold purple'`                      | 모듈의 스타일입니다.                                                 |
| `detect_extensions` | `['jl']`                             | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['Project.toml', 'Manifest.toml']`  | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `disabled`          | `false`                              | `julia` 모듈을 비활성화합니다.                                              |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v1.4.0` | `julia`의 버전               |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[julia]
symbol = '∴ '
```

## Kotlin

`kotlin` 모듈은 현재 설치된 [Kotlin](https://kotlinlang.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `.kt` 또는 `.kts` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                                   |
| ------------------- | ------------------------------------ | ----------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                    |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다.     |
| `기호`                | `'🅺 '`                               | Kotlin의 기호를 나타내는 형식 문자열입니다.                            |
| `detect_extensions` | `['kt', 'kts']`                      | 이 모듈을 트리거해야 하는 확장자입니다.                                  |
| `detect_files`      | `[]`                                 | 이 모듈을 트리거해야 하는 파일 이름입니다.                                   |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                    |
| `style`             | `'bold blue'`                        | 모듈의 스타일입니다.                                                     |
| `kotlin_binary`     | `'kotlin'`                           | 버전을 가져올 때 Starship이 실행하는 kotlin 바이너리를 구성합니다. |
| `disabled`          | `false`                              | `kotlin` 모듈을 비활성화합니다.                                                 |

### 변수

| 변수      | 예시        | 설명                                 |
| --------- | --------- | ------------------------------------ |
| version   | `v1.4.21` | `kotlin`의 버전              |
| 기호        |           | `symbol` 옵션의 값을 반영합니다. |
| style* |           | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[kotlin]
symbol = '🅺 '
```

```toml
# ~/.config/starship.toml

[kotlin]
# 설치된 버전 정보를 얻기 위해 Kotlin 컴파일러 바이너리 사용
kotlin_binary = 'kotlinc'
```

## Kubernetes

현재 [Kubernetes 컨텍스트](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#context) 이름과, 설정된 경우 네임스페이스, 사용자 및 클러스터를 kubeconfig 파일에서 표시합니다. 네임스페이스는 kubeconfig 파일에 설정해야 하며, `kubectl config set-context starship-context --namespace astronaut`를 통해 설정할 수 있습니다. 마찬가지로 사용자 및 클러스터는 `kubectl config set-context starship-context --user starship-user` 및 `kubectl config set-context starship-context --cluster starship-cluster`를 통해 설정할 수 있습니다. `$KUBECONFIG` 환경 변수가 설정된 경우 모듈은 해당 변수를 사용하고, 그렇지 않으면 `~/.kube/config`를 사용합니다.

::: tip

이 모듈은 기본적으로 비활성화되어 있습니다. 활성화하려면 설정 파일에서 `disabled`를 `false`로 설정하세요.

모듈이 활성화되면 항상 활성화됩니다. 단, `detect_env_vars`, `detect_extensions`, `detect_files` 또는 `detect_folders`가 설정된 경우에는 해당 조건과 일치하는 디렉토리에서만 모듈이 활성화됩니다.

:::

### 옵션

::: warning

`context_aliases` 및 `user_aliases` 옵션은 더 이상 사용되지 않습니다. 대신 `contexts` 및 해당 `context_alias`, `user_alias` 옵션을 사용하세요.

:::

| 옵션              | 기본값                                              | 설명                                                           |
| ------------------- | --------------------------------------------------- | -------------------------------------------------------------- |
| `기호`                | `'☸ '`                                               | 클러스터 앞에 표시되는 기호를 나타내는 형식 문자열입니다.                         |
| `format`            | `'[$symbol$context( \'($namespace\))]($style) in '` | 모듈의 형식입니다.                                            |
| `style`             | `'cyan bold'`                                       | 모듈의 스타일입니다.                                             |
| `context_aliases`*  | `{}`                                                 | 표시할 컨텍스트 별칭 테이블입니다.                                  |
| `user_aliases`*     | `{}`                                                 | 표시할 사용자 별칭 테이블입니다.                                     |
| `detect_extensions` | `[]`                                                 | 이 모듈을 트리거해야 하는 확장자입니다.                          |
| `detect_files`      | `[]`                                                 | 이 모듈을 트리거해야 하는 파일 이름입니다.                           |
| `detect_folders`    | `[]`                                                 | 이 모듈을 트리거해야 하는 폴더입니다.                            |
| `detect_env_vars`   | `[]`                                                 | 이 모듈을 트리거해야 하는 환경 변수입니다.              |
| `contexts`          | `[]`                                                 | 특정 컨텍스트에 대한 사용자 지정 스타일 및 기호입니다.                  |
| `disabled`          | `true`                                               | `kubernetes` 모듈을 비활성화합니다.                                      |

*: 이 옵션은 더 이상 사용되지 않습니다. 대신 해당 `context_alias` 및 `user_alias` 옵션과 함께 `contexts`를 추가하세요.

특정 환경에 대한 모듈 스타일을 사용자 지정하려면 다음 구성을 `contexts` 목록의 일부로 사용하세요:

| 변수          | 설명                                                                              |
| ----------------- | ------------------------------------------------------------------------ |
| `context_pattern` | **필수** 현재 Kubernetes 컨텍스트 이름을 일치시키는 정규식입니다.                |
| `user_pattern`    | 현재 Kubernetes 사용자 이름을 일치시키는 정규식입니다.                               |
| `context_alias`   | 전체 컨텍스트 이름 대신 표시할 컨텍스트 별칭입니다.                               |
| `user_alias`      | 전체 사용자 이름 대신 표시할 사용자 별칭입니다.                                     |
| `style`           | 이 컨텍스트를 사용할 때 모듈의 스타일입니다. 설정되지 않은 경우 모듈의 스타일을 사용합니다.   |
| `기호`              | 이 컨텍스트를 사용할 때 모듈의 기호입니다. 설정되지 않은 경우 모듈의 기호를 사용합니다. |

모든 `*_pattern` 정규식은 `^<pattern>$`으로 고정되므로 전체 문자열과 일치해야 합니다. `*_pattern` 정규식에는 캡처 그룹이 포함될 수 있으며, 이는 해당 별칭에서 `$name` 및 `$N`을 통해 참조될 수 있습니다 (아래 예시 및 [rust Regex::replace() 문서](https://docs.rs/regex/latest/regex/struct.Regex.html#method.replace) 참조).

### 변수

| 변수      | 예시                   | 설명                                     |
| --------- | -------------------- | ---------------------------------------- |
| context   | `starship-context`   | 현재 kubernetes 컨텍스트 이름      |
| namespace | `starship-namespace` | 설정된 경우 현재 kubernetes 네임스페이스 |
| user      | `starship-user`      | 설정된 경우 현재 kubernetes 사용자      |
| cluster   | `starship-cluster`   | 설정된 경우 현재 kubernetes 클러스터   |
| 기호        |                      | `symbol` 옵션의 값을 반영합니다.     |
| style* |                      | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[kubernetes]
format = 'on [$symbol$context( \'($namespace\)))]($style) in '
disabled = false
contexts = [
  { context_pattern = "dev.local.cluster.k8s", style = "green", symbol = "💔 " },
]
```

`k8s` 파일을 포함하는 디렉토리에서만 모듈 표시

```toml
# ~/.config/starship.toml

[kubernetes]
disabled = false
detect_files = ['k8s']
```

#### Kubernetes 컨텍스트별 구성

`contexts` 구성 옵션은 이름이 정의된 정규식과 일치하는 경우 현재 Kubernetes 컨텍스트 이름(스타일 및 기호)을 사용자 지정하는 데 사용됩니다.

```toml
# ~/.config/starship.toml

[[kubernetes.contexts]]
# "bold red" 스타일 + 기본 기호, Kubernetes 현재 컨텍스트 이름이 "production" *이고* 현재 사용자
# "admin_user"와 같을 때
context_pattern = "production"
user_pattern = "admin_user"
style = "bold red"
context_alias = "prod"
user_alias = "admin"

[[kubernetes.contexts]]
# "green" 스타일 + 다른 기호, Kubernetes 현재 컨텍스트 이름에 openshift가 포함될 때
context_pattern = ".*openshift.*"
style = "green"
symbol = "💔 "
context_alias = "openshift"

[[kubernetes.contexts]]
# 캡처 그룹 사용
# GKE, AWS 및 기타 클라우드 공급자의 컨텍스트에는 일반적으로 리전/영역과 같은 추가 정보가 포함됩니다.
# 다음 항목은 GKE 형식 (`gke_projectname_zone_cluster-name`)과 일치합니다.
# 그리고 모든 일치하는 kube 컨텍스트를 더 읽기 쉬운 형식 (`gke-cluster-name`)으로 이름을 바꿉니다:
context_pattern = "gke_.*_(?P<cluster>[\w-]+)"
context_alias = "gke-$cluster"
```

## 줄 바꿈

`line_break` 모듈은 프롬프트를 두 줄로 분리합니다.

### 옵션

| 옵션     | 기본값 | 설명                                                         |
| ---------- | ------- | ------------------------------------------------------------------ |
| `disabled` | `false` | `line_break` 모듈을 비활성화하여 프롬프트를 한 줄로 만듭니다. |

### 예시

```toml
# ~/.config/starship.toml

[line_break]
disabled = true
```

## 로컬 IP

`localip` 모듈은 기본 네트워크 인터페이스의 IPv4 주소를 표시합니다.

### 옵션

| 옵션     | 기본값                   | 설명                                         |
| ---------- | ------------------------- | ------------------------------------------------------ |
| `ssh_only` | `true`                    | SSH 세션에 연결된 경우에만 IP 주소를 표시합니다. |
| `format`   | `'[$localipv4]($style) '` | 모듈의 형식입니다.                             |
| `style`    | `'bold yellow'`           | 모듈의 스타일입니다.                              |
| `disabled` | `true`                    | `localip` 모듈을 비활성화합니다.                         |

### 변수

| 변수      | 예시           | 설명                         |
| --------- | ------------ | ----------------------------------- |
| localipv4 | 192.168.1.13 | 기본 IPv4 주소를 포함합니다.   |
| style* |              | `style` 옵션의 값을 반영합니다. |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[localip]
ssh_only = false
format = '@[$localipv4](bold red) '
disabled = false
```

## Lua

`lua` 모듈은 현재 설치된 [Lua](http://www.lua.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `.lua-version` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `lua` 디렉토리가 포함되어 있습니다.
- 현재 디렉토리에 `.lua` 확장자를 가진 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                                |
| ------------------- | ------------------------------------ | -------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                 |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다.  |
| `기호`                | `'🌙 '`                               | Lua의 기호를 나타내는 형식 문자열입니다.                            |
| `detect_extensions` | `['lua']`                            | 이 모듈을 트리거해야 하는 확장자입니다.                               |
| `detect_files`      | `['.lua-version']`                   | 이 모듈을 트리거해야 하는 파일 이름입니다.                                |
| `detect_folders`    | `['lua']`                            | 이 모듈을 트리거해야 하는 폴더입니다.                                  |
| `style`             | `'bold blue'`                        | 모듈의 스타일입니다.                                                  |
| `lua_binary`        | `'lua'`                              | 버전을 가져올 때 Starship이 실행하는 lua 바이너리를 구성합니다. |
| `disabled`          | `false`                              | `lua` 모듈을 비활성화합니다.                                                 |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v5.4.0` | `lua`의 버전                 |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

lua]
format = 'via [🌕 $version](bold blue) '
```

## 메모리 사용량

`memory_usage` 모듈은 현재 시스템 메모리 및 스왑 사용량을 표시합니다.

기본적으로 총 시스템 스왑이 0이 아닌 경우 스왑 사용량이 표시됩니다.

::: tip

이 모듈은 기본적으로 비활성화되어 있습니다. 활성화하려면 설정 파일에서 `disabled`를 `false`로 설정하세요.

:::

### 옵션

| 옵션      | 기본값                                         | 설명                                              |
| --------- | ----------------------------------------------- | -------------------------------------------------------- |
| `threshold` | `75`                                            | 이 백분율을 초과하지 않는 한 메모리 사용량을 숨깁니다. |
| `format`    | `'via $symbol [${ram}( \| ${swap})]($style) '` | 모듈의 형식입니다.                               |
| `기호`        | `'🐏'`                                           | 메모리 사용량을 표시하기 전에 사용되는 기호입니다.      |
| `style`     | `'bold dimmed white'`                           | 모듈의 스타일입니다.                                |
| `disabled`  | `true`                                          | `memory_usage` 모듈을 비활성화합니다.                      |

### 변수

| 변수         | 예시            | 설명                                                        |
| -------------- | ------------- | ------------------------------------------------------------------ |
| ram              | `31GiB/65GiB` | 현재 시스템 메모리의 사용량/총량입니다.                  |
| ram_pct          | `48%`         | 현재 시스템 메모리의 백분율입니다.                       |
| swap	*     | `1GiB/4GiB`   | 현재 시스템 스왑 메모리 파일의 스왑 메모리 크기입니다.       |
| swap_pct	* | `77%`         | 현재 시스템 스왑 메모리 파일의 스왑 메모리 백분율입니다. |
| 기호               | `🐏`           | `symbol` 옵션의 값을 반영합니다.                               |
| style*        |               | `style` 옵션의 값을 반영합니다.                                |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다. *	: SWAP 파일 정보는 현재 시스템에서 감지된 경우에만 표시됩니다.

### 예시

```toml
# ~/.config/starship.toml

[memory_usage]
disabled = false
threshold = -1
symbol = ' '
style = 'bold dimmed green'
```

## Meson

`meson` 모듈은 현재 Meson 개발자 환경 상태를 표시합니다.

기본적으로 `$MESON_DEVENV`가 설정된 경우 Meson 프로젝트 이름이 표시됩니다.

### 옵션

| 옵션              | 기본값                         | 설명                                                                               |
| ------------------- | ---------------------------------- | ----------------------------------------------------------------------------------------- |
| `truncation_length` | `2^32 - 1`                         | 프로젝트 이름을 `N` 그래프로 자릅니다.                                                |
| `truncation_symbol` | `'…'`                              | 프로젝트 이름이 잘렸음을 나타내는 데 사용되는 기호입니다. 기호가 없는 경우 `''`를 사용할 수 있습니다. |
| `format`            | `'via [$symbol$project]($style) '` | 모듈의 형식입니다.                                                                |
| `기호`                | `'⬢ '`                             | 프로젝트 이름을 표시하기 전에 사용되는 기호입니다.                                       |
| `style`             | `'blue bold'`                      | 모듈의 스타일입니다.                                                                 |
| `disabled`          | `false`                            | `meson` 모듈을 비활성화합니다.                                                              |

### 변수

| 변수      | 예시         | 설명                                 |
| --------- | ---------- | ------------------------------------ |
| project   | `starship` | 현재 Meson 프로젝트 이름       |
| 기호        | `🐏`        | `symbol` 옵션의 값을 반영합니다. |
| style* |            | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[meson]
disabled = false
truncation_symbol = '--'
symbol = ' '
style = 'bold dimmed green'
```

## Mercurial 브랜치

`hg_branch` 모듈은 현재 디렉토리의 리포지토리에 있는 활성 브랜치 및 토픽을 표시합니다.

### 옵션

| 옵션              | 기본값                                   | 설명                                                                                  |
| ------------------- | ----------------------------------------- | -------------------------------------------------------------------------------------------- |
| `기호`                | `' '`                                    | 현재 디렉토리의 리포지토리에 있는 hg 북마크 또는 브랜치 이름 앞에 사용되는 기호입니다. |
| `style`             | `'bold purple'`                           | 모듈의 스타일입니다.                                                                    |
| `format`            | `'on [$symbol$branch(:$topic)]($style) '` | 모듈의 형식입니다.                                                                   |
| `truncation_length` | `2^63 - 1`                                | hg 브랜치 / 토픽 이름을 `N` 그래프로 자릅니다.                                        |
| `truncation_symbol` | `'…'`                                     | 브랜치 이름이 잘렸음을 나타내는 데 사용되는 기호입니다.                             |
| `disabled`          | `true`                                    | `hg_branch` 모듈을 비활성화합니다.                                                             |

### 변수

| 변수      | 예시        | 설명                                 |
| --------- | --------- | ------------------------------------ |
| branch    | `master`  | 활성 mercurial 브랜치          |
| topic     | `feature` | 활성 mercurial 토픽           |
| 기호        |           | `symbol` 옵션의 값을 반영합니다. |
| style* |           | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[hg_branch]
format = 'on [🌱 $branch](bold purple)'
truncation_length = 4
truncation_symbol = ''
```

## Mise

`mise` 모듈은 `mise doctor`를 실행하여 보고된 현재 mise 상태를 표시합니다.

### 옵션

| 옵션             | 기본값                          | 설명                                      |
| ------------------ | -------------------------------- | ------------------------------------------------ | |
| `기호`               | `'mise '`                        | _mise_ 상태를 표시하기 전에 사용되는 기호입니다. | |
| `style`            | `'bold purple'`                  | 모듈의 스타일입니다.                        |
| `format`           | `'on [$symbol$health]($style) '` | 모듈의 형식입니다.                       |
| `healthy_symbol`   | `healthy`                        | _mise_가 건강할 때 표시되는 메시지입니다.    |
| `unhealthy_symbol` | `unhealthy`                      | _mise_가 건강하지 않을 때 표시되는 메시지입니다.  |
| `disabled`         | `true`                           | `mise` 모듈을 비활성화합니다.                      |

### 변수

| 변수      | 예시        | 설명                                 |
| --------- | --------- | ------------------------------------ |
| health    | `healthy` | _mise_의 상태                 |
| 기호        |           | `symbol` 옵션의 값을 반영합니다. |
| style* |           | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[mise]
health = 'ready'
```

## Mojo

`mojo` 모듈은 현재 설치된 Mojo 프로그래밍 언어 버전을 표시합니다.

### 옵션

| 옵션              | 기본값                               | 설명                                             |
| ------------------- | ------------------------------------ | ------------------------------------------------------ |
| `format`            | `'with [$symbol($version )]($style)'` | 모듈의 형식입니다.                             |
| `기호`                | `'🔥 '`                                | Mojo 버전을 표시하기 전에 사용되는 기호입니다. |
| `style`             | `'bold 208'`                         | 모듈의 스타일입니다.                              |
| `disabled`          | `false`                              | `mojo` 모듈을 비활성화합니다.                            |
| `detect_extensions` | `['mojo', '🔥']`                       | 이 모듈을 트리거해야 하는 확장자입니다.           |
| `detect_files`      | `[]`                                 | 이 모듈을 트리거해야 하는 파일 이름입니다.            |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.             |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `24.4.0` | `mojo`의 버전                |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[mojo]
format = 'via [mojo ($version )]($style)'
```

## NATS

`nats` 모듈은 현재 [NATS](https://nats.io) 컨텍스트의 이름을 표시합니다.

### 옵션

| 옵션     | 기본값                    | 설명                                                  |
| ---------- | -------------------------- | ------------------------------------------------------------ |
| `기호`       | `'✉️ '`                    | NATS 컨텍스트 앞에 사용되는 기호입니다 (기본값은 비어 있음).
| `style`    | `'bold purple'`            | 모듈의 스타일입니다.                                     |
| `format`   | `'[$symbol$name]($style)'` | 모듈의 형식입니다.                                   |
| `disabled` | `false`                    | `nats` 모듈을 비활성화합니다.                                  |

### 변수

| 변수      | 예시          | 설명                                 |
| --------- | ----------- | ------------------------------------ |
| name      | `localhost` | 현재 NATS 컨텍스트의 이름         |
| 기호        |             | `symbol` 옵션의 값을 반영합니다. |
| style* |             | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
[nats]
format = '[$symbol]($style)'
style = 'bold purple'
```

## 네트워크 네임스페이스

`netns` 모듈은 현재 네트워크 네임스페이스를 표시합니다. 이는 `ip netns identify`를 사용하여 네트워크 네임스페이스를 가져오므로 `/var/run/netns`에 마운트된 네트워크 네임스페이스만 감지됩니다.

### 옵션

| 옵션     | 기본값                           | 설명                                                       |
| ---------- | --------------------------------- | ------------------------------------------------------------------ |
| `format`   | `'[$symbol \[$name\]]($style)'` | 모듈의 형식입니다.                                        |
| `기호`       | `'🛜 '`                            | 네트워크 네임스페이스 앞에 사용되는 기호입니다 (기본값은 비어 있음).
| `style`    | `'blue bold dimmed'`              | 모듈의 스타일입니다.                                         |
| `disabled` | `false`                           | `netns` 모듈을 비활성화합니다.                                      |

### 변수

| 변수      | 예시         | 설명                               | |
| --------- | ---------- | ----------------------------------------- |
| name      | `my-netns` | 현재 네트워크 네임스페이스의 이름 |
| 기호        |            | `symbol` 옵션의 값을 반영합니다.      |
| style* |            | `style` 옵션의 값을 반영합니다.       |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[netns]
style = 'bold yellow'
symbol = '🌐 '
```

## Nim

`nim` 모듈은 현재 설치된 [Nim](https://nim-lang.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `nim.cfg` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.nim` 확장자를 가진 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.nims` 확장자를 가진 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.nimble` 확장자를 가진 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식                                                 |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'👑 '`                               | Nim 버전을 표시하기 전에 사용되는 기호입니다.                     |
| `detect_extensions` | `['nim', 'nims', 'nimble']`          | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['nim.cfg']`                        | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `style`             | `'bold yellow'`                      | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                              | `nim` 모듈을 비활성화합니다.                                                |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v1.2.0` | `nimc`의 버전                |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

nim]
style = 'yellow'
symbol = '🎣 '
```

## Nix-shell

`nix_shell` 모듈은 [nix-shell](https://nixos.org/guides/nix-pills/developing-with-nix-shell.html) 환경을 표시합니다. nix-shell 환경 내부에 있을 때 모듈이 표시됩니다.

### 옵션

| 옵션        | 기본값                                        | 설명                                                           |
| ------------- | --------------------------------------------- | --------------------------------------------------------------------- |
| `format`      | `'via [$symbol$state( \'($name\))]($style) '` | 모듈의 형식입니다.                                            |
| `기호`          | `'❄️ '`                                        | nix-shell의 기호를 나타내는 형식 문자열입니다.                 |
| `style`       | `'bold blue'`                                  | 모듈의 스타일입니다.                                             |
| `impure_msg`  | `'impure'`                                     | 셸이 불순할 때 표시되는 형식 문자열입니다.                       |
| `pure_msg`    | `'pure'`                                       | 셸이 순수할 때 표시되는 형식 문자열입니다.                         |
| `unknown_msg` | `''`                                           | 셸이 순수/불순한지 알 수 없을 때 표시되는 형식 문자열입니다. |
| `disabled`    | `false`                                        | `nix_shell` 모듈을 비활성화합니다.                                      |
| `heuristic`   | `false`                                        | 새로운 `nix shell`-스타일 셸을 휴리스틱으로 감지하려고 시도합니다. |

### 변수

| 변수      | 예시      | 설명                                 |
| ----------- | ------- | ------------------------------------ |
| state     | `pure`  | nix-shell의 상태           |
| name      | `lorri` | nix-shell의 이름            |
| 기호        |         | `symbol` 옵션의 값을 반영합니다. |
| style* |         | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[nix_shell]
disabled = true
impure_msg = '[impure shell](bold red)'
pure_msg = '[pure shell](bold green)'
unknown_msg = '[unknown shell](bold yellow)'
format = 'via [☃️ $state( \'($name\))]($style) '
```

## Node.js

`nodejs` 모듈은 현재 설치된 [Node.js](https://nodejs.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `package.json` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.node-version` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.nvmrc` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `node_modules` 디렉토리가 포함되어 있습니다.
- 현재 디렉토리에 `.js`, `.mjs` 또는 `.cjs` 확장자를 가진 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.ts`, `.mts` 또는 `.cts` 확장자를 가진 파일이 포함되어 있습니다.

추가로, 디렉토리에 `bunfig.toml`, `bun.lock` 또는 `bun.lockb` 파일이 포함되어 있으면 기본적으로 모듈이 숨겨져 위의 조건을 재정의합니다.

### 옵션

| 옵션              | 기본값                                       | 설명                                                                                           |
| ------------------- | --------------------------------------------- | -------------------------------------------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'`          | 모듈의 형식입니다.                                                                            |
| `version_format`    | `'v${raw}'`                                   | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다.                             |
| `기호`                | `' '`                                        | Node.js의 기호를 나타내는 형식 문자열입니다.                                                   |
| `detect_extensions` | `['js', 'mjs', 'cjs', 'ts', 'mts', 'cts']`    | 이 모듈을 트리거해야 하는 확장자입니다.                                                          |
| `detect_files`      | `['package.json', '.node-version', '.nvmrc']` | 이 모듈을 트리거해야 하는 파일 이름입니다.                                                           |
| `detect_folders`    | `['node_modules']`                            | 이 모듈을 트리거해야 하는 폴더입니다.                                                             |
| `style`             | `'bold green'`                                | 모듈의 스타일입니다.                                                                             |
| `disabled`          | `false`                                       | `nodejs` 모듈을 비활성화합니다.                                                                         |
| `not_capable_style` | `'bold red'`                                  | package.json의 engines 속성이 Node.js 버전과 일치하지 않을 때 모듈의 스타일입니다. |

### 변수

| 변수        | 예시            | 설명                                                                                                                                               |
| ----------- | ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| version         | `v13.12.0`    | `node`의 버전                                                                                                                                     |
| engines_version | `>=12.0.0` | `package.json`의 engines 속성에 설정된 `node` 버전 요구 사항입니다. `node` 버전과 일치하지 않는 경우에만 표시됩니다. |
| 기호              |               | `symbol` 옵션의 값을 반영합니다.                                                                                       |
| style*       |               | `style` 옵션의 값을 반영합니다.                                                                                                                       |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[nodejs]
format = 'via [🤖 $version](bold green) '
```

## OCaml

`ocaml` 모듈은 현재 설치된 [OCaml](https://ocaml.org/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `.opam` 확장자를 가진 파일 또는 `_opam` 디렉토리가 포함되어 있습니다.
- 현재 디렉토리에 `esy.lock` 디렉토리가 포함되어 있습니다.
- 현재 디렉토리에 `dune` 또는 `dune-project` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `jbuild` 또는 `jbuild-ignore` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.merlin` 파일이 포함되어 있습니다.
- 현재 디렉토리에 `.ml`, `.mli`, `.re` 또는 `.rei` 확장자를 가진 파일이 포함되어 있습니다.

### 옵션

| 옵션                    | 기본값                                                                     | 설명                                                               |
| ------------------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `format`                  | `'via [$symbol($version )(\'($switch_indicator$switch_name\) )]($style)'` | 모듈의 형식 문자열입니다.                                         |
| `version_format`          | `'v${raw}'`                                                                | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                      | `'🐫 '`                                                                     | OCaml 버전을 표시하기 전에 사용되는 기호입니다.                   |
| `global_switch_indicator` | `''`                                                                       | 전역 OPAM 스위치를 나타내는 데 사용되는 형식 문자열입니다.                   |
| `local_switch_indicator`  | `'*'`                                                                      | 로컬 OPAM 스위치를 나타내는 데 사용되는 형식 문자열입니다.                    |
| `detect_extensions`       | `['opam', 'ml', 'mli', 're', 'rei']`                                       | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`            | `['dune', 'dune-project', 'jbuild', 'jbuild-ignore', '.merlin']`           | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`          | `['_opam', 'esy.lock']`                                                    | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `style`                   | `'bold yellow'`                                                            | 모듈의 스타일입니다.                                                 |
| `disabled`                | `false`                                                                    | `ocaml` 모듈을 비활성화합니다.                                              |

### 변수

| 변수         | 예시           | 설명                                                       |
| ---------------- | ------------ | ----------------------------------------------------------------- |
| version          | `v4.10.0`    | `ocaml`의 버전                                            |
| switch_name      | `my-project` | 활성 OPAM 스위치                                            |
| switch_indicator |              | 현재 활성 OPAM 스위치의 `indicator` 값을 반영합니다. |
| 기호               |              | `symbol` 옵션의 값을 반영합니다.                              |
| style*        |              | `style` 옵션의 값을 반영합니다.                               |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[ocaml]
format = 'via [🐪 $version]($style) '
```

## Odin

`odin` 모듈은 현재 설치된 [Odin](https://odin-lang.org/) 버전을 표시합니다. 기본적으로 현재 디렉토리에 `.odin` 파일이 포함되어 있으면 모듈이 표시됩니다.

### 옵션

| 옵션              | 기본값                               | 설명                                           |
| ------------------- | ------------------------------------ | ----------------------------------------------------- |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                            |
| `show_commit`       | `false`                              | 커밋을 버전의 일부로 표시합니다.              |
| `기호`                | `'Ø '`                               | Zig의 버전을 표시하기 전에 사용되는 기호입니다. |
| `style`             | `'bold bright-blue'`                 | 모듈의 스타일입니다.                             |
| `disabled`          | `false`                              | `odin` 모듈을 비활성화합니다.                           |
| `detect_extensions` | `['odin']`                           | 이 모듈을 트리거해야 하는 확장자입니다.          |
| `detect_files`      | `[]`                                 | 이 모듈을 트리거해야 하는 파일 이름입니다.           |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.             |

### 변수

| 변수      | 예시            | 설명                                 |
| --------- | ------------- | ------------------------------------ |
| version   | `dev-2024-03` | `odin`의 버전                |
| 기호        |               | `symbol` 옵션의 값을 반영합니다. |
| style* |               | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[odin]
format = 'via [󰹩 ($version )]($style)'
show_commit = true
```

## Open Policy Agent

`opa` 모듈은 현재 설치된 OPA 도구 버전을 표시합니다. 기본적으로 현재 디렉토리에 `.rego` 파일이 포함되어 있으면 모듈이 표시됩니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'🪖  '`                              | OPA의 기호를 나타내는 형식 문자열입니다.                           |
| `detect_extensions` | `['rego']`                           | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `[]`                                 | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `style`             | `'bold blue'`                        | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                              | 이 모듈을 비활성화합니다.                                                     |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v0.44.0` | `opa`의 버전                 |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[opa]
format = 'via [⛑️  $version](bold red) '
```

## OpenStack

`openstack` 모듈은 현재 OpenStack 클라우드 및 프로젝트를 표시합니다. 이 모듈은 `OS_CLOUD` 환경 변수가 설정된 경우에만 활성화되며, 이 경우 [기본 위치](https://docs.openstack.org/python-openstackclient/latest/configuration/index.html#configuration-files) 중 하나에서 `clouds.yaml` 파일을 읽어 현재 사용 중인 프로젝트를 가져옵니다.

### 옵션

| 옵션     | 기본값                                         | 설명                                                     |
| ---------- | ----------------------------------------------- | -------------------------------------------------------------- |
| `format`   | `'on [$symbol$cloud(\'($project\))]($style) '` | 모듈의 형식입니다.                                     |
| `기호`       | `'☁️ '`                                         | 현재 OpenStack 클라우드를 표시하기 전에 사용되는 기호입니다. |
| `style`    | `'bold yellow'`                                 | 모듈의 스타일입니다.                                      |
| `disabled` | `false`                                         | `openstack` 모듈을 비활성화합니다.                               |

### 변수

| 변수      | 예시     | 설명                                 |
| --------- | ------ | -------------------------------------------------------------- |
| cloud     | `corp` | 현재 OpenStack 클라우드          |
| project   | `dev`  | 현재 OpenStack 프로젝트        |
| 기호        |        | `symbol` 옵션의 값을 반영합니다. |
| style* |        | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[openstack]
format = 'on [$symbol$cloud(\'($project\))]($style) '
style = 'bold yellow'
symbol = '☁️ '
```

## OS

`os` 모듈은 현재 운영 체제를 표시합니다. OS 정보는 [os_info](https://lib.rs/crates/os_info) 크레이트를 통해 감지됩니다.

::: warning

이 모듈에 사용되는 [os_info](https://lib.rs/crates/os_info) 크레이트는 일부 시스템에서 부정확할 수 있습니다.

:::

::: tip

이 모듈은 기본적으로 비활성화되어 있습니다. 활성화하려면 설정 파일에서 `disabled`를 `false`로 설정하세요.

:::

### 옵션

| 옵션     | 기본값               | 설명                                                                                            |
| ---------- | --------------------- | ------------------------------------------------------------------------------------------------------ |
| `format`   | `'[$symbol]($style)'` | 모듈의 형식입니다.                                                            |
| `style`    | `'bold white'`        | 모듈의 스타일입니다.                                                             |
| `disabled` | `true`                | `os` 모듈을 비활성화합니다.                                                             |
| `symbols`  |                       | 각 운영 체제에 해당 기호를 매핑하는 테이블입니다.

`symbols`를 사용하면 각 운영 체제 유형에 대한 임의의 기호를 정의할 수 있습니다. 구성에서 정의되지 않은 운영 체제 유형은 아래 기본 기호 테이블을 사용합니다. 현재 모듈에서 지원되는 모든 운영 체제는 아래에 나열되어 있습니다. 운영 체제를 추가하려면 [기능 요청](https://github.com/starship/starship/issues/new/choose)을 자유롭게 열어주세요.

```toml
# 이것은 기본 기호 테이블입니다.
[os.symbols]
AIX = "➿ "
Alpaquita = "🔔 "
AlmaLinux = "💠 "
Alpine = "🏔️ "
Amazon = "🙂 "
Android = "🤖 "
Arch = "🎗️ "
Artix = "🎗️ "
Bluefin = "🐟 "
CachyOS = "🎗️ "
CentOS = "💠 "
Debian = "🌀 "
DragonFly = "🐉 "
Emscripten = "🔗 "
EndeavourOS = "🚀 "
Fedora = "🎩 "
FreeBSD = "😈 "
Garuda = "🦅 "
Gentoo = "🗜️ "
HardenedBSD = "🛡️ "
Illumos = "🐦 "
Kali = "🐉 "
Linux = "🐧 "
Mabox = "📦 "
Macos = "🍎 "
Manjaro = "🥭 "
Mariner = "🌊 "
MidnightBSD = "🌘 "
Mint = "🌿 "
NetBSD = "🚩 "
NixOS = "❄️ "
Nobara = "🎩 "
OpenBSD = "🐡 "
OpenCloudOS = "☁️ "
openEuler = "🦉 "
openSUSE = "🦎 "
OracleLinux = "🦴 "
Pop = "🍭 "
Raspbian = "🍓 "
Redhat = "🎩 "
RedHatEnterprise = "🎩 "
RockyLinux = "💠 "
Redox = "🧪 "
Solus = "⛵ "
SUSE = "🦎 "
Ubuntu = "🎯 "
Ultramarine = "🔷 "
Unknown = "❓ "
Uos = "🐲 "
Void = "  "
Windows = "🪟 "
```

### 변수

| 변수      | 예시           | 설명                                                        |
| --------- | ------------ | ------------------------------------------------------------------ |
| `기호`        | `🎗️`         | `symbols` 고급 옵션에서 현재 운영 체제 기호 |
| name      | `Arch Linux` | 현재 운영 체제 이름                                  |
| 종류        | `Arch`       | 현재 운영 체제 유형                                  |
| codename  |              | 해당되는 경우 현재 운영 체제 코드 이름               |
| edition   |              | 해당되는 경우 현재 운영 체제 버전                   |
| version   |              | 해당되는 경우 현재 운영 체제 버전                   |
| style* |              | `style` 옵션의 값을 반영합니다.                                |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[os]
format = "on [($name )]($style)"
style = "bold blue"
disabled = false

[os.symbols]
Windows = " "
Arch = "Arch is the best! "
```

## 사용자 이름

`username` 모듈은 활성 사용자의 사용자 이름을 표시합니다. 이 모듈은 다음 조건 중 하나라도 충족되면 표시됩니다:

- 현재 사용자가 루트/관리자입니다.
- 현재 사용자가 로그인한 사용자와 동일하지 않습니다.
- 사용자가 현재 SSH 세션으로 연결되어 있습니다.
- `show_always` 변수가 true로 설정되었습니다.
- `detect_env_vars` 배열에 설정된 환경 변수 이름이 하나 이상 포함되어 있습니다.

::: tip

SSH 연결은 `SSH_CONNECTION`, `SSH_CLIENT`, `SSH_TTY` 환경 변수를 확인하여 감지됩니다. SSH 호스트가 이러한 변수를 설정하지 않는 경우, 변수 중 하나를 더미 값으로 설정하는 것이 한 가지 해결 방법입니다.

:::

### 옵션

| 옵션            | 기본값                 | 설명                                               |
| ----------------- | ----------------------- | --------------------------------------------------------- |
| `style_root`      | `'bold red'`            | 사용자가 루트/관리자일 때 사용되는 스타일입니다.               |
| `style_user`      | `'bold yellow'`         | 루트가 아닌 사용자에 대해 사용되는 스타일입니다.                        |
| `detect_env_vars` | `[]`                    | 이 모듈을 트리거해야 하는 환경 변수입니다. |
| `format`          | `'[$user]($style) in '` | 모듈의 형식입니다.                                | |
| `show_always`     | `false`                 | `username` 모듈을 항상 표시합니다. |
| `disabled`        | `false`                 | `username` 모듈을 비활성화합니다.                           |
| `aliases`         | `{}`                    | 시스템 사용자 이름을 다른 이름으로 변환합니다.             |

### 변수

| 변수 | 예시           | 설명                                                                                 |
| -------- | ------------ | ------------------------------------------------------------------------------------------- |
| `style`  | `'red bold'` | 루트가 로그인된 경우 `style_root`의 값을, 그렇지 않으면 `style_user`의 값을 반영합니다. |
| `user`   | `'matchai'`  | 현재 로그인된 사용자 ID입니다.                                                            |

### 예시

#### 항상 사용자 이름 표시

```toml
# ~/.config/starship.toml

[username]
style_user = 'white bold'
style_root = 'black bold'
format = 'user: [$user]($style) '
disabled = false
show_always = true
alias = { "corpuser034g" = "matchai" }
```

## Vagrant

`vagrant` 모듈은 현재 설치된 [Vagrant](https://www.vagrantup.com/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `Vagrantfile` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'⍱ '`                               | Vagrant의 기호를 나타내는 형식 문자열입니다.                       |
| `detect_extensions` | `[]`                                 | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['Vagrantfile']`                    | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                |
| `style`             | `'cyan bold'`                        | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                              | `vagrant` 모듈을 비활성화합니다.                                              |

### 변수

| 변수      | 예시               | 설명                                 |
| --------- | ---------------- | ------------------------------------ |
| version   | `Vagrant 2.2.10` | `Vagrant`의 버전             |
| 기호        |                  | `symbol` 옵션의 값을 반영합니다. |
| style* |                  | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[vagrant]
format = 'via [⍱ $version](bold white) '
```

## V

`vlang` 모듈은 현재 설치된 [V](https://vlang.io/) 버전을 표시합니다. 기본적으로 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `.v` 확장자를 가진 파일이 포함되어 있습니다.
- 현재 디렉토리에 `v.mod`, `vpkg.json` 또는 `.vpkg-lock.json` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                                      | 설명                                                               |
| ------------------- | -------------------------------------------- | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'`         | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                                  | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'V '`                                       | V의 기호를 나타내는 형식 문자열입니다.                              |
| `detect_extensions` | `['v']`                                      | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `['v.mod', 'vpkg.json', '.vpkg-lock.json' ]` | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                         | 이 모듈을 트리거해야 하는 폴더입니다.                                 |
| `style`             | `'blue bold'`                                | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                                      | `vlang` 모듈을 비활성화합니다.                                              |

### 변수

| 변수      | 예시     | 설명                                 |
| --------- | ------ | ------------------------------------ |
| version   | `v0.2` | `v`의 버전                   |
| 기호        |        | `symbol` 옵션의 값을 반영합니다. |
| style* |        | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml
[vlang]
format = 'via [V $version](blue bold) '
```

## VCSH

`vcsh` 모듈은 현재 활성 [VCSH](https://github.com/RichiH/vcsh) 리포지토리를 표시합니다. 리포지토리가 현재 사용 중인 경우에만 모듈이 표시됩니다.

### 옵션

| 옵션     | 기본값                          | 설명                                             |
| ---------- | -------------------------------- | ------------------------------------------------------ |
| `기호`       | `''`                             | 리포지토리 이름을 표시하기 전에 사용되는 기호입니다. |
| `style`    | `'bold yellow'`                  | 모듈의 스타일입니다.                              |
| `format`   | `'vcsh [$symbol$repo]($style) '` | 모듈의 형식입니다.                             |
| `disabled` | `false`                          | `vcsh` 모듈을 비활성화합니다.                            |

### 변수

| 변수      | 예시                                          | 설명                                 |
| --------- | ------------------------------------------- | ------------------------------------ |
| repo      | `dotfiles` (VCSH 리포 이름이 dotfiles인 경우) | 활성 리포지토리 이름           |
| 기호        |                                             | `symbol` 옵션의 값을 반영합니다. |
| style* | `black bold dimmed`                         | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[vcsh]
format = '[🆅 $repo](bold blue) '
```

## Zig

기본적으로 `zig` 모듈은 현재 설치된 [Zig](https://ziglang.org/) 버전을 표시합니다. 다음 조건 중 하나라도 충족되면 모듈이 표시됩니다:

- 현재 디렉토리에 `.zig` 파일이 포함되어 있습니다.

### 옵션

| 옵션              | 기본값                               | 설명                                                               |
| ------------------- | ------------------------------------ | ------------------------------------------------------------------------ |
| `format`            | `'via [$symbol($version )]($style)'` | 모듈의 형식입니다.                                                |
| `version_format`    | `'v${raw}'`                          | 버전 형식입니다. 사용 가능한 변수는 `raw`, `major`, `minor`, `patch`입니다. |
| `기호`                | `'↯ '`                               | Zig 버전을 표시하기 전에 사용되는 기호입니다.                     |
| `style`             | `'bold yellow'`                      | 모듈의 스타일입니다.                                                 |
| `disabled`          | `false`                              | `zig` 모듈을 비활성화합니다.                                                |
| `detect_extensions` | `['zig']`                            | 이 모듈을 트리거해야 하는 확장자입니다.                              |
| `detect_files`      | `[]`                                 | 이 모듈을 트리거해야 하는 파일 이름입니다.                               |
| `detect_folders`    | `[]`                                 | 이 모듈을 트리거해야 하는 폴더입니다.                                 |

### 변수

| 변수      | 예시       | 설명                                 |
| --------- | -------- | ------------------------------------ |
| version   | `v0.6.0` | `zig`의 버전                 |
| 기호        |          | `symbol` 옵션의 값을 반영합니다. |
| style* |          | `style` 옵션의 값을 반영합니다.  |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

### 예시

```toml
# ~/.config/starship.toml

[zig]
symbol = '⚡️ '
```

## 사용자 지정 명령

`custom` 모듈은 임의의 명령 출력을 표시합니다.

다음 조건 중 하나라도 충족되면 이러한 모듈이 표시됩니다:

- 현재 디렉토리에 `detect_files`에 있는 파일 이름이 포함되어 있습니다.
- 현재 디렉토리에 `detect_folders`에 있는 디렉토리 이름이 포함되어 있습니다.
- 현재 디렉토리에 `detect_extensions`에 있는 확장자를 가진 파일이 포함되어 있습니다.
- `when` 명령이 0을 반환합니다.
- 현재 운영 체제 (`std::env::consts::OS`)가 `os` 필드와 일치하는 경우 (정의된 경우).

::: tip

여러 사용자 지정 모듈을 `.`을 사용하여 정의할 수 있습니다.

:::

::: tip

사용자 지정 모듈이 표시되는 순서는 최상위 `format`에 `${custom.foo}`를 포함하여 개별적으로 설정할 수 있습니다 (점(.)이 포함되어 있으므로 `${...}`를 사용해야 합니다). 기본적으로 `custom` 모듈은 정의된 순서대로 모든 사용자 지정 모듈을 표시합니다.

:::

::: tip

[Issue #1252](https://github.com/starship/starship/discussions/1252)에는 사용자 지정 모듈의 예가 포함되어 있습니다. 흥미로운 예가 있다면 거기에 공유해 주세요!

:::

::: warning `unsafe_no_escape`가 활성화되었거나 starship v1.20 이전 버전에서는 명령 출력이 이스케이프되지 않고 프롬프트에 표시됩니다.

명령이 생성하는 모든 출력은 수정되지 않고 프롬프트에 표시됩니다. 즉, 출력에 셸별 해석 가능한 시퀀스가 포함된 경우 해석될 수 있습니다. 셸에 따라 예를 들어 백틱으로 묶인 문자열이 셸에서 실행될 수 있습니다. 이러한 시퀀스는 일반적으로 셸별입니다. 예를 들어, bash 시퀀스(`
`)를 작성하는 명령 모듈을 작성할 수 있지만 이 모듈은 fish 또는 zsh 셸에서는 작동하지 않습니다.

형식 문자열에는 셸별 프롬프트 시퀀스도 포함될 수 있습니다. 예를 들어 [Bash](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html), [Zsh](https://zsh.sourceforge.io/Doc/Release/Prompt-Expansion.html)입니다.

:::

### 옵션

| 옵션              | 기본값                         | 설명                                                                                                                                                                                                 |
| ------------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `command`           | `''`                            | 출력해야 하는 명령입니다. 명령은 셸의 표준 입력으로 전달됩니다.                                                                                                                                   |
| `when`              | `false`                         | 부울 값 (`true` 또는 `false`, 따옴표 없음) 또는 조건으로 사용되는 문자열 셸 명령입니다. 문자열인 경우 `shell`이 실행 시 `0` 상태 코드를 반환하면 모듈이 표시됩니다.                                                        |
| `require_repo`      | `false`                         | `true`인 경우 모듈은 (git) 리포지토리가 포함된 경로에서만 표시됩니다. 이 옵션만으로는 표시 조건으로 충분하지 않습니다.
                                                                                                                           |
| `shell`             |                                 | [아래 참조](#custom-command-shell)                                                                                                                                                               |
| `description`       | `'<custom module>'`       | `starship explain`을 실행할 때 표시되는 모듈 설명입니다.                                                                                                                                                    |
| `unsafe_no_escape`  | `false`                         | 설정된 경우 명령 출력은 셸에서 해석될 수 있는 문자에 대해 이스케이프되지 않습니다.                                                                                                                                   |
| `detect_files`      | `[]`                            | 일치하는 파일을 검색할 작업 디렉토리의 파일입니다.                                                                                                                                                           |
| `detect_folders`    | `[]`                            | 일치하는 디렉토리를 검색할 작업 디렉토리의 디렉토리입니다.                                                                                                                                                     |
| `detect_extensions` | `[]`                            | 일치하는 확장자를 검색할 작업 디렉토리의 확장자입니다.                                                                                                                                                      |
| `기호`                | `''`                            | 명령 출력을 표시하기 전에 사용되는 기호입니다.                                                                                                                                                                |
| `style`             | `'bold green'`                  | 모듈의 스타일입니다.                                                                                                                                                                        |
| `format`            | `'[$symbol($output )]($style)'` | 모듈의 형식입니다.                                                                                                                                                                                                 |
| `disabled`          | `false`                         | 이 `custom` 모듈을 비활성화합니다.                                                                                                                                                                   |
| `os`                |                                 | 모듈이 표시될 운영 체제 이름 (unix, linux, macos, windows, ... ) [가능한 값 참조](https://doc.rust-lang.org/std/env/consts/constant.OS.html).                                                                                  |
| `use_stdin`         |                                 | 명령에 표준 입력을 통해 전달할지 또는 인수로 전달할지 여부를 재정의하는 선택적 부울 값입니다. 설정되지 않은 경우 표준 입력이 기본적으로 사용되지만 셸이 지원하지 않는 경우 (cmd, nushell)에는 그렇지 않습니다. 이를 설정하면 셸별 인수 처리가 비활성화됩니다. |
| `ignore_timeout`    | `false`                         | 전역 `command_timeout` 설정을 무시하고 외부 명령이 아무리 오래 걸리더라도 계속 실행합니다.                                                                                                                                                      |

### 변수

| 변수      | 설명                                     |
| --------- | -------------------------------------- |
| output    | `shell`에서 실행된 `command`의 출력 |
| 기호        | `symbol` 옵션의 값을 반영합니다.   |
| style* | `style` 옵션의 값을 반영합니다.    |


*: 이 변수는 스타일 문자열의 일부로만 사용할 수 있습니다.

#### 사용자 지정 명령 셸

`shell`은 문자열 목록을 허용하며, 여기서:

- 첫 번째 문자열은 명령을 실행하는 데 사용할 셸의 경로입니다.
- 후속 인수는 셸에 전달됩니다.

설정되지 않은 경우 STARSHIP_SHELL을 거쳐 Linux에서는 'sh', Windows에서는 'cmd /C'로 대체됩니다.

`command` (및 해당되는 경우 `when`)는 표준 입력으로 전달됩니다.

`shell`이 제공되지 않거나 하나의 요소만 포함하고 Starship이 PowerShell을 사용할 것으로 감지하면 다음 인수가 자동으로 추가됩니다: `-NoProfile -Command -`. `shell`이 제공되지 않거나 하나의 요소만 포함하고 Starship이 Cmd를 사용할 것으로 감지하면 다음 인수가 자동으로 추가됩니다: `/C` 및 `stdin`은 `false`로 설정됩니다. `shell`이 제공되지 않거나 하나의 요소만 포함하고 Starship이 Nushell을 사용할 것으로 감지하면 다음 인수가 자동으로 추가됩니다: `-c` 및 `stdin`은 `false`로 설정됩니다. 이 동작은 셸에 인수를 명시적으로 전달하여 피할 수 있습니다. 예:

```toml
shell = ['pwsh', '-Command', '-']
```

::: warning 사용자 지정 셸 구성을 올바르게 종료하도록 하세요

사용자 지정 명령을 설정하는 경우 starship에서 사용하는 기본 셸이 명령을 `shell` 옵션을 통해 올바르게 실행하고 정상적으로 종료하는지 확인하세요.

예를 들어 PowerShell은 한 줄 명령을 실행하기 위해 `-Command` 매개변수가 필요합니다. 이 매개변수를 생략하면 starship이 재귀 루프에 빠질 수 있으며, 이 경우 셸은 starship 자체를 다시 실행하여 사용자 지정 명령을 다시 실행하려고 할 수 있으며, 이는 끝없는 루프에 빠질 수 있습니다.

PowerShell의 `-NoProfile`과 유사한 매개변수는 다른 셸에서도 추가 로딩 시간을 피하기 위해 권장됩니다. starship 호출마다 사용자 지정 프로필을 로드하는 것을 방지합니다.

셸 자동 감지 및 올바른 매개변수 추가는 현재 구현되어 있지만 모든 셸이 포함되지는 않을 수 있습니다. 이러한 시나리오에 해당하는 경우 셸 세부 정보와 starship 구성을 포함하여 [이슈를 열어주세요](https://github.com/starship/starship/issues/new/choose).

:::

### 예시

```toml
# ~/.config/starship.toml

[custom.foo]
command = 'echo foo' # 명령의 출력을 표시합니다.
detect_files = ['foo'] # 필터를 지정할 수 있지만 와일드카드는 지원되지 않습니다.
when = ''' test "$HOME" = "$PWD" '''
format = ' transcending [$output]($style)'

[custom.time]
command = 'time /T'
detect_extensions = ['pst'] # *.pst 파일을 필터링합니다.
shell = ['pwsh.exe', '-NoProfile', '-Command', '-']

[custom.time-as-arg]
command = 'time /T'
detect_extensions = ['pst'] # *.pst 파일을 필터링합니다.
shell = ['pwsh.exe', '-NoProfile', '-Command']
use_stdin = false
```