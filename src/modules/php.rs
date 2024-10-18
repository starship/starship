use super::{Context, Module, ModuleConfig};

use crate::configs::php::PhpConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current PHP version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("php");
    let config: PhpConfig = PhpConfig::try_load(module.config);
    let is_php_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .set_extensions(&config.detect_extensions)
        .is_match();

    if !is_php_project {
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
                    let php_version = if config.use_symfony_cli {
                        let php_binary = "symfony";
                        let command_parameters = &[
                            "php",
                            "-nr",
                            "echo PHP_MAJOR_VERSION.\".\".PHP_MINOR_VERSION.\".\".PHP_RELEASE_VERSION;",
                        ];

                        context.exec_cmd(php_binary, command_parameters)?.stdout

                    } else {
                        let php_binary = "php";
                        let command_parameters = &[
                            "-nr",
                            "echo PHP_MAJOR_VERSION.\".\".PHP_MINOR_VERSION.\".\".PHP_RELEASE_VERSION;",
                        ];

                        context.exec_cmd(php_binary, command_parameters)?.stdout
                    };

                    VersionFormatter::format_module_version(module.get_name(), &php_version, config.version_format).map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `php`:\n{}", error);
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

    #[test]
    fn folder_without_php_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("php").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_composer_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("composer.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("php").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(147).bold().paint("🐘 v7.3.8 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_php_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".php-version"))?.sync_all()?;

        let actual = ModuleRenderer::new("php").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(147).bold().paint("🐘 v7.3.8 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_php_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.php"))?.sync_all()?;

        let actual = ModuleRenderer::new("php").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(147).bold().paint("🐘 v7.3.8 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_composer_file_by_using_symfony_cli() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("composer.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("php")
            .config(toml::toml! {
                [php]
                use_symfony_cli = true
            })
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(147).bold().paint("🐘 v8.1.13 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_php_version_by_using_symfony_cli() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".php-version"))?.sync_all()?;

        let actual = ModuleRenderer::new("php")
            .config(toml::toml! {
                [php]
                use_symfony_cli = true
            })
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(147).bold().paint("🐘 v8.1.13 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_php_file_by_using_symfony_cli() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.php"))?.sync_all()?;

        let actual = ModuleRenderer::new("php")
            .config(toml::toml! {
                [php]
                use_symfony_cli = true
            })
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(147).bold().paint("🐘 v8.1.13 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
