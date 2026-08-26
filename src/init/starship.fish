function __starship_set_job_count --description 'Set STARSHIP_JOBS using fish job groups (or legacy PIDs if toggled)'
    # To force legacy behavior (process PIDs), set this variable to "false":
    #   set -g __starship_fish_use_job_groups "false"
    set -l mode -g
    test "$__starship_fish_use_job_groups" = false; and set mode -p
    set -g STARSHIP_JOBS (jobs $mode 2>/dev/null | count)
end

functions -q __starship_stream_cleanup; and __starship_stream_cleanup

set -g __starship_stream_prompt '' ''
set -g __starship_stream_timings '' ''
set -g __starship_stream_active 0 0
set -g __starship_stream_worker '' ''
set -g __starship_stream_renderer '' ''
set -g __starship_stream_state \
    __starship_stream_state_$fish_pid \
    __starship_stream_right_state_$fish_pid
set -g __starship_stream_directory (mktemp -d 2>/dev/null)
set -g __starship_transient 0 0
set -g __starship_prompt_arguments

if test -n "$__starship_stream_directory"
    command mkfifo "$__starship_stream_directory/1" "$__starship_stream_directory/2" 2>/dev/null; or begin
        command rm -r -- "$__starship_stream_directory" 2>/dev/null
        set -g __starship_stream_directory
    end
end

for state in $__starship_stream_state
    set -e -U $state 2>/dev/null
end

# Each side owns a universal-variable snapshot. Fish coalesces variable events,
# so every publication carries the complete [event prompt renderer timings].
function __starship_stream_apply --argument-names side
    test "$__starship_stream_active[$side]" = 1; or return
    set -l state_name $__starship_stream_state[$side]
    set -l state $$state_name
    test (count $state) -ge 4; or return

    set -g __starship_stream_timings[$side] $state[4]
    contains -- $state[1] READY PATCH REPAIR; or return
    set -g __starship_stream_prompt[$side] $state[2]
    contains -- 1 $__starship_transient; or commandline -f repaint
end

function __starship_stream_apply_left --on-variable $__starship_stream_state[1]
    __starship_stream_apply 1
end

function __starship_stream_apply_right --on-variable $__starship_stream_state[2]
    __starship_stream_apply 2
end

function __starship_stream_stop --argument-names side
    for process in $__starship_stream_worker[$side] $__starship_stream_renderer[$side]
        command kill $process 2>/dev/null
    end
    set -g __starship_stream_worker[$side] ''
    set -g __starship_stream_renderer[$side] ''
    set -g __starship_stream_active[$side] 0
end

function __starship_stream_stop_all
    __starship_stream_stop 1
    __starship_stream_stop 2
end

function __starship_stream_cleanup --on-event fish_exit
    __starship_stream_stop_all
    for state in $__starship_stream_state
        set -e -U $state 2>/dev/null
    end
    test -z "$__starship_stream_directory"; or command rm -r -- "$__starship_stream_directory" 2>/dev/null
end

function __starship_stream_sync --argument-names side
    set -l prompt
    ::STARSHIP:: prompt $argv[2..] | read -z prompt
    set -g __starship_stream_prompt[$side] "$prompt"
end

