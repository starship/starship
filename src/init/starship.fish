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
set -g STARSHIP_RIGHT_PROMPT ''
set -g STARSHIP_TIMINGS ''
set -g STARSHIP_RIGHT_TIMINGS ''
set -g __starship_stream_active_for_current_line 0
set -g __starship_stream_right_active_for_current_line 0
set -g __starship_stream_processes
set -g __starship_stream_right_processes
set -g __starship_stream_state_name __starship_stream_state_$fish_pid
set -g __starship_stream_right_state_name __starship_stream_right_state_$fish_pid
set -g __starship_stream_directory (mktemp -d 2>/dev/null)
set -e -U $__starship_stream_state_name 2>/dev/null
set -e -U $__starship_stream_right_state_name 2>/dev/null

# Variable events may coalesce, so publish complete snapshots. The left and
# right prompts each stream into their own universal variable; a redraw
# re-issues both `fish_prompt` and `fish_right_prompt`, so either side's patch
# repaints the whole prompt.
function __starship_stream_apply_target --argument-names state_name_variable active_variable prompt_variable timings_variable
    test $$active_variable -eq 1; or return
    set -l state_name $$state_name_variable
    set -l state $$state_name
    test (count $state) -ge 4; or return
    set -g $timings_variable $state[4]

    contains -- $state[1] READY PATCH REPAIR; or return
    set -g $prompt_variable $state[2]
    if test "$TRANSIENT" != 1; and test "$RIGHT_TRANSIENT" != 1
        commandline -f repaint
    end
end

function __starship_stream_apply --on-variable $__starship_stream_state_name
    __starship_stream_apply_target __starship_stream_state_name __starship_stream_active_for_current_line STARSHIP_PROMPT STARSHIP_TIMINGS
end

function __starship_stream_right_apply --on-variable $__starship_stream_right_state_name
    __starship_stream_apply_target __starship_stream_right_state_name __starship_stream_right_active_for_current_line STARSHIP_RIGHT_PROMPT STARSHIP_RIGHT_TIMINGS
end

function __starship_stream_stop --argument-names target
    set -l processes_variable __starship_stream_processes
    set -l active_variable __starship_stream_active_for_current_line
    if test "$target" = right
        set processes_variable __starship_stream_right_processes
        set active_variable __starship_stream_right_active_for_current_line
    end
    for process in $$processes_variable
        command kill $process 2>/dev/null
    end
    set -g $processes_variable
    set -g $active_variable 0
end

function __starship_stream_stop_all
    __starship_stream_stop main
    __starship_stream_stop right
end

function __starship_stream_cleanup --on-event fish_exit
    __starship_stream_stop_all
    set -e -U $__starship_stream_state_name 2>/dev/null
    set -e -U $__starship_stream_right_state_name 2>/dev/null
    if test -n "$__starship_stream_directory"
        command rm -rf -- "$__starship_stream_directory" 2>/dev/null
    end
end

# The `target` argument selects the left prompt (`main`) or the right prompt
# (`right`); the two differ only in which universal variable they publish into
# and whether `--right` is threaded through to Starship.
function __starship_stream_start --argument-names target terminal_width keymap command_status command_pipestatus command_duration jobs
    set -l state_name_variable __starship_stream_state_name
    set -l active_variable __starship_stream_active_for_current_line
    set -l processes_variable __starship_stream_processes
    set -l prompt_variable STARSHIP_PROMPT
    set -l timings_variable STARSHIP_TIMINGS
    set -l stream_target
    if test "$target" = right
        set state_name_variable __starship_stream_right_state_name
        set active_variable __starship_stream_right_active_for_current_line
        set processes_variable __starship_stream_right_processes
        set prompt_variable STARSHIP_RIGHT_PROMPT
        set timings_variable STARSHIP_RIGHT_TIMINGS
        set stream_target --right
    end

    __starship_stream_stop $target
    set -g $active_variable 1

    set -l state_name $$state_name_variable

    if set -q $state_name
        set -l previous $$state_name
        test (count $previous) -ge 4; and set -g $timings_variable $previous[4]
    end
    set -e -U $state_name 2>/dev/null

    set -l arguments \
        --terminal-width="$terminal_width" \
        --keymap="$keymap" \
        --status="$command_status" \
        --pipestatus="$command_pipestatus" \
        --cmd-duration="$command_duration" \
        --jobs="$jobs"

    set -l timings_value $$timings_variable

    if test -z "$__starship_stream_directory"
        ::STARSHIP:: prompt $stream_target $arguments | read -gz $prompt_variable
        return
    end

    set -l ready_fifo "$__starship_stream_directory/"(random)
    command mkfifo "$ready_fifo" 2>/dev/null; or begin
        ::STARSHIP:: prompt $stream_target $arguments | read -gz $prompt_variable
        return
    end

    # `< /dev/null`: this renderer is backgrounded and outlives the prompt
    # that launched it, but nothing here ever reads stdin. Left inherited,
    # it would be the shell's own pty, and a renderer still alive when the
    # shell exits would keep that pty from ever reporting end-of-file.
    ::STARSHIP:: stream $stream_target $arguments --timings="$timings_value" < /dev/null | $__fish_bin_dir/fish -c '
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
    ' -- $state_name "$ready_fifo" $stream_target $arguments &
    set -g $processes_variable (jobs --last --pid)

    read < "$ready_fifo"
    command rm -f -- "$ready_fifo" 2>/dev/null

    if set -q $state_name
        set -l state $$state_name
        if test (count $state) -ge 4
            set -g $prompt_variable $state[2]
            set -g $timings_variable $state[4]
            if test -n "$state[3]"; and not contains -- $state[3] $$processes_variable
                set -a $processes_variable $state[3]
            end
            return
        end
    end

    __starship_stream_stop $target
    set -g $active_variable 1
    ::STARSHIP:: prompt $stream_target $arguments | read -gz $prompt_variable
end

function __starship_stream_preexec --on-event fish_preexec
    __starship_stream_stop_all
    set -g STARSHIP_PROMPT ''
    set -g STARSHIP_RIGHT_PROMPT ''
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
            __starship_stream_start main "$COLUMNS" "$STARSHIP_KEYMAP" "$STARSHIP_CMD_STATUS" "$STARSHIP_CMD_PIPESTATUS" "$STARSHIP_DURATION" "$STARSHIP_JOBS"
        end
        printf '%s' "$STARSHIP_PROMPT"
    end
end

function fish_right_prompt
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
        if test "$__starship_stream_right_active_for_current_line" = 0
            __starship_stream_start right "$COLUMNS" "$STARSHIP_KEYMAP" "$STARSHIP_CMD_STATUS" "$STARSHIP_CMD_PIPESTATUS" "$STARSHIP_DURATION" "$STARSHIP_JOBS"
        end
        printf '%s' "$STARSHIP_RIGHT_PROMPT"
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
