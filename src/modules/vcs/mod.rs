//! The Version Control System (VCS) module exposes information from the currently active VCS,
//! trying them in a preconfigured order.
//!
//! This allows exposing information from only one VCS when several are present, as can be the case
//! for [colocated Git repos in Jujutsu][coloc].
//! It also makes reusing already parsed repository information easier.
//!
//! [coloc]: https://martinvonz.github.io/jj/latest/git-compatibility/#co-located-jujutsugit-repos

use std::path::{Path, PathBuf};

use crate::configs::vcs::{Vcs, VcsConfig};
use crate::formatter::StringFormatter;
use crate::segment::Segment;

use super::{Context, Module, ModuleConfig};

pub mod jujutsu;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("vcs");
    let config = VcsConfig::try_load(module.config);

    if config.disabled || config.order.is_empty() {
        return None;
    }

    let (root, vcs) = find_vcs_root(context, &config)?;

    let parsed = match vcs {
        Vcs::Jujutsu => jujutsu::segments(context, root, &config.jujutsu)?,
    };

    module.set_segments(match parsed {
        Ok(segments) if segments.is_empty() => return None,
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `vcs.{vcs}`:\n{error}");
            return None;
        }
    });

    Some(module)
}

/// Find the closest VCS root marker by searching upwards.
fn find_vcs_root<'a>(context: &'a Context, config: &VcsConfig<'a>) -> Option<(&'a Path, Vcs)> {
    let current_dir = &context.current_dir;

    // We want to avoid reallocations during the search so we pre-allocate a buffer with enough
    // capacity to hold the longest path + any marker (we could find the length of the longest
    // marker programmatically but that would actually cost more than preallocating a few bytes too many).
    let mut buf = PathBuf::with_capacity(current_dir.capacity() + 15);

    for dir in current_dir.ancestors() {
        // Then for each dir, we do `<dir>`, clearing in case `dir` is not an absolute path for
        // some reason
        buf.clear();
        buf.push(dir);

        for &vcs in &config.order {
            // Then we push, so it becomes `<dir>/<marker>`
            buf.push(vcs.marker());

            if buf.exists() {
                // In case we find it, we return the VCS but also the root dir: we already did the
                // work of finding it, we can give it as a parameter to the called command or
                // library so it can avoid doing its own search.
                return Some((dir, vcs));
            }

            // Remove the current marker to prepare for the next one
            buf.pop();
        }
    }

    None
}

fn format_text<F>(
    format_str: &str,
    config_path: &str,
    context: &Context,
    mapper: F,
) -> Option<Vec<Segment>>
where
    F: Fn(&str) -> Option<String> + Send + Sync,
{
    if let Ok(formatter) = StringFormatter::new(format_str) {
        formatter
            .map(|variable| mapper(variable).map(Ok))
            .parse(None, Some(context))
            .ok()
    } else {
        log::warn!("Error parsing format string `vcs.{}`", config_path);
        None
    }
}

fn format_count(
    format_str: &str,
    config_path: &str,
    context: &Context,
    count: usize,
) -> Option<Vec<Segment>> {
    if count == 0 {
        return None;
    }

    format_text(
        format_str,
        config_path,
        context,
        |variable| match variable {
            "count" => Some(count.to_string()),
            _ => None,
        },
    )
}
