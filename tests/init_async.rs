#![cfg(unix)]
#![allow(clippy::disallowed_methods)] // Integration tests must launch real shells.

use alacritty_terminal::event::{Event, EventListener, WindowSize};
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::Line;
use alacritty_terminal::term::{Config, Term};
use alacritty_terminal::tty::{self, EventedReadWrite, Options, Shell as PtyShell};
use alacritty_terminal::vte::ansi::Processor;
use std::collections::HashMap;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

const TIMEOUT: Duration = Duration::from_secs(15);
const COLUMNS: usize = 160;
const LINES: usize = 24;

#[derive(Clone)]
struct Listener(mpsc::Sender<Event>);

impl EventListener for Listener {
    fn send_event(&self, event: Event) {
        let _ = self.0.send(event);
    }
}

struct Size;

impl Dimensions for Size {
    fn total_lines(&self) -> usize {
        LINES
    }

    fn screen_lines(&self) -> usize {
        LINES
    }

    fn columns(&self) -> usize {
        COLUMNS
    }
}

struct Pty {
    pty: tty::Pty,
    parser: Processor,
    terminal: Term<Listener>,
    events: mpsc::Receiver<Event>,
}

impl Pty {
    fn spawn(launch: Launch, cwd: &Path) -> Self {
        let pty = tty::new(
            &Options {
                shell: Some(PtyShell::new(
                    launch.program.to_str().expect("shell path is UTF-8").into(),
                    launch.args,
                )),
                working_directory: Some(cwd.into()),
                drain_on_exit: false,
                env: launch.environment,
            },
            WindowSize {
                num_lines: LINES as u16,
                num_cols: COLUMNS as u16,
                cell_width: 8,
                cell_height: 16,
            },
            0,
        )
        .unwrap_or_else(|error| panic!("failed to start {}: {error}", launch.program.display()));
        let (sender, events) = mpsc::channel();

        Self {
            pty,
            parser: Processor::new(),
            terminal: Term::new(Config::default(), &Size, Listener(sender)),
            events,
        }
    }

    fn send(&mut self, input: &str) {
        self.pty
            .writer()
            .write_all(input.as_bytes())
            .unwrap_or_else(|error| panic!("failed to write to shell: {error}"));
        self.pty
            .writer()
            .flush()
            .expect("failed to flush shell input");
    }

