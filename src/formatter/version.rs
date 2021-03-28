use super::string_formatter::StringFormatterError;
use super::StringFormatter;
use versions::Versioning;

pub struct VersionFormatter<'a> {
    formatter: StringFormatter<'a>,
}

impl<'a> VersionFormatter<'a> {
    /// Creates an instance of a VersionFormatter from a format string
    ///
    /// Like the StringFormatter, this will throw an error when the string isn't
    /// parseable.
    pub fn new(format: &'a str) -> Result<Self, StringFormatterError> {
        let formatter = StringFormatter::new(format)?;

        Ok(Self { formatter })
    }

    /// Formats a version structure into a readable string
    ///
    /// No matter what comes in, this will return some usable string
    pub fn format_version(self, version: &str) -> String {
        let parsed = Versioning::new(version);
        let formatted = self
            .formatter
            .map(|variable| match variable {
                "raw" => Some(Ok(version.to_string())),
                "major" => match parsed.as_ref() {
                    Some(Versioning::Ideal(v)) => Some(Ok(v.major.to_string())),
                    Some(Versioning::General(v)) => Some(Ok(v.nth_lenient(0)?.to_string())),
                    _ => None,
                },
                "minor" => match parsed.as_ref() {
                    Some(Versioning::Ideal(v)) => Some(Ok(v.minor.to_string())),
                    Some(Versioning::General(v)) => Some(Ok(v.nth_lenient(1)?.to_string())),
                    _ => None,
                },
                "patch" => match parsed.as_ref() {
                    Some(Versioning::Ideal(v)) => Some(Ok(v.patch.to_string())),
                    Some(Versioning::General(v)) => Some(Ok(v.nth_lenient(2)?.to_string())),
                    _ => None,
                },
                _ => None,
            })
            .parse(None);
        match formatted {
            Ok(segments) => segments
                .iter()
                .map(|segment| segment.value.as_str())
                .collect::<Vec<&str>>()
                .join(""),
            Err(_) => version.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_full() {
        const FORMAT_STR: &str = "major:${major} minor:${minor} patch:${patch} raw:${raw}";
        let result = VersionFormatter::new(FORMAT_STR)
            .unwrap()
            .format_version("1.2.3");
        assert_eq!(result, "major:1 minor:2 patch:3 raw:1.2.3");
    }

    #[test]
    fn test_semver_partial() {
        const FORMAT_STR: &str = "major:${major} minor:${minor} patch:${patch} raw:${raw}";
        let result = VersionFormatter::new(FORMAT_STR)
            .unwrap()
            .format_version("1.2");
        assert_eq!(result, "major:1 minor:2 patch: raw:1.2");
    }

    #[test]
    fn test_general() {
        const FORMAT_STR: &str = "major:${major} minor:${minor} patch:${patch} raw:${raw}";
        let result = VersionFormatter::new(FORMAT_STR)
            .unwrap()
            .format_version("1.2-a.3");
        assert_eq!(result, "major:1 minor:2 patch: raw:1.2-a.3");
    }

    #[test]
    fn test_mess() {
        const FORMAT_STR: &str = "major:${major} minor:${minor} patch:${patch} raw:${raw}";
        let result = VersionFormatter::new(FORMAT_STR)
            .unwrap()
            .format_version("utter junk");
        assert_eq!(result, "major: minor: patch: raw:utter junk");
    }
}
