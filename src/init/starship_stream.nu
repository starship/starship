# Streams the prompt: `starship prompt --stream` draws it as soon as what
# renders quickly has, and refines it as the rest does. `starship init nu` picks
# this script only for a nushell with `commandline set-prompt` and the job
# mailbox.

const STARSHIP_JOB = "starship-stream"

## Takes whatever is waiting in a mailbox slot. `job recv` throws on an empty
## mailbox, but "nothing has arrived yet" is an ordinary answer here.
def starship-recv [tag: int]: nothing -> string {
    try { job recv --tag $tag --timeout 0sec } catch { "" }
}

## Kills every stream in a job table.
def starship-stream-stop [jobs: table] {
    for stream in ($jobs | where description == $STARSHIP_JOB) { try { job kill $stream.id } }
}

## Takes the job table the caller already has, because the count of the user's
## own jobs and the list of streams to kill are the same snapshot.
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

## Shows prompts: through the mailbox, to PROMPT_COMMAND and
## PROMPT_COMMAND_RIGHT if they are still waiting for the first, and on the
## line editor, for prompts already drawn.
def starship-show [main: string, right: string, mailbox: int] {
    $main | job send 0 --tag $mailbox
    $right | job send 0 --tag ($mailbox + 2)
    commandline set-prompt --right $right $main
}

## Runs a stream inside a job, applying each frame as it lands.
##
## A frame is a keyword and two NUL-terminated fields, so the reader cuts the
## output at every NUL and takes three fields at a time. `bytes split` reads the
## output as it arrives, so frames are applied as they land, and a prompt of
## several lines arrives as it is.
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

    # A stream that fails draws nothing, as a failing `starship prompt` would,
    # without keeping the prompt waiting for it.
    if not $shown {
        starship-show "" "" $mailbox
    }
}

## Starts the stream for a new prompt, and returns its main prompt.
def starship-stream-start [] {
    # One listing of jobs gives both the user's own, which are counted, and the
    # last prompt's stream, which is stopped.
    let jobs = job list
    let arguments = starship-prompt-arguments $jobs
    starship-stream-stop $jobs

    # Nothing a stopped stream sent can be taken for this prompt's, and the
    # timings the last stream reported are handed to this one.
    let mailbox = $env.STARSHIP_MAILBOX
    job flush --tag $mailbox
    job flush --tag ($mailbox + 2)
    let timings = starship-recv ($mailbox + 1)
    job spawn --description $STARSHIP_JOB { starship-stream-read $arguments $timings $mailbox }

    # The stream draws its first paint within its fallback.
    job recv --tag $mailbox
}

export-env {
    $env.STARSHIP_SHELL = "nu"

    # A deep merge keeps every other hook and setting. `pre_execution` may be a
    # bare closure rather than a list, and `--strategy=append` would drop that
    # one silently, so build the list with `append`, which takes either shape.
    let hooks = $env.config?.hooks? | default {}
    $env.config = (
        $env.config?
        | default {}
        | merge deep {
            render_right_prompt_on_last_line: true
            hooks: {pre_execution: ($hooks.pre_execution? | default [] | append {|| starship-stream-stop (job list) })}
        }
    )

    # Three mailbox slots for the session: the main prompt's first paint, the
    # timings for the next stream, and the right prompt.
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
