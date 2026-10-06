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

    if [[ ${STARSHIP_BLE_STREAM-} ]]; then
        starship_ble_stream_stop
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
    if [[ ${STARSHIP_BLE_STREAM-} ]]; then
        starship_ble_stream_start "${ARGS[@]}"
        PS1='\q{starship}'
    else
        PS1="$(::STARSHIP:: prompt "${ARGS[@]}")"
        if [[ ${BLE_ATTACHED-} ]]; then
            local nlns=${PS1//[!$'\n']}
            bleopt prompt_rps1="$nlns$(::STARSHIP:: prompt --right "${ARGS[@]}")"
        fi
    fi
    STARSHIP_PREEXEC_READY=true  # Signal that we can safely restart the timer
}

# If the user appears to be using https://github.com/akinomyoga/ble.sh,
# then hook our functions into their framework, and stream the prompt where
# bash names descriptors by variables and declares globals inside functions.
if [[ ${BLE_VERSION-} && _ble_version -ge 400 ]]; then
    blehook PREEXEC!='starship_preexec "$_"'
    blehook PRECMD!='starship_precmd'
    ((BASH_VERSINFO[0] * 100 + BASH_VERSINFO[1] >= 402)) && STARSHIP_BLE_STREAM=1
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

# With ble.sh, the prompt is streamed: `starship prompt --stream` draws it as
# soon as what renders quickly has, and refines it as the rest does. Its frames arrive
# on a file descriptor that an idle task of ble.sh reads while the user types.
# STARSHIP_STREAM holds
#   1  the main prompt, which \q{starship} draws
#   2  the right prompt, which ble.sh draws as prompt_rps1
#   p  the process id the stream reported, to stop it with
#   f  the descriptor its frames arrive on
#   c  whether the stream has reported every module rendered
#   t  the timings it reported, which the next stream is handed
if [[ ${STARSHIP_BLE_STREAM-} ]]; then
    declare -gA STARSHIP_STREAM

    # ble.sh draws a part of a prompt again only when one of its hashes expands
    # differently, so the hash is the variable holding the main prompt.
    function ble/prompt/backslash:starship {
        ble/prompt/unit/add-hash '${STARSHIP_STREAM[1]}'
        ble/prompt/process-prompt-string "${STARSHIP_STREAM[1]}"
    }

    # Sets the main prompt to $1 and the right one to $2. ble.sh draws the
    # right prompt beside the main prompt's first line, so it is moved down by
    # as many lines as the main prompt has.
    starship_ble_set_prompts() {
        STARSHIP_STREAM[1]=$1 STARSHIP_STREAM[2]=$2
        local lines=${1//[!$'\n']}
        bleopt prompt_rps1="$lines$2"
    }

    # Sets the variable named $1 to the milliseconds since the epoch, without a
    # process where bash keeps the time itself.
    if [[ ${EPOCHREALTIME-} ]]; then
        starship_ble_now() { local microseconds=${EPOCHREALTIME/[^0-9]/}; printf -v "$1" %s "${microseconds:0:-3}"; }
    else
        starship_ble_now() { printf -v "$1" %s "$(::STARSHIP:: time)"; }
    fi

    # Reads a frame, a keyword and two fields, each ended by a NUL, from
    # descriptor $1 into the variables named $2, $3 and $4. ble.sh replaces
    # `read` with a function that reads from the terminal while it is attached,
    # so the builtin reads the stream.
    starship_ble_read_frame() {
        IFS= builtin read -r -d '' -u "$1" "$2" &&
            IFS= builtin read -r -d '' -u "$1" "$3" &&
            IFS= builtin read -r -d '' -u "$1" "$4"
    }

    # Stops the stream, and the idle task that reads it.
    starship_ble_stream_stop() {
        local fd=${STARSHIP_STREAM[f]-} pid=${STARSHIP_STREAM[p]-}
        ble/util/idle.cancel starship_ble_stream_step
        [[ $fd ]] && exec {fd}<&-
        [[ $pid ]] && kill "$pid" 2>/dev/null
        STARSHIP_STREAM[f]= STARSHIP_STREAM[p]= STARSHIP_STREAM[c]=
    }

    # Takes whatever frames have arrived whenever ble.sh is idle, and draws the
    # latest prompts once: drawing can ask the terminal where its cursor is,
    # and replies that arrive after the next drawing has begun land on the
    # command line as typed text. ble.sh draws what an idle task changed only
    # when its next task is further off than 50 milliseconds, so the task draws
    # them itself.
    starship_ble_stream_step() {
        local fd=${STARSHIP_STREAM[f]} kind first second main right drawn= ended=
        while builtin read -t 0 -u "$fd"; do
            if ! starship_ble_read_frame "$fd" kind first second; then
                ended=1
                break
            fi
            case $kind in
                PROMPT) main=$first right=$second drawn=1 ;;
                COMPLETE) STARSHIP_STREAM[t]=$first STARSHIP_STREAM[c]=1 ;;
            esac
        done
        if [[ $drawn ]]; then
            starship_ble_set_prompts "$main" "$right"
            ble/application/render
        fi
        if [[ $ended ]]; then
            starship_ble_stream_stop
        elif [[ ${STARSHIP_STREAM[c]-} ]]; then
            # From then on, frames arrive only as refreshes fall due, on
            # multiples of their periods in wall-clock time, so the stream is
            # looked at once a second, just after the second turns over.
            local now
            starship_ble_now now
            ble/util/idle.sleep $((1050 - now % 1000))
        else
            # Refinements arrive no closer together than this.
            ble/util/idle.sleep 50
        fi
    }

    # Starts a stream for the prompt about to be drawn, with the arguments that
    # tell starship about the shell, and waits for its first prompts, which the
    # stream draws within its fallback. A stream that fails draws nothing, as a
    # failing `starship prompt` would.
    starship_ble_stream_start() {
        starship_ble_stream_stop
        local fd kind first second
        exec {fd}< <(::STARSHIP:: prompt --stream --timings="${STARSHIP_STREAM[t]-}" "$@" 2>/dev/null)
        STARSHIP_STREAM[f]=$fd
        starship_ble_set_prompts "" ""
        while starship_ble_read_frame "$fd" kind first second; do
            case $kind in
                PROCESS) STARSHIP_STREAM[p]=$first ;;
                PROMPT)
                    starship_ble_set_prompts "$first" "$second"
                    ble/util/idle.push starship_ble_stream_step
                    return
                    ;;
            esac
        done
        starship_ble_stream_stop
    }
fi

# Set up the start time and STARSHIP_SHELL, which controls shell-specific sequences
STARSHIP_START_TIME=$(::STARSHIP:: time)
export STARSHIP_SHELL="bash"

# Set up the session key that will be used to store logs
STARSHIP_SESSION_KEY="$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM"; # Random generates a number b/w 0 - 32767
STARSHIP_SESSION_KEY="${STARSHIP_SESSION_KEY}0000000000000000" # Pad it to 16+ chars.
export STARSHIP_SESSION_KEY=${STARSHIP_SESSION_KEY:0:16}; # Trim to 16-digits if excess.

# Set the continuation prompt
PS2="$(::STARSHIP:: prompt --continuation)"

