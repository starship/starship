#!/usr/bin/env bash
# Drops you into an interactive shell running the streaming-demo prompt
# (examples/streaming-demo/starship.toml) against a debug build of this
# checkout's starship binary.
#
# Usage: examples/streaming-demo/try.sh [zsh|fish|bash|powershell|nu|xonsh]
# Defaults to zsh, the only cell-precise transport. Bash requires BLE_SH;
# Nushell and Xonsh honor NUSHELL_MAIN and XONSH_MAIN respectively.
#
# Self-contained: builds its own throwaway rc file rather than relying on
# your personal dotfiles, so it works even if your zsh/fish config isn't set
# up on this machine (or isn't what you normally use day to day).

set -euo pipefail

script_directory="$(cd "${BASH_SOURCE[0]%/*}" && pwd -P)"
repository_root="$(cd "$script_directory/../.." && pwd)"
shell_name="${1:-zsh}"

case "$shell_name" in
zsh | fish) shell_command=$shell_name ;;
bash) shell_command=${BASH:-bash} ;;
powershell) shell_command=${POWERSHELL_MAIN:-pwsh} ;;
nu) shell_command=${NUSHELL_MAIN:-nu} ;;
xonsh) shell_command=${XONSH_MAIN:-xonsh} ;;
*)
    printf "error: unsupported shell %q\n" "$shell_name" >&2
    exit 1
    ;;
esac

if ! shell_path=$(command -v -- "$shell_command"); then
    printf 'error: %s is not executable\n' "$shell_command" >&2
    exit 1
fi
if [[ $shell_name == bash && ! -r ${BLE_SH-} ]]; then
    printf 'error: bash streaming requires BLE_SH to name ble.sh\n' >&2
    exit 1
fi

printf 'Building starship...\n' >&2
cargo build --manifest-path "$repository_root/Cargo.toml" --quiet

export STARSHIP_CONFIG="$script_directory/starship.toml"

demo_rc_directory="$(mktemp -d)"
trap 'rm -rf -- "$demo_rc_directory"' EXIT
init="$demo_rc_directory/starship.$shell_name"
[[ $shell_name != zsh ]] || init="$demo_rc_directory/.zshrc"
[[ $shell_name != powershell ]] || init="$demo_rc_directory/starship.ps1"
"$repository_root/target/debug/starship" init "$shell_name" --print-full-init >"$init"

printf 'Starting %s with STARSHIP_CONFIG=%s\n' "$shell_name" "$STARSHIP_CONFIG" >&2
printf 'Watch git data refine after the directory and the clock tick unaided.\n' >&2

case "$shell_name" in
zsh)
    ZDOTDIR="$demo_rc_directory" "$shell_path" -i
    ;;
fish) "$shell_path" --no-config -i -C "source $init" ;;
bash)
    printf 'source %q\nsource %q\n' "$BLE_SH" "$init" >"$demo_rc_directory/bashrc"
    "$shell_path" --noprofile --rcfile "$demo_rc_directory/bashrc" -i
    ;;
powershell) "$shell_path" -NoLogo -NoProfile -NoExit -File "$init" ;;
nu) "$shell_path" --config "$init" ;;
xonsh) "$shell_path" --interactive --shell-type=prompt_toolkit --rc "$init" ;;
esac
