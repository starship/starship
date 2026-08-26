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

// Generous relative to a lone shell's real startup time: these tests share
// the Test Suite job's CPU with the rest of the coverage-instrumented
// workspace suite (see workflow.yml and .config/nextest.toml) rather than
// running in a dedicated job, so wall-clock delays can run long under
// contention without any real shell or module misbehaving.
const TIMEOUT: Duration = Duration::from_secs(45);
/// How long a shell must draw nothing before it counts as ready for input.
const QUIET: Duration = Duration::from_millis(250);
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
    // Which shell is on the far end. It names the shell in every diagnostic
    // below — with six of these running the same assertions, "never saw FAST"
    // on its own does not say who failed — and it is where `enter` comes from.
    shell: Shell,
    // Set once the shell has closed its side of the pty. Every wait below ends
    // the moment this is true: a shell that has exited will never draw anything
    // again, so sitting out the rest of the timeout only delays the report.
    exited: bool,
}

impl Pty {
    fn spawn(launch: Launch, cwd: &Path, shell: Shell) -> Self {
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
            shell,
            exited: false,
        }
    }

    fn send(&mut self, input: &str) {
        self.pty
            .writer()
            .write_all(input.as_bytes())
            .unwrap_or_else(|error| panic!("failed to write to {}: {error}", self.shell.name()));
        self.pty
            .writer()
            .flush()
            .expect("failed to flush shell input");
    }

    /// Submits whatever has been typed, in the byte the shell's line editor
    /// actually reads as Enter.
    fn enter(&mut self) {
        let enter = self.shell.enter();
        self.send(enter);
    }

    /// Pumps until the shell has drawn something and then gone quiet, so that
    /// input typed next arrives at a line editor that has already claimed the
    /// terminal. Typing into a shell that has not finished starting loses the
    /// keystrokes outright, and the failure that follows looks like a broken
    /// init script rather than the race it is.
    fn settle(&mut self) {
        let deadline = Instant::now() + TIMEOUT;
        let mut previous = self.screen();
        while Instant::now() < deadline && !self.exited {
            self.pump(QUIET);
            let current = self.screen();
            if current == previous && !current.trim().is_empty() {
                return;
            }
            previous = current;
        }
    }

    fn pump(&mut self, timeout: Duration) -> bool {
        let deadline = Instant::now() + timeout;
        let mut bytes = [0; 4096];

        while Instant::now() < deadline {
            match self.pty.reader().read(&mut bytes) {
                Ok(0) => self.exited = true,
                Ok(read) => {
                    self.parser.advance(&mut self.terminal, &bytes[..read]);
                }
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10))
                }
                // Linux, unlike macOS, answers a read on the master with EIO
                // rather than EOF once the shell has exited and closed its
                // side of the pty — the same "nothing more is coming" signal
                // as `Ok(0)`, not a real failure.
                Err(error) if error.raw_os_error() == Some(nix::errno::Errno::EIO as i32) => {
                    self.exited = true;
                }
                Err(error) => {
                    panic!("failed to read {} output: {error}", self.shell.name())
                }
            }

            // Drain every reply the terminal's own query handling (cursor
            // position reports, etc.) has queued so far, regardless of
            // whether this exact iteration's read produced anything. A
            // reply that is only ever sent "when a read happens to also
            // land in the same iteration" can be permanently missed: once
            // the shell blocks waiting for exactly that reply, it stops
            // sending anything else, so there is never another read to
            // trigger a later drain either — a self-inflicted deadlock
            // between this harness and the shell, not a shell bug. This was
            // observed for real: the final `ESC[6n` a shell issues right
            // before redrawing its next prompt, with nothing queued to
            // follow it, went unanswered under the old drain-on-read-only
            // logic and the shell hung forever waiting for it.
            while let Ok(Event::PtyWrite(reply)) = self.events.try_recv() {
                let _ = self.pty.writer().write_all(reply.as_bytes());
            }

            if self.exited {
                return true;
            }
        }

        false
    }

    /// What the shell is showing right now. Assertions match on this alone:
    /// a prompt that refined in place has to be visible, not merely to have
    /// been printed at some point.
    fn screen(&self) -> String {
        self.rows(0)
    }

    /// Everything the shell has drawn, scrollback included — for diagnostics
    /// only. A shell that fails at startup tends to print the reason and then
    /// scroll it away behind whatever it prints next, so a report that showed
    /// only the visible screen would reliably hide the one line worth reading.
    fn transcript(&self) -> String {
        self.rows(self.terminal.grid().topmost_line().0)
    }

    fn rows(&self, first: i32) -> String {
        let grid = self.terminal.grid();
        (first..=grid.bottommost_line().0)
            .map(|row| {
                grid[Line(row)]
                    .into_iter()
                    .map(|cell| cell.c)
                    .collect::<String>()
                    .trim_end()
                    .to_owned()
            })
            .collect::<Vec<_>>()
            .join("\n")
            .trim_matches('\n')
            .to_owned()
    }

    fn wait_for(&mut self, needle: &str) -> String {
        let deadline = Instant::now() + TIMEOUT;
        loop {
            let screen = self.screen();
            if screen.contains(needle) {
                return screen;
            }
            assert!(
                !self.exited,
                "{} exited before {needle:?} appeared; it drew:\n{}",
                self.shell.name(),
                self.transcript(),
            );
            assert!(
                Instant::now() < deadline,
                "{} never drew {needle:?} within {TIMEOUT:?}; it drew:\n{}",
                self.shell.name(),
                self.transcript(),
            );
            self.pump(Duration::from_millis(25));
        }
    }

    fn close(&mut self) {
        self.send("exit");
        self.enter();
        let deadline = Instant::now() + TIMEOUT;
        while !self.pump(Duration::from_millis(25)) {
            assert!(
                Instant::now() < deadline,
                "{} did not exit; it drew:\n{}",
                self.shell.name(),
                self.transcript(),
            );
        }
    }

    fn abandon(&mut self) {
        let _ = self.pty.writer().write_all(b"exit");
        let _ = self.pty.writer().write_all(self.shell.enter().as_bytes());
    }
}

