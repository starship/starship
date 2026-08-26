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

STARSHIP_BLE_ENABLED=
declare -A STARSHIP_STREAM=([0.prompt]='' [1.prompt]='')
STARSHIP_PROMPT_ARGUMENTS=()
STARSHIP_FRAME=()

function ble/prompt/backslash:starship {
    ble/prompt/unit/add-hash '${STARSHIP_STREAM[0.prompt]}'
    ble/prompt/process-prompt-string "${STARSHIP_STREAM[0.prompt]}"
}

starship_ble_read_frame() {
    local descriptor=$1 kind first second
    IFS= read -r -d '' -u "$descriptor" kind &&
        IFS= read -r -d '' -u "$descriptor" first &&
        IFS= read -r -d '' -u "$descriptor" second || return
    STARSHIP_FRAME=("$kind" "$first" "$second")
}

# ble.sh's `prompt_rps1` carries the right prompt, but it is drawn on the line
# holding the left prompt's final row, so it is padded with the left prompt's
# newlines. Either stream recomputes it from the latest left and right values.
starship_ble_refresh_rps1() {
    local lines=${STARSHIP_STREAM[0.prompt]//[!$'\n']}
    bleopt prompt_rps1="$lines${STARSHIP_STREAM[1.prompt]}"
}

starship_ble_set_prompt() {
    STARSHIP_STREAM[$1.prompt]=$2
    starship_ble_refresh_rps1
}

starship_ble_stream_stop() {
    local side=$1 descriptor=${STARSHIP_STREAM[$1.fd]-} process=${STARSHIP_STREAM[$1.pid]-}
    ble/util/idle.cancel "starship_ble_stream_step $side" 2>/dev/null || :
    [[ ! $descriptor ]] || exec {descriptor}<&-
    [[ ! $process ]] || kill "$process" 2>/dev/null || :
    unset "STARSHIP_STREAM[$side.fd]" "STARSHIP_STREAM[$side.pid]" "STARSHIP_STREAM[$side.done]"
}

starship_ble_stream_end() {
    local side=$1 complete=${STARSHIP_STREAM[$1.done]-} target=()
    ((side)) && target=(--right)
    starship_ble_stream_stop "$side"
    if [[ ! $complete ]]; then
        starship_ble_set_prompt "$side" "$(::STARSHIP:: prompt "${target[@]}" "${STARSHIP_PROMPT_ARGUMENTS[@]}")"
        ble/textarea#redraw
    fi
}

starship_ble_stream_step() {
    local side=$1 descriptor=${STARSHIP_STREAM[$1.fd]-}
    while read -t 0 -u "$descriptor"; do
        if ! starship_ble_read_frame "$descriptor"; then
            starship_ble_stream_end "$side"
            return
        fi
        case ${STARSHIP_FRAME[0]} in
            PATCH)
                starship_ble_set_prompt "$side" "${STARSHIP_FRAME[1]}"
                ble/textarea#redraw
                ;;
            COMPLETE)
                STARSHIP_STREAM[$side.timings]=${STARSHIP_FRAME[1]}
                STARSHIP_STREAM[$side.done]=1
                ;;
        esac
    done
    if ! kill -0 "${STARSHIP_STREAM[$side.pid]-}" 2>/dev/null; then
        starship_ble_stream_end "$side"
        return
    fi
    ble/util/idle.sleep 51
}

starship_ble_stream_start() {
    local side=$1 descriptor target=()
    shift
    ((side)) && target=(--right)
    starship_ble_stream_stop "$side"

    exec {descriptor}< <(
        ::STARSHIP:: stream --transport ble "${target[@]}" "$@" --timings="${STARSHIP_STREAM[$side.timings]-}" 2>/dev/null
    )
    STARSHIP_STREAM[$side.fd]=$descriptor
    if ! starship_ble_read_frame "$descriptor" ||
       [[ ${STARSHIP_FRAME[0]} != READY ]]; then
        starship_ble_stream_stop "$side"
        starship_ble_set_prompt "$side" "$(::STARSHIP:: prompt "${target[@]}" "$@")"
        return
    fi

    STARSHIP_STREAM[$side.pid]=${STARSHIP_FRAME[2]}
    starship_ble_set_prompt "$side" "${STARSHIP_FRAME[1]}"
    ble/util/idle.push "starship_ble_stream_step $side"
}

# Will be run before *every* command (even ones in pipes!)
starship_preexec() {
    # Save previous command's last argument, otherwise it will be set to "starship_preexec"
    local PREV_LAST_ARG=$1

    if [[ $STARSHIP_BLE_ENABLED ]]; then
        starship_ble_stream_stop 0
        starship_ble_stream_stop 1
    fi

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
        eval "$STARSHIP_PROMPT_COMMAND"
    fi

    local -a ARGS=(--terminal-width="${COLUMNS}" --status="${STARSHIP_CMD_STATUS}" --pipestatus="${STARSHIP_PIPE_STATUS[*]}" --jobs="${NUM_JOBS}" --shlvl="${SHLVL}")
    # Prepare the timer data, if needed.
    if [[ -n "${STARSHIP_START_TIME-}" ]]; then
        STARSHIP_END_TIME=$(::STARSHIP:: time)
        STARSHIP_DURATION=$((STARSHIP_END_TIME - STARSHIP_START_TIME))
        ARGS+=( --cmd-duration="${STARSHIP_DURATION}")
        STARSHIP_START_TIME=""
    fi
    if [[ $STARSHIP_BLE_ENABLED ]]; then
        STARSHIP_PROMPT_ARGUMENTS=("${ARGS[@]}")
        starship_ble_stream_start 1 "${ARGS[@]}"
        starship_ble_stream_start 0 "${ARGS[@]}"
        PS1='\q{starship}'
    else
        PS1="$(::STARSHIP:: prompt "${ARGS[@]}")"
    fi
    STARSHIP_PREEXEC_READY=true  # Signal that we can safely restart the timer
}

# If the user appears to be using https://github.com/akinomyoga/ble.sh,
# then hook our functions into their framework.
if [[ ${BLE_VERSION-} && _ble_version -ge 400 ]]; then
    STARSHIP_BLE_ENABLED=1
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
        if [[ "${PS0-}" != *"starship_preexec_ps0"* ]]; then
            PS0='${STARSHIP_START_TIME:$((STARSHIP_START_TIME="$(starship_preexec_ps0)",STARSHIP_PREEXEC_READY=0,0)):0}'"${PS0-}"
        fi
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
