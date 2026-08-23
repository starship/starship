function __starship_set_job_count --description 'Set STARSHIP_JOBS using fish job groups (or legacy PIDs if toggled)'
    # To force legacy behavior (process PIDs), set this variable to "false":
    #   set -g __starship_fish_use_job_groups "false"
    if test "$__starship_fish_use_job_groups" = "false"
        # Legacy behavior: counts PIDs (may overcount pipelines with terminated producers)
        set -g STARSHIP_JOBS (jobs -p 2>/dev/null | count)
    else
        # Default behavior: count job groups
        set -g STARSHIP_JOBS (jobs -g 2>/dev/null | count)
    end    
end

set -g STARSHIP_PROMPT ''
set -g STARSHIP_TIMINGS ''
set -g __starship_stream_active_for_current_line 0
set -g __starship_stream_processes
set -g __starship_stream_state_name __starship_stream_state_$fish_pid
set -g __starship_stream_directory (mktemp -d 2>/dev/null)
set -e -U $__starship_stream_state_name 2>/dev/null

# Variable events may coalesce, so publish complete snapshots.
function __starship_stream_apply --on-variable $__starship_stream_state_name
    test $__starship_stream_active_for_current_line -eq 1; or return
    set -l state $$__starship_stream_state_name
    test (count $state) -ge 4; or return
    set -g STARSHIP_TIMINGS $state[4]

    contains -- $state[1] READY PATCH REPAIR; or return
    set -g STARSHIP_PROMPT $state[2]
    if test "$TRANSIENT" != 1; and test "$RIGHT_TRANSIENT" != 1
        commandline -f repaint
    end
end

function __starship_stream_stop
    for process in $__starship_stream_processes
        command kill $process 2>/dev/null
    end
    set -g __starship_stream_processes
    set -g __starship_stream_active_for_current_line 0
end

function __starship_stream_cleanup --on-event fish_exit
    __starship_stream_stop
    set -e -U $__starship_stream_state_name 2>/dev/null
    if test -n "$__starship_stream_directory"
        command rm -rf -- "$__starship_stream_directory" 2>/dev/null
    end
end

function __starship_stream_start --argument-names terminal_width keymap command_status command_pipestatus command_duration jobs
    __starship_stream_stop
    set -g __starship_stream_active_for_current_line 1

    if set -q $__starship_stream_state_name
        set -l previous $$__starship_stream_state_name
        test (count $previous) -ge 4; and set -g STARSHIP_TIMINGS $previous[4]
    end
    set -e -U $__starship_stream_state_name 2>/dev/null

    set -l arguments \
        --terminal-width="$terminal_width" \
        --keymap="$keymap" \
        --status="$command_status" \
        --pipestatus="$command_pipestatus" \
        --cmd-duration="$command_duration" \
        --jobs="$jobs"

    if test -z "$__starship_stream_directory"
        ::STARSHIP:: prompt $arguments | read -gz STARSHIP_PROMPT
        return
    end

    set -l ready_fifo "$__starship_stream_directory/"(random)
    command mkfifo "$ready_fifo" 2>/dev/null; or begin
        ::STARSHIP:: prompt $arguments | read -gz STARSHIP_PROMPT
        return
    end

    ::STARSHIP:: stream $arguments --timings="$STARSHIP_TIMINGS" | $__fish_bin_dir/fish -c '
        set -l state_name $argv[1]
        set -l ready_fifo $argv[2]
        set -l arguments $argv[3..]
        set -l prompt ""
        set -l process ""
        set -l timings ""
        set -l ready 0
        set -l complete 0

        while read -z kind
            read -z first; and read -z second; or break

            switch $kind
                case READY
                    set prompt "$first"
                    set process "$second"
                    set -U $state_name READY "$prompt" "$process" "$timings"
                    echo READY > "$ready_fifo"
                    set ready 1
                case PATCH
                    set prompt "$first"
                    set -U $state_name PATCH "$prompt" "$process" "$timings"
                case COMPLETE
                    set timings "$first"
                    set complete 1
                    set -U $state_name COMPLETE "$prompt" "$process" "$timings"
            end
        end

        if test $complete -eq 0
            ::STARSHIP:: prompt $arguments | read -z prompt
            set -U $state_name REPAIR "$prompt" "$process" "$timings"
            test $ready -eq 1; or echo READY > "$ready_fifo"
        end
    ' -- $__starship_stream_state_name "$ready_fifo" $arguments &
    set -g __starship_stream_processes (jobs --last --pid)

    read < "$ready_fifo"
    command rm -f -- "$ready_fifo" 2>/dev/null

    if set -q $__starship_stream_state_name
        set -l state $$__starship_stream_state_name
        if test (count $state) -ge 4
            set -g STARSHIP_PROMPT $state[2]
            set -g STARSHIP_TIMINGS $state[4]
            if test -n "$state[3]"; and not contains -- $state[3] $__starship_stream_processes
                set -a __starship_stream_processes $state[3]
            end
            return
        end
    end

    __starship_stream_stop
    set -g __starship_stream_active_for_current_line 1
    ::STARSHIP:: prompt $arguments | read -gz STARSHIP_PROMPT
end

function __starship_stream_preexec --on-event fish_preexec
    __starship_stream_stop
    set -g STARSHIP_PROMPT ''
end

