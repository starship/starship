function fish_prompt
    switch "$fish_key_bindings"
        case fish_hybrid_key_bindings fish_vi_key_bindings
            set keymap "$fish_bind_mode"
        case '*'
            set keymap insert
    end
    set -l exit_code $status
    # Account for changes in variable name between v2.7 and v3.0
    set -l CMD_DURATION "$CMD_DURATION$cmd_duration"
    set -l starship_duration (math --scale=0 "$CMD_DURATION / 1000")
    ## STARSHIP ## prompt --status=$exit_code --keymap=$keymap --cmd-duration=$starship_duration --jobs=(count (jobs -p))
end
function fish_mode_prompt; end
export STARSHIP_SHELL="fish"
