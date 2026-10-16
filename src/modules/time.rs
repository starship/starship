use jiff::{
    Timestamp, Zoned,
    civil::{Time, date},
    fmt::strtime,
    tz::{Offset, TimeZone},
};

use std::time::Duration;

use super::{Context, Module, ModuleConfig};
use crate::config::Either;
use crate::configs::time::{TimeConfig, TimezoneWrapper};
use crate::formatter::StringFormatter;

/// How often to refresh the time shown, from a hundredth of a second to a day,
/// according to how often Jiff's formatted output changes.
pub fn period(context: &Context) -> Option<Duration> {
    schedule(context).map(|schedule| schedule.period)
}

/// A time module's cadence and the time zone whose boundaries it follows.
pub(crate) struct Schedule {
    period: Duration,
    time_zone: TimeZone,
}

impl Schedule {
    /// How long until the next local boundary or time zone transition.
    pub(crate) fn delay(&self, now: Timestamp) -> Option<Duration> {
        let period = i128::try_from(self.period.as_nanos()).ok()?;
        let offset = i128::from(self.time_zone.to_offset(now).seconds()) * 1_000_000_000;
        let local = now.as_nanosecond() + offset;
        let left = period - local.checked_rem_euclid(period)?;
        let boundary = Duration::from_nanos(u64::try_from(left).ok()?);
        let transition = self.time_zone.following(now).next().and_then(|transition| {
            Duration::try_from(transition.timestamp().duration_since(now)).ok()
        });
        Some(transition.map_or(boundary, |transition| transition.min(boundary)))
    }
}

pub(crate) fn schedule(context: &Context) -> Option<Schedule> {
    let configuration = TimeConfig::try_load(context.config.get_module_config("time"));
    if configuration.disabled || !configuration.refresh {
        return None;
    }

    let time_zone = display_time_zone(&configuration.utc_time_offset);
    let format = configuration
        .time_format
        .unwrap_or(if configuration.use_12hr { "%r" } else { "%T" });
    let turns_over = format_period(format, &time_zone, Timestamp::now());

    // A valid range can make the module appear or disappear even when its
    // formatted value is constant, so revisit it at least once per second.
    let (display_start, display_end) = parse_time_range(configuration.time_range);
    let range =
        (display_start.is_some() || display_end.is_some()).then_some(Duration::from_secs(1));
    turns_over
        .into_iter()
        .chain(range)
        .min()
        .map(|period| Schedule { period, time_zone })
}

fn format_period(format: &str, time_zone: &TimeZone, now: Timestamp) -> Option<Duration> {
    // The sub-day steps cross a boundary of their own size and no larger,
    // the hour's into the afternoon. Longer probes also detect week, month,
    // quarter, year and century fields, which we revisit at least daily.
    let before = date(2001, 2, 3)
        .at(11, 5, 6, 789_000_000)
        .to_zoned(time_zone.clone())
        .ok()?;
    let shown = |at: &Zoned| strtime::format(format, at).ok();
    let before_shown = shown(&before);
    let day = Duration::from_secs(86_400);
    [
        Duration::from_millis(10),
        Duration::from_millis(100),
        Duration::from_secs(1),
        Duration::from_secs(60),
        Duration::from_secs(3_600),
        day,
        day * 7,
        day * 93,
        day * 366,
        day * 36_525,
    ]
    .into_iter()
    .find(|&step| {
        before
            .checked_add(step)
            .is_ok_and(|after| shown(&after) != before_shown)
    })
    .map(|step| step.min(day))
    .or_else(|| {
        // Rules can have changed since the reference date. Probe the upcoming
        // seasonal transitions too, so zone-only formats remain live under
        // the configured zone's current rules. Literals and IANA names stay
        // constant and need no refresh.
        let now_shown = shown(&now.to_zoned(time_zone.clone()));
        time_zone
            .following(now)
            .take(2)
            .any(|transition| {
                shown(&transition.timestamp().to_zoned(time_zone.clone())) != now_shown
            })
            .then_some(day)
    })
}

