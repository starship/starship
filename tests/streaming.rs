//! Streamed prompts, driven through real shells in a pseudo-terminal: a
//! correct frame does not by itself make a correct redraw, which is up to the
//! shell's line editor.
#![cfg(unix)]
// The lint keeps a program run by name from being found in the working
// directory; these tests run starship by its absolute path.
#![expect(clippy::disallowed_methods)]

use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, PoisonError, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use alacritty_terminal::event::{Event, EventListener, WindowSize};
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::Line;
use alacritty_terminal::term::{Config, Term};
use alacritty_terminal::tty::{self, EventedReadWrite, Options, Shell as Program};
use alacritty_terminal::vte::ansi::Processor;
use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use tempfile::TempDir;

/// Generous, because these tests share a busy machine with the rest of the
/// suite, and a starved shell is slow without being wrong.
const TIMEOUT: Duration = Duration::from_secs(45);
/// How long a shell has to draw nothing to count as waiting for input.
const QUIET: Duration = Duration::from_millis(250);
const COLUMNS: usize = 160;
const LINES: usize = 24;

/// One shell at a time: several at once can starve each other into timing out.
static ONE_SHELL_AT_A_TIME: Mutex<()> = Mutex::new(());

#[derive(Clone, Copy)]
enum Shell {
    Zsh,
}

impl Shell {
    const fn name(self) -> &'static str {
        match self {
            Self::Zsh => "zsh",
        }
    }

    /// The shell's program: the one `STARSHIP_TEST_<NAME>` names, such as
    /// `STARSHIP_TEST_ZSH`, or else the one on `PATH`.
    fn program(self) -> PathBuf {
        std::env::var_os(format!("STARSHIP_TEST_{}", self.name().to_uppercase()))
            .map_or_else(|| PathBuf::from(self.name()), PathBuf::from)
    }

    /// A command line that prints `RESULT:typed`, to type while a prompt is
    /// being refined.
    const fn input(self) -> &'static str {
        match self {
            Self::Zsh => "printf 'RESULT:%s\\n' typed",
        }
    }

    /// A command line that waits until the prompt it was typed at is long
    /// finished, then prints `RESULT:typed`.
    const fn sleep_then_input(self) -> &'static str {
        match self {
            Self::Zsh => "sleep 3; printf 'RESULT:%s\\n' typed",
        }
    }

    /// The shell's arguments, and the command that then loads starship, if
    /// they do not.
    fn arguments(self) -> (Vec<String>, Option<&'static str>) {
        let source = Some("source \"$STARSHIP_INIT\"\n");
        match self {
            Self::Zsh => (vec!["-f".into(), "-i".into()], source),
        }
    }

    /// The arguments that make starship print the shell's whole init script.
    const fn init_arguments(self) -> &'static [&'static str] {
        match self {
            Self::Zsh => &["init", "zsh", "--print-full-init"],
        }
    }

    /// `PATH`, with starship and this shell first, where init scripts look.
    fn search_path(self) -> OsString {
        let program = self.program();
        let first = [starship().parent(), program.parent()];
        let inherited = std::env::var_os("PATH").unwrap_or_default();
        std::env::join_paths(
            first
                .into_iter()
                .flatten()
                .filter(|directory| !directory.as_os_str().is_empty())
                .map(Path::to_path_buf)
                .chain(std::env::split_paths(&inherited)),
        )
        .expect("no directory on PATH holds a separator")
    }
}

/// A scratch directory holding a configuration and the shell's init script.
struct Fixture {
    directory: TempDir,
    init: PathBuf,
}

impl Fixture {
    fn new(shell: Shell, config: &str) -> Self {
        let directory = tempfile::tempdir().expect("a scratch directory");
        fs::write(directory.path().join("starship.toml"), config).expect("a configuration");
        let output = Command::new(starship())
            .args(shell.init_arguments())
            .env("PATH", shell.search_path())
            .output()
            .expect("starship prints its init script");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let init = directory.path().join("starship-init");
        fs::write(&init, output.stdout).expect("an init script");
        Self { directory, init }
    }

