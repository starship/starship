function fish_prompt
    switch "$fish_key_bindings"
        case fish_hybrid_key_bindings fish_vi_key_bindings
            set STARSHIP_KEYMAP "$fish_bind_mode"
        case '*'
            set STARSHIP_KEYMAP insert
    end
    set STARSHIP_CMD_PIPESTATUS $pipestatus
    set STARSHIP_CMD_STATUS $status
    # Account for changes in variable name between v2.7 and v3.0
    set STARSHIP_DURATION "$CMD_DURATION$cmd_duration"
    set STARSHIP_JOBS (count (jobs -p))
    if test "$TRANSIENT" = "1"
        # Clear from cursor to end of screen as `commandline -f repaint` does not do this
        # See https://github.com/fish-shell/fish-shell/issues/8418
        printf \e\[0J
        if type -q starship_transient_prompt_func
            starship_transient_prompt_func
        else
            printf "\e[1;32m❯\e[0m "
        end
    else
        ::STARSHIP:: prompt $STARSHIP_ADD_NEWLINE_FLAGS --terminal-width="$COLUMNS" --status=$STARSHIP_CMD_STATUS --pipestatus="$STARSHIP_CMD_PIPESTATUS" --keymap=$STARSHIP_KEYMAP --cmd-duration=$STARSHIP_DURATION --jobs=$STARSHIP_JOBS
    end
    set -g STARSHIP_ADD_NEWLINE_FLAGS
end

function fish_right_prompt
    switch "$fish_key_bindings"
        case fish_hybrid_key_bindings fish_vi_key_bindings
            set STARSHIP_KEYMAP "$fish_bind_mode"
        case '*'
            set STARSHIP_KEYMAP insert
    end
    set STARSHIP_CMD_PIPESTATUS $pipestatus
    set STARSHIP_CMD_STATUS $status
    # Account for changes in variable name between v2.7 and v3.0
    set STARSHIP_DURATION "$CMD_DURATION$cmd_duration"
    set STARSHIP_JOBS (count (jobs -p))
    if test "$TRANSIENT" = "1"
        if type -q starship_transient_rprompt_func
            starship_transient_rprompt_func
        else
            printf ""
        end
    else
        ::STARSHIP:: prompt $STARSHIP_ADD_NEWLINE_FLAGS --right --terminal-width="$COLUMNS" --status=$STARSHIP_CMD_STATUS --pipestatus="$STARSHIP_CMD_PIPESTATUS" --keymap=$STARSHIP_KEYMAP --cmd-duration=$STARSHIP_DURATION --jobs=$STARSHIP_JOBS
    end
    set -g STARSHIP_ADD_NEWLINE_FLAGS
end

# Disable virtualenv prompt, it breaks starship
set -g VIRTUAL_ENV_DISABLE_PROMPT 1

# Remove default mode prompt
builtin functions -e fish_mode_prompt

set -gx STARSHIP_SHELL "fish"
set -gx STARSHIP_ADD_NEWLINE_FLAGS --disable-add-newline

# Transience related functions
function reset-transient --on-event fish_postexec
    set -g TRANSIENT 0
end

function transient_execute
    if commandline --is-valid
        set -g TRANSIENT 1
        commandline -f repaint
    else
        set -g TRANSIENT 0
    end
    commandline -f execute
end

# --user is the default, but listed anyway to make it explicit.
function enable_transience --description 'enable transient prompt keybindings'
    bind --user \r transient_execute
    bind --user -M insert \r transient_execute
end

# Erase the transient prompt related key bindings.
# --user is the default, but listed anyway to make it explicit.
# Erasing a user binding will revert to the preset.
function disable_transience --description 'remove transient prompt keybindings'
    bind --user -e \r
    bind --user -M insert -e \r
end

# On screen clear (ctrl+l), do not draw a newline
function starship_clear
    set -g STARSHIP_ADD_NEWLINE_FLAGS --disable-add-newline
end

# Hook into the clear screen function
# if the user has already defined a `clear` function, wrap that definition
if functions -q clear
    functions -c clear __original_clear
end

function clear
    starship_clear
    if functions -q __original_clear
        __original_clear
    else
        command clear
    end
end

# Set up the session key that will be used to store logs
# We don't use `random [min] [max]` because it is unavailable in older versions of fish shell
set -gx STARSHIP_SESSION_KEY (string sub -s1 -l16 (random)(random)(random)(random)(random)0000000000000000)
