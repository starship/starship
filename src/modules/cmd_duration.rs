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

impl Duration {
    fn get_days(&self) -> Option<&u128> {
        match self.days {
            0 => None,
            _ => Some(&self.days),
        }
    }

    fn get_hours(&self) -> Option<&u8> {
        match self.hours {
            0 => self.get_days().and(Some(&self.hours)),
            _ => Some(&self.hours),
        }
    }

    fn get_minutes(&self) -> Option<&u8> {
        match self.minutes {
            0 => self.get_hours().and(Some(&self.minutes)),
            _ => Some(&self.minutes),
        }
    }

    fn get_seconds(&self) -> Option<&u8> {
        match self.seconds {
            0 => self.get_minutes().and(Some(&self.seconds)),
            _ => Some(&self.seconds),
        }
    }

    fn get_milliseconds(&self) -> Option<&u16> {
        match self.millis {
            0 => self.get_seconds().and(Some(&self.millis)),
            _ => Some(&self.millis),
        }
    }
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
                "d" => duration.get_days().map(|days| format!("{}", days)).map(Ok),
                "hh" => duration
                    .get_hours()
                    .and_then(|hours| match hours {
                        0 => config.show_zero_units.then(|| 0),
                        _ => Some(*hours),
                    })
                    .map(|hours| format!("{:02}", hours))
                    .map(Ok),
                "h" => duration
                    .get_hours()
                    .and_then(|hours| match hours {
                        0 => config.show_zero_units.then(|| 0),
                        _ => Some(*hours),
                    })
                    .map(|hours| format!("{}", hours))
                    .map(Ok),
                "mm" => duration
                    .get_minutes()
                    .and_then(|minutes| match minutes {
                        0 => config.show_zero_units.then(|| 0),
                        _ => Some(*minutes),
                    })
                    .map(|minutes| format!("{:02}", minutes))
                    .map(Ok),
                "m" => duration
                    .get_minutes()
                    .and_then(|minutes| match minutes {
                        0 => config.show_zero_units.then(|| 0),
                        _ => Some(*minutes),
                    })
                    .map(|minutes| format!("{}", minutes))
                    .map(Ok),
                "ss" => duration
                    .get_seconds()
                    .and_then(|seconds| match seconds {
                        0 => config.show_zero_units.then(|| 0),
                        _ => Some(*seconds),
                    })
                    .map(|seconds| format!("{:02}", seconds))
                    .map(Ok),
                "s" => duration
                    .get_seconds()
                    .and_then(|seconds| match seconds {
                        0 => config.show_zero_units.then(|| 0),
                        _ => Some(*seconds),
                    })
                    .map(|seconds| format!("{}", seconds))
                    .map(Ok),
                "SSS" => match config.show_milliseconds {
                    true => duration
                        .get_milliseconds()
                        .and_then(|millis| match millis {
                            0 => config.show_zero_units.then(|| 0),
                            _ => Some(*millis),
                        })
                        .map(|millis| format!("{:003}", millis))
                        .map(Ok),
                    _ => None,
                },
                "S" => match config.show_milliseconds {
                    true => duration
                        .get_milliseconds()
                        .and_then(|millis| match millis {
                            0 => config.show_zero_units.then(|| 0),
                            _ => Some(*millis),
                        })
                        .map(|millis| format!("{}", millis))
                        .map(Ok),
                    _ => None,
                },
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
    fn config_show_ms_duration_50ms() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 10
                show_milliseconds = true
            })
            .cmd_duration(50)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("50ms ")));
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
    fn config_zero_units_duration_7230s() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                show_zero_units = true
            })
            .cmd_duration(7_230_000)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("2h0m30s ")));
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
    fn config_custom_format_duration_5ms() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 0
                show_milliseconds = true
                format = "took [(${d}d )(${hh}h )(${mm}m )(${ss}s )(${SSS}ms )]($style)"
            })
            .cmd_duration(5)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("005ms ")));
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_custom_format_duration_30ms() {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 0
                show_milliseconds = true
                format = "took [(${d}d )(${hh}h )(${mm}m )(${ss}s )(${SSS}ms )]($style)"
            })
            .cmd_duration(30)
            .collect();

        let expected = Some(format!("took {}", Color::Yellow.bold().paint("030ms ")));
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

        let expected = Some(format!(
            "took {}",
            Color::Yellow.bold().paint("02h 04m 30s ")
        ));
        assert_eq!(expected, actual);
    }
}
