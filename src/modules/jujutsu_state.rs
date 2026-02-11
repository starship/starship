use super::{Context, Module, ModuleConfig};

use crate::configs::jujutsu_state::JujutsuStateConfig;
use crate::formatter::StringFormatter;
use crate::modules::utils::jujutsu::get_jujutsu_info;
use crate::modules::vcs;

/// Creates a module with the Jujutsu state indicators in the current directory
///
/// Will display state indicators (conflicted, divergent, hidden, working copy conflicts)
/// if the current directory is a jj repo
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jujutsu_state");
    let config: JujutsuStateConfig = JujutsuStateConfig::try_load(module.config);

    // We default to disabled=true, so we have to check after loading our config module.
    if config.disabled {
        return None;
    }

    // Only run in jj repositories
    vcs::discover_repo_root(context, vcs::Vcs::Jujutsu)?;

    let jujutsu_info = get_jujutsu_info(context)?;

    if !jujutsu_info.conflicted
        && !jujutsu_info.divergent
        && !jujutsu_info.hidden
        && !jujutsu_info.immutable
    {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                "conflicted_style" => Some(Ok(config.conflicted_style)),
                "divergent_style" => Some(Ok(config.divergent_style)),
                "hidden_style" => Some(Ok(config.hidden_style)),
                "immutable_style" => Some(Ok(config.immutable_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "conflicted" => {
                    if jujutsu_info.conflicted {
                        Some(Ok(config.conflicted))
                    } else {
                        None
                    }
                }
                "divergent" => {
                    if jujutsu_info.divergent {
                        Some(Ok(config.divergent))
                    } else {
                        None
                    }
                }
                "hidden" => {
                    if jujutsu_info.hidden {
                        Some(Ok(config.hidden))
                    } else {
                        None
                    }
                }
                "immutable" => {
                    if jujutsu_info.immutable {
                        Some(Ok(config.immutable))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jujutsu_state`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
