use std::env;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::time::Instant;

use super::{Context, Module, RootModuleConfig};

use crate::{configs::custom::CustomConfig, formatter::StringFormatter, utils::create_command};

/// Creates a custom module with some configuration
///
/// The relevant TOML config will set the files, extensions, and directories needed
/// for the module to be displayed. If none of them match, and optional "when"
/// command can be run -- if its result is 0, the module will be shown.
///
/// Finally, the content of the module itself is also set by a command.
pub fn module<'a>(name: &str, context: &'a Context) -> Option<Module<'a>> {
    let start: Instant = Instant::now();
    let toml_config = context.config.get_custom_module_config(name).expect(
        "modules::custom::module should only be called after ensuring that the module exists",
    );
    let config = CustomConfig::load(toml_config);

    if let Some(os) = config.os {
        if os != env::consts::OS && !(os == "unix" && cfg!(unix)) {
            return None;
        }
    }

    let mut is_match = context
        .try_begin_scan()?
        .set_files(&config.files)
        .set_extensions(&config.extensions)
        .set_folders(&config.directories)
        .is_match();

    if !is_match {
        if let Some(when) = config.when {
            is_match = exec_when(when, &config.shell.0);
        }

        if !is_match {
            return None;
        }
    }

    let mut module = Module::new(name, config.description, Some(toml_config));

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
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
                    let output = exec_command(config.command, &config.shell.0)?;
                    let trimmed = output.trim();

                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(Ok(trimmed.to_string()))
                    }
                }
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
    let elapsed = start.elapsed();
    log::trace!("Took {:?} to compute custom module {:?}", elapsed, name);
    module.duration = elapsed;
    Some(module)
}

