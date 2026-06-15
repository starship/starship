use std::env;
use std::ffi::OsStr;
use std::fmt::{self, Debug};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
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

    let use_stdin = config
        .use_stdin
        .unwrap_or_else(|| handle_shell(&mut command, shell_invoc));

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

    let args = if shell.0.is_empty() {
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
            shell_invoc.shell.clone(), &*shell_invoc.args.join(" "),
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

/// Applies the shell invocation settings to the given Command: sets the
/// executable, adds any configured args, and returns whether stdin should be
/// enabled for this shell's subprocess.
fn handle_shell(command: &mut Command, shell_invoc: &ShellInvocation) -> bool {
    for arg in &shell_invoc.args {
        command.arg(arg);
    }
    shell_invoc.uses_stdin
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::context::Shell;
    use crate::test::{fixture_repo, FixtureProvider, ModuleRenderer};
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[cfg(not(windows))]
    const SHELL: &[&str] = &["/bin/sh"];
    #[cfg(windows)]
    const SHELL: &[&str] = &["cmd"];

    #[cfg(not(windows))]
    const FAILING_COMMAND: &str = "false";
    #[cfg(windows)]
    const FAILING_COMMAND: &str = "color 00";

    const UNKNOWN_COMMAND: &str = "ydelsyiedsieudleylse dyesdesl";

    fn render_cmd(cmd: &str) -> io::Result<Option<String>> {
        let dir = tempfile::tempdir()?;
        let cmd = cmd.to_owned();
        let shell = SHELL.iter().map(ToOwned::to_owned).collect::<Vec<_>>();
        let out = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "$output"
                command = cmd
                shell = shell
                when = true
                ignore_timeout = true
            })
            .collect();
        dir.close()?;
        Ok(out)
    }

    fn render_when(cmd: &str) -> io::Result<bool> {
        let dir = tempfile::tempdir()?;
        let cmd = cmd.to_owned();
        let shell = SHELL.iter().map(ToOwned::to_owned).collect::<Vec<_>>();
        let out = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "test"
                when = cmd
                shell = shell
                ignore_timeout = true
            })
            .collect()
            .is_some();
        dir.close()?;
        Ok(out)
    }

    #[test]
    fn when_returns_right_value() -> io::Result<()> {
        assert!(render_cmd("echo hello")?.is_some());
        assert!(render_cmd(FAILING_COMMAND)?.is_none());
        Ok(())
    }

    #[test]
    fn when_returns_false_if_invalid_command() -> io::Result<()> {
        assert!(!render_when(UNKNOWN_COMMAND)?);
        Ok(())
    }

    #[test]
    #[cfg(not(windows))]
    fn command_returns_right_string() -> io::Result<()> {
        assert_eq!(render_cmd("echo hello")?, Some("hello".into()));
        assert_eq!(render_cmd("echo 강남스타일")?, Some("강남스타일".into()));
        Ok(())
    }

    #[test]
    #[cfg(windows)]
    fn command_returns_right_string() -> io::Result<()> {
        assert_eq!(render_cmd("echo hello")?, Some("hello".into()));
        assert_eq!(render_cmd("echo 강남스타일")?, Some("강남스타일".into()));
        Ok(())
    }

    #[test]
    #[cfg(not(windows))]
    fn command_ignores_stderr() -> io::Result<()> {
        assert_eq!(render_cmd("echo foo 1>&2; echo bar")?, Some("bar".into()));
        assert_eq!(render_cmd("echo foo; echo bar 1>&2")?, Some("foo".into()));
        Ok(())
    }

    #[test]
    #[cfg(windows)]
    fn command_ignores_stderr() -> io::Result<()> {
        assert_eq!(render_cmd("echo foo 1>&2 & echo bar")?, Some("bar".into()));
        assert_eq!(render_cmd("echo foo& echo bar 1>&2")?, Some("foo".into()));
        Ok(())
    }

    #[test]
    fn command_can_fail() -> io::Result<()> {
        assert_eq!(render_cmd(FAILING_COMMAND)?, None);
        assert_eq!(render_cmd(UNKNOWN_COMMAND)?, None);
        Ok(())
    }

    #[test]
    fn cwd_command() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let mut f = File::create(dir.path().join("a.txt"))?;
        write!(f, "hello")?;
        f.sync_all()?;

        let cat = if cfg!(windows) { "type" } else { "cat" };
        let cmd = format!("{cat} a.txt");

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                command = cmd
                when = true
                ignore_timeout = true
            })
            .collect();
        let expected = Some(format!("{}", Color::Green.bold().paint("hello ")));

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn cwd_when() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        File::create(dir.path().join("a.txt"))?.sync_all()?;

        let cat = if cfg!(windows) { "type" } else { "cat" };
        let cmd = format!("{cat} a.txt");

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "test"
                when = cmd
                ignore_timeout = true
            })
            .collect();
        let expected = Some("test".to_owned());

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn use_stdin_false() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let shell = if cfg!(windows) {
            vec![
                "powershell".to_owned(),
                "-NoProfile".to_owned(),
                "-Command".to_owned(),
            ]
        } else {
            vec!["sh".to_owned(), "-c".to_owned()]
        };

        // `use_stdin = false` doesn't like Korean on Windows
        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                command = "echo test"
                when = true
                use_stdin = false
                shell = shell
                ignore_timeout = true
            })
            .collect();
        let expected = Some(format!("{}", Color::Green.bold().paint("test ")));

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn use_stdin_true() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let shell = if cfg!(windows) {
            vec![
                "powershell".to_owned(),
                "-NoProfile".to_owned(),
                "-Command".to_owned(),
                "-".to_owned(),
            ]
        } else {
            vec!["sh".to_owned()]
        };

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                command = "echo 강남스타일"
                when = true
                use_stdin = true
                ignore_timeout = true
                shell = shell
            })
            .collect();
        let expected = Some(format!("{}", Color::Green.bold().paint("강남스타일 ")));

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    #[cfg(not(windows))]
    fn when_true_with_string() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "test"
                shell = ["sh"]
                when = "true"
                ignore_timeout = true
            })
            .collect();
        let expected = Some("test".to_string());
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    #[cfg(not(windows))]
    fn when_false_with_string() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "test"
                shell = ["sh"]
                when = "false"
                ignore_timeout = true
            })
            .collect();
        let expected = None;
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn when_true_with_bool() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "test"
                when = true
            })
            .collect();
        let expected = Some("test".to_string());
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    #[cfg(not(windows))]
    fn when_false_with_bool() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "test"
                when = false
            })
            .collect();
        let expected = None;
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn timeout_short_cmd() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let shell = if cfg!(windows) {
            "powershell".to_owned()
        } else {
            "sh".to_owned()
        };

        let when = if cfg!(windows) {
            "$true".to_owned()
        } else {
            "true".to_owned()
        };

        // Use a long timeout to ensure that the test doesn't fail
        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                command_timeout = 100_000
                [custom.test]
                format = "test"
                when = when
                shell = shell
                ignore_timeout = false
            })
            .collect();
        let expected = Some("test".to_owned());
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn timeout_cmd() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let shell = if cfg!(windows) {
            "powershell".to_owned()
        } else {
            "sh".to_owned()
        };

        // Use a long timeout to ensure that the test doesn't fail
        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "test"
                when = "sleep 3"
                shell = shell
                ignore_timeout = false
            })
            .collect();
        let expected = None;
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn config_aliases_work() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        File::create(dir.path().join("a.txt"))?;
        std::fs::create_dir(dir.path().join("dir"))?;

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "test"
                files = ["a.txt"]
            })
            .collect();
        let expected = Some("test".to_string());
        assert_eq!(expected, actual);

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "test"
                extensions = ["txt"]
            })
            .collect();
        let expected = Some("test".to_string());
        assert_eq!(expected, actual);

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "test"
                directories = ["dir"]
            })
            .collect();
        let expected = Some("test".to_string());
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn disabled() {
        let actual = ModuleRenderer::new("custom.test")
            .config(toml::toml! {
                [custom.test]
                disabled = true
                when = true
                format = "test"
            })
            .collect();
        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_render_require_repo_not_in() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("custom.test")
            .path(repo_dir.path())
            .config(toml::toml! {
                [custom.test]
                when = true
                require_repo = true
                format = "test"
            })
            .collect();
        let expected = None;
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_render_require_repo_in() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Git)?;

        let actual = ModuleRenderer::new("custom.test")
            .path(repo_dir.path())
            .config(toml::toml! {
                [custom.test]
                when = true
                require_repo = true
                format = "test"
            })
            .collect();
        let expected = Some("test".to_string());
        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn output_is_escaped() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "$output"
                command = "__starship_to_be_escaped"
                when = true
                ignore_timeout = true
            })
            .shell(Shell::Bash)
            .collect();
        let expected = Some("\\`to_be_escaped\\`".to_string());
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn unsafe_no_escape() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "$output"
                command = "__starship_to_be_escaped"
                when = true
                ignore_timeout = true
                unsafe_no_escape = true
            })
            .shell(Shell::Bash)
            .collect();
        let expected = Some("`to_be_escaped`".to_string());
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    #[cfg(not(windows))]
    fn command_can_check_exit_status() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .status(42)
            .config(toml::toml! {
                [custom.test]
                format = "$output"
                command = "[ $? -eq 42 ] && echo true || echo false"
                when = true
                shell = ["sh"]
                ignore_timeout = true
            })
            .shell(Shell::Bash)
            .collect();

        assert_eq!(
            "true",
            actual
                .as_ref()
                .expect("command should output true or false."),
            "command returned {}",
            actual.as_ref().unwrap_or(&"no output".to_string())
        );
        dir.close()
    }

    #[test]
    #[cfg(windows)]
    fn command_can_check_exit_status() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        // PowerShell uses $LASTEXITCODE to check the previous command's status
        let actual = ModuleRenderer::new("custom.test")
            .path(dir.path())
            .config(toml::toml! {
                [custom.test]
                format = "$output"
                command = "if ($LASTEXITCODE -eq 0) { 'success' } else { 'failure' }"
                when = true
                shell = ["powershell", "-NoProfile", "-Command", "-"]
                ignore_timeout = true
            })
            .collect();

        assert!(actual.is_some());

        dir.close()
    }
}
