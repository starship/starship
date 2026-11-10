use super::{Context, Module, ModuleConfig};

use crate::configs::cmd_duration::CmdDurationConfig;
use crate::formatter::StringFormatter;
use crate::utils::render_time;

/// Outputs the time it took the last command to execute
///
/// Will only print if last command took more than a certain amount of time to
/// execute. Default is two seconds, but can be set by config option `min_time`.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("cmd_duration");
    let config: CmdDurationConfig = CmdDurationConfig::try_load(module.config);

    if config.min_time < 0 {
        log::warn!(
            "min_time in [cmd_duration] ({}) was less than zero",
            config.min_time
        );
        return None;
    }

    let elapsed = context.get_cmd_duration()?;
    let config_min = config.min_time as u128;

    if elapsed < config_min {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "duration" => Some(Ok(render_time(elapsed, config.show_milliseconds))),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `cmd_duration`: \n{error}");
            return None;
        }
    });

    Some(undistract_me(module, &config, context, elapsed))
}

/// Returns `true` when a desktop notification should be shown for the last command.
#[cfg_attr(not(feature = "notify"), allow(dead_code))]
fn should_notify(config: &CmdDurationConfig, last_command: Option<&str>, elapsed: u128) -> bool {
    if !config.show_notifications || (config.min_time_to_notify as u128) > elapsed {
        return false;
    }

    match last_command {
        Some(command) => !is_ignored_command(command, &config.notify_ignore_commands),
        None => true,
    }
}

/// Wrappers that are skipped when looking for the name of the command that was run,
/// each with its options that take a separate argument (as in `sudo -u root vim`).
const COMMAND_PREFIXES: &[(&str, &[&str])] = &[
    (
        "sudo",
        &[
            "-u",
            "--user",
            "-g",
            "--group",
            "-h",
            "--host",
            "-p",
            "--prompt",
            "-C",
            "--close-from",
            "-D",
            "--chdir",
            "-r",
            "--role",
            "-t",
            "--type",
            "-U",
            "--other-user",
            "-T",
            "--command-timeout",
        ],
    ),
    ("doas", &["-u", "-C"]),
    ("env", &["-u", "--unset", "-C", "--chdir"]),
    ("command", &[]),
    ("builtin", &[]),
    ("exec", &["-a"]),
    ("nohup", &[]),
    ("time", &["-f", "--format", "-o", "--output"]),
];

/// Checks whether the program run by `command` is listed in `ignored`.
///
/// Leading environment variable assignments (`FOO=bar cmd`) and common wrappers
/// such as `sudo` (with their options) are skipped, and the program is compared
/// by its file name, so `/usr/bin/vim` and `sudo -u root vim` both match `vim`.
fn is_ignored_command(command: &str, ignored: &[&str]) -> bool {
    if ignored.is_empty() {
        return false;
    }

    let mut words = command.split_whitespace();
    let mut options_with_argument: &[&str] = &[];
    let program = loop {
        let Some(word) = words.next() else {
            return false;
        };
        let name = word.rsplit('/').next().unwrap_or(word);
        if let Some((_, options)) = COMMAND_PREFIXES.iter().find(|(prefix, _)| *prefix == name) {
            options_with_argument = options;
        } else if word.starts_with('-') {
            if options_with_argument.contains(&word) {
                words.next();
            }
        } else if !(word.contains('=') && !word.starts_with('=')) {
            break word;
        }
    };

    let name = program.rsplit('/').next().unwrap_or(program);
    ignored
        .iter()
        .any(|ignored| *ignored == name || *ignored == program)
}

#[cfg(not(feature = "notify"))]
fn undistract_me<'a>(
    module: Module<'a>,
    _config: &CmdDurationConfig,
    _context: &'a Context,
    _elapsed: u128,
) -> Module<'a> {
    module
}

#[cfg(feature = "notify")]
fn undistract_me<'a>(
    module: Module<'a>,
    config: &CmdDurationConfig,
    context: &'a Context,
    elapsed: u128,
) -> Module<'a> {
    use notify_rust::{Notification, Timeout};
    use nu_ansi_term::{AnsiStrings, unstyle};

    if should_notify(config, context.properties.last_command.as_deref(), elapsed) {
        if cfg!(target_os = "linux") {
            let in_graphical_session = ["DISPLAY", "WAYLAND_DISPLAY", "MIR_SOCKET"]
                .iter()
                .find_map(|&var| context.get_env(var).filter(|val| !val.is_empty()))
                .is_some();

            if !in_graphical_session {
                return module;
            }
        }

        // On macOS 26+ notify-rust will get stuck finding the current application identifier
        // so we set it manually to the default terminal app.
        #[cfg(target_os = "macos")]
        let _ = notify_rust::set_application("com.apple.Terminal");

        let body = format!(
            "Command execution {}",
            unstyle(&AnsiStrings(&module.ansi_strings()))
        );

        let timeout = match config.notification_timeout {
            Some(v) => Timeout::Milliseconds(v),
            None => Timeout::Default,
        };

        let mut notification = Notification::new();
        notification
            .summary("Command finished")
            .body(&body)
            .icon("utilities-terminal")
            .timeout(timeout);

        if let Err(err) = notification.show() {
            log::trace!("Cannot show notification: {err}");
        }
    }

    module
}

