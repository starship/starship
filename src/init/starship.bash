# We use PROMPT_COMMAND and the DEBUG trap to generate timing information. We try
# to avoid clobbering what we can, and try to give the user ways around our
# clobbers, if it's unavoidable. For example, PROMPT_COMMAND is appended to,
# and the DEBUG trap is layered with other traps, if it exists.

# A bash quirk is that the DEBUG trap is fired every time a command runs, even
# if it's later on in the pipeline. If uncorrected, this could cause bad timing
# data for commands like `slow | slow | fast`, since the timer starts at the start
# of the "fast" command.

# To solve this, we set a flag `STARSHIP_PREEXEC_READY` when the prompt is
# drawn, and only start the timer if this flag is present. That way, timing is
# for the entire command, and not just a portion of it.

# A way to set '$?', since bash does not allow assigning to '$?' directly
function _starship_set_return() { return "${1:-0}"; }

# Will be run before *every* command (even ones in pipes!)
starship_preexec() {
    # Save previous command's last argument, otherwise it will be set to "starship_preexec"
    local PREV_LAST_ARG=$1

    # Avoid restarting the timer for commands in the same pipeline
    if [ "${STARSHIP_PREEXEC_READY:-}" = "true" ]; then
        STARSHIP_PREEXEC_READY=false
        STARSHIP_START_TIME=$(::STARSHIP:: time)
    fi

    : "$PREV_LAST_ARG"
}

