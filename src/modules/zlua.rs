use crate::configs::zlua::ZluaConfig;

use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::utils;

use std::fmt;
use std::str::Lines;
use std::str::SplitWhitespace;

// Constants

/// Stores the environment variable hosting the LUA executable.
static ENV_VAR_LUA_EXE: &str = "ZLUA_LUAEXE";

/// Stores the environment variable hosting the _z.lua_ script.
static ENV_VAR_ZLUA_SCRIPT: &str = "ZLUA_SCRIPT";

/// Creates a module with the z.lua weight of the current working directory.
///
/// Will display the weight if the z.lua script is loaded in the current
/// shell.
///
/// Will activate on explicit configuration enabling and on presence of
/// environment variables `ZLUA_LUAEXE` and `ZLUA_SCRIPT`.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("zlua");
    let config: ZluaConfig<'_> = ZluaConfig::try_load(module.config);

    module.set_style(config.style);
    module.get_prefix().set_value(config.prefix);
    module.get_suffix().set_value(config.suffix);

    let zlua_lines = get_zlua_line_entries(&config)?;
    let cwd = context.current_dir.to_str()?;
    let zentry = get_cwd_entry(&zlua_lines, cwd)?;

    module.create_segment("zsymbol", &config.symbol);
    module.create_segment("zweight", &SegmentConfig::new(zentry.weight));

    Some(module)
}

/// Gets the z.lua entries as lines describing weights and folders.
fn get_zlua_line_entries(config: &ZluaConfig<'_>) -> Option<String> {
    let lua_exe = get_loc_lua_exe(config)?;
    let zlua_script = get_loc_zlua_script(config)?;

    let output = utils::exec_cmd(&lua_exe, &[&zlua_script, "-l"])?;
    Some(format!("{}", output.stdout))
}

/// Gets the ZEntry for the current directory.
///
/// # Arguments
///
/// * `zlines` - The z.lua line entries as a string.
/// * `cwd` - The current working directory.
fn get_cwd_entry<'a>(zlines: &'a str, cwd: &str) -> Option<ZEntry<'a>> {
    let mut lines: Lines = zlines.lines();
    lines.find_map(|line| to_z_entry(line).filter(|entry| entry.folder == cwd))
}

fn get_env_variable(name: &str) -> Option<String> {
    match std::env::var(name) {
        Ok(value) => Some(value),
        Err(_) => {
            log::debug!("Could not get the value of the '{}' env variable.", name);
            None
        }
    }
}

fn get_config_var(config_value: &str, env_var_name: &str) -> Option<String> {
    let clean_value = config_value.trim();
    if !clean_value.is_empty() {
        log::debug!("Found z.lua configuration value '{}'.", clean_value);
        Some(String::from(clean_value))
    } else {
        let env_var_value: Option<String> = get_env_variable(env_var_name);
        let unwrapped_value: &str = env_var_value.as_deref().unwrap_or("Missing");
        log::debug!(
            "Try to use the z.lua environment value '{}'.",
            unwrapped_value
        );
        env_var_value
    }
}

fn get_loc_lua_exe(config: &ZluaConfig<'_>) -> Option<String> {
    get_config_var(config.lua_exe_location, ENV_VAR_LUA_EXE)
}

fn get_loc_zlua_script(config: &ZluaConfig<'_>) -> Option<String> {
    get_config_var(config.zlua_script_location, ENV_VAR_ZLUA_SCRIPT)
}

/// Transforms a string line from the z.lua output into a ZEntry struct.
///
/// # Arguments
///
/// * `zline` - The z.lua line output.
fn to_z_entry(zline: &str) -> Option<ZEntry> {
    let mut zentries: SplitWhitespace = zline.split_whitespace();
    zentries
        .next()
        .and_then(|weight| zentries.next().map(|folder| (weight, folder)))
        .map(|(weight, folder)| ZEntry {
            weight: weight.trim(),
            folder: folder.trim(),
        })
}

#[derive(Debug)]
struct ZEntry<'a> {
    weight: &'a str,
    folder: &'a str,
}

impl fmt::Display for ZEntry<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "weight: {}, folder: {}", self.weight, self.folder)
    }
}

impl std::cmp::PartialEq for ZEntry<'_> {
    fn eq(&self, other: &ZEntry) -> bool {
        self.weight == other.weight && self.folder == other.folder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similar_z_entry() {
        let line = "686        D:\\Projects\\starship";
        assert_eq!(
            to_z_entry(line),
            Some(ZEntry {
                weight: "686",
                folder: "D:\\Projects\\starship"
            })
        );
    }

    #[test]
    fn test_non_similar_z_entry() {
        let line = "78         D:\\Projects\\guides";
        assert_ne!(
            to_z_entry(line),
            Some(ZEntry {
                weight: "",
                folder: "D:\\Projects\\starship"
            })
        );
    }

    #[test]
    fn test_z_entry_with_missing_weight() {
        let line = "        D:\\Projects\\starship";
        assert_eq!(to_z_entry(line), None);
    }

    #[test]
    fn test_z_entry_with_missing_folder() {
        let line = "78         ";
        assert_eq!(to_z_entry(line), None);
    }

    #[test]
    fn test_z_entry_equality() {
        let entry1 = ZEntry {
            folder: "D:\\Projects\\starship",
            weight: "1",
        };
        let entry2 = ZEntry {
            folder: "D:\\Projects\\starship",
            weight: "1",
        };

        assert_eq!(entry1, entry2);
    }

    #[test]
    fn test_z_entry_inequal_weight() {
        let entry1 = ZEntry {
            folder: "D:\\Projects\\starship",
            weight: "1",
        };
        let entry2 = ZEntry {
            folder: "D:\\Projects\\starship",
            weight: "2",
        };

        assert_ne!(entry1, entry2);
    }

    #[test]
    fn test_z_entry_inequal_folder() {
        let entry1 = ZEntry {
            folder: "D:\\Projects\\starship",
            weight: "1",
        };
        let entry2 = ZEntry {
            folder: "D:\\Projects\\starship\\docs",
            weight: "1",
        };

        assert_ne!(entry1, entry2);
    }

    #[test]
    fn test_present_cwd_entry() {
        let ex_output = "21         D:\\Projects
76         D:\\Projects\\starship\\docs
78         D:\\Projects\\guides
686        D:\\Projects\\starship";
        let zentry: Option<ZEntry> = get_cwd_entry(ex_output, "D:\\Projects\\starship");

        assert_eq!(
            zentry,
            Some(ZEntry {
                weight: "686",
                folder: "D:\\Projects\\starship"
            })
        );
    }

    #[test]
    fn test_missing_cwd_entry() {
        let ex_output = "21         D:\\Projects
76         D:\\Projects\\starship\\docs
78         D:\\Projects\\guides
686        D:\\Projects\\starship";
        let zentry: Option<ZEntry> = get_cwd_entry(ex_output, "D:\\Projects\\starship\\target");

        assert_eq!(zentry, None);
    }

    #[test]
    #[ignore]
    fn test_exec_cmd() {
        let result = get_zlua_line_entries(&ZluaConfig::new());
        assert!(result.is_some(), "No result was returned!");
    }
}
