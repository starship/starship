use std::{env, fs, path};

use super::{Context, Module, RootModuleConfig};

use crate::configs::terraform::TerraformConfig;

/// Creates a module with the current terraform workspace,
/// iff there is a `.tf` file in the current directory.
///
/// First looks at `$TF_WORKSPACE`.
/// If that isn't set, looks at `$TF_DATA_DIR` to see if the data directory is overridden.
/// If it isn't looks in the local data directory, `.terraform`.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // See: https://github.com/hashicorp/terraform/blob/master/command/meta.go#L543

    let is_terraform_project = context.try_begin_scan()?.set_extensions(&["tf"]).is_match();
    if !is_terraform_project {
        return None;
    }

    let workspace = if let Ok(workspace) = env::var("TF_WORKSPACE") {
        workspace
    } else {
        let mut workspace_file = match env::var("TF_DATA_DIR") {
            Ok(path) => path::PathBuf::from(path), // https://github.com/hashicorp/terraform/blob/fb1aefe22b5d880459ad48ed98357e23730bd668/commands.go#L61
            Err(_) => path::PathBuf::from(".terraform"), // https://github.com/hashicorp/terraform/blob/047733d20ce1ee9702a650349e6defbb855c1ec5/command/meta.go#L212
        };
        workspace_file.push("environment");

        fs::read_to_string(workspace_file).ok()?
    };

    let mut module = context.new_module("terraform");
    let config = TerraformConfig::try_load(module.config);

    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("workspace", &config.workspace.with_value(&workspace));

    Some(module)
}
