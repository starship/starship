use std::env;
use std::ffi::OsStr;
use std::fmt::{self, Debug};
use std::io::Write;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;

use process_control::{ChildExt, Control, Output};

use super::{Context, Module, ModuleConfig};

use crate::config::VecOr;
use crate::{
    config::Either, configs::custom::CustomConfig, formatter::StringFormatter,
    utils::create_command,
};

/// This struct is used to store the shell invocation information. It exists
/// as a lightweight interface that holds the shell cmd, args. It also holds
/// uses_stdin, which is used to determine whether the invocation expects
/// scripts.
struct ShellInvocation {
    shell: String,
    args: Vec<String>,
    subshell: Subshell,
    uses_stdin: bool,
}

enum Subshell {
    Parens,
    ShellInvoc,
}

/// Creates a custom module with some configuration
///
/// The relevant TOML config will set the files, extensions, and directories needed
/// for the module to be displayed. If none of them match, and optional "when"
/// command can be run -- if its result is 0, the module will be shown.
///
/// Finally, the content of the module itself is also set by a command.
pub fn module<'a>(name: &str, context: &'a Context) -> Option<Module<'a>> {
    let toml_config = get_config(name, context)?;
    let config = CustomConfig::load(toml_config);
    if config.disabled {
        return None;
    }

    if let Some(os) = config.os
        && os != env::consts::OS
        && !(os == "unix" && cfg!(unix))
    {
        return None;
    }

    if config.require_repo && context.get_repo().is_err() {
        return None;
    }

    // Note: Forward config if `Module` ends up needing `config`
    let mut module = Module::new(format!("custom.{name}"), config.description, None);

    let mut is_match = context
        .try_begin_scan()?
        .set_extensions(&config.detect_extensions)
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_match {
        is_match = match config.when {
            Either::First(b) => b,
            Either::Second(s) => exec_when(s, &config, context),
        };

        if !is_match {
            return None;
        }
    }

    let variables_closure = |variable: &str| match variable {
        "output" => {
            let output = exec_command(config.command, context, &config)?;
            let trimmed = output.trim();

            if trimmed.is_empty() {
                None
            } else {
                Some(Ok(trimmed.to_string()))
            }
        }
        _ => None,
    };

    let parsed = StringFormatter::new(config.format).and_then(|mut formatter| {
        formatter = formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            });

        if config.unsafe_no_escape {
            formatter = formatter.map_no_escaping(variables_closure)
        } else {
            formatter = formatter.map(variables_closure)
        }

        formatter.parse(None, Some(context))
    });

    match parsed {
        Ok(segments) => module.set_segments(segments),
        Err(error) => {
            log::warn!("Error in module `custom.{name}`:\n{error}");
        }
    };
    Some(module)
}

/// Gets the TOML config for the custom module, handling the case where the module is not defined
fn get_config<'a>(module_name: &str, context: &'a Context<'a>) -> Option<&'a toml::Value> {
    struct DebugCustomModules<'tmp>(&'tmp toml::value::Table);

    impl Debug for DebugCustomModules<'_> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_list().entries(self.0.keys()).finish()
        }
    }

    let config = context.config.get_custom_module_config(module_name);

    if config.is_some() {
        return config;
    } else if let Some(modules) = context.config.get_custom_modules() {
        log::debug!(
            "top level format contains custom module {module_name:?}, but no configuration was provided. Configuration for the following modules were provided: {:?}",
            DebugCustomModules(modules),
        );
    } else {
        log::debug!(
            "top level format contains custom module {module_name:?}, but no configuration was provided.",
        );
    };
    None
}

