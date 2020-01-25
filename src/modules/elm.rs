use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::elm::ElmConfig;
use crate::utils;

/// Creates a module with the current Elm version
///
/// Will display the Elm version if any of the following criteria are met:
///     - The current directory contains a `elm.json` file
///     - The current directory contains a `elm-package.json` file
///     - The current directory contains a `elm-stuff` folder
///     - The current directory contains a `*.elm` files
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_elm_project = context
        .try_begin_scan()?
        .set_files(&["elm.json", "elm-package.json"])
        .set_extensions(&["elm"])
        .set_folders(&["elm-stuff"])
        .is_match();

    if !is_elm_project {
        return None;
    }

    let elm_version = utils::exec_cmd(
        "elm",
        &["--version"],
    )?
    .stdout;
    let formatted_version = format_elm_version(&elm_version)?;

    let mut module = context.new_module("elm");
    let config: ElmConfig = ElmConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(&formatted_version));

    Some(module)
}

fn format_elm_version(elm_version: &str) -> Option<String> {
    let formatted_version = format!("v{}", elm_version.trim());
    Some(formatted_version)
}
