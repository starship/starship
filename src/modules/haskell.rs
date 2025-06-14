use super::{Context, Module, ModuleConfig};

use crate::configs::haskell::HaskellConfig;
use crate::formatter::StringFormatter;
use crate::utils::read_file;
use std::path::PathBuf;

struct HaskellResult {
    compiler_name: Option<String>,
    compiler_version: Option<String>,
    snapshot: Option<String>,
}

/// Creates a module with the current Haskell version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("haskell");
    let config = HaskellConfig::try_load(module.config);

    let haskell_result: HaskellResult = stack_result(context, &config)
        .or(cabal_result(context, &config))
        .or(plain_haskell_result(context, &config))?;

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
                "compiler_name" => haskell_result.compiler_name.clone().map(Ok),
                "compiler_id" => get_compiler_id(&haskell_result).map(Ok),
                "compiler_version" => haskell_result.compiler_version.clone().map(Ok),
                "ghc_version" => haskell_result.compiler_version.clone().map(Ok), // TODO: The `ghc_version` field should be removed in favour of `compiler_version`.
                "snapshot" => haskell_result.snapshot.clone().map(Ok),
                "version" => get_version(&haskell_result).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `haskell`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn get_compiler_id(haskell_result: &HaskellResult) -> Option<String> {
    let compiler_name = haskell_result.compiler_name.clone()?;
    let compiler_version = haskell_result.compiler_version.clone()?;
    Some(format!("{compiler_name}-{compiler_version}"))
}

fn get_version(haskell_result: &HaskellResult) -> Option<String> {
    haskell_result
        .snapshot
        .clone()
        .or(get_compiler_id(haskell_result))
}

/// Try to get a Stack project setup
fn stack_result<'a>(context: &'a Context, config: &HaskellConfig) -> Option<HaskellResult> {
    let is_project_in_current_dir: bool = context
        .try_begin_scan()?
        .set_files(&config.stack_files)
        .set_extensions(&config.stack_extensions)
        .set_folders(&config.stack_folders)
        .is_match();

    let maybe_project_root: Option<PathBuf> = if is_project_in_current_dir {
        Some(context.current_dir.clone())
    } else {
        context
            .begin_ancestor_scan()
            .set_files(&config.stack_files)
            .set_folders(&config.stack_folders)
            .scan()
    };

    maybe_project_root.map(|project_root| {
        log::trace!("Found Stack project at {:?}", project_root);
        let compiler = get_stack_compiler(context);
        HaskellResult {
            compiler_name: compiler.clone().map(|pair| pair.0),
            compiler_version: compiler.clone().map(|pair| pair.1),
            snapshot: get_stack_snapshot(context, project_root),
        }
    })
}

fn get_stack_compiler(context: &Context) -> Option<(String, String)> {
    let compiler_version = context
        .exec_cmd(
            "stack",
            &["--no-install-ghc", "query", "compiler", "wanted"],
        )?
        .stdout
        .trim()
        .strip_prefix("ghc-")?
        .to_string();
    Some(("ghc".to_string(), compiler_version))
}

fn get_stack_snapshot(context: &Context, project_root: PathBuf) -> Option<String> {
    let stack_yaml_name: String = context
        .get_env("STACK_YAML")
        .unwrap_or("stack.yaml".to_string());
    let stack_yaml_path: PathBuf = project_root.join(stack_yaml_name);
    let file_contents = read_file(stack_yaml_path).ok()?;
    let yaml = yaml_rust2::YamlLoader::load_from_str(&file_contents).ok()?;
    let version = yaml.first()?["resolver"]
        .as_str()
        .or_else(|| yaml.first()?["snapshot"].as_str())
        .filter(|s| s.starts_with("lts") || s.starts_with("nightly") || s.starts_with("ghc"))
        .unwrap_or("<custom snapshot>");
    Some(version.to_string())
}

