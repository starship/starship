use super::{Context, Module, SegmentConfig};

use crate::config::RootModuleConfig;
use crate::configs::js_library::JsLibraryConfig;
use crate::modules::package::extract_package_version;
use crate::utils;

use node_resolve::resolve_from;
use rayon::prelude::*;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("js_library");

    let config = JsLibraryConfig::try_load(module.config);

    let mut results: Vec<_> = config
        .map
        .par_iter()
        .map(|(_, v)| v)
        .filter(|config_item| !config_item.disabled)
        .filter_map(|config_item| {
            let dep = config_item.dependency;

            let target = format!("{}/package.json", dep);
            log::debug!("looking for {} from {:?}", dep, &context.current_dir);

            let result = resolve_from(&target, context.current_dir.clone());
            if let Err(err) = result {
                log::debug!("{}: resolution failed: {:?}", dep, err);
                return None;
            }

            let pkg_path = result.unwrap();
            log::debug!("{}: resolution successful: {:?}", dep, &pkg_path);

            let result = utils::read_file(&pkg_path);
            if let Err(err) = result {
                log::debug!("{}: reading package.json failed: {:?}", dep, err);
                return None;
            }

            let content = result.unwrap();
            let result = extract_package_version(&content);
            if result.is_none() {
                log::debug!("{}: version not found", dep);
                return None;
            }

            let version = result.unwrap();
            log::debug!("{}: version found: {}", dep, version);

            Some((config_item, version))
        })
        .collect();

    if results.is_empty() {
        return None;
    }

    results.sort_unstable_by_key(|(config_item, _)| config_item.dependency);

    module.get_prefix().set_value(config.prefix);
    module.get_suffix().set_value(config.suffix);

    for (i, (config_item, version)) in results.iter().enumerate() {
        if i != 0 {
            module.create_segment("separator", &SegmentConfig::new(config.separator));
        }

        let segment_config = SegmentConfig::new(version.trim()).with_style(config_item.style);

        module.create_segment("symbol", &config_item.symbol);
        module.create_segment("version", &segment_config);
    }

    Some(module)
}
