use super::{Context, Module, RootModuleConfig};

use crate::configs::dart::DartConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Dart version
///
/// Will display the Dart version if any of the following criteria are met:
///     - Current directory contains a file with `.dart` extension
///     - Current directory contains a `.dart_tool` directory
///     - Current directory contains a `pubspec.yaml`/`pubspec.yml` or `pubspec.lock` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_dart_project = context
        .try_begin_scan()?
        .set_extensions(&["dart"])
        .set_folders(&[".dart_tool"])
        .set_files(&["pubspec.yaml", "pubspec.yml", "pubspec.lock"])
        .is_match();

    if !is_dart_project {
        return None;
    }

    let dart_version = utils::exec_cmd("dart", &["--version"])?.stderr;

    let mut module = context.new_module("dart");
    let config: DartConfig = DartConfig::try_load(module.config);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => parse_dart_version(&dart_version).map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `dart`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_dart_version(dart_version: &str) -> Option<String> {
    let version = dart_version
        // split into ["Dart", "VM", "version:", "2.8.4", "(stable)", ...]
        .split_whitespace()
        // return "2.8.4"
        .nth(3)?;

    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::parse_dart_version;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn test_parse_dart_version() {
        let input = "Dart VM version: 2.8.4 (stable)";
        assert_eq!(parse_dart_version(input), Some("v2.8.4".to_string()));
    }

    #[test]
    fn folder_without_dart_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("dart", dir.path(), None);
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dart_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.dart"))?.sync_all()?;

        let actual = render_module("dart", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🎯 v2.8.4")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dart_tool_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join(".dart_tool"))?;

        let actual = render_module("dart", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🎯 v2.8.4")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_yaml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pubspec.yaml"))?.sync_all()?;

        let actual = render_module("dart", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🎯 v2.8.4")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_yml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pubspec.yml"))?.sync_all()?;

        let actual = render_module("dart", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🎯 v2.8.4")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_lock_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pubspec.lock"))?.sync_all()?;

        let actual = render_module("dart", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🎯 v2.8.4")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
