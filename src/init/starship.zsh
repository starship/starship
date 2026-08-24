# ZSH has a quirk where `preexec` is only run if a command is actually run (i.e
# pressing ENTER at an empty command line will not cause preexec to fire). This
# can cause timing issues, as a user who presses "ENTER" without running a command
# will see the time to the start of the last command, which may be very large.

# To fix this, we create STARSHIP_START_TIME upon preexec() firing, and destroy it
# after drawing the prompt. This ensures that the timing for one command is only
# ever drawn once (for the prompt immediately after it is run).

zmodload zsh/parameter  # Needed to access jobstates variable for STARSHIP_JOBS_COUNT

# Defines a function `__starship_get_time` that sets the time since epoch in millis in STARSHIP_CAPTURED_TIME.
if [[ $ZSH_VERSION == ([1-4]*) ]]; then
    # ZSH <= 5; Does not have a built-in variable so we will rely on Starship's inbuilt time function.
    __starship_get_time() {
        STARSHIP_CAPTURED_TIME=$(::STARSHIP:: time)
    }
else
    zmodload zsh/datetime
    zmodload zsh/mathfunc
    __starship_get_time() {
        (( STARSHIP_CAPTURED_TIME = int(rint(EPOCHREALTIME * 1000)) ))
    }
fi

# The two functions below follow the naming convention `prompt_<theme>_<hook>`
# for compatibility with Zsh's prompt system. See
# https://github.com/zsh-users/zsh/blob/2876c25a28b8052d6683027998cc118fc9b50157/Functions/Prompts/promptinit#L155

# Runs before each new command line.
prompt_starship_precmd() {
    # Save the status, because subsequent commands in this function will change $?
    STARSHIP_CMD_STATUS=$? STARSHIP_PIPE_STATUS=(${pipestatus[@]})

    # Calculate duration if a command was executed
    if (( ${+STARSHIP_START_TIME} )); then
        # If an arithmetic expression evaluates to 0, its exit status is 1:
        # "The return status is 0 if the arithmetic value of the expression is non-zero, 1 if it is zero, and 2 if an error occurred."
        # In rare cases, the subtraction below can result in an int 0 result (yes, really),
        # which would then kill the shell if 'set -e' is in effect.
        # We therefore have to assign the result outside the expression (using 'STARSHIP_DURATION=$((...))'),
        # because unlike '(())', '$(())' gets a return status of 0 even if the expression evaluates to int 0
        # (but it still surfaces a potential error, normally status 2, as status 1).
        __starship_get_time && STARSHIP_DURATION=$(( STARSHIP_CAPTURED_TIME - STARSHIP_START_TIME ))
        unset STARSHIP_START_TIME
    # Drop status and duration otherwise
    else
        unset STARSHIP_DURATION STARSHIP_CMD_STATUS STARSHIP_PIPE_STATUS
    fi

    # Use length of jobstates array as number of jobs. Expansion fails inside
    # quotes so we set it here and then use the value later on.
    STARSHIP_JOBS_COUNT="${#jobstates[*]}"

    starship_stream_start
}

# Runs after the user submits the command line, but before it is executed and
# only if there's an actual command to run
prompt_starship_preexec() {
    # Stop the stream now — the line it was rendering into is about to be replaced.
    starship_stream_stop
    __starship_get_time && STARSHIP_START_TIME=$STARSHIP_CAPTURED_TIME
}

# Add hook functions
autoload -Uz add-zsh-hook
add-zsh-hook precmd prompt_starship_precmd
add-zsh-hook preexec prompt_starship_preexec

add-zsh-hook zshexit starship_stream_stop

typeset -gA STARSHIP_STREAM=(left.prompt '' right.prompt '')

# Mirrors "is the edit buffer empty" into an ordinary variable, because the
# obvious direct check cannot be done from where it is needed. `zle -F`
# handlers (see `starship_stream_readable` below) are invoked through the
# plain shell function-call mechanism, not through zle's widget dispatch, so
# the zle special parameters `BUFFER`/`PREDISPLAY`/`POSTDISPLAY` are simply
# unbound inside them — they read as empty unconditionally, whether or not
# the user has actually typed anything. That made the previous direct check
# there a no-op that always reported "buffer is empty", so a refinement's
# fast path (a raw cell-precise repaint, safe only when nothing is on screen
# but the prompt) could fire while the user was mid-edit and splice its
# output into the middle of their unsubmitted command line.
#
# `zle-line-pre-redraw` is a real widget: zle calls it immediately before
# every redraw of the line, with the special parameters valid, so this is
# the one place that can observe them truthfully. Recording the answer here,
# for a `zle -F` handler to read back later, is the only reliable way to
# learn it from that side.
typeset -g STARSHIP_EDITOR_BUFFER_EMPTY=1