// `PowerShell` trips `enum_variant_names` by ending in the enum's own name.
// These are the shells' real names; spelling one of them differently to please
// the lint would be worse than the lint.
#[allow(clippy::enum_variant_names)]
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

    /// The line the test types. Every shell reports `RESULT:`; the two that
    /// keep their renderer alive inside the shell process also report `LEAKED:`,
    /// the count of streaming machinery left over from the prompt that just
    /// finished, which must be zero.
    const fn input(self) -> &'static str {
        match self {
            Self::Nushell => {
                "print (['RESULT', 'typing-survived'] | str join ':'); let count = job list | where {|job| $job.description | str starts-with starship-stream } | length; print $\"LEAKED:($count)\""
            }
            // One runspace is the shell's own and one is the pump that every
            // prompt of the session re-uses, so anything past two is a stream
            // that was started and never torn down. Counting PowerShell *jobs*
            // here — as this once did — could only ever report zero: the pump
            // deliberately runs on a bare runspace, because a thread job's
            // runspace stalls behind Console.ReadKey on Unix, and so no job is
            // ever created whether the teardown works or not.
            Self::PowerShell => {
                "Write-Output 'RESULT:typing-survived'; Write-Output \"LEAKED:$((Get-Runspace).Count - 2)\""
            }
            Self::Xonsh => "print('RESULT:typing-survived')",
            Self::Zsh | Self::Fish | Self::Bash => "printf 'RESULT:%s\\n' typing-survived",
        }
    }

    /// The byte that submits the typed line. Every other shell's line editor
    /// accepts a bare `\n`; PSReadLine's raw-mode key parser does not treat
    /// LF as Enter at all — confirmed with a bare `pwsh` session that never
    /// even sources Starship — so PowerShell alone needs a literal `\r`.
    const fn enter(self) -> &'static str {
        match self {
            Self::PowerShell => "\r",
            Self::Zsh | Self::Fish | Self::Bash | Self::Nushell | Self::Xonsh => "\n",
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
                vec![
                    "--env-config".into(),
                    fixture.nu_env_config().display().to_string(),
                    "--config".into(),
                    fixture.init.display().to_string(),
                ],
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

fn named_binary(env_var: &str, description: &str) -> PathBuf {
    let path = env::var_os(env_var)
        .map(PathBuf::from)
        .unwrap_or_else(|| panic!("{env_var} must name {description}"));
    assert!(
        path.is_file(),
        "{env_var} is not a file: {}",
        path.display()
    );
    path
}

fn ble() -> PathBuf {
    named_binary("BLE_SH", "ble.sh")
}

fn nushell() -> PathBuf {
    named_binary("NUSHELL_MAIN", "the Nushell main binary")
}

fn powershell() -> PathBuf {
    named_binary("POWERSHELL_MAIN", "the PowerShell binary")
}

fn xonsh() -> PathBuf {
    named_binary("XONSH_MAIN", "the Xonsh binary")
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
format = "${custom.fast}${custom.slow}$line_break$character"
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

        let init = directory
            .path()
            .join(if matches!(shell, Shell::PowerShell) {
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

    /// nushell parses its env config separately from the main config the init
    /// occupies, so turning the banner off from here survives an init that
    /// fails to parse — and the banner is a dozen lines of art that would
    /// otherwise scroll that very parse error off a twenty-four line screen.
    /// Naming the file also stops nushell from writing a default one and
    /// announcing that it did.
    fn nu_env_config(&self) -> PathBuf {
        let config = self.directory.path().join("env.nu");
        fs::write(&config, "$env.config.show_banner = false\n")
            .expect("failed to write nushell env config");
        config
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
        let mut pty = Pty::spawn(launch, fixture.directory.path(), shell);
        if let Startup::Source(command) = startup {
            pty.settle();
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
        "{}'s first paint waited for SLOW:\n{first}",
        shell.name()
    );

    let input = shell.input();
    if matches!(shell, Shell::PowerShell) {
        // The PowerShell pump only writes Rust's cursor-neutral repaint while
        // PSReadLine's input buffer is empty, so the refinement lands before
        // typing begins — and typed input can never be clobbered by a repaint
        // at all.
        session.pty.wait_for("SLOW");
        session.pty.send(input);
    } else {
        session.pty.send(input);
        let refined = session.pty.wait_for("SLOW");
        assert!(
            refined.replace('\n', "").contains(input),
            "{}'s refinement lost input:\n{refined}",
            shell.name()
        );
    }

    session.pty.enter();
    session.pty.wait_for("RESULT:typing-survived");
    if matches!(shell, Shell::Nushell | Shell::PowerShell) {
        session.pty.wait_for("LEAKED:0");
    }
    session.close();
}

/// Regression test: `right_format` must stream and repaint on its own, the
/// same as `format` does — never recomputed once per prompt draw via a
/// blocking synchronous `starship prompt --right` call. A live clock placed
/// there advances with no input sent, proving the stream is running.
///
/// Every shell that draws a right prompt at all drives an independent stream
/// for it, and every one of those is covered below. PowerShell is the omission
/// to not go looking for: it has no right prompt to stream, since PSReadLine
/// offers no hook to place one.
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

/// Regression test: a prompt that opens with a blank line — what `add_newline`
/// produces, and the default — must survive the wire intact.
///
/// The blank line is the dangerous one. It is an empty field line sitting right
/// where a field terminator would be, and a reader that compares it loosely
/// takes it for the terminator and returns an empty prompt: PowerShell's `-eq`
/// is culture-sensitive, a NUL collates as nothing, and so `'' -eq "`0"` is
/// true. Every other config in this file sets `add_newline = false`, so nothing
/// else here would ever notice.
fn a_leading_blank_line_survives(shell: Shell) {
    let mut session = Session::start_with_config(
        shell,
        r#"
format = "${custom.fast}${custom.slow}$line_break$character"
add_newline = true

[custom.fast]
command = "printf FAST"
when = true
format = "[$output]($style) "
style = ""
shell = ["/bin/sh"]

[custom.slow]
command = "sleep 2; printf SLOW"
when = true
format = "[$output]($style) "
style = ""
shell = ["/bin/sh"]
ignore_timeout = true

[character]
success_symbol = ">"
"#,
    );

    // The first paint must carry the prompt, not an empty string truncated at
    // the leading blank line, and the refinement must still land on top of it.
    session.pty.wait_for("FAST");
    session.pty.wait_for("SLOW");
    session.close();
}

/// Regression test: a vi-mode change must repaint the prompt with the new mode
/// indicator. Only `character` reads `--keymap`, and a renderer is told one
/// through argv and nothing else, so the indicator can only change if a fresh
/// stream ran — and the shell must pick that stream up without blocking on its
/// handshake, which is what makes Esc cheap enough to hold down.
fn vi_mode_repaints_the_indicator(shell: Shell) {
    let mut session = Session::start_with_config(
        shell,
        r#"
format = "MARK$character"
add_newline = false

[character]
success_symbol = "INSERT"
vicmd_symbol = "NORMAL"
"#,
    );

    session.pty.wait_for("INSERT");
    session.pty.send("bindkey -v");
    session.pty.enter();
    session.pty.wait_for("INSERT");

    // Escape leaves insert mode, so the next paint has to say NORMAL.
    session.pty.send("\x1b");
    session.pty.wait_for("NORMAL");

    // Back to insert, or `close`'s "exit" is read as vi command-mode keys.
    session.pty.send("i");
    session.pty.wait_for("INSERT");
    session.close();
}

// Every shell above gets the same pair of `streams`/`right_prompt_ticks_on_its_own`
// wrappers (only the `#[ignore]` reason and, for PowerShell, the coverage
// differ), so generate them instead of hand-copying eleven near-identical
// stanzas — a future shell added to just one list would otherwise be an easy
// omission to miss in the other.
macro_rules! shell_test {
    ($name:ident, $body:ident, $shell:expr, $reason:literal) => {
        #[test]
        #[ignore = $reason]
        fn $name() {
            $body($shell);
        }
    };
}

shell_test!(
    zsh_streams_a_live_prompt,
    streams,
    Shell::Zsh,
    "requires zsh"
);
shell_test!(
    fish_streams_a_live_prompt,
    streams,
    Shell::Fish,
    "requires fish"
);
shell_test!(
    bash_ble_streams_a_live_prompt,
    streams,
    Shell::Bash,
    "requires BLE_SH and Bash 4+"
);
shell_test!(
    powershell_streams_a_live_prompt,
    streams,
    Shell::PowerShell,
    "requires POWERSHELL_MAIN"
);
shell_test!(
    nushell_streams_a_live_prompt,
    streams,
    Shell::Nushell,
    "requires NUSHELL_MAIN"
);
shell_test!(
    xonsh_streams_a_live_prompt,
    streams,
    Shell::Xonsh,
    "requires XONSH_MAIN"
);

shell_test!(
    zsh_a_leading_blank_line_survives,
    a_leading_blank_line_survives,
    Shell::Zsh,
    "requires zsh"
);

shell_test!(
    fish_a_leading_blank_line_survives,
    a_leading_blank_line_survives,
    Shell::Fish,
    "requires fish"
);

shell_test!(
    bash_ble_a_leading_blank_line_survives,
    a_leading_blank_line_survives,
    Shell::Bash,
    "requires BLE_SH and Bash 4+"
);

shell_test!(
    powershell_a_leading_blank_line_survives,
    a_leading_blank_line_survives,
    Shell::PowerShell,
    "requires POWERSHELL_MAIN"
);

shell_test!(
    nushell_a_leading_blank_line_survives,
    a_leading_blank_line_survives,
    Shell::Nushell,
    "requires NUSHELL_MAIN"
);

shell_test!(
    xonsh_a_leading_blank_line_survives,
    a_leading_blank_line_survives,
    Shell::Xonsh,
    "requires XONSH_MAIN"
);

shell_test!(
    zsh_vi_mode_repaints_the_indicator,
    vi_mode_repaints_the_indicator,
    Shell::Zsh,
    "requires zsh"
);

shell_test!(
    zsh_right_prompt_ticks_on_its_own,
    right_prompt_ticks_on_its_own,
    Shell::Zsh,
    "requires zsh"
);
shell_test!(
    fish_right_prompt_ticks_on_its_own,
    right_prompt_ticks_on_its_own,
    Shell::Fish,
    "requires fish"
);
shell_test!(
    bash_ble_right_prompt_ticks_on_its_own,
    right_prompt_ticks_on_its_own,
    Shell::Bash,
    "requires BLE_SH and Bash 4+"
);
shell_test!(
    nushell_right_prompt_ticks_on_its_own,
    right_prompt_ticks_on_its_own,
    Shell::Nushell,
    "requires NUSHELL_MAIN"
);
shell_test!(
    xonsh_right_prompt_ticks_on_its_own,
    right_prompt_ticks_on_its_own,
    Shell::Xonsh,
    "requires XONSH_MAIN"
);
