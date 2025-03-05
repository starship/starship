use std::borrow::Cow;
use std::path::Path;

use super::{Context, Module, ModuleConfig};

use crate::configs::vcs::{Vcs, VcsConfig};
use crate::formatter::string_formatter::StringFormatterError;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("vcs");
    let config = VcsConfig::try_load(module.config);

    if config.disabled || config.order.is_empty() {
        return None;
    }

    let vcs = config
        .order
        .into_iter()
        .find(|vcs| discover_repo_root(context, *vcs).is_some())?;

    let modules = match vcs {
        Vcs::Fossil => config.fossil_modules,
        Vcs::Git => config.git_modules,
        Vcs::Hg => config.hg_modules,
        Vcs::Pijul => config.pijul_modules,
    };

    if modules.is_empty() {
        return None;
    }

    let parsed = StringFormatter::new(modules).and_then(|formatter| {
        formatter
            .map_variables_to_segments(|variable| match variable {
                "vcs" => Some(Err(StringFormatterError::Custom(
                    "cannot recursively include the `vcs` module in itself".into(),
                ))),
                module => super::handle(module, context).map(|m| Ok(m.segments)),
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `vcs`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

pub fn discover_repo_root<'a>(context: &'a Context, vcs: Vcs) -> Option<Cow<'a, Path>> {
    let scan = context.begin_ancestor_scan();

    let scan = match vcs {
        Vcs::Fossil => scan.set_files(if cfg!(windows) {
            &["_FOSSIL_"]
        } else {
            &[".fslckout"]
        }),
        Vcs::Hg => scan.set_folders(&[".hg"]),
        Vcs::Pijul => scan.set_folders(&[".pijul"]),
        Vcs::Git => return context.get_repo().ok().map(|r| r.repo.path().into()),
    };

    scan.scan().map(Into::into)
}
