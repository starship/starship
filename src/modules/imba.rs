use super::{Context, Module, RootModuleConfig};

use crate::configs::imba::ImbaConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Imba version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("imba");
    let config = ImbaConfig::try_load(module.config);
    let is_imba_project = context
        .try_begin_scan()?
        .set_extensions(&config.detect_extensions)
        .is_match();

    if !is_imba_project {
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
                    .exec_cmd("imba", &["-V"])
                    .map(|output| parse_imba_version(output.stdout.trim()))
                    .map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `imba`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_imba_version(imba_version: &str) -> String {
    let version = imba_version
        // trim trailing new line
        .trim();

    format!("v{}", version)
}

#[cfg(test)]
mod tests {
    use super::parse_imba_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_imba_version() {
        const OUTPUT: &str = "2.0.0-alpha.132\n";
        assert_eq!(
            parse_imba_version(OUTPUT.trim()),
            "v2.0.0-alpha.132".to_string()
        )
    }

    #[test]
    fn folder_without_imba_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("imba").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_imba_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.imba"))?.sync_all()?;
        let actual = ModuleRenderer::new("imba").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🐤 v2.0.0-alpha.132 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
