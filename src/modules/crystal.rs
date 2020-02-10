use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::crystal::CrystalConfig;
use crate::utils;

/// Creates a module with the current Crystal version
///
/// Will display the Crystal version if any of the following criteria are met:
///     - Current directory contains a `.cr` file
///     - Current directory contains a `shard.yml` file
pub async fn module<'a>(context: &'a Context<'_>) -> Option<Module<'a>> {
    let is_crystal_project = context
        .try_begin_scan()?
        .set_files(&["shard.yml"])
        .set_extensions(&["cr"])
        .is_match();

    if !is_crystal_project {
        return None;
    }

    let crystal_version = utils::exec_cmd("crystal", &["--version"]).await?.stdout;
    let formatted_version = format_crystal_version(&crystal_version)?;

    let mut module = context.new_module("crystal");
    let config: CrystalConfig = CrystalConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(&formatted_version));

    Some(module)
}

fn format_crystal_version(crystal_version: &str) -> Option<String> {
    let version = crystal_version
        // split into ["Crystal", "0.32.1", ...]
        .split_whitespace()
        // return "0.32.1"
        .nth(1)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}

#[cfg(test)]
mod tests {
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use tempfile;

    #[tokio::test]
    async fn folder_without_crystal_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("crystal", dir.path()).await;
        let expected = None;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_shard_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("shard.yml"))?.sync_all()?;

        let actual = render_module("crystal", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Red.bold().paint("🔮 v0.32.1")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_cr_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.cr"))?.sync_all()?;

        let actual = render_module("crystal", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Red.bold().paint("🔮 v0.32.1")));
        assert_eq!(expected, actual);
        Ok(())
    }
}
