# ZSH only fires `preexec` when a command actually runs, so an empty ENTER
# would otherwise show the previous command's duration forever. We therefore
# create STARSHIP_START_TIME in preexec and destroy it after drawing the
# prompt, so a duration is only ever drawn once.

zmodload zsh/parameter zsh/datetime zsh/sched  # jobstates; EPOCHREALTIME; resize debounce

# Millis since epoch. zsh < 5.8 has no EPOCHREALTIME and falls back to
# starship's own clock. print -f rounds; no zsh/mathfunc, no `::STARSHIP:: time`
# on 5.8+.
if (( ${+EPOCHREALTIME} )); then
    __starship_get_time() { print -v STARSHIP_CAPTURED_TIME -f '%.0f' $(( EPOCHREALTIME * 1000 )); }
else
    __starship_get_time() { STARSHIP_CAPTURED_TIME=$(::STARSHIP:: time); }
fi

# Named prompt_<theme>_<hook> for Zsh's prompt system compatibility. See
# https://github.com/zsh-users/zsh/blob/2876c25a28b8052d6683027998cc118fc9b50157/Functions/Prompts/promptinit#L155
prompt_starship_precmd() {
    # Save the status, because subsequent commands in this function will change $?
    STARSHIP_CMD_STATUS=$? STARSHIP_PIPE_STATUS=(${pipestatus[@]})

    if (( ${+STARSHIP_START_TIME} )); then
        # Assign outside '(())': an expression evaluating to 0 has exit status
        # 1 there, which kills shells running with 'set -e'; '$(())' keeps
        # status 0 while still surfacing arithmetic errors.
        __starship_get_time && STARSHIP_DURATION=$(( STARSHIP_CAPTURED_TIME - STARSHIP_START_TIME ))
        unset STARSHIP_START_TIME
    else
        unset STARSHIP_DURATION STARSHIP_CMD_STATUS STARSHIP_PIPE_STATUS
    fi

    # Width and keymap stay first: a resize and a mode change each rewrite
    # their own entry in place rather than rebuilding the vector.
    STARSHIP_ARGS=(
        --terminal-width=$COLUMNS --keymap=${KEYMAP:-}
        --status=${STARSHIP_CMD_STATUS:-} --pipestatus="${STARSHIP_PIPE_STATUS[*]:-}"
        --cmd-duration=${STARSHIP_DURATION:-} --jobs=${#jobstates}
    )
    starship_stream_start
}

prompt_starship_preexec() {
    starship_stream_stop
    __starship_get_time && STARSHIP_START_TIME=$STARSHIP_CAPTURED_TIME
}

autoload -Uz add-zsh-hook add-zle-hook-widget
add-zsh-hook precmd prompt_starship_precmd
add-zsh-hook preexec prompt_starship_preexec
add-zsh-hook zshexit starship_stream_stop

# One map, one renderer, both sides:
#   1   left prompt text (real newlines) — what PROMPT expands
#   2   right prompt text — what RPROMPT expands
#   p   renderer pid (READY carries it: process substitution cannot set $!,
#       and coproc is one global slot a plugin may already own)
#   f   pipe fd, while watched
#   t   last COMPLETE payload, handed back as --timings= verbatim
typeset -gA STARSHIP_STREAM

# One compact frame (`KEYWORD\0first\0second\0`) from fd $1 into $2 $3 $4.
# $5 is an optional timeout in seconds: the handshake bounds its wait so a
# renderer that never paints cannot freeze the prompt. `zle -F` omits it
# because the watcher only fires once the pipe is already readable.
starship_read_frame() {
    local -a t=(${5:+-t} $5)
    IFS= read -r -u $1 -d '' $t "$2" &&
        IFS= read -r -u $1 -d '' $t "$3" &&
        IFS= read -r -u $1 -d '' $t "$4"
}

# Drops the renderer and its pipe. Timings stay: they belong to the session.
starship_stream_stop() {
    local fd=$STARSHIP_STREAM[f] pid=$STARSHIP_STREAM[p]
    [[ -n $fd ]] && { zle -F $fd 2>/dev/null; { exec {fd}<&- } 2>/dev/null }
    [[ -n $pid ]] && kill $pid 2>/dev/null
    STARSHIP_STREAM[f]= STARSHIP_STREAM[p]=
}

# `zle -F -w` invokes this as a widget, so BUFFER/PREDISPLAY/POSTDISPLAY are
# valid — a plain `zle -F` function reads them empty unconditionally, which is
# why this is not a function handler plus a pre-redraw mirror.
# One frame per call; if more are buffered the watcher fires again immediately,
# so a slow round of key queries never piles up behind a drain loop.
starship_stream_consume() {
    local fd=$1 kind first second
    if ! starship_read_frame $fd kind first second; then
        starship_stream_stop
        return
    fi
    case $kind in
        # Only precmd blocks on READY; a stream adopted mid-line lands here
        # instead. It carries a pid where a patch carries repaint bytes, so
        # clearing $second is what drops it onto the whole-prompt redraw below.
        READY) STARSHIP_STREAM[p]=$second second= ;&
        PATCH)
            STARSHIP_STREAM[1]=$first
            # Cell-precise left repaint only when the line editor is at rest;
            # otherwise redraw the whole prompt so typed input is never harmed.
            if [[ -n $second && -z ${BUFFER-}${PREDISPLAY-}${POSTDISPLAY-} ]]; then
                print -rn -- "$second"
            elif zle; then
                zle -I
                zle reset-prompt
            fi
            ;;
        # The right side has no cells to repaint in place: zsh draws RPROMPT
        # itself, so a redraw is the only way it reaches the screen.
        RIGHT)
            STARSHIP_STREAM[2]=$first
            zle && { zle -I; zle reset-prompt }
            ;;
        # Stream stays open after COMPLETE for dynamic modules still re-polling.
        COMPLETE) STARSHIP_STREAM[t]=$first ;;
    esac
}
zle -N starship_stream_consume

