use super::{Context, Module, RootModuleConfig};

use crate::configs::cmd_duration::CmdDurationConfig;
use crate::formatter::StringFormatter;

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

// Render the time into a nice human-readable string
fn render_time(raw_millis: u128, show_millis: bool) -> String {
    // Calculate a simple breakdown into days/hours/minutes/seconds/milliseconds
    let (millis, raw_seconds) = (raw_millis % 1000, raw_millis / 1000);
    let (seconds, raw_minutes) = (raw_seconds % 60, raw_seconds / 60);
    let (minutes, raw_hours) = (raw_minutes % 60, raw_minutes / 60);
    let (hours, days) = (raw_hours % 24, raw_hours / 24);

    let components = [days, hours, minutes, seconds];
    let suffixes = ["d", "h", "m", "s"];

    let mut rendered_components: Vec<String> = components
        .iter()
        .zip(&suffixes)
        .map(render_time_component)
        .collect();
    if show_millis || raw_millis < 1000 {
        rendered_components.push(render_time_component((&millis, &"ms")));
    }
    rendered_components.join("")
}

/// Render a single component of the time string, giving an empty string if component is zero
fn render_time_component((component, suffix): (&u128, &&str)) -> String {
    match component {
        0 => String::new(),
        n => format!("{}{}", n, suffix),
    }
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
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::io;

    #[test]
    fn test_500ms() {
        assert_eq!(render_time(500 as u128, true), "500ms")
    }
    #[test]
    fn test_10s() {
        assert_eq!(render_time(10_000 as u128, true), "10s")
    }
    #[test]
    fn test_90s() {
        assert_eq!(render_time(90_000 as u128, true), "1m30s")
    }
    #[test]
    fn test_10110s() {
        assert_eq!(render_time(10_110_000 as u128, true), "2h48m30s")
    }
    #[test]
    fn test_1d() {
        assert_eq!(render_time(86_400_000 as u128, true), "1d")
    }

    #[test]
    fn config_blank_duration_1s() -> io::Result<()> {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(1000)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn config_blank_duration_5s() -> io::Result<()> {
        let actual = ModuleRenderer::new("cmd_duration")
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("took {} ", Color::Yellow.bold().paint("5s")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn config_5s_duration_3s() -> io::Result<()> {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 5000
            })
            .cmd_duration(3000)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn config_5s_duration_10s() -> io::Result<()> {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                min_time = 5000
            })
            .cmd_duration(10000)
            .collect();

        let expected = Some(format!("took {} ", Color::Yellow.bold().paint("10s")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn config_1s_duration_prefix_underwent() -> io::Result<()> {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "underwent [$duration]($style) "
            })
            .cmd_duration(1000)
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn config_5s_duration_prefix_underwent() -> io::Result<()> {
        let actual = ModuleRenderer::new("cmd_duration")
            .config(toml::toml! {
                [cmd_duration]
                format = "underwent [$duration]($style) "
            })
            .cmd_duration(5000)
            .collect();

        let expected = Some(format!("underwent {} ", Color::Yellow.bold().paint("5s")));
        assert_eq!(expected, actual);
        Ok(())
    }
}
