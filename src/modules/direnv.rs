use std::borrow::Cow;
use std::path::PathBuf;
use std::str::FromStr;

use super::{Context, Module, ModuleConfig};

use crate::configs::direnv::DirenvConfig;
use crate::formatter::StringFormatter;

use serde::Deserialize;

/// Creates a module with the current direnv rc
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("direnv");
    let config = DirenvConfig::try_load(module.config);
    let has_detected_env_var = context.detect_env_vars(&config.detect_env_vars);

    let direnv_applies = !config.disabled
        && (has_detected_env_var
            || context
                .try_begin_scan()?
                .set_extensions(&config.detect_extensions)
                .set_files(&config.detect_files)
                .set_folders(&config.detect_folders)
                .is_match());

    if !direnv_applies {
        return None;
    }

    // the `--json` flag is silently ignored for direnv versions <2.33.0
    let direnv_status = &context.exec_cmd("direnv", &["status", "--json"])?.stdout;
    let state = match DirenvState::from_str(direnv_status) {
        Ok(s) => s,
        Err(e) => {
            log::warn!("{e}");

            return None;
        }
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "symbol" => Some(Ok(Cow::from(config.symbol))),
                "rc_path" => Some(Ok(state.rc_path.to_string_lossy())),
                "allowed" => Some(Ok(match state.allowed {
                    AllowStatus::Allowed => Cow::from(config.allowed_msg),
                    AllowStatus::NotAllowed => Cow::from(config.not_allowed_msg),
                    AllowStatus::Denied => Cow::from(config.denied_msg),
                })),
                "loaded" => state
                    .loaded
                    .then_some(config.loaded_msg)
                    .or(Some(config.unloaded_msg))
                    .map(Cow::from)
                    .map(Ok),
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

struct DirenvState {
    pub rc_path: PathBuf,
    pub allowed: AllowStatus,
    pub loaded: bool,
}

impl FromStr for DirenvState {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str::<RawDirenvState>(s) {
            Ok(raw) => Ok(Self {
                rc_path: raw.state.found_rc.path,
                allowed: raw.state.found_rc.allowed.try_into()?,
                loaded: matches!(
                    raw.state.loaded_rc.allowed.try_into()?,
                    AllowStatus::Allowed
                ),
            }),
            Err(_) => Self::from_lines(s),
        }
    }
}

impl DirenvState {
    fn from_lines(s: &str) -> Result<Self, Cow<'static, str>> {
        let mut rc_path = PathBuf::new();
        let mut allowed = None;
        let mut loaded = true;

        for line in s.lines() {
            if let Some(path) = line.strip_prefix("Found RC path") {
                rc_path = PathBuf::from_str(path.trim()).map_err(|e| Cow::from(e.to_string()))?
            } else if let Some(value) = line.strip_prefix("Found RC allowed") {
                allowed = Some(AllowStatus::from_str(value.trim())?);
            } else if line.contains("No .envrc or .env loaded") {
                loaded = false;
            };
        }

        if rc_path.as_os_str().is_empty() || allowed.is_none() {
            return Err(Cow::from("unknown direnv state"));
        }

        Ok(Self {
            rc_path,
            allowed: allowed.unwrap(),
            loaded,
        })
    }
}

#[derive(Debug)]
enum AllowStatus {
    Allowed,
    NotAllowed,
    Denied,
}

impl FromStr for AllowStatus {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "true" => Ok(Self::Allowed),
            "1" => Ok(Self::NotAllowed),
            "2" | "false" => Ok(Self::Denied),
            _ => Err(Cow::from("invalid allow status")),
        }
    }
}

impl TryFrom<u8> for AllowStatus {
    type Error = Cow<'static, str>;

