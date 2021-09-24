use std::fs::read_to_string;
use std::path::PathBuf;

use super::{Context, Module, RootModuleConfig};

use crate::configs::juju::JujuConfig;
use crate::formatter::StringFormatter;

use yaml_rust::{Yaml, YamlLoader};

fn get_yaml<P: Into<PathBuf>>(path: P) -> Option<Yaml> {
    let contents = read_to_string(path.into()).ok()?;
    Some(YamlLoader::load_from_str(&contents).ok()?.get(0)?.clone())
}

/// Creates a module that display the current Juju version and model
///
/// Will display the Juju version if the Juju snap is installed.
/// Will also display the active controller and model if they exist.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("juju");
    let config = JujuConfig::try_load(module.config);

    // Read version information directly from the snap directory,
    // instead of querying the CLI about version information.
    // JUJU_SNAP_DIR is not something actually set by snapcraft,
    // but is used for unit testing.
    let snap_dir = context
        .get_env("JUJU_SNAP_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| "/snap/juju/current".into());

    let doc = get_yaml(snap_dir.join("meta/snap.yaml"))?;
    let version = doc["version"].as_str()?;

    // Optionally, calculate the controller and model, and display
    // it in parentheses. The user may not have an active model,
    // in which case we just show the version number.
    let model = context
        .get_home()
        .map(|d| d.join(".local/share/juju/"))
        .and_then(|d| {
            let doc = get_yaml(d.join("controllers.yaml"))?;
            doc["current-controller"].as_str().and_then(|c| {
                let doc = get_yaml(d.join("models.yaml"))?;
                let model = doc["controllers"][c]["current-model"].as_str()?;
                Some(format!(" ({}:{})", c, model))
            })
        });

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
                "version" => Some(Ok(version)),
                "model" => model.as_ref().map(|m| Ok(m.as_str())),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::error!("Error in module `juju`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::{create_dir_all, write};
    use std::io;
    use std::path::PathBuf;

    fn write_file(path: PathBuf, contents: &[u8]) {
        create_dir_all(path.parent().unwrap()).unwrap();
        write(path, contents).unwrap();
    }

    #[test]
    fn test_none() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("juju")
            .env("HOME", dir.path().to_str().unwrap())
            .env("JUJU_SNAP_DIR", dir.path().to_str().unwrap())
            .path(dir.path())
            .collect();

        assert_eq!(None, actual);

        dir.close()
    }

    #[test]
    fn test_version_only() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        write_file(dir.path().join("meta/snap.yaml"), b"version: 1.2.3");

        let actual = ModuleRenderer::new("juju")
            .env("HOME", dir.path().to_str().unwrap())
            .env("JUJU_SNAP_DIR", dir.path().to_str().unwrap())
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {} ",
            Color::RGB(0xE9, 0x54, 0x20).paint("🔮 1.2.3")
        ));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn test_controller_only() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        write_file(dir.path().join("meta/snap.yaml"), b"version: 1.2.3");
        write_file(
            dir.path().join(".local/share/juju/controllers.yaml"),
            b"current-controller: foo",
        );

        let actual = ModuleRenderer::new("juju")
            .env("HOME", dir.path().to_str().unwrap())
            .env("JUJU_SNAP_DIR", dir.path().to_str().unwrap())
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {} ",
            Color::RGB(0xE9, 0x54, 0x20).paint("🔮 1.2.3")
        ));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn test_controller_and_model() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        write_file(dir.path().join("meta/snap.yaml"), b"version: 1.2.3");
        write_file(
            dir.path().join(".local/share/juju/controllers.yaml"),
            b"current-controller: foo",
        );
        write_file(
            dir.path().join(".local/share/juju/models.yaml"),
            b"controllers: { foo: { current-model: bar } }",
        );

        let actual = ModuleRenderer::new("juju")
            .env("HOME", dir.path().to_str().unwrap())
            .env("JUJU_SNAP_DIR", dir.path().to_str().unwrap())
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {} ",
            Color::RGB(0xE9, 0x54, 0x20).paint("🔮 1.2.3 (foo:bar)")
        ));
        assert_eq!(expected, actual);

        dir.close()
    }
}