    fn pump(&mut self, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;
        let mut bytes = [0; 4096];

        while Instant::now() < deadline {
            match self.pty.reader().read(&mut bytes) {
                Ok(0) => return true,
                Ok(read) => {
                    self.parser.advance(&mut self.terminal, &bytes[..read]);
                    while let Ok(Event::PtyWrite(reply)) = self.events.try_recv() {
                        let _ = self.pty.writer().write_all(reply.as_bytes());
                    }
                }
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10))
                }
                // Linux, unlike macOS, answers a read on the master with EIO
                // rather than EOF once the shell has exited and closed its
                // side of the pty — the same "nothing more is coming" signal
                // as `Ok(0)`, not a real failure.
                Err(error) if error.raw_os_error() == Some(nix::errno::Errno::EIO as i32) => {
                    return true;
                }
                Err(error) => panic!("failed to read shell output: {error}"),
            }
        }

        false
    }

    fn screen(&self) -> String {
        let grid = self.terminal.grid();
        (0..grid.screen_lines())
            .map(|row| {
                let line = grid[Line(row as i32)]
                    .into_iter()
                    .map(|cell| cell.c)
                    .collect::<String>();
                line.trim_end().to_owned()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn wait_for(&mut self, needle: &str) -> String {
        let deadline = Instant::now() + TIMEOUT;
        loop {
            let screen = self.screen();
            if screen.contains(needle) {
                return screen;
            }
            assert!(
                Instant::now() < deadline,
                "never saw {needle:?}; screen was:\n{screen}"
            );
            self.pump(Duration::from_millis(25));
        }
    }

    fn close(&mut self) {
        self.send("exit\n");
        let deadline = Instant::now() + TIMEOUT;
        while !self.pump(Duration::from_millis(25)) {
            assert!(
                Instant::now() < deadline,
                "shell did not exit; screen was:\n{}",
                self.screen(),
            );
        }
    }

    fn abandon(&mut self) {
        let _ = self.pty.writer().write_all(b"exit\n");
    }
}

#[derive(Clone, Copy)]
enum Shell {
    Zsh,
    Fish,
    Bash,
    PowerShell,
    Nushell,
    Xonsh,
}

impl Shell {
    const fn name(self) -> &'static str {
        match self {
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::Bash => "bash",
            Self::PowerShell => "powershell",
            Self::Nushell => "nu",
            Self::Xonsh => "xonsh",
        }
    }

    const fn input(self) -> &'static str {
        match self {
            Self::Nushell => {
                "print (['RESULT', 'typing-survived'] | str join ':'); let count = job list | where {|job| $job.description | str starts-with starship-stream } | length; print $\"JOB_COUNT:($count)\""
            }
            Self::PowerShell => {
                "Write-Output 'RESULT:typing-survived'; Write-Output \"JOB_COUNT:$(@(Get-Job | Where-Object Name -Like 'Starship.Stream.*').Count)\""
            }
            Self::Xonsh => "print('RESULT:typing-survived')",
            Self::Zsh | Self::Fish | Self::Bash => "printf 'RESULT:%s\\n' typing-survived",
        }
    }

    fn launch(self, fixture: &Fixture) -> Launch {
        let mut environment = fixture.environment();
        let (program, args, startup) = match self {
            Self::Zsh => (
                PathBuf::from("zsh"),
                vec!["-f".into(), "-i".into()],
                Startup::Source("source \"$STARSHIP_INIT\"\n"),
            ),
            Self::Fish => {
                environment.insert("fish_features".into(), "no-query-term".into());
                (
                    PathBuf::from("fish"),
                    vec!["--interactive".into()],
                    Startup::Source("source \"$STARSHIP_INIT\"\n"),
                )
            }
            Self::Bash => {
                environment.insert("BLE_SH".into(), ble().display().to_string());
                (
                    bash(),
                    vec![
                        "--noprofile".into(),
                        "--rcfile".into(),
                        fixture.bashrc().display().to_string(),
                        "-i".into(),
                    ],
                    Startup::Ready,
                )
            }
            Self::PowerShell => (
                powershell(),
                vec![
                    "-NoLogo".into(),
                    "-NoProfile".into(),
                    "-NoExit".into(),
                    "-Command".into(),
                    ". $env:STARSHIP_INIT".into(),
                ],
                Startup::Ready,
            ),
            Self::Nushell => (
                nushell(),
                vec!["--config".into(), fixture.init.display().to_string()],
                Startup::Ready,
            ),
            Self::Xonsh => (
                xonsh(),
                vec![
                    "--interactive".into(),
                    "--shell-type=prompt_toolkit".into(),
                    "--rc".into(),
                    fixture.init.display().to_string(),
                ],
                Startup::Ready,
            ),
        };

        Launch {
            program,
            args,
            startup,
            environment,
        }
    }
}

#[derive(Clone, Copy)]
enum Startup {
    Source(&'static str),
    Ready,
}

struct Launch {
    program: PathBuf,
    args: Vec<String>,
    startup: Startup,
    environment: HashMap<String, String>,
}

fn bash() -> PathBuf {
    let path = env::var_os("BASH")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/bin/bash"));
    let output = Command::new(&path)
        .arg("--version")
        .output()
        .unwrap_or_else(|error| panic!("BASH must name Bash 4+: {}: {error}", path.display()));
    let major = String::from_utf8_lossy(&output.stdout)
        .split(|character: char| !character.is_ascii_digit())
        .find_map(|number| number.parse::<u32>().ok());
    assert!(
        output.status.success() && major.is_some_and(|major| major >= 4),
        "BASH must name Bash 4+: {}",
        path.display()
    );
    path
}

fn ble() -> PathBuf {
    let path = env::var_os("BLE_SH")
        .map(PathBuf::from)
        .expect("BLE_SH must name ble.sh");
    assert!(path.is_file(), "BLE_SH is not a file: {}", path.display());
    path
}

fn nushell() -> PathBuf {
    let path = env::var_os("NUSHELL_MAIN")
        .map(PathBuf::from)
        .expect("NUSHELL_MAIN must name the Nushell main binary");
    assert!(
        path.is_file(),
        "NUSHELL_MAIN is not a file: {}",
        path.display()
    );
    path
}

fn powershell() -> PathBuf {
    let path = env::var_os("POWERSHELL_MAIN")
        .map(PathBuf::from)
        .expect("POWERSHELL_MAIN must name the PowerShell binary");
    assert!(
        path.is_file(),
        "POWERSHELL_MAIN is not a file: {}",
        path.display()
    );
    path
}

fn xonsh() -> PathBuf {
    let path = env::var_os("XONSH_MAIN")
        .map(PathBuf::from)
        .expect("XONSH_MAIN must name the Xonsh binary");
    assert!(
        path.is_file(),
        "XONSH_MAIN is not a file: {}",
        path.display()
    );
    path
}

struct Fixture {
    directory: TempDir,
    init: PathBuf,
}

impl Fixture {
    fn new(shell: Shell) -> Self {
        Self::with_config(
            shell,
            r#"
format = "${custom.fast}${custom.slow}$character"
add_newline = false

[custom.fast]
command = "printf FAST"
when = true
format = "[$output]($style) "
style = "red"
shell = ["/bin/sh"]

[custom.slow]
command = "sleep 2; printf SLOW"
when = true
format = "[$output]($style) "
style = "red"
shell = ["/bin/sh"]
ignore_timeout = true

[character]
success_symbol = ">"
"#,
        )
    }