/// Try to get a Cabal project setup
fn cabal_result<'a>(context: &'a Context, config: &HaskellConfig) -> Option<HaskellResult> {
    let is_project_in_current_dir: bool = context
        .try_begin_scan()?
        .set_files(&config.cabal_files)
        .set_extensions(&config.cabal_extensions)
        .set_folders(&config.cabal_folders)
        .is_match();

    let maybe_project_root: Option<PathBuf> = if is_project_in_current_dir {
        Some(context.current_dir.clone())
    } else {
        context
            .begin_ancestor_scan()
            .set_files(&config.cabal_files)
            .set_folders(&config.cabal_folders)
            .scan()
    };

    maybe_project_root.map(|project_root| {
        log::trace!("Found Cabal project at {:?}", project_root);
        let compiler = get_cabal_compiler(context);
        HaskellResult {
            compiler_name: compiler.clone().map(|pair| pair.0),
            compiler_version: compiler.clone().map(|pair| pair.1),
            snapshot: None,
        }
    })
}

fn get_cabal_compiler(context: &Context) -> Option<(String, String)> {
    let output: String = context
        .exec_cmd(
            "cabal",
            &["-v0", "path", "--format=json", "--compiler-info"],
        )?
        .stdout;
    let parsed_json: serde_json::Value = serde_json::from_str(output.as_str()).ok()?;
    let compiler = parsed_json.get("compiler")?;
    let compiler_name = compiler.get("flavour")?.as_str()?;
    let compiler_version = compiler
        .get("id")?
        .as_str()?
        .strip_prefix(format!("{compiler_name}-").as_str())?;
    log::trace!("Cabal output 4: {:?}", compiler_version);
    Some((compiler_name.to_string(), compiler_version.to_string()))
}

/// Neither a Cabal nor a Stack project; But maybe there are some Haskell files in the current
/// directory.
fn plain_haskell_result<'a>(context: &'a Context, config: &HaskellConfig) -> Option<HaskellResult> {
    let is_haskell_directory: bool = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_haskell_directory {
        return None;
    }

    log::trace!("Found Haskell files in current directory");
    let compiler = get_plain_compiler(context, config);
    Some(HaskellResult {
        compiler_name: compiler.clone().map(|pair| pair.0),
        compiler_version: compiler.clone().map(|pair| pair.1),
        snapshot: None,
    })
}

