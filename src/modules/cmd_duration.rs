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
                "duration_in_milliseconds" => Some(Ok(format!("{elapsed}"))),
                "duration_modulo_milliseconds" => Some(Ok(format!("{}", elapsed % 1000))),
                "duration_in_seconds" => {
                    let seconds = elapsed / 1000;
                    (seconds > 0 || config.show_leading_zeros).then(|| Ok(format!("{seconds}")))
                }
                "duration_modulo_seconds" => {
                    let seconds = elapsed / 1000;
                    (seconds > 0 || config.show_leading_zeros)
                        .then(|| Ok(format!("{}", seconds % 60)))
                }
                "duration_in_minutes" => {
                    let minutes = elapsed / 60_000;
                    (minutes > 0 || config.show_leading_zeros).then(|| Ok(format!("{minutes}")))
                }
                "duration_modulo_minutes" => {
                    let minutes = elapsed / 60_000;
                    (minutes > 0 || config.show_leading_zeros)
                        .then(|| Ok(format!("{}", minutes % 60)))
                }
                "duration_in_hours" => {
                    let hours = elapsed / 3_600_000;
                    (hours > 0 || config.show_leading_zeros).then(|| Ok(format!("{hours}")))
                }
                "duration_modulo_hours" => {
                    let hours = elapsed / 3_600_000;
                    (hours > 0 || config.show_leading_zeros).then(|| Ok(format!("{}", hours % 24)))
                }
                "duration_in_days" => {
                    let days = elapsed / 86_400_000;
                    (days > 0 || config.show_leading_zeros).then(|| Ok(format!("{days}")))
                }
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

    if config.show_notifications && config.min_time_to_notify as u128 <= elapsed {
        if cfg!(target_os = "linux") {
            let in_graphical_session = ["DISPLAY", "WAYLAND_DISPLAY", "MIR_SOCKET"]
                .iter()
                .find_map(|&var| context.get_env(var).filter(|val| !val.is_empty()))
                .is_some();

            if !in_graphical_session {
                return module;
            };
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
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

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

    #[test]
    fn duration_milliseconds() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "[(($duration_in_milliseconds)ms)]($style)"
                min_time = 0
            })
            .cmd_duration(600)
            .collect();

        let expected = Some(format!("{}", Color::Yellow.bold().paint("600ms")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_seconds() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "[(($duration_in_seconds)s)]($style)"
            })
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("{}", Color::Yellow.bold().paint("5s")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_minutes() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "[(($duration_in_minutes)m)]($style)"
            })
            .cmd_duration(5 * 60 * 1000)
            .collect();

        let expected = Some(format!("{}", Color::Yellow.bold().paint("5m")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_hours() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "[(($duration_in_hours)h)]($style)"
            })
            .cmd_duration(5 * 60 * 60 * 1000)
            .collect();

        let expected = Some(format!("{}", Color::Yellow.bold().paint("5h")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_days() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "[(($duration_in_days)d)]($style)"
            })
            .cmd_duration(5 * 24 * 60 * 60 * 1000)
            .collect();

        let expected = Some(format!("{}", Color::Yellow.bold().paint("5d")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_with_spaces() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "[(($duration_in_days)d )(($duration_modulo_hours)h )(($duration_modulo_minutes)m )(($duration_modulo_seconds)s )(($duration_modulo_milliseconds)ms)]($style)"
            })
            .cmd_duration(100_000_600)
            .collect();

        let expected = Some(format!(
            "{}",
            Color::Yellow.bold().paint("1d 3h 46m 40s 600ms")
        ));
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_hours_with_zero_minutes() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "[(($duration_in_days)d )(($duration_modulo_hours)h )(($duration_modulo_minutes)m )(($duration_modulo_seconds)s)]($style)"
            })
            .cmd_duration(10_840_000)
            .collect();

        let expected = Some(format!("{}", Color::Yellow.bold().paint("3h 0m 40s")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn duration_show_leading_zeros() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "[(($duration_in_days)d )(($duration_modulo_hours)h )(($duration_modulo_minutes)m )(($duration_modulo_seconds)s)]($style)"
                show_leading_zeros = true
            })
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("{}", Color::Yellow.bold().paint("0d 0h 0m 5s")));
        assert_eq!(expected, actual);
    }
}
