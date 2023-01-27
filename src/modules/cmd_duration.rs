use super::{Context, Module, ModuleConfig};

use crate::configs::cmd_duration::CmdDurationConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::duration::Duration;

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
    let config_min = config.min_time as u64;

    if elapsed < config_min {
        return None;
    }

    let duration = Duration::from(elapsed);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "duration" | "humanized" => {
                    Some(Ok(duration.fmt_humanized(config.show_milliseconds)))
                }
                "d" => Some(Ok(duration.fmt_days(false))),
                "dd" => Some(Ok(duration.fmt_days(true))),
                "h" => Some(Ok(duration.fmt_hours(false))),
                "hh" => Some(Ok(duration.fmt_hours(true))),
                "m" => Some(Ok(duration.fmt_minutes(false))),
                "mm" => Some(Ok(duration.fmt_minutes(true))),
                "s" => Some(Ok(duration.fmt_seconds(false))),
                "ss" => Some(Ok(duration.fmt_seconds(true))),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `cmd_duration`: \n{}", error);
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
    _elapsed: u64,
) -> Module<'a> {
    module
}

#[cfg(feature = "notify")]
fn undistract_me<'a>(
    module: Module<'a>,
    config: &CmdDurationConfig,
    context: &'a Context,
    elapsed: u64,
) -> Module<'a> {
    use notify_rust::{Notification, Timeout};
    use nu_ansi_term::{unstyle, AnsiStrings};

    if config.show_notifications && config.min_time_to_notify as u64 <= elapsed {
        if cfg!(target_os = "linux") {
            let in_graphical_session = ["DISPLAY", "WAYLAND_DISPLAY", "MIR_SOCKET"]
                .iter()
                .find_map(|&var| context.get_env(var).filter(|val| !val.is_empty()))
                .is_some();

            if !in_graphical_session {
                return module;
            };
        }

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
            log::trace!("Cannot show notification: {}", err);
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
    fn config_format_hhmmss_duration_5s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "[${hh}:${mm}:${ss}]($style) "
            })
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("{} ", Color::Yellow.bold().paint("00:00:05")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_format_hms_duration_12h5m4s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "[${h}:${m}:${s}]($style) "
            })
            .cmd_duration(43_504_000)
            .collect();

        let expected = Some(format!("{} ", Color::Yellow.bold().paint("12:5:4")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_format_hhmmss_duration_3h30m15s_prefix_underwent() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "underwent [${hh}:${mm}:${ss}]($style) "
            })
            .cmd_duration(12_615_000)
            .collect();

        let expected = Some(format!(
            "underwent {} ",
            Color::Yellow.bold().paint("03:30:15")
        ));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_humanized_duration_337ms() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 200
                show_milliseconds = true
                format = "[${duration}]($style) "
            })
            .cmd_duration(337)
            .collect();

        let expected = Some(format!("{} ", Color::Yellow.bold().paint("337ms")));
        assert_eq!(expected, actual);
    }
}