starship_prompt_arguments() {
    reply=(
        --terminal-width="$COLUMNS"
        --keymap="${KEYMAP:-}"
        --status="${STARSHIP_CMD_STATUS:-}"
        --pipestatus="${STARSHIP_PIPE_STATUS[*]:-}"
        --cmd-duration="${STARSHIP_DURATION:-}"
        --jobs="${STARSHIP_JOBS_COUNT:-0}"
    )
}

starship_read_frame() {
    local descriptor=$1 kind first second
    IFS= read -r -d '' -u "$descriptor" kind &&
        IFS= read -r -d '' -u "$descriptor" first &&
        IFS= read -r -d '' -u "$descriptor" second || return
    reply=("$kind" "$first" "$second")
}

starship_stream_stop() {
    local side descriptor process
    for side in ${@:-left right}; do
        descriptor=${STARSHIP_STREAM[$side.fd]-}
        process=${STARSHIP_STREAM[$side.pid]-}
        if [[ -n $descriptor ]]; then
            zle -F "$descriptor" 2>/dev/null
            { exec {descriptor}<&- } 2>/dev/null
            unset "STARSHIP_STREAM[$descriptor.side]"
        fi
        [[ -z $process ]] || kill "$process" 2>/dev/null
        unset "STARSHIP_STREAM[$side.fd]" "STARSHIP_STREAM[$side.pid]" "STARSHIP_STREAM[$side.done]"
    done
}

starship_editor_is_busy() {
    [[ ${WIDGET-} == *complete* || ${WIDGET-} == *search* ||
       ${LASTWIDGET-} == *complete* || ${LASTWIDGET-} == *search* ]]
}

starship_editor_is_at_prompt() {
    (( STARSHIP_EDITOR_BUFFER_EMPTY ))
}

starship_apply_prompt() {
    local side=$1 prompt=$2 repaint=$3
    STARSHIP_STREAM[$side.prompt]=$prompt
    starship_editor_is_busy && return

    if [[ $side == left && -n $repaint ]] && starship_editor_is_at_prompt; then
        zle -I
        print -rn -- "$repaint"
    else
        # zle -F handlers run outside the ordinary widget context zle expects,
        # so a `reset-prompt` here without first invalidating the display can
        # make zle redisplay the wrong prompt in the wrong place (it has been
        # observed drawing the right prompt's freshly patched text over the
        # left prompt's position). `zle -I` (see zshzle(1)) is the documented
        # way to tell zle its cached display is stale before an out-of-band
        # handler asks it to redraw.
        zle -I
        zle reset-prompt
    fi
}

starship_stream_readable() {
    local descriptor=$1 side=${STARSHIP_STREAM[$1.side]-} complete target=()
    [[ -n $side ]] || return

    if [[ -n ${2-} ]] || ! starship_read_frame "$descriptor"; then
        complete=${STARSHIP_STREAM[$side.done]-}
        starship_stream_stop "$side"
        if [[ ! $complete ]]; then
            [[ $side == right ]] && target=(--right)
            starship_prompt_arguments
            STARSHIP_STREAM[$side.prompt]="$(::STARSHIP:: prompt "${target[@]}" "${reply[@]}")"
            # See the comment beside `zle -I` in starship_apply_prompt above.
            zle -I
            zle reset-prompt
        fi
        return
    fi

    case $reply[1] in
        PATCH)
            starship_apply_prompt "$side" "$reply[2]" "$reply[3]"
            ;;
        COMPLETE)
            STARSHIP_STREAM[$side.timings]=$reply[2]
            STARSHIP_STREAM[$side.done]=1
            ;;
    esac
}

starship_stream_start_one() {
    local side=$1 descriptor
    local -a target=()
    shift
    [[ $side == right ]] && target=(--right)

    exec {descriptor}< <(
        ::STARSHIP:: stream "${target[@]}" "$@" --timings="${STARSHIP_STREAM[$side.timings]-}" 2>/dev/null
    )
    STARSHIP_STREAM[$side.fd]=$descriptor
    STARSHIP_STREAM[$descriptor.side]=$side
    if ! starship_read_frame "$descriptor" || [[ $reply[1] != READY ]]; then
        starship_stream_stop "$side"
        STARSHIP_STREAM[$side.prompt]="$(::STARSHIP:: prompt "${target[@]}" "$@")"
        return
    fi

    STARSHIP_STREAM[$side.prompt]=$reply[2]
    STARSHIP_STREAM[$side.pid]=$reply[3]
    zle -F "$descriptor" starship_stream_readable
}

