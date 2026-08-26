# Timing uses PROMPT_COMMAND plus either the DEBUG trap, PS0, or ble.sh hooks.
# We append rather than clobber wherever the shell allows it. A bash quirk:
# the DEBUG trap fires for *every* command in a pipeline, which would time
# only the last stage of `slow | slow | fast`. We therefore set
# STARSHIP_PREEXEC_READY once the prompt is drawn and start the timer only
# while it holds, so timing covers the whole line.

_starship_set_return() { return "${1:-0}"; } # bash cannot assign $?

# Milliseconds since epoch. EPOCHREALTIME (bash 5.0+) is seconds.fraction —
# same split ble.sh uses. PS0/DEBUG on bash 4.x still forks `starship time`.
if [[ ${EPOCHREALTIME-} ]]; then
    starship_now() { local LC_ALL= LC_NUMERIC=C t=$EPOCHREALTIME f=${t#*.}000; printf -v "$1" %s "$((${t%%.*}*1000+10#${f:0:3}))"; }
else
    starship_now() { printf -v "$1" %s "$(::STARSHIP:: time)"; }
fi

# STARSHIP_PROMPT is 0=left 1=right; one renderer draws both, so its fd, pid,
# timings and completion are each one value rather than a pair. A frame is
# KEYWORD\0first\0second\0; prompts carry their own newlines, so PS1 receives
# them with nothing to decode. Export the shell name before any hook can spawn
# `stream --transport ble`.
STARSHIP_PROMPT=() STARSHIP_FRAME=() STARSHIP_ARGS=()
STARSHIP_FD= STARSHIP_PID= STARSHIP_DONE= STARSHIP_TIMINGS=
STARSHIP_RPS1_PAD=
export STARSHIP_SHELL=bash

# \q{starship} is the left prompt. Hash the variable (ble's $PWD pattern),
# not the expanded text: a frozen string never expires the cached unit, so
# a PATCH of the same line count would leave FAST on screen forever.
# ble.sh draws rps1 on the left prompt's final row, so a multi-line left
# pads the right with that many newlines — a static bleopt, not a second
# \q unit (those issue extra cursor queries).
function ble/prompt/backslash:starship {
    ble/prompt/unit/add-hash '${STARSHIP_PROMPT[0]}'
    ble/prompt/process-prompt-string "${STARSHIP_PROMPT[0]}"
}
# The pad is the left prompt with everything but its newlines deleted, so it
# only ever changes when the left side does. Recomputing it on a right-side
# patch would rescan the whole left prompt, escapes and all, for a string that
# cannot have changed — and a right-side clock patches far more often than the
# left is redrawn.
starship_ble_set_prompt() {
    STARSHIP_PROMPT[$1]=$2
    ((${1})) || STARSHIP_RPS1_PAD=${2//[!$'\n']}
    bleopt prompt_rps1="${STARSHIP_RPS1_PAD}${STARSHIP_PROMPT[1]}"
}

# $2 is a handshake timeout in seconds: a renderer that never paints must
# not freeze the prompt. Idle reads are already gated by `read -t 0`.
# mapfile has no timeout, so the handshake waits on the first field
# (frames are written atomically).
starship_ble_read_frame() {
    STARSHIP_FRAME=()
    if [[ ${2-} ]]; then IFS= read -r -d '' -t "$2" -u "$1" 'STARSHIP_FRAME[0]' && mapfile -d '' -t -n 2 -O 1 -u "$1" STARSHIP_FRAME
    else mapfile -d '' -t -n 3 -u "$1" STARSHIP_FRAME; fi
    ((${#STARSHIP_FRAME[@]}==3))
}

starship_ble_stream_stop() {
    local fd=$STARSHIP_FD pid=$STARSHIP_PID
    [[ $fd ]] && exec {fd}<&-
    [[ $pid ]] && kill "$pid" 2>/dev/null || :
    STARSHIP_FD= STARSHIP_PID= STARSHIP_DONE=
}

# A dead renderer falls back to a synchronous render of both sides, unless it
# had already reported COMPLETE and so has nothing left to say.
starship_ble_stream_end() {
    local complete=$STARSHIP_DONE
    starship_ble_stream_stop
    if [[ ! $complete ]]; then
        starship_ble_set_prompt 0 "$(::STARSHIP:: prompt "${STARSHIP_ARGS[@]}")"
        starship_ble_set_prompt 1 "$(::STARSHIP:: prompt --right "${STARSHIP_ARGS[@]}")"
        ble/textarea#redraw
    fi
}

# One idle task, one frame per side per tick, then one redraw. Draining a
# burst of PATCHes races ble.sh cursor-position queries and corrupts the
# line (57af33fb). idle.sleep 51 is the documented 20Hz yield — not a
# substitute for that one-frame cap.
starship_ble_stream_step() {
    local fd=$STARSHIP_FD drew
    if [[ $fd ]]; then
        if read -t 0 -u "$fd"; then
            if starship_ble_read_frame "$fd"; then
                case ${STARSHIP_FRAME[0]} in
                    PATCH) starship_ble_set_prompt 0 "${STARSHIP_FRAME[1]}"; drew=1 ;;
                    RIGHT) starship_ble_set_prompt 1 "${STARSHIP_FRAME[1]}"; drew=1 ;;
                    COMPLETE) STARSHIP_TIMINGS=${STARSHIP_FRAME[1]}; STARSHIP_DONE=1 ;;
                esac
            else starship_ble_stream_end; fi
        elif ! kill -0 "$STARSHIP_PID" 2>/dev/null; then
            starship_ble_stream_end
        fi
    fi
    [[ $drew ]] && ble/textarea#redraw
    # Measured: handing the wait to ble.sh's own backing-off idle interval via
    # `idle.wait-condition` (its only fd-ish primitive — it has no wait-on-
    # readable-fd) slowed refinement by more than an order of magnitude in the
    # pty suite. The flat 20Hz yield stays.
    [[ $STARSHIP_FD ]] && ble/util/idle.sleep 51
}

starship_ble_stream_start() {
    local fd attempt
    starship_ble_stream_stop
    exec {fd}< <(::STARSHIP:: stream --both --transport ble "$@" --timings="$STARSHIP_TIMINGS" 2>/dev/null)
    STARSHIP_FD=$fd
    # READY is what ends the wait, but the right side's first paint usually
    # reaches the pipe before it, so anything else is applied on the way past
    # and the first draw has both sides. Bounded, so a renderer that never says
    # READY cannot hold the prompt.
    for attempt in 1 2 3 4; do
        starship_ble_read_frame "$fd" 2 || break
        case ${STARSHIP_FRAME[0]} in
            RIGHT) starship_ble_set_prompt 1 "${STARSHIP_FRAME[1]}" ;;
            READY)
                STARSHIP_PID=${STARSHIP_FRAME[2]}
                starship_ble_set_prompt 0 "${STARSHIP_FRAME[1]}"
                ble/util/idle.cancel starship_ble_stream_step 2>/dev/null || :
                ble/util/idle.push starship_ble_stream_step
                return
                ;;
        esac
    done
    starship_ble_stream_stop
    starship_ble_set_prompt 0 "$(::STARSHIP:: prompt "$@")"
    starship_ble_set_prompt 1 "$(::STARSHIP:: prompt --right "$@")"
}

# `\j` (bash 4.4 ${var@P}) is the job-count builtin; older bash still splits
# `jobs -p`. BASH_VERSINFO cannot change mid-session, so the branch — and the
# `eval` that keeps ${var@P} from being a parse error on the shells that lack
# it — is spent once here rather than on every prompt.
if ((BASH_VERSINFO[0]*100+BASH_VERSINFO[1]>=404)); then
    eval 'starship_count_jobs() { local _j="\j"; NUM_JOBS=${_j@P}; }'
else
    starship_count_jobs() { set -- $(jobs -p); NUM_JOBS=$#; }
fi

starship_preexec() {
    local PREV_LAST_ARG=$1
    if [[ $STARSHIP_BLE ]]; then
        ble/util/idle.cancel starship_ble_stream_step 2>/dev/null || :
        starship_ble_stream_stop
    fi
    # Avoid restarting the timer for commands in the same pipeline.
    if [[ ${STARSHIP_PREEXEC_READY:-} == true ]]; then
        STARSHIP_PREEXEC_READY=false
        starship_now STARSHIP_START_TIME
    fi
    : "$PREV_LAST_ARG"
}

starship_precmd() {
    STARSHIP_CMD_STATUS=$? STARSHIP_PIPE_STATUS=("${PIPESTATUS[@]}")
    [[ ${BLE_ATTACHED-} && ${#BLE_PIPESTATUS[@]} -gt 0 ]] && STARSHIP_PIPE_STATUS=("${BLE_PIPESTATUS[@]}")
    [[ ${BP_PIPESTATUS+x} && ${#BP_PIPESTATUS[@]} -gt 0 ]] && STARSHIP_PIPE_STATUS=("${BP_PIPESTATUS[@]}")

    # A bash bug reports external processes launched inside PROMPT_COMMAND as
    # background jobs (starship#5159); run `jobs` once to flush finished ones,
    # then count. The flush has to stay here, per prompt and ahead of the
    # preserved user command, or tools like z/autojump show phantom jobs.
    jobs &>/dev/null
    local NUM_JOBS=0
    starship_count_jobs

    "${starship_precmd_user_func-:}"
    _starship_set_return "$STARSHIP_CMD_STATUS" # remaining user prompt pipeline
    [[ ${STARSHIP_PROMPT_COMMAND-} ]] && eval "$STARSHIP_PROMPT_COMMAND"

    local -a ARGS=(--terminal-width="${COLUMNS}" --status="${STARSHIP_CMD_STATUS}" --pipestatus="${STARSHIP_PIPE_STATUS[*]}" --jobs="${NUM_JOBS}" --shlvl="${SHLVL}")
    if [[ ${STARSHIP_START_TIME-} ]]; then
        starship_now STARSHIP_NOW
        ARGS+=(--cmd-duration=$((STARSHIP_NOW - STARSHIP_START_TIME)))
        STARSHIP_START_TIME=
    fi
    if [[ $STARSHIP_BLE ]]; then
        STARSHIP_ARGS=("${ARGS[@]}")
        starship_ble_stream_start "${ARGS[@]}"
        PS1='\q{starship}'
    else
        PS1="$(::STARSHIP:: prompt "${ARGS[@]}")"
    fi
    STARSHIP_PREEXEC_READY=true
}

# Hook into ble.sh when present, else bash-preexec, else raw hooks.
if [[ ${BLE_VERSION-} && $_ble_version -ge 400 ]]; then
    STARSHIP_BLE=1
    blehook PREEXEC!='starship_preexec "$_"'
    blehook PRECMD!='starship_precmd'
elif [[ ${bash_preexec_imported-} || ${__bp_imported-} || ${preexec_functions-} || ${precmd_functions-} ]]; then
    starship_preexec_all() { starship_preexec "$_"; }
    preexec_functions+=(starship_preexec_all)
    precmd_functions+=(starship_precmd)
else
    if ((BASH_VERSINFO[0]*100+BASH_VERSINFO[1]>=404)); then
        # Capture the start time inside PS0; the arithmetic-expansion
        # substring trick assigns while printing nothing.
        if [[ ${PS0-} != *STARSHIP_START_TIME* ]]; then
            if [[ ${EPOCHREALTIME-} ]]; then
                PS0='${STARSHIP_START_TIME:$((STARSHIP_START_TIME=${EPOCHREALTIME//./}/1000,STARSHIP_PREEXEC_READY=0,0)):0}'"${PS0-}"
            else
                starship_preexec_ps0() { ::STARSHIP:: time; }
                PS0='${STARSHIP_START_TIME:$((STARSHIP_START_TIME="$(starship_preexec_ps0)",STARSHIP_PREEXEC_READY=0,0)):0}'"${PS0-}"
            fi
        fi
    else
        # Layer onto an existing DEBUG trap instead of clobbering it.
        eval "STARSHIP_DEBUG_TRAP=($(trap -p DEBUG))"
        STARSHIP_DEBUG_TRAP=("${STARSHIP_DEBUG_TRAP[2]}")
        if [[ -z $STARSHIP_DEBUG_TRAP ]]; then
            trap 'starship_preexec "$_"' DEBUG
        elif [[ $STARSHIP_DEBUG_TRAP != 'starship_preexec "$_"' && $STARSHIP_DEBUG_TRAP != 'starship_preexec_all "$_"' ]]; then
            starship_preexec_all() { local PREV_LAST_ARG=$1; eval -- "$STARSHIP_DEBUG_TRAP"; starship_preexec; : "$PREV_LAST_ARG"; }
            trap 'starship_preexec_all "$_"' DEBUG
        fi
    fi
    # Preserve any existing PROMPT_COMMAND: appending breaks $? propagation,
    # so we hold it and run it from inside starship_precmd instead.
    if [[ ${PROMPT_COMMAND-} != *starship_precmd* ]]; then
        STARSHIP_PROMPT_COMMAND=${PROMPT_COMMAND-}
        PROMPT_COMMAND=starship_precmd
    fi
fi

shopt -s checkwinsize
printf -v STARSHIP_SESSION_KEY '%04x%04x%04x%04x' $RANDOM $RANDOM $RANDOM $RANDOM
export STARSHIP_SESSION_KEY
PS2="$(::STARSHIP:: prompt --continuation)"
