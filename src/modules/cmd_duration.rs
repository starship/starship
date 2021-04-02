use super::{Context, Module, RootModuleConfig};

use crate::configs::cmd_duration::CmdDurationConfig;
use crate::formatter::StringFormatter;

struct Duration {
    pub days: u128,
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub millis: u16,
}

impl From<u128> for Duration {
    fn from(raw_millis: u128) -> Self {
        // Calculate a simple breakdown into days/hours/minutes/seconds/milliseconds
        let (millis, raw_seconds) = ((raw_millis % 1000) as u16, raw_millis / 1000);
        let (seconds, raw_minutes) = ((raw_seconds % 60) as u8, raw_seconds / 60);
        let (minutes, raw_hours) = ((raw_minutes % 60) as u8, raw_minutes / 60);
        let (hours, days) = ((raw_hours % 24) as u8, raw_hours / 24);

        Self {
            days,
            hours,
            minutes,
            seconds,
            millis,
        }
    }
}

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
    if elapsed < config.min_time as u128 {
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
                "d" => (duration.days > 0)
                    .then(|| duration.days.to_string())
                    .map(Ok),
                "hh" => (duration.hours > 0)
                    .then(|| format!("{:02}", duration.hours))
                    .map(Ok),
                "h" => (duration.hours > 0)
                    .then(|| duration.hours.to_string())
                    .map(Ok),
                "mm" => (duration.minutes > 0)
                    .then(|| format!("{:02}", duration.minutes))
                    .map(Ok),
                "m" => (duration.minutes > 0)
                    .then(|| duration.minutes.to_string())
                    .map(Ok),
                "ss" => (duration.seconds > 0)
                    .then(|| format!("{:02}", duration.seconds))
                    .map(Ok),
                "s" => (duration.seconds > 0)
                    .then(|| duration.seconds.to_string())
                    .map(Ok),
                "SSS" => (config.show_milliseconds && duration.millis > 0)
                    .then(|| format!("{:03}", duration.millis))
                    .map(Ok),
                "S" => (config.show_milliseconds && duration.millis > 0)
                    .then(|| duration.millis.to_string())
                    .map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `cmd_duration`: \n{}", error);
            return None;
        }
    });

    Some(undistract_me(module, &config, elapsed))
}

#[cfg(not(feature = "notify-rust"))]
fn undistract_me<'a, 'b>(
    module: Module<'a>,
    config: &'b CmdDurationConfig,
    _elapsed: u128,
) -> Module<'a> {
    if config.show_notifications {
        log::debug!("This version of starship was built without notification support.");
    }

    module
}

#[cfg(feature = "notify-rust")]
fn undistract_me<'a, 'b>(
    module: Module<'a>,
    config: &'b CmdDurationConfig,
    elapsed: u128,
) -> Module<'a> {
    use ansi_term::{unstyle, ANSIStrings};
    use notify_rust::{Notification, Timeout};

    if config.show_notifications && config.min_time_to_notify as u128 <= elapsed {
        let body = format!(
            "Command execution {}",
            unstyle(&ANSIStrings(&module.ansi_strings()))
        );

        let mut notification = Notification::new();
        notification
            .summary("Command finished")
            .body(&body)
            .icon("utilities-terminal")
            .timeout(Timeout::Milliseconds(750));

        if let Err(err) = notification.show() {
            log::trace!("Cannot show notification: {}", err);
        }
    }

    module
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;

    #[test]
    fn config_blank_duration_500ms() {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(500)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_show_ms_duration_500ms() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 500
                show_milliseconds = true
            })
            .cmd_duration(500)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("500ms ")));
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

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("5s ")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_duration_90s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(90_000)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("1m30s ")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_duration_7230s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(7_230_000)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("2h30s ")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_duration_10110s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(10_110_000)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("2h48m30s ")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_blank_duration_1d() {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(86_400_000)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("1d ")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_min_5s_duration_3s() {
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
    fn config_min_5s_duration_10s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 5000
            })
            .cmd_duration(10000)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("10s ")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_custom_prefix_duration_1s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "underwent [(${d}d)(${h}h)(${m}m)(${s}s)(${S}ms) ]($style)"
            })
            .cmd_duration(1000)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_custom_prefix_duration_5s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "underwent [(${d}d)(${h}h)(${m}m)(${s}s)(${S}ms) ]($style)"
            })
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("underwent {}", Color::Yellow.bold().paint("5s ")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_custom_format_duration_7470s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "took [(${d}d )(${hh}h )(${mm}m )(${ss}s )(${SSS}ms )]($style)"
            })
            .cmd_duration(7_470_000)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("02h 04m 30s ")));
        assert_eq!(expected, actual);
    }
}
