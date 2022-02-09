use super::{Context, Module, RootModuleConfig};

use crate::configs::haskell::HaskellConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Haskell version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("haskell");
    let config = HaskellConfig::try_load(module.config);

    let is_hs_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_hs_project {
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
                "ghc_version" => get_ghc_version(context).map(Ok),
                "resolver_version" => get_stack_resolver_version(context).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `haskell`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_ghc_version(context: &Context) -> Option<String> {
    Some(
        context
            .exec_cmd("ghc", &["--numeric-version"])?
            .stdout
            .trim()
            .to_string()
    )
}

fn get_stack_resolver_version(context: &Context) -> Option<String> {
    let file_contents = utils::read_file(context.current_dir.join("stack.yaml")).ok()?;
    let yaml = yaml_rust::YamlLoader::load_from_str(&file_contents).ok()?;
    let version = yaml.first()?["resolver"]
        .as_str()
        .or(yaml.first()?["snapshot"].as_str())?;
    if version.starts_with("lts") || version.starts_with("nightly") || version.starts_with("ghc") {
        return Some(version.trim().to_string());
    } else {
        return Some("<custom snapshot>".to_string());
    }
}

fn get_version(context: &Context) -> Option<String> {
    // Use cached `dir_contents` to avoid unnecessary fs accesses
    if is_stack_project(context) {
        return get_stack_resolver_version(context);
    } else {
        return get_ghc_version(context);
    }
}

fn is_stack_project(context: &Context) -> bool {
    match context.dir_contents() {
        Ok(dir) => dir.has_file_name("stack.yaml"),
        Err(_) => false,
    }
}
