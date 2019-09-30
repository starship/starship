use ansi_term::Color;
use chrono::{DateTime, Local, FixedOffset, Utc, Duration};

use super::{Context, Module};

/// Outputs the current time
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("time");

    // Remove when logic for disabled by default exists
    if module.config_value_bool("disabled").unwrap_or(true) {
        return None;
    }

    let module_style = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Yellow.bold());
    module.set_style(module_style);

    // Load module settings
    let is_12hr = module.config_value_bool("12hr").unwrap_or(false);
    let utc_time_offset_str = module.config_value_str("utc_time_offset").unwrap_or("local");

    log::trace!("utc_time_offset_str: {}", utc_time_offset_str);

    let default_format = if is_12hr { "%r" } else { "%T" };
    let time_format = module
        .config_value_str("format")
        .unwrap_or(default_format)
        .to_owned();

    log::trace!(
        "Timer module is enabled with format string: {}",
        time_format
    );

    let formatted_time_string: String;
    if utc_time_offset_str == "local" {
        log::trace!("Using local time");
        let local: DateTime<Local> = Local::now();
        formatted_time_string = format_time_local(&time_format, local);
    } else {
        let utc_time_offset_in_hours = utc_time_offset_str.parse::<i32>().unwrap_or(0);
        let utc_time_offset_in_seconds = utc_time_offset_in_hours * 60 * 60;
        // TODO: Add check for out of range offsets

        let target_offset = FixedOffset::east(utc_time_offset_in_seconds);
        let seconds_between_target_and_utc = target_offset.local_minus_utc();
        let duration_between_target_and_utc = Duration::seconds(seconds_between_target_and_utc as i64);
        let utc_plus_duration = Utc::now().checked_add_signed(duration_between_target_and_utc).unwrap();

        formatted_time_string = format_time_utc(&time_format, utc_plus_duration);
    }

    module.new_segment("time", &formatted_time_string);
    module.get_prefix().set_value("at ");

    Some(module)
}


/// Format a given time into the given string. This function should be referentially
/// transparent, which makes it easy to test (unlike anything involving the actual time)
fn format_time_local(time_format: &str, local_time: DateTime<Local>) -> String {
    local_time.format(time_format).to_string()
}

fn format_time_utc(time_format: &str, utc_time: DateTime<Utc>) -> String {
    utc_time.format(time_format).to_string()
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
        let formatted = format_time_local(FMT_12, time);
        assert_eq!(formatted, "12:00:00 AM");
    }

    #[test]
    fn test_midnight_24hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(0, 0, 0);
        let formatted = format_time_local(FMT_24, time);
        assert_eq!(formatted, "00:00:00");
    }

    #[test]
    fn test_noon_12hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(12, 0, 0);
        let formatted = format_time_local(FMT_12, time);
        assert_eq!(formatted, "12:00:00 PM");
    }

    #[test]
    fn test_noon_24hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(12, 0, 0);
        let formatted = format_time_local(FMT_24, time);
        assert_eq!(formatted, "12:00:00");
    }

    #[test]
    fn test_arbtime_12hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let formatted = format_time_local(FMT_12, time);
        assert_eq!(formatted, "03:36:47 PM");
    }

    #[test]
    fn test_arbtime_24hr() {
        let time = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let formatted = format_time_local(FMT_24, time);
        assert_eq!(formatted, "15:36:47");
    }

    #[test]
    fn test_format_with_paren() {
        let time = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let formatted = format_time_local("[%T]", time);
        assert_eq!(formatted, "[15:36:47]");
    }
}
