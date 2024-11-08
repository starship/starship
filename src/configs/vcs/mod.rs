use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

pub mod jujutsu;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct VcsConfig<'a> {
    pub disabled: bool,
    pub order: IndexSet<Vcs>,
    #[serde(borrow, default)]
    pub jujutsu: jujutsu::JjConfig<'a>,
}

impl<'a> Default for VcsConfig<'a> {
    fn default() -> Self {
        Self {
            disabled: true,
            order: [
                // TODO(poliorcetics): make the default be only Git, avoiding costs for the
                // vast majority of users.
                Vcs::Jujutsu,
            ]
            .into_iter()
            .collect(),
            jujutsu: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(rename_all = "lowercase")]
pub enum Vcs {
    Jujutsu,
}

impl Vcs {
    /// Marker file or directory indicating the VCS is active.
    pub const fn marker(&self) -> &'static str {
        match self {
            Self::Jujutsu => ".jj",
        }
    }
}

impl std::fmt::Display for Vcs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Jujutsu => "jujutsu",
        };

        f.write_str(s)
    }
}
