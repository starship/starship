use super::{Context, Module, ModuleConfig};

use crate::configs::jujutsu::JujutsuConfig;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu");
    let config = JujutsuConfig::try_load(module.config);
    let is_jj_repo = context
        .try_begin_scan()?
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_jj_repo {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map(|variable| match variable {
                "commit_info" => {
                    let output = &context.exec_cmd("jj", &["log", "-r@", "-n1", "--ignore-working-copy", "--no-graph", "--color","always", "-T", config.template])?.stdout;
                    Some(Ok(output.to_owned()))
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use std::fs::create_dir;
    use std::io;

    #[test]
    fn folder_without_jj_repo() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("jujutsu").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_jj_repo() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        create_dir(dir.path().join(".jj"))?;
        let actual = ModuleRenderer::new("jujutsu").path(dir.path()).collect();
        let expected = Some("jj main \"initial commit\" ");
        assert_eq!(expected, actual.as_deref());
        dir.close()
    }
}
