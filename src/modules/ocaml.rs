use super::{Context, Module, RootModuleConfig};

use crate::configs::ocaml::OCamlConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current OCaml version
///
/// Will display the OCaml version if any of the following criteria are met:
///     - Current directory contains a file with `.opam` extension or `_opam` directory
///     - Current directory contains a `esy.lock` directory
///     - Current directory contains a `dune` or `dune-project` file
///     - Current directory contains a `jbuild` or `jbuild-ignore` file
///     - Current directory contains a `.merlin` file
///     - Current directory contains a file with `.ml`, `.mli`, `.re` or `.rei` extension
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_ocaml_project = context
        .try_begin_scan()?
        .set_files(&["dune", "dune-project", "jbuild", "jbuild-ignore", ".merlin"])
        .set_folders(&["_opam", "esy.lock"])
        .set_extensions(&["opam", "ml", "mli", "re", "rei"])
        .is_match();

    if !is_ocaml_project {
        return None;
    }

    let is_esy_project = context
        .try_begin_scan()?
        .set_folders(&["esy.lock"])
        .is_match();

    let ocaml_version = if is_esy_project {
        utils::exec_cmd("esy", &["ocaml", "-vnum"])?.stdout
    } else {
        utils::exec_cmd("ocaml", &["-vnum"])?.stdout
    };

    let mut module = context.new_module("ocaml");
    let config: OCamlConfig = OCamlConfig::try_load(module.config);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => Some(Ok(format!("v{}", &ocaml_version.trim()))),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `ocaml`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn folder_without_ocaml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_opam_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.opam"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_opam_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join("_opam"))?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_esy_lock_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join("esy.lock"))?;
        File::create(dir.path().join("package.json"))?.sync_all()?;
        fs::write(
            dir.path().join("package.lock"),
            "{\"dependencies\": {\"ocaml\": \"4.8.1000\"}}",
        )?;
        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.08.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dune() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dune"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dune_project() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dune-project"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jbuild() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("jbuild"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jbuild_ignore() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("jbuild-ignore"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_merlin_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".merlin"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.ml"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mli_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.mli"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_re_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.re"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_rei_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.rei"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
