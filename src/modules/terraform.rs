use super::{Context, Module, RootModuleConfig};

use crate::configs::terraform::TerraformConfig;
use crate::utils;
use std::env;
use std::io;
use std::path::PathBuf;

/// Creates a module with the current Terraform workspace
///
/// Will display the Terraform workspace if any of the following criteria are met:
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
        Err(e) if e.kind() == io::ErrorKind::NotFound => Some("default".to_string()),
        Ok(s) => Some(s),
        _ => None,
    }
}
