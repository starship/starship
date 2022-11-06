use super::{Context, Module, ModuleConfig};

use crate::configs::lua::LuaConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils::get_command_string_output;

/// Creates a module with the current Lua version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("lua");
    let config = LuaConfig::try_load(module.config);

    let is_lua_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .set_extensions(&config.detect_extensions)
        .is_match();

    if !is_lua_project {
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
                "version" => {
                    let lua_version_string =
                        get_command_string_output(context.exec_cmd(config.lua_binary, &["-v"])?);
                    let lua_version = parse_lua_version(&lua_version_string)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &lua_version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `lua`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_lua_version(lua_version: &str) -> Option<String> {
    // lua -v output looks like this:
    // Lua 5.4.0  Copyright (C) 1994-2020 Lua.org, PUC-Rio

    // luajit -v output looks like this:
    // LuaJIT 2.0.5 -- Copyright (C) 2005-2017 Mike Pall. http://luajit.org/
    let version = lua_version
        // Lua: split into ["Lua", "5.4.0", "Copyright", ...]
        // LuaJIT: split into ["LuaJIT", "2.0.5", "--", ...]
        .split_whitespace()
        // Lua: take "5.4.0"
        // LuaJIT: take "2.0.5"
        .nth(1)?;

    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn folder_without_lua_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("lua").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_lua_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.lua"))?.sync_all()?;
        let actual = ModuleRenderer::new("lua").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🌙 v5.4.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_lua_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".lua-version"))?.sync_all()?;

        let actual = ModuleRenderer::new("lua").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🌙 v5.4.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_lua_folder() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let lua_dir = dir.path().join("lua");
        fs::create_dir_all(lua_dir)?;

        let actual = ModuleRenderer::new("lua").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("🌙 v5.4.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn lua_binary_is_luajit() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.lua"))?.sync_all()?;

        let config = toml::toml! {
             [lua]
             lua_binary = "luajit"
        };

        let actual = ModuleRenderer::new("lua")
            .path(dir.path())
            .config(config)
            .collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint("🌙 v2.0.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_parse_lua_version() {
        let lua_input = "Lua 5.4.0  Copyright (C) 1994-2020 Lua.org, PUC-Rio";
        assert_eq!(parse_lua_version(lua_input), Some("5.4.0".to_string()));

        let luajit_input =
            "LuaJIT 2.1.0-beta3 -- Copyright (C) 2005-2017 Mike Pall. http://luajit.org/";
        assert_eq!(
            parse_lua_version(luajit_input),
            Some("2.1.0-beta3".to_string())
        );
    }
}
