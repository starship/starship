use super::{Context, Module, ModuleConfig};

use crate::configs::jj_status::JJStatusConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the JJ status in the current working directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("jj_status");
    let config = JJStatusConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    let current_change = context.get_jj_repo()?.current_change(context)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "all" => Some("$conflicted$description$hidden$immutable$added$copied$deleted$modified$renamed"),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "added" => current_change.status_added().then_some(Ok(config.added)),
                "copied" => current_change.status_copied().then_some(Ok(config.copied)),
                "deleted" => current_change
                    .status_deleted()
                    .then_some(Ok(config.deleted)),
                "modified" => current_change
                    .status_modified()
                    .then_some(Ok(config.modified)),
                "renamed" => current_change
                    .status_renamed()
                    .then_some(Ok(config.renamed)),
                "conflicted" => current_change.conflicted().then_some(Ok(config.conflicted)),
                "description" => Some(Ok(match current_change.description() {
                    true => config.description_present,
                    false => config.description_empty,
                })),
                "hidden" => current_change.hidden().then_some(Ok(config.hidden)),
                "immutable" => current_change.immutable().then_some(Ok(config.immutable)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `jj_status`:\n{error}");
            return None;
        }
    });

    Some(module)
}
