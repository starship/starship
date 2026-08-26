## Re-sourcing wipes any earlier session's streams and temp files.
functions -q __starship_stream_cleanup; and __starship_stream_cleanup

set -g __starship_stream_prompt '' ''
set -g __starship_stream_worker ''
set -g __starship_transient 0 0
set -g __starship_prompt_arguments

# Pid-keyed dir holding one file, because one renderer draws both sides. Not a
# fifo: fish `exec` replaces the shell (no held pipe fd) and `read` has no
# timeout, so a fifo cannot be drained after coalesced SIGUSR1. Atomic rename +
# `left\0right\0timings\0` never blocks or tears; COMPLETE timings ride as the
# last field of the same record so a later paint cannot wipe them.
set -g __starship_d /tmp/starship-$fish_pid
set -q TMPDIR; and set -g __starship_d $TMPDIR/starship-$fish_pid
test -d $__starship_d; or command mkdir -p -- $__starship_d
set -g __starship_state $__starship_d/prompt

function __starship_stream_update --on-signal USR1
    test -n "$__starship_stream_worker"; and test -f $__starship_state; or return
    # Both sides come out of the one record. Their statuses are not checked: an
    # empty side is an ordinary reading, and the rename that published this made
    # all three fields current together.
    begin
        read -z left
        read -z right
    end <$__starship_state
    set -g __starship_stream_prompt[1] "$left"
    set -g __starship_stream_prompt[2] "$right"
    test -n "$__starship_in_prompt"; or contains -- 1 $__starship_transient; or commandline -f repaint
end

# One renderer, so one pid to stop — and `kill` not being a fish builtin, one
# fork to stop it instead of the two this used to cost.
function __starship_stream_stop
    set -l worker $__starship_stream_worker
    set -g __starship_stream_worker ''
    test -n "$worker"; and test "$worker" != sync; and command kill $worker 2>/dev/null
end

function __starship_stream_cleanup --on-event fish_exit
    __starship_stream_stop
    command rm -r -- $__starship_d
end

function __starship_stream_start
    __starship_stream_stop
    # The record is `left\0right\0timings\0`; only the timings are wanted here,
    # but `read` takes one NUL field per call, so the two leading fields go to a
    # throwaway to reach the third. Not `read -z _`: `$_` is read-only in fish,
    # so that errored every prompt the file survived — which, before the snapshot
    # outlived a draw, was never.
    set -l timings discarded
    test -f $__starship_state; and begin
        read -z discarded
        read -z discarded
        read -z timings
    end <$__starship_state
    # `--detach` forks the renderer into the background before it does any work
    # and hands its first paint and its pid back up this pipeline, so this read
    # *is* the handshake: it blocks for exactly as long as that paint takes.
    #
    # fish cannot do better than poll otherwise — it holds no pipe across a
    # prompt and its `read` has no timeout — and polling cost a `sleep` fork
    # plus a whole tick of latency for a paint that lands in single-digit
    # milliseconds. Nothing here clears the snapshot first either: the first
    # paint arrives on the pipe, not out of the file, so a stale one cannot be
    # mistaken for it.
    ::STARSHIP:: stream --both --detach --publish-state=$__starship_state \
        --signal-pid=$fish_pid --timings="$timings" $argv 2>/dev/null | begin
        read -z left
        read -z right
        read -z worker
    end
    if test -n "$worker"
        set -g __starship_stream_prompt[1] "$left"
        set -g __starship_stream_prompt[2] "$right"
        set -g __starship_stream_worker $worker
        return
    end
    # No first paint: render both sides synchronously and mark the stream served
    # so this draw does not retry.
    ::STARSHIP:: prompt $argv | read -z left
    ::STARSHIP:: prompt --right $argv | read -z right
    set -g __starship_stream_prompt[1] "$left"
    set -g __starship_stream_prompt[2] "$right"
    set -g __starship_stream_worker sync
end

function __starship_stream_preexec --on-event fish_preexec
    __starship_stream_stop
    set -g __starship_stream_prompt '' ''
    set -g __starship_prompt_arguments
end

function __starship_prompt --a side
    set -g __starship_in_prompt 1
    set -l command_pipestatus $pipestatus
    set -l command_status $status
    if not set -q __starship_prompt_arguments[1]
        set -l keymap insert
        contains -- "$fish_key_bindings" fish_hybrid_key_bindings fish_vi_key_bindings fish_helix_key_bindings; and set keymap "$fish_bind_mode"
        # Job groups by default; force legacy PIDs:
        #   set -g __starship_fish_use_job_groups "false"
        set -l mode -g
        test "$__starship_fish_use_job_groups" = false; and set mode -p
        set -g __starship_prompt_arguments --terminal-width="$COLUMNS" --status="$command_status" \
            --pipestatus="$command_pipestatus" --keymap="$keymap" --cmd-duration="$CMD_DURATION$cmd_duration" \
            --jobs=(count (jobs $mode 2>/dev/null))
    end
    set -l arguments $__starship_prompt_arguments
    if contains -- --final-rendering $argv; or test "$__starship_transient[$side]" = 1
        if test "$__starship_transient[$side]" = 1
            set -g __starship_transient[$side] 0
            test $side -eq 1; and printf \e\[0J
        end
        set -l transients starship_transient_prompt_func starship_transient_rprompt_func
        if functions -q $transients[$side]
            $transients[$side] $arguments
        else if test $side -eq 1
            printf "\e[1;32m❯\e[0m "
        end
        set -e __starship_in_prompt
        return
    end
    test -n "$__starship_stream_worker"; or __starship_stream_start $arguments
    printf %s "$__starship_stream_prompt[$side]"
    set -e __starship_in_prompt
end

function fish_prompt; __starship_prompt 1 $argv; end
function fish_right_prompt; __starship_prompt 2 $argv; end

set -g VIRTUAL_ENV_DISABLE_PROMPT 1
builtin functions -e fish_mode_prompt
set -gx STARSHIP_SHELL fish

function __starship_reset_transient --on-event fish_postexec
    set -g __starship_transient 0 0
end

# fish >= 4.1 uses the `fish_transient_prompt` builtin; older fish binds Enter.
function __starship_transient_execute
    if commandline --is-valid; and not commandline --paging-mode
        set -g __starship_transient 1 1
        commandline -f repaint
    end
    commandline -f execute
end

set -l v (string split . $FISH_VERSION)
set -g __starship_transient_builtin (test $v[1] -gt 4; or test $v[1] -eq 4 -a $v[2] -ge 1; and echo 1)

function enable_transience --description 'enable transient prompt keybindings'
    test -n "$__starship_transient_builtin"; and set -g fish_transient_prompt 1; and return
    bind --user \r __starship_transient_execute
    bind --user -M insert \r __starship_transient_execute
end

function disable_transience --description 'remove transient prompt keybindings'
    test -n "$__starship_transient_builtin"; and set -g fish_transient_prompt 0; and return
    bind --user -e \r
    bind --user -M insert -e \r
end

set -gx STARSHIP_SESSION_KEY (random 1000000000000000 9999999999999999)
