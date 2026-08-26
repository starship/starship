# Requires `commandline set-prompt`.

const STARSHIP_JOB = "starship-stream"

def starship-stream-jobs [] {
    job list | where description == $STARSHIP_JOB
}

def starship-stream-stop [] {
    starship-stream-jobs | each {|job| try { job kill $job.id }} | ignore
}

def starship-command-duration [] {
    if $env.CMD_DURATION_MS == "0823" { 0 } else { $env.CMD_DURATION_MS }
}

def starship-prompt-arguments []: nothing -> list<string> {
    [
        "--cmd-duration"
        (starship-command-duration | into string)
        $"--status=($env.LAST_EXIT_CODE)"
        "--terminal-width"
        ((term size).columns | into string)
        "--jobs"
        (job list | where description != $STARSHIP_JOB | length | into string)
    ]
}

def starship-render [arguments: list<string>] {
    ^::STARSHIP:: prompt ...$arguments
}

def starship-stream-apply [frame: record, complete: bool]: nothing -> bool {
    match $frame.kind {
        "READY" => {
            $frame.prompt | job send 0 --tag $env.STARSHIP_READY_TAG
            $complete
        }
        "PATCH" => {
            commandline set-prompt $frame.prompt
            $complete
        }
        "COMPLETE" => {
            $frame.timings | to json --raw | job send 0 --tag $env.STARSHIP_TIMINGS_TAG
            true
        }
        _ => $complete
    }
}

def starship-stream-read [arguments: list<string>, timings: string] {
    let complete = try {
        ^::STARSHIP:: stream --frames json ...$arguments $"--timings=($timings)"
        | from json --objects
        | generate {|frame, complete = false|
            let complete = starship-stream-apply $frame $complete
            {out: $complete, next: $complete}
        }
        | last
        | default false
    } catch {
        false
    }

    if not $complete {
        let prompt = starship-render $arguments
        commandline set-prompt $prompt
        $prompt | job send 0 --tag $env.STARSHIP_READY_TAG
    }
}

def starship-stream-start [arguments: list<string>] {
    starship-stream-stop
    job flush --tag $env.STARSHIP_READY_TAG

    let timings = try {
        job recv --tag $env.STARSHIP_TIMINGS_TAG --timeout 0sec
    } catch {
        ""
    }

    job spawn --description $STARSHIP_JOB {
        starship-stream-read $arguments $timings
    } | ignore

    try {
        job recv --tag $env.STARSHIP_READY_TAG --timeout 2sec
    } catch {
        starship-stream-stop
        starship-render $arguments
    }
}

export-env {
    $env.STARSHIP_SHELL = "nu"
    let mailbox = random int 1..9223372036854775806

    let hooks = $env.config?.hooks? | default {}
    let pre_execution = $hooks.pre_execution? | default [] | append {||
        job list
        | where description == $STARSHIP_JOB
        | each {|job| try { job kill $job.id }}
        | ignore
    }
    $env.config = (
        $env.config?
        | default {}
        | merge {render_right_prompt_on_last_line: true}
        | upsert hooks ($hooks | upsert pre_execution $pre_execution)
    )

    load-env {
        STARSHIP_SESSION_KEY: (random chars -l 16)
        STARSHIP_READY_TAG: $mailbox
        STARSHIP_TIMINGS_TAG: ($mailbox + 1)
        PROMPT_MULTILINE_INDICATOR: (^::STARSHIP:: prompt --continuation)
        PROMPT_INDICATOR: ""
        PROMPT_COMMAND: {|| starship-stream-start (starship-prompt-arguments) }
        PROMPT_COMMAND_RIGHT: {||
            ^::STARSHIP:: prompt --right ...(starship-prompt-arguments)
        }
    }
}