    fn with_config(shell: Shell, config: &str) -> Self {
        let directory = tempfile::tempdir().expect("failed to create scratch directory");
        fs::write(directory.path().join("starship.toml"), config).expect("failed to write config");

        let init = directory.path().join(if matches!(shell, Shell::PowerShell) {
            "starship-init.ps1"
        } else {
            "starship-init"
        });
        let mut command = Command::new(starship());
        command
            .args(["init", shell.name(), "--print-full-init"])
            .env("PATH", path_with_starship_first());
        let output = command
            .output()
            .unwrap_or_else(|error| panic!("failed to emit {} init: {error}", shell.name()));
        assert!(
            output.status.success(),
            "failed to emit {} init: {}",
            shell.name(),
            String::from_utf8_lossy(&output.stderr),
        );
        fs::write(&init, output.stdout).expect("failed to write shell init");

        Self { directory, init }
    }

    fn bashrc(&self) -> PathBuf {
        let bashrc = self.directory.path().join("bashrc");
        fs::write(&bashrc, "source \"$BLE_SH\"\nsource \"$STARSHIP_INIT\"\n")
            .expect("failed to write Bash rcfile");
        bashrc
    }

    fn environment(&self) -> HashMap<String, String> {
        let xdg = self.directory.path().join("xdg");
        fs::create_dir_all(xdg.join("fish")).expect("failed to create fish XDG directory");

        let mut environment = HashMap::from([
            (
                "STARSHIP_CONFIG".into(),
                self.directory
                    .path()
                    .join("starship.toml")
                    .display()
                    .to_string(),
            ),
            ("STARSHIP_INIT".into(), self.init.display().to_string()),
            ("TERM".into(), "xterm-256color".into()),
            ("PS1".into(), "$ ".into()),
            ("HOME".into(), self.directory.path().display().to_string()),
            ("XDG_CONFIG_HOME".into(), xdg.display().to_string()),
            ("XDG_DATA_HOME".into(), xdg.display().to_string()),
        ]);
        if let Some(path) = env::var_os("PATH") {
            environment.insert("PATH".into(), path.display().to_string());
        }
        if let Some(trace) = env::var_os("STARSHIP_STREAM_TRACE") {
            environment.insert("STARSHIP_STREAM_TRACE".into(), trace.display().to_string());
        }
        environment
    }
}

fn starship() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_starship"))
}

fn path_with_starship_first() -> OsString {
    let bin_directory = starship()
        .parent()
        .expect("the Starship test binary has a parent directory");
    let inherited = env::var_os("PATH")
        .map(|path| env::split_paths(&path).collect::<Vec<_>>())
        .unwrap_or_default();
    env::join_paths(std::iter::once(bin_directory.to_path_buf()).chain(inherited))
        .expect("PATH entries contain no path separator")
}

struct Session {
    pty: Pty,
    _fixture: Fixture,
    closed: bool,
}

impl Session {
    fn start(shell: Shell) -> Self {
        Self::start_with_fixture(shell, Fixture::new(shell))
    }

    fn start_with_config(shell: Shell, config: &str) -> Self {
        Self::start_with_fixture(shell, Fixture::with_config(shell, config))
    }

