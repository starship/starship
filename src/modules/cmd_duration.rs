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

    let props = &context.properties;
    let elapsed = props
        .get("cmd_duration")
        .unwrap_or(&"invalid_time".into())
        .parse::<u128>()
        .ok()?;

    /* TODO: Once error handling is implemented, warn the user if their config
    min time is nonsensical */
    if config.min_time < 0 {
        log::debug!(
            "[WARN]: min_time in [cmd_duration] ({}) was less than zero",
            config.min_time
        );
        return None;
    }

    if elapsed < config.min_time as u128 {
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

    Some(module)
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