# Will be run before the prompt is drawn
starship_precmd() {
    # Save the status, because commands in this pipeline will change $?
    STARSHIP_CMD_STATUS=$? STARSHIP_PIPE_STATUS=("${PIPESTATUS[@]}")
    if [[ ${BLE_ATTACHED-} && ${#BLE_PIPESTATUS[@]} -gt 0 ]]; then
        STARSHIP_PIPE_STATUS=("${BLE_PIPESTATUS[@]}")
    fi
    if [[ -n "${BP_PIPESTATUS-}" ]] && [[ "${#BP_PIPESTATUS[@]}" -gt 0 ]]; then
        STARSHIP_PIPE_STATUS=("${BP_PIPESTATUS[@]}")
    fi

    # Due to a bug in certain Bash versions, any external process launched
    # inside $PROMPT_COMMAND will be reported by `jobs` as a background job:
    #
    #   [1]  42135 Done                    /bin/echo
    #
    # This is a workaround - we run `jobs` once to clear out any completed jobs
    # first, and then we run it again and count the number of jobs.
    #
    # More context: https://github.com/starship/starship/issues/5159
    # Original bug: https://lists.gnu.org/archive/html/bug-bash/2022-07/msg00117.html
    jobs &>/dev/null

    local job NUM_JOBS=0 IFS=$' \t\n'
    # Evaluate the number of jobs before running the preserved prompt command, so that tools
    # like z/autojump, which background certain jobs, do not cause spurious background jobs
    # to be displayed by starship. Also avoids forking to run `wc`, slightly improving perf.
    for job in $(jobs -p); do [[ $job ]] && ((NUM_JOBS++)); done

    # Run the bash precmd function, if it's set. If not set, evaluates to no-op
    "${starship_precmd_user_func-:}"

    # Set $? to the preserved value before running additional parts of the prompt
    # command pipeline, which may rely on it.
    _starship_set_return "$STARSHIP_CMD_STATUS"

    if [[ -n "${STARSHIP_PROMPT_COMMAND-}" ]]; then
        if ((BASH_VERSINFO[0] > 5 || BASH_VERSINFO[0] == 5 && BASH_VERSINFO[1] >= 1)) && [[ "${STARSHIP_PROMPT_COMMAND@a}" == "a" ]]; then
            # Extended `STARSHIP_PROMPT_COMMAND` array usage
            # If Bash 5.1+ and `STARSHIP_PROMPT_COMMAND` is non-empty array
            # PR: https://github.com/starship/starship/pull/7603
            \builtin local __starship_prompt_subcommand
            for __starship_prompt_subcommand in "${STARSHIP_PROMPT_COMMAND[@]}"; do
                eval "${__starship_prompt_subcommand}"
            done
        else
            eval "$STARSHIP_PROMPT_COMMAND"
        fi
    fi

    local -a ARGS=(--terminal-width="${COLUMNS}" --status="${STARSHIP_CMD_STATUS}" --pipestatus="${STARSHIP_PIPE_STATUS[*]}" --jobs="${NUM_JOBS}" --shlvl="${SHLVL}")
    # Prepare the timer data, if needed.
    if [[ -n "${STARSHIP_START_TIME-}" ]]; then
        STARSHIP_END_TIME=$(::STARSHIP:: time)
        STARSHIP_DURATION=$((STARSHIP_END_TIME - STARSHIP_START_TIME))
        ARGS+=("--cmd-duration=${STARSHIP_DURATION}")
        STARSHIP_START_TIME=""
    fi
    PS1="$(::STARSHIP:: prompt "${ARGS[@]}")"
    if [[ ${BLE_ATTACHED-} ]]; then
        local nlns=${PS1//[!$'\n']}
        bleopt prompt_rps1="$nlns$(::STARSHIP:: prompt --right "${ARGS[@]}")"
    fi
    STARSHIP_PREEXEC_READY=true  # Signal that we can safely restart the timer
}

# If the user appears to be using https://github.com/akinomyoga/ble.sh,
# then hook our functions into their framework.
if [[ ${BLE_VERSION-} && _ble_version -ge 400 ]]; then
    blehook PREEXEC!='starship_preexec "$_"'
    blehook PRECMD!='starship_precmd'
# If the user appears to be using https://github.com/rcaloras/bash-preexec,
# then hook our functions into their framework.
elif [[ -n "${bash_preexec_imported:-}" || -n "${__bp_imported:-}" || -n "${preexec_functions-}" || -n "${precmd_functions-}" ]]; then
    # bash-preexec needs a single function--wrap the args into a closure and pass
    starship_preexec_all(){ starship_preexec "$_"; }
    preexec_functions+=(starship_preexec_all)
    precmd_functions+=(starship_precmd)
else
    if [[ -n "${BASH_VERSION-}" ]] && [[ "${BASH_VERSINFO[0]}" -gt 4 || ( "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 ) ]]; then
        starship_preexec_ps0() {
            ::STARSHIP:: time
        }
        # In order to set STARSHIP_START_TIME use an arithmetic expansion that evaluates to 0
        # To avoid printing anything, use the return value in an ${var:offset:length} substring expansion
        # with offset and length evaluating to 0.
        PS0='${STARSHIP_START_TIME:$((STARSHIP_START_TIME="$(starship_preexec_ps0)",STARSHIP_PREEXEC_READY=0,0)):0}'"${PS0-}"
    else
        # We want to avoid destroying an existing DEBUG hook. If we detect one, create
        # a new function that runs both the existing function AND our function, then
        # re-trap DEBUG to use this new function. This prevents a trap clobber.
        eval "STARSHIP_DEBUG_TRAP=($(trap -p DEBUG))"
        STARSHIP_DEBUG_TRAP=("${STARSHIP_DEBUG_TRAP[2]}")
        if [[ -z "$STARSHIP_DEBUG_TRAP" ]]; then
            trap 'starship_preexec "$_"' DEBUG
        elif [[ "$STARSHIP_DEBUG_TRAP" != 'starship_preexec "$_"' && "$STARSHIP_DEBUG_TRAP" != 'starship_preexec_all "$_"' ]]; then
            starship_preexec_all() {
                local PREV_LAST_ARG=$1 ; eval -- "$STARSHIP_DEBUG_TRAP"; starship_preexec; : "$PREV_LAST_ARG";
            }
            trap 'starship_preexec_all "$_"' DEBUG
        fi
    fi

    # Finally, prepare the precmd function and set up the start time. We will avoid to
    # add multiple instances of the starship function and keep other user functions if any.
    if ((BASH_VERSINFO[0] > 5 || BASH_VERSINFO[0] == 5 && BASH_VERSINFO[1] >= 1)) && [[ -v PROMPT_COMMAND && "${PROMPT_COMMAND@a}" == "a" ]]; then
        # In Bash 5.1+, the type of PROMPT_COMMAND can be 'array'. Old assignment
        # commands will work with the first element instead of the full PROMPT_COMMAND.
        # Even so, when 'string' PROMPT_COMMAND is detected when `Starship` is triggered,
        # use old fallback logic.
        # PR: https://github.com/starship/starship/pull/7603
        if [[ -v IFS ]]; then
            \builtin declare __starship_ifs="${IFS}"
        else
            \builtin unset __starship_ifs
        fi
        \builtin declare IFS=''  # Use empty IFS to figure out if array is empty
        \builtin declare -ag STARSHIP_PROMPT_COMMAND  # Force global variable
        if [[ -z "${STARSHIP_PROMPT_COMMAND[*]}" ]]; then  # Fix: Reenter this line should not overwrite STARSHIP_PROMPT_COMMAND
            STARSHIP_PROMPT_COMMAND=()
        fi

        \builtin declare __prompt_subcommand
        for __prompt_subcommand in "${PROMPT_COMMAND[@]}"; do  # Test if PROMPT_COMMAND contains 'starship_precmd'
            if [[ "${__prompt_subcommand}" == *"starship_precmd"* ]]; then  # If modify this line, modify line 158,178 as well
                \builtin break
            fi
        done

        if [[ "${__prompt_subcommand}" != *"starship_precmd"* ]]; then  # If not contain 'starship_precmd' (first initialization)
            for __prompt_subcommand in "${PROMPT_COMMAND[@]}"; do  # Join non-empty STARSHIP_PROMPT_COMMAND array
                if [[ -n "${__prompt_subcommand}" ]]; then
                    STARSHIP_PROMPT_COMMAND+=("${__prompt_subcommand}")
                fi
            done
            PROMPT_COMMAND=("starship_precmd")
            if [[ -z "${STARSHIP_PROMPT_COMMAND[*]}" ]]; then  # If STARSHIP_PROMPT_COMMAND is empty
                \builtin unset STARSHIP_PROMPT_COMMAND  # Unset STARSHIP_PROMPT_COMMAND
            fi
        fi
        \builtin unset __prompt_subcommand

        if [[ -v __starship_ifs ]]; then  # If detect IFS defined
            # TODO: Maybe we can remove IFS when declared in a function
            IFS="${__starship_ifs}" # Recover IFS
            \builtin unset __starship_ifs
        else
            \builtin unset IFS  # Remove definition of IFS
        fi
    else
        if [[ -z "${PROMPT_COMMAND-}" ]]; then
            PROMPT_COMMAND="starship_precmd"
        elif [[ "$PROMPT_COMMAND" != *"starship_precmd"* ]]; then
            # Appending to PROMPT_COMMAND breaks exit status ($?) checking.
            # Prepending to PROMPT_COMMAND breaks "command duration" module.
            # So, we are preserving the existing PROMPT_COMMAND
            # which will be executed later in the starship_precmd function
            STARSHIP_PROMPT_COMMAND="$PROMPT_COMMAND"
            PROMPT_COMMAND="starship_precmd"
        fi
    fi
fi

# Ensure that $COLUMNS gets set
shopt -s checkwinsize

# Set up the start time and STARSHIP_SHELL, which controls shell-specific sequences
STARSHIP_START_TIME=$(::STARSHIP:: time)
export STARSHIP_SHELL="bash"

# Set up the session key that will be used to store logs
STARSHIP_SESSION_KEY="$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM"; # Random generates a number b/w 0 - 32767
STARSHIP_SESSION_KEY="${STARSHIP_SESSION_KEY}0000000000000000" # Pad it to 16+ chars.
export STARSHIP_SESSION_KEY=${STARSHIP_SESSION_KEY:0:16}; # Trim to 16-digits if excess.

# Set the continuation prompt
PS2="$(::STARSHIP:: prompt --continuation)"
