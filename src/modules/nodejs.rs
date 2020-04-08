use super::{Context, Module, RootModuleConfig};

use crate::configs::nodejs::NodejsConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Node.js version
///
/// Will display the Node.js version if any of the following criteria are met:
///     - Current directory contains a `.js` file
///     - Current directory contains a `package.json` or `.node-version` file
///     - Current directory contains a `node_modules` directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_js_project = context
        .try_begin_scan()?
        .set_files(&["package.json", ".node-version"])
        .set_extensions(&["js"])
        .set_folders(&["node_modules"])
        .is_match();

    if !is_js_project {
        return None;
    }

    let module_version = utils::exec_cmd("node", &["--version"])?.stdout;

    let mut module = context.new_module("nodejs");
    let config: NodejsConfig = NodejsConfig::try_load(module.config);
    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter.map(|variable| match variable {
            "version" => Some(module_version.clone()),
            _ => None,
        })
    } else {
        log::warn!("Error parsing format string in `nodejs.format`");
        return None;
    };

    module.set_segments(formatter.parse(None));

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::{self, File};
    use std::io;
    use tempfile;

    #[test]
    fn folder_without_node_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("nodejs", dir.path());
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path());
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_node_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".node-version"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path());
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_js_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.js"))?.sync_all()?;

        let actual = render_module("nodejs", dir.path());
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_node_modules() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let node_modules = dir.path().join("node_modules");
        fs::create_dir_all(&node_modules)?;

        let actual = render_module("nodejs", dir.path());
        let expected = Some(format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
