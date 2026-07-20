use super::{Context, Module, ModuleConfig};

use crate::configs::jj_change::JJChangeConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the JJ change and commit in the current directory
///
/// Will display the JJ change hash truncated by default if the current directory is a JJ repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_change");
    let config = JJChangeConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let current = context.get_jj_repo()?.current_change(context)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "change" => Some("[$change_prefix]($prefix_style)[$change_suffix]($suffix_style)"),
                "commit" => Some("[$commit_prefix]($prefix_style)[$commit_suffix]($suffix_style)"),
                _ => None,
            })
            .map_style(|variable| match variable {
                "prefix_style" => Some(Ok(config.prefix_style)),
                "suffix_style" => Some(Ok(config.suffix_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "change_prefix" => prefix(current.change.as_ref(), current.change_shortest).map(Ok),
                "change_suffix" => suffix(
                    current.change.as_ref(),
                    current.change_shortest,
                    config.change_hash_length,
                )
                .map(Ok),
                "commit_prefix" => prefix(current.commit.as_ref(), current.commit_shortest).map(Ok),
                "commit_suffix" => suffix(
                    current.commit.as_ref(),
                    current.commit_shortest,
                    config.commit_hash_length,
                )
                .map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj_change`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn prefix(id: &str, prefix_size: u8) -> Option<&str> {
    id.get(..prefix_size.into()).filter(|s| !s.is_empty())
}

fn suffix(id: &str, prefix_size: u8, hash_length: u8) -> Option<&str> {
    let end = prefix_size.max(hash_length);
    id.get(prefix_size.into()..end.into())
        .filter(|s| !s.is_empty())
}
