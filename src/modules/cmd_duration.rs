use ansi_term::Color;

use super::{Context, Module};

/// Outputs the time it took the last command to execute
///
/// Will only print if last command took more than a certain amount of time to
/// execute. Default is two seconds, but can be set by config option `min_time`.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("cmd_duration");

    let arguments = &context.arguments;
    let elapsed = arguments
        .value_of("cmd_duration")
        .unwrap_or("invalid_time")
        .parse::<u64>()
        .ok()?;

    let signed_config_min = module.config_value_i64("min_time").unwrap_or(2000) * 1_000_000;

    /* TODO: Once error handling is implemented, warn the user if their config
    min time is nonsensical */
    if signed_config_min < 0 {
        log::debug!(
            "[WARN]: min_time in [cmd_duration] ({}) was less than zero",
            signed_config_min
        );
        return None;
    }

    let config_min = signed_config_min as u64;

    let module_color = match elapsed {
        time if time < config_min => return None,
        _ => module
            .config_value_style("style")
            .unwrap_or_else(|| Color::Yellow.bold()),
    };

    module.set_style(module_color);
    module.new_segment("cmd_duration", &format!("took {}", render_time(elapsed)));
    module.get_prefix().set_value("");

    Some(module)
}

// Render the time into a nice human-readable string
fn render_time(raw_nanoseconds: u64) -> String {
    // Calculate a simple breakdown into days/hours/minutes/seconds
    let raw_milliseconds = raw_nanoseconds / 1_000_000;
    let (milliseconds, raw_seconds) = (raw_milliseconds % 1000, raw_milliseconds / 1000);
    let (seconds, raw_minutes) = (raw_seconds % 60, raw_seconds / 60);
    let (minutes, raw_hours) = (raw_minutes % 60, raw_minutes / 60);
    let (hours, days) = (raw_hours % 24, raw_hours / 24);

    let components = [days, hours, minutes, seconds, milliseconds];
    let suffixes = ["d", "h", "m", "s", "ms"];

    let rendered_components: Vec<String> = components
        .iter()
        .zip(&suffixes)
        .map(render_time_component)
        .collect();
    rendered_components.join("")
}

/// Render a single component of the time string, giving an empty string if component is zero
fn render_time_component((component, suffix): (&u64, &&str)) -> String {
    match component {
        0 => String::new(),
        n => format!("{}{}", n, suffix),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_100ms() {
        assert_eq!(render_time(100 * 1_000_000 as u64), "100ms")
    }
    #[test]
    fn test_10s() {
        assert_eq!(render_time(10 * 1_000_000_000 as u64), "10s")
    }
    #[test]
    fn test_90s() {
        assert_eq!(render_time(90 * 1_000_000_000 as u64), "1m30s")
    }
    #[test]
    fn test_10110s() {
        assert_eq!(render_time(10110 * 1_000_000_000 as u64), "2h48m30s")
    }
    #[test]
    fn test_1d() {
        assert_eq!(render_time(86400 * 1_000_000_000 as u64), "1d")
    }
}