# Launch both renderers, then adopt each pipe with `zle -F -w`.
#
# With an argument, adoption is all this does: the watcher takes READY like any
# other frame and redraws when it lands. That is the path every redraw takes,
# because a redraw already has a prompt on screen and zle up to replace it.
# Only precmd blocks — it has to return prompt text, and a blank first draw
# would flash — so only precmd can be made to wait two seconds per side, and
# only precmd falls back to a synchronous render.
starship_stream_start() {
    starship_stream_stop
    local fd kind first second attempt
    exec {fd}< <(::STARSHIP:: stream --both --timings="$STARSHIP_STREAM[t]" "${STARSHIP_ARGS[@]}" 2>/dev/null)
    STARSHIP_STREAM[f]=$fd
    if (( $# )); then
        zle -F -w $fd starship_stream_consume
        return
    fi
    # Only READY ends the wait, but the right side's first paint usually beats
    # it onto the pipe, so anything arriving first is applied on the way past
    # rather than left for the watcher — the first draw then has both sides.
    # Bounded, so a renderer emitting anything but READY cannot hold the prompt.
    for attempt in 1 2 3 4; do
        starship_read_frame $fd kind first second 2 || break
        case $kind in
            RIGHT) STARSHIP_STREAM[2]=$first ;;
            READY)
                STARSHIP_STREAM[1]=$first STARSHIP_STREAM[p]=$second
                zle -F -w $fd starship_stream_consume
                return
                ;;
        esac
    done
    # No first paint: draw both sides the slow way and leave nothing watching.
    { exec {fd}<&- } 2>/dev/null
    STARSHIP_STREAM[f]=
    STARSHIP_STREAM[1]="$(::STARSHIP:: prompt "${STARSHIP_ARGS[@]}")"
    STARSHIP_STREAM[2]="$(::STARSHIP:: prompt --right "${STARSHIP_ARGS[@]}")"
}

# Only `character` reads --keymap, but a renderer learns one through argv and
# nothing else, so the mode indicator needs a fresh stream. Adopting it instead
# of blocking on it means Esc never costs more than the write to the pipe; the
# indicator changes when READY lands, a frame later.
starship_zle-keymap-select() {
    STARSHIP_ARGS[2]=--keymap=${KEYMAP:-}
    starship_stream_start adopt
}
add-zle-hook-widget zle-keymap-select starship_zle-keymap-select

# Chain any existing WINCH trap, then redraw on resize while the editor is up.
if (( ${+functions[TRAPWINCH]} )); then
    functions[starship_preserved_winch]=$functions[TRAPWINCH]
elif [[ -n ${traps[WINCH]-} ]]; then
    eval "starship_preserved_winch() { ${traps[WINCH]} }"
fi

# Width reaches a renderer through argv too, so a reflowed prompt means a new
# one. Dragging a window edge emits a WINCH per step, and a stream per step
# would be a spawn per step, so the respawn trails the drag by a second —
# `sched` is the only timer zsh has that neither blocks nor forks, and one
# second is its floor. Cancel by matching our own entry: bare indices shift
# under anything else the user has scheduled.
starship_stream_rewidth() {
    STARSHIP_ARGS[1]=--terminal-width=$COLUMNS
    zle && starship_stream_start adopt
}
TRAPWINCH() {
    (( ${+functions[starship_preserved_winch]} )) && starship_preserved_winch "$@"
    zle || return
    local pending=$zsh_scheduled_events[(I)*starship_stream_rewidth*]
    (( pending )) && sched -$pending
    sched +1 starship_stream_rewidth
    # Re-wrap what is already on screen now; the reflowed text arrives later.
    zle -I
    zle reset-prompt
}

print -v STARSHIP_SESSION_KEY -f '%04x%04x%04x%04x' $RANDOM $RANDOM $RANDOM $RANDOM
export STARSHIP_SHELL=zsh STARSHIP_SESSION_KEY
VIRTUAL_ENV_DISABLE_PROMPT=1
setopt promptsubst

# A variable, not a command substitution, so a redraw re-expands the stored
# prompt without rerunning starship (needs promptsubst).
PROMPT='${STARSHIP_STREAM[1]}'
RPROMPT='${STARSHIP_STREAM[2]}'
PROMPT2="$(::STARSHIP:: prompt --continuation)"
