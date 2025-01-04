use crate::{
    config::ModuleConfig, configs::moonbit::MoonBitConfig, formatter::StringFormatter,
    module::Module,
};

use super::Context;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("moonbit");
    let config = MoonBitConfig::try_load(module.config);
    let is_moonbit_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_moonbit_project {
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
                    .exec_cmd("moonc", &["-v"])
                    .map(|cmd_output| Ok(cmd_output.stdout)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `moonbit`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_moonbit_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("moonbit").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_moonbit_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.mbt"))?.sync_all()?;

        let actual = ModuleRenderer::new("moonbit").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("🐰 v0.1.20241231+ba15a9a4e ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mod_manifest() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("moon.mod.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("moonbit").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("🐰 v0.1.20241231+ba15a9a4e ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_pkg_manifest() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("moon.pkg.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("moonbit").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("🐰 v0.1.20241231+ba15a9a4e ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
