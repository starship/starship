def starship_prompt():
    last_cmd = __xonsh__.history[-1] if __xonsh__.history else None
    status = last_cmd.rtn if last_cmd else 0
    duration = round((last_cmd.ts[1] - last_cmd.ts[0]) * 1000) if last_cmd else 0
    jobs = len($(jobs).splitlines())
    return $(::STARSHIP:: prompt --status @(status) --jobs @(jobs) --cmd-duration @(duration))


$PROMPT = starship_prompt
$STARSHIP_SHELL = "xonsh"
