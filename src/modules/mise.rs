use std::collections::HashMap;

use super::{Context, Module, ModuleConfig};

use crate::configs::mise::MiseConfig;
use crate::formatter::StringFormatter;
use serde::Deserialize;

type MiseTools = HashMap<String, Vec<MiseToolInfo>>;

#[derive(Deserialize, Debug)]
struct MiseToolInfo {
    // version: String,
    // requested_version: String,
    // install_path: PathBuf,
    // source: MiseSource, (type: String, path: PathBuf)
    installed: bool,
    // active: bool,
}

/// Creates a module with the current mise config
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("mise");
    let config = MiseConfig::try_load(module.config);

    let mise_applies = !config.disabled
        && context
            .try_begin_scan()?
            .set_extensions(&config.detect_extensions)
            .set_files(&config.detect_files)
            .set_folders(&config.detect_folders)
            .is_match();

    if !mise_applies {
        return None;
    }

    // Build the mise ls command with appropriate flags
    let mut args = vec!["ls", "--current", "--json"];
    if config.local_only {
        args.push("--local");
    }

    // Execute mise ls command and parse the output
    let output = context.exec_cmd("mise", &args)?;
    let tools: MiseTools = serde_json::from_str(&output.stdout).ok()?;

    let (required_count, installed_count) = tools
        .values()
        .flatten()
        .fold((0, 0), |(required, installed), tool_info| {
            (required + 1, installed + tool_info.installed as usize)
        });

    // Don't show the module if there are no tools configured
    if required_count == 0 {
        return None;
    }

    // Check health status if enabled
    let healthy_str = if config.healthy_enabled {
        let doctor_output = context.exec_cmd("mise", &["doctor"]);
        if doctor_output.is_some() {
            "healthy "
        } else {
            "unhealthy "
        }
    } else {
        ""
    };

    // Convert counts to strings for the formatter
    let installed_str = installed_count.to_string();
    let required_str = required_count.to_string();

    // Determine the appropriate style based on installation status
    let selected_style = if installed_count == 0 {
        config.style_missing_all
    } else if installed_count < required_count {
        config.style_missing_some
    } else {
        config.style
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(selected_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "symbol" => Some(Ok(config.symbol)),
                "installed" => Some(Ok(installed_str.as_str())),
                "required" => Some(Ok(required_str.as_str())),
                "healthy" => Some(Ok(healthy_str)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(e) => {
            log::warn!("{e}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;

    use std::io;

    #[test]
    fn folder_without_mise_config() {
        let renderer = ModuleRenderer::new("mise").config(toml::toml! {
            [mise]
            disabled = false
        });

        assert_eq!(None, renderer.collect());
    }

    #[test]
    fn folder_with_mise_config_all_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
            })
            .cmd(
                "mise ls --current --json --local",
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{
                            "node": [
                                {
                                    "version": "20.0.0",
                                    "requested_version": "lts",
                                    "install_path": "/home/user/.local/share/mise/installs/node/20.0.0",
                                    "source": {
                                        "type": "mise.toml",
                                        "url": "/home/user/.config/mise/mise.toml"
                                    },
                                    "installed": true,
                                    "active": true
                                }
                            ],
                            "python": [
                                {
                                    "version": "3.11.0",
                                    "requested_version": "latest",
                                    "install_path": "/home/user/.local/share/mise/installs/python/3.11.0",
                                    "source": {
                                        "type": "mise.toml",
                                        "url": "/home/user/.config/mise/mise.toml"
                                    },
                                    "installed": true,
                                    "active": true
                                }
                            ]
                        }"#,
                    ),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!(
            "with {} ",
            Color::Purple.bold().paint("💾 mise 2/2")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_some_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
            })
            .cmd(
                "mise ls --current --json --local",
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{
                            "node": [
                                {
                                    "version": "20.0.0",
                                    "requested_version": "lts",
                                    "install_path": "/home/user/.local/share/mise/installs/node/20.0.0",
                                    "source": {
                                        "type": "mise.toml",
                                        "url": "/home/user/.config/mise/mise.toml"
                                    },
                                    "installed": true,
                                    "active": true
                                }
                            ],
                            "python": [
                                {
                                    "version": "3.11.0",
                                    "requested_version": "latest",
                                    "install_path": "/home/user/.local/share/mise/installs/python/3.11.0",
                                    "source": {
                                        "type": "mise.toml",
                                        "url": "/home/user/.config/mise/mise.toml"
                                    },
                                    "installed": false,
                                    "active": false
                                }
                            ]
                        }"#,
                    ),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!(
            "with {} ",
            Color::Yellow.bold().paint("💾 mise 1/2")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_none_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
            })
            .cmd(
                "mise ls --current --json --local",
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{
                            "node": [
                                {
                                    "version": "20.0.0",
                                    "requested_version": "lts",
                                    "install_path": "/home/user/.local/share/mise/installs/node/20.0.0",
                                    "source": {
                                        "type": "mise.toml",
                                        "url": "/home/user/.config/mise/mise.toml"
                                    },
                                    "installed": false,
                                    "active": false
                                }
                            ],
                            "python": [
                                {
                                    "version": "3.11.0",
                                    "requested_version": "latest",
                                    "install_path": "/home/user/.local/share/mise/installs/python/3.11.0",
                                    "source": {
                                        "type": "mise.toml",
                                        "url": "/home/user/.config/mise/mise.toml"
                                    },
                                    "installed": false,
                                    "active": false
                                }
                            ]
                        }"#,
                    ),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!("with {} ", Color::Red.bold().paint("💾 mise 0/2")));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_no_tools() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
            })
            .cmd(
                "mise ls --current --json",
                Some(CommandOutput {
                    stdout: String::from("{}"),
                    stderr: String::default(),
                }),
            );

        // Should return None when no tools are configured
        assert_eq!(None, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_folder() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_dir = dir.path().join(".mise");

        std::fs::create_dir_all(config_dir)?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
            })
            .cmd(
                "mise ls --current --json --local",
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{
                            "ruby": [
                                {
                                    "version": "3.2.0",
                                    "requested_version": "latest",
                                    "install_path": "/home/user/.local/share/mise/installs/ruby/3.2.0",
                                    "source": {
                                        "type": "mise.toml",
                                        "url": "/home/user/.config/mise/mise.toml"
                                    },
                                    "installed": true,
                                    "active": true
                                }
                            ]
                        }"#,
                    ),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!(
            "with {} ",
            Color::Purple.bold().paint("💾 mise 1/1")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_local_only_false() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
                local_only = false
            })
            .cmd(
                "mise ls --current --json",
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{
                            "ruby": [
                                {
                                    "version": "3.2.0",
                                    "requested_version": "latest",
                                    "install_path": "/home/user/.local/share/mise/installs/ruby/3.2.0",
                                    "source": {
                                        "type": "mise.toml",
                                        "url": "/home/user/.config/mise/mise.toml"
                                    },
                                    "installed": true,
                                    "active": true
                                }
                            ]
                        }"#,
                    ),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!(
            "with {} ",
            Color::Purple.bold().paint("💾 mise 1/1")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_healthy_enabled_healthy() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
                healthy_enabled = true
            })
            .cmd(
                "mise ls --current --json --local",
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{
                            "node": [
                                {
                                    "version": "20.0.0",
                                    "requested_version": "lts",
                                    "install_path": "/home/user/.local/share/mise/installs/node/20.0.0",
                                    "source": {
                                        "type": "mise.toml",
                                        "url": "/home/user/.config/mise/mise.toml"
                                    },
                                    "installed": true,
                                    "active": true
                                }
                            ]
                        }"#,
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "mise doctor",
                Some(CommandOutput {
                    stdout: String::from("All checks passed"),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!(
            "with {} ",
            Color::Purple.bold().paint("💾 mise healthy 1/1")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_healthy_enabled_unhealthy() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
                healthy_enabled = true
            })
            .cmd(
                "mise ls --current --json --local",
                Some(CommandOutput {
                    stdout: String::from(
                        r#"{
                            "node": [
                                {
                                    "version": "20.0.0",
                                    "requested_version": "lts",
                                    "install_path": "/home/user/.local/share/mise/installs/node/20.0.0",
                                    "source": {
                                        "type": "mise.toml",
                                        "url": "/home/user/.config/mise/mise.toml"
                                    },
                                    "installed": true,
                                    "active": true
                                }
                            ]
                        }"#,
                    ),
                    stderr: String::default(),
                }),
            )
            .cmd("mise doctor", None);

        let expected = Some(format!(
            "with {} ",
            Color::Purple.bold().paint("💾 mise unhealthy 1/1")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }
}
