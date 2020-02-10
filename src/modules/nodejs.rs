use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::nodejs::NodejsConfig;
use crate::utils;

/// Creates a module with the current Node.js version
///
/// Will display the Node.js version if any of the following criteria are met:
///     - Current directory contains a `.js` file
///     - Current directory contains a `package.json` file
///     - Current directory contains a `node_modules` directory
pub async fn module<'a>(context: &'a Context<'_>) -> Option<Module<'a>> {
    let is_js_project = context
        .try_begin_scan()?
        .set_files(&["package.json"])
        .set_extensions(&["js"])
        .set_folders(&["node_modules"])
        .is_match();

    if !is_js_project {
        return None;
    }

    let node_version = utils::exec_cmd("node", &["--version"]).await?.stdout;

    let mut module = context.new_module("nodejs");
    let config: NodejsConfig = NodejsConfig::try_load(module.config);

    module.set_style(config.style);

    let formatted_version = node_version.trim();
    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(formatted_version));

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::{self, File};
    use std::io;
    use tempfile;

    #[tokio::test]
    async fn folder_without_node_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("nodejs", dir.path()).await;
        let expected = None;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_package_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_js_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.js"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_node_modules() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let node_modules = dir.path().join("node_modules");
        fs::create_dir_all(&node_modules)?;

        let actual = render_module("nodejs", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        Ok(())
    }
}
