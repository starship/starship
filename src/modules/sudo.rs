use std::{
    env,
    path::{Path, PathBuf},
};

use super::{Context, Module, ModuleConfig};

use crate::configs::sudo::SudoConfig;
use crate::formatter::StringFormatter;

use semver::Version;

#[inline]
fn get_cache_dir(context: &Context) -> Option<PathBuf> {
    let cache = context
        .get_env_os("STARSHIP_CACHE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            dirs::cache_dir()
                .or_else(|| crate::utils::home_dir().map(|home| home.join(".cache")))
                .unwrap_or_else(std::env::temp_dir)
                .join("starship")
        });

    if !cache.exists()
        && let Err(err) = std::fs::create_dir_all(&cache)
    {
        log::error!("Unable to create cache dir {cache:?}: {err:?}!");
        return None;
    }

    Some(cache)
}

#[derive(Debug, PartialEq)]
enum FlagStatus {
    FlagUpdatePending,
    FlagUpToDate,
    SudoOld,
    NoFlag,
}

impl FlagStatus {
    fn from_cache<P: AsRef<Path>>(supports_n: P, sudo_checked: P) -> Self {
        let Ok(checked_meta) = sudo_checked.as_ref().metadata() else {
            return Self::NoFlag;
        };
        let Ok(flag_mtime) = checked_meta.modified() else {
            return Self::NoFlag;
        };

        let Ok(sudo_path) = which::which_global("sudo") else {
            return Self::NoFlag;
        };
        let Ok(sudo_meta) = sudo_path.metadata() else {
            return Self::NoFlag;
        };
        let Ok(sudo_mtime) = sudo_meta.modified() else {
            return Self::NoFlag;
        };

        if flag_mtime < sudo_mtime {
            return Self::FlagUpdatePending;
        }

        if supports_n.as_ref().exists() {
            Self::FlagUpToDate
        } else {
            Self::SudoOld
        }
    }
}

fn supports_n_flag(context: &Context) -> bool {
    let Some(cache) = get_cache_dir(context) else {
        return false;
    };
    let flag_supports_n = cache.join("STARSHIP_SUDO_SUPPORTS_N_FLAG");
    let flag_checked = cache.join("STARSHIP_SUDO_VERSION_CHECKED");

    let write_cache = |supports: bool| -> std::io::Result<()> {
        // always touch sudo_checked to mark that we ran
        std::fs::File::create(&flag_checked)?;

        if supports {
            let _ = std::fs::File::create(&flag_supports_n);
        } else {
            // ensure supports_n is absent so SudoOld is reported next time
            let _ = std::fs::remove_file(&flag_supports_n);
        }

        Ok(())
    };

    let check_version = || -> bool {
        let supported = context
            .exec_cmd("sudo", &["-V"])
            .and_then(|out| {
                out.stdout
                    .lines()
                    .take(1)
                    .flat_map(|ver_line| ver_line.split_whitespace())
                    .find_map(|word| {
                        // take only the leading "1.9.17" part, drop "p2"
                        let trimmed = word.split(|c: char| !c.is_numeric() && c != '.').next()?;
                        Version::parse(trimmed).ok()
                    })
            })
            .is_some_and(|ver| ver >= Version::new(1, 9, 12));

        let _ = write_cache(supported);

        supported
    };

    let flag_status = FlagStatus::from_cache(&flag_supports_n, &flag_checked);
    log::debug!("sudo flag status: {:#?}", flag_status);

    match flag_status {
        FlagStatus::FlagUpToDate => true,
        FlagStatus::SudoOld => false,
        FlagStatus::FlagUpdatePending | FlagStatus::NoFlag => check_version(),
    }
}

