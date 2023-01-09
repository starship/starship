use super::string_formatter::StringFormatterError;
use super::StringFormatter;
use crate::segment;
use once_cell::sync::Lazy;
use std::ops::Deref;
use versions::Versioning;

pub struct VersionFormatter<'a> {
    formatter: StringFormatter<'a>,
}

impl<'a> VersionFormatter<'a> {
    /// Creates an instance of a `VersionFormatter` from a format string
    ///
    /// Like the `StringFormatter`, this will throw an error when the string isn't
    /// parseable.
    pub fn new(format: &'a str) -> Result<Self, StringFormatterError> {
        let formatter = StringFormatter::new(format)?;

        Ok(Self { formatter })
    }

    /// Format version string using provided format
    pub fn format_version(
        version: &'a str,
        format: &'a str,
    ) -> Result<String, StringFormatterError> {
        Self::new(format).and_then(|formatter| formatter.format(version))
    }

    /// Formats a version structure into a readable string
    pub fn format(self, version: &'a str) -> Result<String, StringFormatterError> {
        let parsed = Lazy::new(|| Versioning::new(version));
        let formatted = self
            .formatter
            .map(|variable| match variable {
                "raw" => Some(Ok(version.to_string())),
                "major" => match parsed.deref().as_ref() {
                    Some(Versioning::Ideal(v)) => Some(Ok(v.major.to_string())),
                    Some(Versioning::General(v)) => Some(Ok(v.nth_lenient(0)?.to_string())),
                    _ => None,
                },
                "minor" => match parsed.deref().as_ref() {
                    Some(Versioning::Ideal(v)) => Some(Ok(v.minor.to_string())),
                    Some(Versioning::General(v)) => Some(Ok(v.nth_lenient(1)?.to_string())),
                    _ => None,
                },
                "patch" => match parsed.deref().as_ref() {
                    Some(Versioning::Ideal(v)) => Some(Ok(v.patch.to_string())),
                    Some(Versioning::General(v)) => Some(Ok(v.nth_lenient(2)?.to_string())),
                    _ => None,
                },
                _ => None,
            })
            .parse(None, None);

        formatted.map(|segments| {
            segments
                .iter()
                .map(segment::Segment::value)
                .collect::<String>()
        })
    }

    pub fn format_module_version(
        module_name: &str,
        version: &str,
        version_format: &str,
    ) -> Option<String> {
        match VersionFormatter::format_version(version, version_format) {
            Ok(formatted) => Some(formatted),
            Err(error) => {
                log::warn!("Error formatting `{}` version:\n{}", module_name, error);
                Some(format!("v{version}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VERSION_FORMAT: &str = "major:${major} minor:${minor} patch:${patch} raw:${raw}";

    #[test]
    fn test_semver_full() {
        assert_eq!(
            VersionFormatter::format_version("1.2.3", VERSION_FORMAT),
            Ok("major:1 minor:2 patch:3 raw:1.2.3".to_string())
        );
    }

    #[test]
    fn test_semver_partial() {
        assert_eq!(
            VersionFormatter::format_version("1.2", VERSION_FORMAT),
            Ok("major:1 minor:2 patch: raw:1.2".to_string())
        );
    }

    #[test]
    fn test_general() {
        assert_eq!(
            VersionFormatter::format_version("1.2-a.3", VERSION_FORMAT),
            Ok("major:1 minor:2 patch: raw:1.2-a.3".to_string())
        );
    }

    #[test]
    fn test_dummy() {
        assert_eq!(
            VersionFormatter::format_version("dummy version", VERSION_FORMAT),
            Ok("major: minor: patch: raw:dummy version".to_string())
        );
    }
}
