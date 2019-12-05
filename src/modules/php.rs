use std::process::Command;

use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::php::PhpConfig;

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

    match get_php_version() {
        Some(php_version) => {
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

fn get_php_version() -> Option<String> {
    match Command::new("php")
        .arg("-r")
        .arg("echo PHP_MAJOR_VERSION.'.'.PHP_MINOR_VERSION.'.'.PHP_RELEASE_VERSION;")
        .output()
    {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
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

    #[test]
    fn test_format_php_version() {
        let input = "7.3.8";
        assert_eq!(format_php_version(input), Some("v7.3.8".to_string()));
    }
}
