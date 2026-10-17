# ZSH has a quirk where `preexec` is only run if a command is actually run (i.e
# pressing ENTER at an empty command line will not cause preexec to fire). This
# can cause timing issues, as a user who presses "ENTER" without running a command
# will see the time to the start of the last command, which may be very large.

# To fix this, we create STARSHIP_START_TIME upon preexec() firing, and destroy it
# after drawing the prompt. This ensures that the timing for one command is only
# ever drawn once (for the prompt immediately after it is run).

zmodload zsh/parameter  # Needed to access jobstates variable for STARSHIP_JOBS_COUNT
zmodload zsh/sched  # Debounce terminal resizing without starting another process

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

    # Run after every precmd hook has updated the shell environment.
    sched +0 starship_stream_start
}

# Runs after the user submits the command line, but before it is executed and
# only if there's an actual command to run
prompt_starship_preexec() {
    starship_stream_stop
    __starship_get_time && STARSHIP_START_TIME=$STARSHIP_CAPTURED_TIME
}

# Add hook functions
autoload -Uz add-zsh-hook
add-zsh-hook precmd prompt_starship_precmd
add-zsh-hook preexec prompt_starship_preexec
add-zsh-hook zshexit starship_stream_stop

# ZLE reads the stream while the user types. STARSHIP_STREAM holds the
# prompts last read, and the descriptor they come from.
typeset -gA STARSHIP_STREAM

# Closing the descriptor ends the stream. Closing with `exec` applies its
# redirection to the shell itself, so its error output is silenced around the
# closing alone.
starship_stream_stop() {
    local descriptor=${STARSHIP_STREAM[f]-}
    [[ -n $descriptor ]] && { zle -F $descriptor 2>/dev/null; { exec {descriptor}<&- } 2>/dev/null }
    STARSHIP_STREAM[f]=
}

# Show the prompts last read as they are: text, which a user who turns on
# promptsubst has escaped to expand to itself.
starship_stream_show() {
    local main=${STARSHIP_STREAM[1]-} right=${STARSHIP_STREAM[2]-}
    if [[ -o promptsubst ]]; then
        main=${${${main//\\/\\\\}//\$/\\\$}//\`/\\\`}
        right=${${${right//\\/\\\\}//\$/\\\$}//\`/\\\`}
    fi
    PROMPT=$main RPROMPT=$right
}

# Read the next prompts from descriptor $1, or stop at the end of the stream.
starship_stream_read() {
    local main right
    IFS= read -r -u $1 -d '' main && IFS= read -r -u $1 -d '' right || {
        starship_stream_stop
        return 1
    }
    STARSHIP_STREAM[1]=$main STARSHIP_STREAM[2]=$right
}

starship_stream_redraw() {
    starship_stream_read $1 && { starship_stream_show; zle -I; zle reset-prompt }
}
zle -N starship_stream_redraw

# Wait for the first paint, unless given an argument to keep the prompts shown
# until the new ones arrive.
starship_stream_start() {
    starship_stream_stop
    local descriptor
    exec {descriptor}< <(::STARSHIP:: prompt --stream --terminal-width="$COLUMNS" --keymap="${KEYMAP:-}" --status="${STARSHIP_CMD_STATUS:-}" --pipestatus="${STARSHIP_PIPE_STATUS[*]:-}" --cmd-duration="${STARSHIP_DURATION:-}" --jobs="$STARSHIP_JOBS_COUNT")
    STARSHIP_STREAM[f]=$descriptor
    if (( ! $# )); then
        STARSHIP_STREAM[1]= STARSHIP_STREAM[2]=
        starship_stream_read $descriptor
        starship_stream_show
    fi
    [[ -n ${STARSHIP_STREAM[f]} ]] && zle -F -w $descriptor starship_stream_redraw
}

# Debounce resize rendering for one second with a nonblocking, fork-free timer.
(( ${+functions[TRAPWINCH]} )) && functions[starship_preserved_winch]=$functions[TRAPWINCH]
starship_stream_resize() {
    zle && starship_stream_start adopt
}
TRAPWINCH() {
    (( ${+functions[starship_preserved_winch]} )) && starship_preserved_winch "$@"
    zle || return
    local pending=$zsh_scheduled_events[(I)*starship_stream_resize*]
    (( pending )) && sched -$pending
    sched +1 starship_stream_resize
    # Rewrap the prompt already shown until the new one arrives.
    zle -I
    zle reset-prompt
}

# Restart asynchronously when vi mode changes.
starship_zle-keymap-select() {
    starship_stream_start adopt
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

export STARSHIP_SHELL="zsh"

# Set up the session key that will be used to store logs
STARSHIP_SESSION_KEY="$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM"; # Random generates a number b/w 0 - 32767
STARSHIP_SESSION_KEY="${STARSHIP_SESSION_KEY}0000000000000000" # Pad it to 16+ chars.
export STARSHIP_SESSION_KEY=${STARSHIP_SESSION_KEY:0:16}; # Trim to 16-digits if excess.

VIRTUAL_ENV_DISABLE_PROMPT=1

PROMPT2="$(::STARSHIP:: prompt --continuation)"
