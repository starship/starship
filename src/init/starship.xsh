import os, subprocess, threading
from prompt_toolkit import ANSI

STARSHIP = ::STARSHIP::


# Compact frames are KEYWORD\0first\0second\0. Popen.stdout has no readuntil
# (that is asyncio), so one os.read fills whatever is already in the pipe —
# a sized read() would block until n bytes or EOF, and this stream stays
# open for heartbeats. One read usually carries several frames, so the count of
# terminators still waiting is carried across iterations rather than recovered by
# rescanning the frames not yet handed out; the buffer is a bytearray so filling
# it appends in place instead of copying what is left over.
def _frames(pipe):
    fd, buf, nuls = pipe.fileno(), bytearray(), 0
    while True:
        while nuls < 3:
            if not (chunk := os.read(fd, 65536)):
                return
            nuls += chunk.count(0)
            buf += chunk
        keyword, first, second, buf = buf.split(b"\0", 3)
        nuls -= 3
        yield keyword.decode(), first.decode(), second.decode()


class _Stream:
    """One renderer, both prompts.

    xonsh asks for $PROMPT and $RIGHT_PROMPT separately and in no order this
    can rely on, so whichever is asked first starts the stream and the other
    reads what it left behind. `started` is what makes that once per prompt
    rather than once per side.
    """

    __slots__ = "proc", "timings", "prompts", "started"

    def __init__(self):
        self.proc, self.timings, self.prompts, self.started = None, "", ["", ""], False

    def render(self, args, flag=()):
        try:
            return subprocess.check_output(
                [STARSHIP, "prompt", *flag, *args], stderr=subprocess.DEVNULL, text=True
            )
        except (OSError, subprocess.CalledProcessError):
            return ""

    def render_both(self, args):
        self.prompts = [self.render(args), self.render(args, ("--right",))]

    def stop(self):
        proc, self.proc, self.started = self.proc, None, False
        proc and proc.poll() is None and proc.kill()

    def publish(self, side, text, proc, session):
        if self.proc is not proc:
            return
        self.prompts[side] = text
        formatted = ANSI(text)

        def apply():
            if self.proc is proc:
                setattr(session, "rprompt" if side else "message", formatted)
                session.app.invalidate()

        try:
            session.app.loop.call_soon_threadsafe(apply)
        except (AttributeError, RuntimeError):
            pass

    def read(self, args, proc, session, frames):
        done = False
        try:
            for kind, first, _ in frames:
                if kind == "PATCH":
                    self.publish(0, first, proc, session)
                elif kind == "RIGHT":
                    self.publish(1, first, proc, session)
                elif kind == "COMPLETE":
                    done = True
                    if self.proc is proc:
                        self.timings = first
        except (OSError, ValueError):
            pass
        if not done:
            self.publish(0, self.render(args), proc, session)
            self.publish(1, self.render(args, ("--right",)), proc, session)

    def begin(self, session):
        if self.started:
            return
        self.stop()
        self.started = True
        args = _args()
        self.prompts = ["", ""]
        if not session:
            self.render_both(args)
            return

        proc = None
        try:
            proc = subprocess.Popen(
                [STARSHIP, "stream", "--both", *args, f"--timings={self.timings}"],
                stdout=subprocess.PIPE,
                stderr=subprocess.DEVNULL,
            )
            frames = _frames(proc.stdout)
            # READY is what ends the wait, but the right side's first paint
            # usually reaches the pipe before it, so anything else is applied on
            # the way past and the first draw has both sides. Bounded, so a
            # renderer that never says READY cannot hold the prompt.
            for _ in range(4):
                kind, first, _second = next(frames)
                if kind == "RIGHT":
                    self.prompts[1] = first
                elif kind == "READY":
                    self.prompts[0] = first
                    break
            else:
                raise ValueError
        except (OSError, ValueError, StopIteration):
            proc and proc.poll() is None and proc.kill()
            self.render_both(args)
            return

        self.proc = proc
        threading.Thread(
            target=self.read, args=(args, proc, session, frames), daemon=True
        ).start()


_S = _Stream()


def _args():
    try:
        last = __xonsh__.history[-1]
    except Exception:
        last = None
    try:
        width = os.get_terminal_size().columns
    except OSError:
        width = 80
    return [
        f"--status={last.rtn if last else 0}",
        f"--jobs={sum(j.get('obj') is not None and j['obj'].poll() is None for j in __xonsh__.all_jobs.values())}",
        f"--cmd-duration={round((last.ts[1] - last.ts[0]) * 1000) if last else 0}",
        f"--terminal-width={width}",
    ]


def starship_prompt(right=False):
    session = getattr(getattr(__xonsh__.shell, "shell", None), "prompter", None)
    _S.begin(session)
    return _S.prompts[right]


@events.on_precommand
@events.on_exit
def _stop(**_):
    _S.stop()


$STARSHIP_SHELL = os.environ["STARSHIP_SHELL"] = "xonsh"
$STARSHIP_SESSION_KEY = os.environ["STARSHIP_SESSION_KEY"] = os.urandom(8).hex()
$PROMPT = starship_prompt
$RIGHT_PROMPT = lambda: starship_prompt(True)
$MULTILINE_PROMPT = _S.render(["--continuation"])
