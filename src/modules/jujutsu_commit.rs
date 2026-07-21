use super::{Context, Module, ModuleConfig};

use crate::configs::jujutsu_commit::JujutsuCommitConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::jujutsu::get_jujutsu_commit_id;

/// Creates a module with the Jujutsu commit hash in the current directory
///
/// Will display the commit hash if the current directory is a jj repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu_commit");
    let config: JujutsuCommitConfig = JujutsuCommitConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let (commit_id, prefix_len) = get_jujutsu_commit_id(context)?;
    let remaining_len = config.commit_hash_length.saturating_sub(prefix_len);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                "prefix_style" => Some(Ok(config.prefix_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "commit" => Some(Ok(commit_id
                    .chars()
                    .take(config.commit_hash_length)
                    .collect::<String>())),
                "commit_prefix" => Some(Ok(commit_id.chars().take(prefix_len).collect::<String>())),
                "commit_suffix" => Some(Ok(commit_id
                    .chars()
                    .skip(prefix_len)
                    .take(remaining_len)
                    .collect::<String>())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jujutsu_commit`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
