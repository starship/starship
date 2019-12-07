use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StarshipRootConfig<'a> {
    pub add_newline: bool,
    pub prompt_order: &'a str,
    pub scan_timeout: u64,
}

impl<'a> RootModuleConfig<'a> for StarshipRootConfig<'a> {
    fn new() -> Self {
        StarshipRootConfig {
            add_newline: true,
            // List of default prompt order
            // NOTE: If this const value is changed then Default prompt order subheading inside
            // prompt heading of config docs needs to be updated according to changes made here.
            prompt_order: "${username}${hostname}${kubernetes}${directory} \
                           ${git_branch}${git_commit}${git_state}${git_status}${hg_branch} \
                           ${package}${dotnet}${golang}${java} \
                           ${nodejs}${python}${ruby}${rust} \
                           ${nix_shell}${conda}${memory_usage}${aws} \
                           ${env_var}${cmd_duration}\n\
                           ${jobs}${battery}${time}${character}",
            scan_timeout: 30,
        }
    }
}
