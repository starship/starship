use super::{Context, Module, RootModuleConfig};

use crate::configs::zig::ZigConfig;
use crate::utils;

/// Creates a module with the current Zig version
///
/// Will display the Zig version if any of the following criteria are met:
///     - The current directory contains a file with extension `.zig`
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_zig_project = context
        .try_begin_scan()?
        .set_extensions(&["zig"])
        .is_match();

    if !is_zig_project {
        return None;
    }

    let zig_version_output = utils::exec_cmd("zig", &["version"])?
        .stdout
        .trim()
        .to_string();
    let zig_version = format!("v{}", zig_version_output);

    let mut module = context.new_module("zig");
    let config = ZigConfig::try_load(module.config);

    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &config.version.with_value(&zig_version));

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_zig() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("zig.txt"))?.sync_all()?;
        let actual = render_module("zig", dir.path(), None);
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_zig_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.zig"))?.sync_all()?;
        let actual = render_module("zig", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("↯ v0.6.0")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
