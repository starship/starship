# Selected only when Nushell supports prompt updates and job mailboxes.

const STARSHIP_JOB = "starship-stream"

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

## Every pair of prompts is pushed to the editor, which repaints in place only
## a prompt pushed to it, and the first also goes to the mailbox the prompt
## waits on. `bytes split` reads as the stream writes, leaving an empty field
## after the last NUL; pairing the fields preserves multiline prompts. What
## the stream reports goes to the file `reports`, as Nushell shows nothing a
## job reports.
def starship-stream-read [arguments: list<string>, mailbox: int, reports: string] {
    let drawn = try {
        ^::STARSHIP:: prompt --stream ...$arguments e> $reports
        | bytes split 0x[00]
        | each { decode }
        | chunks 2
        | where ($it | length) == 2
        | reduce --fold false {|prompts, drawn|
            if not $drawn {
                $prompts.1 | job send 0 --tag ($mailbox + 1)
                $prompts.0 | job send 0 --tag $mailbox
            }
            commandline set-prompt --right $prompts.1 $prompts.0
            true
        }
    } catch { false }

    # Unblock the prompt even if the stream fails.
    if not $drawn {
        "" | job send 0 --tag ($mailbox + 1)
        "" | job send 0 --tag $mailbox
    }
}

def starship-stream-start [] {
    let jobs = job list
    let arguments = starship-prompt-arguments $jobs
    starship-stream-stop $jobs

    # A stream stopped before the prompt read them leaves its first prompts.
    let mailbox = $env.STARSHIP_MAILBOX
    job flush --tag $mailbox
    job flush --tag ($mailbox + 1)
    let reports = $env.STARSHIP_REPORTS
    job spawn --description $STARSHIP_JOB { starship-stream-read $arguments $mailbox $reports }

    let main = job recv --tag $mailbox
    # The stream closes its error output as it first paints, so what it
    # reported before then comes ahead of the prompt.
    let reported = try { open --raw $reports } catch { "" }
    if ($reported | is-not-empty) { print --stderr --no-newline $reported }
    $main
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

    # Mailbox slots: main, right.
    let mailbox = random int 1..9223372036854775806
    let session = random chars -l 16
    load-env {
        STARSHIP_SESSION_KEY: $session
        STARSHIP_MAILBOX: $mailbox
        STARSHIP_REPORTS: ($nu.temp-dir | path join $"starship_($session).reports")
        PROMPT_MULTILINE_INDICATOR: (^::STARSHIP:: prompt --continuation)
        PROMPT_INDICATOR: ""
        PROMPT_COMMAND: {|| starship-stream-start }
        # Sent before the main prompt, so there by the time it is drawn.
        PROMPT_COMMAND_RIGHT: {|| try { job recv --tag ($env.STARSHIP_MAILBOX + 1) --timeout 0sec } catch { "" } }
    }
}
