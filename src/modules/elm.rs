use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::elm::ElmConfig;
use crate::utils;

/// Creates a module with the current Elm version
///
/// Will display the Elm version if any of the following criteria are met:
///     - The current directory contains a `elm.json` file
///     - The current directory contains a `elm-package.json` file
///     - The current directory contains a `elm-stuff` folder
///     - The current directory contains a `*.elm` files
pub async fn module<'a>(context: &'a Context<'_>) -> Option<Module<'a>> {
    let is_elm_project = context
        .try_begin_scan()?
        .set_files(&["elm.json", "elm-package.json"])
        .set_extensions(&["elm"])
        .set_folders(&["elm-stuff"])
        .is_match();

    if !is_elm_project {
        return None;
    }

    let elm_version = utils::exec_cmd("elm", &["--version"]).await?.stdout;
    let formatted_version = Some(format!("v{}", elm_version.trim()))?;

    let mut module = context.new_module("elm");
    let config: ElmConfig = ElmConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(&formatted_version));

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
    async fn folder_without_elm() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("elm", dir.path()).await;
        let expected = None;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_elm_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("elm.json"))?.sync_all()?;
        let actual = render_module("elm", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_elm_package_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("elm-package.json"))?.sync_all()?;
        let actual = render_module("elm", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_elm_stuff_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let elmstuff = dir.path().join("elm-stuff");
        fs::create_dir_all(&elmstuff)?;
        let actual = render_module("elm", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_elm_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.elm"))?.sync_all()?;
        let actual = render_module("elm", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1")));
        assert_eq!(expected, actual);
        Ok(())
    }
}