/// Outputs the current time
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("time");
    let config: TimeConfig = TimeConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    }

    // Hide prompt if current time is not inside time_range
    let (display_start, display_end) = parse_time_range(config.time_range);
    let time_now = Zoned::now().time();
    if !is_inside_time_range(time_now, display_start, display_end) {
        return None;
    }

    let default_format = if config.use_12hr { "%r" } else { "%T" };
    let time_format = config.time_format.unwrap_or(default_format);

    log::trace!("Timer module is enabled with format string: {time_format}");

    let formatted_time_string = format_time(
        time_format,
        Timestamp::now().to_zoned(display_time_zone(&config.utc_time_offset)),
    );

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
            log::warn!("Error in module `time`: \n{error}");
            return None;
        }
    });

    Some(module)
}

fn display_time_zone(utc_time_offset: &Either<TimezoneWrapper, &str>) -> TimeZone {
    match utc_time_offset {
        Either::First(TimezoneWrapper(time_zone)) => time_zone.clone(),
        Either::Second("local") => TimeZone::system(),
        Either::Second(offset) => parse_utc_time_offset(offset).unwrap_or_else(|_| {
            log::warn!(
                "Invalid utc_time_offset configuration provided! Falling back to \"local\"."
            );
            TimeZone::system()
        }),
    }
}

fn parse_utc_time_offset(utc_time_offset_str: &str) -> Result<TimeZone, String> {
    // Using floats to allow 30/45 minute offsets: https://www.timeanddate.com/time/time-zones-interesting.html
    let utc_time_offset_in_hours = utc_time_offset_str.parse::<f32>().unwrap_or(
        // Passing out of range value to force falling back to "local"
        25_f32,
    );
    if utc_time_offset_in_hours < 24_f32 && utc_time_offset_in_hours > -24_f32 {
        let utc_offset_in_seconds: i32 = (utc_time_offset_in_hours * 3600_f32) as i32;
        let timezone_offset = Offset::from_seconds(utc_offset_in_seconds)
            .map_err(|err| format!("Invalid timezone offset: {err:?}"))?;
        log::trace!("Target timezone offset is {timezone_offset}");
        Ok(TimeZone::fixed(timezone_offset))
    } else {
        Err("Invalid timezone offset.".to_string())
    }
}

/// Format a given time into the given string. This function should be referentially
/// transparent, which makes it easy to test (unlike anything involving the actual time)
fn format_time(time_format: &str, local_time: Zoned) -> String {
    local_time.strftime(time_format).to_string()
}

