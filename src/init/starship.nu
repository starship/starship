# Requires a nushell with `commandline set-prompt` and the job mailbox.

const STARSHIP_JOB = "starship-stream"

## Takes whatever is waiting in a mailbox slot. `job recv` throws on an empty
## mailbox, but "nothing has arrived yet" is an ordinary answer here.
def starship-recv [tag: int]: nothing -> string {
    try { job recv --tag $tag --timeout 0sec } catch { "" }
}

## Kills every starship stream in a job table, or just the one job named.
def starship-stream-stop [jobs: table, job: string = $STARSHIP_JOB] {
    for stream in ($jobs | where description starts-with $job) { try { job kill $stream.id } }
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
        $"--jobs=($jobs | where description not-starts-with $STARSHIP_JOB | length)"
    ]
}

## Runs the one stream inside a job, applying each frame as it lands, and
## reports whether the render ever completed.
##
## A frame is three NUL-terminated fields, a keyword and two payloads, so the
## whole reader is three pipeline stages that carry no state between them: cut
## the byte stream at every NUL, decode each field, take three fields to a frame.
## `bytes split` reads a child's stdout lazily, so frames are applied as they
## land rather than at exit, and a multi-line prompt arrives verbatim with
## nothing to unescape. nu is prompt-replace, so a frame's second payload, a
## process id or the cells a cell-precise shell would repaint in place, is never
## read; the empty field left by the last terminator at EOF matches no pattern.
##
## One renderer draws both sides. The left's first paint rides the mailbox back
## to the main thread, which returns it from PROMPT_COMMAND before the first
## draw. The right's has no such handshake, so it both rides its own slot and
## paints itself, and whichever the draw reaches first is the one that shows.
def starship-stream-read [arguments: list<string>, timings: string, mailbox: int] {
    let completed = try {
        ^::STARSHIP:: stream --both ...$arguments $"--timings=($timings)"
        | bytes split 0x[00]
        | each { decode }
        | chunks 3
        | reduce --fold false {|frame, completed|
            match $frame {
                ["READY" $prompt $_] => { $prompt | job send 0 --tag $mailbox; $completed }
                ["PATCH" $prompt $_] => { ignore; commandline set-prompt $prompt; $completed }
                ["RIGHT" $prompt $_] => {
                    $prompt | job send 0 --tag ($mailbox + 2)
                    ignore
                    commandline set-prompt --right $prompt
                    $completed
                }
                ["COMPLETE" $timings $_] => { $timings | job send 0 --tag ($mailbox + 1); true }
                _ => $completed
            }
        }
    } catch { false }

    # A stream that died mid-render leaves the prompt unrefined; redraw it the
    # slow way. Hand it to the main thread first, in case it is still waiting.
    if not $completed {
        let prompt = ^::STARSHIP:: prompt ...$arguments
        $prompt | job send 0 --tag $mailbox
        ignore
        commandline set-prompt $prompt
    }
}

## Draws the left prompt and returns it: PROMPT_COMMAND's value is the prompt.
def starship-stream-start [] {
    # One `job list` serves both readings of it — the streams left over from the
    # last prompt, which are killed here, and the user's own jobs, which are
    # counted for `--jobs`. Taking the snapshot before the kill keeps the count
    # measuring what it did when this ran as two separate listings.
    let jobs = job list
    let arguments = starship-prompt-arguments $jobs
    starship-stream-stop $jobs

    # Both first-paint slots start empty, so nothing from a killed stream can be
    # mistaken for this prompt's. The timings the last prompt measured ride back
    # verbatim, so the renderer starts knowing what each module has been costing.
    let mailbox = $env.STARSHIP_MAILBOX
    job flush --tag $mailbox
    job flush --tag ($mailbox + 2)
    let timings = starship-recv ($mailbox + 1)
    job spawn --description $STARSHIP_JOB { starship-stream-read $arguments $timings $mailbox }

    # Bound the handshake; a stuck stream falls back to a synchronous render.
    try { job recv --tag $mailbox --timeout 2sec } catch {
        # Re-list: the snapshot above predates the job this is about to kill.
        starship-stream-stop (job list)
        ^::STARSHIP:: prompt ...$arguments
    }
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

    # Three mailbox slots off one base: the left's first paint, the timings
    # handed forward to the next prompt, and the right's first paint. The base
    # does not change over a session, so it is drawn once, here.
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