/// Attempt to run the given command in a shell by passing it as either
/// `stdin` or an argument to `get_shell_invoc`, depending on the
/// configuration or by invoking a platform-specific fallback shell if
/// `shell` is empty.
fn shell_command(cmd: &str, config: &CustomConfig, context: &Context) -> Option<Output> {
    log::trace!("shell_command run with the following command: {}", cmd);

    let shell_invoc: &ShellInvocation = &get_shell_invoc(&config.shell, context);
    let cmd_with_status = inject_status_subshell(cmd, shell_invoc, context);

    let mut command = match create_command(shell_invoc.shell.as_str()) {
        Ok(command) => command,
        Err(error) if !shell_invoc.args.is_empty() => {
            log::debug!(
                "Error creating command with STARSHIP_SHELL, falling back to fallback shell: {error}"
            );

            if cfg!(windows) {
                create_command("cmd").ok()?
            } else {
                create_command("/usr/bin/env").ok()?
            }
        }
        _ => return None,
    };

    command
        .current_dir(&context.current_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let use_stdin = config.use_stdin.unwrap_or(shell_invoc.uses_stdin);

    for arg in &shell_invoc.args {
        command.arg(arg);
    }

    if !use_stdin {
        command.arg(cmd_with_status);
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            log::debug!(
                "Failed to run command with given shell or STARSHIP_SHELL env variable:: {error}"
            );
            return None;
        }
    };

    if use_stdin {
        child.stdin.as_mut()?.write_all(cmd.as_bytes()).ok()?;
    }

    log::trace!("Final command:");
    log::trace!("  program={:?}", command.get_program());
    log::trace!("  args={:?}", command.get_args().collect::<Vec<_>>());
    log::trace!("  use_stdin={}", use_stdin);

    let mut output = child.controlled_with_output();

    if !config.ignore_timeout {
        output = output
            .time_limit(Duration::from_millis(context.root_config.command_timeout))
            .terminate_for_timeout()
    }

    match output.wait().ok()? {
        None => {
            log::warn!("Executing custom command {cmd:?} timed out.");
            log::warn!(
                "You can set command_timeout in your config to a higher value or set ignore_timeout to true for this module to allow longer-running commands to keep executing."
            );
            None
        }
        Some(status) => Some(status),
    }
}

/// Execute the given command capturing all output, and return whether it return 0
fn exec_when(cmd: &str, config: &CustomConfig, context: &Context) -> bool {
    log::trace!("Running '{cmd}'");

    if let Some(output) = shell_command(cmd, config, context) {
        if !output.status.success() {
            log::trace!("non-zero exit code '{:?}'", output.status.code());
            log::trace!(
                "stdout: {}",
                std::str::from_utf8(&output.stdout).unwrap_or("<invalid utf8>")
            );
            log::trace!(
                "stderr: {}",
                std::str::from_utf8(&output.stderr).unwrap_or("<invalid utf8>")
            );
        }

        output.status.success()
    } else {
        log::debug!("Cannot start command");

        false
    }
}

/// Execute the given command, returning its output on success
fn exec_command(cmd: &str, context: &Context, config: &CustomConfig) -> Option<String> {
    log::trace!("Running '{cmd}'");

    #[cfg(test)]
    if cmd == "__starship_to_be_escaped" {
        return Some("`to_be_escaped`".to_string());
    }

    if let Some(output) = shell_command(cmd, config, context) {
        if !output.status.success() {
            log::trace!("Non-zero exit code '{:?}'", output.status.code());
            log::trace!(
                "stdout: {}",
                std::str::from_utf8(&output.stdout).unwrap_or("<invalid utf8>")
            );
            log::trace!(
                "stderr: {}",
                std::str::from_utf8(&output.stderr).unwrap_or("<invalid utf8>")
            );
            return None;
        }

        Some(String::from_utf8_lossy(&output.stdout).into())
    } else {
        None
    }
}

/// Determines the appropriate shell invocation strategy by matching the shell
/// name against known shells and returning defaults for args, subshell support,
/// and stdin handling.
///
/// If the user provided custom shell args, those take precedence; otherwise,
/// we use hardcoded defaults per shell (bash, zsh, sh, fish, nushell, powershell, cmd).
fn get_shell_invoc(shell: &VecOr<&str>, context: &Context) -> ShellInvocation {
    let shell_with_fallback = if !shell.0.is_empty() {
        shell.0[0].to_string()
    } else if let Some(env_shell) = context.get_env("STARSHIP_SHELL") {
        env_shell
    } else if cfg!(windows) {
        "cmd".to_string()
    } else {
        "sh".to_string()
    };

    let shell_name = Path::new(&shell_with_fallback)
        .file_stem()
        .and_then(OsStr::to_str);

    let (default_args, subshell, uses_stdin): (&[&str], _, _) = match shell_name {
        Some("pwsh" | "powershell") => {
            (&["-NoProfile", "-Command", "-"], Subshell::ShellInvoc, true)
        }
        Some("cmd") => (&["/C"], Subshell::ShellInvoc, false),
        Some("bash" | "zsh" | "sh" | "ksh" | "csh" | "tcsh") => (&["-c"], Subshell::Parens, false),
        _ => (&["-c"], Subshell::ShellInvoc, false),
    };

    let args = if shell.0.len() <= 1 {
        default_args.iter().map(|&s| s.to_string()).collect()
    } else {
        shell.0[1..].iter().map(|&s| s.to_string()).collect()
    };

    log::trace!(
        "shell={}, args={:?}, uses_stdin={}",
        shell_with_fallback,
        args,
        uses_stdin
    );

    ShellInvocation {
        shell: shell_with_fallback,
        args,
        subshell,
        uses_stdin,
    }
}

