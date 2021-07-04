let-env STARSHIP_SHELL = "nu"
let-env STARSHIP_SESSION = (random chars -l 16)

def starship_prompt [] {
    # jobs are not supported
    # status is not supported
    ^::STARSHIP:: prompt --cmd-duration $nu.env.CMD_DURATION_MS
}
