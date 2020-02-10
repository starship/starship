use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::haskell::HaskellConfig;
use crate::utils;

/// Creates a module with the current Haskell Stack version
///
/// Will display the Haskell version if any of the following criteria are met:
///     - Current directory contains a `stack.yaml` file
///     - Current directory contains a `.cabal` file
///     - Current directory contains a `package.yaml` file
pub async fn module<'a>(context: &'a Context<'_>) -> Option<Module<'a>> {
    let is_haskell_project = context
        .try_begin_scan()?
        .set_files(&["package.yaml", "stack.yaml", "package.yml", "stack.yml"])
        .set_extensions(&["cabal"])
        .is_match();

    if !is_haskell_project {
        return None;
    }

    let haskell_version = utils::exec_cmd(
        "stack",
        &["ghc", "--", "--numeric-version", "--no-install-ghc"],
    )
    .await?
    .stdout;
    let formatted_version = Some(format!("v{}", haskell_version.trim()))?;

    let mut module = context.new_module("haskell");
    let config: HaskellConfig = HaskellConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(&formatted_version));

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use tempfile;

    #[tokio::test]
    async fn folder_without_stack_yaml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("haskell", dir.path()).await;
        let expected = None;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_hpack_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.yaml"))?.sync_all()?;
        let actual = render_module("haskell", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Red.bold().paint("λ v8.6.5")));
        assert_eq!(expected, actual);
        Ok(())
    }
    #[tokio::test]
    async fn folder_with_cabal_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("tokio::test.cabal"))?.sync_all()?;
        let actual = render_module("haskell", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Red.bold().paint("λ v8.6.5")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[tokio::test]
    async fn folder_with_stack_yaml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("stack.yaml"))?.sync_all()?;
        let actual = render_module("haskell", dir.path()).await;
        let expected = Some(format!("via {} ", Color::Red.bold().paint("λ v8.6.5")));
        assert_eq!(expected, actual);
        Ok(())
    }
}
