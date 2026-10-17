//! Streamed prompts, driven through real shells in a pseudo-terminal: a
//! correct frame does not by itself make a correct redraw, which is up to the
//! shell's line editor.
#![cfg(unix)]

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
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
    Fish,
    Nushell,
    /// bash with readline, which draws each prompt once.
    Bash,
    BashWithBle,
    Xonsh,
    Pwsh,
}

impl Shell {
    const fn name(self) -> &'static str {
        match self {
            Self::Zsh => "zsh",
            Self::Fish => "fish",
            Self::Nushell => "nu",
            Self::Bash | Self::BashWithBle => "bash",
            Self::Xonsh => "xonsh",
            Self::Pwsh => "pwsh",
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
            Self::Zsh | Self::Fish | Self::Bash | Self::BashWithBle => {
                "printf 'RESULT:%s\\n' typed"
            }
            Self::Nushell | Self::Xonsh => "print ('RESULT:' + 'typed')",
            Self::Pwsh => "Write-Output ('RESULT:' + 'typed')",
        }
    }

    /// A command line that waits until the prompt it was typed at is long
    /// finished, then prints `RESULT:typed`. PowerShell sleeps in steps,
    /// since it runs events between them.
    const fn sleep_then_input(self) -> &'static str {
        match self {
            Self::Zsh | Self::Fish | Self::Bash | Self::BashWithBle => {
                "sleep 3; printf 'RESULT:%s\\n' typed"
            }
            Self::Nushell => "sleep 3sec; print ('RESULT:' + 'typed')",
            Self::Xonsh => "import time; time.sleep(3); print('RESULT:' + 'typed')",
            Self::Pwsh => {
                "1..15 | ForEach-Object { Start-Sleep -Milliseconds 200 }; Write-Output ('RESULT:' + 'typed')"
            }
        }
    }

    /// The command that turns on the shell's vi key bindings, if it has them.
    const fn vi_key_bindings(self) -> Option<&'static str> {
        match self {
            Self::Zsh => Some("bindkey -v"),
            Self::Fish => Some("fish_vi_key_bindings"),
            Self::Nushell | Self::Bash | Self::BashWithBle | Self::Xonsh | Self::Pwsh => None,
        }
    }

    /// The shell's arguments, and the command that then loads starship, if
    /// they do not.
    fn arguments(self, fixture: &Fixture) -> (Vec<String>, Option<&'static str>) {
        let source = Some("source \"$STARSHIP_INIT\"\n");
        match self {
            Self::Zsh => (vec!["-f".into(), "-i".into()], source),
            Self::Fish => (vec!["--no-config".into(), "--interactive".into()], source),
            Self::Nushell => {
                let environment = fixture.directory.path().join("env.nu");
                fs::write(&environment, "$env.config.show_banner = false\n")
                    .expect("a nushell environment");
                let arguments = [
                    "--env-config".as_ref(),
                    environment.as_os_str(),
                    "--config".as_ref(),
                    fixture.init.as_os_str(),
                ];
                (
                    arguments
                        .iter()
                        .map(|argument| argument.to_string_lossy().into_owned())
                        .collect(),
                    None,
                )
            }
            Self::Bash | Self::BashWithBle => {
                let ble = match self {
                    Self::BashWithBle => {
                        let ble = std::env::var_os("STARSHIP_TEST_BLE")
                            .expect("STARSHIP_TEST_BLE names ble.sh");
                        format!("source '{}'\n", Path::new(&ble).display())
                    }
                    _ => String::new(),
                };
                let rcfile = fixture.directory.path().join("bashrc");
                fs::write(&rcfile, format!("{ble}source \"$STARSHIP_INIT\"\n"))
                    .expect("a bash rcfile");
                (
                    vec![
                        "--noprofile".into(),
                        "--rcfile".into(),
                        rcfile.display().to_string(),
                        "-i".into(),
                    ],
                    None,
                )
            }
            Self::Xonsh => (
                vec![
                    "--rc".into(),
                    fixture.init.display().to_string(),
                    "--interactive".into(),
                ],
                None,
            ),
            Self::Pwsh => (
                vec![
                    "-NoLogo".into(),
                    "-NoProfile".into(),
                    "-NoExit".into(),
                    "-Command".into(),
                    "Invoke-Expression (Get-Content -Raw -LiteralPath $env:STARSHIP_INIT)".into(),
                ],
                None,
            ),
        }
    }

    /// The arguments that make starship print the shell's whole init script.
    const fn init_arguments(self) -> &'static [&'static str] {
        match self {
            Self::Zsh => &["init", "zsh", "--print-full-init"],
            Self::Fish => &["init", "fish", "--print-full-init"],
            Self::Nushell => &["init", "nu"],
            Self::Bash | Self::BashWithBle => &["init", "bash", "--print-full-init"],
            Self::Xonsh => &["init", "xonsh", "--print-full-init"],
            Self::Pwsh => &["init", "powershell", "--print-full-init"],
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
    fn new(shell: Shell, configuration: &str) -> Self {
        let directory = tempfile::tempdir().expect("a scratch directory");
        fs::write(directory.path().join("starship.toml"), configuration).expect("a configuration");
        let output = command(starship())
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
            // xonsh greets a user without a configuration of their own.
            ("XONSH_SUPPRESS_WELCOME".into(), "1".into()),
        ])
    }
}

