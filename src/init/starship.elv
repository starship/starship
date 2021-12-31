set-env STARSHIP_SHELL "elvish"
set-env STARSHIP_SESSION_KEY (::STARSHIP:: session)

# Install starship
set edit:prompt = {
    # Note:
    # Elvish does not appear to support exit status codes (--status)
    var cmd-duration = (printf "%.0f" (* $edit:command-duration 1000))
    ::STARSHIP:: prompt --jobs=$num-bg-jobs --cmd-duration=$cmd-duration
}

set edit:rprompt = {
    # Note:
    # Elvish does not appear to support exit status codes (--status)
    var cmd-duration = (printf "%.0f" (* $edit:command-duration 1000))
    ::STARSHIP:: prompt --right --jobs=$num-bg-jobs --cmd-duration=$cmd-duration
}
