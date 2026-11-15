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

__starship_get_terminal_position() {
    emulate -L zsh
    setopt extendedglob
    local LC_ALL=C
    unset STARSHIP_TERMINAL_ROW STARSHIP_TERMINAL_COLUMN
    if [[ ${STARSHIP_CURSOR_QUERY_DISABLED:-0} == 1 || ! -o interactive || ! -t 0 || ! -t 1 || -z ${TTY:-} || ${TERM:-dumb} == dumb ]]; then
        return 0
    fi
    (( ${KEYS_QUEUED_COUNT:-0} == 0 )) || return 0
    [[ ${STARSHIP_ADD_NEWLINE:-0} == 1 ]] || return 0

    local -a line_init_hooks bindings match mbegin mend reply
    zstyle -a zle-line-init widgets line_init_hooks
    [[ ${widgets[zle-line-init]:-} == user:azhw:zle-line-init ]] || return 0
    local hook keymap hook_function hook_body
    local -i has_input_hook=0
    for hook in "${line_init_hooks[@]}"; do
        hook=${hook#<->:}
        hook_function=${widgets[$hook]#user:}
        if [[ $hook_function == starship_zle-line-init ]]; then
            has_input_hook=1
            continue
        fi
        if [[ $hook_function == _ghostty_zle_line_init ]]; then
            local expected_hook original_call=''
            hook_body=${functions[$hook_function]}
            if [[ -n ${widgets[._ghostty_orig_zle-line-init]:-} ]]; then
                if [[ ${widgets[._ghostty_orig_zle-line-init]} == user:* ]]; then
                    original_call=$'\n builtin zle ._ghostty_orig_zle-line-init -N -- "$@"'
                else
                    original_call=$'\n builtin zle ._ghostty_orig_zle-line-init -Nw -- "$@"'
                fi
            fi
            __starship_ghostty_reference() {
                if [[ $PS1 != *$'%{\e]133;A'* ]]; then
                    builtin print -nu $_ghostty_fd '\e]133;P;k=i\a\e]133;B\a'
                fi
                builtin local steady=0
                [[ "$GHOSTTY_SHELL_FEATURES" == *"cursor:steady"* ]] && steady=1
                case ${KEYMAP-} in
                    vicmd|visual) builtin print -nu "$_ghostty_fd" "\e[$(( 1 + steady )) q" ;;
                    *) builtin print -nu "$_ghostty_fd" "\e[$(( 5 + steady )) q" ;;
                esac
            }
            functions[__starship_ghostty_reference]+=$original_call
            expected_hook=${functions[__starship_ghostty_reference]}
            unfunction __starship_ghostty_reference
            if [[ $hook_body != "$expected_hook" ]]; then
                __starship_ghostty_reference() {
                    if [[ $PS1 != *$'%{\e]133;A'* ]]; then
                        builtin print -nu $_ghostty_fd '\e]133;P;k=i\a\e]133;B\a'
                    fi
                    builtin true
                }
                functions[__starship_ghostty_reference]+=$original_call
                expected_hook=${functions[__starship_ghostty_reference]}
                unfunction __starship_ghostty_reference
                [[ $hook_body == "$expected_hook" ]] || return 0
            fi
            hook_function=${widgets[._ghostty_orig_zle-line-init]:-}
            [[ -n $hook_function ]] || continue
            hook_function=${hook_function#user:}
            [[ $hook_function != starship_zle-line-init ]] || continue
        fi
        hook_body=${functions[$hook_function]:-}
        hook_body=${${hook_body##[[:space:]]#}%%[[:space:]]#}
        [[ $hook_body == ':' || $hook_body == 'return 0' || $hook_body == 'echoti smkx' ]] || return 0
    done
    (( has_input_hook )) || return 0
    for keymap in main viins vicmd; do
        bindings=("${(@f)$(bindkey -M "$keymap" $'\e'; bindkey -M "$keymap" $'\e['; bindkey -M "$keymap" $'\e[?'; bindkey -M "$keymap" -p $'\e[?')}")
        [[ ${#bindings} == 3 && $bindings[2] == *' undefined-key' && $bindings[3] == *' starship_zle-cursor-report' ]] || return 0
        [[ $bindings[1] == *' undefined-key' ||
           ( $bindings[1] == *' vi-cmd-mode' && ${widgets[vi-cmd-mode]:-} == builtin ) ||
           ( $bindings[1] == *' beep' && ${widgets[beep]:-} == builtin ) ]] || return 0
    done
    zmodload zsh/datetime zsh/system zsh/zselect 2>/dev/null || return 0

    local tty_state fd response='' char
    local -F initial_timeout=0.2 report_timeout=1 deadline
    tty_state=$(command stty -g < "$TTY" 2>/dev/null) || return 0
    exec {fd}<> "$TTY" || return 0
    {
        command stty -echo -icanon min 1 time 0 <&$fd || return 0
        zselect -t 0 -r $fd && return 0
        deadline=$(( EPOCHREALTIME + initial_timeout ))
        # Private CPR replies cannot be mistaken for modified function keys.
        print -rn -- $'\e[?6n' >&$fd
        while (( EPOCHREALTIME < deadline )); do
            sysread -i $fd -s 1 -t "$(( deadline - EPOCHREALTIME ))" char || break
            response+=$char
            [[ $response != $'\e[?'[1-9] ]] || deadline=$(( EPOCHREALTIME + report_timeout ))
            if [[ $response == (#b)$'\e[?'(<1->)\;(<1->)(\;<1->|)R ]]; then
                STARSHIP_TERMINAL_ROW=$match[1]
                STARSHIP_TERMINAL_COLUMN=$match[2]
                return 0
            fi
            case $response in
                $'\e'|$'\e['|$'\e[?'|$'\e[?'<1->|$'\e[?'<1->\;|$'\e[?'<1->\;<1->|$'\e[?'<1->\;<1->\;|$'\e[?'<1->\;<1->\;<1->) ;;
                *) break ;;
            esac
        done

        if [[ $response != $'\e[?'[1-9][0-9\;]# ]]; then
            [[ ${response%?} != $'\e[?'[1-9][0-9\;]# ]] || response=$char
            STARSHIP_PENDING_INPUT+=$response
        fi
        [[ -n $response ]] || STARSHIP_CURSOR_QUERY_DISABLED=1
    } always {
        command stty "$tty_state" <&$fd
        exec {fd}<&-
    }
    return 0
}

__starship_update_prompt() {
    emulate -L zsh -o extendedglob
    STARSHIP_PROMPT='' STARSHIP_ADD_NEWLINE=0
    unset STARSHIP_RPROMPT
    local output
    output=$(::STARSHIP:: prompt --zsh --terminal-width="$COLUMNS" --keymap="${KEYMAP:-}" --status="${STARSHIP_CMD_STATUS:-}" --pipestatus="${STARSHIP_PIPE_STATUS[*]:-}" --cmd-duration="${STARSHIP_DURATION:-}" --jobs="$STARSHIP_JOBS_COUNT") || return 0
    STARSHIP_ADD_NEWLINE=${output[1]}
    STARSHIP_PROMPT=${output[2,-1]}
    if [[ $PROMPT == (%\{[^%\$\`]#%\})#"$STARSHIP_PROMPT_TEMPLATE"(%\{[^%\$\`]#%\})# &&
          $RPROMPT == (%\{[^%\$\`]#%\})#"$STARSHIP_RPROMPT_TEMPLATE"(%\{[^%\$\`]#%\})# ]]; then
        STARSHIP_RPROMPT=$(::STARSHIP:: prompt --right --terminal-width="$COLUMNS" --keymap="${KEYMAP:-}" --status="${STARSHIP_CMD_STATUS:-}" --pipestatus="${STARSHIP_PIPE_STATUS[*]:-}" --cmd-duration="${STARSHIP_DURATION:-}" --jobs="$STARSHIP_JOBS_COUNT")
        if [[ ${STARSHIP_NEEDS_CURSOR_POSITION:-0} == 1 ]]; then
            unset STARSHIP_NEEDS_CURSOR_POSITION
            __starship_get_terminal_position
        fi
    else
        unset STARSHIP_NEEDS_CURSOR_POSITION STARSHIP_TERMINAL_ROW STARSHIP_TERMINAL_COLUMN
    fi
    if [[ $STARSHIP_ADD_NEWLINE == 1 && ${STARSHIP_TERMINAL_ROW:-0} == 1 && ${STARSHIP_TERMINAL_COLUMN:-0} == 1 ]]; then
        STARSHIP_PROMPT=${STARSHIP_PROMPT#$'\n'}
    fi
    return 0
}

starship_zle-line-init() {
    if [[ -n ${STARSHIP_PENDING_INPUT:-} ]]; then
        zle -U -- "$STARSHIP_PENDING_INPUT"
        unset STARSHIP_PENDING_INPUT
    fi
}

starship_zle-clear-screen() {
    STARSHIP_TERMINAL_ROW=1
    STARSHIP_TERMINAL_COLUMN=1
    zle starship-clear-screen-original "$@"
}

starship_zle-cursor-report() {
    emulate -L zsh
    setopt extendedglob
    local LC_ALL=C
    local response=$KEYS char
    local -F initial_timeout=0.2 report_timeout=1 deadline
    deadline=$(( EPOCHREALTIME + initial_timeout ))
    while (( EPOCHREALTIME < deadline )); do
        IFS= read -r -s -k 1 -t "$(( deadline - EPOCHREALTIME ))" char || break
        response+=$char
        [[ $response != $'\e[?'[1-9] ]] || deadline=$(( EPOCHREALTIME + report_timeout ))
        [[ $char == [@-~] || $char != [[:print:]] ]] && break
    done
    if [[ $response != $'\e[?'<1->\;<1->(\;<1->|)R && $response != $'\e[?'[1-9][0-9\;]# ]]; then
        [[ ${response%?} != $'\e[?'[1-9][0-9\;]# ]] || response=$char
        zle -U -- "${response#$'\e[?'}"
    fi
}

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
    unset STARSHIP_RPROMPT
    STARSHIP_NEEDS_CURSOR_POSITION=1
}

# Runs after the user submits the command line, but before it is executed and
# only if there's an actual command to run
prompt_starship_preexec() {
    __starship_get_time && STARSHIP_START_TIME=$STARSHIP_CAPTURED_TIME
}

# Add hook functions
autoload -Uz add-zsh-hook
add-zsh-hook precmd prompt_starship_precmd
add-zsh-hook preexec prompt_starship_preexec

if [[ -z ${widgets[starship-clear-screen-original]:-} ]]; then
    zle -A clear-screen starship-clear-screen-original
    zle -N clear-screen starship_zle-clear-screen
fi

autoload -Uz add-zle-hook-widget
add-zle-hook-widget line-init starship_zle-line-init

zle -N starship_zle-cursor-report
for _starship_keymap in emacs viins vicmd; do
    if [[ $(bindkey -M "$_starship_keymap" $'\e[?') == *' undefined-key' ]]; then
        bindkey -M "$_starship_keymap" $'\e[?' starship_zle-cursor-report
    fi
done
unset _starship_keymap

# Set up a function to redraw the prompt if the user switches vi modes
starship_zle-keymap-select() {
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

export STARSHIP_SHELL="zsh"

# Set up the session key that will be used to store logs
STARSHIP_SESSION_KEY="$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM"; # Random generates a number b/w 0 - 32767
STARSHIP_SESSION_KEY="${STARSHIP_SESSION_KEY}0000000000000000" # Pad it to 16+ chars.
export STARSHIP_SESSION_KEY=${STARSHIP_SESSION_KEY:0:16}; # Trim to 16-digits if excess.

VIRTUAL_ENV_DISABLE_PROMPT=1

setopt promptsubst

functions -M __starship_update_prompt 0 0
# Run in the parent after precmd hooks; redraws reuse the captured cursor position.
STARSHIP_PROMPT_TEMPLATE='${${:-$(( __starship_update_prompt() ))}:#*}${STARSHIP_PROMPT}'
STARSHIP_RPROMPT_TEMPLATE='${STARSHIP_RPROMPT-$('::STARSHIP::' prompt --right --terminal-width="$COLUMNS" --keymap="${KEYMAP:-}" --status="${STARSHIP_CMD_STATUS:-}" --pipestatus="${STARSHIP_PIPE_STATUS[*]:-}" --cmd-duration="${STARSHIP_DURATION:-}" --jobs="$STARSHIP_JOBS_COUNT")}'
PROMPT=$STARSHIP_PROMPT_TEMPLATE
RPROMPT=$STARSHIP_RPROMPT_TEMPLATE
PROMPT2="$(::STARSHIP:: prompt --continuation)"
