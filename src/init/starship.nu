let-env STARSHIP_SHELL = "nu"
let-env STARSHIP_SESSION = (random chars)

def _starship_prompt [] {
    # jobs are not supported
    # status is not supported
    ^::STARSHIP:: prompt --cmd-duration $nu.env.CMD_DURATION_MS
}
