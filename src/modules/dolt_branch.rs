use unicode_segmentation::UnicodeSegmentation;

use super::{Context, Module, RootModuleConfig};

use crate::configs::dolt_branch::DoltBranchConfig;
use crate::formatter::StringFormatter;
use crate::utils::get_command_string_output;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("dolt_branch");
    let config: DoltBranchConfig = DoltBranchConfig::try_load(module.config);

    let is_dolt_repo = context
        .try_begin_scan()?
        .set_files(&config.detect_file)
        .set_folders(&config.detect_folder)
        .is_match();
    if !is_dolt_repo {
        return None;
    }

    if config.disabled {
        return None;
    }

    let len = if config.truncation_length <= 0 {
        log::warn!(
            "\"truncation_length\" should be a positive value, got {}",
            config.truncation_length
        );
        std::usize::MAX
    } else {
        config.truncation_length as usize
    };

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
                "branch" => {
                    let command = context.exec_cmd("dolt", &["branch", "--show-current"])?;
                    let branch_name = get_command_string_output(command);
                    let mut graphemes: Vec<&str> = branch_name.graphemes(true).collect();

                    let truncation_symbol =
                        UnicodeSegmentation::graphemes(config.truncation_symbol, true)
                            .next()
                            .unwrap_or("");

                    let trunc_length = len.min(graphemes.len());
                    if trunc_length < graphemes.len() {
                        graphemes[trunc_length] = truncation_symbol;
                        graphemes.truncate(trunc_length + 1);
                    }
                    Some(Ok(graphemes.concat()))
                }
                _ => None,
            })
            .parse(None, Some(context))
    });
    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `dolt_branch`:\n{}", error);
            return None;
        }
    });
    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::create_dir;
    use std::io;
    use toml::toml;

    #[test]
    fn test_non_repo_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("dolt_branch")
            .path(repo_dir.path())
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_repo_dir() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        create_dir(repo_dir.path().join(".dolt"))?;

        let actual = ModuleRenderer::new("dolt_branch")
            .path(&repo_dir.path())
            .collect();

        let expected = Some(format!(
            "on {}",
            Color::Purple.bold().paint(format!("\u{e0a0} {}", "main")),
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_disabled() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        create_dir(repo_dir.path().join(".dolt"))?;

        let actual = ModuleRenderer::new("dolt_branch")
            .path(repo_dir.path())
            .config(toml! {
                [dolt_branch]
                disabled = true
            })
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_style() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        create_dir(repo_dir.path().join(".dolt"))?;

        let actual = ModuleRenderer::new("dolt_branch")
            .path(repo_dir.path())
            .config(toml! {
                [dolt_branch]
                style = "green bold"
            })
            .collect();

        let expected = Some(format!(
            "on {}",
            Color::Green.bold().paint(format!("\u{e0a0} {}", "main"))
        ));

        assert_eq!(expected, actual);
        repo_dir.close()
    }

    #[test]
    fn test_truncation() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;

        create_dir(repo_dir.path().join(".dolt"))?;

        let actual = ModuleRenderer::new("dolt_branch")
            .path(repo_dir.path())
            .config(toml! {
                [dolt_branch]
                truncation_length = 3
                truncation_symbol = "."
            })
            .collect();

        let expected = Some(format!(
            "on {}",
            Color::Purple
                .bold()
                .paint(format!("\u{e0a0} {}{}", "mai", "."))
        ));
        assert_eq!(expected, actual);

        repo_dir.close()
    }
}