    fn environment(&self, shell: Shell) -> HashMap<String, String> {
        let directory = self.directory.path();
        HashMap::from([
            ("HOME".into(), directory.display().to_string()),
            ("PATH".into(), shell.search_path().display().to_string()),
            (
                "STARSHIP_CONFIG".into(),
                directory.join("starship.toml").display().to_string(),
            ),
            ("STARSHIP_INIT".into(), self.init.display().to_string()),
            ("TERM".into(), "xterm-256color".into()),
            ("XDG_CONFIG_HOME".into(), directory.display().to_string()),
            ("XDG_DATA_HOME".into(), directory.display().to_string()),
        ])
    }
}

fn starship() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_starship"))
}

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

/// A shell running in a pseudo-terminal, and the screen it has drawn.
struct Session {
    shell: Shell,
    pty: tty::Pty,
    parser: Processor,
    terminal: Term<Listener>,
    events: mpsc::Receiver<Event>,
    exited: bool,
    closed: bool,
    _fixture: Fixture,
    _turn: MutexGuard<'static, ()>,
}

impl Session {
    fn start(shell: Shell, config: &str) -> Self {
        let turn = ONE_SHELL_AT_A_TIME
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let fixture = Fixture::new(shell, config);
        let (arguments, load) = shell.arguments();
        let pty = tty::new(
            &Options {
                shell: Some(Program::new(
                    shell.program().display().to_string(),
                    arguments,
                )),
                working_directory: Some(fixture.directory.path().into()),
                drain_on_exit: false,
                env: fixture.environment(shell),
            },
            WindowSize {
                num_lines: LINES as u16,
                num_cols: COLUMNS as u16,
                cell_width: 8,
                cell_height: 16,
            },
            0,
        )
        .unwrap_or_else(|error| panic!("unable to start {}: {error}", shell.name()));
        let (sender, events) = mpsc::channel();
        let mut session = Self {
            shell,
            pty,
            parser: Processor::new(),
            terminal: Term::new(Config::default(), &Size, Listener(sender)),
            events,
            exited: false,
            closed: false,
            _fixture: fixture,
            _turn: turn,
        };
        if let Some(load) = load {
            session.settle();
            session.send(load);
        }
        session
    }

    fn send(&mut self, input: &str) {
        let writer = self.pty.writer();
        writer
            .write_all(input.as_bytes())
            .and_then(|()| writer.flush())
            .unwrap_or_else(|error| panic!("unable to type into {}: {error}", self.shell.name()));
    }

    /// Presses Enter, which a terminal sends as a carriage return.
    fn enter(&mut self) {
        self.send("\r");
    }

    /// Waits for the shell to draw something and then go quiet, so that what
    /// is typed next reaches a line editor that is ready for it.
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

    /// Reads what the shell draws for `timeout`, answering its queries.
    fn pump(&mut self, timeout: Duration) {
        let deadline = Instant::now() + timeout;
        let mut bytes = [0; 4096];
        while Instant::now() < deadline && !self.exited {
            match self.pty.reader().read(&mut bytes) {
                Ok(0) => self.exited = true,
                Ok(read) => self.parser.advance(&mut self.terminal, &bytes[..read]),
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                // Linux reports an exited shell as an error rather than as the
                // end of the stream.
                Err(error) if error.raw_os_error() == Some(nix::errno::Errno::EIO as i32) => {
                    self.exited = true;
                }
                Err(error) => panic!("unable to read from {}: {error}", self.shell.name()),
            }
            // A line editor that asks where the cursor is waits for the answer,
            // whether or not anything else has been read in the meantime.
            while let Ok(Event::PtyWrite(reply)) = self.events.try_recv() {
                let _ = self.pty.writer().write_all(reply.as_bytes());
            }
        }
    }

    /// The visible screen, which is what assertions are about.
    fn screen(&self) -> String {
        self.rows(0)
    }

