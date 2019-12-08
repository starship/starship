use super::{Context, Module, RootModuleConfig};

use crate::configs::terraform::TerraformConfig;
use crate::utils;
use std::env;
use std::io;
use std::path::PathBuf;

/// Creates a module with the current Terraform version and workspace
///
/// Will display the Terraform version and workspace if any of the following criteria are met:
///     - Current directory contains a `.terraform` directory
///     - Current directory contains a file with the `.tf` extension
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_terraform_project = context
        .try_begin_scan()?
        .set_folders(&[".terraform"])
        .set_extensions(&["tf"])
        .is_match();

    if !is_terraform_project {
        return None;
    }

    let mut module = context.new_module("terraform");
    let config: TerraformConfig = TerraformConfig::try_load(module.config);

    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);

    if config.show_version {
        let terraform_version =
            format_terraform_version(&utils::exec_cmd("terraform", &["version"])?.stdout.as_str())?;
        module.create_segment("version", &config.version.with_value(&terraform_version));
    }

    let terraform_workspace = &get_terraform_workspace(&context.current_dir)?;
    module.create_segment(
        "workspace",
        &config.workspace.with_value(&terraform_workspace),
    );

    Some(module)
}

// Determines the currently selected workspace (see https://github.com/hashicorp/terraform/blob/master/command/meta.go for the original implementation)
fn get_terraform_workspace(cwd: &PathBuf) -> Option<String> {
    // Workspace can be explicitly overwritten by an env var
    let workspace_override = env::var("TF_WORKSPACE");
    if workspace_override.is_ok() {
        return workspace_override.ok();
    }

    // Data directory containing current workspace can be overwritten by an env var
    let datadir = match env::var("TF_DATA_DIR") {
        Ok(s) => PathBuf::from(s),
        Err(_) => cwd.join(".terraform"),
    };
    match utils::read_file(datadir.join("environment")) {
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => Some("default".to_string()),
        Ok(s) => Some(s),
        _ => None,
    }
}

fn format_terraform_version(version: &str) -> Option<String> {
    // `terraform version` output looks like this
    // Terraform v0.12.14
    // With potential extra output if it detects you are not running the latest version
    Some(
        version
            .lines()
            .next()?
            .trim_start_matches("Terraform ")
            .trim()
            .to_owned()
            + " ",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_terraform_version_release() {
        let input = "Terraform v0.12.14";
        assert_eq!(
            format_terraform_version(input),
            Some("v0.12.14 ".to_string())
        );
    }

    #[test]
    fn test_format_terraform_version_prerelease() {
        let input = "Terraform v0.12.14-rc1";
        assert_eq!(
            format_terraform_version(input),
            Some("v0.12.14-rc1 ".to_string())
        );
    }

    #[test]
    fn test_format_terraform_version_development() {
        let input = "Terraform v0.12.14-dev (cca89f74)";
        assert_eq!(
            format_terraform_version(input),
            Some("v0.12.14-dev (cca89f74) ".to_string())
        );
    }

    #[test]
    fn test_format_terraform_version_multiline() {
        let input = "Terraform v0.12.13

Your version of Terraform is out of date! The latest version
is 0.12.14. You can update by downloading from www.terraform.io/downloads.html

";
        assert_eq!(
            format_terraform_version(input),
            Some("v0.12.13 ".to_string())
        );
    }
}
