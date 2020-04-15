use super::{Context, Module, RootModuleConfig};

use crate::configs::haskell::HaskellConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Haskell Stack version
///
/// Will display the Haskell version if any of the following criteria are met:
///     - Current directory contains a `stack.yaml` file
///     - Current directory contains a `.cabal` file
///     - Current directory contains a `package.yaml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_haskell_project = context
        .try_begin_scan()?
        .set_files(&["package.yaml", "stack.yaml", "package.yml", "stack.yml"])
        .set_extensions(&["cabal"])
        .is_match();

    if !is_haskell_project {
        return None;
    }

    let mut module = context.new_module("haskell");
    let config: HaskellConfig = HaskellConfig::try_load(module.config);

    let haskell_version = utils::exec_cmd(
        "stack",
        &["ghc", "--no-install-ghc", "--", "--numeric-version"],
    )?
    .stdout;
    let formatted_version = Some(format!("v{}", haskell_version.trim()))?;

    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter.map(|variable| match variable {
            "version" => Some(formatted_version.clone()),
            _ => None,
        })
    } else {
        log::warn!("Error parsing format string in `haskell.format`");
        return None;
    };
    module.set_segments(formatter.parse(None));
    module.get_prefix().set_value("");
    module.get_suffix().set_value("");
    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use tempfile;

    #[test]
    fn folder_without_stack_yaml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("haskell", dir.path());
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_hpack_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.yaml"))?.sync_all()?;
        let actual = render_module("haskell", dir.path());
        let expected = Some(format!("via {} ", Color::Red.bold().paint("λ v8.6.5")));
        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_cabal_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("test.cabal"))?.sync_all()?;
        let actual = render_module("haskell", dir.path());
        let expected = Some(format!("via {} ", Color::Red.bold().paint("λ v8.6.5")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_stack_yaml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("stack.yaml"))?.sync_all()?;
        let actual = render_module("haskell", dir.path());
        let expected = Some(format!("via {} ", Color::Red.bold().paint("λ v8.6.5")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
