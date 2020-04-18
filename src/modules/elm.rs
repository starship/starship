use super::{Context, Module, RootModuleConfig};

use crate::configs::elm::ElmConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Elm version
///
/// Will display the Elm version if any of the following criteria are met:
///     - The current directory contains a `elm.json` file
///     - The current directory contains a `elm-package.json` file
///     - The current directory contains a `.elm-version` file
///     - The current directory contains a `elm-stuff` folder
///     - The current directory contains a `*.elm` files
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_elm_project = context
        .try_begin_scan()?
        .set_files(&["elm.json", "elm-package.json", ".elm-version"])
        .set_extensions(&["elm"])
        .set_folders(&["elm-stuff"])
        .is_match();

    if !is_elm_project {
        return None;
    }

    let elm_version = utils::exec_cmd("elm", &["--version"])?.stdout;
    let module_version = Some(format!("v{}", elm_version.trim()))?;

    let mut module = context.new_module("elm");
    let config: ElmConfig = ElmConfig::try_load(module.config);
    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter.map(|variable| match variable {
            "version" => Some(module_version.clone()),
            _ => None,
        })
    } else {
        log::warn!("Error parsing format string in `elm.format`");
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
    fn folder_without_elm() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("elm", dir.path());
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_elm_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("elm.json"))?.sync_all()?;
        let actual = render_module("elm", dir.path());
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_elm_package_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("elm-package.json"))?.sync_all()?;
        let actual = render_module("elm", dir.path());
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_elm_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".elm-version"))?.sync_all()?;
        let actual = render_module("elm", dir.path());
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_elm_stuff_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let elmstuff = dir.path().join("elm-stuff");
        fs::create_dir_all(&elmstuff)?;
        let actual = render_module("elm", dir.path());
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_elm_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.elm"))?.sync_all()?;
        let actual = render_module("elm", dir.path());
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