fn starship() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_starship"))
}

/// A command running `program`, looked up on `PATH` unless it is a path, so
/// that nothing in the working directory runs in its place.
fn command(program: impl AsRef<OsStr>) -> Command {
    let found = which::which(program.as_ref())
        .unwrap_or_else(|error| panic!("{:?} is not found: {error}", program.as_ref()));
    #[expect(clippy::disallowed_methods, reason = "the program was looked up")]
    Command::new(found)
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
    fn start(shell: Shell, configuration: &str) -> Self {
        let turn = ONE_SHELL_AT_A_TIME
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let fixture = Fixture::new(shell, configuration);
        let (arguments, load) = shell.arguments(&fixture);
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
        // Typed ahead of the line editor, text meets the terminal's own line
        // discipline, which turns Enter into a line feed.
        self.settle();
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
add_newline = true

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

fn redraws_once_finished(shell: Shell) {
    let mut session = Session::start(shell, FAST_AND_SLOW);
    let first = session.wait_for(">");
    assert!(!first.contains("SLOW"), "the first paint waited:\n{first}");
    session.wait_for("FAST SLOW");

    session.send(shell.input());
    session.wait_for(shell.input());
    session.enter();
    session.wait_for("RESULT:typed");
    session.close();
}

fn an_accepted_line_is_left_alone(shell: Shell) {
    let mut session = Session::start(shell, FAST_AND_SLOW);
    let first = session.wait_for(">");
    assert!(!first.contains("SLOW"), "the first paint waited:\n{first}");
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

/// Text that a shell would expand, were it not escaped for that shell.
const LITERAL: &str = r"$(echo EXPANDED) `echo EXPANDED` %n \w {user} 100% \$HOME \$(echo EXPANDED) \`echo EXPANDED` \\$HOME \\";

fn text_is_shown_as_it_is(shell: Shell) {
    let configuration = r#"
format = "${custom.literal}END"
add_newline = false

[custom.literal]
command = '''printf '%s' 'LITERAL' '''
when = true
shell = ["/bin/sh"]
"#;
    let mut session = Session::start(shell, &configuration.replace("LITERAL", LITERAL));
    let screen = session.wait_for("END");
    assert!(
        screen.contains(LITERAL),
        "{} did not show {LITERAL:?}:\n{screen}",
        shell.name()
    );
    session.close();
}

fn the_right_prompt_is_kept_current(shell: Shell) {
    let mut session = Session::start(
        shell,
        r#"
format = "MARK$character"
right_format = "${custom.right}"
add_newline = false

[custom.right]
command = "if test -f right-text; then cat right-text; else printf RIGHT_BEFORE; fi"
when = true
shell = ["/bin/sh"]
format = "$output"
refresh = 100
"#,
    );
    session.wait_for("RIGHT_BEFORE");
    fs::write(
        session._fixture.directory.path().join("right-text"),
        "RIGHT_AFTER",
    )
    .unwrap();
    let screen = session.wait_for("RIGHT_AFTER");
    assert!(
        screen.contains("MARK"),
        "the main prompt was lost:\n{screen}"
    );
    assert!(
        !screen.contains("RIGHT_BEFORE"),
        "stale right prompt:\n{screen}"
    );
    session.close();
}

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

/// A stream started while the line is edited, as for the vi command keymap,
/// reports into the log alone: here the keymap's malformed symbol, which
/// nothing reported before, must not land in the line.
fn a_stream_started_while_editing_reports_into_the_log_alone(shell: Shell) {
    let mut session = Session::start(
        shell,
        r#"
format = "MARK$character"
add_newline = false

[character]
success_symbol = "INSERT"
vimcmd_symbol = "[NORMAL"
"#,
    );
    session.wait_for("MARKINSERT");
    let bindings = shell
        .vi_key_bindings()
        .expect("a shell with vi key bindings");
    session.send(bindings);
    session.wait_for(bindings);
    session.enter();
    session.settle();
    session.send("\x1b");
    let logs = session._fixture.directory.path().join(".cache/starship");
    let deadline = Instant::now() + TIMEOUT;
    while !logged(&logs, "Error in module `character`") {
        assert!(
            Instant::now() < deadline,
            "the vi command keymap was never drawn"
        );
        session.pump(Duration::from_millis(25));
    }
    session.pump(QUIET);
    let screen = session.screen();
    assert!(
        !screen.contains("Error in module"),
        "a report landed in the line:\n{screen}"
    );
    // Back to insert mode, so that `exit` is typed rather than run as commands.
    session.send("i");
    session.close();
}

/// A stream ends as the command typed at its prompt starts, rather than
/// render and refresh while the command runs.
fn a_command_runs_without_its_stream(shell: Shell) {
    let mut session = Session::start(
        shell,
        r#"
format = "${custom.stream}$character"
add_newline = false

[custom.stream]
command = 'echo $PPID > "$HOME/stream"; printf LIVE'
when = true
shell = ["/bin/sh"]
format = "$output"
refresh = 100

[character]
success_symbol = ">"
"#,
    );
    session.wait_for("LIVE>");
    session.send(shell.sleep_then_input());
    session.wait_for(shell.sleep_then_input());
    session.enter();
    session.pump(QUIET);
    // The module's command records the process that runs it: the stream.
    let record = session._fixture.directory.path().join("stream");
    let stream = fs::read_to_string(record).expect("the stream rendered");
    let stream = Pid::from_raw(stream.trim().parse().expect("a process ID"));
    assert!(ends(stream), "the stream outlived the start of the command");
    session.wait_for("RESULT:typed");
    session.close();
}

/// What a stream reports before its first paint, such as warnings about its
/// configuration, is shown ahead of the prompt, as for a prompt printed
/// whole; what it reports later goes to the log alone, rather than into the
/// line being typed.
fn warnings_are_shown_only_before_the_first_paint(shell: Shell) {
    let mut session = Session::start(
        shell,
        r#"
format = "${custom.late}MARK$character"
add_newline = false
command_timeout = 500

[character]
success_symbol = ">"
unknown_option = true

[custom.late]
command = "sleep 2"
when = true
shell = ["/bin/sh"]
"#,
    );
    let screen = session.wait_for("MARK>");
    assert!(
        screen.contains("unknown_option"),
        "the configuration's warning is missing:\n{screen}"
    );
    let logs = session._fixture.directory.path().join(".cache/starship");
    let deadline = Instant::now() + TIMEOUT;
    while !logged(&logs, "timed out") {
        assert!(Instant::now() < deadline, "the module never timed out");
        session.pump(Duration::from_millis(25));
    }
    session.pump(QUIET);
    let screen = session.screen();
    assert!(
        !screen.contains("timed out"),
        "a warning landed after the first paint:\n{screen}"
    );
    session.close();
}

/// Whether a session's log in `directory` holds `text`.
fn logged(directory: &Path, text: &str) -> bool {
    fs::read_dir(directory)
        .into_iter()
        .flatten()
        .flatten()
        .any(|entry| fs::read_to_string(entry.path()).is_ok_and(|log| log.contains(text)))
}

/// Each behavior, as a test in each shell that has it, named for both.
macro_rules! in_shells {
    ($($shell:ident, $requirement:literal: $($test:ident => $behavior:ident),+;)+) => {
        $($(
            #[test]
            #[ignore = $requirement]
            fn $test() {
                $behavior(Shell::$shell);
            }
        )+)+
    };
}

in_shells! {
    Zsh, "requires zsh":
        zsh_streams => streams,
        zsh_leaves_an_accepted_line_alone => an_accepted_line_is_left_alone,
        zsh_shows_text_as_it_is => text_is_shown_as_it_is,
        zsh_keeps_the_right_prompt_current => the_right_prompt_is_kept_current,
        zsh_redraws_the_vi_mode_indicator => vi_mode_redraws_the_indicator,
        zsh_runs_a_command_without_its_stream => a_command_runs_without_its_stream,
        zsh_shows_warnings_only_before_the_first_paint => warnings_are_shown_only_before_the_first_paint,
        zsh_reports_into_the_log_alone_while_editing => a_stream_started_while_editing_reports_into_the_log_alone;
    Fish, "requires fish":
        fish_streams => streams,
        fish_leaves_an_accepted_line_alone => an_accepted_line_is_left_alone,
        fish_shows_text_as_it_is => text_is_shown_as_it_is,
        fish_keeps_the_right_prompt_current => the_right_prompt_is_kept_current,
        fish_runs_a_command_without_its_stream => a_command_runs_without_its_stream,
        fish_shows_warnings_only_before_the_first_paint => warnings_are_shown_only_before_the_first_paint,
        fish_reports_into_the_log_alone_while_editing => a_stream_started_while_editing_reports_into_the_log_alone;
    Nushell, "requires nu 0.115 or later":
        nushell_streams => streams,
        nushell_leaves_an_accepted_line_alone => an_accepted_line_is_left_alone,
        nushell_shows_text_as_it_is => text_is_shown_as_it_is,
        nushell_keeps_the_right_prompt_current => the_right_prompt_is_kept_current,
        nushell_runs_a_command_without_its_stream => a_command_runs_without_its_stream,
        nushell_shows_warnings_only_before_the_first_paint => warnings_are_shown_only_before_the_first_paint;
    Bash, "requires bash":
        bash_shows_text_as_it_is => text_is_shown_as_it_is;
    BashWithBle, "requires bash 4.2 or later, and STARSHIP_TEST_BLE naming ble.sh":
        bash_with_ble_streams => streams,
        bash_with_ble_leaves_an_accepted_line_alone => an_accepted_line_is_left_alone,
        bash_with_ble_shows_text_as_it_is => text_is_shown_as_it_is,
        bash_with_ble_keeps_the_right_prompt_current => the_right_prompt_is_kept_current,
        bash_with_ble_runs_a_command_without_its_stream => a_command_runs_without_its_stream,
        bash_with_ble_shows_warnings_only_before_the_first_paint => warnings_are_shown_only_before_the_first_paint;
    Xonsh, "requires xonsh":
        xonsh_streams => streams,
        xonsh_leaves_an_accepted_line_alone => an_accepted_line_is_left_alone,
        xonsh_shows_text_as_it_is => text_is_shown_as_it_is,
        xonsh_keeps_the_right_prompt_current => the_right_prompt_is_kept_current,
        xonsh_runs_a_command_without_its_stream => a_command_runs_without_its_stream,
        xonsh_shows_warnings_only_before_the_first_paint => warnings_are_shown_only_before_the_first_paint;
    Pwsh, "requires pwsh":
        pwsh_redraws_once_finished => redraws_once_finished,
        pwsh_leaves_an_accepted_line_alone => an_accepted_line_is_left_alone,
        pwsh_shows_text_as_it_is => text_is_shown_as_it_is,
        pwsh_runs_a_command_without_its_stream => a_command_runs_without_its_stream,
        pwsh_shows_warnings_only_before_the_first_paint => warnings_are_shown_only_before_the_first_paint;
}

/// PowerShell redraws its prompt once complete only while nothing has been
/// typed: PSReadLine misplaces a prompt redrawn beneath typed text, which
/// command output then overwrites.
#[test]
#[ignore = "requires pwsh"]
fn pwsh_leaves_a_line_being_typed_alone() {
    let mut session = Session::start(
        Shell::Pwsh,
        r#"
format = "${custom.fast}${custom.slow}$line_break$character"
add_newline = true

[custom.fast]
command = "printf FAST"
when = true
shell = ["/bin/sh"]

[custom.slow]
command = "sleep 2; touch finished; printf SLOW"
when = true
shell = ["/bin/sh"]
ignore_timeout = true

[character]
success_symbol = ">"
"#,
    );
    session.wait_for(">");
    let input = Shell::Pwsh.input();
    let (first, rest) = input.split_at(input.len() / 2);
    session.send(first);
    session.wait_for(first);
    let finished = session._fixture.directory.path().join("finished");
    let deadline = Instant::now() + TIMEOUT;
    while !finished.exists() {
        assert!(Instant::now() < deadline, "the slow module never finished");
        session.pump(Duration::from_millis(25));
    }
    // Time for the stream to send the complete prompt, which PSReadLine takes
    // up as it reads the next key.
    session.pump(Duration::from_secs(1));
    for character in rest.chars() {
        session.send(character.encode_utf8(&mut [0; 4]));
        session.pump(Duration::from_millis(50));
    }
    let typed = session.wait_for(input);
    assert!(
        !typed.contains("SLOW"),
        "redrawn beneath typed text:\n{typed}"
    );

    session.enter();
    let screen = session.wait_for("RESULT:typed");
    assert_eq!(1, screen.matches("RESULT:typed").count(), "{screen}");
    assert!(
        screen.contains("FAST"),
        "output overwrote the prompt:\n{screen}"
    );
    session.close();
}

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
#[ignore = "requires fish"]
fn fish_stops_its_stream_which_removes_its_directory() {
    let fixture = Fixture::new(
        Shell::Fish,
        r#"
format = "MARK${custom.slow}"
add_newline = false
[custom.slow]
command = 'touch rendering; sleep 30'
when = true
shell = ["/bin/sh"]
ignore_timeout = true
"#,
    );
    let output = command(Shell::Fish.program())
        .args([
            "--no-config",
            "--command",
            r#"
source "$STARSHIP_INIT"
__starship_prompt 1 --status=0 >/dev/null
set -l directory $__starship_stream_directory
test -d "$directory"; or exit 1
for attempt in (seq 50)
    test -f rendering; and break
    sleep 0.1
end
test -f rendering; or exit 2
__starship_stream_stop
for attempt in (seq 20)
    test -d "$directory"; or break
    sleep 0.1
end
test -d "$directory"; and exit 3
printf 'STOPPED\n'
"#,
        ])
        .envs(fixture.environment(Shell::Fish))
        .current_dir(fixture.directory.path())
        .output()
        .expect("fish starts and stops a stream");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(b"STOPPED\n", output.stdout.as_slice());
    assert!(output.stderr.is_empty(), "{output:?}");
}

#[test]
#[ignore = "requires fish"]
fn fish_retries_failed_streams_and_caches_complete_prompt_pairs() {
    let fixture = Fixture::new(Shell::Fish, "format = \"MARK\"\nadd_newline = false\n");
    // Replace only the native command, keeping the generated integration intact.
    let source = fs::read_to_string(&fixture.init).unwrap();
    fs::write(
        &fixture.init,
        source.replace(starship().to_str().unwrap(), "__starship_test_command"),
    )
    .unwrap();
    let output = command(Shell::Fish.program())
        .arg("--no-config")
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/fish-stream.fish"))
        .envs(fixture.environment(Shell::Fish))
        .current_dir(fixture.directory.path())
        .output()
        .expect("fish exercises stream startup and caching");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(
        b"PASS: stream fallback and prompt pair caching\n",
        output.stdout.as_slice()
    );
    assert!(output.stderr.is_empty(), "{output:?}");
}

/// A module whose command never finishes, writing the command's process ID to
/// `$STARSHIP_RENDERING` once it runs.
fn never_finishing(asynchronous: bool) -> String {
    format!(
        r#"
format = "${{custom.slow}}"
add_newline = false
[async]
enabled = {asynchronous}
[custom.slow]
command = 'echo $$ > "$STARSHIP_RENDERING"; exec sleep 30'
shell = ["/bin/sh"]
when = true
ignore_timeout = true
format = "$output"
"#
    )
}

/// The process the module of [`never_finishing`] runs, once it runs.
fn rendering(path: &Path) -> Pid {
    let started = Instant::now();
    loop {
        if let Some(process) = fs::read_to_string(path)
            .ok()
            .and_then(|process| process.trim().parse().ok())
        {
            return Pid::from_raw(process);
        }
        assert!(
            started.elapsed() < TIMEOUT,
            "the module never started rendering"
        );
        thread::sleep(Duration::from_millis(10));
    }
}

/// Whether `process` ends within two seconds.
fn ends(process: Pid) -> bool {
    let deadline = Instant::now() + Duration::from_secs(2);
    while signal::kill(process, None).is_ok() {
        if Instant::now() > deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(10));
    }
    true
}

/// A stream rendering a module that never finishes ends the moment the shell
/// stops reading it, whether or not it has drawn the first paint, and ends
/// the command the module runs with it.
fn a_stream_ends_once_its_shell_stops_reading(asynchronous: bool) {
    let fixture = Fixture::new(Shell::Bash, &never_finishing(asynchronous));
    let rendering = fixture.directory.path().join("rendering");
    let mut stream = command(starship())
        .args(["prompt", "--stream"])
        .envs(fixture.environment(Shell::Bash))
        .env("STARSHIP_RENDERING", &rendering)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut prompts = stream.stdout.take().unwrap();
    if asynchronous {
        let mut first = [0; 2];
        prompts.read_exact(&mut first).unwrap();
        assert_eq!(
            b"\0\0", &first,
            "the first paint, before the module renders"
        );
    }
    let command = self::rendering(&rendering);

    drop(prompts);
    let closed = Instant::now();
    let status = loop {
        if let Some(status) = stream.try_wait().unwrap() {
            break status;
        }
        if closed.elapsed() > Duration::from_secs(2) {
            let _ = stream.kill();
            let _ = stream.wait();
            panic!("the stream kept rendering for a shell that stopped reading");
        }
        thread::sleep(Duration::from_millis(10));
    };
    assert!(status.success(), "{status}");
    assert!(ends(command), "the module's command outlived the stream");
}

/// Fish can stop waiting for the first prompts before it knows where they
/// will be, and so before it could ask the stream to stop: the stream stops,
/// removing its directory and ending the command its module runs.
#[test]
fn a_stream_for_fish_ends_once_fish_stops_waiting_for_the_first_prompts() {
    let fixture = Fixture::new(Shell::Fish, &never_finishing(false));
    let rendering = fixture.directory.path().join("rendering");
    let temporary = fixture.directory.path().join("temporary");
    fs::create_dir(&temporary).unwrap();
    let mut fish = command("sleep").arg("30").spawn().unwrap();
    let mut stream = command(starship())
        .args(["prompt", "--stream", &format!("--signal-pid={}", fish.id())])
        .envs(fixture.environment(Shell::Fish))
        .env("STARSHIP_RENDERING", &rendering)
        .env("TMPDIR", &temporary)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let waiting = stream.stdout.take().unwrap();
    let command = self::rendering(&rendering);
    assert_eq!(1, fs::read_dir(&temporary).unwrap().count(), "a directory");

    drop(waiting);
    let stopped = ends(command);
    let _ = stream.wait();
    let _ = fish.kill();
    let _ = fish.wait();
    assert!(stopped, "the module's command outlived the stream");
    assert_eq!(
        0,
        fs::read_dir(&temporary).unwrap().count(),
        "the directory remains"
    );
}

#[test]
fn a_stream_ends_once_its_shell_stops_reading_the_first_paint() {
    a_stream_ends_once_its_shell_stops_reading(true);
}

#[test]
fn a_stream_without_async_ends_once_its_shell_stops_waiting_for_it() {
    a_stream_ends_once_its_shell_stops_reading(false);
}

#[test]
#[ignore = "requires pwsh"]
fn pwsh_reads_prompts_split_anywhere_and_cancels_its_reader() {
    let output = command(Shell::Pwsh.program())
        .args(["-NoLogo", "-NoProfile", "-NonInteractive", "-File"])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/powershell-stream.ps1"))
        .arg("-Starship")
        .arg(starship())
        .output()
        .expect("PowerShell exercises the production stream pump");
    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("PASS: split UTF-8"));
}
