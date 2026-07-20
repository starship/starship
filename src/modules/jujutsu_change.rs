use super::{Context, Module, ModuleConfig};

use crate::configs::jujutsu_change::JujutsuChangeConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::jujutsu::get_jujutsu_change_id;
use crate::modules::vcs;

/// Creates a module with the Jujutsu change ID in the current directory
///
/// Will display the change ID if the current directory is a jj repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu_change");
    let config: JujutsuChangeConfig = JujutsuChangeConfig::try_load(module.config);

    // We default to disabled=true, so we have to check after loading our config module.
    if config.disabled {
        return None;
    }

    // Only run in jj repositories
    vcs::discover_repo_root(context, vcs::Vcs::Jujutsu)?;

    let (change_id, prefix_len) = get_jujutsu_change_id(context)?;
    let remaining_len = config.change_id_length.saturating_sub(prefix_len);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "prefix_style" => Some(Ok(config.prefix_style)),
                "suffix_style" => Some(Ok(config.suffix_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "change_id" => Some(Ok(change_id
                    .chars()
                    .take(config.change_id_length)
                    .collect::<String>())),
                "change_prefix" => Some(Ok(change_id.chars().take(prefix_len).collect::<String>())),
                "change_suffix" => Some(Ok(change_id
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
            log::warn!("Error in module `jujutsu_change`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
