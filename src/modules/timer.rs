use ansi_term::Color;
use std::path::Path;

use super::{Context, Module};

/// Outputs the time it took the last command to execute
///
/// Will only print if last command took more than two seconds to execute.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let arguments = &context.arguments;
    let elapsed = arguments
        .value_of("elapsed_time")
        .unwrap_or("0")
        .parse::<u64>()
        .ok()?;

    // These breakpoints can be adjusted in config later
    let module_color = Color::Yellow.bold();
    
    /*
    match elapsed {
        0...2 => return None, // We're not printing anything for now
        2...10 => Color::Green.bold(),
        11...60 => Color::Yellow.bold(),
        _ => Color::Red.bold(),
    };*/

    let mut module = context.new_module("timer")?;
    module.set_style(module_color);
    module.new_segment(
        "timer",
        &format!("took {}", render_time(&elapsed)),
    );
    module.get_prefix().set_value("");

    Some(module)
}

/// Render the time into a nice human-readable string
fn render_time(raw_seconds: &u64) -> String {
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
    let time = match component {
        0 => String::new(),
        n => format!("{}{}", n, suffix),
    };
    time
}
