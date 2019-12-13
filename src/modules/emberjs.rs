use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::emberjs::EmberjsConfig;
use crate::modules::package::extract_package_version;
use crate::utils;

/// Creates a module with the current Ember.js version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_js_project = context
        .try_begin_scan()?
        .set_files(&["package.json"])
        .set_folders(&["node_modules"])
        .is_match();

    if !is_js_project {
        return None;
    }

    let pkg_path = context
        .current_dir
        .join("node_modules")
        .join("ember-source")
        .join("package.json");
    let content = utils::read_file(pkg_path).ok()?;

    extract_package_version(&content).map(|version| {
        let mut module = context.new_module("emberjs");
        let config = EmberjsConfig::try_load(module.config);

        module.set_style(config.style);
        module.get_prefix().set_value("with ");

        let formatted_version = version.trim();
        module.create_segment("symbol", &config.symbol);
        module.create_segment("version", &SegmentConfig::new(formatted_version));

        module
    })
}
