use super::{Context, Module, ModuleConfig};

use crate::configs::terraform::{TerraformConfig, TFENV_VERSION_FILE};
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils;

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
                "version" => get_cli_version(&module, context, &config).map(Ok),
                "tfenvversion" => get_tfenv_version(context, &module, &config).map(Ok),
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

fn get_cli_version(module: &Module, ctx: &Context, cfg: &TerraformConfig) -> Option<String> {
    let version = parse_terraform_version(&ctx.exec_cmd("terraform", &["version"])?.stdout)?;
    format(module, &version, cfg)
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

fn format(module: &Module, version: &str, cfg: &TerraformConfig) -> Option<String> {
    VersionFormatter::format_module_version(module.get_name(), version, cfg.version_format)
}

fn get_tfenv_version(ctx: &Context, module: &Module, cfg: &TerraformConfig) -> Option<String> {
    ctx.get_env("TFENV_TERRAFORM_VERSION")
        .or_else(|| try_tfenv_file(ctx))
        .and_then(parse_tfenv_version)
        .and_then(|version| format(module, &version, cfg))
}

fn try_tfenv_file(ctx: &Context) -> Option<String> {
    let file_path = ctx.current_dir.join(TFENV_VERSION_FILE);
    if !file_path.exists() || !file_path.is_file() {
        return None;
    }
    utils::read_file(file_path).ok()
}

fn parse_tfenv_version(version: String) -> Option<String> {
    // Tfenv versions are simply exact versions starship expects, like 0.6.16 or 1.0.8
    // so the only thing to be done is to remove newline.
    // It is possible to specify version like `latest:^0.8` and it's left as is,
    // because it's not possible to determine the exact version without calling
    // `terraform` binary.
    let version = version.lines().next()?.trim();
    if version.is_empty() {
        log::warn!("The terraform version is blank");
        None
    } else {
        Some(format!("{} ", version))
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::{self, create_dir_all, File};
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

    #[test]
    fn test_render_when_version_file_present() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let version_filepath = dir.path().join(".terraform-version");
        let mut file = File::create(version_filepath)?;
        file.write_all(b"0.6.16")?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("terraform").path(dir.path()).collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 v0.6.16 default")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_render_when_version_in_file_is_regex() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let version_filepath = dir.path().join(".terraform-version");
        let mut file = File::create(version_filepath)?;
        file.write_all(b"latest:^0.8")?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("terraform").path(dir.path()).collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 vlatest:^0.8 default")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_render_when_version_in_file_is_blank() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let version_filepath = dir.path().join(".terraform-version");
        let mut file = File::create(version_filepath)?;
        file.write_all(b"    ")?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("terraform").path(dir.path()).collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 default")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_render_when_file_not_existing() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let version_filepath = dir.path().join(".tf");
        let _file = File::create(version_filepath)?;

        let actual = ModuleRenderer::new("terraform").path(dir.path()).collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 default")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_render_but_there_is_directory_instead_of_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let version_filepath = dir.path().join(".terraform-version");
        create_dir_all(version_filepath)?;

        let actual = ModuleRenderer::new("terraform").path(dir.path()).collect();
        let expected = None;

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_render_when_file_present_but_overridden_by_env() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let version_filepath = dir.path().join(".terraform-version");
        let mut file = File::create(version_filepath)?;
        file.write_all(b"0.6.16")?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("terraform")
            .env("TFENV_TERRAFORM_VERSION", "0.12.11")
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 v0.12.11 default")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_render_with_only_env_set() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let version_filepath = dir.path().join(".tf");
        let _file = File::create(version_filepath)?;

        let actual = ModuleRenderer::new("terraform")
            .env("TFENV_TERRAFORM_VERSION", "0.12.10")
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {} ",
            Color::Fixed(105).bold().paint("💠 v0.12.10 default")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_parse_tfenv_version() {
        let test_cases = vec![
            ("1.0.8", Some("1.0.8 ".to_string())),
            (" ", None),
            ("", None),
            ("\n", None),
            ("latest:^0.8", Some("latest:^0.8 ".to_string())),
            (
                r#"0.12.11

Other, not important
lines
"#,
                Some("0.12.11 ".to_string()),
            ),
        ];

        for test_case in test_cases {
            let actual = parse_tfenv_version(test_case.0.to_string());
            let expected = test_case.1;

            assert_eq!(actual, expected);
        }
    }
}
