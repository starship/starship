use super::utils::query_parser::parse_query;
use super::{Context, Module, SegmentConfig};
use spongy::{parse_with, Item, Wrapper};

use crate::config::RootModuleConfig;
use crate::configs::cmd_duration::CmdDurationConfig;

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
        .parse::<u64>()
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

    let config_min = config.min_time as u64;

    let module_color = match elapsed {
        time if time < config_min => return None,
        _ => config.style,
    };

    let cmd_duration_stacked = parse_with(config.format, |item: &Item| -> Option<String> {
        match item.wrapper {
            Wrapper::DollarCurly => {
                let (segment, _query) = parse_query(&item.text);
                match segment {
                    "duration" => Some(render_time(elapsed)),
                    _ => None,
                }
            }
            _ => None,
        }
    })
    .ok()?;

    module.set_style(module_color);
    module.create_segment("cmd_duration", &SegmentConfig::new(&cmd_duration_stacked));
    module.get_prefix().set_value("");

    Some(module)
}

// Render the time into a nice human-readable string
fn render_time(raw_seconds: u64) -> String {
    // Calculate a simple breakdown into days/hours/minutes/seconds
    let (seconds, raw_minutes) = (raw_seconds % 60, raw_seconds / 60);
    let (minutes, raw_hours) = (raw_minutes % 60, raw_minutes / 60);
    let (hours, days) = (raw_hours % 24, raw_hours / 24);

    let components = [days, hours, minutes, seconds];
    let suffixes = ["d", "h", "m", "s"];

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
    fn test_10s() {
        assert_eq!(render_time(10 as u64), "10s")
    }
    #[test]
    fn test_90s() {
        assert_eq!(render_time(90 as u64), "1m30s")
    }
    #[test]
    fn test_10110s() {
        assert_eq!(render_time(10110 as u64), "2h48m30s")
    }
    #[test]
    fn test_1d() {
        assert_eq!(render_time(86400 as u64), "1d")
    }
}
