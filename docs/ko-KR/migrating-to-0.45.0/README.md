# v0.45.0으로 마이그레이션

Starship v0.45.0은 대규모 v1.0.0을 준비하기 위한 주요 변경 사항이 포함된 릴리스입니다. 더 높은 수준의 사용자 정의를 허용하기 위해 프롬프트에서 구성이 수행되는 방식에 대한 몇 가지 주요 변경 사항을 적용했습니다.

이 가이드는 주요 변경 사항을 안내하기 위한 것입니다.

## `prompt_order`가 최상위 `format`으로 대체되었습니다.

v0.45.0 이전에는 `prompt_order`가 Starship에서 렌더링되어야 하는 순서대로 모듈 이름 배열을 허용했습니다.

Starship v0.45.0은 대신 `format` 값을 허용하여 모듈 자체 외부에서 프롬프트를 사용자 정의할 수 있습니다.

**v0.45.0 이전 구성 예시**

```toml
prompt_order = [
  "username",
  "hostname",
  "directory",
  "git_branch",
  "git_commit",
  "git_state",
  "git_status",
  "cmd_duration",
  "custom",
  "line_break",
  "jobs",
  "battery",
  "time",
  "character",
]
```

**v0.45.0 구성 예시**

```toml
format = """
  $username\
  $hostname\
  $directory\
  $git_branch\
  $git_commit\
  $git_state\
  $git_status\
  $cmd_duration\
  $custom\
  $line_break\
  $jobs\
  $battery\
  $time\
  $character\
  """
```

## 모듈 `prefix` 및 `suffix`가 `format`으로 대체되었습니다.

v0.45.0 이전에는 일부 모듈이 모듈이 렌더링되는 방식을 스타일링하기 위해 `prefix` 및/또는 `suffix`를 허용했습니다.

Starship v0.45.0은 대신 `format` 값을 허용하여 모듈이 렌더링되는 방식을 더욱 사용자 정의할 수 있습니다. 컨텍스트 기반 변수에 대한 접두사 및 접미사를 정의하는 대신, 이제 변수를 모듈의 출력을 나타내는 형식 문자열 내에서 대체할 수 있습니다.

**v0.45.0 이전 구성 예시**

```toml
[cmd_duration]
prefix = "took "
```

**v0.45.0 구성 예시**

```toml
[cmd_duration]
# $duration – 명령 지속 시간 (예: "15s")
# $style    – 모듈의 기본 스타일 (예: "bold yellow")
format = "took [$duration]($style) "
```

### 영향을 받는 모듈

#### 문자

| 제거된 속성                  | 대체               |
| ----------------------- | ---------------- |
| `기호`                    | `success_symbol` |
| `use_symbol_for_status` | `error_symbol`   |
| `style_success`         | `success_symbol` |
| `style_failure`         | `error_symbol`   |

**기본 구성 변경 사항**

```diff
[character]
-- symbol = "❯"
-- error_symbol = "✖"
-- use_symbol_for_status = true
-- vicmd_symbol = "❮"
++ success_symbol = "[❯](bold green)"
++ error_symbol = "[❯](bold red)"
++ vicmd_symbol = "[❮](bold green)"
```

이전에는 `use_symbol_for_status` 속성이 마지막 명령이 0이 아닌 상태 코드를 반환했을 때 `error_symbol`을 표시하도록 프롬프트를 구성하는 데 사용되었습니다.

v0.45.0 릴리스에서는 이제 0이 아닌 상태 코드 후에 항상 `error_symbol`을 사용하여 `use_symbol_for_status` 및 `error_symbol` 속성을 통합합니다.

이전 `use_symbol_for_status = true` 구성을 사용하도록 프롬프트를 구성하려면 구성 파일에 다음을 추가하세요.

```toml
[character]
error_symbol = "[✖](bold red)"
```

_참고:_ `character` 요소는 자동으로 뒤에 공백을 추가하므로 다른 `format` 문자열과 달리 위 예시에서는 특별히 추가하지 않습니다.

#### 명령 지속 시간

| 제거된 속성   | 대체       |
| -------- | -------- |
| `prefix` | `format` |

**기본 구성 변경 사항**

```diff
[cmd_duration]
-- prefix = "took "
++ format = "took [$duration]($style) "
```

#### 디렉토리

| 제거된 속성   | 대체       |
| -------- | -------- |
| `prefix` | `format` |

**기본 구성 변경 사항**

```diff
[directory]
-- prefix = "in "
++ format = "[$path]($style)[$read_only]($read_only_style) "
```

#### 환경 변수

| 제거된 속성   | 대체       |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**기본 구성 변경 사항**

```diff
[env_var]
-- prefix = ""
-- suffix = ""
++ format = "with [$env_value]($style) "
```

#### Git 커밋

| 제거된 속성   | 대체       |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**기본 구성 변경 사항**

```diff
[git_commit]
-- prefix = "("
-- suffix = ")"
++ format = '[\]($hash\)]($style) '
```

#### Git 상태

| 제거된 속성            | 대체       |
| ----------------- | -------- |
| `prefix`          | `format` |
| `suffix`          | `format` |
| `show_sync_count` | `format` |

**기본 구성 변경 사항**

```diff
[git_status]
-- prefix = "["
-- suffix = "]"
-- show_sync_count = false
++ format = '([\\\[$all_status$ahead_behind\\\]]($style) )'
```

이전에는 `show_sync_count` 속성이 브랜치가 원격 브랜치보다 앞서거나 뒤처진 커밋 수를 표시하도록 프롬프트를 구성하는 데 사용되었습니다.

v0.45.0 릴리스에서는 이를 `ahead`, `behind`, `diverged`의 세 가지 개별 속성으로 대체했습니다.

이전 `show_sync_count = true` 구성을 사용하도록 프롬프트를 구성하려면 구성 파일에 다음을 설정하세요.

```toml
[git_status]
ahead = "⇡${count}"
diverged = "⇕⇡${ahead_count}⇣${behind_count}"
behind = "⇣${count}"
```

#### 호스트 이름

| 제거된 속성   | 대체       |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**기본 구성 변경 사항**

```diff
[hostname]
-- prefix = ""
-- suffix = ""
++ format = "[$hostname]($style) in "
```

#### Singularity

| 제거된 속성   | 대체       |
| -------- | -------- |
| `label`  | `format` |
| `prefix` | `format` |
| `suffix` | `format` |

**기본 구성 변경 사항**

```diff
[singularity]
-- prefix = ""
-- suffix = ""
++ format = '[$symbol\\\[$env\\\]]($style) '
```

#### 시간

| 제거된 속성   | 대체            |
| -------- | ------------- |
| `format` | `time_format` |

**기본 구성 변경 사항**

```diff
[time]
-- format = "🕙[ %T ]"
++ time_format = "%T"
++ format = "at 🕙[$time]($style) "
```

#### 사용자 지정 명령

| 제거된 속성   | 대체       |
| -------- | -------- |
| `prefix` | `format` |
| `suffix` | `format` |

**기본 구성 변경 사항**

```diff
[custom.example]
-- prefix = ""
-- suffix = ""
++ format = "[$symbol$output]($style) "
```
