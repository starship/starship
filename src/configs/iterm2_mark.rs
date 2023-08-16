use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ITerm2MarkConfig<'a> {
    /// A vector of supported terminal application names.
    /// The iTerm2 set an environment variable named 'TERM_PROGRAM' when it starts.
    /// We check this environment variable to determine if this module should be activated.
    /// Currently, the value of TERM_PROGRAM is set to 'iTerm.app'. (see Default trait below)
    /// In case of changing, we let user to specify the value of TERM_PROGRAM.
    #[serde(borrow)]
    pub term_programs: Vec<&'a str>,

    /// Activate this module by force or not.
    /// Sometimes it is not reliable by the environment variable 'TERM_PROGRAM'
    /// to activate this module. For example, the iTerm.app failed to set the environment variable
    /// when remote login by ssh. Users could set the `force` flag to true in the config file under
    /// the 'iterm2_mark' section.
    /// Note the differences between `force` and `disabled`. When `disabled` is set to be true,
    /// this module will not run at all.
    pub force: bool,

    /// Disable this module or not.
    pub disabled: bool,
}

impl<'a> Default for ITerm2MarkConfig<'a> {
    fn default() -> Self {
        ITerm2MarkConfig {
            term_programs: vec!["iTerm.app"],
            force: false,
            disabled: false,
        }
    }
}
