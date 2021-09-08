set-env STARSHIP_SHELL "elvish"
set-env STARSHIP_SESSION_KEY (::STARSHIP:: session)

# Define Hooks
local:cmd-start-time = 0
local:cmd-end-time = 0

fn starship-after-readline-hook [line]{
    cmd-start-time = (::STARSHIP:: time)
}

fn starship-before-readline-hook {
    cmd-end-time = (::STARSHIP:: time)
    cmd-duration = (- $cmd-end-time $cmd-start-time)
}

# Install Hooks
edit:after-readline = [ $@edit:after-readline $starship-after-readline-hook~ ]
edit:before-readline = [ $@edit:before-readline $starship-before-readline-hook~ ]

# Install starship
edit:prompt = {
    # Note:
    # Elvish does not appear to support exit status codes (--status)

    if (== $cmd-start-time 0) {
        ::STARSHIP:: prompt --jobs=$num-bg-jobs
    } else {
        ::STARSHIP:: prompt --jobs=$num-bg-jobs --cmd-duration=$cmd-duration
    }
}

edit:rprompt = {
    # Note:
    # Elvish does not appear to support exit status codes (--status)

    if (== $cmd-start-time 0) {
        ::STARSHIP:: prompt --right --jobs=$num-bg-jobs
    } else {
        ::STARSHIP:: prompt --right --jobs=$num-bg-jobs --cmd-duration=$cmd-duration
    }
}
