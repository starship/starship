use super::{Context, Module, RootModuleConfig};
use crate::configs::heroku::HerokuConfig;
use crate::formatter::StringFormatter;
use crate::utils::read_file;

/// Read environment settings.
/// mimics
/// https://github.com/heroku/heroku-cli-command/blob/3e13ae899f64616c316dcf05398724fdd53c4d8a/src/vars.ts#L1-L48
/// only the methods we use.
mod heroku_vars {
    use super::Context;
    use url::Url;

    fn host(context: &Context) -> String {
        context
            .get_env("HEROKU_HOST")
            .unwrap_or_else(|| "heroku.com".to_string())
    }

    fn env_git_host(context: &Context) -> Option<String> {
        context.get_env("HEROKU_GIT_HOST")
    }

    fn git_host(context: &Context) -> String {
        if let Some(env_git_host) = env_git_host(context) {
            return env_git_host;
        }

        let host = host(context);
        if host.starts_with("http") {
            if let Ok(url) = Url::parse(&host) {
                if let Some(host) = url.host_str() {
                    return host.to_string();
                }
            }
        }
        host
    }

    fn http_git_host(context: &Context) -> String {
        if let Some(git_host) = env_git_host(context) {
            return git_host;
        }
        let host = host(context);
        if host.starts_with("http") {
            if let Ok(url) = Url::parse(&host) {
                if let Some(host) = url.host_str() {
                    return host.to_string();
                }
            }
        }
        format!("git.{}", host)
    }

    pub(super) fn git_prefixes(context: &Context) -> Vec<String> {
        vec![
            format!("git@{}:", git_host(context)),
            format!("ssh://git@{}/", git_host(context)),
            format!("https://{}/", http_git_host(context)),
        ]
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::test::default_context;

        #[test]
        fn test_default_prefixes() {
            let context = default_context();

            assert_eq!(
                git_prefixes(&context),
                vec![
                    "git@heroku.com:",
                    "ssh://git@heroku.com/",
                    "https://git.heroku.com/",
                ]
            );
        }

        #[test]
        fn test_git_heroku_host() {
            let mut context = default_context();
            context.env.insert("HEROKU_HOST", "will.be.ignored".into());
            context
                .env
                .insert("HEROKU_GIT_HOST", "other.git.host".into());

            assert_eq!(
                git_prefixes(&context),
                vec![
                    "git@other.git.host:",
                    "ssh://git@other.git.host/",
                    "https://other.git.host/",
                ]
            );
        }

        #[test]
        fn test_other_heroku_host() {
            let mut context = default_context();
            context.env.insert("HEROKU_HOST", "other.host".into());

            assert_eq!(
                git_prefixes(&context),
                vec![
                    "git@other.host:",
                    "ssh://git@other.host/",
                    "https://git.other.host/",
                ]
            );
        }
        #[test]
        fn test_other_heroku_host_with_http() {
            let mut context = default_context();
            context
                .env
                .insert("HEROKU_HOST", "https://other.host".into());

            assert_eq!(
                git_prefixes(&context),
                vec![
                    "git@other.host:",
                    "ssh://git@other.host/",
                    "https://other.host/",
                ]
            );
        }
    }
}

/// find currently used Heroku app.
/// Mimics:
/// https://github.com/heroku/heroku-cli-command/blob/3e13ae899f64616c316dcf05398724fdd53c4d8a/src/flags/app.ts#L22-L40
/// with
/// https://github.com/heroku/heroku-cli-command/blob/5e13ae899f64616c316dcf05398724fdd53c4d8a/src/git.ts
mod heroku_app {
    use super::{heroku_vars, Context};
    use git2::Repository;
    use regex::Regex;

    fn config_remote(repo: &Repository) -> Option<String> {
        let config = repo.config().ok()?;
        config.get_string("heroku.remote").ok()
    }

