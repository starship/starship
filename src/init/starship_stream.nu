# Selected only when Nushell supports prompt updates and job mailboxes.

const STARSHIP_JOB = "starship-stream"

## An empty mailbox is normal; `job recv` otherwise throws.
def starship-recv [tag: int]: nothing -> string {
    try { job recv --tag $tag --timeout 0sec } catch { "" }
}

def starship-stream-stop [jobs: table] {
    for stream in ($jobs | where description == $STARSHIP_JOB) { try { job kill $stream.id } }
}

def starship-prompt-arguments [jobs: table]: nothing -> list<string> {
    # A fresh session reports the magic duration "0823"; show none.
    let duration = if $env.CMD_DURATION_MS == "0823" { 0 } else { $env.CMD_DURATION_MS }
    [
        $"--cmd-duration=($duration)"
        $"--status=($env.LAST_EXIT_CODE)"
        $"--terminal-width=((term size).columns)"
        $"--jobs=($jobs | where description != $STARSHIP_JOB | length)"
    ]
}

## Deliver startup prompts to the mailbox and updates to the editor.
def starship-show [main: string, right: string, mailbox: int] {
    $main | job send 0 --tag $mailbox
    $right | job send 0 --tag ($mailbox + 2)
    commandline set-prompt --right $right $main
}

## `bytes split` reads incrementally; grouping NUL fields preserves multiline prompts.
def starship-stream-read [arguments: list<string>, timings: string, mailbox: int] {
    let shown = try {
        ^::STARSHIP:: prompt --stream ...$arguments $"--timings=($timings)"
        | bytes split 0x[00]
        | each { decode }
        | chunks 3
        | reduce --fold false {|frame, shown|
            match $frame {
                ["PROMPT" $main $right] => { starship-show $main $right $mailbox; true }
                ["COMPLETE" $timings $_] => { $timings | job send 0 --tag ($mailbox + 1); $shown }
                _ => $shown
            }
        }
    } catch { false }

    # Unblock startup even if the stream fails.
    if not $shown {
        starship-show "" "" $mailbox
    }
}

def starship-stream-start [] {
    let jobs = job list
    let arguments = starship-prompt-arguments $jobs
    starship-stream-stop $jobs

    # Discard stale prompts while retaining the last timing report.
    let mailbox = $env.STARSHIP_MAILBOX
    job flush --tag $mailbox
    job flush --tag ($mailbox + 2)
    let timings = starship-recv ($mailbox + 1)
    job spawn --description $STARSHIP_JOB { starship-stream-read $arguments $timings $mailbox }

    job recv --tag $mailbox
}

export-env {
    $env.STARSHIP_SHELL = "nu"

    # `append` preserves a bare closure or a list; deep merge keeps other settings.
    let hooks = $env.config?.hooks? | default {}
    $env.config = (
        $env.config?
        | default {}
        | merge deep {
            render_right_prompt_on_last_line: true
            hooks: {pre_execution: ($hooks.pre_execution? | default [] | append {|| starship-stream-stop (job list) })}
        }
    )

    # Mailbox slots: main, timings, right.
    let mailbox = random int 1..9223372036854775803
    load-env {
        STARSHIP_SESSION_KEY: (random chars -l 16)
        STARSHIP_MAILBOX: $mailbox
        PROMPT_MULTILINE_INDICATOR: (^::STARSHIP:: prompt --continuation)
        PROMPT_INDICATOR: ""
        PROMPT_COMMAND: {|| starship-stream-start }
        PROMPT_COMMAND_RIGHT: {|| starship-recv ($env.STARSHIP_MAILBOX + 2) }
    }
}
