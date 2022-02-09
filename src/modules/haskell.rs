use super::{Context, Module, RootModuleConfig};

use crate::configs::haskell::HaskellConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Haskell version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("haskell");
    let config = HaskellConfig::try_load(module.config);

    let is_hs_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_hs_project {
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
                "version" => get_version(context).map(Ok),
                "ghc_version" => get_ghc_version(context).map(Ok),
                "snapshot" => get_snapshot(context).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `haskell`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_ghc_version(context: &Context) -> Option<String> {
    Some(
        context
            .exec_cmd("ghc", &["--numeric-version"])?
            .stdout
            .trim()
            .to_string(),
    )
}

fn get_snapshot(context: &Context) -> Option<String> {
    if !is_stack_project(context) {
        return None;
    }
    let file_contents = utils::read_file(context.current_dir.join("stack.yaml")).ok()?;
    let yaml = yaml_rust::YamlLoader::load_from_str(&file_contents).ok()?;
    let version = yaml.first()?["resolver"]
        .as_str()
        .or_else(|| yaml.first()?["snapshot"].as_str())
        .filter(|s| s.starts_with("lts") || s.starts_with("nightly") || s.starts_with("ghc"))
        .unwrap_or("<custom snapshot>");
    Some(version.to_string())
}

fn get_version(context: &Context) -> Option<String> {
    get_snapshot(context).or_else(|| get_ghc_version(context))
}

fn is_stack_project(context: &Context) -> bool {
    match context.dir_contents() {
        Ok(dir) => dir.has_file_name("stack.yaml"),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use std::io::Write;

    #[test]
    fn folder_without_hs_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_stack() -> io::Result<()> {
        let cases = vec![
            ("resolver: lts-18.12\n", "lts-18.12"),
            ("snapshot:\tnightly-2011-11-11", "nightly-2011-11-11"),
            ("snapshot: ghc-8.10.7", "ghc-8.10.7"),
            (
                "snapshot: https://github.com/whatever/xxx.yaml\n",
                "<custom snapshot>",
            ),
            (
                "resolver:\n  url: https://github.com/whatever/xxx.yaml\n",
                "<custom snapshot>",
            ),
        ];
        for (yaml, resolver) in &cases {
            let dir = tempfile::tempdir()?;
            let mut file = File::create(dir.path().join("stack.yaml"))?;
            file.write(yaml.as_bytes())?;
            file.sync_all()?;
            let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
            let expected = Some(format!(
                "via {}",
                Color::Purple.bold().paint(format!(" {} ", resolver))
            ));
            assert_eq!(expected, actual);
            dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn folder_cabal() -> io::Result<()> {
        let should_trigger = vec!["a.hs", "b.hs-boot", "cabal.project"];
        for hs_file in &should_trigger {
            let dir = tempfile::tempdir()?;
            File::create(dir.path().join(hs_file))?.sync_all()?;
            let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
            let expected = Some(format!("via {}", Color::Purple.bold().paint(" 9.2.1 ")));
            assert_eq!(expected, actual);
            dir.close()?;
        }
        Ok(())
    }
}
