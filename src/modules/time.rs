use chrono::{DateTime, FixedOffset, Local, NaiveTime, Utc};

use super::{Context, Module, ModuleConfig};
use crate::configs::time::TimeConfig;
use crate::formatter::StringFormatter;

/// Outputs the current time
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("time");
    let config: TimeConfig = TimeConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    };

    // Hide prompt if current time is not inside time_range
    let (display_start, display_end) = parse_time_range(config.time_range);
    let time_now = Local::now().time();
    if !is_inside_time_range(time_now, display_start, display_end) {
        return None;
    }

    let default_format = if config.use_12hr { "%r" } else { "%T" };
    let time_format = config.time_format.unwrap_or(default_format);

    log::trace!(
        "Timer module is enabled with format string: {}",
        time_format
    );

    let formatted_time_string = if config.utc_time_offset != "local" {
        match create_offset_time_string(Utc::now(), config.utc_time_offset, time_format) {
            Ok(formatted_string) => formatted_string,
            Err(_) => {
                log::warn!(
                    "Invalid utc_time_offset configuration provided! Falling back to \"local\"."
                );
                format_time(time_format, Local::now())
            }
        }
    } else {
        format_time(time_format, Local::now())
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "time" => Some(Ok(&formatted_time_string)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `time`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

fn create_offset_time_string(
    utc_time: DateTime<Utc>,
    utc_time_offset_str: &str,
    time_format: &str,
) -> Result<String, &'static str> {
    // Using floats to allow 30/45 minute offsets: https://www.timeanddate.com/time/time-zones-interesting.html
    let utc_time_offset_in_hours = utc_time_offset_str.parse::<f32>().unwrap_or(
        // Passing out of range value to force falling back to "local"
        25_f32,
    );
    if utc_time_offset_in_hours < 24_f32 && utc_time_offset_in_hours > -24_f32 {
        let utc_offset_in_seconds: i32 = (utc_time_offset_in_hours * 3600_f32) as i32;
        let Some(timezone_offset) = FixedOffset::east_opt(utc_offset_in_seconds) else {
            return Err("Invalid offset");
        };
        log::trace!("Target timezone offset is {}", timezone_offset);

        let target_time = utc_time.with_timezone(&timezone_offset);
        log::trace!("Time in target timezone now is {}", target_time);

        Ok(format_time_fixed_offset(time_format, target_time))
    } else {
        Err("Invalid timezone offset.")
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

/// Returns true if `time_now` is between `time_start` and `time_end`.
/// If one of these values is not given, then it is ignored.
/// It also handles cases where `time_start` and `time_end` have a midnight in between
fn is_inside_time_range(
    time_now: NaiveTime,
    time_start: Option<NaiveTime>,
    time_end: Option<NaiveTime>,
) -> bool {
    match (time_start, time_end) {
        (None, None) => true,
        (Some(i), None) => time_now > i,
        (None, Some(i)) => time_now < i,
        (Some(i), Some(j)) => {
            if i < j {
                i < time_now && time_now < j
            } else {
                time_now > i || time_now < j
            }
        }
    }
}

/// Parses the config's `time_range` field and returns the starting time and ending time.
/// The range is in the format START_TIME-END_TIME, with `START_TIME` and `END_TIME` being optional.
///
/// If one of the ranges is invalid or not provided, then the corresponding field in the output
/// tuple is None
fn parse_time_range(time_range: &str) -> (Option<NaiveTime>, Option<NaiveTime>) {
    let value = String::from(time_range);

    // Check if there is exactly one hyphen, and fail otherwise
    if value.matches('-').count() != 1 {
        return (None, None);
    }

    // Split time_range into the two ranges
    let (start, end) = value.split_at(value.find('-').unwrap());
    let end = &end[1..];

    // Parse the ranges
    let start_time = NaiveTime::parse_from_str(start, "%H:%M:%S").ok();
    let end_time = NaiveTime::parse_from_str(end, "%H:%M:%S").ok();

    (start_time, end_time)
}

/* Because we cannot make acceptance tests for the time module, these unit
tests become extra important */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use chrono::offset::TimeZone;

    const FMT_12: &str = "%r";
    const FMT_24: &str = "%T";

    #[test]
    fn test_midnight_12hr() {
        let time = Local.with_ymd_and_hms(2014, 7, 8, 0, 0, 0).unwrap();
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "12:00:00 AM");
    }

    #[test]
    fn test_midnight_24hr() {
        let time = Local.with_ymd_and_hms(2014, 7, 8, 0, 0, 0).unwrap();
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "00:00:00");
    }

    #[test]
    fn test_noon_12hr() {
        let time = Local.with_ymd_and_hms(2014, 7, 8, 12, 0, 0).unwrap();
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "12:00:00 PM");
    }

    #[test]
    fn test_noon_24hr() {
        let time = Local.with_ymd_and_hms(2014, 7, 8, 12, 0, 0).unwrap();
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "12:00:00");
    }

    #[test]
    fn test_arbtime_12hr() {
        let time = Local.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "03:36:47 PM");
    }

    #[test]
    fn test_arbtime_24hr() {
        let time = Local.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "15:36:47");
    }

    #[test]
    fn test_format_with_paren() {
        let time = Local.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let formatted = format_time("[%T]", time);
        assert_eq!(formatted, "[15:36:47]");
    }

    #[test]
    fn test_midnight_12hr_fixed_offset() {
        let timezone_offset = FixedOffset::east_opt(0).unwrap();
        let time = Utc
            .with_ymd_and_hms(2014, 7, 8, 0, 0, 0)
            .unwrap()
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_12, time);
        assert_eq!(formatted, "12:00:00 AM");
    }

    #[test]
    fn test_midnight_24hr_fixed_offset() {
        let timezone_offset = FixedOffset::east_opt(0).unwrap();
        let time = Utc
            .with_ymd_and_hms(2014, 7, 8, 0, 0, 0)
            .unwrap()
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_24, time);
        assert_eq!(formatted, "00:00:00");
    }

    #[test]
    fn test_noon_12hr_fixed_offset() {
        let timezone_offset = FixedOffset::east_opt(0).unwrap();
        let time = Utc
            .with_ymd_and_hms(2014, 7, 8, 12, 0, 0)
            .unwrap()
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_12, time);
        assert_eq!(formatted, "12:00:00 PM");
    }

    #[test]
    fn test_noon_24hr_fixed_offset() {
        let timezone_offset = FixedOffset::east_opt(0).unwrap();
        let time = Utc
            .with_ymd_and_hms(2014, 7, 8, 12, 0, 0)
            .unwrap()
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_24, time);
        assert_eq!(formatted, "12:00:00");
    }

    #[test]
    fn test_arbtime_12hr_fixed_offset() {
        let timezone_offset = FixedOffset::east_opt(0).unwrap();
        let time = Utc
            .with_ymd_and_hms(2014, 7, 8, 15, 36, 47)
            .unwrap()
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_12, time);
        assert_eq!(formatted, "03:36:47 PM");
    }

    #[test]
    fn test_arbtime_24hr_fixed_offset() {
        let timezone_offset = FixedOffset::east_opt(0).unwrap();
        let time = Utc
            .with_ymd_and_hms(2014, 7, 8, 15, 36, 47)
            .unwrap()
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset(FMT_24, time);
        assert_eq!(formatted, "15:36:47");
    }

    #[test]
    fn test_format_with_paren_fixed_offset() {
        let timezone_offset = FixedOffset::east_opt(0).unwrap();
        let time = Utc
            .with_ymd_and_hms(2014, 7, 8, 15, 36, 47)
            .unwrap()
            .with_timezone(&timezone_offset);
        let formatted = format_time_fixed_offset("[%T]", time);
        assert_eq!(formatted, "[15:36:47]");
    }

    #[test]
    fn test_create_formatted_time_string_with_minus_3() {
        let utc_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let utc_time_offset_str = "-3";

        let actual = create_offset_time_string(utc_time, utc_time_offset_str, FMT_12).unwrap();
        assert_eq!(actual, "12:36:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_plus_5() {
        let utc_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let utc_time_offset_str = "+5";

        let actual = create_offset_time_string(utc_time, utc_time_offset_str, FMT_12).unwrap();
        assert_eq!(actual, "08:36:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_plus_9_30() {
        let utc_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let utc_time_offset_str = "+9.5";

        let actual = create_offset_time_string(utc_time, utc_time_offset_str, FMT_12).unwrap();
        assert_eq!(actual, "01:06:47 AM");
    }

    #[test]
    fn test_create_formatted_time_string_with_plus_5_45() {
        let utc_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let utc_time_offset_str = "+5.75";

        let actual = create_offset_time_string(utc_time, utc_time_offset_str, FMT_12).unwrap();
        assert_eq!(actual, "09:21:47 PM");
    }

    #[test]
    fn test_create_formatted_time_string_with_plus_24() {
        let utc_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let utc_time_offset_str = "+24";

        create_offset_time_string(utc_time, utc_time_offset_str, FMT_12)
            .expect_err("Invalid timezone offset.");
    }

    #[test]
    fn test_create_formatted_time_string_with_minus_24() {
        let utc_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let utc_time_offset_str = "-24";

        create_offset_time_string(utc_time, utc_time_offset_str, FMT_12)
            .expect_err("Invalid timezone offset.");
    }

    #[test]
    fn test_create_formatted_time_string_with_plus_9001() {
        let utc_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let utc_time_offset_str = "+9001";

        create_offset_time_string(utc_time, utc_time_offset_str, FMT_12)
            .expect_err("Invalid timezone offset.");
    }

    #[test]
    fn test_create_formatted_time_string_with_minus_4242() {
        let utc_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let utc_time_offset_str = "-4242";

        create_offset_time_string(utc_time, utc_time_offset_str, FMT_12)
            .expect_err("Invalid timezone offset.");
    }

    #[test]
    fn test_create_formatted_time_string_with_invalid_string() {
        let utc_time: DateTime<Utc> = Utc.with_ymd_and_hms(2014, 7, 8, 15, 36, 47).unwrap();
        let utc_time_offset_str = "completely wrong config";

        create_offset_time_string(utc_time, utc_time_offset_str, FMT_12)
            .expect_err("Invalid timezone offset.");
    }

    #[test]
    fn test_parse_invalid_time_range() {
        let time_range = "10:00:00-12:00:00-13:00:00";
        let time_range_2 = "10:00:00";

        assert_eq!(parse_time_range(time_range), (None, None));
        assert_eq!(parse_time_range(time_range_2), (None, None));
    }

    #[test]
    fn test_parse_start_time_range() {
        let time_range = "10:00:00-";

        assert_eq!(
            parse_time_range(time_range),
            (Some(NaiveTime::from_hms_opt(10, 00, 00).unwrap()), None)
        );
    }

    #[test]
    fn test_parse_end_time_range() {
        let time_range = "-22:00:00";

        assert_eq!(
            parse_time_range(time_range),
            (None, Some(NaiveTime::from_hms_opt(22, 00, 00).unwrap()))
        );
    }

    #[test]
    fn test_parse_both_time_ranges() {
        let time_range = "10:00:00-16:00:00";

        assert_eq!(
            parse_time_range(time_range),
            (
                Some(NaiveTime::from_hms_opt(10, 00, 00).unwrap()),
                Some(NaiveTime::from_hms_opt(16, 00, 00).unwrap())
            )
        );
    }

    #[test]
    fn test_is_inside_time_range_with_no_range() {
        let time_start = None;
        let time_end = None;
        let time_now = NaiveTime::from_hms_opt(10, 00, 00).unwrap();

        assert!(is_inside_time_range(time_now, time_start, time_end));
    }

    #[test]
    fn test_is_inside_time_range_with_start_range() {
        let time_start = Some(NaiveTime::from_hms_opt(10, 00, 00).unwrap());
        let time_now = NaiveTime::from_hms_opt(12, 00, 00).unwrap();
        let time_now2 = NaiveTime::from_hms_opt(8, 00, 00).unwrap();

        assert!(is_inside_time_range(time_now, time_start, None));
        assert!(!is_inside_time_range(time_now2, time_start, None));
    }

    #[test]
    fn test_is_inside_time_range_with_end_range() {
        let time_end = Some(NaiveTime::from_hms_opt(16, 00, 00).unwrap());
        let time_now = NaiveTime::from_hms_opt(15, 00, 00).unwrap();
        let time_now2 = NaiveTime::from_hms_opt(19, 00, 00).unwrap();

        assert!(is_inside_time_range(time_now, None, time_end));
        assert!(!is_inside_time_range(time_now2, None, time_end));
    }

    #[test]
    fn test_is_inside_time_range_with_complete_range() {
        let time_start = Some(NaiveTime::from_hms_opt(9, 00, 00).unwrap());
        let time_end = Some(NaiveTime::from_hms_opt(18, 00, 00).unwrap());
        let time_now = NaiveTime::from_hms_opt(3, 00, 00).unwrap();
        let time_now2 = NaiveTime::from_hms_opt(13, 00, 00).unwrap();
        let time_now3 = NaiveTime::from_hms_opt(20, 00, 00).unwrap();

        assert!(!is_inside_time_range(time_now, time_start, time_end));
        assert!(is_inside_time_range(time_now2, time_start, time_end));
        assert!(!is_inside_time_range(time_now3, time_start, time_end));
    }

    #[test]
    fn test_is_inside_time_range_with_complete_range_passing_midnight() {
        let time_start = Some(NaiveTime::from_hms_opt(19, 00, 00).unwrap());
        let time_end = Some(NaiveTime::from_hms_opt(12, 00, 00).unwrap());
        let time_now = NaiveTime::from_hms_opt(3, 00, 00).unwrap();
        let time_now2 = NaiveTime::from_hms_opt(13, 00, 00).unwrap();
        let time_now3 = NaiveTime::from_hms_opt(20, 00, 00).unwrap();

        assert!(is_inside_time_range(time_now, time_start, time_end));
        assert!(!is_inside_time_range(time_now2, time_start, time_end));
        assert!(is_inside_time_range(time_now3, time_start, time_end));
    }

    #[test]
    fn config_enabled() {
        let actual = ModuleRenderer::new("time")
            .config(toml::toml! {
                [time]
                disabled = false
            })
            .collect();

        // We can't test what it actually is...but we can assert that it is something
        assert!(actual.is_some());
    }

    #[test]
    fn config_blank() {
        let actual = ModuleRenderer::new("time").collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn config_check_prefix_and_suffix() {
        let actual = ModuleRenderer::new("time")
            .config(toml::toml! {
                [time]
                disabled = false
                format = "at [\\[$time\\]]($style) "
                time_format = "%T"
            })
            .collect()
            .unwrap();

        // This is the prefix with "at ", the color code, then the prefix char [
        let col_prefix = format!("at {}{}[", '\u{1b}', "[1;33m");

        // This is the suffix with suffix char ']', then color codes, then a space
        let col_suffix = format!("]{}{} ", '\u{1b}', "[0m");

        assert!(actual.starts_with(&col_prefix));
        assert!(actual.ends_with(&col_suffix));
    }
}
