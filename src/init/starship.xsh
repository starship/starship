import json
import shutil
import subprocess
import threading
import time
import uuid

from xonsh.events import events


STARSHIP = ::STARSHIP::


def _starship_arguments():
    last = __xonsh__.history[-1] if __xonsh__.history else None
    jobs = sum(
        job.get("obj") is not None and job["obj"].poll() is None
        for job in __xonsh__.all_jobs.values()
    )
    duration = round((last.ts[1] - last.ts[0]) * 1000) if last else 0
    return [
        f"--status={last.rtn if last else 0}",
        f"--jobs={jobs}",
        f"--cmd-duration={duration}",
        f"--terminal-width={shutil.get_terminal_size().columns}",
    ]


def _starship_run(command):
    try:
        return subprocess.check_output(
            [STARSHIP, *command],
            stderr=subprocess.DEVNULL,
            text=True,
            env=__xonsh__.env.detype(),
        )
    except (OSError, subprocess.CalledProcessError):
        return ""


class _StarshipStream:
    __slots__ = "flag", "prompt", "process", "right", "timings"

    def __init__(self, right):
        self.right = right
        self.flag = ["--right"] if right else []
        self.prompt = self.timings = ""
        self.process = None

    def render(self, arguments):
        return _starship_run(["prompt", *self.flag, *arguments])

    def stop(self):
        process, self.process = self.process, None
        if process is not None and process.poll() is None:
            process.kill()

    def publish(self, prompt, process, session):
        if self.process is not process:
            return
        self.prompt = prompt
        app = session.app
        while self.process is process and not app.is_running:
            time.sleep(0.001)
        if self.process is not process:
            return

        from prompt_toolkit import ANSI

        formatted = ANSI(prompt)

        def apply():
            if self.process is process:
                setattr(session, "rprompt" if self.right else "message", formatted)
                app.invalidate()

        try:
            app.loop.call_soon_threadsafe(apply)
        except RuntimeError:
            pass

    def read(self, arguments, process, session):
        complete = False
        try:
            for line in process.stdout:
                frame = json.loads(line)
                if frame["kind"] == "PATCH":
                    self.publish(frame["prompt"], process, session)
                elif frame["kind"] == "COMPLETE":
                    complete = True
                    if self.process is process:
                        self.timings = json.dumps(
                            frame["timings"], separators=(",", ":")
                        )
        except (OSError, ValueError):
            pass

        if not complete:
            self.publish(self.render(arguments), process, session)

    def start(self, arguments, session):
        self.stop()
        process = None
        try:
            process = subprocess.Popen(
                [
                    STARSHIP,
                    "stream",
                    "--frames=json",
                    *self.flag,
                    *arguments,
                    f"--timings={self.timings}",
                ],
                stdout=subprocess.PIPE,
                stderr=subprocess.DEVNULL,
                text=True,
                bufsize=1,
                env=__xonsh__.env.detype(),
            )
            ready = json.loads(process.stdout.readline())
            if ready["kind"] != "READY":
                raise ValueError("starship stream did not become ready")
        except (OSError, ValueError):
            if process is not None and process.poll() is None:
                process.kill()
            return self.render(arguments)

        self.process, self.prompt = process, ready["prompt"]
        threading.Thread(
            target=self.read,
            args=(arguments, process, session),
            name=f"starship-{'right' if self.right else 'left'}",
            daemon=True,
        ).start()
        return self.prompt


_STARSHIP_STREAMS = (_StarshipStream(False), _StarshipStream(True))
_STARSHIP_ARGUMENTS = None


def starship_prompt(right=False):
    global _STARSHIP_ARGUMENTS
    if not right:
        _STARSHIP_ARGUMENTS = _starship_arguments()
    stream = _STARSHIP_STREAMS[right]
    arguments = _STARSHIP_ARGUMENTS or _starship_arguments()
    session = getattr(getattr(__xonsh__.shell, "shell", None), "prompter", None)
    return stream.start(arguments, session) if session else stream.render(arguments)


def _starship_stop(**_):
    global _STARSHIP_ARGUMENTS
    _STARSHIP_ARGUMENTS = None
    for stream in _STARSHIP_STREAMS:
        stream.stop()


events.on_precommand(_starship_stop)
events.on_exit(_starship_stop)
__xonsh__.env.update(
    STARSHIP_SHELL="xonsh",
    STARSHIP_SESSION_KEY=uuid.uuid4().hex[:16],
    PROMPT=starship_prompt,
    RIGHT_PROMPT=lambda: starship_prompt(True),
    MULTILINE_PROMPT=_starship_run(["prompt", "--continuation"]),
)