    fn get_apps_in_git_remotes(
        context: &Context,
        repo: &Repository,
        only_remote: Option<String>,
    ) -> Result<Vec<String>, git2::Error> {
        let prefixes = heroku_vars::git_prefixes(context);
        let remote_regexes: Vec<Regex> = prefixes
            .iter()
            .filter_map(|prefix| {
                let regex = format!(r"{}(.*)\.git", prefix);

                match Regex::new(&regex) {
                    Ok(re) => Some(re),
                    Err(err) => {
                        log::debug!(
                            "could not parse regex \"{}\" for Heroku remotes: {:?}",
                            regex,
                            err
                        );
                        None
                    }
                }
            })
            .collect();

        let mut app_remotes = Vec::new();
        for remote_name in repo.remotes()?.iter().flatten() {
            if let Some(ref only_remote) = only_remote {
                if remote_name != only_remote {
                    continue;
                }
            }

            let remote = repo.find_remote(remote_name)?;

            if let Some(url) = remote.url() {
                for regex in &remote_regexes {
                    if let Some(capture) = regex.captures_iter(url).next() {
                        app_remotes.push(capture[1].to_string());
                        break;
                    }
                }
            }
        }

        Ok(app_remotes)
    }

    pub(super) fn get_app(context: &Context) -> Option<String> {
        if let Some(env_app) = context.get_env("HEROKU_APP") {
            return Some(env_app);
        }

        let repository = context.get_repo().ok()?.open().ok()?;

        if let Ok(apps) = get_apps_in_git_remotes(context, &repository, config_remote(&repository))
        {
            if apps.len() == 1 {
                return Some(apps[0].clone());
            }
            // there can be more than one valid app, in more than one git remote.
            // In Heroku commands you typically have to provide `--remote` to
            // define which remote (and through that the app) to use.
            // Here we don't show any app when there is more than one.
        }

        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{
            test::{default_context, fixture_repo, FixtureProvider},
            utils::create_command,
        };
        use std::io;
        use tempfile::TempDir;

        fn add_remote(repo_dir: &TempDir, name: &str, url: &str) -> io::Result<()> {
            create_command("git")?
                .args(&["remote", "add", name, url])
                .current_dir(repo_dir.path())
                .output()?;

            Ok(())
        }

        #[test]
        fn everything_empty() {
            let context = default_context();
            assert!(get_app(&context).is_none());
        }

        #[test]
        fn from_env() {
            let mut context = default_context();
            context.env.insert("HEROKU_APP", "some_app".to_owned());
            assert_eq!(get_app(&context).unwrap(), "some_app");
        }

        #[test]
        fn from_remote() -> io::Result<()> {
            let repo_dir = fixture_repo(FixtureProvider::Git)?;
            let mut context = default_context();
            context.current_dir = repo_dir.path().into();

            add_remote(&repo_dir, "some_name", "git@heroku.com:some_app_name.git")?;
            add_remote(&repo_dir, "some_other_name", "git@github.com:repo.git")?;

            assert_eq!(get_app(&context).unwrap(), "some_app_name");

            Ok(())
        }

        #[test]
        fn multiple_apps() -> io::Result<()> {
            let repo_dir = fixture_repo(FixtureProvider::Git)?;
            let mut context = default_context();
            context.current_dir = repo_dir.path().into();

            add_remote(&repo_dir, "some_name", "git@heroku.com:some_app_name.git")?;
            add_remote(
                &repo_dir,
                "some_other_name",
                "https://git.heroku.com/other_app_name.git",
            )?;

            assert!(get_app(&context).is_none());

            Ok(())
        }
    }
}

