use crate::utils::CommandOutput;
use std::process::ExitStatus;
use std::time::Instant;

use super::{Context, Module, RootModuleConfig};

use crate::{configs::custom::CustomConfig, formatter::StringFormatter};

/// Creates a custom module with some configuration
///
/// The relevant TOML config will set the files, extensions, and directories needed
/// for the module to be displayed. If none of them match, and optional "when"
/// command can be run -- if its result is 0, the module will be shown.
///
/// Finally, the content of the module itself is also set by a command.
pub async fn module<'a>(name: &str, context: &'a Context<'a>) -> Option<Module<'a>> {
    let start: Instant = Instant::now();
    let toml_config = context.config.get_custom_module_config(name).expect(
        "modules::custom::module should only be called after ensuring that the module exists",
    );
    let config = CustomConfig::load(toml_config);

    let mut scan_dir = context.try_begin_scan()?;

    if !config.files.0.is_empty() {
        scan_dir = scan_dir.set_files(&config.files.0);
    }
    if !config.extensions.0.is_empty() {
        scan_dir = scan_dir.set_extensions(&config.extensions.0);
    }
    if !config.directories.0.is_empty() {
        scan_dir = scan_dir.set_folders(&config.directories.0);
    }

    let mut is_match = scan_dir.is_match();

    if !is_match {
        if let Some(when) = config.when {
            is_match = exec_command(context, when, &config.shell.0).await.is_some();
        }

        if !is_match {
            return None;
        }
    }

    let mut module = Module::new(name, config.description, Some(toml_config));

    let parsed = match StringFormatter::new(config.format) {
        Ok(formatter) => formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .async_map(|variable| {
                let config = &config;
                async move {
                    match variable.as_ref() {
                        "output" => {
                            let output =
                                exec_command(context, config.command, &config.shell.0).await?;
                            let trimmed = output.trim();

                            if trimmed.is_empty() {
                                None
                            } else {
                                Some(Ok(trimmed.to_string()))
                            }
                        }
                        _ => None,
                    }
                }
            })
            .await
            .parse(None),
        Err(e) => Err(e),
    };

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
async fn shell_command(
    context: &Context<'_>,
    cmd: &str,
    shell_args: &[&str],
) -> Option<(ExitStatus, CommandOutput)> {
    let (shell, shell_args) = get_shell(shell_args);

    let shell = shell.as_ref();
    let shell_args = adjust_args_for_powershell(shell, shell_args);

    if let Some(ret) = context.exec_cmd_with_stdin(shell, shell_args, cmd).await {
        return Some(ret);
    }

    log::debug!(
        "Could not launch command with given shell or STARSHIP_SHELL env variable, retrying with /usr/bin/env sh"
    );
    context
        .exec_cmd_with_stdin("/usr/bin/env", &["sh"], cmd)
        .await
}

/// Attempt to run the given command in a shell by passing it as `stdin` to `get_shell()`,
/// or by invoking cmd.exe /C.
#[cfg(windows)]
async fn shell_command(
    context: &Context<'_>,
    cmd: &str,
    shell_args: &[&str],
) -> Option<(ExitStatus, CommandOutput)> {
    let (shell, shell_args) = if !shell_args.is_empty() {
        (
            Some(std::borrow::Cow::Borrowed(shell_args[0])),
            &shell_args[1..],
        )
    } else if let Ok(env_shell) = std::env::var("STARSHIP_SHELL") {
        (Some(std::borrow::Cow::Owned(env_shell)), &[] as &[&str])
    } else {
        (None, &[] as &[&str])
    };

    if let Some(forced_shell) = shell {
        let forced_shell = forced_shell.as_ref();
        let shell_args = adjust_args_for_powershell(forced_shell, shell_args);

        if let Some(ret) = context
            .exec_cmd_with_stdin(forced_shell, shell_args, cmd)
            .await
        {
            return Some(ret);
        }

        log::debug!(
            "Could not launch command with given shell or STARSHIP_SHELL env variable, retrying with cmd.exe /C"
        );
    }

    context.exec_cmd_status("cmd.exe", &["/C", cmd]).await
}

/// Execute the given command, returning its output on success
async fn exec_command(context: &Context<'_>, cmd: &str, shell_args: &[&str]) -> Option<String> {
    log::trace!("Running '{}'", cmd);

    if let Some((status, output)) = shell_command(context, cmd, shell_args).await {
        if !status.success() {
            log::trace!("Non-zero exit code '{:?}'", status.code());
            log::trace!("stdout: {}", output.stdout);
            log::trace!("stderr: {}", output.stderr);
            return None;
        }

        Some(output.stdout)
    } else {
        None
    }
}

/// If the specified shell refers to PowerShell, adds the arguments "-Command -" to the
/// given command.
fn adjust_args_for_powershell<'a>(shell: &str, shell_args: &'a [&'a str]) -> &'a [&'a str] {
    let is_powershell = shell.ends_with("pwsh.exe")
        || shell.ends_with("powershell.exe")
        || shell.ends_with("pwsh")
        || shell.ends_with("powershell");

    if is_powershell && shell_args.is_empty() {
        &["-NoProfile", "-Command", "-"]
    } else {
        shell_args
    }
}

#[cfg(test)]
mod tests {

    #[cfg(not(windows))]
    const SHELL: &[&str] = &["/bin/sh"];
    #[cfg(windows)]
    const SHELL: &[&str] = &[];

    #[cfg(not(windows))]
    const FAILING_COMMAND: &str = "false";
    #[cfg(windows)]
    const FAILING_COMMAND: &str = "color 00";

    const UNKNOWN_COMMAND: &str = "ydelsyiedsieudleylse dyesdesl";

    fn exec_command(cmd: &str, shell_args: &[&str]) -> Option<String> {
        use crate::config::StarshipConfig;
        use crate::modules::{Context, Shell};
        use std::path::PathBuf;

        let mut context = Context::new_with_shell_and_path(
            clap::ArgMatches::default(),
            Shell::Unknown,
            PathBuf::new(),
            PathBuf::new(),
        );
        context.config = StarshipConfig { config: None };
        async_std::task::block_on(super::exec_command(&context, cmd, shell_args))
    }

    #[test]
    fn when_returns_right_value() {
        assert!(exec_command("echo hello", SHELL).is_some());
        assert!(!exec_command(FAILING_COMMAND, SHELL).is_some());
    }

    #[test]
    fn when_returns_false_if_invalid_command() {
        assert!(!exec_command(UNKNOWN_COMMAND, SHELL).is_some());
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
