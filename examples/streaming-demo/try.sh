#!/usr/bin/env bash
# Drops you into an interactive shell running the streaming-demo prompt
# (examples/streaming-demo/starship.toml) against a debug build of this
# checkout's starship binary.
#
# Usage: examples/streaming-demo/try.sh [zsh|fish]
# Defaults to zsh, the only shell that reaches Tier::CellPrecise, so
# git_branch/git_status visibly repaint in place once they resolve and the
# clock ticks without a cell-width reflow.
#
# Self-contained: builds its own throwaway rc file rather than relying on
# your personal dotfiles, so it works even if your zsh/fish config isn't set
# up on this machine (or isn't what you normally use day to day).

set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/../.." && pwd)"
shell_name="${1:-zsh}"

case "$shell_name" in
zsh | fish) ;;
*)
    echo "error: try.sh supports zsh or fish (got '$shell_name')" >&2
    echo "  zsh reaches Tier::CellPrecise; fish reaches Tier::PromptReplace." >&2
    exit 1
    ;;
esac

if ! command -v "$shell_name" >/dev/null 2>&1; then
    echo "error: $shell_name is not on PATH" >&2
    exit 1
fi

echo "Building starship..." >&2
cargo build --manifest-path "$repository_root/Cargo.toml" --quiet

export STARSHIP_CONFIG="$script_directory/starship.toml"

demo_rc_directory="$(mktemp -d)"
trap 'rm -rf "$demo_rc_directory"' EXIT

echo "Starting $shell_name with the streaming demo prompt (STARSHIP_CONFIG=$STARSHIP_CONFIG)." >&2
echo "Watch \$git_branch/\$git_status pop in after \$directory, and the clock tick on its own." >&2

debug_bin_directory="$repository_root/target/debug"

if [ "$shell_name" = zsh ]; then
    # Re-assert PATH here rather than trusting the inherited one: on this
    # machine /etc/zshenv sources nix-darwin's set-environment, which
    # re-prepends the Nix profile's own (older, non-streaming) starship
    # *after* zsh starts, silently shadowing the one built above.
    cat >"$demo_rc_directory/.zshrc" <<EOF
export PATH="$debug_bin_directory:\$PATH"
eval "\$(starship init zsh)"
EOF
    ZDOTDIR="$demo_rc_directory" zsh -i
else
    cat >"$demo_rc_directory/config.fish" <<EOF
set -gx PATH "$debug_bin_directory" \$PATH
starship init fish | source
EOF
    fish -i -C "source $demo_rc_directory/config.fish"
fi
