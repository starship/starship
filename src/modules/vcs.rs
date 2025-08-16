use std::borrow::Cow;
use std::path::Path;

use super::{Context, Module, ModuleConfig};

use crate::configs::vcs::VcsConfig;
use crate::formatter::StringFormatter;
use crate::formatter::string_formatter::StringFormatterError;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("vcs");
    let config = VcsConfig::try_load(module.config);

    if config.disabled || config.order.is_empty() {
        return None;
    }

    let vcs = config
        .order
        .into_iter()
        .filter_map(|vcs| Vcs::try_from(vcs).ok())
        .find(|vcs| discover_repo_root(context, *vcs).is_some())?;

    let modules = match vcs {
        Vcs::Fossil => config.fossil_modules,
        Vcs::Git => config.git_modules,
        Vcs::Hg => config.hg_modules,
        Vcs::Pijul => config.pijul_modules,
    };

    if modules.is_empty() {
        return None;
    }

    let parsed = StringFormatter::new(modules).and_then(|formatter| {
        formatter
            .map_variables_to_segments(|variable| match variable {
                "vcs" => Some(Err(StringFormatterError::Custom(
                    "cannot recursively include the `vcs` module in itself".into(),
                ))),
                module => super::handle(module, context).map(|m| Ok(m.segments)),
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `vcs`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

pub fn discover_repo_root<'a>(context: &'a Context, vcs: Vcs) -> Option<Cow<'a, Path>> {
    let scan = context.begin_ancestor_scan();

    let scan = match vcs {
        Vcs::Fossil => scan.set_files(if cfg!(windows) {
            &["_FOSSIL_"]
        } else {
            &[".fslckout"]
        }),
        Vcs::Hg => scan.set_folders(&[".hg"]),
        Vcs::Pijul => scan.set_folders(&[".pijul"]),
        Vcs::Git => return context.get_repo().ok().map(|r| r.repo.path().into()),
    };

    scan.scan().map(Into::into)
}

#[derive(Debug, Copy, Clone)]
pub enum Vcs {
    Fossil,
    Git,
    // NOTE: uses `hg` to correspond to existing `hg_branch` module
    Hg,
    Pijul,
}

impl<'a> TryFrom<&'a str> for Vcs {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "fossil" => Ok(Self::Fossil),
            "git" => Ok(Self::Git),
            "hg" | "mercurial" => Ok(Self::Hg),
            "pijul" => Ok(Self::Pijul),
            _ => Err(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use nu_ansi_term::Color;

    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};

    #[test]
    fn empty_order_disables() {
        let actual = ModuleRenderer::new("vcs")
            .config(toml::toml! {
                [vcs]
                order = []
            })
            .collect();
        assert_eq!(actual, None);
    }

    #[test]
    fn empty_modules_disables() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Fossil)?;

        let actual = ModuleRenderer::new("vcs")
            .config(toml::toml! {
                [vcs]
                order = ["fossil"]
                fossil_modules = ""
            })
            .path(repo_dir.path())
            .collect();
        assert_eq!(actual, None);

        repo_dir.close()
    }

    #[test]
    fn recursive_vcs_include_fails() -> io::Result<()> {
        let repo_dir = fixture_repo(FixtureProvider::Fossil)?;

        let actual = ModuleRenderer::new("vcs")
            .config(toml::toml! {
                [vcs]
                order = ["fossil"]
                fossil_modules = "$vcs"
            })
            .path(repo_dir.path())
            .collect();
        assert_eq!(actual, None);

        repo_dir.close()
    }

    #[test]
    fn detect_fossil() -> io::Result<()> {
        with_marker(
            "fossil",
            FixtureProvider::Fossil,
            Some(format!("{}", Color::Green.bold().paint("test "))),
        )
    }

    #[test]
    fn detect_git() -> io::Result<()> {
        with_marker(
            "git",
            FixtureProvider::Git,
            Some(format!("{}", Color::Green.bold().paint("test "))),
        )
    }

    #[test]
    fn detect_hg() -> io::Result<()> {
        with_marker(
            "hg",
            FixtureProvider::Hg,
            Some(format!("{}", Color::Green.bold().paint("test "))),
        )
    }

    #[test]
    fn detect_hg_alias_mercurial() -> io::Result<()> {
        with_marker(
            "mercurial",
            FixtureProvider::Hg,
            Some(format!("{}", Color::Green.bold().paint("test "))),
        )
    }

    #[test]
    fn detect_pijul() -> io::Result<()> {
        with_marker(
            "pijul",
            FixtureProvider::Pijul,
            Some(format!("{}", Color::Green.bold().paint("test "))),
        )
    }

    #[test]
    fn invalid_vcs_is_none() -> io::Result<()> {
        with_marker("does_not_exists", FixtureProvider::Fossil, None)
    }

    #[track_caller]
    fn with_marker(
        vcs_name: &'static str,
        fixture: FixtureProvider,
        expected: Option<String>,
    ) -> io::Result<()> {
        let repo_dir = match fixture {
            // Custom handling of Mercurial because we only care to detect the repo root, not run `hg` commands
            FixtureProvider::Hg => {
                let repo_dir = tempfile::tempdir()?;
                std::fs::create_dir(repo_dir.path().join(".hg"))?;
                repo_dir
            }
            _ => fixture_repo(fixture)?,
        };

        let config = toml::toml! {
            [vcs]
            order = [vcs_name]
            // Use `custom.test` for VCSes
            fossil_modules = "${custom.test}"
            git_modules = "${custom.test}"
            hg_modules = "${custom.test}"
            pijul_modules = "${custom.test}"

            // Inserting the `custom.test` module to have something printed that we control
            [custom.test]
            command = "echo test"
            when = true
        };

        let actual = ModuleRenderer::new("vcs")
            .config(config)
            .path(repo_dir.path())
            .collect();

        assert_eq!(actual, expected);

        repo_dir.close()
    }
}
