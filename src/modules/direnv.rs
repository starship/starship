use std::borrow::Cow;
use std::path::PathBuf;
use std::str::FromStr;

use super::{Context, Module, ModuleConfig};

use crate::configs::direnv::DirenvConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current direnv rc
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("direnv");
    let config = DirenvConfig::try_load(module.config);

    let direnv_applies = !config.disabled
        && context
            .try_begin_scan()?
            .set_extensions(&config.detect_extensions)
            .set_files(&config.detect_files)
            .set_folders(&config.detect_folders)
            .is_match();

    if !direnv_applies {
        return None;
    }

    let direnv_status = &context.exec_cmd("direnv", &["status"])?.stdout;
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
    Denied,
}

impl FromStr for AllowStatus {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "true" => Ok(AllowStatus::Allowed),
            "false" => Ok(AllowStatus::Denied),
            _ => Err(Cow::from("invalid allow status")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use std::io;
    use std::path::Path;
    #[test]
    fn folder_without_rc_files() {
        let renderer = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .cmd(
                "direnv status",
                Some(CommandOutput {
                    stdout: status_cmd_output_without_rc(),
                    stderr: String::default(),
                }),
            );

        assert_eq!(None, renderer.collect());
    }
    #[test]
    fn folder_with_unloaded_rc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");

        std::fs::File::create(&rc_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .path(dir.path())
            .cmd(
                "direnv status",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc(dir.path(), false, true),
                    stderr: String::default(),
                }),
            );

        assert_eq!(
            Some(format!("direnv not loaded/allowed ")),
            renderer.collect()
        );

        dir.close()
    }
    #[test]
    fn folder_with_loaded_rc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");

        std::fs::File::create(&rc_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .path(dir.path())
            .cmd(
                "direnv status",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc(dir.path(), true, true),
                    stderr: String::default(),
                }),
            );

        assert_eq!(Some(format!("direnv loaded/allowed ")), renderer.collect());

        dir.close()
    }
    #[test]
    fn folder_with_loaded_and_denied_rc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rc_path = dir.path().join(".envrc");

        std::fs::File::create(&rc_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("direnv")
            .config(toml::toml! {
                [direnv]
                disabled = false
            })
            .path(dir.path())
            .cmd(
                "direnv status",
                Some(CommandOutput {
                    stdout: status_cmd_output_with_rc(dir.path(), true, false),
                    stderr: String::default(),
                }),
            );

        assert_eq!(Some(format!("direnv loaded/denied ")), renderer.collect());

        dir.close()
    }
    fn status_cmd_output_without_rc() -> String {
        String::from(
            r#"\
direnv exec path /usr/bin/direnv
DIRENV_CONFIG /home/test/.config/direnv
bash_path /usr/bin/bash
disable_stdin false
warn_timeout 5s
whitelist.prefix []
whitelist.exact map[]
No .envrc or .env loaded
No .envrc or .env found"#,
        )
    }
    fn status_cmd_output_with_rc(dir: impl AsRef<Path>, loaded: bool, allowed: bool) -> String {
        let rc_path = dir.as_ref().join(".envrc");
        let rc_path = rc_path.to_string_lossy();

        let loaded = if loaded {
            format!(
                r#"\
            Loaded RC path {rc_path}
            Loaded watch: ".envrc" - 2023-04-30T09:51:04-04:00
            Loaded watch: "../.local/share/direnv/allow/abcd" - 2023-04-30T09:52:58-04:00
            Loaded RC allowed false
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
}