    fn try_from(u: u8) -> Result<Self, Self::Error> {
        match u {
            0 => Ok(Self::Allowed),
            1 => Ok(Self::NotAllowed),
            2 => Ok(Self::Denied),
            _ => Err(Cow::from("unknown integer allow status")),
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawDirenvState {
    pub state: State,
}

#[derive(Debug, Deserialize)]
struct State {
    #[serde(rename = "foundRC")]
    pub found_rc: RCStatus,
    #[serde(rename = "loadedRC")]
    pub loaded_rc: RCStatus,
}

#[derive(Debug, Deserialize)]
struct RCStatus {
    pub allowed: u8,
    pub path: PathBuf,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;
    use std::io;
    use std::path::Path;
    #[test]
    fn folder_without_rc_files_pre_2_33() {
        let renderer = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .cmd(
                "direnv status --json",
                Some(CommandOutput {
                    stdout: status_cmd_output_without_rc(),
                    stderr: String::default(),
                }),
            );

        assert_eq!(None, renderer.collect());
    }
    #[test]
    fn folder_without_rc_files() {
        let renderer = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .cmd(
                "direnv status --json",
                Some(CommandOutput {
                    stdout: status_cmd_output_without_rc_json(),
                    stderr: String::default(),
                }),
            );

        assert_eq!(None, renderer.collect());
    }
    #[test]
    fn folder_with_unloaded_rc_file_pre_2_33() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");

        std::fs::File::create(rc_path)?.sync_all()?;

        let actual = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .path(dir.path())
            .cmd(
                "direnv status --json",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc(dir.path(), false, "0", true),
                    stderr: String::default(),
                }),
            )
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::LightYellow.bold().paint("direnv not loaded/allowed")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_unloaded_rc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");

        std::fs::File::create(rc_path)?.sync_all()?;

        let actual = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .path(dir.path())
            .cmd(
                "direnv status --json",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc_json(dir.path(), 1, 0),
                    stderr: String::default(),
                }),
            )
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::LightYellow.bold().paint("direnv not loaded/allowed")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_loaded_rc_file_pre_2_33() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");

        std::fs::File::create(rc_path)?.sync_all()?;

        let actual = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .path(dir.path())
            .cmd(
                "direnv status --json",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc(dir.path(), true, "0", true),
                    stderr: String::default(),
                }),
            )
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::LightYellow.bold().paint("direnv loaded/allowed")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_loaded_rc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");

        std::fs::File::create(rc_path)?.sync_all()?;

        let actual = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .path(dir.path())
            .cmd(
                "direnv status --json",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc_json(dir.path(), 0, 0),
                    stderr: String::default(),
                }),
            )
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::LightYellow.bold().paint("direnv loaded/allowed")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_loaded_rc_file_in_subdirectory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");
        let sub_dir_path = dir.path().join("sub_dir");

        std::fs::File::create(rc_path)?.sync_all()?;
        std::fs::DirBuilder::new()
            .recursive(true)
            .create(&sub_dir_path)?;

        let actual = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                    disabled = false
            })
            .path(&sub_dir_path)
            .env("DIRENV_FILE", "file")
            .cmd(
                "direnv status --json",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc_json(dir.path(), 0, 0),
                    stderr: String::default(),
                }),
            )
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::LightYellow.bold().paint("direnv loaded/allowed")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_loaded_and_denied_rc_file_pre_2_33() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");

        std::fs::File::create(rc_path)?.sync_all()?;

        let actual = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .path(dir.path())
            .cmd(
                "direnv status --json",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc(dir.path(), true, "2", true),
                    stderr: String::default(),
                }),
            )
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::LightYellow.bold().paint("direnv loaded/denied")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_loaded_and_not_allowed_rc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");

        std::fs::File::create(rc_path)?.sync_all()?;

        let actual = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .path(dir.path())
            .cmd(
                "direnv status --json",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc_json(dir.path(), 0, 1),
                    stderr: String::default(),
                }),
            )
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::LightYellow.bold().paint("direnv loaded/not allowed")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_loaded_and_denied_rc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");

        std::fs::File::create(rc_path)?.sync_all()?;

        let actual = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .path(dir.path())
            .cmd(
                "direnv status --json",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc_json(dir.path(), 0, 2),
                    stderr: String::default(),
                }),
            )
            .collect();
        let expected = Some(format!(
            "{} ",
            Color::LightYellow.bold().paint("direnv loaded/denied")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
    fn status_cmd_output_without_rc() -> String {
        String::from(
            r"\
direnv exec path /usr/bin/direnv
DIRENV_CONFIG /home/test/.config/direnv
bash_path /usr/bin/bash
disable_stdin false
warn_timeout 5s
whitelist.prefix []
whitelist.exact map[]
No .envrc or .env loaded
No .envrc or .env found",
        )
    }
    fn status_cmd_output_without_rc_json() -> String {
        json!({
            "config": {
                "ConfigDir": config_dir(),
                "SelfPath": self_path(),
            },
            "state": {
                "foundRC": null,
                "loadedRC": null,
            }
        })
        .to_string()
    }
    fn status_cmd_output_with_rc(
        dir: impl AsRef<Path>,
        loaded: bool,
        allowed: &str,
        use_legacy_boolean_flags: bool,
    ) -> String {
        let rc_path = dir.as_ref().join(".envrc");
        let rc_path = rc_path.to_string_lossy();

        let allowed_value = match (use_legacy_boolean_flags, allowed) {
            (true, "0") => "true",
            (true, ..) => "false",
            (false, val) => val,
        };

        let loaded = if loaded {
            format!(
                r#"\
            Loaded RC path {rc_path}
            Loaded watch: ".envrc" - 2023-04-30T09:51:04-04:00
            Loaded watch: "../.local/share/direnv/allow/abcd" - 2023-04-30T09:52:58-04:00
            Loaded RC allowed {allowed_value}
            Loaded RC allowPath
            "#
            )
        } else {
            String::from("No .envrc or .env loaded")
        };

        let state = allowed.to_string();

        format!(
            r#"\
direnv exec path /usr/bin/direnv
DIRENV_CONFIG /home/test/.config/direnv
bash_path /usr/bin/bash
disable_stdin false
warn_timeout 5s
whitelist.prefix []
whitelist.exact map[]
{loaded}
Found RC path {rc_path}
Found watch: ".envrc" - 2023-04-25T18:45:54-04:00
Found watch: "../.local/share/direnv/allow/abcd" - 1969-12-31T19:00:00-05:00
Found RC allowed {state}
Found RC allowPath /home/test/.local/share/direnv/allow/abcd
"#
        )
    }
    fn status_cmd_output_with_rc_json(dir: impl AsRef<Path>, loaded: u8, allowed: u8) -> String {
        let rc_path = dir.as_ref().join(".envrc");
        let rc_path = rc_path.to_string_lossy();

        json!({
            "config": {
                "ConfigDir": config_dir(),
                "SelfPath": self_path(),
            },
            "state": {
                "foundRC": {
                    "allowed": allowed,
                    "path": rc_path,
                },
                "loadedRC": {
                    "allowed": loaded,
                    "path": rc_path,
                }
            }
        })
        .to_string()
    }
    #[cfg(windows)]
    fn config_dir() -> &'static str {
        r"C:\\Users\\test\\AppData\\Local\\direnv"
    }
    #[cfg(not(windows))]
    fn config_dir() -> &'static str {
        "/home/test/.config/direnv"
    }
    #[cfg(windows)]
    fn self_path() -> &'static str {
        r"C:\\Program Files\\direnv\\direnv.exe"
    }
    #[cfg(not(windows))]
    fn self_path() -> &'static str {
        "/usr/bin/direnv"
    }
}