fn get_plain_compiler(context: &Context, config: &HaskellConfig) -> Option<(String, String)> {
    let compiler_name = config.default_compiler.to_string();
    let compiler_version = context
        .exec_cmd(config.default_compiler, &["--numeric-version"])?
        .stdout
        .trim()
        .to_string();
    Some((compiler_name, compiler_version))
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::fs::create_dir;
    use std::io;
    use std::io::Write;

    #[test]
    fn folder_without_haskell_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_stack() -> io::Result<()> {
        let cases = vec![
            ("resolver: lts-18.12", "lts-18.12"),
            ("snapshot: nightly-2011-11-11", "nightly-2011-11-11"),
            ("snapshot: ghc-8.10.7", "ghc-8.10.7"),
            (
                "snapshot: https://example.com/whatever/xxx.yaml",
                "<custom snapshot>",
            ),
            (
                "resolver:\n  url: https://example.com/whatever/xxx.yaml",
                "<custom snapshot>",
            ),
        ];
        let other_hs_files = vec!["M.hs", "M.hs-boot", "cabal.project"];
        for (yaml, resolver) in &cases {
            let dir = tempfile::tempdir()?;

            let mut file = File::create(dir.path().join("stack.yaml"))?;
            file.write_all(yaml.as_bytes())?;
            file.sync_all()?;

            for hs_file in &other_hs_files {
                File::create(dir.path().join(hs_file))?.sync_all()?;
            }

            let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
            let expected = Some(format!(
                "via {}",
                Color::Purple.bold().paint(format!("λ {resolver} "))
            ));
            assert_eq!(expected, actual);

            dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn folder_stack_nested() -> io::Result<()> {
        let other_hs_files = vec!["M.hs", "M.hs-boot", "cabal.project"];
        let dir = tempfile::tempdir()?;
        let nested = dir.path().join("nested");

        let mut file = File::create(dir.path().join("stack.yaml"))?;
        file.write_all("resolver: lts-18.12".as_bytes())?;
        file.sync_all()?;

        create_dir(&nested)?;
        for hs_file in &other_hs_files {
            File::create(nested.as_path().join(hs_file))?.sync_all()?;
        }

        let actual = ModuleRenderer::new("haskell")
            .path(nested.as_path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint(format!("λ lts-18.12 "))
        ));
        assert_eq!(expected, actual);

        dir.close()?;
        Ok(())
    }

    #[test]
    fn folder_dotcabal() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        File::create(dir.path().join("name.cabal"))?.sync_all()?;

        let other_hs_files = vec!["M.hs", "M.hs-boot"];
        for hs_file in &other_hs_files {
            File::create(dir.path().join(hs_file))?.sync_all()?;
        }

        let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("λ ghc-9.2.1 ")
        ));
        assert_eq!(expected, actual);

        dir.close()?;
        Ok(())
    }

    #[test]
    fn folder_cabalproject() -> io::Result<()> {
        // We mock the `cabal` command while testing, so there is really no point to run multiple
        // tests with different cabal.project content.
        let cases = vec![
            "with-compiler: ghc",
            // "with-compiler: ghc-9.2.1",
        ];
        let other_hs_files = vec!["M.hs", "M.hs-boot"];
        for content in &cases {
            let dir = tempfile::tempdir()?;

            let mut file = File::create(dir.path().join("cabal.project"))?;
            file.write_all(content.as_bytes())?;
            file.sync_all()?;

            for hs_file in &other_hs_files {
                File::create(dir.path().join(hs_file))?.sync_all()?;
            }

            let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
            let expected = Some(format!(
                "via {}",
                Color::Purple.bold().paint("λ ghc-9.2.1 ")
            ));
            assert_eq!(expected, actual);

            dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn folder_cabalproject_nested() -> io::Result<()> {
        let other_hs_files = vec!["M.hs", "M.hs-boot"];
        let dir = tempfile::tempdir()?;
        let nested = dir.path().join("nested");

        let mut file = File::create(dir.path().join("cabal.project"))?;
        file.write_all("with-compiler: ghc".as_bytes())?;
        file.sync_all()?;

        create_dir(&nested)?;
        for hs_file in &other_hs_files {
            File::create(nested.as_path().join(hs_file))?.sync_all()?;
        }

        let actual = ModuleRenderer::new("haskell")
            .path(nested.as_path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint(format!("λ ghc-9.2.1 "))
        ));
        assert_eq!(expected, actual);

        dir.close()?;
        Ok(())
    }

    #[test]
    fn folder_plain() -> io::Result<()> {
        let should_trigger = vec!["M.hs", "M.hs-boot"];
        for hs_file in &should_trigger {
            let dir = tempfile::tempdir()?;

            File::create(dir.path().join(hs_file))?.sync_all()?;

            let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
            let expected = Some(format!(
                "via {}",
                Color::Purple.bold().paint("λ ghc-9.2.1 ")
            ));
            assert_eq!(expected, actual);

            dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn non_default_compiler() -> io::Result<()> {
        let should_trigger = vec!["M.hs", "M.hs-boot"];
        for hs_file in &should_trigger {
            let dir = tempfile::tempdir()?;

            File::create(dir.path().join(hs_file))?.sync_all()?;

            let renderer = ModuleRenderer::new("haskell")
                .path(dir.path())
                .config(toml::toml! {
                    [haskell]
                    default_compiler = "mhs"
                });
            let actual = renderer.collect();
            let expected = Some(format!(
                "via {}",
                Color::Purple.bold().paint("λ mhs-0.13.3.0 ")
            ));
            assert_eq!(expected, actual);

            dir.close()?;
        }
        Ok(())
    }
}