    fn start_with_fixture(shell: Shell, fixture: Fixture) -> Self {
        let launch = shell.launch(&fixture);
        let startup = launch.startup;
        let mut pty = Pty::spawn(launch, fixture.directory.path());
        if let Startup::Source(command) = startup {
            pty.send(command);
        }
        Self {
            pty,
            _fixture: fixture,
            closed: false,
        }
    }

    fn close(&mut self) {
        self.pty.close();
        self.closed = true;
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.closed {
            return;
        }
        self.pty.abandon();
        self.pty.pump(Duration::from_millis(100));
    }
}

fn streams(shell: Shell) {
    let mut session = Session::start(shell);
    let first = session.pty.wait_for("FAST");
    assert!(
        !first.contains("SLOW"),
        "first paint waited for SLOW:\n{first}"
    );

    let input = shell.input();
    let refined = session.pty.wait_for("SLOW");
    session.pty.send(input);
    assert!(
        refined.replace('\n', "").contains(input),
        "refinement lost input:\n{refined}"
    );

    session.pty.send("\n");
    session.pty.wait_for("RESULT:typing-survived");
    if matches!(shell, Shell::Nushell | Shell::PowerShell) {
        session.pty.wait_for("JOB_COUNT:0");
    }
    session.close();
}

#[test]
#[ignore = "requires zsh"]
fn zsh_streams_a_live_prompt() {
    streams(Shell::Zsh);
}

/// Regression test: `right_format` must stream and repaint on its own, the
/// same as `format` does — never recomputed once per prompt draw via a
/// blocking synchronous `starship prompt --right` call. Every refinable shell
/// drives an independent right-prompt stream. A live clock placed there
/// advances with no input sent, proving the stream is running.
fn right_prompt_ticks_on_its_own(shell: Shell) {
    let mut session = Session::start_with_config(
        shell,
        r#"
format = "MARK$character"
add_newline = false

right_format = "$time"

[character]
success_symbol = ">"

[time]
disabled = false
format = "[$time]($style)"
style = ""
time_format = "%S%.6f"

[async.dynamic]
time = 30
"#,
    );

    session.pty.wait_for("MARK");
    session.pty.pump(Duration::from_millis(50));
    let first = session.pty.screen();

    let deadline = Instant::now() + TIMEOUT;
    loop {
        session.pty.pump(Duration::from_millis(25));
        let later = session.pty.screen();
        if later != first {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "right prompt never changed on its own, with no input sent:\n{first}"
        );
    }

    session.close();
}

#[test]
#[ignore = "requires zsh"]
fn zsh_right_prompt_ticks_on_its_own() {
    right_prompt_ticks_on_its_own(Shell::Zsh);
}

#[test]
#[ignore = "requires fish"]
fn fish_right_prompt_ticks_on_its_own() {
    right_prompt_ticks_on_its_own(Shell::Fish);
}

#[test]
#[ignore = "requires BLE_SH and Bash 4+"]
fn bash_ble_right_prompt_ticks_on_its_own() {
    right_prompt_ticks_on_its_own(Shell::Bash);
}

#[test]
#[ignore = "requires fish"]
fn fish_streams_a_live_prompt() {
    streams(Shell::Fish);
}

#[test]
#[ignore = "requires BLE_SH and Bash 4+"]
fn bash_ble_streams_a_live_prompt() {
    streams(Shell::Bash);
}

#[test]
#[ignore = "requires POWERSHELL_MAIN"]
fn powershell_streams_a_live_prompt() {
    streams(Shell::PowerShell);
}

#[test]
#[ignore = "requires NUSHELL_MAIN"]
fn nushell_streams_a_live_prompt() {
    streams(Shell::Nushell);
}

#[test]
#[ignore = "requires XONSH_MAIN"]
fn xonsh_streams_a_live_prompt() {
    streams(Shell::Xonsh);
}

#[test]
#[ignore = "requires NUSHELL_MAIN"]
fn nushell_right_prompt_ticks_on_its_own() {
    right_prompt_ticks_on_its_own(Shell::Nushell);
}

#[test]
#[ignore = "requires XONSH_MAIN"]
fn xonsh_right_prompt_ticks_on_its_own() {
    right_prompt_ticks_on_its_own(Shell::Xonsh);
}