/// Prepends exit-code injection so the command can access the previous
/// command's actual exit status via `$?` or `$status`, rather than always `0`.
///
/// Uses shell-appropriate syntax:
/// - subshells for POSIX,
/// - nested shell invocation otherwise.
fn inject_status_subshell(cmd: &str, shell_invoc: &ShellInvocation, context: &Context) -> String {
    let exit_code: i64 = context
        .properties
        .status_code
        .as_deref()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let cmd_with_status = match shell_invoc.subshell {
        Subshell::Parens => format!("(exit {}); {}", exit_code, cmd),
        Subshell::ShellInvoc => format!(
            "{} {} \"exit {}\"; {}",
            shell_invoc.shell.clone(),
            shell_invoc.args.join(" "),
            exit_code,
            cmd
        ),
    };

    log::trace!(
        "The final command was built, including status subshell: {}",
        cmd_with_status
    );

    cmd_with_status
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::Shell;
    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;
    use std::path::Path;
    use std::sync::OnceLock;

    #[cfg(not(windows))]
    mod shell {
        pub const SHELL: &[&str] = &["/bin/sh"];
        pub const FAILING_COMMAND: &str = "false";
        pub const UNKNOWN_COMMAND: &str = "ydelsyiedsieudleylse dyesdesl";
    }

    #[cfg(windows)]
    mod shell {
        pub const SHELL: &[&str] = &["cmd"];
        pub const FAILING_COMMAND: &str = "color 00";
        pub const UNKNOWN_COMMAND: &str = "ydelsyiedsieudleylse dyesdesl";
    }

    use shell::*;

    fn shell_value() -> ShellValue {
        static DEFAULT_SHELL: OnceLock<ShellValue> = OnceLock::new();
        DEFAULT_SHELL
            .get_or_init(|| ShellValue::Array(SHELL.iter().map(|s| s.to_string()).collect()))
            .clone()
    }

    #[derive(Clone)]
    pub enum ShellValue {
        String(String),
        Array(Vec<String>),
    }

    pub struct CustomConfigBuilder {
        command: Option<String>,
        format: Option<String>,
        when: Option<String>,
        shell: Option<ShellValue>,
        disabled: Option<bool>,
        require_repo: Option<bool>,
        use_stdin: Option<bool>,
        ignore_timeout: Option<bool>,
        unsafe_no_escape: Option<bool>,
    }

    pub trait IntoWhen {
        fn into_when(self) -> String;
    }

    impl IntoWhen for bool {
        fn into_when(self) -> String {
            self.to_string()
        }
    }

    impl IntoWhen for &str {
        fn into_when(self) -> String {
            self.to_string()
        }
    }

    impl IntoWhen for String {
        fn into_when(self) -> String {
            self
        }
    }

    pub trait IntoShell {
        fn into_shell(self) -> ShellValue;
    }

    impl IntoShell for &str {
        fn into_shell(self) -> ShellValue {
            ShellValue::String(self.to_string())
        }
    }

    impl IntoShell for String {
        fn into_shell(self) -> ShellValue {
            ShellValue::String(self)
        }
    }

    impl IntoShell for Vec<String> {
        fn into_shell(self) -> ShellValue {
            ShellValue::Array(self)
        }
    }

    impl IntoShell for &[&str] {
        fn into_shell(self) -> ShellValue {
            ShellValue::Array(self.iter().map(|s| s.to_string()).collect())
        }
    }

    impl IntoShell for Vec<&str> {
        fn into_shell(self) -> ShellValue {
            ShellValue::Array(self.iter().map(|s| s.to_string()).collect())
        }
    }

    impl CustomConfigBuilder {
        pub fn new() -> Self {
            Self {
                command: None,
                format: None,
                when: None,
                shell: None,
                disabled: None,
                require_repo: None,
                use_stdin: None,
                ignore_timeout: None,
                unsafe_no_escape: None,
            }
        }

        pub fn command(mut self, cmd: impl Into<String>) -> Self {
            self.command = Some(cmd.into());
            self
        }

        pub fn format(mut self, fmt: impl Into<String>) -> Self {
            self.format = Some(fmt.into());
            self
        }

        pub fn when(mut self, w: impl IntoWhen) -> Self {
            self.when = Some(w.into_when());
            self
        }

        pub fn shell(mut self, s: impl IntoShell) -> Self {
            self.shell = Some(s.into_shell());
            self
        }

        pub fn disabled(mut self, d: bool) -> Self {
            self.disabled = Some(d);
            self
        }

        pub fn require_repo(mut self, r: bool) -> Self {
            self.require_repo = Some(r);
            self
        }

        pub fn use_stdin(mut self, u: bool) -> Self {
            self.use_stdin = Some(u);
            self
        }

        pub fn ignore_timeout(mut self, i: bool) -> Self {
            self.ignore_timeout = Some(i);
            self
        }

        pub fn unsafe_no_escape(mut self, u: bool) -> Self {
            self.unsafe_no_escape = Some(u);
            self
        }

        fn build(self) -> toml::Table {
            let mut test_table = toml::map::Map::new();

            if let Some(cmd) = self.command {
                test_table.insert("command".to_string(), toml::Value::String(cmd));
            }
            if let Some(fmt) = self.format {
                test_table.insert("format".to_string(), toml::Value::String(fmt));
            }
            if let Some(w) = self.when {
                test_table.insert("when".to_string(), toml::Value::String(w));
            }
            if let Some(d) = self.disabled {
                test_table.insert("disabled".to_string(), toml::Value::Boolean(d));
            }
            if let Some(r) = self.require_repo {
                test_table.insert("require_repo".to_string(), toml::Value::Boolean(r));
            }

            match self.shell {
                Some(ShellValue::String(s)) => {
                    test_table.insert("shell".to_string(), toml::Value::String(s));
                }
                Some(ShellValue::Array(arr)) => {
                    test_table.insert(
                        "shell".to_string(),
                        toml::Value::Array(arr.into_iter().map(toml::Value::String).collect()),
                    );
                }
                None => {
                    let default = shell_value();
                    match default {
                        ShellValue::Array(arr) => {
                            test_table.insert(
                                "shell".to_string(),
                                toml::Value::Array(
                                    arr.into_iter().map(toml::Value::String).collect(),
                                ),
                            );
                        }
                        ShellValue::String(s) => {
                            test_table.insert("shell".to_string(), toml::Value::String(s));
                        }
                    }
                }
            }

            if let Some(val) = self.use_stdin {
                test_table.insert("use_stdin".to_string(), toml::Value::Boolean(val));
            }
            if let Some(val) = self.ignore_timeout {
                test_table.insert("ignore_timeout".to_string(), toml::Value::Boolean(val));
            }
            if let Some(val) = self.unsafe_no_escape {
                test_table.insert("unsafe_no_escape".to_string(), toml::Value::Boolean(val));
            }

            let mut custom_table = toml::map::Map::new();
            custom_table.insert("test".to_string(), toml::Value::Table(test_table));

            let mut config = toml::map::Map::new();
            config.insert("custom".to_string(), toml::Value::Table(custom_table));

            config
        }
    }

    struct TestFixture {
        dir: tempfile::TempDir,
    }

    impl TestFixture {
        fn new() -> io::Result<Self> {
            Ok(Self {
                dir: tempfile::tempdir()?,
            })
        }

        fn renderer(&self) -> ModuleRenderer<'_> {
            ModuleRenderer::new("custom.test").path(self.dir.path())
        }

        fn path(&self) -> &Path {
            self.dir.path()
        }

        fn close(self) -> io::Result<()> {
            self.dir.close()
        }
    }

    #[test]
    fn when_returns_right_value() -> io::Result<()> {
        let fixture = TestFixture::new()?;
        let config = CustomConfigBuilder::new()
            .format("$output")
            .command("echo hello")
            .when("true")
            .ignore_timeout(true)
            .build();
        assert!(fixture.renderer().config(config).collect().is_some());
        fixture.close()
    }

    #[test]
    fn when_returns_false_if_invalid_command() -> io::Result<()> {
        let fixture = TestFixture::new()?;
        let config = CustomConfigBuilder::new()
            .format("test")
            .when(UNKNOWN_COMMAND)
            .ignore_timeout(true)
            .build();
        assert!(fixture.renderer().config(config).collect().is_none());
        fixture.close()
    }

    #[test]
    fn command_returns_right_string() -> io::Result<()> {
        let fixture = TestFixture::new()?;
        let config = CustomConfigBuilder::new()
            .format("$output")
            .command("echo hello")
            .when("true")
            .ignore_timeout(true)
            .build();
        assert_eq!(
            fixture
                .renderer()
                .config(config)
                .collect()
                .expect("command_returns_right_string should not be None")
                .as_str(),
            "hello"
        );

        let config = CustomConfigBuilder::new()
            .format("$output")
            .command("echo 강남스타일")
            .when("true")
            .ignore_timeout(true)
            .build();
        assert_eq!(
            fixture
                .renderer()
                .config(config)
                .collect()
                .expect("command_returns_right_string should not be None")
                .as_str(),
            "강남스타일"
        );
        fixture.close()
    }

    #[test]
    fn command_ignores_stderr() -> io::Result<()> {
        let fixture = TestFixture::new()?;
        let config = CustomConfigBuilder::new()
            .format("$output")
            .command("echo foo 1>&2; echo bar")
            .when("true")
            .ignore_timeout(true)
            .build();
        assert_eq!(
            fixture
                .renderer()
                .config(config)
                .collect()
                .expect("command_ignores_stderr should not be None")
                .as_str(),
            "bar"
        );

        let config = CustomConfigBuilder::new()
            .format("$output")
            .command("echo foo; echo bar 1>&2")
            .when("true")
            .ignore_timeout(true)
            .build();
        assert_eq!(
            fixture
                .renderer()
                .config(config)
                .collect()
                .expect("command_ignores_stderr should not be None")
                .as_str(),
            "foo"
        );
        fixture.close()
    }

    #[test]
    fn command_can_fail() -> io::Result<()> {
        let fixture = TestFixture::new()?;
        let config = CustomConfigBuilder::new()
            .format("$output")
            .command(FAILING_COMMAND)
            .when("true")
            .ignore_timeout(true)
            .build();
        assert!(fixture.renderer().config(config).collect().is_none());

        let config = CustomConfigBuilder::new()
            .format("$output")
            .command(UNKNOWN_COMMAND)
            .when("true")
            .ignore_timeout(true)
            .build();
        assert!(fixture.renderer().config(config).collect().is_none());
        fixture.close()
    }

    #[test]
    fn cwd_command() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        let mut f = File::create(fixture.path().join("a.txt"))?;
        write!(f, "hello")?;
        f.sync_all()?;

        let cat = if cfg!(windows) { "type" } else { "cat" };
        let cmd = format!("{cat} a.txt");

        let config = CustomConfigBuilder::new()
            .command(cmd)
            .when("true")
            .ignore_timeout(true)
            .build();

        assert_eq!(
            fixture
                .renderer()
                .config(config)
                .collect()
                .expect("cwd_command should not be None"),
            format!("{}", Color::Green.bold().paint("hello "))
        );
        fixture.close()
    }

    #[test]
    fn cwd_when() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        File::create(fixture.path().join("a.txt"))?.sync_all()?;

        let cat = if cfg!(windows) { "type" } else { "cat" };
        let cmd = format!("{cat} a.txt");

        let config = CustomConfigBuilder::new()
            .format("test")
            .when(cmd)
            .ignore_timeout(true)
            .build();

        assert_eq!(
            fixture
                .renderer()
                .config(config)
                .collect()
                .expect("cwd_when should not be None")
                .as_str(),
            "test"
        );
        fixture.close()
    }

    #[test]
    #[cfg(not(windows))]
    fn use_stdin_false() -> io::Result<()> {
        // TODO: PowerShell on Windows mangles non-ASCII characters when
        //     commands are passed as arguments (e.g., Korean text fails to
        //     escape properly). Stdin mode works around this.
        //   A proper fix would use `-EncodedCommand` with UTF-16LE base64
        //     encoding when arg-mode is requested on Windows.
        //   Until then, we'll just skip this test case on Windows.

        let fixture = TestFixture::new()?;

        let shell = if cfg!(windows) {
            vec![
                "powershell".to_owned(),
                "-NoProfile".to_owned(),
                "-Command".to_owned(),
            ]
        } else {
            vec!["sh".to_owned(), "-c".to_owned()]
        };

        let shell_refs: Vec<&str> = shell.iter().map(|s| s.as_str()).collect();

        let config = CustomConfigBuilder::new()
            .command("echo test")
            .when("true")
            .use_stdin(false)
            .ignore_timeout(true)
            .shell(shell_refs)
            .build();

        assert_eq!(
            fixture
                .renderer()
                .config(config)
                .collect()
                .expect("use_stdin_false should not be None"),
            format!("{}", Color::Green.bold().paint("test "))
        );
        fixture.close()
    }

    #[test]
    fn use_stdin_true() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        let shell = if cfg!(windows) {
            vec![
                "powershell".to_owned(),
                "-NoProfile".to_owned(),
                "-Command".to_owned(),
                "-".to_owned(),
            ]
        } else {
            vec!["sh".to_owned(), "-".to_owned()]
        };

        let shell_refs: Vec<&str> = shell.iter().map(|s| s.as_str()).collect();

        let config = CustomConfigBuilder::new()
            .command("echo 강남스타일")
            .when("true")
            .use_stdin(true)
            .ignore_timeout(true)
            .shell(shell_refs)
            .build();

        assert_eq!(
            fixture
                .renderer()
                .config(config)
                .collect()
                .expect("use_stdin_true should not be None"),
            format!("{}", Color::Green.bold().paint("강남스타일 "))
        );
        fixture.close()
    }

    #[test]
    fn when_true_with_string() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        let config = CustomConfigBuilder::new()
            .format("test")
            .when("true")
            .ignore_timeout(true)
            .build();

        assert_eq!(
            fixture
                .renderer()
                .config(config)
                .collect()
                .expect("when_true_with_string should not be None")
                .as_str(),
            "test"
        );

        fixture.close()
    }

    #[test]
    fn when_false_with_string() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        let config = CustomConfigBuilder::new()
            .format("test")
            .when("false")
            .ignore_timeout(true)
            .build();

        assert!(fixture.renderer().config(config).collect().is_none());

        fixture.close()
    }

    #[test]
    fn when_true_with_bool() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        assert_eq!(
            fixture
                .renderer()
                .config(toml::toml! {
                    [custom.test]
                    format = "test"
                    when = true
                })
                .collect()
                .expect("when_true_with_bool should not be None")
                .as_str(),
            "test"
        );

        fixture.close()
    }

    #[test]
    #[cfg(not(windows))]
    fn when_false_with_bool() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        assert!(
            fixture
                .renderer()
                .config(toml::toml! {
                    [custom.test]
                    format = "test"
                    when = false
                })
                .collect()
                .is_none()
        );

        fixture.close()
    }

    #[test]
    fn timeout_short_cmd() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        let when = if cfg!(windows) {
            "$true".to_owned()
        } else {
            "true".to_owned()
        };

        let shell = if cfg!(windows) {
            "powershell".to_owned()
        } else {
            "sh".to_owned()
        };

        let config = CustomConfigBuilder::new()
            .format("test")
            .when(when)
            .ignore_timeout(false)
            .shell(shell)
            .build();

        assert_eq!(
            fixture
                .renderer()
                .config(config)
                .collect()
                .expect("timeout_short_cmd should not be None")
                .as_str(),
            "test"
        );

        fixture.close()
    }

    #[test]
    fn timeout_cmd() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        let shell = if cfg!(windows) {
            "powershell".to_owned()
        } else {
            "sh".to_owned()
        };

        // Use a long timeout to ensure that the test doesn't fail
        let config = CustomConfigBuilder::new()
            .format("test")
            .when("sleep 3")
            .ignore_timeout(false)
            .shell(shell)
            .build();

        assert!(fixture.renderer().config(config).collect().is_none());

        fixture.close()
    }

    #[test]
    fn config_aliases_work() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        File::create(fixture.path().join("a.txt"))?;
        std::fs::create_dir(fixture.path().join("dir"))?;

        assert_eq!(
            fixture
                .renderer()
                .config(toml::toml! {
                    [custom.test]
                    format = "test"
                    files = ["a.txt"]
                })
                .collect()
                .expect("config_aliases_work: files a.txt should not return None")
                .as_str(),
            "test"
        );

        assert_eq!(
            fixture
                .renderer()
                .config(toml::toml! {
                    [custom.test]
                    format = "test"
                    extensions = ["txt"]
                })
                .collect()
                .expect("config_aliases_work: extensions txt should not return None")
                .as_str(),
            "test"
        );

        assert_eq!(
            fixture
                .renderer()
                .config(toml::toml! {
                    [custom.test]
                    format = "test"
                    directories = ["dir"]
                })
                .collect()
                .expect("config_aliases_work: directories dir should not return None")
                .as_str(),
            "test"
        );

        fixture.close()
    }

    #[test]
    fn disabled() {
        assert!(
            ModuleRenderer::new("custom.test")
                .config(
                    CustomConfigBuilder::new()
                        .disabled(true)
                        .when(true)
                        .format("test")
                        .build()
                )
                .collect()
                .is_none()
        );
    }

    #[test]
    fn test_render_require_repo_not_in() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        assert!(
            fixture
                .renderer()
                .config(
                    CustomConfigBuilder::new()
                        .when(true)
                        .require_repo(true)
                        .format("test")
                        .build()
                )
                .collect()
                .is_none()
        );

        fixture.close()
    }

    #[test]
    fn test_render_require_repo_in() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        assert_eq!(
            ModuleRenderer::new("custom.test")
                .path(repo_dir.path())
                .config(
                    CustomConfigBuilder::new()
                        .when(true)
                        .require_repo(true)
                        .format("test")
                        .build()
                )
                .collect()
                .expect("test_render_require_repo_in should not be None")
                .as_str(),
            "test"
        );

        repo_dir.close()
    }

    #[test]
    fn output_is_escaped() -> io::Result<()> {
        let fixture = TestFixture::new()?;
        assert_eq!(
            fixture
                .renderer()
                .config(
                    CustomConfigBuilder::new()
                        .format("$output")
                        .command("__starship_to_be_escaped")
                        .when(true)
                        .ignore_timeout(true)
                        .build()
                )
                .shell(Shell::Bash)
                .collect()
                .expect("output_is_escaped should not be None")
                .as_str(),
            "\\`to_be_escaped\\`"
        );

        fixture.close()
    }

    #[test]
    fn unsafe_no_escape() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        assert_eq!(
            fixture
                .renderer()
                .config(
                    CustomConfigBuilder::new()
                        .format("$output")
                        .command("__starship_to_be_escaped")
                        .when(true)
                        .ignore_timeout(true)
                        .unsafe_no_escape(true)
                        .build()
                )
                .shell(Shell::Bash)
                .collect()
                .expect("unsafe_no_escape should not be None")
                .as_str(),
            "`to_be_escaped`"
        );

        fixture.close()
    }

    #[test]
    fn command_can_check_exit_status() -> io::Result<()> {
        let fixture = TestFixture::new()?;

        let (command, shell_val) = if cfg!(windows) {
            (
                "if ($LASTEXITCODE -eq 42) { 'true' } else { 'false' }",
                Shell::PowerShell,
            )
        } else {
            ("[ $? -eq 42 ] && echo true || echo false", Shell::Bash)
        };

        let config = CustomConfigBuilder::new()
            .format("$output")
            .command(command)
            .when("true")
            .ignore_timeout(true)
            .build();

        assert_eq!(
            fixture
                .renderer()
                .status(42)
                .config(config)
                .shell(shell_val)
                .collect()
                .expect("command_can_check_exit_status should not be None")
                .as_str(),
            "true"
        );

        fixture.close()
    }

    #[test]
    #[cfg(not(windows))] // is fish available on win?   
    fn command_can_check_exit_status_fish() -> io::Result<()> {
        let fixture = TestFixture::new()?;
        let command = "[ $status -eq 42 ] && echo true || echo false";

        let config = CustomConfigBuilder::new()
            .format("$output")
            .command(command)
            .when("true")
            .ignore_timeout(true)
            .build();

        assert_eq!(
            fixture
                .renderer()
                .status(42)
                .config(config)
                .shell(Shell::Fish)
                .collect()
                .expect("fish test should not be None")
                .as_str(),
            "true"
        );

        fixture.close()
    }
}
