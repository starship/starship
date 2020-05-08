use std::env;
use std::path::Path;

use super::{Context, Module, RootModuleConfig};
use crate::configs::virtualenv::VirtualEnvConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current virtualenv version
///
/// Will display the virtualenv folder if any of the following criteria are met:
///     - Current directory contains a file with the `.py` extension
///     - The environment variable `VIRTUAL_ENV` is present
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("virtualenv");
    let config: VirtualEnvConfig = VirtualEnvConfig::try_load(module.config);

    let is_py_project = context.try_begin_scan()?.set_extensions(&["py"]).is_match();
    let is_venv = env::var("VIRTUAL_ENV").ok().is_some();

    if !is_py_project && !is_venv {
        return None;
    }

    let virtual_env = get_python_virtual_env()?;
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
                // This may result in multiple calls to `get_module_version` when a user have
                // multiple `$version` variables defined in `format`.
                "virtualenv" => Some(Ok(virtual_env.clone())),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `virtualenv`:\n{}", error);
            return None;
        }
    });

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}

fn get_python_virtual_env() -> Option<String> {
    env::var("VIRTUAL_ENV")
        .ok()
        .and_then(|value| get_folder_virtual_env(&value))
}

fn get_folder_virtual_env(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .map(|filename| String::from(filename.to_str().unwrap_or("")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_folder_virtual_env() {
        let path = "/users/starship-captain/my-python-project/venv";
        let actual: String = get_folder_virtual_env(path).unwrap();
        assert_eq!(actual, "venv");
    }
}
