// Duration is a struct that represents a duration of time. It is used to handle the time elapsed of a
// command execution.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Duration {
    total: u64,
    milliseconds: u64,
    seconds: u64,
    minutes: u64,
    hours: u64,
    days: u64,
}

impl From<u64> for Duration {
    fn from(raw: u64) -> Self {
        let (milliseconds, leftover_seconds) = (raw % 1000, raw / 1000);
        let (seconds, leftover_minutes) = (leftover_seconds % 60, leftover_seconds / 60);
        let (minutes, leftover_hours) = (leftover_minutes % 60, leftover_minutes / 60);
        let (hours, days) = (leftover_hours % 24, leftover_hours / 24);

        Self {
            total: raw,
            milliseconds,
            seconds,
            minutes,
            hours,
            days,
        }
    }
}

impl Duration {
    fn format(value: u64, padded: bool) -> String {
        if padded {
            return format!("{value:02}");
        }

        value.to_string()
    }

    pub fn fmt_days(self, padded: bool) -> String {
        Self::format(self.days, padded)
    }

    pub fn fmt_hours(self, padded: bool) -> String {
        Self::format(self.hours, padded)
    }

    pub fn fmt_minutes(self, padded: bool) -> String {
        Self::format(self.minutes, padded)
    }

    pub fn fmt_seconds(self, padded: bool) -> String {
        Self::format(self.seconds, padded)
    }

    pub fn fmt_humanized(self, show_milliseconds: bool) -> String {
        // Make sure it renders something if the time equals zero instead of an empty string
        if self.total == 0 {
            return "0ms".into();
        }

        let components = [self.days, self.hours, self.minutes, self.seconds];
        let suffixes = ["d", "h", "m", "s"];

        let mut rendered_components: Vec<String> = components
            .iter()
            .zip(&suffixes)
            .map(Self::humanized_component)
            .collect();

        if show_milliseconds || self.total < 1000 {
            rendered_components.push(Self::humanized_component((&self.milliseconds, &"ms")));
        }

        rendered_components.join("")
    }

    fn humanized_component((component, suffix): (&u64, &&str)) -> String {
        match component {
            0 => String::new(),
            n => format!("{n}{suffix}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0ms() {
        assert_eq!(Duration::from(0).fmt_humanized(true), "0ms")
    }

    #[test]
    fn test_500ms() {
        assert_eq!(Duration::from(500).fmt_humanized(true), "500ms")
    }

    #[test]
    fn test_10s() {
        assert_eq!(Duration::from(10_000).fmt_humanized(true), "10s")
    }

    #[test]
    fn test_90s() {
        assert_eq!(Duration::from(90_000).fmt_humanized(true), "1m30s")
    }

    #[test]
    fn test_10110s() {
        assert_eq!(Duration::from(10_110_000).fmt_humanized(true), "2h48m30s")
    }

    #[test]
    fn test_1d() {
        assert_eq!(Duration::from(86_400_000).fmt_humanized(true), "1d")
    }
}
