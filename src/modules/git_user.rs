use super::{Context, Module, ModuleConfig};

use crate::{
    configs::git_user::GitUserConfig, formatter::StringFormatter,
    modules::utils::truncate::truncate_text,
};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("git_user");
    let config = GitUserConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let repo = context.get_repo().ok();
    if !config.show_always && repo.is_none() {
        return None;
    }

    let len = if config.truncation_length <= 0 {
        log::warn!(
            "\"truncation_length\" should be a positive value, found {}",
            config.truncation_length
        );
        usize::MAX
    } else {
        config.truncation_length as usize
    };

    let git_config = open_git_config(repo);
    let [name, email] = ["user.name", "user.email"].map(|key| {
        git_config
            .as_ref()
            .and_then(|c| read_git_config_key(c, key))
            .map(|v| truncate_text(&v, len, config.truncation_symbol))
    });

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "name" => name.as_deref().map(|n| Ok(n.to_string())),
                "email" => email.as_deref().map(|e| Ok(e.to_string())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `git_user`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn open_git_config(repo: Option<&crate::context::Repo>) -> Option<gix::config::File<'static>> {
    match repo {
        Some(r) => Some(r.open().config_snapshot().clone()),
        None => gix::config::File::from_globals().ok(),
    }
}

fn read_git_config_key(config: &gix::config::File, key: &str) -> Option<String> {
    config
        .string(key)
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::{test::ModuleRenderer, utils::create_command};
    use nu_ansi_term::Color;

    fn make_repo_with_user(name: &str, email: &str) -> io::Result<tempfile::TempDir> {
        let dir = tempfile::tempdir()?;
        let path = dir.path();

        create_command("git")?
            .args(["init", path.to_str().unwrap()])
            .output()?;
        create_command("git")?
            .args(["-C", path.to_str().unwrap(), "config", "user.name", name])
            .output()?;
        create_command("git")?
            .args(["-C", path.to_str().unwrap(), "config", "user.email", email])
            .output()?;

        Ok(dir)
    }

    #[test]
    fn renders_name_and_email_in_git_repo() -> io::Result<()> {
        let dir = make_repo_with_user("Alice", "alice@example.com")?;

        let actual = ModuleRenderer::new("git_user").path(dir.path()).collect();

        let expected = Some(format!(
            "as {} ",
            Color::Green.bold().paint("Alice (alice@example.com)")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn custom_format_name_only() -> io::Result<()> {
        let dir = make_repo_with_user("Bob", "bob@example.com")?;

        let actual = ModuleRenderer::new("git_user")
            .path(dir.path())
            .config(toml::toml! {
                [git_user]
                format = "[$name]($style) "
            })
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint("Bob")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn custom_format_email_only() -> io::Result<()> {
        let dir = make_repo_with_user("Carol", "carol@example.com")?;

        let actual = ModuleRenderer::new("git_user")
            .path(dir.path())
            .config(toml::toml! {
                [git_user]
                format = "[$email]($style) "
            })
            .collect();

        let expected = Some(format!(
            "{} ",
            Color::Green.bold().paint("carol@example.com")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn custom_style_is_applied() -> io::Result<()> {
        let dir = make_repo_with_user("Dave", "dave@example.com")?;

        let actual = ModuleRenderer::new("git_user")
            .path(dir.path())
            .config(toml::toml! {
                [git_user]
                style = "bold green"
                format = "[$name]($style) "
            })
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint("Dave")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn truncates_long_name() -> io::Result<()> {
        let dir = make_repo_with_user("Verylongusername", "v@example.com")?;

        let actual = ModuleRenderer::new("git_user")
            .path(dir.path())
            .config(toml::toml! {
                [git_user]
                truncation_length = 5
                format = "[$name]($style) "
            })
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint("Veryl…")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn truncates_long_email() -> io::Result<()> {
        let dir = make_repo_with_user("Eve", "verylongemail@example.com")?;

        let actual = ModuleRenderer::new("git_user")
            .path(dir.path())
            .config(toml::toml! {
                [git_user]
                truncation_length = 9
                format = "[$email]($style) "
            })
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint("verylonge…")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn zero_truncation_length_shows_full_string() -> io::Result<()> {
        let long_name = "A".repeat(1000);
        let dir = make_repo_with_user(&long_name, "e@e.com")?;

        let actual = ModuleRenderer::new("git_user")
            .path(dir.path())
            .config(toml::toml! {
                [git_user]
                truncation_length = 0
                format = "[$name]($style) "
            })
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint(long_name)));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn custom_truncation_symbol() -> io::Result<()> {
        let dir = make_repo_with_user("Verylongusername", "v@example.com")?;

        let actual = ModuleRenderer::new("git_user")
            .path(dir.path())
            .config(toml::toml! {
                [git_user]
                truncation_length = 4
                truncation_symbol = "⁛"
                format = "[$name]($style) "
            })
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint("Very⁛")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn empty_truncation_symbol() -> io::Result<()> {
        let dir = make_repo_with_user("Verylongusername", "v@example.com")?;

        let actual = ModuleRenderer::new("git_user")
            .path(dir.path())
            .config(toml::toml! {
                [git_user]
                truncation_length = 4
                truncation_symbol = ""
                format = "[$name]($style) "
            })
            .collect();

        let expected = Some(format!("{} ", Color::Green.bold().paint("Very")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn does_not_render_name_when_missing() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        create_command("git")?
            .args(["init", dir.path().to_str().unwrap()])
            .output()?;
        create_command("git")?
            .args([
                "-C",
                dir.path().to_str().unwrap(),
                "config",
                "user.email",
                "x@x.com",
            ])
            .output()?;
        create_command("git")?
            .args([
                "-C",
                dir.path().to_str().unwrap(),
                "config",
                "user.name",
                "",
            ])
            .output()?;

        let actual = ModuleRenderer::new("git_user").path(dir.path()).collect();

        let expected = Some(format!("as {} ", Color::Green.bold().paint(" (x@x.com)")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn does_not_render_email_when_missing() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        create_command("git")?
            .args(["init", dir.path().to_str().unwrap()])
            .output()?;
        create_command("git")?
            .args([
                "-C",
                dir.path().to_str().unwrap(),
                "config",
                "user.name",
                "Frank",
            ])
            .output()?;
        create_command("git")?
            .args([
                "-C",
                dir.path().to_str().unwrap(),
                "config",
                "user.email",
                "",
            ])
            .output()?;

        let actual = ModuleRenderer::new("git_user").path(dir.path()).collect();

        let expected = Some(format!("as {} ", Color::Green.bold().paint("Frank")));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn disabled_module_renders_nothing() -> io::Result<()> {
        let dir = make_repo_with_user("Grace", "grace@example.com")?;

        let actual = ModuleRenderer::new("git_user")
            .path(dir.path())
            .config(toml::toml! {
                [git_user]
                disabled = true
            })
            .collect();

        assert_eq!(actual, None);
        dir.close()
    }
}
