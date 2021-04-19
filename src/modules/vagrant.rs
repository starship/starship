use super::{Context, Module, RootModuleConfig};

use crate::configs::vagrant::VagrantConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Vagrant version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("vagrant");
    let config = VagrantConfig::try_load(module.config);

    let is_vagrant_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_vagrant_project {
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
                "version" => format_vagrant_version(
                    &context.exec_cmd("vagrant", &["--version"])?.stdout.as_str(),
                )
                .map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `vagrant`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn format_vagrant_version(vagrant_stdout: &str) -> Option<String> {
    // `vagrant --version` output looks like this:
    // Vagrant 2.2.10
    let version = vagrant_stdout
        // split into ["Vagrant","2.2.10"]
        .split_whitespace()
        // return "2.2.10"
        .nth(1)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_vagrant_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("vagrant").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_vagrant_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Vagrantfile"))?.sync_all()?;

        let actual = ModuleRenderer::new("vagrant").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Cyan.bold().paint("⍱ v2.2.10 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_vagrant_version() {
        let vagrant = "Vagrant 2.2.10\n";
        assert_eq!(format_vagrant_version(vagrant), Some("v2.2.10".to_string()));
    }
}