fn account_from_netrc_file(context: &Context) -> Option<String> {
    // https://linux.die.net/man/5/netrc
    // This parses simple .netrc files, it could return wrong
    // results for files with `macdef`.
    // We only look at explicit machine definitions, not at
    // defaults. Heroku doesn't use the netrc default section.
    // Lastly, Heroku uses the `netrc-parser` package to edit
    // the file, which also supports GPG encrypted netrc files.
    // This we also don't support right now.

    let content = read_file(&context.get_home()?.join(".netrc")).ok()?;
    let tokens: Vec<&str> = content.split_whitespace().collect();

    let mut last_machine_was_heroku = false;

    for slice in tokens.chunks(2) {
        if let &[key, value] = slice {
            if key == "machine" {
                last_machine_was_heroku = value == "api.heroku.com" || value == "git.heroku.com";
            } else if key == "default" {
                last_machine_was_heroku = false;
            } else if key == "login" && last_machine_was_heroku {
                return Some(value.to_string());
            }
        }
    }
    None
}

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("heroku");
    let config = HerokuConfig::try_load(module.config);

    let app_name = heroku_app::get_app(context).map(|app| {
        config
            .app_aliases
            .get(&app)
            .map_or(app, |&app| app.to_string())
    });

    let account = account_from_netrc_file(context).map(|account| {
        config
            .account_aliases
            .get(&account)
            .map_or(account, |&account| account.to_string())
    });

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "app_name" => app_name.as_ref().map(Ok),
                "account" => account.as_ref().map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `heroku`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{self, Write};
    use std::path::Path;

    use crate::test::default_context;
    use crate::test::ModuleRenderer;

    fn with_temporary_home(f: impl FnOnce(&Context, &Path)) {
        let home_dir = tempfile::tempdir().unwrap();

        let path = home_dir.path();
        let mut context = default_context();
        context
            .env
            .insert("HOME", path.to_str().unwrap().to_string());

        f(&context, path)
    }

    fn create_netrc(directory: &Path, content: &str) {
        let mut netrc = fs::File::create(directory.join(".netrc")).unwrap();
        netrc.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn no_netrc() {
        with_temporary_home(|context, _| {
            assert!(account_from_netrc_file(context).is_none());
        })
    }

    #[test]
    fn just_one_token() {
        with_temporary_home(|context, path| {
            create_netrc(path, "some_string");
            assert!(account_from_netrc_file(context).is_none());
        })
    }

    #[test]
    fn other_machine() {
        with_temporary_home(|context, path| {
            create_netrc(path, "machine some.machine login other_name");
            assert!(account_from_netrc_file(context).is_none());
        })
    }

    #[test]
    fn heroku_machine() {
        with_temporary_home(|context, path| {
            create_netrc(
                path,
                "
                machine api.heroku.com \n 
                login the_account \n 
                password the_password \n",
            );
            assert_eq!(account_from_netrc_file(context).unwrap(), "the_account");
        })
    }

    #[test]
    fn default_in_between_heroku_machine() {
        with_temporary_home(|context, path| {
            create_netrc(
                path,
                "
                machine api.heroku.com \n 
                default \n
                login the_account \n 
                password the_password \n",
            );
            assert!(account_from_netrc_file(context).is_none());
        })
    }

    #[test]
    fn show_nothing_without_git_or_env_or_netrc() -> io::Result<()> {
        let repo_dir = tempfile::tempdir()?;
        let home_dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("heroku")
            .env("HOME", home_dir.path().to_str().unwrap())
            .path(repo_dir.path())
            .collect();

        assert_eq!(None, actual);
        Ok(())
    }

    #[test]
    fn full_module() {
        let home_dir = tempfile::tempdir().unwrap();
        create_netrc(
            home_dir.path(),
            "machine api.heroku.com \n login the_account \n ",
        );

        let actual = ModuleRenderer::new("heroku")
            .env("HOME", home_dir.path().to_str().unwrap())
            .env("HEROKU_APP", "app_name")
            .collect();

        assert_eq!(
            Some(
                "\u{1b}[34m \u{e77b}  on app_name (via \u{f007} the_account) \u{1b}[0m".to_string()
            ),
            actual
        );
    }

    #[test]
    fn with_aliases() {
        let home_dir = tempfile::tempdir().unwrap();
        create_netrc(
            home_dir.path(),
            "machine api.heroku.com \n login the_account \n ",
        );

        let actual = ModuleRenderer::new("heroku")
            .env("HOME", home_dir.path().to_str().unwrap())
            .env("HEROKU_APP", "app_name")
            .config(toml::toml! {
                [heroku.app_aliases]
                app_name = "the_app_alias"
                [heroku.account_aliases]
                the_account = "the_account_alias"
            })
            .collect();

        assert_eq!(
            Some(
                "\u{1b}[34m \u{e77b}  on the_app_alias (via \u{f007} the_account_alias) \u{1b}[0m"
                    .to_string()
            ),
            actual
        );
    }
}
