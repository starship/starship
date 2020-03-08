use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::php::PhpConfig;
use crate::utils;

/// Creates a module with the current PHP version
///
/// Will display the PHP version if any of the following criteria are met:
///     - Current directory contains a `.php` file
///     - Current directory contains a `composer.json` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_php_project = context
        .try_begin_scan()?
        .set_files(&["composer.json"])
        .set_extensions(&["php"])
        .is_match();

    if !is_php_project {
        return None;
    }

    match utils::exec_cmd(
        "php",
        &[
            "-r",
            "echo PHP_MAJOR_VERSION.'.'.PHP_MINOR_VERSION.'.'.PHP_RELEASE_VERSION;",
        ],
    ) {
        Some(php_cmd_output) => {
            let php_version = php_cmd_output.stdout;

            let mut module = context.new_module("php");
            let config: PhpConfig = PhpConfig::try_load(module.config);

            module.set_style(config.style);

            let formatted_version = format_php_version(&php_version)?;
            module.create_segment("symbol", &config.symbol);
            module.create_segment("version", &SegmentConfig::new(&formatted_version));

            Some(module)
        }
        None => None,
    }
}

fn format_php_version(php_version: &str) -> Option<String> {
    let mut formatted_version = String::with_capacity(php_version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(php_version);
    Some(formatted_version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use tempfile;

    #[test]
    fn test_format_php_version() {
        let input = "7.3.8";
        assert_eq!(format_php_version(input), Some("v7.3.8".to_string()));
    }

    #[test]
    fn folder_without_php_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = render_module("php", dir.path());

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_composer_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("composer.json"))?.sync_all()?;

        let actual = render_module("php", dir.path());

        let expected = Some(format!(
            "via {} ",
            Color::Fixed(147).bold().paint("🐘 v7.3.8")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_php_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.php"))?.sync_all()?;

        let actual = render_module("php", dir.path());

        let expected = Some(format!(
            "via {} ",
            Color::Fixed(147).bold().paint("🐘 v7.3.8")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
