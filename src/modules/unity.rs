use super::{Context, Module, ModuleConfig};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaml_rust2::YamlLoader;

use crate::configs::unity::UnityConfig;
use crate::formatter::string_formatter::StringFormatterError;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Unity version and platform
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("unity");
    let config = UnityConfig::try_load(module.config);

    let is_unity_project = context
        .dir_contents()
        .map(|c| c.has_folder("Assets") && c.has_folder("ProjectSettings"))
        .ok()?;

    let version_file_path = Path::new(&context.logical_dir)
        .join("ProjectSettings")
        .join("ProjectVersion.txt");

    if !is_unity_project || !&version_file_path.exists() {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let mut file = File::open(&version_file_path).ok()?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).ok()?;

                    let yaml = YamlLoader::load_from_str(&contents).ok()?;
                    let yaml = &yaml[0]["m_EditorVersion"];

                    if let Some(m_editor_version) = yaml.as_str() {
                        Some(Ok(VersionFormatter::format_module_version(
                            module.get_name(),
                            m_editor_version.trim(),
                            config.version_format,
                        )
                        .unwrap()))
                    } else {
                        Some(Err(StringFormatterError::Custom(String::from(
                            "Could not find Unity version.",
                        ))))
                    }
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `unity`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;
    use std::io::Write;

    #[test]
    fn folder_without_unity() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("unity.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("unity").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_unity() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        std::fs::create_dir(dir.path().join("Assets"))?;
        let ps_path = dir.path().join("ProjectSettings");
        std::fs::create_dir(&ps_path)?;

        let mut pv_file = File::create(ps_path.join("ProjectVersion.txt"))?;
        pv_file.write_all(b"m_EditorVersion: 2022.3.30f1\n")?;
        pv_file.write_all(b"m_EditorVersionWithRevision: 2022.3.30f1 (70558241b701)")?;
        pv_file.sync_all()?;

        let actual = ModuleRenderer::new("unity").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint("🎮 v2022.3.30f1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_without_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        std::fs::create_dir(dir.path().join("Assets"))?;
        let ps_path = dir.path().join("ProjectSettings");
        std::fs::create_dir(ps_path)?;

        let actual = ModuleRenderer::new("unity").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }
}
