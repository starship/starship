import random
import string


def starship_prompt():
    last_cmd = __xonsh__.history[-1] if __xonsh__.history else None
    status = last_cmd.rtn if last_cmd else 0
    jobs = len(__xonsh__.all_jobs)
    duration = round((last_cmd.ts[1] - last_cmd.ts[0]) * 1000) if last_cmd else 0
    return $(::STARSHIP:: prompt --status=@(status) --jobs=@(jobs) --cmd-duration=@(duration))


$PROMPT = starship_prompt
$STARSHIP_SHELL = "xonsh"
$STARSHIP_SESSION_KEY = "".join(random.choice(string.ascii_letters + string.digits) for _ in range(16))
