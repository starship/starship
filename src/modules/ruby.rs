use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::ruby::RubyConfig;
use crate::utils;

/// Creates a module with the current Ruby version
///
/// Will display the Ruby version if any of the following criteria are met:
///     - Current directory contains a `.rb` file
///     - Current directory contains a `Gemfile` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_rb_project = context
        .try_begin_scan()?
        .set_files(&["Gemfile"])
        .set_extensions(&["rb"])
        .is_match();

    if !is_rb_project {
        return None;
    }

    let ruby_version = utils::exec_cmd("ruby", &["-v"])?.stdout;
    let formatted_version = format_ruby_version(&ruby_version)?;

    let mut module = context.new_module("ruby");
    let config: RubyConfig = RubyConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(&formatted_version));

    Some(module)
}

fn format_ruby_version(ruby_version: &str) -> Option<String> {
    let version = ruby_version
        // split into ["ruby", "2.6.0p0", "linux/amd64"]
        .split_whitespace()
        // return "2.6.0p0"
        .nth(1)?
        .get(0..5)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use tempfile;

    #[test]
    fn folder_without_ruby_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = render_module("ruby", dir.path());

        let expected = None;
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn folder_with_gemfile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Gemfile"))?.sync_all()?;

        let actual = render_module("ruby", dir.path());

        let expected = Some(format!("via {} ", Color::Red.bold().paint("💎 v2.5.1")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn folder_with_rb_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.rb"))?.sync_all()?;

        let actual = render_module("ruby", dir.path());

        let expected = Some(format!("via {} ", Color::Red.bold().paint("💎 v2.5.1")));
        assert_eq!(expected, actual);
        Ok(())
    }

    #[test]
    fn test_format_ruby_version() -> io::Result<()> {
        assert_eq!(
            format_ruby_version("ruby 2.5.1p57 (2018-03-29 revision 63029) [x86_64-linux-gnu]"),
            Some("v2.5.1".to_string())
        );
        assert_eq!(
            format_ruby_version("ruby 2.7.0p0 (2019-12-25 revision 647ee6f091) [x86_64-linux-musl]"),
            Some("v2.7.0".to_string())
        );

        Ok(())
    }
}
