use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use super::{Context, Module, ModuleConfig};
use crate::configs::hg_state::HgStateConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the state of hg repository at the current directory
///
/// During a mercurial operation it will show: MERGING, REBASING, UPDATING etc.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("hg_state");
    let config: HgStateConfig = HgStateConfig::try_load(module.config);

    // As we default to disabled=true, we have to check here after loading our config module,
    // before it was only checking against whatever is in the config starship.toml
    if config.disabled {
        return None;
    }

    let repo_root = context.begin_ancestor_scan().set_folders(&[".hg"]).scan()?;

    let state_description = get_state_description(&repo_root, &config)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "state" => Some(state_description.label),
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
            log::warn!("Error in module `hg_state`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn get_state_description<'a>(
    hg_root: &Path,
    config: &HgStateConfig<'a>,
) -> Option<StateDescription<'a>> {
    if hg_root.join(".hg").join("rebasestate").exists() {
        Some(StateDescription {
            label: config.rebase,
        })
    } else if hg_root.join(".hg").join("updatestate").exists() {
        Some(StateDescription {
            label: config.update,
        })
    } else if hg_root.join(".hg").join("bisect.state").exists() {
        Some(StateDescription {
            label: config.bisect,
        })
    } else if hg_root.join(".hg").join("graftstate").exists() {
        Some(StateDescription {
            label: config.graft,
        })
    } else if hg_root
        .join(".hg")
        .join("transplant")
        .join("journal")
        .exists()
    {
        Some(StateDescription {
            label: config.transplant,
        })
    } else if hg_root.join(".hg").join("histedit-state").exists() {
        Some(StateDescription {
            label: config.histedit,
        })
    } else if is_merge_state(hg_root).is_ok() {
        Some(StateDescription {
            label: config.merge,
        })
    } else {
        None
    }
}

fn is_merge_state(hg_root: &Path) -> io::Result<bool> {
    let dirstate_path = hg_root.join(".hg").join("dirstate");

    let mut file = File::open(dirstate_path)?;
    let mut buffer = [0u8; 40]; // First 40 bytes: 20 for p1, 20 for p2
    file.read_exact(&mut buffer)?;

    let p2 = &buffer[20..40];
    let is_merge = p2.iter().any(|&b| b != 0);

    Ok(is_merge)
}

struct StateDescription<'a> {
    label: &'a str,
}

#[cfg(test)]
mod tests {
    use nu_ansi_term::Color;
    use std::fs::{File, create_dir_all};
    use std::io;
    use std::path::Path;

    use crate::test::{FixtureProvider, ModuleRenderer, fixture_repo};
    use crate::utils::create_command;

    #[test]
    fn test_hg_show_nothing_on_empty_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("hg_state")
            .path(repo_dir.path())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_disabled_per_default() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        run_hg(&["whatever", "blubber"], repo_dir)?;
        let actual = ModuleRenderer::new("hg_state")
            .path(repo_dir.to_str().unwrap())
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_merging() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        run_hg(&["branch", "-f", "branch-name-101"], repo_dir)?;
        run_hg(
            &[
                "commit",
                "-m",
                "empty commit 101",
                "-u",
                "fake user <fake@user>",
            ],
            repo_dir,
        )?;
        run_hg(&["update", "-r", "branch-name-0"], repo_dir)?;
        run_hg(&["merge", "-r", "branch-name-101"], repo_dir)?;
        expect_hg_state_with_config(
            repo_dir,
            Some(format!("({}) ", Color::Yellow.bold().paint("MERGING"))),
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_rebasing() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        File::create(repo_dir.join(".hg").join("rebasestate"))?;
        expect_hg_state_with_config(
            repo_dir,
            Some(format!("({}) ", Color::Yellow.bold().paint("REBASING"))),
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_updating() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        File::create(repo_dir.join(".hg").join("updatestate"))?;
        expect_hg_state_with_config(
            repo_dir,
            Some(format!("({}) ", Color::Yellow.bold().paint("UPDATING"))),
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_bisecting() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        run_hg(&["bisect", "--good"], repo_dir)?;
        expect_hg_state_with_config(
            repo_dir,
            Some(format!("({}) ", Color::Yellow.bold().paint("BISECTING"))),
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_grafting() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        File::create(repo_dir.join(".hg").join("graftstate"))?;
        expect_hg_state_with_config(
            repo_dir,
            Some(format!("({}) ", Color::Yellow.bold().paint("GRAFTING"))),
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_transplanting() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        create_dir_all(repo_dir.join(".hg").join("transplant"))?;
        File::create(repo_dir.join(".hg").join("transplant").join("journal"))?;
        expect_hg_state_with_config(
            repo_dir,
            Some(format!(
                "({}) ",
                Color::Yellow.bold().paint("TRANSPLANTING")
            )),
        );
        tempdir.close()
    }

    #[test]
    #[ignore]
    fn test_hg_histediting() -> io::Result<()> {
        let tempdir = fixture_repo(FixtureProvider::Hg)?;
        let repo_dir = tempdir.path();
        File::create(repo_dir.join(".hg").join("histedit-state"))?;
        expect_hg_state_with_config(
            repo_dir,
            Some(format!("({}) ", Color::Yellow.bold().paint("HISTEDITING"))),
        );
        tempdir.close()
    }

    fn run_hg(args: &[&str], repo_dir: &Path) -> io::Result<()> {
        create_command("hg")?
            .args(args)
            .current_dir(repo_dir)
            .output()?;
        Ok(())
    }

    fn expect_hg_state_with_config(repo_dir: &Path, expected: Option<String>) {
        let actual = ModuleRenderer::new("hg_state")
            .path(repo_dir.to_str().unwrap())
            .config({
                toml::toml! {
                    [hg_state]
                    disabled = false
                }
            })
            .collect();

        assert_eq!(expected, actual);
    }
}
