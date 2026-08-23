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

STARSHIP_STREAM_DESCRIPTOR=
STARSHIP_STREAM_PROCESS=
STARSHIP_STREAM_COMPLETE=
STARSHIP_TIMINGS=

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
    if [[ -n $STARSHIP_STREAM_DESCRIPTOR ]]; then
        zle -F "$STARSHIP_STREAM_DESCRIPTOR" 2>/dev/null
        { exec {STARSHIP_STREAM_DESCRIPTOR}<&- } 2>/dev/null
        STARSHIP_STREAM_DESCRIPTOR=
    fi
    if [[ -n $STARSHIP_STREAM_PROCESS ]]; then
        kill "$STARSHIP_STREAM_PROCESS" 2>/dev/null
        STARSHIP_STREAM_PROCESS=
    fi
    STARSHIP_STREAM_COMPLETE=
}

starship_editor_is_busy() {
    [[ ${WIDGET-} == *complete* || ${WIDGET-} == *search* ||
       ${LASTWIDGET-} == *complete* || ${LASTWIDGET-} == *search* ]]
}

starship_editor_is_at_prompt() {
    [[ -z ${BUFFER-}${PREDISPLAY-}${POSTDISPLAY-} ]]
}

starship_apply_prompt() {
    local prompt=$1 repaint=$2
    STARSHIP_PROMPT=$prompt
    starship_editor_is_busy && return

    if [[ -n $repaint ]] && starship_editor_is_at_prompt; then
        zle -I
        print -rn -- "$repaint"
    else
        zle reset-prompt
    fi
}

starship_stream_readable() {
    if [[ -n ${2-} ]] || ! starship_read_frame "$1"; then
        local complete=$STARSHIP_STREAM_COMPLETE
        starship_stream_stop
        if [[ ! $complete ]]; then
            starship_render_synchronously
            zle reset-prompt
        fi
        return
    fi

    case $reply[1] in
        PATCH)
            starship_apply_prompt "$reply[2]" "$reply[3]"
            ;;
        COMPLETE)
            STARSHIP_TIMINGS=$reply[2]
            STARSHIP_STREAM_COMPLETE=1
            ;;
    esac
}

starship_render_synchronously() {
    starship_stream_stop
    starship_prompt_arguments
    local -a arguments=("${reply[@]}")
    STARSHIP_PROMPT="$(::STARSHIP:: prompt "${arguments[@]}")"
    STARSHIP_RIGHT_PROMPT="$(::STARSHIP:: prompt --right "${arguments[@]}")"
}

starship_stream_start() {
    starship_stream_stop
    starship_prompt_arguments
    local -a arguments=("${reply[@]}")

    STARSHIP_RIGHT_PROMPT="$(::STARSHIP:: prompt --right "${arguments[@]}")"
    exec {STARSHIP_STREAM_DESCRIPTOR}< <(::STARSHIP:: stream "${arguments[@]}" --timings="$STARSHIP_TIMINGS" 2>/dev/null)

    if [[ -z $STARSHIP_STREAM_DESCRIPTOR ]] ||
       ! starship_read_frame "$STARSHIP_STREAM_DESCRIPTOR" ||
       [[ $reply[1] != READY ]]; then
        starship_render_synchronously
        return
    fi

    STARSHIP_PROMPT=$reply[2]
    STARSHIP_STREAM_PROCESS=$reply[3]
    zle -F "$STARSHIP_STREAM_DESCRIPTOR" starship_stream_readable
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
PROMPT='${STARSHIP_PROMPT}'
RPROMPT='${STARSHIP_RIGHT_PROMPT}'
PROMPT2="$(::STARSHIP:: prompt --continuation)"
