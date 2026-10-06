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
    """The prompt, streamed: `starship prompt --stream` draws it as soon as
    what renders quickly has, and refines it as the rest does. Its frames
    arrive on a pipe that the line editor's event loop reads while the user
    types.
    """

    def __init__(self):
        # The main and the right prompt, as the stream last drew them.
        self.prompts = ["", ""]
        # The timings the last stream reported, which the next one is handed.
        self.timings = ""
        self.process = None
        # What has arrived from the stream but is not yet a whole frame.
        self.received = b""
        # The event loop reading the stream, while the prompt is shown.
        self.loop = None

    def apply(self):
        """Applies every whole frame that has arrived, each a keyword and two
        fields ending in a NUL, and returns the keywords of those applied."""
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
        """Starts a stream for the prompt about to be drawn, and waits for its
        first prompts, which the stream draws within its fallback. Only
        prompt_toolkit refines a prompt it shows, and only an event loop on
        unix reads a pipe as it fills; anywhere else, the stream is read until
        every module has rendered, and the prompt is drawn once."""
        self.stop()
        # A stream that fails draws nothing, as a failing `starship prompt`
        # would.
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
        """Reads the stream until a frame of `keyword` arrives, and returns
        whether one did before the stream ended."""
        descriptor = self.process.stdout.fileno()
        while received := os.read(descriptor, 65536):
            self.received += received
            if keyword in self.apply():
                return True
        return False

    def attach(self):
        """Reads the stream on the event loop that shows the prompt."""
        if self.process is not None:
            self.loop = asyncio.get_running_loop()
            self.loop.add_reader(self.process.stdout.fileno(), self.receive)

    def receive(self):
        """Draws the prompts that have arrived."""
        received = os.read(self.process.stdout.fileno(), 65536)
        if not received:
            self.stop()
            return
        self.received += received
        if b"PROMPT" not in self.apply():
            return
        shell = __xonsh__.shell.shell
        # xonsh formats a prompt once, before showing it, unless it is asked to
        # on every key press, so a refined prompt is formatted here.
        if not __xonsh__.env.get("UPDATE_PROMPT_ON_KEYPRESS"):
            shell.prompter.message = shell.prompt_tokens()
            shell.prompter.rprompt = shell.rprompt_tokens()
        shell.prompter.app.invalidate()

    def stop(self):
        """Stops the stream, and the event loop reading it."""
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
