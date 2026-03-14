use super::{Context, Module, ModuleConfig};

use crate::configs::lean::LeanConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Lean version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("lean");
    let config = LeanConfig::try_load(module.config);

    let is_lean_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_lean_project {
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
                "lean_version" => get_lean_version(context).map(Ok),
                "toolchain_version" => get_toolchain(context).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `lean`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn get_lean_version(context: &Context) -> Option<String> {
    Some(
        context
            .exec_cmd("lean", &["--short-version"])?
            .stdout
            .trim()
            .to_string(),
    )
}

fn get_toolchain(context: &Context) -> Option<String> {
    if !has_lean_toolchain_file(context) {
        return None;
    }

    let file_contents = context.read_file_from_pwd("lean-toolchain")?;
    let version = file_contents.split_once(":");
    match version {
        Some(("leanprover/lean4", version)) => Some(version[1..].trim().to_string()),
        Some((_, version)) => Some(format!("custom: {}", version.trim().to_string())),
        None => None,
    }
}

fn get_version(context: &Context) -> Option<String> {
    get_toolchain(context).or_else(|| get_lean_version(context))
}

fn has_lean_toolchain_file(context: &Context) -> bool {
    match context.dir_contents() {
        Ok(dir) => dir.has_file_name("lean-toolchain"),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;
    use std::io::Write;

    #[test]
    fn folder_without_lean_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("lean").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn toolchain() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("lean-toolchain"))?;
        let version = "4.27.0-rc1";
        file.write_all(format!("leanprover/lean4:v{}", version).as_bytes())?;
        file.sync_all()?;
        let actual = ModuleRenderer::new("lean").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.paint(format!(" L∃∀N {version} "))
        ));
        assert_eq!(expected, actual);
        dir.close()?;
        Ok(())
    }

    #[test]
    fn lean_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("lakefile.lean"))?.sync_all()?;
        let version = "4.5.0";
        let actual = ModuleRenderer::new("lean")
            .path(dir.path())
            .config(toml::toml! {
                [lean]
                format = "via [$symbol($lean_version )]($style)"
            })
            .cmd(
                "lean --short-version",
                Some(CommandOutput {
                    stdout: format!("{version}\n"),
                    stderr: String::default(),
                }),
            )
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.paint(format!(" L∃∀N {version} "))
        ));
        assert_eq!(expected, actual);
        dir.close()?;
        Ok(())
    }
}
