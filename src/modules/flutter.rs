use super::{Context, Module, ModuleConfig};

use crate::configs::flutter::FlutterConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils::get_command_string_output;
use yaml_rust::Yaml;
use yaml_rust::YamlLoader;

/// Creates a module with the current Flutter version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("flutter");
    let config = FlutterConfig::try_load(module.config);

    let is_dart_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_dart_project {
        return None;
    }

    // try to read and parse pubspec.yaml and check for flutter dependency
    if config.check_pubspec {
        has_flutter_dependency(context)?;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        let flutter_output = get_flutter_version(context);
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
                "version" => VersionFormatter::format_module_version(
                    module.get_name(),
                    &flutter_output.clone()?.0,
                    config.version_format,
                )
                .map(Ok),
                _ => None,
            })
            .map(|variable| match variable {
                "channel" => flutter_output.clone().map(|(_, c)| Ok(c)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `flutter`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_flutter_version(context: &Context) -> Option<(String, String)> {
    let command = context.exec_cmd("flutter", &["--version"])?;
    let output = &get_command_string_output(command);
    let (flutter_version, channel) = parse_flutter_version(output)?;
    Some((flutter_version, channel))
}

fn has_flutter_dependency(context: &Context) -> Option<()> {
    // let docs = YamlLoader::load_from_str(&pubspec)?;
    // let doc = docs.get(0).unwrap();
    // let dependencies = doc["dependencies"].as_hash();
    // if dependencies.is_none() {
    //     return None;
    // }
    // let flutter = dependencies.unwrap().get(&"flutter");
    let flutter_key = Yaml::String("flutter".to_string());
    let doc = context
        .read_file_from_pwd("pubspec.yaml")
        .map(|c| YamlLoader::load_from_str(&c))?;
    match doc {
        Ok(docs) => docs
            .get(0)
            .and_then(|doc| doc["dependencies"].as_hash())
            .and_then(|deps| deps.get(&flutter_key))
            .map(|_| ()),
        Err(e) => {
            log::warn!(
                "Error in module `flutter`: \nFailed to parse pubspec.yaml\n{}",
                e
            );
            None
        }
    }
}

fn parse_flutter_version(flutter_version: &str) -> Option<(String, String)> {
    let split = flutter_version
        // Flutter 3.11.0-1.0.pre.1 • channel beta • https://github.com/flutter/flutter.git
        // split into ["Flutter", "3.11.0-1.0.pre.1", "•", "channel", "beta", "•", "https://github.com/flutter/flutter.git"]
        .split_whitespace();
    Some((
        split.clone().nth(1)?.to_string(),
        split.clone().nth(4)?.to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io::{self, Write};

    const EXPECTED_FLUTTER_VERSION: &str = "🐦 v3.11.0-1.0.pre.1 (beta) ";

    #[test]
    fn folder_without_dart_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dart_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.dart"))?;
        write_pubspec(&dir)?;

        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint(EXPECTED_FLUTTER_VERSION)
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    fn write_pubspec(dir: &tempfile::TempDir) -> Result<(), io::Error> {
        let mut pubspec = File::create(dir.path().join("pubspec.yaml"))?;
        pubspec.write_all(
            b"
            name: any
            dependencies:
              flutter:
                sdk: flutter
        ",
        )?;
        pubspec.sync_all()?;
        Ok(())
    }
    fn write_pubspec_without_flutter(dir: &tempfile::TempDir) -> Result<(), io::Error> {
        let mut pubspec = File::create(dir.path().join("pubspec.yaml"))?;
        pubspec.write_all(
            b"
            name: any
            dependencies:
                cupertino_icons: ^0.1.2
        ",
        )?;
        pubspec.sync_all()?;
        Ok(())
    }

    #[test]
    fn folder_with_dart_tool_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join(".dart_tool"))?;
        write_pubspec(&dir)?;

        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint(EXPECTED_FLUTTER_VERSION)
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_yaml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        write_pubspec(&dir)?;

        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint(EXPECTED_FLUTTER_VERSION)
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_yaml_file_without_flutter_dependency() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        write_pubspec_without_flutter(&dir)?;

        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        assert_eq!(None, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_lock_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pubspec.lock"))?.sync_all()?;
        write_pubspec(&dir)?;

        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint(EXPECTED_FLUTTER_VERSION)
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn detect_version_output_in_stdout() -> io::Result<()> {
        // after dart 2.15.0, version info output in stdout.
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.dart"))?.sync_all()?;
        write_pubspec(&dir)?;

        let actual = ModuleRenderer::new("flutter")
            .cmd(
                "flutter --version",
                Some(CommandOutput {
                    stdout: String::from("Flutter 3.11.0-1.0.pre.1 • channel beta • https://github.com/flutter/flutter.git
                    Framework • revision 74e4b092e5 (2 weeks ago) • 2023-05-10 07:08:22 -0700
                    Engine • revision 992cdb6cd4
                    Tools • Dart 3.1.0 (build 3.1.0-63.1.beta) • DevTools 2.23.1"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint("🐦 v3.11.0-1.0.pre.1 (beta) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
