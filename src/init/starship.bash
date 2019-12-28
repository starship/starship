# We use PROMPT_COMMAND and the DEBUG trap to generate timing information. We try
# to avoid clobbering what we can, and try to give the user ways around our
# clobbers, if it's unavoidable. For example, PROMPT_COMMAND is appended to,
# and the DEBUG trap is layered with other traps, if it exists.

# A bash quirk is that the DEBUG trap is fired every time a command runs, even
# if it's later on in the pipeline. If uncorrected, this could cause bad timing
# data for commands like `slow | slow | fast`, since the timer starts at the start
# of the "fast" command.

# To solve this, we set a flag `PREEXEC_READY` when the prompt is drawn, and only
# start the timer if this flag is present. That way, timing is for the entire command,
# and not just a portion of it.

# Will be run before *every* command (even ones in pipes!)
starship_preexec() {
    # Save previous command's last argument, otherwise it will be set to "starship_preexec"
    local PREV_LAST_ARG=$1

    # Avoid restarting the timer for commands in the same pipeline
    if [ "$PREEXEC_READY" = "true" ]; then
        PREEXEC_READY=false
        STARSHIP_START_TIME=$(::STARSHIP:: time)
    fi
    
    : "$PREV_LAST_ARG"
}

# Will be run before the prompt is drawn
starship_precmd() {
    # Save the status, because commands in this pipeline will change $?
    STATUS=$?

    # Run the bash precmd function, if it's set. If not set, evaluates to no-op
    "${starship_precmd_user_func-:}"

    # Prepare the timer data, if needed.
    if [[ $STARSHIP_START_TIME ]]; then
        STARSHIP_END_TIME=$(::STARSHIP:: time)
        STARSHIP_DURATION=$((STARSHIP_END_TIME - STARSHIP_START_TIME))
        PS1="$(::STARSHIP:: prompt --status=$STATUS --jobs="$(jobs -p | wc -l)" --cmd-duration=$STARSHIP_DURATION)"
        unset STARSHIP_START_TIME
    else
        PS1="$(::STARSHIP:: prompt --status=$STATUS --jobs="$(jobs -p | wc -l)")"
    fi
    PREEXEC_READY=true;  # Signal that we can safely restart the timer
}

# If the user appears to be using https://github.com/rcaloras/bash-preexec,
# then hook our functions into their framework.
if [[ $preexec_functions ]]; then
    preexec_functions+=('starship_preexec "$_"')
    precmd_functions+=(starship_precmd)
else
# We want to avoid destroying an existing DEBUG hook. If we detect one, create
# a new function that runs both the existing function AND our function, then
# re-trap DEBUG to use this new function. This prevents a trap clobber.
    dbg_trap="$(trap -p DEBUG | cut -d' ' -f3 | tr -d \')"
    if [[ -z "$dbg_trap" ]]; then
        trap 'starship_preexec "$_"' DEBUG
    elif [[ "$dbg_trap" != 'starship_preexec "$_"' && "$dbg_trap" != 'starship_preexec_all "$_"' ]]; then
        function starship_preexec_all(){
            local PREV_LAST_ARG=$1 ; $dbg_trap; starship_preexec; : "$PREV_LAST_ARG";
        }
        trap 'starship_preexec_all "$_"' DEBUG
    fi
 
    # Finally, prepare the precmd function and set up the start time. We will avoid to 
    # add multiple instances of the starship function and keep other user functions if any.
    if [[ -z "$PROMPT_COMMAND" ]]; then
        PROMPT_COMMAND="starship_precmd"
    elif [[ "$PROMPT_COMMAND" != *"starship_precmd" ]]; then
        # Remove any trailing semicolon before appending (PR #784)
        PROMPT_COMMAND="${PROMPT_COMMAND%;};starship_precmd"
    fi
fi

# Set up the start time and STARSHIP_SHELL, which controls shell-specific sequences
STARSHIP_START_TIME=$(::STARSHIP:: time)
export STARSHIP_SHELL="bash"