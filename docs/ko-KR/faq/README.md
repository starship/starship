# 자주 묻는 질문

## 데모 GIF에는 어떤 구성을 사용했나요?

- **터미널 에뮬레이터**: [iTerm2](https://iterm2.com/)
  - **테마**: Minimal
  - **배색**: [Snazzy](https://github.com/sindresorhus/iterm2-snazzy)
  - **폰트**: [FiraCode Nerd Font](https://www.nerdfonts.com/font-downloads)
- **셸**: [Fish Shell](https://fishshell.com/)
  - **구성**: [matchai's Dotfiles](https://github.com/matchai/dotfiles/blob/b6c6a701d0af8d145a8370288c00bb9f0648b5c2/.config/fish/config.fish)
  - **프롬프트**: [Starship](https://starship.rs/)

## How do I get command completion as shown in the demo GIF?

Completion support, or autocomplete, is provided by your shell of choice. In the case of the demo, the demo was done with [Fish Shell](https://fishshell.com/), which provides completions by default. If you use Z Shell (zsh), I'd suggest taking a look at [zsh-autosuggestions](https://github.com/zsh-users/zsh-autosuggestions).

## Do top level `format` and `<module>.disabled` do the same thing?

Yes, they can both be used to disable modules in the prompt. If all you plan to do is disable modules, `<module>.disabled` is the preferred way to do so for these reasons:

- Disabling modules is more explicit than omitting them from the top level `format`
- Newly created modules will be added to the prompt as Starship is updated

## The docs say Starship is cross-shell. Why isn't my preferred shell supported?

The way Starship is built, it should be possible to add support for virtually any shell. The starship binary is stateless and shell agnostic, so as long as your shell supports prompt customization and shell expansion, Starship can be used.

Here's a small example getting Starship working with bash:

```sh
# Get the status code from the last command executed
STATUS=$?

# Get the number of jobs running.
NUM_JOBS=$(jobs -p | wc -l)

# Set the prompt to the output of `starship prompt`
PS1="$(starship prompt --status=$STATUS --jobs=$NUM_JOBS)"
```

The [Bash implementation](https://github.com/starship/starship/blob/master/src/init/starship.bash) built into Starship is slightly more complex to allow for advanced features like the [Command Duration module](https://starship.rs/config/#command-duration) and to ensure that Starship is compatible with pre-installed Bash configurations.

`starship 프롬프트`에서 지원하는 모든 플래그 값을 보려면 아래 명령어를 사용하세요:

```sh
starship prompt --help
```

The prompt will use as much context as is provided, but no flags are "required".

## 오래된 버전의 glibc가 있는 Linux 배포판에서 Starship을 어떻게 실행하나요?

미리 빌드된 바이너리를 실행할 때 (예를 들어 CentOS 6 혹은 7에서) "_version 'GLIBC_2.18' not found (required by starship)_" 같은 오류가 보인다면, `glibc` 대신 `musl`로 컴파일된 바이너리 파일을 사용하세요.

```sh
curl -sS https://starship.rs/install.sh | sh -s -- --platform unknown-linux-musl
```

## 왜 `Executing command "..." timed out.` 경고가 뜨나요?

Starship executes different commands to get information to display in the prompt, for example the version of a program or the current git status. To make sure starship doesn't hang while trying to execute these commands we set a time limit, if a command takes longer than this limit starship will stop the execution of the command and output the above warning, this is expected behaviour. This time limit is configurable using the [`command_timeout`key](../config/#prompt) so if you want you can increase the time limit. You can also follow the debugging steps below to see which command is being slow and see if you can optimise it. Finally you can set the `STARSHIP_LOG` env var to `error` to hide these warnings.

## 이해할 수 없거나 예상치 못한 기호가 보이는데 무슨 뜻인가요?

If you see symbols that you don't recognise you can use `starship explain` to explain the currently showing modules.

## Starship is doing something unexpected, how can I debug it?

You can enable the debug logs by using the `STARSHIP_LOG` env var. These logs can be very verbose so it is often useful to use the `module` command if you are trying to debug a particular module, for example, if you are trying to debug the `rust` module you could run the following command to get the trace logs and output from the module.

```sh
env STARSHIP_LOG=trace starship module rust
```

If starship is being slow you can try using the `timings` command to see if there is a particular module or command that is to blame.

```sh
env STARSHIP_LOG=trace starship timings
```

This will output the trace log and a breakdown of all modules that either took more than 1ms to execute or produced some output.

Finally if you find a bug you can use the `bug-report` command to create a GitHub issue.

```sh
starship bug-report
```

## Why don't I see a glyph symbol in my prompt?

The most common cause of this is system misconfiguration. Some Linux distros in particular do not come with font support out-of-the-box. You need to ensure that:

- Your locale is set to a UTF-8 value, like `de_DE.UTF-8` or `ja_JP.UTF-8`. If `LC_ALL` is not a UTF-8 value, [you will need to change it](https://www.tecmint.com/set-system-locales-in-linux/).
- You have an emoji font installed. Most systems come with an emoji font by default, but some (notably Arch Linux) do not. You can usually install one through your system's package manager--[noto emoji](https://www.google.com/get/noto/help/emoji/) is a popular choice.
- You are using a [Nerd Font](https://www.nerdfonts.com/).

To test your system, run the following commands in a terminal:

```sh
echo -e "\xf0\x9f\x90\x8d"
echo -e "\xee\x82\xa0"
```

The first line should produce a [snake emoji](https://emojipedia.org/snake/), while the second should produce a [powerline branch symbol (e0a0)](https://github.com/ryanoasis/powerline-extra-symbols#glyphs).

If either symbol fails to display correctly, your system is still misconfigured. Unfortunately, getting font configuration correct is sometimes difficult. Users on the Discord may be able to help. If both symbols display correctly, but you still don't see them in starship, [file a bug report!](https://github.com/starship/starship/issues/new/choose)

## Starship을 어떻게 삭제하나요?

Starship은 처음 설치하는 것만큼이나 쉽게 제거할 수 있습니다.

1. 셸 설정 파일 (예시: `~/.bashrc`) 에서 Starship 초기화에 사용되는 모든 줄을 제거하세요.
1. Starship 바이너리 파일을 제거하세요.

Starship을 패키지 매니저로 설치하였다면 해당 패키지 매니저의 제거 지침 문서를 참조해 주세요.

Starship을 설치 스크립트로 설치하였다면 바이너리 파일 제거를 위해 아래 명령어를 실행하세요:

```sh
# starship 바이너리 파일을 찾고 제거합니다.
sh -c 'rm "$(command -v 'starship')"'
```

## How do I install Starship without `sudo`?

The shell install script (`https://starship.rs/install.sh`) only attempts to use `sudo` if the target installation directory is not writable by the current user. The default installation directory is the value of the `$BIN_DIR` environment variable or `/usr/local/bin` if `$BIN_DIR` is not set. If you instead set the installation directory to one that is writable by your user, you should be able to install starship without `sudo`. For example, `curl -sS https://starship.rs/install.sh | sh -s -- -b ~/.local/bin` uses the `-b` command line option of the install script to set the installation directory to `~/.local/bin`.

For a non-interactive installation of Starship, don't forget to add the `-y` option to skip the confirmation. Check the source of the installation script for a list of all supported installation options.

When using a package manager, see the documentation for your package manager about installing with or without `sudo`.