/// Returns true if `time_now` is between `time_start` and `time_end`.
/// If one of these values is not given, then it is ignored.
/// It also handles cases where `time_start` and `time_end` have a midnight in between
fn is_inside_time_range(time_now: Time, time_start: Option<Time>, time_end: Option<Time>) -> bool {
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
fn parse_time_range(time_range: &str) -> (Option<Time>, Option<Time>) {
    let value = String::from(time_range);

    // Check if there is exactly one hyphen, and fail otherwise
    if value.matches('-').count() != 1 {
        return (None, None);
    }

    // Split time_range into the two ranges
    let (start, end) = value.split_at(value.find('-').unwrap());
    let end = &end[1..];

    // Parse the ranges
    let start_time = start.parse::<Time>().ok();
    let end_time = end.parse::<Time>().ok();

    (start_time, end_time)
}

/* Because we cannot make acceptance tests for the time module, these unit
tests become extra important */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use jiff::civil::date;

    const FMT_12: &str = "%r";
    const FMT_24: &str = "%T";

    fn configured_schedule(format: &str, offset: &str, range: &str) -> Option<Schedule> {
        let configuration = toml::Table::from_iter([(
            "time".to_owned(),
            toml::Table::from_iter([
                ("disabled".to_owned(), false.into()),
                ("time_format".to_owned(), format.into()),
                ("utc_time_offset".to_owned(), offset.into()),
                ("time_range".to_owned(), range.into()),
            ])
            .into(),
        )]);
        schedule(&crate::test::default_context().set_config(configuration))
    }

    #[test]
    fn refresh_boundaries_follow_the_same_time_zone_as_the_display() {
        for (format, offset, now, before, after, period) in [
            (
                "%F",
                "-4",
                "2026-10-16T03:59:59.750Z",
                "2026-10-15",
                "2026-10-16",
                86_400,
            ),
            ("%H", "+5.75", "2026-10-16T06:14:59.750Z", "11", "12", 3_600),
        ] {
            let schedule = configured_schedule(format, offset, "-").unwrap();
            let now: Timestamp = now.parse().unwrap();
            let delay = schedule.delay(now).unwrap();
            assert_eq!(delay, Duration::from_millis(250), "{format} in {offset}");
            assert_eq!(
                before,
                format_time(format, now.to_zoned(schedule.time_zone.clone()))
            );
            let boundary = now.checked_add(delay).unwrap();
            assert_eq!(
                after,
                format_time(format, boundary.to_zoned(schedule.time_zone.clone()))
            );
            assert_eq!(schedule.delay(boundary), Some(Duration::from_secs(period)));
        }
    }

    #[test]
    fn timezone_only_formats_refresh_at_the_next_actual_dst_transition() {
        let before: Timestamp = "2026-11-01T05:59:59.750Z".parse().unwrap();
        let after: Timestamp = "2026-11-01T06:00:00Z".parse().unwrap();
        for (format, old, new) in [("%Z", "EDT", "EST"), ("%:z", "-04:00", "-05:00")] {
            let schedule = configured_schedule(format, "America/New_York", "-").unwrap();
            assert_eq!(schedule.delay(before), Some(Duration::from_millis(250)));
            assert_eq!(
                old,
                format_time(format, before.to_zoned(schedule.time_zone.clone()))
            );
            assert_eq!(
                new,
                format_time(format, after.to_zoned(schedule.time_zone.clone()))
            );
            assert_eq!(
                schedule.delay(after),
                Some(Duration::from_secs(23 * 3_600)),
                "the next midnight uses the new offset"
            );
        }
    }

    #[test]
    fn upcoming_zone_rules_are_probed_when_the_reference_dates_miss_the_season() {
        // All the reference probes fall outside this September-only season.
        let zone = TimeZone::posix("STD0DST,M9.1.0,M10.1.0").unwrap();
        let now: Timestamp = "2026-01-01T00:00:00Z".parse().unwrap();
        for format in ["%Z", "%:z"] {
            assert_eq!(
                format_period(format, &zone, now),
                Some(Duration::from_secs(86_400)),
                "{format}"
            );
        }
        assert_eq!(None, format_period("%%Z %%:z", &zone, now));
    }

    #[test]
    fn constant_timezone_formats_have_no_refresh_schedule() {
        for (format, zone) in [
            ("%Z", "Etc/UTC"),
            ("%:z", "Etc/UTC"),
            ("%:z", "0"),
            ("%Q", "America/New_York"),
            ("%%Z %%:z", "America/New_York"),
        ] {
            assert!(
                configured_schedule(format, zone, "-").is_none(),
                "{format} in {zone}"
            );
        }
    }

    #[test]
    fn a_local_time_range_is_checked_each_second_even_in_another_display_zone() {
        let schedule = configured_schedule("today", "+5.75", "10:00:00-14:00:00").unwrap();
        let now: Timestamp = "2026-10-16T06:14:59.750Z".parse().unwrap();
        assert_eq!(schedule.period, Duration::from_secs(1));
        assert_eq!(schedule.delay(now), Some(Duration::from_millis(250)));
    }

    #[test]
    fn the_time_shown_turns_over_as_often_as_jiff_shows_it_otherwise() {
        let every = |milliseconds| Some(Duration::from_millis(milliseconds));
        for (time_format, time_range, period) in [
            (None, "-", every(1_000)),
            (Some("%-I:%M %p"), "-", every(60_000)),
            (Some("%:z %R"), "-", every(60_000)),
            (Some("%I %p"), "-", every(3_600_000)),
            (Some("%Y-%m-%d"), "-", every(86_400_000)),
            (Some("%Y"), "-", every(86_400_000)),
            (Some("%y"), "-", every(86_400_000)),
            (Some("%C"), "-", every(86_400_000)),
            (Some("%G"), "-", every(86_400_000)),
            (Some("%g"), "-", every(86_400_000)),
            (Some("%B"), "-", every(86_400_000)),
            (Some("%b"), "-", every(86_400_000)),
            (Some("%h"), "-", every(86_400_000)),
            (Some("%m"), "-", every(86_400_000)),
            (Some("%q"), "-", every(86_400_000)),
            (Some("%U"), "-", every(86_400_000)),
            (Some("%V"), "-", every(86_400_000)),
            (Some("%W"), "-", every(86_400_000)),
            (Some("%H:%M:%S%.3f"), "-", every(10)),
            (Some("%1f"), "-", every(100)),
            (Some("%.1f"), "-", every(100)),
            (Some("%1N"), "-", every(100)),
            (Some("%01f"), "-", every(100)),
            (Some("%5.1f"), "-", every(100)),
            (Some("%S %.1f"), "-", every(100)),
            (Some("%2f"), "-", every(10)),
            (Some("%.3f"), "-", every(10)),
            (Some("%f"), "-", every(10)),
            (Some("%.f"), "-", every(10)),
            (Some("%N"), "-", every(10)),
            (Some("%0N"), "-", every(10)),
            (Some("%0f"), "-", every(10)),
            (Some("%9f"), "-", every(10)),
            (Some("%%f %S"), "-", every(1_000)),
            (Some("%.0f"), "-", None),
            (Some("100%% done"), "-", None),
            (Some("%%Y %%B %%C %%V"), "-", None),
            (Some("at noon"), "-", None),
            (Some("trailing %"), "-", None),
            (Some("at noon"), "10:00:00-14:00:00", every(1_000)),
            (Some("at noon"), "10:00:00-14:00:00-15:00:00", None),
        ] {
            let mut time = toml::Table::new();
            time.insert("disabled".to_owned(), false.into());
            time.insert("time_range".to_owned(), time_range.into());
            if let Some(time_format) = time_format {
                time.insert("time_format".to_owned(), time_format.into());
            }
            let context = crate::test::default_context()
                .set_config(toml::Table::from_iter([("time".to_owned(), time.into())]));

            assert_eq!(
                period,
                super::period(&context),
                "{time_format:?} in {time_range}"
            );
        }
        assert_eq!(
            None,
            super::period(&crate::test::default_context()),
            "disabled"
        );
    }

    #[test]
    fn test_midnight_12hr() {
        let time = date(2014, 7, 8)
            .at(0, 0, 0, 0)
            .to_zoned(TimeZone::system())
            .unwrap();
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "12:00:00 AM");
    }

    #[test]
    fn test_midnight_24hr() {
        let time = date(2014, 7, 8)
            .at(0, 0, 0, 0)
            .to_zoned(TimeZone::system())
            .unwrap();
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "00:00:00");
    }

    #[test]
    fn test_noon_12hr() {
        let time = date(2014, 7, 8)
            .at(12, 0, 0, 0)
            .to_zoned(TimeZone::system())
            .unwrap();
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "12:00:00 PM");
    }

    #[test]
    fn test_noon_24hr() {
        let time = date(2014, 7, 8)
            .at(12, 0, 0, 0)
            .to_zoned(TimeZone::system())
            .unwrap();
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "12:00:00");
    }

    #[test]
    fn test_arbtime_12hr() {
        let time = date(2014, 7, 8)
            .at(15, 36, 47, 0)
            .to_zoned(TimeZone::system())
            .unwrap();
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "3:36:47 PM");
    }

    #[test]
    fn test_arbtime_24hr() {
        let time = date(2014, 7, 8)
            .at(15, 36, 47, 0)
            .to_zoned(TimeZone::system())
            .unwrap();
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "15:36:47");
    }

    #[test]
    fn test_format_with_paren() {
        let time = date(2014, 7, 8)
            .at(15, 36, 47, 0)
            .to_zoned(TimeZone::system())
            .unwrap();
        let formatted = format_time("[%T]", time);
        assert_eq!(formatted, "[15:36:47]");
    }

    #[test]
    fn test_midnight_12hr_fixed_offset() {
        let timezone_offset = Offset::from_seconds(0).unwrap();
        let tz = TimeZone::fixed(timezone_offset);
        let time = date(2014, 7, 8).at(0, 0, 0, 0).to_zoned(tz).unwrap();
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "12:00:00 AM");
    }

    #[test]
    fn test_midnight_24hr_fixed_offset() {
        let timezone_offset = Offset::from_seconds(0).unwrap();
        let tz = TimeZone::fixed(timezone_offset);
        let time = date(2014, 7, 8).at(0, 0, 0, 0).to_zoned(tz).unwrap();
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "00:00:00");
    }

    #[test]
    fn test_noon_12hr_fixed_offset() {
        let timezone_offset = Offset::from_seconds(0).unwrap();
        let tz = TimeZone::fixed(timezone_offset);
        let time = date(2014, 7, 8).at(12, 0, 0, 0).to_zoned(tz).unwrap();
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "12:00:00 PM");
    }

    #[test]
    fn test_noon_24hr_fixed_offset() {
        let timezone_offset = Offset::from_seconds(0).unwrap();
        let tz = TimeZone::fixed(timezone_offset);
        let time = date(2014, 7, 8).at(12, 0, 0, 0).to_zoned(tz).unwrap();
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "12:00:00");
    }

    #[test]
    fn test_arbtime_12hr_fixed_offset() {
        let timezone_offset = Offset::from_seconds(0).unwrap();
        let tz = TimeZone::fixed(timezone_offset);
        let time = date(2014, 7, 8).at(15, 36, 47, 0).to_zoned(tz).unwrap();
        let formatted = format_time(FMT_12, time);
        assert_eq!(formatted, "3:36:47 PM");
    }

    #[test]
    fn test_arbtime_24hr_fixed_offset() {
        let timezone_offset = Offset::from_seconds(0).unwrap();
        let tz = TimeZone::fixed(timezone_offset);
        let time = date(2014, 7, 8).at(15, 36, 47, 0).to_zoned(tz).unwrap();
        let formatted = format_time(FMT_24, time);
        assert_eq!(formatted, "15:36:47");
    }

    #[test]
    fn test_format_with_paren_fixed_offset() {
        let timezone_offset = Offset::from_seconds(0).unwrap();
        let tz = TimeZone::fixed(timezone_offset);
        let time = date(2014, 7, 8).at(15, 36, 47, 0).to_zoned(tz).unwrap();
        let formatted = format_time("[%T]", time);
        assert_eq!(formatted, "[15:36:47]");
    }

    #[test]
    fn numeric_utc_offsets_are_applied_to_the_displayed_time() {
        let utc_time = date(2014, 7, 8)
            .at(15, 36, 47, 0)
            .to_zoned(TimeZone::UTC)
            .unwrap()
            .timestamp();
        for (offset, expected) in [
            ("-3", "12:36:47 PM"),
            ("+5", "8:36:47 PM"),
            ("+9.5", "1:06:47 AM"),
            ("+5.75", "9:21:47 PM"),
        ] {
            let time_zone = parse_utc_time_offset(offset).unwrap();
            assert_eq!(format_time(FMT_12, utc_time.to_zoned(time_zone)), expected);
        }
    }

    #[test]
    fn numeric_utc_offsets_reject_invalid_values() {
        for offset in ["+24", "-24", "+9001", "-4242", "completely wrong config"] {
            parse_utc_time_offset(offset).expect_err("Invalid timezone offset.");
        }
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
            (Some(Time::new(10, 0, 0, 0).unwrap()), None)
        );
    }

    #[test]
    fn test_parse_end_time_range() {
        let time_range = "-22:00:00";

        assert_eq!(
            parse_time_range(time_range),
            (None, Some(Time::new(22, 0, 0, 0).unwrap()))
        );
    }

    #[test]
    fn test_parse_both_time_ranges() {
        let time_range = "10:00:00-16:00:00";

        assert_eq!(
            parse_time_range(time_range),
            (
                Some(Time::new(10, 0, 0, 0).unwrap()),
                Some(Time::new(16, 0, 0, 0).unwrap())
            )
        );
    }

    #[test]
    fn test_is_inside_time_range_with_no_range() {
        let time_start = None;
        let time_end = None;
        let time_now = Time::new(10, 0, 0, 0).unwrap();

        assert!(is_inside_time_range(time_now, time_start, time_end));
    }

    #[test]
    fn test_is_inside_time_range_with_start_range() {
        let time_start = Some(Time::new(10, 0, 0, 0).unwrap());
        let time_now = Time::new(12, 0, 0, 0).unwrap();
        let time_now2 = Time::new(8, 0, 0, 0).unwrap();

        assert!(is_inside_time_range(time_now, time_start, None));
        assert!(!is_inside_time_range(time_now2, time_start, None));
    }

    #[test]
    fn test_is_inside_time_range_with_end_range() {
        let time_end = Some(Time::new(16, 0, 0, 0).unwrap());
        let time_now = Time::new(15, 0, 0, 0).unwrap();
        let time_now2 = Time::new(19, 0, 0, 0).unwrap();

        assert!(is_inside_time_range(time_now, None, time_end));
        assert!(!is_inside_time_range(time_now2, None, time_end));
    }

    #[test]
    fn test_is_inside_time_range_with_complete_range() {
        let time_start = Some(Time::new(9, 0, 0, 0).unwrap());
        let time_end = Some(Time::new(18, 0, 0, 0).unwrap());
        let time_now = Time::new(3, 0, 0, 0).unwrap();
        let time_now2 = Time::new(13, 0, 0, 0).unwrap();
        let time_now3 = Time::new(20, 0, 0, 0).unwrap();

        assert!(!is_inside_time_range(time_now, time_start, time_end));
        assert!(is_inside_time_range(time_now2, time_start, time_end));
        assert!(!is_inside_time_range(time_now3, time_start, time_end));
    }

    #[test]
    fn test_is_inside_time_range_with_complete_range_passing_midnight() {
        let time_start = Some(Time::new(19, 0, 0, 0).unwrap());
        let time_end = Some(Time::new(12, 0, 0, 0).unwrap());
        let time_now = Time::new(3, 0, 0, 0).unwrap();
        let time_now2 = Time::new(13, 0, 0, 0).unwrap();
        let time_now3 = Time::new(20, 0, 0, 0).unwrap();

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

    #[test]
    fn config_check_invalid_tz() {
        let actual = ModuleRenderer::new("time")
            .config(toml::toml! {
                [time]
                disabled = false
                time_format = "%T"
                utc_time_offset = "invalid"
            })
            .collect();

        assert!(actual.is_some(), "Falls back to local time");
    }

    #[test]
    fn module_tz() {
        use nu_ansi_term::Color;

        let actual = ModuleRenderer::new("time")
            .config(toml::toml! {
                [time]
                disabled = false
                time_format = "%:z"
                utc_time_offset = "Asia/Kolkata"
            })
            .collect();

        let expected = Some(format!("at {} ", Color::Yellow.bold().paint("+05:30")));

        assert_eq!(actual, expected, "Uses timezone");
    }

    #[test]
    fn module_offset() {
        use nu_ansi_term::Color;

        let actual = ModuleRenderer::new("time")
            .config(toml::toml! {
                [time]
                disabled = false
                time_format = "%:z"
                utc_time_offset = "-1.75"
            })
            .collect();

        let expected = Some(format!("at {} ", Color::Yellow.bold().paint("-01:45")));

        assert_eq!(actual, expected, "Uses timezone offset");
    }

    #[test]
    fn module_tz_abbreviation() {
        let actual = ModuleRenderer::new("time")
            .config(toml::toml! {
                [time]
                disabled = false
                time_format = "%Z"
                utc_time_offset = "America/New_York"
            })
            .collect();

        // Should output a timezone abbreviation like "EST" or "EDT"
        assert!(
            actual.is_some(),
            "Timezone abbreviation should be displayed"
        );
        let output = actual.unwrap();
        assert!(
            output.contains("EST") || output.contains("EDT"),
            "Should contain timezone abbreviation"
        );
    }

    #[test]
    fn module_tz_identifier() {
        use nu_ansi_term::Color;

        let actual = ModuleRenderer::new("time")
            .config(toml::toml! {
                [time]
                disabled = false
                time_format = "%Q"
                utc_time_offset = "America/New_York"
            })
            .collect();

        let expected = Some(format!(
            "at {} ",
            Color::Yellow.bold().paint("America/New_York")
        ));

        assert_eq!(actual, expected, "Uses IANA timezone identifier");
    }
}