#[cfg(test)]
mod tests {
    use super::{is_ignored_command, should_notify};
    use crate::configs::cmd_duration::CmdDurationConfig;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn ignored_command_matches_program_name() {
        let ignored = ["vim", "less"];
        assert!(is_ignored_command("vim src/main.rs", &ignored));
        assert!(is_ignored_command("less", &ignored));
        assert!(is_ignored_command("  vim  ", &ignored));
        assert!(!is_ignored_command("cargo build", &ignored));
        assert!(!is_ignored_command("vimdiff a b", &ignored));
        assert!(!is_ignored_command("echo vim", &ignored));
        assert!(!is_ignored_command("", &ignored));
    }

    #[test]
    fn ignored_command_skips_prefixes_and_env_assignments() {
        let ignored = ["vim"];
        assert!(is_ignored_command("sudo vim /etc/hosts", &ignored));
        assert!(is_ignored_command("sudo -E vim /etc/hosts", &ignored));
        assert!(is_ignored_command("doas vim /etc/hosts", &ignored));
        assert!(is_ignored_command("EDITOR=nano TERM=xterm vim", &ignored));
        assert!(is_ignored_command("/usr/bin/vim file", &ignored));
        assert!(!is_ignored_command("sudo make install", &ignored));
        assert!(is_ignored_command("sudo -u root vim /etc/hosts", &ignored));
        assert!(is_ignored_command("sudo --user root -E vim", &ignored));
        assert!(is_ignored_command("doas -u admin vim", &ignored));
        assert!(is_ignored_command("env EDITOR=nano vim", &ignored));
        assert!(is_ignored_command("env -u TERM vim", &ignored));
        assert!(is_ignored_command("/usr/bin/sudo vim", &ignored));
        assert!(is_ignored_command("time -f %e vim", &ignored));
        assert!(!is_ignored_command("sudo -u vim make", &ignored));
    }

    #[test]
    fn ignored_command_accepts_full_paths_in_config() {
        assert!(is_ignored_command("/opt/bin/htop", &["/opt/bin/htop"]));
        assert!(!is_ignored_command("vim", &[]));
    }

    #[test]
    fn notification_suppressed_for_ignored_command() {
        let config = CmdDurationConfig {
            show_notifications: true,
            min_time_to_notify: 1_000,
            notify_ignore_commands: vec!["vim", "ssh"],
            ..CmdDurationConfig::default()
        };

        assert!(!should_notify(&config, Some("ssh my-server"), 60_000));
        assert!(should_notify(&config, Some("cargo test"), 60_000));
        assert!(!should_notify(&config, Some("cargo test"), 500));

        // Shells that don't report the last command keep the previous behaviour.
        assert!(should_notify(&config, None, 60_000));
    }

    #[test]
    fn notification_disabled_ignores_list() {
        let config = CmdDurationConfig {
            show_notifications: false,
            min_time_to_notify: 1_000,
            ..CmdDurationConfig::default()
        };
        assert!(!should_notify(&config, Some("cargo test"), 60_000));
    }

    #[test]
    fn notify_ignore_commands_does_not_hide_module() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                notify_ignore_commands = ["vim"]
            })
            .last_command("vim notes.md")
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("took {} ", Color::Yellow.bold().paint("5s")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_duration_1s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(1000)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_duration_5s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("took {} ", Color::Yellow.bold().paint("5s")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_5s_duration_3s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 5000
            })
            .cmd_duration(3000)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_5s_duration_10s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 5000
            })
            .cmd_duration(10000)
            .collect();

        let expected = Some(format!("took {} ", Color::Yellow.bold().paint("10s")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_1s_duration_prefix_underwent() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "underwent [$duration]($style) "
            })
            .cmd_duration(1000)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_5s_duration_prefix_underwent() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "underwent [$duration]($style) "
            })
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("underwent {} ", Color::Yellow.bold().paint("5s")));
        assert_eq!(expected, actual);
    }
}
