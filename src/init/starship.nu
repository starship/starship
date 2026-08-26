# Requires `commandline set-prompt`.

const STARSHIP_JOB = "starship-stream"
const STARSHIP_TARGETS = [
    {side: "left", flag: [], offset: 0}
    {side: "right", flag: ["--right"], offset: 2}
]

def starship-stream-stop [target?: record] {
    let job = $target | get -o job
    job list
    | where {|running| if ($job | is-empty) { $running.description | str starts-with $STARSHIP_JOB } else { $running.description == $job } }
    | each {|running| try { job kill $running.id }}
    | ignore
}

def starship-prompt-arguments []: nothing -> list<string> {
    let duration = if $env.CMD_DURATION_MS == "0823" { 0 } else { $env.CMD_DURATION_MS }
    [
        $"--cmd-duration=($duration)"
        $"--status=($env.LAST_EXIT_CODE)"
        $"--terminal-width=((term size).columns)"
        $"--jobs=(job list | where {|job| not ($job.description | str starts-with $STARSHIP_JOB) } | length)"
    ]
}

def starship-render [target: record, arguments: list<string>] {
    ^::STARSHIP:: prompt ...$target.flag ...$arguments
}

def starship-set-prompt [target: record, prompt: string] {
    if $target.side == "right" { commandline set-prompt --right $prompt } else { commandline set-prompt $prompt }
}

def starship-stream-read [target: record, arguments: list<string>] {
    let complete = try {
        ^::STARSHIP:: stream --frames json ...$target.flag ...$arguments $"--timings=($target.timings)"
        | from json --objects
        | generate {|frame, complete = false|
            let complete = match $frame.kind {
                "READY" => {
                    if $target.side == "right" { starship-set-prompt $target $frame.prompt }
                    $frame.prompt | job send 0 --tag $target.ready_tag
                    $complete
                }
                "PATCH" => { starship-set-prompt $target $frame.prompt; $complete }
                "COMPLETE" => { $frame.timings | to json --raw | job send 0 --tag $target.timings_tag; true }
                _ => $complete
            }
            {out: $complete, next: $complete}
        }
        | last
        | default false
    } catch { false }

    if not $complete {
        let prompt = starship-render $target $arguments
        starship-set-prompt $target $prompt
        $prompt | job send 0 --tag $target.ready_tag
    }
}

# Each side owns its process and mailbox slots. Launch both before waiting on
# the left: the right-prompt hook must never hold the editor before it accepts
# input, and its READY frame applies itself asynchronously.
def starship-stream-launch [target: record, arguments: list<string>] {
    let ready_tag = $env.STARSHIP_MAILBOX + $target.offset
    let timings_tag = $ready_tag + 1
    job flush --tag $ready_tag
    let target = $target | merge {
        job: $"($STARSHIP_JOB)-($target.side)"
        ready_tag: $ready_tag
        timings_tag: $timings_tag
        timings: (try { job recv --tag $timings_tag --timeout 0sec } catch { "" })
    }
    job spawn --description $target.job { starship-stream-read $target $arguments } | ignore
    $target
}

def starship-stream-ready [target: record, arguments: list<string>] {
    let ready_tag = $env.STARSHIP_MAILBOX + $target.offset
    try { job recv --tag $ready_tag --timeout 2sec } catch {
        starship-stream-stop $target
        starship-render $target $arguments
    }
}

def starship-stream-start [arguments: list<string>] {
    starship-stream-stop
    let targets = $STARSHIP_TARGETS | each {|target| starship-stream-launch $target $arguments }
    starship-stream-ready $targets.0 $arguments
}

def starship-right-prompt [] {
    try { job recv --tag ($env.STARSHIP_MAILBOX + $STARSHIP_TARGETS.1.offset) --timeout 0sec } catch { "" }
}

export-env {
    $env.STARSHIP_SHELL = "nu"
    let hooks = $env.config?.hooks? | default {}
    let pre_execution = $hooks.pre_execution? | default [] | append {|| starship-stream-stop }
    $env.config = (
        $env.config?
        | default {}
        | merge {render_right_prompt_on_last_line: true}
        | upsert hooks ($hooks | upsert pre_execution $pre_execution)
    )

    load-env {
        STARSHIP_SESSION_KEY: (random chars -l 16)
        STARSHIP_MAILBOX: (random int 1..9223372036854775803)
        PROMPT_MULTILINE_INDICATOR: (^::STARSHIP:: prompt --continuation)
        PROMPT_INDICATOR: ""
        PROMPT_COMMAND: {||
            starship-stream-start (starship-prompt-arguments)
        }
        PROMPT_COMMAND_RIGHT: {|| starship-right-prompt }
    }
}