starship_stream_start() {
    starship_stream_stop
    starship_prompt_arguments
    local -a arguments=("${reply[@]}")
    starship_stream_start_one right "${arguments[@]}"
    starship_stream_start_one left "${arguments[@]}"
}

# Set up a function to redraw the prompt if the user switches vi modes
starship_zle-keymap-select() {
    # Keymap changes require a new render.
    starship_stream_start
    zle reset-prompt
}

## Check for existing keymap-select widget.
if [[ -v widgets[zle-keymap-select] ]]; then
    # zle-keymap-select is a special widget so it'll be "user:fnName" or nothing. Let's get fnName only.
    __starship_preserved_zle_keymap_select=${widgets[zle-keymap-select]#user:}
fi

if [[ -z ${__starship_preserved_zle_keymap_select:-} ]]; then
    zle -N zle-keymap-select starship_zle-keymap-select;
else
    # Define a wrapper fn to call the original widget fn and then Starship's.
    starship_zle-keymap-select-wrapped() {
        $__starship_preserved_zle_keymap_select "$@";
        starship_zle-keymap-select "$@";
    }
    zle -N zle-keymap-select starship_zle-keymap-select-wrapped;
fi

# Keeps STARSHIP_EDITOR_BUFFER_EMPTY (see its declaration above) in sync with
# reality. Unlike `zle-keymap-select`, this widget can fire on every single
# keystroke, so it does no more than a plain parameter check.
starship_zle-line-pre-redraw() {
    if [[ -z ${BUFFER-}${PREDISPLAY-}${POSTDISPLAY-} ]]; then
        STARSHIP_EDITOR_BUFFER_EMPTY=1
    else
        STARSHIP_EDITOR_BUFFER_EMPTY=0
    fi
}

## Check for an existing zle-line-pre-redraw widget (zsh-syntax-highlighting
## and similar plugins commonly define one) so it keeps working unchanged.
if [[ -v widgets[zle-line-pre-redraw] ]]; then
    __starship_preserved_zle_line_pre_redraw=${widgets[zle-line-pre-redraw]#user:}
fi

if [[ -z ${__starship_preserved_zle_line_pre_redraw:-} ]]; then
    zle -N zle-line-pre-redraw starship_zle-line-pre-redraw;
else
    # Define a wrapper fn to call the original widget fn and then Starship's.
    starship_zle-line-pre-redraw-wrapped() {
        $__starship_preserved_zle_line_pre_redraw "$@";
        starship_zle-line-pre-redraw "$@";
    }
    zle -N zle-line-pre-redraw starship_zle-line-pre-redraw-wrapped;
fi

if (( ${+functions[TRAPWINCH]} )); then
    functions[starship_preserved_winch_handler]=$functions[TRAPWINCH]
elif [[ -n ${traps[WINCH]-} ]]; then
    eval "starship_preserved_winch_handler() { ${traps[WINCH]} }"
fi

TRAPWINCH() {
    if (( ${+functions[starship_preserved_winch_handler]} )); then
        starship_preserved_winch_handler "$@"
    fi
    if zle; then
        starship_stream_start
        # A trap is, like a `zle -F` handler, outside the ordinary widget
        # context — see the comment beside `zle -I` in starship_apply_prompt.
        zle -I
        zle reset-prompt
    fi
}

export STARSHIP_SHELL="zsh"

# Set up the session key that will be used to store logs
STARSHIP_SESSION_KEY="$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM"; # Random generates a number b/w 0 - 32767
STARSHIP_SESSION_KEY="${STARSHIP_SESSION_KEY}0000000000000000" # Pad it to 16+ chars.
export STARSHIP_SESSION_KEY=${STARSHIP_SESSION_KEY:0:16}; # Trim to 16-digits if excess.

VIRTUAL_ENV_DISABLE_PROMPT=1

setopt promptsubst

# A variable, not a command substitution, so a redraw re-expands the stored prompt without rerunning starship (needs promptsubst).
PROMPT='${STARSHIP_STREAM[left.prompt]}'
RPROMPT='${STARSHIP_STREAM[right.prompt]}'
PROMPT2="$(::STARSHIP:: prompt --continuation)"
