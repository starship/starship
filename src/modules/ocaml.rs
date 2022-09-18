use super::{Context, Module, ModuleConfig};
use once_cell::sync::Lazy;
use std::ops::Deref;
use std::path::Path;

use crate::configs::ocaml::OCamlConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

#[derive(Debug, PartialEq)]
enum SwitchType {
    Global,
    Local,
}
type OpamSwitch = (SwitchType, String);

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

    let opam_switch: Lazy<Option<OpamSwitch>, _> = Lazy::new(|| get_opam_switch(context));

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                "switch_indicator" => {
                    let (switch_type, _) = &opam_switch.deref().as_ref()?;
                    match switch_type {
                        SwitchType::Global => Some(config.global_switch_indicator),
                        SwitchType::Local => Some(config.local_switch_indicator),
                    }
                }
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "switch_name" => {
                    let (_, name) = opam_switch.deref().as_ref()?;
                    Some(Ok(name.to_string()))
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
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        ocaml_version.trim(),
                        config.version_format,
                    )
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
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

fn get_opam_switch(context: &Context) -> Option<OpamSwitch> {
    let opam_switch = context
        .exec_cmd("opam", &["switch", "show", "--safe"])?
        .stdout;

    parse_opam_switch(opam_switch.trim())
}

fn parse_opam_switch(opam_switch: &str) -> Option<OpamSwitch> {
    if opam_switch.is_empty() {
        return None;
    }

    let path = Path::new(opam_switch);
    if !path.has_root() {
        Some((SwitchType::Global, opam_switch.to_string()))
    } else {
        Some((SwitchType::Local, path.file_name()?.to_str()?.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_opam_switch, SwitchType};
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn test_parse_opam_switch() {
        let global_switch = "ocaml-base-compiler.4.10.0";
        let local_switch = "/path/to/my-project";
        assert_eq!(
            parse_opam_switch(global_switch),
            Some((SwitchType::Global, "ocaml-base-compiler.4.10.0".to_string()))
        );
        assert_eq!(
            parse_opam_switch(local_switch),
            Some((SwitchType::Local, "my-project".to_string()))
        );
    }

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
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_opam_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::create_dir_all(dir.path().join("_opam"))?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
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
            r#"{"dependencies": {"ocaml": "4.8.1000"}}"#,
        )?;
        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.08.1 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dune() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dune"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dune_project() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dune-project"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jbuild() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("jbuild"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jbuild_ignore() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("jbuild-ignore"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_merlin_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".merlin"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.ml"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mli_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.mli"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_re_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.re"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_rei_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.rei"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (default) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn without_opam_switch() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.ml"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml")
            .cmd(
                "opam switch show --safe",
                Some(CommandOutput {
                    stdout: String::default(),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🐫 v4.10.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn with_global_opam_switch() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.ml"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml")
            .cmd(
                "opam switch show --safe",
                Some(CommandOutput {
                    stdout: String::from("ocaml-base-compiler.4.10.0\n"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow
                .bold()
                .paint("🐫 v4.10.0 (ocaml-base-compiler.4.10.0) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn with_global_opam_switch_custom_indicator() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.ml"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml")
            .config(toml::toml! {
                [ocaml]
                global_switch_indicator = "g/"
            })
            .cmd(
                "opam switch show --safe",
                Some(CommandOutput {
                    stdout: String::from("ocaml-base-compiler.4.10.0\n"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow
                .bold()
                .paint("🐫 v4.10.0 (g/ocaml-base-compiler.4.10.0) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn with_local_opam_switch() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.ml"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml")
            .cmd(
                "opam switch show --safe",
                Some(CommandOutput {
                    stdout: String::from("/path/to/my-project\n"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (*my-project) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn with_local_opam_switch_custom_indicator() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.ml"))?.sync_all()?;

        let actual = ModuleRenderer::new("ocaml")
            .config(toml::toml! {
                [ocaml]
                local_switch_indicator = "^"
            })
            .cmd(
                "opam switch show --safe",
                Some(CommandOutput {
                    stdout: String::from("/path/to/my-project\n"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐫 v4.10.0 (^my-project) ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