function fish_prompt
    switch "$fish_key_bindings"
        case fish_hybrid_key_bindings fish_vi_key_bindings fish_helix_key_bindings
            set STARSHIP_KEYMAP "$fish_bind_mode"
        case '*'
            set STARSHIP_KEYMAP insert
    end

    set STARSHIP_CMD_PIPESTATUS $pipestatus
    set STARSHIP_CMD_STATUS $status
    # Account for changes in variable name between v2.7 and v3.0
    set STARSHIP_DURATION "$CMD_DURATION$cmd_duration"

    __starship_set_job_count

    if contains -- --final-rendering $argv; or test "$TRANSIENT" = "1"
        if test "$TRANSIENT" = "1"
            set -g TRANSIENT 0
            # Clear from cursor to end of screen as `commandline -f repaint` does not do this
            # See https://github.com/fish-shell/fish-shell/issues/8418
            printf \e\[0J
        end
        if type -q starship_transient_prompt_func
            starship_transient_prompt_func --terminal-width="$COLUMNS" --status=$STARSHIP_CMD_STATUS --pipestatus="$STARSHIP_CMD_PIPESTATUS" --keymap=$STARSHIP_KEYMAP --cmd-duration=$STARSHIP_DURATION --jobs=$STARSHIP_JOBS
        else
            printf "\e[1;32m❯\e[0m "
        end
    else
        if test "$__starship_stream_active_for_current_line" = 0
            __starship_stream_start "$COLUMNS" "$STARSHIP_KEYMAP" "$STARSHIP_CMD_STATUS" "$STARSHIP_CMD_PIPESTATUS" "$STARSHIP_DURATION" "$STARSHIP_JOBS"
        end
        printf '%s' "$STARSHIP_PROMPT"
    end
end

function fish_right_prompt
    switch "$fish_key_bindings"
        case fish_hybrid_key_bindings fish_vi_key_bindings fish_helix_keybindings
            set STARSHIP_KEYMAP "$fish_bind_mode"
        case '*'
            set STARSHIP_KEYMAP insert
    end

    set STARSHIP_CMD_PIPESTATUS $pipestatus
    set STARSHIP_CMD_STATUS $status
    # Account for changes in variable name between v2.7 and v3.0
    set STARSHIP_DURATION "$CMD_DURATION$cmd_duration"

    # Now it's safe to call job count function (after status capture)
    __starship_set_job_count

    if contains -- --final-rendering $argv; or test "$RIGHT_TRANSIENT" = "1"
        set -g RIGHT_TRANSIENT 0
        if type -q starship_transient_rprompt_func
            starship_transient_rprompt_func --terminal-width="$COLUMNS" --status=$STARSHIP_CMD_STATUS --pipestatus="$STARSHIP_CMD_PIPESTATUS" --keymap=$STARSHIP_KEYMAP --cmd-duration=$STARSHIP_DURATION --jobs=$STARSHIP_JOBS
        else
            printf ""
        end
    else
        ::STARSHIP:: prompt --right --terminal-width="$COLUMNS" --status=$STARSHIP_CMD_STATUS --pipestatus="$STARSHIP_CMD_PIPESTATUS" --keymap=$STARSHIP_KEYMAP --cmd-duration=$STARSHIP_DURATION --jobs=$STARSHIP_JOBS
    end
end

# Disable virtualenv prompt, it breaks starship
set -g VIRTUAL_ENV_DISABLE_PROMPT 1

# Remove default mode prompt
builtin functions -e fish_mode_prompt

set -gx STARSHIP_SHELL "fish"

# Transience related functions
function __starship_reset_transient --on-event fish_postexec
    set -g TRANSIENT 0
    set -g RIGHT_TRANSIENT 0
end

function __starship_transient_execute
    if commandline --is-valid || test -z (commandline | string collect) && not commandline --paging-mode
        set -g TRANSIENT 1
        set -g RIGHT_TRANSIENT 1
        commandline -f repaint
    end
    commandline -f execute
end

function __starship_fish_version_at_least --description 'Check if fish version is at least the given version'
    set -l parts (string split '.' $FISH_VERSION)
    set -l major $parts[1]
    set -l minor 0
    if set -q parts[2]
        set minor $parts[2]
    end

    set req_parts (string split '.' $argv[1])
    set req_major $req_parts[1]
    set req_minor 0
    if set -q req_parts[2]
        set req_minor $req_parts[2]
    end

    if test $major -gt $req_major
        return 0
    else if test $major -eq $req_major -a $minor -ge $req_minor
        return 0
    else
        return 1
    end
end

# --user is the default, but listed anyway to make it explicit.
function enable_transience --description 'enable transient prompt keybindings'
    # fish >= 4.1 has transient prompt support built
    if __starship_fish_version_at_least 4.1
        set -g fish_transient_prompt 1
        return
    end
    bind --user \r __starship_transient_execute
    bind --user -M insert \r __starship_transient_execute
end

# Erase the transient prompt related key bindings.
# --user is the default, but listed anyway to make it explicit.
# Erasing a user binding will revert to the preset.
function disable_transience --description 'remove transient prompt keybindings'
    # fish >= 4.1 has transient prompt support built
    if __starship_fish_version_at_least 4.1
        set -g fish_transient_prompt 0
        return
    end
    bind --user -e \r
    bind --user -M insert -e \r
end

# Set up the session key that will be used to store logs
# We don't use `random [min] [max]` because it is unavailable in older versions of fish shell
set -gx STARSHIP_SESSION_KEY (string sub -s1 -l16 (random)(random)(random)(random)(random)0000000000000000)