/// Return the invoking shell, using `shell` and fallbacking in order to STARSHIP_SHELL and "sh"
#[cfg(not(windows))]
fn get_shell<'a, 'b>(shell_args: &'b [&'a str]) -> (std::borrow::Cow<'a, str>, &'b [&'a str]) {
    if !shell_args.is_empty() {
        (shell_args[0].into(), &shell_args[1..])
    } else if let Ok(env_shell) = std::env::var("STARSHIP_SHELL") {
        (env_shell.into(), &[] as &[&str])
    } else {
        ("sh".into(), &[] as &[&str])
    }
}

/// Attempt to run the given command in a shell by passing it as `stdin` to `get_shell()`
#[cfg(not(windows))]
fn shell_command(cmd: &str, shell_args: &[&str]) -> Option<Output> {
    let (shell, shell_args) = get_shell(shell_args);
    let mut command = create_command(shell.as_ref()).ok()?;

    command
        .args(shell_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    handle_powershell(&mut command, &shell, shell_args);

    let mut child = match command.spawn() {
        Ok(command) => command,
        Err(err) => {
            log::trace!("Error executing command: {:?}", err);
            log::debug!(
                "Could not launch command with given shell or STARSHIP_SHELL env variable, retrying with /usr/bin/env sh"
            );

            #[allow(clippy::disallowed_method)]
            Command::new("/usr/bin/env")
                .arg("sh")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .ok()?
        }
    };

    child.stdin.as_mut()?.write_all(cmd.as_bytes()).ok()?;
    child.wait_with_output().ok()
}

/// Attempt to run the given command in a shell by passing it as `stdin` to `get_shell()`,
/// or by invoking cmd.exe /C.
#[cfg(windows)]
fn shell_command(cmd: &str, shell_args: &[&str]) -> Option<Output> {
    let (shell, shell_args) = if !shell_args.is_empty() {
        (
            Some(std::borrow::Cow::Borrowed(shell_args[0])),
            &shell_args[1..],
        )
    } else if let Some(env_shell) = std::env::var("STARSHIP_SHELL")
        .ok()
        .filter(|s| !cfg!(test) && !s.is_empty())
    {
        (Some(std::borrow::Cow::Owned(env_shell)), &[] as &[&str])
    } else {
        (None, &[] as &[&str])
    };

    if let Some(forced_shell) = shell {
        let mut command = create_command(forced_shell.as_ref()).ok()?;

        command
            .args(shell_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if forced_shell.ends_with("cmd.exe") || forced_shell.ends_with("cmd") {
            log::error!("Custom command output with Command shell is not supported. Use PowerShell instead.");
            return None;
        }

        handle_powershell(&mut command, &forced_shell, shell_args);

        if let Ok(mut child) = command.spawn() {
            child.stdin.as_mut()?.write_all(cmd.as_bytes()).ok()?;

            return child.wait_with_output().ok();
        }

        log::debug!(
            "Could not launch command with given shell or STARSHIP_SHELL env variable, retrying with cmd.exe /C"
        );
    }

    let command = create_command("cmd")
        .ok()?
        .arg("/C")
        .arg(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    command.ok()?.wait_with_output().ok()
}

/// Execute the given command capturing all output, and return whether it return 0
fn exec_when(cmd: &str, shell_args: &[&str]) -> bool {
    log::trace!("Running '{}'", cmd);

    if let Some(output) = shell_command(cmd, shell_args) {
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
fn exec_command(cmd: &str, shell_args: &[&str]) -> Option<String> {
    log::trace!("Running '{}'", cmd);

    if let Some(output) = shell_command(cmd, shell_args) {
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

/// If the specified shell refers to PowerShell, adds the arguments "-Command -" to the
/// given command.
fn handle_powershell(command: &mut Command, shell: &str, shell_args: &[&str]) {
    let is_powershell = shell.ends_with("pwsh.exe")
        || shell.ends_with("powershell.exe")
        || shell.ends_with("pwsh")
        || shell.ends_with("powershell");

    if is_powershell && shell_args.is_empty() {
        command.arg("-NoProfile").arg("-Command").arg("-");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(windows))]
    const SHELL: &[&str] = &["/bin/sh"];
    #[cfg(windows)]
    const SHELL: &[&str] = &[];

    #[cfg(not(windows))]
    const FAILING_COMMAND: &str = "false";
    #[cfg(windows)]
    const FAILING_COMMAND: &str = "color 00";

    const UNKNOWN_COMMAND: &str = "ydelsyiedsieudleylse dyesdesl";

    #[test]
    fn when_returns_right_value() {
        assert!(exec_when("echo hello", SHELL));
        assert!(!exec_when(FAILING_COMMAND, SHELL));
    }

    #[test]
    fn when_returns_false_if_invalid_command() {
        assert!(!exec_when(UNKNOWN_COMMAND, SHELL));
    }

    #[test]
    #[cfg(not(windows))]
    fn command_returns_right_string() {
        assert_eq!(exec_command("echo hello", SHELL), Some("hello\n".into()));
        assert_eq!(
            exec_command("echo 강남스타일", SHELL),
            Some("강남스타일\n".into())
        );
    }

    #[test]
    #[cfg(windows)]
    fn command_returns_right_string() {
        assert_eq!(exec_command("echo hello", SHELL), Some("hello\r\n".into()));
        assert_eq!(
            exec_command("echo 강남스타일", SHELL),
            Some("강남스타일\r\n".into())
        );
    }

    #[test]
    #[cfg(not(windows))]
    fn command_ignores_stderr() {
        assert_eq!(
            exec_command("echo foo 1>&2; echo bar", SHELL),
            Some("bar\n".into())
        );
        assert_eq!(
            exec_command("echo foo; echo bar 1>&2", SHELL),
            Some("foo\n".into())
        );
    }

    #[test]
    #[cfg(windows)]
    fn command_ignores_stderr() {
        assert_eq!(
            exec_command("echo foo 1>&2 & echo bar", SHELL),
            Some("bar\r\n".into())
        );
        assert_eq!(
            exec_command("echo foo& echo bar 1>&2", SHELL),
            Some("foo\r\n".into())
        );
    }

    #[test]
    fn command_can_fail() {
        assert_eq!(exec_command(FAILING_COMMAND, SHELL), None);
        assert_eq!(exec_command(UNKNOWN_COMMAND, SHELL), None);
    }
}
