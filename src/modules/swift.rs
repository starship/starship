use super::{Context, Module, ModuleConfig};

use crate::configs::swift::SwiftConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils::CommandOutput;

/// Creates a module with the current Swift version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("swift");
    let config: SwiftConfig = SwiftConfig::try_load(module.config);

    let is_swift_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .set_extensions(&config.detect_extensions)
        .is_match();

    if !is_swift_project {
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
                    let swift_version =
                        parse_swift_version(&swift_version_output(context)?.stdout)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &swift_version,
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
            log::warn!("Error in module `swift`:\n{error}");
            return None;
        }
    });

    Some(module)
}

/// Returns the output of the fastest available `swift` version query.
///
/// `swift --version` goes through the Swift driver, which locates the
/// toolchain on every invocation. On macOS that alone can take seconds,
/// tripping the global command timeout so the module disappears (see #7639).
/// `swift-frontend --version` prints a compatible first line but starts much
/// faster, so prefer it and fall back to the driver.
fn swift_version_output(context: &Context) -> Option<CommandOutput> {
    let output = context.exec_cmd("swift-frontend", &["--version"]);
    if output
        .as_ref()
        .is_some_and(|o| parse_swift_version(&o.stdout).is_some())
    {
        return output;
    }
    #[cfg(target_os = "macos")]
    {
        // `swift-frontend` usually isn't on PATH on macOS; locate it inside
        // the active toolchain instead. `xcrun --find` itself is fast.
        if let Some(found) = context.exec_cmd("xcrun", &["--find", "swift-frontend"]) {
            let output = context.exec_cmd(found.stdout.trim(), &["--version"]);
            if output
                .as_ref()
                .is_some_and(|o| parse_swift_version(&o.stdout).is_some())
            {
                return output;
            }
        }
    }
    context.exec_cmd("swift", &["--version"])
}

fn parse_swift_version(swift_version: &str) -> Option<String> {
    // split into ["Apple", "Swift", "version", "5.2.2", ...] or
    //            ["Swift", "version", "5.3-dev", ...]
    let mut split = swift_version.split_whitespace();
    let _ = split.position(|t| t == "version")?;
    // return "5.2.2" or "5.3-dev"
    let version = split.next()?;

    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_swift_version;
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_swift_version() {
        let input = "Apple Swift version 5.2.2";
        assert_eq!(parse_swift_version(input), Some(String::from("5.2.2")));
    }

    #[test]
    fn test_parse_swift_version_without_org_name() {
        let input = "Swift version 5.3-dev (LLVM ..., Swift ...)";
        assert_eq!(parse_swift_version(input), Some(String::from("5.3-dev")));
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

    #[test]
    fn folder_with_swift_file_prefers_swift_frontend() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.swift"))?.sync_all()?;
        let actual = ModuleRenderer::new("swift")
            .cmd(
                "swift-frontend --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "Apple Swift version 6.3.1 (swiftlang-6.3.1.1.2 clang-2100.0.123.102)\nTarget: arm64-apple-macosx26.0\n",
                    ),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("🐦 v6.3.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_swift_file_falls_back_to_swift_driver() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.swift"))?.sync_all()?;
        let actual = ModuleRenderer::new("swift")
            .cmd("swift-frontend --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("🐦 v5.2.2 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn folder_with_swift_file_finds_frontend_via_xcrun() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.swift"))?.sync_all()?;
        let actual = ModuleRenderer::new("swift")
            .cmd("swift-frontend --version", None)
            .cmd(
                "xcrun --find swift-frontend",
                Some(CommandOutput {
                    stdout: String::from("swift-frontend-from-xcrun\n"),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "swift-frontend-from-xcrun --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "Apple Swift version 6.3.1 (swiftlang-6.3.1.1.2 clang-2100.0.123.102)\nTarget: arm64-apple-macosx26.0\n",
                    ),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(202).bold().paint("🐦 v6.3.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
