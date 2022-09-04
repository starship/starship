use super::{Context, Module, ModuleConfig};

use crate::configs::terraform::TerraformConfig;
use crate::formatter::StringFormatter;
use crate::utils;

use crate::formatter::VersionFormatter;
use std::io;
use std::path::PathBuf;

/// Creates a module with the current Terraform version and workspace
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("terraform");
    let config: TerraformConfig = TerraformConfig::try_load(module.config);

    let is_terraform_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .set_extensions(&config.detect_extensions)
        .is_match();

    if !is_terraform_project {
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
                "version" => {
                    let terraform_version = parse_terraform_version(
                        context.exec_cmd("terraform", &["version"])?.stdout.as_str(),
                    )?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &terraform_version,
                        config.version_format,
                    )
                }
                .map(Ok),
                "workspace" => get_terraform_workspace(context).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `terraform`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

// Determines the currently selected workspace (see https://github.com/hashicorp/terraform/blob/master/command/meta.go for the original implementation)
fn get_terraform_workspace(context: &Context) -> Option<String> {
    // Workspace can be explicitly overwritten by an env var
    let workspace_override = context.get_env("TF_WORKSPACE");
    if workspace_override.is_some() {
        return workspace_override;
    }

    // Data directory containing current workspace can be overwritten by an env var
    let datadir = match context.get_env("TF_DATA_DIR") {
        Some(s) => PathBuf::from(s),
        None => context.current_dir.join(".terraform"),
    };
    match utils::read_file(datadir.join("environment")) {
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Some("default".to_string()),
        Ok(s) => Some(s),
        _ => None,
    }
}

fn parse_terraform_version(version: &str) -> Option<String> {
    // `terraform version` output looks like this
    // Terraform v0.12.14
    // With potential extra output if it detects you are not running the latest version
    let version = version
        .lines()
        .next()?
        .trim_start_matches("Terraform ")
        .trim()
        .trim_start_matches('v');

    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io::{self, Write};

    #[test]
    fn test_parse_terraform_version_release() {
        let input = "Terraform v0.12.14";
        assert_eq!(parse_terraform_version(input), Some("0.12.14".to_string()));
    }

    #[test]
    fn test_parse_terraform_version_prerelease() {
        let input = "Terraform v0.12.14-rc1";
        assert_eq!(
            parse_terraform_version(input),
            Some("0.12.14-rc1".to_string())
        );
    }

    #[test]
    fn test_parse_terraform_version_development() {
        let input = "Terraform v0.12.14-dev (cca89f74)";
        assert_eq!(
            parse_terraform_version(input),
            Some("0.12.14-dev (cca89f74)".to_string())
        );
    }

    #[test]
    fn test_parse_terraform_version_multiline() {
        let input = "Terraform v0.12.13

Your version of Terraform is out of date! The latest version
is 0.12.14. You can update by downloading from www.terraform.io/downloads.html

";
        assert_eq!(parse_terraform_version(input), Some("0.12.13".to_string()));
    }

    #[test]
    fn folder_with_dotterraform_with_version_no_environment() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let tf_dir = dir.path().join(".terraform");
        fs::create_dir(&tf_dir)?;

        let actual = ModuleRenderer::new("terraform")
            .path(dir.path())
            .config(toml::toml! {
                [terraform]
                format = "via [$symbol$version $workspace]($style) "
            })
            .collect();

        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 v0.12.14 default")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dotterraform_with_version_with_environment() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let tf_dir = dir.path().join(".terraform");
        fs::create_dir(&tf_dir)?;
        let mut file = File::create(tf_dir.join("environment"))?;
        file.write_all(b"development")?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("terraform")
            .path(dir.path())
            .config(toml::toml! {
                [terraform]
                format = "via [$symbol$version $workspace]($style) "
            })
            .collect();

        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 v0.12.14 development")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_without_dotterraform() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("terraform").path(dir.path()).collect();
        let expected = None;

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_tf_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.tf"))?;

        let actual = ModuleRenderer::new("terraform").path(dir.path()).collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 default")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_workspace_override() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.tf"))?;

        let actual = ModuleRenderer::new("terraform")
            .path(dir.path())
            .env("TF_WORKSPACE", "development")
            .collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 development")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_datadir_override() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.tf"))?;

        let datadir = tempfile::tempdir()?;
        let mut file = File::create(datadir.path().join("environment"))?;
        file.write_all(b"development")?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("terraform")
            .path(dir.path())
            .env("TF_DATA_DIR", datadir.path().to_str().unwrap())
            .collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 development")
        ));

        assert_eq!(expected, actual);
        dir.close()?;
        datadir.close()
    }

    #[test]
    fn folder_with_dotterraform_no_environment() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let tf_dir = dir.path().join(".terraform");
        fs::create_dir(&tf_dir)?;

        let actual = ModuleRenderer::new("terraform").path(dir.path()).collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 default")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dotterraform_with_environment() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let tf_dir = dir.path().join(".terraform");
        fs::create_dir(&tf_dir)?;
        let mut file = File::create(tf_dir.join("environment"))?;
        file.write_all(b"development")?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("terraform").path(dir.path()).collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 development")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
}
