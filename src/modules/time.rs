use ansi_term::Color;
use chrono::{DateTime, Local};

use super::{Context, Module};

/// Outputs the current time
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("time");

    if module.config_value_bool("disabled").unwrap_or(true) {
        return None;
    }

    let module_style = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Yellow.bold());
    module.set_style(module_style);

    // Load module settings
    let is_12hr = module.config_value_bool("12hr").unwrap_or(false);

    let default_format = if is_12hr { "%r" } else { "%T" };
    let time_format = module
        .config_value_str("format")
        .unwrap_or(default_format)
        .to_owned();

    log::trace!(
        "Timer module is enabled with format string: {}",
        time_format
    );

    let local: DateTime<Local> = Local::now();
    let formatted_time_string = format_time(&time_format, local);
    module.new_segment("time", &formatted_time_string);
    module.get_prefix().set_value("at ");

    Some(module)
}

/// Format a given time into the given string. This function should be referentially
/// transparent, which makes it easy to test (unlike anything involving the actual time)
fn format_time(time_format: &str, local_time: DateTime<Local>) -> String {
    local_time.format(time_format).to_string()
}

/* Because we cannot make acceptance tests for the time module, these unit
tests become extra important */
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;

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

    #[test]
    fn test_format_with_paren() {
        let time = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let formatted = format_time("[%T]", time);
        assert_eq!(formatted, "[15:36:47]");
    }
}
