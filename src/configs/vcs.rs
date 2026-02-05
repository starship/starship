//! The VCS module is a gathering of other Version Control System modules.
//!
//! It offers the option to order VCSes in a "discovery" order and will then use the `<vcs>.format`
//! option for it.
//!
//! Notably, this module is only a dispatcher: it offers nothing beyond "try these VCSes in this
//! order and use these modules for the first one found".
//!
//! Individual VCS modules are still configured at the toplevel starship configuration, they are
//! not included as subconfigurations of the current module.

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct VcsConfig<'a> {
    /// Order in which to discover VCSes.
    /// The first one found is the one used.
    pub order: Vec<&'a str>,
    /// Disables the VCS module.
    pub disabled: bool,
    /// Modules to use when Fossil is matched.
    ///
    /// They are configured separately at the top level.
    pub fossil_modules: &'a str,
    /// Modules to use when Git is matched.
    ///
    /// They are configured separately at the top level.
    pub git_modules: &'a str,
    /// Modules to use when Mercurial is matched.
    ///
    /// They are configured separately at the top level.
    pub hg_modules: &'a str, // NOTE: uses `hg` to correspond to existing `hg_branch` module
    /// Modules to use when Pijul is matched.
    ///
    /// They are configured separately at the top level.
    pub pijul_modules: &'a str,
}

impl Default for VcsConfig<'_> {
    fn default() -> Self {
        VcsConfig {
            order: vec!["git", "hg", "pijul", "fossil"],
            disabled: false,
            fossil_modules: "$fossil_branch$fossil_metrics",
            git_modules: "$git_branch$git_commit$git_state$git_metrics$git_status",
            hg_modules: "$hg_branch$hg_state",
            pijul_modules: "$pijul_channel",
        }
    }
}
