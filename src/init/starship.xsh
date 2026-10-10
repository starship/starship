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
        self.timings = ""
        self.process = None
        self.received = b""
        self.loop = None

    def apply(self):
        """Apply complete frames and retain partial bytes; return applied keywords."""
        applied = set()
        while self.received.count(b"\0") >= 3:
            kind, first, second, self.received = self.received.split(b"\0", 3)
            if kind == b"PROMPT":
                self.prompts = [first.decode(), second.decode()]
            elif kind == b"COMPLETE":
                self.timings = first.decode()
            applied.add(kind)
        return applied

    def start(self):
        """Stream with prompt_toolkit on Unix; otherwise wait for completion."""
        self.stop()
        self.prompts = ["", ""]
        prompter = getattr(__xonsh__.shell.shell, "prompter", None)
        refined = prompter is not None and os.name == "posix"
        self.process = subprocess.Popen(
            [starship_program, "prompt", "--stream", f"--timings={self.timings}", *starship_arguments()],
            env=__xonsh__.env.detype(),
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
        )
        if self.read_until(b"PROMPT" if refined else b"COMPLETE") and refined:
            prompter.app.pre_run_callables.append(self.attach)
            return
        self.stop()

    def read_until(self, keyword):
        """Return whether the requested frame arrives before EOF."""
        descriptor = self.process.stdout.fileno()
        while received := os.read(descriptor, 65536):
            self.received += received
            if keyword in self.apply():
                return True
        return False

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
        if b"PROMPT" not in self.apply():
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
