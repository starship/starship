function fish_prompt
    switch "$fish_key_bindings"
        case fish_hybrid_key_bindings fish_vi_key_bindings
            set keymap "$fish_bind_mode"
        case '*'
            set keymap insert
    end
    set exit_code $status
    # Account for changes in variable name between v2.7 and v3.0
    set starship_duration "$CMD_DURATION$cmd_duration"
    ::STARSHIP:: prompt --status=$exit_code --keymap=$keymap --cmd-duration=$starship_duration --jobs=(count (jobs -p))
end

# disable virtualenv prompt, it breaks starship
set -g VIRTUAL_ENV_DISABLE_PROMPT 1

function fish_mode_prompt; end
set -gx STARSHIP_SHELL "fish"

# Set up the session key that will be used to store logs
set -gx STARSHIP_SESSION_KEY (random 10000000000000 9999999999999999)
