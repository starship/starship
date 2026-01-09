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
        ::STARSHIP:: prompt --terminal-width="$COLUMNS" --status=$STARSHIP_CMD_STATUS --pipestatus="$STARSHIP_CMD_PIPESTATUS" --keymap=$STARSHIP_KEYMAP --cmd-duration=$STARSHIP_DURATION --jobs=$STARSHIP_JOBS
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
