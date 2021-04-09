use super::{Context, Module, RootModuleConfig};

use crate::configs::deno::DenoConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Deno version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("deno");
    let config = DenoConfig::try_load(module.config);
    let is_deno_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .is_match();

    if !is_deno_project {
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
                "version" => context
                    .exec_cmd("deno", &["--version"])
                    .map(|command_output| command_output.stdout)
                    .and_then(|deno_version_output| {
                        Some(format!("v{}", parse_deno_version(&deno_version_output)?))
                    })
                    .map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `deno`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_deno_version(deno_version: &str) -> Option<String> {
    let version = deno_version
        // split into ["deno", "1.8.3"]
        .split_whitespace()
        // return "1.8.3"
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
    fn deno_version() {
        let ok_versions = ["1.1.1", "2.0.0"];
        let not_ok_versions = ["abc", " \n.", ". ", "abc."];

        let all_some = ok_versions.iter().all(|&v| parse_deno_version(v).is_some());
        let all_none = not_ok_versions
            .iter()
            .any(|&v| parse_deno_version(v).is_some());

        assert_eq!(true, all_some);
        assert_eq!(true, all_none);

        let sample_deno_output = "\
deno 1.8.3 (release, x86_64-pc-windows-msvc)
v8 9.0.257.3
typescript 4.2.2
        ";

        assert_eq!(Some("1.8.3"), parse_deno_version(sample_deno_output))
    }

    #[test]
    fn folder_without_deno() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mod_ts() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mod.ts"))?.sync_all()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🦕 v1.8.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_deps_js() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("deps.js"))?.sync_all()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🦕 v1.8.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
