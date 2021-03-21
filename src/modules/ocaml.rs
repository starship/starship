use super::{Context, Module, RootModuleConfig};
use std::path::Path;

use crate::configs::ocaml::OCamlConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current OCaml version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("ocaml");
    let config: OCamlConfig = OCamlConfig::try_load(module.config);
    let is_ocaml_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .set_extensions(&config.detect_extensions)
        .is_match();

    if !is_ocaml_project {
        return None;
    }

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
                "switch" => {
                    let stdout = context
                        .exec_cmd("opam", &["switch", "-s", "--safe", "show"])?
                        .stdout;
                    let switch_raw = stdout.trim();
                    Some(Ok(Path::new(switch_raw).file_name()?.to_str()?.to_owned()))
                }
                "version" => {
                    let is_esy_project = context
                        .try_begin_scan()?
                        .set_folders(&["esy.lock"])
                        .is_match();

                    let ocaml_version = if is_esy_project {
                        context.exec_cmd("esy", &["ocaml", "-vnum"])?.stdout
                    } else {
                        context.exec_cmd("ocaml", &["-vnum"])?.stdout
                    };
                    Some(Ok(format!("v{}", &ocaml_version.trim())))
                }
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
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_opam_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join("_opam"))?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
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
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.08.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dune() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dune"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dune_project() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dune-project"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jbuild() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("jbuild"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jbuild_ignore() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("jbuild-ignore"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_merlin_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".merlin"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.ml"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mli_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.mli"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_re_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.re"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_rei_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.rei"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ml_file_and_uses_switch_variable() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.ml"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml")
            .path(dir.path())
            .config(toml::toml! {
               [ocaml]
               format = "$switch"
            })
            .collect();
        let expected = Some("my-test.switch".to_string());
        assert_eq!(expected, actual);
        dir.close()
    }
}
