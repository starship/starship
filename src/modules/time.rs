use ansi_term::Color;
use chrono::offset::TimeZone;
use chrono::{DateTime, Local};

use super::{Context, Module};

/// Outputs the current time
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("time")?;

    // Remove when logic for disabled by default exists
    if module.config_value_bool("disabled").unwrap_or(true) {
        return None;
    }

    let module_style = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Yellow.bold());
    module.set_style(module_style);

    // Get module settings set up
    let prefix = module.config_value_str("prefix").unwrap_or("").to_owned();
    let suffix = module.config_value_str("suffix").unwrap_or("").to_owned();
    let is_12hr = module.config_value_bool("12hr").unwrap_or(false);
    let default_format = if is_12hr { "%r" } else { "%T" };
    let time_format = module
        .config_value_str("format")
        .unwrap_or(default_format)
        .to_owned();

    let local: DateTime<Local> = Local::now();
    let formatted_time_string = format_time(&time_format, local);
    module.new_segment(
        "time",
        &format!("{}{}{}", prefix, formatted_time_string, suffix),
    );
    module.get_prefix().set_value("at ");

    Some(module)
}

/// Format a given time into the given string. This function should be referentially
/// transparent, which makes it easy to test (unlike anything involving the actual time)
fn format_time(time_format: &str, localtime: DateTime<Local>) -> String {
    // Get module settings
    localtime.format(time_format).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FMT_12: &str = "%r";
    const FMT_24: &str = "%T";

    #[test]
    fn test_midnight_12hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(0, 0, 0);
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "12:00:00 AM");
    }

    #[test]
    fn test_midnight_24hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(0, 0, 0);
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "00:00:00");
    }

    #[test]
    fn test_noon_12hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(12, 0, 0);
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "12:00:00 PM");
    }

    #[test]
    fn test_noon_24hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(12, 0, 0);
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "12:00:00");
    }

    #[test]
    fn test_arbtime_12hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "03:36:47 PM");
    }

    #[test]
    fn test_arbtime_24hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "15:36:47");
    }

}
