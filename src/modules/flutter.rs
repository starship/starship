use super::{Context, Module, ModuleConfig};

use crate::configs::flutter::FlutterConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils::get_command_string_output;

/// Creates a module with the current Flutter version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("flutter");
    let config: FlutterConfig = FlutterConfig::try_load(module.config);
    
    let dir_contents = context.dir_contents().ok()?;
    let is_flutter_project = dir_contents.has_any_positive_file_name(&config.detect_files)
        && dir_contents.has_all_positive_folder(&config.detect_folders);

    if !is_flutter_project {
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
                    let command = context.exec_cmd("cat", &[".dart_tool/version"])?;
                    let flutter_version = &get_command_string_output(command);
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &flutter_version,
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
            log::warn!("Error in module `flutter`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;
    use std::io::Write;

    #[test]
    fn folder_without_ios_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join(".dart_tool"))?;
        fs::create_dir_all(dir.path().join("android"))?;
        File::create(dir.path().join("pubspec.yaml"))?.sync_all()?;

        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }
    
    #[test]
    fn folder_without_android_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join(".dart_tool"))?;
        fs::create_dir_all(dir.path().join("ios"))?;
        File::create(dir.path().join("pubspec.yaml"))?.sync_all()?;
        
        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }
    
    #[test]
    fn folder_without_dart_tool_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join("android"))?;
        fs::create_dir_all(dir.path().join("ios"))?;
        File::create(dir.path().join("pubspec.yaml"))?.sync_all()?;
        
        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_yaml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join(".dart_tool"))?;
        fs::create_dir_all(dir.path().join("android"))?;
        fs::create_dir_all(dir.path().join("ios"))?;
        File::create(dir.path().join("pubspec.yaml"))?.sync_all()?;
        File::create(dir.path().join(".dart_tool/version"))?.write_all(b"3.3.8")?;

        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🇫 v3.3.8 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_yml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join(".dart_tool"))?;
        fs::create_dir_all(dir.path().join("android"))?;
        fs::create_dir_all(dir.path().join("ios"))?;
        File::create(dir.path().join("pubspec.yml"))?.sync_all()?;
        File::create(dir.path().join(".dart_tool/version"))?.write_all(b"3.3.8")?;
    
        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🇫 v3.3.8 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pubspec_lock_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join(".dart_tool"))?;
        fs::create_dir_all(dir.path().join("android"))?;
        fs::create_dir_all(dir.path().join("ios"))?;
        File::create(dir.path().join("pubspec.lock"))?.sync_all()?;
        File::create(dir.path().join(".dart_tool/version"))?.write_all(b"3.3.8")?;
    
        let actual = ModuleRenderer::new("flutter").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🇫 v3.3.8 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn detect_version_output_in_stdout() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join(".dart_tool"))?;
        fs::create_dir_all(dir.path().join("android"))?;
        fs::create_dir_all(dir.path().join("ios"))?;
        File::create(dir.path().join("pubspec.yaml"))?.sync_all()?;
    
        let actual = ModuleRenderer::new("flutter")
            .cmd(
                "cat .dart_tool/version",
                Some(CommandOutput {
                    stdout: String::from("3.3.8"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🇫 v3.3.8 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
