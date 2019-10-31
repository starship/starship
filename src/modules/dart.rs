use std::process::Command;

use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::dart::DartConfig;

/// Creates a module with the current Dart version
///
/// Will display the Dart version if any of the following criteria are met:
///     - Current directory contains a `.dart` file
///     - Current directory contains a `pubspec.yaml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_dart_project = context
        .try_begin_scan()?
        .set_files(&["pubspec.yaml"])
        .set_extensions(&["dart"])
        .is_match();

    if !is_dart_project {
        return None;
    }

    let formatted_version = format_dart_version(&get_dart_version()?)?;

    let mut module = context.new_module("dart");
    let config: DartConfig = DartConfig::try_load(module.config);

    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &config.version.with_value(&formatted_version));

    Some(module)
}

fn get_dart_version() -> Option<String> {
    match Command::new("dart").arg("--version").output() {
        Ok(output) => {
            if !output.status.success() {
                log::warn!(
                    "Non-Zero exit code '{}' when executing `dart --version`",
                    output.status
                );
                return None;
            }

            Some(String::from_utf8(output.stderr).unwrap())
        }
        Err(_) => None,
    }
}

fn format_dart_version(dart_stdout: &str) -> Option<String> {
    // `dart --version` output will look somthing like this:
    // Dart VM version: 2.5.2 (Tue Apr 20 04:20:00 2019 +0200) on "macos_x64"

    let version = dart_stdout.split(' ').nth(3)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);

    formatted_version.push('v');
    formatted_version.push_str(version);

    Some(formatted_version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_dart_version() {
        let input = "Dart VM version: 2.5.2 (Tue Apr 20 04:20:00 2019 +0200)";
        assert_eq!(format_dart_version(input), Some("v2.5.2".to_string()));
    }
}
