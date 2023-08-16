set-env STARSHIP_SHELL "elvish"
set-env STARSHIP_SESSION_KEY (to-string (randint 10000000000000 10000000000000000))

# Define Hooks
var cmd-status-code = 0

fn starship-after-command-hook {|m|
    var error = $m[error]
    if (is $error $nil) {
        set cmd-status-code = 0
    } else {
        try {
            set cmd-status-code = $error[reason][exit-status]
        } catch {
            # The error is from the built-in commands and they have no status code.
            set cmd-status-code = 1
        }
    }
}

# Install Hooks
set edit:after-command = [ $@edit:after-command $starship-after-command-hook~ ]

# Install starship
set edit:prompt = {
    var cmd-duration = (printf "%.0f" (* $edit:command-duration 1000))
    ::STARSHIP:: prompt --jobs=$num-bg-jobs --cmd-duration=$cmd-duration --status=$cmd-status-code --logical-path=$pwd
}

set edit:rprompt = {
    var cmd-duration = (printf "%.0f" (* $edit:command-duration 1000))
    ::STARSHIP:: prompt --right --jobs=$num-bg-jobs --cmd-duration=$cmd-duration --status=$cmd-status-code --logical-path=$pwd
}