/// Creates a module with sudo credential cache status
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("sudo");
    let config = SudoConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    if !config.allow_windows && env::consts::FAMILY == "windows" {
        return None;
    }

    let sudo_flags = if supports_n_flag(context) {
        "-Nnv"
    } else {
        "-nv"
    };
    let is_sudo_cached = context.exec_cmd("sudo", &[sudo_flags]).is_some();

    if !is_sudo_cached {
        return None;
    }

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
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `sudo`:\n{error}");
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::modules::sudo::FlagStatus;
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;

    static OLD_SUDO_V: &str = "\
Sudo version 1.9.5p2
Sudoers policy plugin version 1.9.5p2
Sudoers file grammar version 48
Sudoers I/O plugin version 1.9.5p2
Sudoers audit plugin version 1.9.5p2";

    #[test]
    fn test_old_sudo_not_cached() {
        let cache = tempfile::tempdir().unwrap();

        let actual = ModuleRenderer::new("sudo")
            .cmd("sudo -nv", None)
            .cmd(
                "sudo -V",
                Some(CommandOutput {
                    stdout: OLD_SUDO_V.to_string(),
                    stderr: String::default(),
                }),
            )
            .config(toml::toml! {
                [sudo]
                disabled = false
                allow_windows = true
            })
            .env("STARSHIP_CACHE", cache.path().to_str().unwrap())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_old_sudo_cached() {
        let cache = tempfile::tempdir().unwrap();

        let actual = ModuleRenderer::new("sudo")
            .cmd(
                "sudo -nv",
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::new(),
                }),
            )
            .cmd(
                "sudo -V",
                Some(CommandOutput {
                    stdout: OLD_SUDO_V.to_string(),
                    stderr: String::default(),
                }),
            )
            .config(toml::toml! {
                [sudo]
                disabled = false
                allow_windows = true
            })
            .env("STARSHIP_CACHE", cache.path().to_str().unwrap())
            .collect();
        let expected = Some(format!("{}", Color::Blue.bold().paint("as 🧙 ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_old_sudo_n_flag_cached() {
        let cache = tempfile::tempdir().unwrap();
        let cache = cache.path();

        let flag_checked = cache.join("STARSHIP_SUDO_VERSION_CHECKED");
        let flag_supports_n = cache.join("STARSHIP_SUDO_SUPPORTS_N_FLAG");

        assert_eq!(
            FlagStatus::NoFlag,
            FlagStatus::from_cache(&flag_supports_n, &flag_checked)
        );

        // setting a time guaranteed older than sudo
        std::fs::File::create(&flag_checked)
            .unwrap()
            .set_modified(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();

        if cfg!(windows) {
            // no sudo on windows
            assert_eq!(
                FlagStatus::NoFlag,
                FlagStatus::from_cache(&flag_supports_n, &flag_checked)
            );
        } else {
            assert_eq!(
                FlagStatus::FlagUpdatePending,
                FlagStatus::from_cache(&flag_supports_n, &flag_checked)
            );
        }

        {
            // if sudo is not cached
            let actual = ModuleRenderer::new("sudo")
                .cmd(
                    "sudo -V",
                    Some(CommandOutput {
                        stdout: OLD_SUDO_V.to_string(),
                        stderr: String::default(),
                    }),
                )
                .cmd("sudo -nv", None)
                .config(toml::toml! {
                    [sudo]
                    disabled = false
                    allow_windows = true
                })
                .env("STARSHIP_CACHE", cache.to_str().unwrap())
                .collect();
            let expected = None;

            assert_eq!(expected, actual);
        }

        if cfg!(windows) {
            // no sudo on windows
            assert_eq!(
                FlagStatus::NoFlag,
                FlagStatus::from_cache(&flag_supports_n, &flag_checked)
            );
        } else {
            assert_eq!(
                FlagStatus::SudoOld,
                FlagStatus::from_cache(&flag_supports_n, &flag_checked)
            );
        }

        {
            // if sudo is cached
            let actual = ModuleRenderer::new("sudo")
                .cmd(
                    "sudo -V",
                    Some(CommandOutput {
                        stdout: OLD_SUDO_V.to_string(),
                        stderr: String::default(),
                    }),
                )
                .cmd(
                    "sudo -nv",
                    Some(CommandOutput {
                        stdout: String::new(),
                        stderr: String::new(),
                    }),
                )
                .config(toml::toml! {
                    [sudo]
                    disabled = false
                    allow_windows = true
                })
                .env("STARSHIP_CACHE", cache.to_str().unwrap())
                .collect();
            let expected = Some(format!("{}", Color::Blue.bold().paint("as 🧙 ")));

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_sudo_n_flag_cached() {
        let cache = tempfile::tempdir().unwrap();
        let cache = cache.path();

        let flag_supports_n = cache.join("STARSHIP_SUDO_SUPPORTS_N_FLAG");
        let flag_checked = cache.join("STARSHIP_SUDO_VERSION_CHECKED");

        assert_eq!(
            FlagStatus::NoFlag,
            FlagStatus::from_cache(&flag_supports_n, &flag_checked)
        );

        // setting a time guaranteed older than sudo
        std::fs::File::create(flag_supports_n)
            .unwrap()
            .set_modified(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();
        std::fs::File::create(flag_checked)
            .unwrap()
            .set_modified(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();

        let flag_supports_n = cache.join("STARSHIP_SUDO_SUPPORTS_N_FLAG");
        let flag_checked = cache.join("STARSHIP_SUDO_VERSION_CHECKED");

        if cfg!(windows) {
            // no sudo on windows
            assert_eq!(
                FlagStatus::NoFlag,
                FlagStatus::from_cache(&flag_supports_n, &flag_checked)
            );
        } else {
            assert_eq!(
                FlagStatus::FlagUpdatePending,
                FlagStatus::from_cache(&flag_supports_n, &flag_checked)
            );
        }

        {
            // sudo not cached
            let actual = ModuleRenderer::new("sudo")
                .cmd("sudo -Nnv", None)
                .config(toml::toml! {
                    [sudo]
                    disabled = false
                    allow_windows = true
                })
                .env("STARSHIP_CACHE", cache.to_str().unwrap())
                .collect();
            let expected = None;

            assert_eq!(expected, actual);
        }

        if cfg!(windows) {
            // no sudo on windows
            assert_eq!(
                FlagStatus::NoFlag,
                FlagStatus::from_cache(&flag_supports_n, &flag_checked)
            );
        } else {
            assert_eq!(
                FlagStatus::FlagUpToDate,
                FlagStatus::from_cache(&flag_supports_n, &flag_checked)
            );
        }

        {
            // sudo is cached
            let actual = ModuleRenderer::new("sudo")
                .cmd(
                    "sudo -Nnv",
                    Some(CommandOutput {
                        stdout: String::new(),
                        stderr: String::new(),
                    }),
                )
                .config(toml::toml! {
                    [sudo]
                    disabled = false
                    allow_windows = true
                })
                .env("STARSHIP_CACHE", cache.to_str().unwrap())
                .collect();
            let expected = Some(format!("{}", Color::Blue.bold().paint("as 🧙 ")));

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_sudo_not_cached() {
        let cache = tempfile::tempdir().unwrap();

        let actual = ModuleRenderer::new("sudo")
            .cmd("sudo -Nnv", None)
            .config(toml::toml! {
                [sudo]
                disabled = false
                allow_windows = true
            })
            .env("STARSHIP_CACHE", cache.path().to_str().unwrap())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sudo_cached() {
        let cache = tempfile::tempdir().unwrap();

        let actual = ModuleRenderer::new("sudo")
            .cmd(
                "sudo -Nnv",
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::new(),
                }),
            )
            .config(toml::toml! {
                [sudo]
                disabled = false
                allow_windows = true
            })
            .env("STARSHIP_CACHE", cache.path().to_str().unwrap())
            .collect();
        let expected = Some(format!("{}", Color::Blue.bold().paint("as 🧙 ")));

        assert_eq!(expected, actual);
    }

    #[test]
    #[cfg(windows)]
    fn test_allow_windows_disabled_blocks_windows() {
        let cache = tempfile::tempdir().unwrap();

        let actual = ModuleRenderer::new("sudo")
            .cmd(
                "sudo -Nnv",
                Some(CommandOutput {
                    stdout: String::new(),
                    stderr: String::new(),
                }),
            )
            .config(toml::toml! {
                [sudo]
                disabled = false
                allow_windows = false
            })
            .env("STARSHIP_CACHE", cache.path().to_str().unwrap())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }
}
