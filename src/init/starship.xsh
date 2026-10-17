import asyncio
import os
import subprocess
import uuid

starship_program = ::STARSHIP::


def starship_arguments():
    last_cmd = __xonsh__.history[-1] if __xonsh__.history else None
    status = last_cmd.rtn if last_cmd else 0
    # I believe this is equivalent to xonsh.jobs.get_next_job_number() for our purposes,
    # but we can't use that function because of https://gitter.im/xonsh/xonsh?at=60e8832d82dd9050f5e0c96a
    jobs = sum(1 for job in __xonsh__.all_jobs.values() if job['obj'] and job['obj'].poll() is None)
    duration = round((last_cmd.ts[1] - last_cmd.ts[0]) * 1000) if last_cmd else 0
    return [f"--status={status}", f"--jobs={jobs}", f"--cmd-duration={duration}"]


class StarshipStream:
    """Read prompt updates through the line editor's event loop."""

    def __init__(self):
        self.prompts = ["", ""]
        self.process = None
        self.received = b""
        self.loop = None

    def apply(self):
        """Take the newest complete pair of prompts; return whether there was one."""
        fields = self.received.split(b"\0")
        complete = (len(fields) - 1) // 2 * 2
        if not complete:
            return False
        self.prompts = [field.decode() for field in fields[complete - 2 : complete]]
        self.received = b"\0".join(fields[complete:])
        return True

    def start(self):
        """Stream with prompt_toolkit on Unix; otherwise draw the prompts once complete."""
        self.stop()
        self.prompts = ["", ""]
        prompter = getattr(__xonsh__.shell.shell, "prompter", None)
        refined = prompter is not None and os.name == "posix"
        self.process = subprocess.Popen(
            [starship_program, "prompt", "--stream", *([] if refined else ["--redraws=0"]), *starship_arguments()],
            env=__xonsh__.env.detype(),
            stdout=subprocess.PIPE,
        )
        descriptor = self.process.stdout.fileno()
        drawn = self.apply()
        while not drawn:
            received = os.read(descriptor, 65536)
            if not received:
                break
            self.received += received
            drawn = self.apply()
        if refined and drawn:
            prompter.app.pre_run_callables.append(self.attach)
        else:
            self.stop()

    def attach(self):
        if self.process is not None:
            self.loop = asyncio.get_running_loop()
            self.loop.add_reader(self.process.stdout.fileno(), self.receive)

    def receive(self):
        received = os.read(self.process.stdout.fileno(), 65536)
        if not received:
            self.stop()
            return
        self.received += received
        if not self.apply():
            return
        shell = __xonsh__.shell.shell
        # Reformat updates when Xonsh is not already doing so on each keypress.
        if not __xonsh__.env.get("UPDATE_PROMPT_ON_KEYPRESS"):
            shell.prompter.message = shell.prompt_tokens()
            shell.prompter.rprompt = shell.rprompt_tokens()
        shell.prompter.app.invalidate()

    def stop(self):
        if self.process is None:
            return
        if self.loop is not None and not self.loop.is_closed():
            self.loop.remove_reader(self.process.stdout.fileno())
        self.process.kill()
        self.process.wait()
        self.process.stdout.close()
        self.process = self.loop = None
        self.received = b""


starship_stream = StarshipStream()


@events.on_pre_prompt_format
def starship_stream_start(**_):
    starship_stream.start()


@events.on_post_prompt
@events.on_exit
def starship_stream_stop(**_):
    starship_stream.stop()


def starship_prompt():
    return starship_stream.prompts[0]

def starship_rprompt():
    return starship_stream.prompts[1]


$PROMPT = starship_prompt
$RIGHT_PROMPT = starship_rprompt
$STARSHIP_SHELL = "xonsh"
$STARSHIP_SESSION_KEY = uuid.uuid4().hex
