use crate::configs::godot::GodotConfig;

use super::{Context, Module};

/// Creates a module that tells if the current directory is in a Godot project.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    use super::ModuleConfig;
    use crate::formatter::StringFormatter;
    use crate::formatter::VersionFormatter;

    let godot_project_root = context
        .begin_ancestor_scan()
        .set_files(&["project.godot"])
        .scan();
    godot_project_root?;

    let mut module = context.new_module("godot");
    let config: GodotConfig = GodotConfig::try_load(module.config);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => match config.show_version {
                    true => {
                        let version_string = get_godot_version(context, &config)?;
                        VersionFormatter::format_module_version(
                            module.get_name(),
                            &version_string,
                            config.version_format,
                        )
                        .map(Ok)
                    }
                    false => None,
                },
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `godot`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

/// Gets the godot version using the configured command.
fn get_godot_version(context: &Context, config: &GodotConfig) -> Option<String> {
    let version_output = context
        .exec_cmd(config.version_command[0], &config.version_command[1..])?
        .stdout;
    let version_vec: Vec<&str> = version_output
        .split('.')
        .take(3)
        .take_while(|substring| substring.parse::<u64>().is_ok())
        .collect();
    if version_vec.len() < 3 {
        return None;
    }
    Some(version_vec.join("."))
}

#[cfg(test)]
mod tests {
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;
    use std::fs::{create_dir_all, File};
    use std::io;

    #[test]
    fn folder_without_godot_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("godot").path(dir.path()).collect();

        let expected = None;

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_godot_file_in_current_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("project.godot"))?.sync_all()?;

        let actual = ModuleRenderer::new("godot").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint("godot ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_godot_file_in_ancestor() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("project.godot"))?.sync_all()?;
        let current_dir = dir.path().join("some/child/folders");
        create_dir_all(&current_dir)?;

        let actual = ModuleRenderer::new("godot").path(current_dir).collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint("godot ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn showing_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("project.godot"))?.sync_all()?;

        let actual = ModuleRenderer::new("godot")
            .path(dir.path())
            .config(toml::toml! {
                [godot]
                show_version = true
            })
            .collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint("godot v4.0.3 ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn version_command_gives_too_few_numbers() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("project.godot"))?.sync_all()?;

        let actual = ModuleRenderer::new("godot")
            .path(dir.path())
            .config(toml::toml! {
                [godot]
                show_version = true
            })
            .cmd(
                "godot --version",
                Some(CommandOutput {
                    stdout: String::from("4.0.not.numbers"),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint("godot ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn version_command_gives_garbage_output() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("project.godot"))?.sync_all()?;

        let actual = ModuleRenderer::new("godot")
            .path(dir.path())
            .config(toml::toml! {
                [godot]
                show_version = true
            })
            .cmd(
                "godot --version",
                Some(CommandOutput {
                    stdout: String::from("Garbage output that won't parse correctly."),
                    stderr: String::default(),
                }),
            )
            .collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint("godot ")));

        assert_eq!(expected, actual);
        dir.close()
    }
}
