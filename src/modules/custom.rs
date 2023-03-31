use std::env;
use std::fmt::{self, Debug};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

use process_control::{ChildExt, Control, ExitStatus, Output};

use super::{Context, Module, ModuleConfig};

use crate::{
    config::Either, configs::custom::CustomConfig, formatter::StringFormatter,
    utils::create_command,
};

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

    if let Some(os) = config.os {
        if os != env::consts::OS && !(os == "unix" && cfg!(unix)) {
            return None;
        }
    }

    // Note: Forward config if `Module` ends up needing `config`
    let mut module = Module::new(&format!("custom.{name}"), config.description, None);

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

    let (output, status) = exec_command(config.command, context, &config)?;
    status.code()?;
    let status_code = status.code().unwrap();
    let format = config
        .formats
        .get(&status_code.to_string())
        .unwrap_or(&config.format);

    let parsed = StringFormatter::new(format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map_no_escaping(|variable| match variable {
                "output" => {
                    let trimmed = output.trim();

                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(Ok(trimmed.to_string()))
                    }
                }
                "status" => Some(Ok(status_code.to_string())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    match parsed {
        Ok(segments) => module.set_segments(segments),
        Err(error) => {
            log::warn!("Error in module `custom.{}`:\n{}", name, error);
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

/// Return the invoking shell, using `shell` and fallbacking in order to `STARSHIP_SHELL` and "sh"/"cmd"
fn get_shell<'a, 'b>(
    shell_args: &'b [&'a str],
    context: &Context,
) -> (std::borrow::Cow<'a, str>, &'b [&'a str]) {
    if !shell_args.is_empty() {
        (shell_args[0].into(), &shell_args[1..])
    } else if let Some(env_shell) = context.get_env("STARSHIP_SHELL") {
        (env_shell.into(), &[] as &[&str])
    } else if cfg!(windows) {
        // `/C` is added by `handle_shell`
        ("cmd".into(), &[] as &[&str])
    } else {
        ("sh".into(), &[] as &[&str])
    }
}

fn get_cmd_wrapper(shell: &str, cmd: &str, use_stdin: bool) -> String {
    match shell {
        "powershell" | "pwsh" => {
            format!("{}; exit $LASTEXITCODE", cmd)
        }
        "bash" | "sh" => {
            let (pre, escape, post) = match use_stdin {
                true => ("", "", ""),
                false => ("\"", "\\", "\""),
            };
            format!("{}{}; exit {}$?{}", pre, cmd, escape, post)
        }
        _ => cmd.to_string(),
    }
}

/// Attempt to run the given command in a shell by passing it as either `stdin` or an argument to `get_shell()`,
/// depending on the configuration or by invoking a platform-specific fallback shell if `shell` is empty.
fn shell_command(cmd: &str, config: &CustomConfig, context: &Context) -> Option<Output> {
    let (shell, shell_args) = get_shell(config.shell.0.as_ref(), context);
    let mut use_stdin = config.use_stdin;

    let mut command = match create_command(shell.as_ref()) {
        Ok(command) => command,
        // Don't attempt to use fallback shell if the user specified a shell
        Err(error) if !shell_args.is_empty() => {
            log::debug!(
                "Error creating command with STARSHIP_SHELL, falling back to fallback shell: {}",
                error
            );

            // Skip `handle_shell` and just set the shell and command
            use_stdin = Some(!cfg!(windows));

            if cfg!(windows) {
                let mut c = create_command("cmd").ok()?;
                c.arg("/C");
                c
            } else {
                let mut c = create_command("/usr/bin/env").ok()?;
                c.arg("sh");
                c
            }
        }
        _ => return None,
    };

    command
        .current_dir(&context.current_dir)
        .args(shell_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let use_stdin = use_stdin.unwrap_or_else(|| handle_shell(&mut command, &shell, shell_args));

    let cmd_wrapper = get_cmd_wrapper(&shell, cmd, use_stdin);

    if !use_stdin {
        command.arg(cmd_wrapper.clone());
    }

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            log::debug!(
                "Failed to run command with given shell or STARSHIP_SHELL env variable:: {}",
                error
            );
            return None;
        }
    };

    if use_stdin {
        child
            .stdin
            .as_mut()?
            .write_all(cmd_wrapper.as_bytes())
            .ok()?;
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
            log::warn!("You can set command_timeout in your config to a higher value or set ignore_timeout to true for this module to allow longer-running commands to keep executing.");
            None
        }
        Some(status) => Some(status),
    }
}

/// Execute the given command capturing all output, and return whether it return 0
fn exec_when(cmd: &str, config: &CustomConfig, context: &Context) -> bool {
    log::trace!("Running '{}'", cmd);

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
fn exec_command(
    cmd: &str,
    context: &Context,
    config: &CustomConfig,
) -> Option<(String, ExitStatus)> {
    log::trace!("Running '{cmd}'");

    if let Some(output) = shell_command(cmd, config, context) {
        Some((
            String::from_utf8_lossy(&output.stdout).into(),
            output.status,
        ))
    } else {
        None
    }
}

/// If the specified shell refers to `PowerShell`, adds the arguments "-Command -" to the
/// given command.
/// Returns `false` if the shell shell expects scripts as arguments, `true` if as `stdin`.
fn handle_shell(command: &mut Command, shell: &str, shell_args: &[&str]) -> bool {
    let shell_exe = Path::new(shell).file_stem();
    let no_args = shell_args.is_empty();

    match shell_exe.and_then(std::ffi::OsStr::to_str) {
        Some("pwsh" | "powershell") => {
            if no_args {
                command.arg("-NoProfile").arg("-Command").arg("-");
            }
            true
        }
        Some("cmd") => {
            if no_args {
                command.arg("/C");
            }
            false
        }
        Some("nu") => {
            if no_args {
                command.arg("-c");
            }
            false
        }
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test::ModuleRenderer;
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
        let shell = SHELL
            .iter()
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<_>>();
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
        let shell = SHELL
            .iter()
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<_>>();
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
    fn when_true_with_string() -> std::io::Result<()> {
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
    fn when_false_with_string() -> std::io::Result<()> {
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
    fn when_true_with_bool() -> std::io::Result<()> {
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
    fn when_false_with_bool() -> std::io::Result<()> {
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
    fn timeout_short_cmd() -> std::io::Result<()> {
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
    fn timeout_cmd() -> std::io::Result<()> {
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
    fn config_aliases_work() -> std::io::Result<()> {
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
}
