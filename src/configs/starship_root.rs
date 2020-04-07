use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StarshipRootConfig<'a> {
    pub format: &'a str,
    pub scan_timeout: u64,
}

impl<'a> RootModuleConfig<'a> for StarshipRootConfig<'a> {
    fn new() -> Self {
        StarshipRootConfig {
            // List of default prompt order
            // NOTE: If this const value is changed then Default prompt order subheading inside
            // prompt heading of config docs needs to be updated according to changes made here.
            format: "\n\
                $username\
                $hostname\
                $singularity\
                $kubernetes\
                $directory\
                $git_branch\
                $git_commit\
                $git_state\
                $git_status\
                $hg_branch\
                $docker_context\
                $package\
                $dotnet\
                $elixir\
                $elm\
                $golang\
                $haskell\
                $java\
                $julia\
                $nodejs\
                $php\
                $python\
                $ruby\
                $rust\
                $terraform\
                $nix_shell\
                $conda\
                $memory_usage\
                $aws\
                $env_var\
                $cmd_duration\
                \n\
                $jobs\
                $battery\
                $time\
                $character\
            ",
            scan_timeout: 30,
        }
    }
}
