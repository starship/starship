use super::{Context, Module, ModuleConfig};

use crate::configs::dart::DartConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils::get_command_string_output;

/// Creates a module with the current Dart version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("dart");
    let config: DartConfig = DartConfig::try_load(module.config);

    let is_dart_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_dart_project {
        return None;
    }

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
                "version" => {
                    let command = context.exec_cmd("dart", &["--version"])?;
                    let dart_version = parse_dart_version(&get_command_string_output(command))?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &dart_version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
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
    Some(
        dart_version
            // split into ["Dart", "VM", "version:", "2.8.4", "(stable)", ...]
            .split_whitespace()
            // return "2.8.4"
            .nth(3)?
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn folder_without_dart_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("dart").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dart_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.dart"))?.sync_all()?;

        let actual = ModuleRenderer::new("dart").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🎯 v2.8.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dart_tool_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join(".dart_tool"))?;

        let actual = ModuleRenderer::new("dart").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🎯 v2.8.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_yaml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pubspec.yaml"))?.sync_all()?;

        let actual = ModuleRenderer::new("dart").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🎯 v2.8.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_yml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pubspec.yml"))?.sync_all()?;

        let actual = ModuleRenderer::new("dart").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🎯 v2.8.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_lock_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pubspec.lock"))?.sync_all()?;

        let actual = ModuleRenderer::new("dart").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🎯 v2.8.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn detect_version_output_in_stdout() -> io::Result<()> {
        // after dart 2.15.0, version info output in stdout.
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.dart"))?.sync_all()?;

        let actual = ModuleRenderer::new("dart")
            .cmd(
                "dart --version",
                Some(CommandOutput {
                    stdout: String::from("Dart SDK version: 2.15.1 (stable) (Tue Dec 14 13:32:21 2021 +0100) on \"linux_x64\""),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🎯 v2.15.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
