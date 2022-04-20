use super::{Context, Module, ModuleConfig};

use crate::configs::solidity::SolidityConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

/// Creates a module with the current Solidity version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("solidity");
    let config = SolidityConfig::try_load(module.config);

    let is_sol_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_sol_project {
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
                    let version = get_module_version(context, &config)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &version,
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
            log::warn!("Error in module `solidity`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_module_version(context: &Context, _config: &SolidityConfig) -> Option<String> {
    let version = parse_solc_version(&context.exec_cmd("solc", &["--version"])?.stdout)?;
    Some(version)
}

fn parse_solc_version(version_output: &str) -> Option<String> {
    let mut iterator = version_output.split_whitespace();
    let _ = iterator.position(|t| t == "Version:")?;
    let version = iterator.next()?;
    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::parse_solc_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_solc_version() {
        let input = "solc, the solidity compiler commandline interface
        Version: 0.8.13+commit.abaa5c0e.Linux.g++";
        assert_eq!(
            parse_solc_version(input),
            Some(String::from("0.8.13+commit.abaa5c0e.Linux.g++"))
        );
    }

    #[test]
    fn folder_without_solidity_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("solidity.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("solidity").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_solidity_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.sol"))?.sync_all()?;
        let actual = ModuleRenderer::new("solidity").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue
                .bold()
                .paint("S v0.8.12+commit.abaa5c0e.Linux.g++ ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
