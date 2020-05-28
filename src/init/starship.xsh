from xonsh.jobs import get_next_job_number


def starship_prompt():
    last_cmd = __xonsh__.history[-1] if __xonsh__.history else None
    status = last_cmd.rtn if last_cmd else 0
    jobs = max(get_next_job_number(), 1) - 1
    duration = round((last_cmd.ts[1] - last_cmd.ts[0]) * 1000) if last_cmd else 0
    return $(::STARSHIP:: prompt --status=@(status) --jobs=@(jobs) --cmd-duration=@(duration))


$PROMPT = starship_prompt
$STARSHIP_SHELL = "xonsh"
