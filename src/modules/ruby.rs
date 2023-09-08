use regex::Regex;

use super::{Context, Module, ModuleConfig};

use crate::configs::ruby::RubyConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

/// Creates a module with the current Ruby version
///
/// Will display the Ruby version if any of the following criteria are met:
///     - Current directory contains a `.rb` file
///     - Current directory contains a `Gemfile` or `.ruby-version` file
///     - The environment variables `RUBY_VERSION` or `RBENV_VERSION` are set
///
/// Will display the current Gemset if any of the following criteria are met:
///     - The environment variable `GEM_HOME` is set by RVM
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("ruby");
    let config = RubyConfig::try_load(module.config);

    let is_rb_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    let is_rb_env = &config
        .detect_variables
        .iter()
        .any(|variable| context.get_env(variable).is_some());

    if !is_rb_project && !is_rb_env {
        return None;
    }

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
                "version" => format_ruby_version(
                    &context.exec_cmd("ruby", &["-v"])?.stdout,
                    config.version_format,
                )
                .map(Ok),
                "gemset" => format_rvm_gemset_version(context).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `ruby`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn format_ruby_version(ruby_version: &str, version_format: &str) -> Option<String> {
    let version = ruby_version
        // split into ["ruby", "2.6.0p0", "linux/amd64"]
        .split_whitespace()
        // return "2.6.0p0"
        .nth(1)?
        // split into ["2.6.0", "0"]
        .split('p')
        // return "2.6.0"
        .next()?;

    match VersionFormatter::format_version(version, version_format) {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            log::warn!("Error formatting `ruby` version:\n{}", error);
            Some(format!("v{version}"))
        }
    }
}

fn format_rvm_gemset_version(context: &Context) -> Option<String> {
    if let Some(path) = context.get_env("GEM_HOME") {
        let stem = path.split('/').last()?;

        let version_re = Regex::new(r"(?:ruby-)(\d.\d.\d)").unwrap();
        let gemset_re = Regex::new(r"@(\S+)").unwrap();

        let version_cap = version_re.captures(stem);
        let gemset_cap = gemset_re.captures(stem);

        if let (Some(ver), Some(gem)) = (version_cap, gemset_cap) {
            let version = ver.get(1)?.as_str();
            let gemset = gem.get(1)?.as_str();
            return Some(format!("{}@{}", version, gemset));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_ruby_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("ruby").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_gemfile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Gemfile"))?.sync_all()?;

        let actual = ModuleRenderer::new("ruby").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("💎 v2.5.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ruby_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".ruby-version"))?.sync_all()?;

        let actual = ModuleRenderer::new("ruby").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("💎 v2.5.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_rb_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.rb"))?.sync_all()?;

        let actual = ModuleRenderer::new("ruby").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("💎 v2.5.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn with_ruby_version_env() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("ruby")
            .path(dir.path())
            .env("RUBY_VERSION", "2.5.1")
            .collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("💎 v2.5.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn with_rbenv_version_env() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("ruby")
            .path(dir.path())
            .env("RBENV_VERSION", "2.6.8")
            .collect();

        // rbenv variable is only detected; its value is not used
        let expected = Some(format!("via {}", Color::Red.bold().paint("💎 v2.5.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn with_rvm_gemset_env() -> io::Result<()> {
        // will check for gemset if the directory is a ruby directory
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.rb"))?.sync_all()?;

        let actual = ModuleRenderer::new("ruby")
            .path(dir.path())
            .env(
                "GEM_HOME",
                "/some/file/path/.rvm/gems/ruby-2.0.0-p481@test-gemset",
            )
            .config(toml::toml! {
                [ruby]
                format = "via [$symbol($gemset )]($style)"
            })
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Red.bold().paint("💎 2.0.0@test-gemset ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_ruby_version() {
        let config = RubyConfig::default();
        assert_eq!(
            format_ruby_version(
                "ruby 2.1.10p492 (2016-04-01 revision 54464) [x86_64-darwin19.0]",
                config.version_format
            ),
            Some("v2.1.10".to_string())
        );
        assert_eq!(
            format_ruby_version(
                "ruby 2.5.1p57 (2018-03-29 revision 63029) [x86_64-linux-gnu]",
                config.version_format
            ),
            Some("v2.5.1".to_string())
        );
        assert_eq!(
            format_ruby_version(
                "ruby 2.7.0p0 (2019-12-25 revision 647ee6f091) [x86_64-linux-musl]",
                config.version_format
            ),
            Some("v2.7.0".to_string())
        );
    }
}
