use super::{Context, Module, ModuleConfig};

use crate::configs::typst::TypstConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Typst version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("typst");
    let config = TypstConfig::try_load(module.config);

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
                "typst_version" => get_typst_config(context).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `typst`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_typst_config(context: &Context) -> Option<String> {
    Some(
        context
            .exec_cmd("typst", &["--version"])?
            .stdout
            .trim()
            .to_string()
            .as_str()[6..11]
            .trim()
            .to_string(),
    )
}

fn get_version(context: &Context) -> Option<String> {
    get_typst_config(context)
}
#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;
    #[test]
    fn read_typst_not_present() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("typst").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn read_typst_present() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        File::create(dir.path().join("test.typ"))?.sync_all()?;

        let actual = ModuleRenderer::new("typst").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::LightBlue.paint("\u{0074}\u{0308} 0.8.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
