use super::{Context, Module, RootModuleConfig};

use crate::configs::typescript::TypeScriptConfig;
use crate::formatter::StringFormatter;
use crate::utils;

use serde_json as json;

/// Creates a module with the current TypeScript version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("typescript");
    let config = TypeScriptConfig::try_load(module.config);
    let should_use = context.try_begin_scan()?.set_extensions(&["ts"]).is_match();
    let is_deno_project = context
        .try_begin_scan()?
        .set_files(&["mod.ts", "deps.ts"])
        .is_match();
    let is_node_project = context
        .try_begin_scan()?
        .set_files(&["package.json"])
        .is_match();
    let global = context.exec_cmd("tsc", &["--version"]).is_some();
    if !should_use && !is_deno_project && !is_node_project && global {
        return None;
    };
    let cmd = if is_deno_project { "deno" } else { "tsc" };
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
                    if is_node_project {
                        let package_json_result: json::Result<json::Value> = json::from_str(
                            &utils::read_file(context.current_dir.join(&"package.json")).unwrap(),
                        );
                        let package_json = package_json_result.unwrap();
                        let deps = package_json
                            .get("dependencies")?
                            .get("typescript")?
                            .as_str();
                        let dev_deps = package_json
                            .get("devDependencies")?
                            .get("typescript")?
                            .as_str();
                        if let Some(deps_v) = deps {
                            Some(format!("v{}", deps_v)).map(Ok)
                        } else if let Some(dev_deps_v) = dev_deps {
                            Some(format!("v{}", dev_deps_v)).map(Ok)
                        } else {
                            None
                        }
                    } else {
                        context
                            .exec_cmd(cmd, &["--version"])
                            .and_then(|output| {
                                if output.stdout.contains("deno") {
                                    parse_deno_version(output.stdout.trim())
                                } else {
                                    parse_tsc_version(output.stdout.trim())
                                }
                            })
                            .map(Ok)
                    }
                }
                _ => None,
            })
            .parse(None)
    });
    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `typescript`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_deno_version(deno_version: &str) -> Option<String> {
    let version = deno_version
        // split into multi lines
        .split("\n")
        // get 3rd line
        .nth(2)
        // unwrap
        .unwrap()
        // split by whitespace
        .split_whitespace()
        // return "4.2.2"
        .nth(1)?;

    Some(format!("v{}", version))
}

fn parse_tsc_version(supplied_version: &str) -> Option<String> {
    let version = supplied_version
        // split into ["Typescript", "4.2.2"]
        .split_whitespace()
        // get 2nd value
        .nth(1)?;

    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::parse_deno_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_deno_version() {
        const OUTPUT: &str = "\
        deno 1.8.3 (release, x86_64-pc-windows-msvc)
        v8 9.0.257.3
        typescript 4.2.2\n";
        assert_eq!(
            parse_deno_version(OUTPUT.trim()),
            Some("v4.2.2".to_string())
        )
    }

    #[test]
    fn folder_without_ts_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("typescript").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ts_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mod.ts"))?.sync_all()?;
        let actual = ModuleRenderer::new("typescript").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint(" ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
