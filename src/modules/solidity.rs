use super::{Context, Module, ModuleConfig};

use crate::configs::solidity::SolidityConfig;
use crate::formatter::{StringFormatter, VersionFormatter};
use crate::utils::get_command_string_output; 

/// Creates a module with the current Solidity version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("solidity");
    let config = SolidityConfig::try_load(module.config);
    let is_sol_project = context
        .try_begin_scan()?
        .set_files(&config.detect_extensions)
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
                    let version = get_solidity_version(context)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                "solcjs_version" => get_solcjs_version(context).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module 'solidity':\n{}", error);
            return None;
        }
    });
    Some(module)
}
fn get_solidity_version(context: &Context) -> Option<String> {
    let command = context.exec_cmd("solc", &["--version"])?;
    let solc_version_string = get_command_string_output(command);

    parse_solc_version(&solc_version_string)
}

fn parse_solc_version(solc_version_string: &str) -> Option<String> {
    let version = solc_version_string
        //output looks like this
        //solc, the solidity compiler commandline interface Version: 0.8.15+commit.e14f2714.Linux.g++
        .split_whitespace()
        //takes the 0.8.15+commit.ei14f2714.Linux.g++
        .nth(7)?;

    Some(version.to_string())
}
fn get_solcjs_version(context: &Context) -> Option<String> {
    
    let solcjs_version_string = parse_solcjs_version(&context.exec_cmd("solcjs", &["--version"])?.stdout)?;
    Some(solcjs_version_string)
}
fn parse_solcjs_version(solcjs_version_string: &str) -> Option<String> {
    let version = solcjs_version_string;
    //Output looks like this solc:0.8.15+commit.e14f2714.Linux.g++
    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_solc_version() {
        let input = "solc, the solidity compiler commandline interface
        Version: 0.8.15+commit.e14f2714.Linux.g++";

        assert_eq!(
            parse_solc_version(input),
            Some(String::from("0.8.15+commit.e14f2714.Linux.g++"))
        );
    }

    #[test]
    fn test_parse_solcjs_version() {
        let input = "0.8.15+commit.e14f2714.Linux.g++";

        assert_eq!(
            parse_solcjs_version(input),
            Some(String::from("0.8.15+commit.e14f2714.Linux.g++"))
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
        File::create(dir.path().join("sol"))?.sync_all()?;
        let actual = ModuleRenderer::new("solidity").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue
                .bold()
                .paint("S 0.8.15+commit.e14f2714.Linux.g++")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
