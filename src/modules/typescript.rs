use super::{Context, Module, RootModuleConfig};

use crate::configs::typescript::TypeScriptConfig;
use crate::formatter::StringFormatter;
use crate::utils;
use serde_json as json;
#[allow(unused_imports)]
use std::io::Write;

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
                        let file_contents =
                            &utils::read_file(context.current_dir.join(&"package.json")).unwrap();
                        let package_json: json::Value = json::from_str(file_contents).ok()?;
                        let deps = package_json["dependencies"]["typescript"].as_str();
                        let dev_deps = package_json["devDependencies"]["typescript"].as_str();
                        if let Some(deps_v) = deps {
                            Some(
                                format!("v{}", deps_v.to_owned())
                                    .trim()
                                    .to_owned()
                                    .replace(" ", ""),
                            )
                            .map(Ok)
                        } else if let Some(dev_deps_v) = dev_deps {
                            Some(
                                format!("v{}", dev_deps_v.to_owned())
                                    .trim()
                                    .to_owned()
                                    .replace(" ", ""),
                            )
                            .map(Ok)
                        } else {
                            None
                        }
                    } else {
                        let cmd = if is_deno_project { "deno" } else { "tsc" };
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
        Ok(mut segments) => {
            &segments.iter_mut().for_each(|seg| {
                let mut n = 0;
                seg.value.retain(|c| {
                    if c.is_whitespace() {
                        dbg!(n);
                        n = n + 1;
                        dbg!(n);
                        if n == 3 {
                            dbg!("n == 3");
                            false
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                });
            });
            segments
        },
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
        .nth(1)?
        // trim
        .trim();

    Some(format!("v{}", version))
}

fn parse_tsc_version(supplied_version: &str) -> Option<String> {
    let version = supplied_version
        // split into ["Typescript", "4.2.2"]
        .split_whitespace()
        // get 2nd value
        .nth(1)?
        // trim
        .trim();

    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::parse_deno_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use std::io::Write;

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
    fn folder_with_deno_ts_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mod.ts"))?.sync_all()?;
        let actual = ModuleRenderer::new("typescript").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint(" v4.2.2")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_json_ts_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hi.ts"))?.sync_all()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.sync_all()?;
        file.write(
            serde_json::json!({ "dependencies": { "typescript": "4.2.1" } })
                .to_string()
                .as_bytes(),
        )?;
        let actual = ModuleRenderer::new("typescript").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint(" v4.2.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ts_files_using_global() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hi.ts"))?.sync_all()?;
        let actual = ModuleRenderer::new("typescript").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint(" v4.2.0")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
