# ZSH has a quirk where `preexec` is only run if a command is actually run (i.e
# pressing ENTER at an empty command line will not cause preexec to fire). This
# can cause timing issues, as a user who presses "ENTER" without running a command
# will see the time to the start of the last command, which may be very large.

# To fix this, we create STARSHIP_CMD_START_TIME upon preexec() firing, and destroy it
# after drawing the prompt. This ensures that the timing for one command is only
# ever drawn once (for the prompt immediately after it is run).

autoload is-at-least
zmodload zsh/parameter # Needed to access jobstates variable for STARSHIP_JOBS_COUNT

if is-at-least 4.3.13; then
    zmodload zsh/datetime
    zmodload zsh/mathfunc
    __starship_time() {
        echo $(( int(rint(EPOCHREALTIME * 1000)) ))
    }
else
    __starship_time() {
        ::STARSHIP:: time
    }
fi

# Will be run before every prompt draw
__starship_precmd() {
    # Save the status, because commands in this pipeline will change $?
    STARSHIP_CMD_STATUS=$?

    # Compute cmd_duration, if we have a time to consume, otherwise clear the
    # previous duration
    if (( ${+STARSHIP_CMD_START_TIME} )); then
        STARSHIP_CMD_DURATION=$(( $(__starship_time) - STARSHIP_CMD_START_TIME ))
        unset STARSHIP_CMD_START_TIME
    else
        unset STARSHIP_CMD_DURATION
    fi

    # Use length of jobstates array as number of jobs. Expansion fails inside
    # quotes so we set it here and then use the value later on.
    STARSHIP_JOBS_COUNT=${#jobstates}
}
__starship_preexec() {
    STARSHIP_CMD_START_TIME="$(__starship_time)"
}

# If precmd/preexec arrays are not already set, set them. If we don't do this,
# the code to detect whether __starship_precmd is already in precmd_functions will
# fail because the array doesn't exist (and same for __starship_preexec)
(( ! ${+precmd_functions} )) && precmd_functions=()
(( ! ${+preexec_functions} )) && preexec_functions=()

# If starship precmd/preexec functions are already hooked, don't double-hook them
# to avoid unnecessary performance degradation in nested shells
(( ! ${precmd_functions[(Ie)__starship_precmd]} )) && precmd_functions+=(__starship_precmd)
(( ! ${preexec_functions[(Ie)__starship_preexec]} )) && preexec_functions+=(__starship_preexec)

# Set up a function to redraw the prompt if the user switches vi modes
# zle-keymap-select is a special widget named like "user:handler-name"
zle_keymap_select=${${widgets[zle-keymap-select]}//user:}
if [[ -n $zle_keymap_select ]]; then
    __starship-zle-keymap-select() {
        $zle_keymap_select "$@"
        zle reset-prompt
    }
else
    __starship-zle-keymap-select() {
        zle reset-prompt
    }
fi
zle -N zle-keymap-select __starship-zle-keymap-select

STARSHIP_CMD_START_TIME="$(__starship_time)"

# Disable virtualenv prompt, it breaks starship
VIRTUAL_ENV_DISABLE_PROMPT=1

export STARSHIP_SHELL="zsh"

# Set up the session key that will be used to store logs
export STARSHIP_SESSION_KEY=${(r:16::0:)"$(repeat 4 echo -n $RANDOM)"}

__starship_prompt() {
    ::STARSHIP:: prompt \
        --keymap=$KEYMAP \
        --status=$STARSHIP_CMD_STATUS \
        --cmd-duration=$STARSHIP_CMD_DURATION \
        --jobs=$STARSHIP_JOBS_COUNT
}

setopt promptsubst
PROMPT='$(__starship_prompt)'