    /// Everything drawn, scrollback included, to explain a failure.
    fn transcript(&self) -> String {
        self.rows(self.terminal.grid().topmost_line().0)
    }

    fn rows(&self, first: i32) -> String {
        let grid = self.terminal.grid();
        (first..=grid.bottommost_line().0)
            .map(|row| {
                let text: String = grid[Line(row)].into_iter().map(|cell| cell.c).collect();
                text.trim_end().to_owned()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn wait_for(&mut self, text: &str) -> String {
        let deadline = Instant::now() + TIMEOUT;
        loop {
            let screen = self.screen();
            if screen.contains(text) {
                return screen;
            }
            assert!(
                !self.exited && Instant::now() < deadline,
                "{} never drew {text:?}; it drew:\n{}",
                self.shell.name(),
                self.transcript(),
            );
            self.pump(Duration::from_millis(25));
        }
    }

    fn close(mut self) {
        self.send("exit");
        // Typed text and an Enter that arrive together read as a paste to some
        // line editors, which then insert a newline rather than run the line.
        self.wait_for("exit");
        self.enter();
        let deadline = Instant::now() + TIMEOUT;
        while !self.exited {
            assert!(
                Instant::now() < deadline,
                "{} did not exit; it drew:\n{}",
                self.shell.name(),
                self.transcript(),
            );
            self.pump(Duration::from_millis(25));
        }
        self.closed = true;
    }
}

impl Drop for Session {
    /// Ends a shell that a failed test left running. Dropping the pty waits
    /// for the shell to exit, and not every shell exits when it hangs up; nor
    /// does a shell finish exiting while what it wrote waits to be read.
    fn drop(&mut self) {
        if !self.closed {
            let shell = Pid::from_raw(self.pty.child().id().cast_signed());
            let _ = signal::kill(shell, Signal::SIGKILL);
            let deadline = Instant::now() + TIMEOUT;
            while !self.exited && Instant::now() < deadline {
                self.pump(Duration::from_millis(25));
            }
        }
    }
}

/// A prompt with a module that renders at once and one that takes two
/// seconds.
const FAST_AND_SLOW: &str = r#"
format = "${custom.fast}${custom.slow}$line_break$character"
add_newline = false

[custom.fast]
command = "printf FAST"
when = true
shell = ["/bin/sh"]

[custom.slow]
command = "sleep 2; printf SLOW"
when = true
shell = ["/bin/sh"]
ignore_timeout = true

[character]
success_symbol = ">"
"#;

/// The prompt is drawn before its slow module renders, refined once it
/// does, and what was typed in between survives.
fn streams(shell: Shell) {
    let mut session = Session::start(shell, FAST_AND_SLOW);
    let first = session.wait_for("FAST");
    assert!(!first.contains("SLOW"), "the first paint waited:\n{first}");

    session.send(shell.input());
    let refined = session.wait_for("SLOW");
    assert!(
        refined.contains(shell.input()),
        "typing was lost:\n{refined}"
    );

    session.enter();
    session.wait_for("RESULT:typed");
    session.close();
}

/// A prompt is not refined once its line has been accepted: the line under
/// it, and what the command printed, would be drawn over.
fn an_accepted_line_is_left_alone(shell: Shell) {
    let mut session = Session::start(shell, FAST_AND_SLOW);
    session.wait_for(">");
    // As `close` explains.
    session.send(shell.sleep_then_input());
    session.wait_for(shell.sleep_then_input());
    session.enter();
    session.wait_for("RESULT:typed");
    let screen = session.wait_for("SLOW");
    assert_eq!(
        screen.matches("SLOW").count(),
        1,
        "the prompt of the accepted line was refined:\n{screen}"
    );
    session.close();
}

/// A prompt that opens with a blank line, as `add_newline` makes it, arrives
/// whole: a reader that took the blank line for the end of a field would
/// lose the prompt after it.
fn a_leading_blank_line_survives(shell: Shell) {
    let mut session = Session::start(
        shell,
        &FAST_AND_SLOW.replace("add_newline = false", "add_newline = true"),
    );
    session.wait_for("FAST");
    session.wait_for("SLOW");
    session.close();
}

/// Text that a shell would expand, were it not escaped for that shell.
const LITERAL: &str = r"$(echo EXPANDED) `echo EXPANDED` %n \w {user} 100%";

/// What a module shows reaches the screen as it is, however much of it a
/// shell would otherwise expand.
fn text_is_shown_as_it_is(shell: Shell) {
    let config = r#"
format = "${custom.literal}END"
add_newline = false

[custom.literal]
command = '''printf '%s' 'LITERAL' '''
when = true
shell = ["/bin/sh"]
"#;
    let mut session = Session::start(shell, &config.replace("LITERAL", LITERAL));
    let screen = session.wait_for("END");
    assert!(
        screen.contains(LITERAL),
        "{} did not show {LITERAL:?}:\n{screen}",
        shell.name()
    );
    session.close();
}

/// The right prompt is kept current on its own, without anything typed.
fn the_right_prompt_is_kept_current(shell: Shell) {
    let mut session = Session::start(
        shell,
        r#"
format = "MARK$character"
right_format = "$time"
add_newline = false

[time]
disabled = false
format = "$time"
time_format = "%S%.6f"
"#,
    );
    session.wait_for("MARK");
    let first = session.screen();
    let deadline = Instant::now() + TIMEOUT;
    while session.screen() == first {
        assert!(Instant::now() < deadline, "the clock never moved:\n{first}");
        session.pump(Duration::from_millis(25));
    }
    session.close();
}

/// Switching to vi command mode redraws the mode indicator, which only a new
/// stream can draw.
fn vi_mode_redraws_the_indicator(shell: Shell) {
    let mut session = Session::start(
        shell,
        r#"
format = "MARK$character"
add_newline = false

[character]
success_symbol = "INSERT"
vimcmd_symbol = "NORMAL"
"#,
    );
    session.wait_for("INSERT");
    session.send("bindkey -v");
    session.enter();
    session.wait_for("INSERT");
    session.send("\x1b");
    session.wait_for("NORMAL");
    // Back to insert mode, so that `exit` is typed rather than run as commands.
    session.send("i");
    session.wait_for("INSERT");
    session.close();
}

/// The prompt shows what every precmd hook has left, including those added
/// after starship's.
#[test]
#[ignore = "requires zsh"]
fn zsh_draws_what_later_precmd_hooks_leave() {
    let mut session = Session::start(
        Shell::Zsh,
        r#"
format = "${env_var.STREAM_TEST}>"
add_newline = false

[env_var.STREAM_TEST]
default = "old"
format = "$env_value"
"#,
    );
    session.wait_for("old>");
    session.send("late() { export STREAM_TEST=updated }; add-zsh-hook precmd late");
    session.wait_for("add-zsh-hook precmd late");
    session.enter();
    session.wait_for("updated>");
    session.close();
}

#[test]
#[ignore = "requires zsh"]
fn zsh_streams() {
    streams(Shell::Zsh);
}

#[test]
#[ignore = "requires zsh"]
fn zsh_leaves_an_accepted_line_alone() {
    an_accepted_line_is_left_alone(Shell::Zsh);
}

#[test]
#[ignore = "requires zsh"]
fn zsh_shows_text_as_it_is() {
    text_is_shown_as_it_is(Shell::Zsh);
}

#[test]
#[ignore = "requires zsh"]
fn zsh_keeps_a_leading_blank_line() {
    a_leading_blank_line_survives(Shell::Zsh);
}

#[test]
#[ignore = "requires zsh"]
fn zsh_keeps_the_right_prompt_current() {
    the_right_prompt_is_kept_current(Shell::Zsh);
}

#[test]
#[ignore = "requires zsh"]
fn zsh_redraws_the_vi_mode_indicator() {
    vi_mode_redraws_the_indicator(Shell::Zsh);
}
