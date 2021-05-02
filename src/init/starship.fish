function fish_prompt
    switch $fish_key_bindings
        case fish_hybrid_key_bindings fish_vi_key_bindings
            set STARSHIP_KEYMAP $fish_bind_mode
        case '*'
            set STARSHIP_KEYMAP insert
    end
    set STARSHIP_CMD_STATUS $status
    # Account for changes in variable name between v2.7 and v3.0
    set STARSHIP_CMD_DURATION "$CMD_DURATION$cmd_duration"
    set STARSHIP_JOBS_COUNT (count (jobs -p))
    ::STARSHIP:: prompt \
        --keymap=$STARSHIP_KEYMAP \
        --status=$STARSHIP_CMD_STATUS \
        --cmd-duration=$STARSHIP_CMD_DURATION \
        --jobs=$STARSHIP_JOBS_COUNT
end

# Disable virtualenv prompt, it breaks starship
set -g VIRTUAL_ENV_DISABLE_PROMPT 1

# Remove default mode prompt
builtin functions -e fish_mode_prompt

set -gx STARSHIP_SHELL "fish"

# Set up the session key that will be used to store logs
set -gx STARSHIP_SESSION_KEY (random 10000000000000 9999999999999999)
