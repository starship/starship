use super::utils::format;
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
            .map_variables_to_segments(|variable| {
                let segments = match variable {
                    "added" => format::count(config.added, "jj_status.added", context, current_change.status.added),
                    "copied" => format::count(config.copied, "jj_status.copied", context, current_change.status.copied),
                    "deleted" => format::count(config.deleted, "jj_status.deleted", context, current_change.status.deleted),
                    "modified" => format::count(config.modified, "jj_status.modified", context, current_change.status.modified),
                    "renamed" => format::count(config.renamed, "jj_status.renamed", context, current_change.status.renamed),
                    "conflicted" => current_change.conflicted().then(|| format::symbol(config.conflicted, "jj_status.conflicted", context)).flatten(),
                    "description" => match current_change.description() {
                        true => format::symbol(config.description_present, "jj_status.description_present", context),
                        false => format::symbol(config.description_empty, "jj_status.description_empty", context),
                    },
                    "hidden" => current_change.hidden().then(|| format::symbol(config.hidden, "jj_status.hidden", context)).flatten(),
                    "immutable" => current_change.immutable().then(|| format::symbol(config.immutable, "jj_status.immutable", context)).flatten(),
                    _ => None,
                };

            segments.map(Ok)
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
