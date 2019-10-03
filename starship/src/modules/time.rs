use ansi_term::Color;
use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

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
    let utc_time_offset_str = module
        .config_value_str("utc_time_offset")
        .unwrap_or("local");

    let default_format = if is_12hr { "%r" } else { "%T" };
    let time_format = module
        .config_value_str("format")
        .unwrap_or(default_format)
        .to_owned();

    log::trace!(
        "Timer module is enabled with format string: {}",
        time_format
    );

    let local_time: DateTime<Local> = Local::now();
    let utc_time: DateTime<Utc> = Utc::now();
    log::trace!("Local time now is {}", local_time);
    log::trace!("UTC time now is {}", utc_time);

    module.new_segment(
        "time",
        &create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, &time_format),
    );
    module.get_prefix().set_value("at ");

    Some(module)
}

fn create_formatted_time_string(
    local_time: DateTime<Local>,
    utc_time: DateTime<Utc>,
    utc_time_offset_str: &str,
    time_format: &str,
) -> String {
    if utc_time_offset_str == "local" {
        format_time(&time_format, local_time)
    } else {
        // Using floats to allow 30/45 minute offsets: https://www.timeanddate.com/time/time-zones-interesting.html
        let utc_time_offset_in_hours = match utc_time_offset_str.parse::<f32>() {
            Ok(parsed_value) => parsed_value,
            // Passing out of range value to force falling back to "local"
            Err(_) => 25_f32,
        };
        if utc_time_offset_in_hours < 24_f32 && utc_time_offset_in_hours > -24_f32 {
            let utc_offset_in_seconds: i32 = (utc_time_offset_in_hours * 3600_f32) as i32;
            let timezone_offset = FixedOffset::east(utc_offset_in_seconds);
            log::trace!("Target timezone offset is {}", timezone_offset);

            let target_time = utc_time.with_timezone(&timezone_offset);
            log::trace!("Time in target timezone now is {}", target_time);

            format_time_fixed_offset(&time_format, target_time)
        } else {
            log::warn!(
                "Invalid utc_time_offset configuration provided! Falling back to \"local\"."
            );
            format_time(&time_format, local_time)
        }
    }
}

/// Format a given time into the given string. This function should be referentially
/// transparent, which makes it easy to test (unlike anything involving the actual time)
fn format_time(time_format: &str, local_time: DateTime<Local>) -> String {
    local_time.format(time_format).to_string()
}

fn format_time_fixed_offset(time_format: &str, utc_time: DateTime<FixedOffset>) -> String {
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

    #[test]
    fn test_midnight_12hr_fixed_offset() {
        let timezone_offset = FixedOffset::east(0);
        let time = Utc
            .ymd(2014, 7, 8)
            .and_hms(0, 0, 0)
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_12, time);
        assert_eq!(formatted, "12:00:00 AM");
    }

    #[test]
    fn test_midnight_24hr_fixed_offset() {
        let timezone_offset = FixedOffset::east(0);
        let time = Utc
            .ymd(2014, 7, 8)
            .and_hms(0, 0, 0)
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_24, time);
        assert_eq!(formatted, "00:00:00");
    }

    #[test]
    fn test_noon_12hr_fixed_offset() {
        let timezone_offset = FixedOffset::east(0);
        let time = Utc
            .ymd(2014, 7, 8)
            .and_hms(12, 0, 0)
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_12, time);
        assert_eq!(formatted, "12:00:00 PM");
    }

    #[test]
    fn test_noon_24hr_fixed_offset() {
        let timezone_offset = FixedOffset::east(0);
        let time = Utc
            .ymd(2014, 7, 8)
            .and_hms(12, 0, 0)
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_24, time);
        assert_eq!(formatted, "12:00:00");
    }

    #[test]
    fn test_arbtime_12hr_fixed_offset() {
        let timezone_offset = FixedOffset::east(0);
        let time = Utc
            .ymd(2014, 7, 8)
            .and_hms(15, 36, 47)
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_12, time);
        assert_eq!(formatted, "03:36:47 PM");
    }

    #[test]
    fn test_arbtime_24hr_fixed_offset() {
        let timezone_offset = FixedOffset::east(0);
        let time = Utc
            .ymd(2014, 7, 8)
            .and_hms(15, 36, 47)
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_24, time);
        assert_eq!(formatted, "15:36:47");
    }

    #[test]
    fn test_format_with_paren_fixed_offset() {
        let timezone_offset = FixedOffset::east(0);
        let time = Utc
            .ymd(2014, 7, 8)
            .and_hms(15, 36, 47)
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset("[%T]", time);
        assert_eq!(formatted, "[15:36:47]");
    }

    #[test]
    fn test_create_formatted_time_string_with_local() {
        let local_time: DateTime<Local> = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time: DateTime<Utc> = Utc.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time_offset_str = "local";

        let actual =
            create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, FMT_12);
        assert_eq!(actual, "03:36:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_minus_3() {
        let local_time: DateTime<Local> = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time: DateTime<Utc> = Utc.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time_offset_str = "-3";

        let actual =
            create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, FMT_12);
        assert_eq!(actual, "12:36:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_plus_5() {
        let local_time: DateTime<Local> = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time: DateTime<Utc> = Utc.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time_offset_str = "+5";

        let actual =
            create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, FMT_12);
        assert_eq!(actual, "08:36:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_plus_9_30() {
        let local_time: DateTime<Local> = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time: DateTime<Utc> = Utc.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time_offset_str = "+9.5";

        let actual =
            create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, FMT_12);
        assert_eq!(actual, "01:06:47 AM");
    }

    #[test]
    fn test_create_formatted_time_string_with_plus_5_45() {
        let local_time: DateTime<Local> = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time: DateTime<Utc> = Utc.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time_offset_str = "+5.75";

        let actual =
            create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, FMT_12);
        assert_eq!(actual, "09:21:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_plus_24() {
        let local_time: DateTime<Local> = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time: DateTime<Utc> = Utc.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time_offset_str = "+24";

        let actual =
            create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, FMT_12);
        assert_eq!(actual, "03:36:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_minus_24() {
        let local_time: DateTime<Local> = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time: DateTime<Utc> = Utc.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time_offset_str = "-24";

        let actual =
            create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, FMT_12);
        assert_eq!(actual, "03:36:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_plus_9001() {
        let local_time: DateTime<Local> = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time: DateTime<Utc> = Utc.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time_offset_str = "+9001";

        let actual =
            create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, FMT_12);
        assert_eq!(actual, "03:36:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_minus_4242() {
        let local_time: DateTime<Local> = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time: DateTime<Utc> = Utc.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time_offset_str = "-4242";

        let actual =
            create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, FMT_12);
        assert_eq!(actual, "03:36:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_invalid_string() {
        let local_time: DateTime<Local> = Local.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time: DateTime<Utc> = Utc.ymd(2014, 7, 8).and_hms(15, 36, 47);
        let utc_time_offset_str = "completely wrong config";

        let actual =
            create_formatted_time_string(local_time, utc_time, &utc_time_offset_str, FMT_12);
        assert_eq!(actual, "03:36:47 PM");
    }
}
