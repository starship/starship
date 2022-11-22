use super::{Context, Module, ModuleConfig};
use crate::configs::daml::DamlConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

const DAML_SDK_VERSION: &str = "sdk-version";
const DAML_SDK_VERSION_ENV: &str = "DAML_SDK_VERSION";
const DAML_YAML: &str = "daml.yaml";

/// Creates a module with the current Daml version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("daml");
    let config: DamlConfig = DamlConfig::try_load(module.config);

    let is_daml_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_daml_project {
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
                    let daml_sdk_version = get_daml_sdk_version(context)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &daml_sdk_version,
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
            log::warn!("Error in module `daml`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_daml_sdk_version(context: &Context) -> Option<String> {
    context
        .get_env(DAML_SDK_VERSION_ENV)
        .or_else(|| read_sdk_version_from_daml_yaml(context))
}

fn read_sdk_version_from_daml_yaml(context: &Context) -> Option<String> {
    let file_contents = context.read_file_from_pwd(DAML_YAML)?;
    let daml_yaml = yaml_rust::YamlLoader::load_from_str(&file_contents).ok()?;
    let sdk_version = daml_yaml.first()?[DAML_SDK_VERSION].as_str()?;
    Some(sdk_version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io::{Result, Write};

    #[test]
    fn folder_without_daml_yaml() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("daml").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_daml_yaml() -> Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(DAML_YAML))?.write_all(b"sdk-version: 2.2.0\n")?;
        let actual = ModuleRenderer::new("daml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("Λ v2.2.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_daml_yaml_without_sdk_version_entry() -> Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(DAML_YAML))?.write_all(b"version: 1.0.0\n")?;
        let actual = ModuleRenderer::new("daml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("Λ ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_daml_yaml_and_daml_sdk_version() -> Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(DAML_YAML))?.write_all(b"sdk-version: 2.2.0\n")?;
        let actual = ModuleRenderer::new("daml")
            .env(DAML_SDK_VERSION_ENV, "2.0.0")
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("Λ v2.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_without_daml_yaml_with_daml_sdk_version() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("daml")
            .env(DAML_SDK_VERSION_ENV, "2.0.0")
            .path(dir.path())
            .collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }
}
