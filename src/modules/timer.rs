use ansi_term::Color;

use super::{Context, Module};

/// Outputs the time it took the last command to execute
///
/// Will only print if last command took more than two seconds to execute.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("timer")?;
    let arguments = &context.arguments;
    let elapsed = arguments
        .value_of("elapsed_time")
        .unwrap_or("x") // not a time
        .parse::<u64>()
        .ok()?;

    let module_color = match elapsed {
        0...2 => return None, // Too short! Don't display anything.
        _ => Color::Yellow.bold(),
    };

    module.set_style(module_color);
    module.new_segment("timer", &format!("took {}", render_time(elapsed)));
    module.get_prefix().set_value("");

    Some(module)
}

/// Render the time into a nice human-readable string
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
    fn test_90s() {
        assert_eq!(render_time(90 as u64), "1m30s")
    }
    fn test_10110s() {
        assert_eq!(render_time(10110 as u64), "1h48m30s")
    }
}
