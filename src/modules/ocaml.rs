use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::ocaml::OCamlConfig;
use crate::utils;

/// Creates a module with the current OCaml version
///
/// Will display the OCaml version if any of the following criteria are met:
///     - Current directory contains a `dune` file
///     - Current directory contains a `dune-project` file
///     - Current directory contains a file with `.ml` extension
///     - Current directory contains a file with `.mli` extension
///     - Current directory contains a file with `.opam` extension
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_ocaml_project = context
        .try_begin_scan()?
        .set_files(&["dune", "dune-project"])
        .set_extensions(&["ml", "mli", "opam"])
        .is_match();

    if !is_ocaml_project {
        return None;
    }

    let ocaml_version = utils::exec_cmd("ocaml", &["--version"])?.stdout;
    let formatted_version = format_ocaml_version(&ocaml_version)?;

    let mut module = context.new_module("ocaml");
    let config = OCamlConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(&formatted_version));

    Some(module)
}

fn format_ocaml_version(ocaml_version: &str) -> Option<String> {
    let version = ocaml_version
        // split into ["The", "OCaml", "toplevel,", "version", "4.10.0"]
        .split_whitespace()
        // return "4.10.0"
        .nth(4)?;

    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::format_ocaml_version;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
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
    fn folder_with_opam_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.opam"))?.sync_all()?;

        let actual = render_module("ocaml", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("🐫 v4.10.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_ocaml_version() {
        assert_eq!(
            format_ocaml_version("The OCaml toplevel, version 4.10.0"),
            Some("v4.10.0".to_string())
        );

        assert_eq!(
            format_ocaml_version("The OCaml toplevel, version 4.08.1"),
            Some("v4.08.1".to_string())
        );
    }
}
