use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::ocaml::OCamlConfig;
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

    let ocaml_version = utils::exec_cmd("ocaml", &["-vnum"])?.stdout;
    let formatted_version = format!("v{}", &ocaml_version);

    let mut module = context.new_module("ocaml");
    let config = OCamlConfig::try_load(module.config);
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

    #[test]
    fn folder_without_ocaml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("ocaml", dir.path(), None);
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_opam_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.opam"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_opam_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join("_opam"))?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_esy_lock_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join("esy.lock"))?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dune() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dune"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dune_project() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dune-project"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jbuild() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("jbuild"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jbuild_ignore() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("jbuild-ignore"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_merlin_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".merlin"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.ml"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mli_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.mli"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_re_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.re"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_rei_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.rei"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