function __starship_stream_start --argument-names side
    __starship_stream_stop $side
    set -g __starship_stream_active[$side] 1

    set -l state_name $__starship_stream_state[$side]
    set -l previous $$state_name
    test (count $previous) -ge 4; and set -g __starship_stream_timings[$side] $previous[4]
    set -e -U $state_name 2>/dev/null

    if test -z "$__starship_stream_directory"
        __starship_stream_sync $side $argv[2..]
        return
    end

    set -l ready "$__starship_stream_directory/$side"
    # `< /dev/null`: this renderer is backgrounded and outlives the prompt
    # that launched it, but nothing here ever reads stdin. Left inherited,
    # it would be the shell's own pty, and a renderer still alive when the
    # shell exits would keep that pty from ever reporting end-of-file.
    ::STARSHIP:: stream $argv[2..] --timings="$__starship_stream_timings[$side]" < /dev/null | $__fish_bin_dir/fish -c '
        set -l state $argv[1]
        set -l ready $argv[2]
        set -e argv[1..2]
        set -l prompt ""
        set -l process ""
        set -l timings ""
        set -l woke 0
        set -l done 0

        while read -z kind; and read -z first; and read -z second
            switch $kind
                case READY
                    set prompt "$first"
                    set process "$second"
                    set woke 1
                case PATCH
                    set prompt "$first"
                case COMPLETE
                    set timings "$first"
                    set done 1
                case "*"
                    continue
            end
            set -U $state $kind "$prompt" "$process" "$timings"
            test "$kind" = READY; and printf "%s\0%s\0%s\0" "$kind" "$prompt" "$process" > "$ready"
        end

        if test $done -eq 0
            ::STARSHIP:: prompt $argv | read -z prompt
            set -U $state REPAIR "$prompt" "$process" "$timings"
            test $woke -eq 1; or printf "REPAIR\0%s\0%s\0" "$prompt" "$process" > "$ready"
        end
    ' -- $state_name "$ready" $argv[2..] &
    set -g __starship_stream_worker[$side] $last_pid

    set -l kind prompt renderer
    begin
        read -z kind
        read -z prompt
        read -z renderer
    end < "$ready"

    if contains -- "$kind" READY REPAIR
        set -g __starship_stream_prompt[$side] "$prompt"
        set -g __starship_stream_renderer[$side] "$renderer"
        return
    end

    __starship_stream_stop $side
    set -g __starship_stream_active[$side] 1
    __starship_stream_sync $side $argv[2..]
end

function __starship_stream_preexec --on-event fish_preexec
    __starship_stream_stop_all
    set -g __starship_stream_prompt '' ''
    set -g __starship_prompt_arguments
end

function __starship_prompt --argument-names side
    set -l command_pipestatus $pipestatus
    set -l command_status $status
    if not set -q __starship_prompt_arguments[1]
        set -l keymap insert
        contains -- "$fish_key_bindings" fish_hybrid_key_bindings fish_vi_key_bindings fish_helix_key_bindings; and set keymap "$fish_bind_mode"
        __starship_set_job_count
        set -g __starship_prompt_arguments \
            --terminal-width="$COLUMNS" \
            --status="$command_status" \
            --pipestatus="$command_pipestatus" \
            --keymap="$keymap" \
            --cmd-duration="$CMD_DURATION$cmd_duration" \
            --jobs="$STARSHIP_JOBS"
    end

    set -l arguments $__starship_prompt_arguments
    test $side -eq 2; and set -p arguments --right

    if contains -- --final-rendering $argv; or test "$__starship_transient[$side]" = 1
        if test "$__starship_transient[$side]" = 1
            set -g __starship_transient[$side] 0
            test $side -eq 1; and printf \e\[0J
        end

        set -l transient_functions starship_transient_prompt_func starship_transient_rprompt_func
        if functions -q $transient_functions[$side]
            $transient_functions[$side] $arguments
        else if test $side -eq 1
            printf "\e[1;32m❯\e[0m "
        end
        return
    end

    test "$__starship_stream_active[$side]" = 1; or __starship_stream_start $side $arguments
    printf %s "$__starship_stream_prompt[$side]"
end

function fish_prompt
    __starship_prompt 1 $argv
end

function fish_right_prompt
    __starship_prompt 2 $argv
end

# Disable virtualenv prompt, it breaks starship
set -g VIRTUAL_ENV_DISABLE_PROMPT 1

# Remove default mode prompt
builtin functions -e fish_mode_prompt

set -gx STARSHIP_SHELL "fish"

# Transience related functions
function __starship_reset_transient --on-event fish_postexec
    set -g __starship_transient 0 0
end

function __starship_transient_execute
    if commandline --is-valid || test -z (commandline | string collect) && not commandline --paging-mode
        set -g __starship_transient 1 1
        commandline -f repaint
    end
    commandline -f execute
end

function __starship_fish_version_at_least --description 'Check if fish version is at least the given version'
    set -l current (string split . $FISH_VERSION)
    set -l required (string split . $argv[1])
    test $current[1] -gt $required[1]; or \
        test $current[1] -eq $required[1] -a $current[2] -ge $required[2]
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
