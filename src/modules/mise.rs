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

/// Represents the state of mise that will be displayed
#[derive(Debug)]
enum MiseState {
    /// Tools installed / required
    Ok { installed: usize, required: usize },
    /// Mise configuration is untrusted
    Untrusted,
    /// General error with mise
    Error,
    /// Mise doctor reports unhealthy
    Unhealthy,
}

/// Check if mise is healthy using `mise doctor`
/// Returns true if healthy, false if unhealthy or command fails
fn check_mise_health(context: &Context) -> bool {
    context.exec_cmd("mise", &["doctor"]).is_some()
}

/// Check if the current directory's mise config is trusted
/// Returns Some(true) if trusted, Some(false) if untrusted, None if unable to determine
fn check_mise_trust(context: &Context) -> Option<bool> {
    let output = context.exec_cmd("mise", &["trust", "--show"])?;

    // Expected format: "<path>: <status>"
    // where status is "trusted" or "untrusted"
    let stdout = output.stdout.trim();

    // Parse the trust status
    if let Some((_, status)) = stdout.rsplit_once(':') {
        let status = status.trim();
        return Some(status == "trusted");
    }

    None
}

/// Get the count of installed vs required tools from mise
/// Returns (required_count, installed_count) or None if unable to parse
fn get_tool_counts(context: &Context, local_only: bool) -> Option<(usize, usize)> {
    // Build the mise ls command with appropriate flags
    let mut args = vec!["ls", "--current", "--json"];
    if local_only {
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

    Some((required_count, installed_count))
}

/// Determine the current state of mise
/// This function orchestrates the health check, trust check, and tool counting
fn determine_mise_state(context: &Context, config: &MiseConfig) -> Option<MiseState> {
    // If health check is enabled, check health first
    // If unhealthy, don't proceed with tool counting
    if config.health_check_enabled && !check_mise_health(context) {
        return Some(MiseState::Unhealthy);
    }

    // Try to get tool counts. If this fails, it might be due to trust issues or other errors
    match get_tool_counts(context, config.local_only) {
        Some((required, installed)) => Some(MiseState::Ok {
            installed,
            required,
        }),
        // Check if it's a trust issue
        None => match check_mise_trust(context) {
            Some(false) => Some(MiseState::Untrusted),
            Some(true) => Some(MiseState::Error), // Trusted but still failed
            None => Some(MiseState::Error),       // Can't determine trust status
        },
    }
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

    let state = determine_mise_state(context, &config)?;
    log::trace!("mise state: {:?}", &state);

    if let MiseState::Ok { required: 0, .. } = state
        && !config.health_check_enabled
    {
        // When healthy is disabled and there are no required versions, we don't want to show the module at all
        log::trace!("no required versions, and no health check enabled - returning None");
        return None;
    }

    let selected_style = match &state {
        MiseState::Ok {
            installed,
            required,
        } if installed >= required => config.style,
        MiseState::Ok { installed: 0, .. } => config.style_missing_all,
        MiseState::Ok { .. } => config.style_missing_some,
        MiseState::Unhealthy => config.style_unhealthy,
        MiseState::Untrusted => config.style_untrusted,
        MiseState::Error => config.style_error,
    };
    log::trace!("style: {:?}", selected_style);

    let status: Option<String> = match &state {
        MiseState::Ok { .. } if config.health_check_enabled => Some(config.healthy_symbol.into()),
        MiseState::Ok { .. } => None,
        MiseState::Unhealthy => Some(config.unhealthy_symbol.into()),
        MiseState::Untrusted => Some(config.untrusted_symbol.into()),
        MiseState::Error => Some(config.error_symbol.into()),
    };
    log::trace!("status: {:?}", status);

    let (installed, required) = match &state {
        MiseState::Ok {
            installed,
            required,
        } if *required > 0 => (Some(installed.to_string()), Some(required.to_string())),
        _ => (None, None),
    };
    log::trace!("installed: {:?}", installed);
    log::trace!("required : {:?}", required);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(selected_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "symbol" => Some(Ok(config.symbol.to_string())),
                "status" => status.as_ref().map(|s| Ok(s.clone())),
                "installed" => installed.as_ref().map(|s| Ok(s.clone())),
                "required" => required.as_ref().map(|s| Ok(s.clone())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(e) => {
            log::warn!("error in module `mise`: {e}");
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

        let expected = Some(format!("on {}", Color::Purple.bold().paint("mise 2/2 ")));
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

        let expected = Some(format!("on {}", Color::Yellow.bold().paint("mise 1/2 ")));
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

        let expected = Some(format!("on {}", Color::Red.bold().paint("mise 0/2 ")));
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
                "mise ls --current --json --local",
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

        let expected = Some(format!("on {}", Color::Purple.bold().paint("mise 1/1 ")));
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

        let expected = Some(format!("on {}", Color::Purple.bold().paint("mise 1/1 ")));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_untrusted() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
            })
            .cmd("mise ls --current --json --local", None)
            .cmd(
                "mise trust --show",
                Some(CommandOutput {
                    stdout: format!("{}: untrusted", dir.path().display()),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!("on {}", Color::Red.bold().paint("mise untrusted ")));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_error() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
            })
            .cmd("mise ls --current --json --local", None)
            .cmd("mise trust --show", None);

        let expected = Some(format!("on {}", Color::Red.bold().paint("mise error ")));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_health_check_enabled_healthy() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
                health_check_enabled = true
            })
            .cmd(
                "mise doctor",
                Some(CommandOutput {
                    stdout: String::default(),
                    stderr: String::default(),
                }),
            )
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
            );

        let expected = Some(format!(
            "on {}",
            Color::Purple.bold().paint("mise healthy 1/1 ")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_health_check_enabled_unhealthy() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
                health_check_enabled = true
            })
            .cmd("mise doctor", None);

        let expected = Some(format!("on {}", Color::Red.bold().paint("mise unhealthy ")));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_health_check_enabled_healthy_no_tools() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
                health_check_enabled = true
            })
            .cmd(
                "mise doctor",
                Some(CommandOutput {
                    stdout: String::default(),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "mise ls --current --json --local",
                Some(CommandOutput {
                    stdout: "{}".into(),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!(
            "on {}",
            Color::Purple.bold().paint("mise healthy ")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_custom_unhealthy_symbol() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
                health_check_enabled = true
                unhealthy_symbol = "💀"
            })
            .cmd("mise doctor", None);

        let expected = Some(format!("on {}", Color::Red.bold().paint("mise 💀 ")));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_custom_healthy_symbol() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
                health_check_enabled = true
                healthy_symbol = "✅"
            })
            .cmd(
                "mise doctor",
                Some(CommandOutput {
                    stdout: String::default(),
                    stderr: String::default(),
                }),
            )
            .cmd(
                "mise ls --current --json --local",
                Some(CommandOutput {
                    stdout: "{}".into(),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!("on {}", Color::Purple.bold().paint("mise ✅ ")));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }
}
