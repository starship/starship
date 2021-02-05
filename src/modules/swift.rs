use super::{Context, Module, RootModuleConfig};

use crate::configs::swift::SwiftConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Swift version
///
/// Will display the Swift version if any of the following criteria are met:
///     - The current directory contains a `Package.swift` file
///     - The current directory contains a file with extension `.swift`
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_swift_project = context
        .try_begin_scan()?
        .set_extensions(&["swift"])
        .is_match();

    if !is_swift_project {
        return None;
    }

    let mut module = context.new_module("swift");
    let config: SwiftConfig = SwiftConfig::try_load(module.config);

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
                    let swift_version = utils::exec_cmd("swift", &["--version"])?.stdout;
                    parse_swift_version(&swift_version).map(Ok)
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `swift`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_swift_version(swift_version: &str) -> Option<String> {
    // split into ["Apple", "Swift", "version", "5.2.2", ...] or
    //            ["Swift", "version", "5.3-dev", ...]
    let mut splited = swift_version.split_whitespace();
    let _ = splited.position(|t| t == "version")?;
    // return "5.2.2" or "5.3-dev"
    let version = splited.next()?;

    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::parse_swift_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_swift_version() {
        let input = "Apple Swift version 5.2.2";
        assert_eq!(parse_swift_version(input), Some(String::from("v5.2.2")));
    }

    #[test]
    fn test_parse_swift_version_without_org_name() {
        let input = "Swift version 5.3-dev (LLVM ..., Swift ...)";
        assert_eq!(parse_swift_version(input), Some(String::from("v5.3-dev")));
    }

    #[test]
    fn folder_without_swift_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("swift.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("swift").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Package.swift"))?.sync_all()?;
        let actual = ModuleRenderer::new("swift").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("🐦 v5.2.2 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_swift_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.swift"))?.sync_all()?;
        let actual = ModuleRenderer::new("swift").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("🐦 v5.2.2 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
