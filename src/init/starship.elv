set-env STARSHIP_SHELL "elvish"
set-env STARSHIP_SESSION_KEY (::STARSHIP:: session)

# Define Hooks
var cmd-status-code = 0
var cmd-duration = 0

fn starship-after-command-hook {|m|
    var error = $m[error]
    if (is $error $nil) {
        set cmd-status-code = 0
    } else {
        try {
            set cmd-status-code = $error[reason][exit-status]
        } except {
            # The error is from the built-in commands and they have no status code.
            set cmd-status-code = 1
        }
    }
    set cmd-duration = (printf "%.0f" (* $m[duration] 1000))
}

# Install Hooks
set edit:after-command = [ $@edit:after-command $starship-after-command-hook~ ]

# Install starship
set edit:prompt = {
    ::STARSHIP:: prompt --jobs=$num-bg-jobs --cmd-duration=$cmd-duration --status $cmd-status-code
    set cmd-duration = 0
}

set edit:rprompt = {
    ::STARSHIP:: prompt --right --jobs=$num-bg-jobs --cmd-duration=$cmd-duration --status $cmd-status-code
    set cmd-duration = 0
}
