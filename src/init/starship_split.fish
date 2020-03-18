function fish_prompt
    switch "$fish_key_bindings"
        case fish_hybrid_key_bindings fish_vi_key_bindings
            set keymap "$fish_bind_mode"
        case '*'
            set keymap insert
    end
    set -l exit_code $status
    # Account for changes in variable name between v2.7 and v3.0
    set -l starship_duration "$CMD_DURATION$cmd_duration"
    ::STARSHIP:: prompt --character-only --status=$exit_code --keymap=$keymap --cmd-duration=$starship_duration --jobs=(count (jobs -p))
end

function fish_right_prompt
    switch "$fish_key_bindings"
        case fish_hybrid_key_bindings fish_vi_key_bindings
            set keymap "$fish_bind_mode"
        case '*'
            set keymap insert
    end
    set -l exit_code $status
    # Account for changes in variable name between v2.7 and v3.0
    set -l starship_duration "$CMD_DURATION$cmd_duration"
    ::STARSHIP:: prompt --status=$exit_code --keymap=$keymap --cmd-duration=$starship_duration --jobs=(count (jobs -p))
end

# disable virtualenv prompt, it breaks starship
set VIRTUAL_ENV_DISABLE_PROMPT 1

function fish_mode_prompt; end
export STARSHIP_SHELL="fish"
