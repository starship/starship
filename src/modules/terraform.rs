use super::{Context, Module, RootModuleConfig};

use crate::configs::terraform::TerraformConfig;
use crate::utils;
use std::io;
use std::path::PathBuf;

/// Creates a module with the current Terraform workspace
///
/// Will display the Terraform workspace if any of the following criteria are met:
///     - Current directory contains a `.terraform` directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_terraform_project = context
        .try_begin_scan()?
        .set_folders(&[".terraform"])
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

fn get_terraform_workspace(path: &PathBuf) -> Option<String> {
    match utils::read_file(path.join(".terraform/environment")) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Some("default".to_string()),
        Ok(s) => Some(s),
        _ => None,
    }
}
