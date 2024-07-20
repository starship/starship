use super::{Context, Module, ModuleConfig};

use crate::configs::mojo::MojoConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Mojo version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("mojo");
    let config = MojoConfig::try_load(module.config);

    let is_mojo_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_mojo_project {
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
                    let mojo_version_output = context.exec_cmd("mojo", &["--version"])?.stdout;
                    let version_items = mojo_version_output.split(' ').collect::<Vec<&str>>();

                    if version_items.len() <= 1 {
                        return None;
                    }

                    if version_items.len() == 2 || !config.show_commit {
                        return Some(Ok(version_items[1].trim().to_string()));
                    }

                    let mojo_version = version_items[1].trim();
                    let mojo_hash = version_items[2].trim();

                    Some(Ok(format!("{} {}", mojo_version, mojo_hash)))
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `mojo`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::configs::mojo::MOJO_DEFAULT_COLOR;
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_mojo() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mojo.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("mojo").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mojo_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.mojo"))?.sync_all()?;
        let actual = ModuleRenderer::new("mojo")
            .path(dir.path())
            .collect()
            .unwrap();
        let expected = format!("with {}", MOJO_DEFAULT_COLOR.bold().paint("🔥 24.4.0 "));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mojo_file_emoji() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.🔥"))?.sync_all()?;
        let actual = ModuleRenderer::new("mojo")
            .path(dir.path())
            .collect()
            .unwrap();
        let expected = format!("with {}", MOJO_DEFAULT_COLOR.bold().paint("🔥 24.4.0 "));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mojo_file_with_commit() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.mojo"))?.sync_all()?;
        let actual = ModuleRenderer::new("mojo")
            .config(toml::toml! {
                [mojo]
                show_commit = true
            })
            .path(dir.path())
            .collect()
            .unwrap();
        let expected = format!(
            "with {}",
            MOJO_DEFAULT_COLOR.bold().paint("🔥 24.4.0 (2cb57382) ")
        );
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mojo_file_with_no_commit_available() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.mojo"))?.sync_all()?;
        let actual = ModuleRenderer::new("mojo")
            .config(toml::toml! {
                [mojo]
                show_commit = true
            })
            .cmd(
                "mojo --version",
                Some(CommandOutput {
                    stdout: String::from("mojo 24.4.0\n"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "with {}",
            MOJO_DEFAULT_COLOR.bold().paint("🔥 24.4.0 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
