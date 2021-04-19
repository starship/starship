use crate::config::ModuleConfig;
use indexmap::IndexMap;
use serde::{self, Serialize};
use starship_module_config_derive::ModuleConfig;

pub mod aws;
pub mod battery;
pub mod character;
pub mod cmake;
pub mod cmd_duration;
pub mod conda;
pub mod crystal;
pub mod custom;
pub mod dart;
pub mod directory;
pub mod docker_context;
pub mod dotnet;
pub mod elixir;
pub mod elm;
pub mod env_var;
pub mod erlang;
pub mod gcloud;
pub mod git_branch;
pub mod git_commit;
pub mod git_state;
pub mod git_status;
pub mod go;
pub mod helm;
pub mod hg_branch;
pub mod hostname;
pub mod java;
pub mod jobs;
pub mod julia;
pub mod kotlin;
pub mod kubernetes;
pub mod lua;
pub mod memory_usage;
pub mod nim;
pub mod nix_shell;
pub mod nodejs;
pub mod ocaml;
pub mod openstack;
pub mod package;
pub mod perl;
pub mod php;
pub mod purescript;
pub mod python;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod shell;
pub mod shlvl;
pub mod singularity;
mod starship_root;
pub mod status;
pub mod swift;
pub mod terraform;
pub mod time;
pub mod username;
pub mod vagrant;
pub mod vcsh;
pub mod vlang;
pub mod zig;

pub use starship_root::*;

#[derive(Default, Serialize, ModuleConfig, Clone)]
#[serde(default)]
pub struct FullConfig<'a> {
    // Root config
    pub format: &'a str,
    pub scan_timeout: u64,
    pub command_timeout: u64,
    pub add_newline: bool,
    // modules
    aws: aws::AwsConfig<'a>,
    battery: battery::BatteryDisplayConfig<'a>,
    character: character::CharacterConfig<'a>,
    cmake: cmake::CMakeConfig<'a>,
    cmd_duration: cmd_duration::CmdDurationConfig<'a>,
    conda: conda::CondaConfig<'a>,
    crystal: crystal::CrystalConfig<'a>,
    dart: dart::DartConfig<'a>,
    directory: directory::DirectoryConfig<'a>,
    docker_context: docker_context::DockerContextConfig<'a>,
    dotnet: dotnet::DotnetConfig<'a>,
    elixir: elixir::ElixirConfig<'a>,
    elm: elm::ElmConfig<'a>,
    env_var: env_var::EnvVarConfig<'a>,
    erlang: erlang::ErlangConfig<'a>,
    gcloud: gcloud::GcloudConfig<'a>,
    git_branch: git_branch::GitBranchConfig<'a>,
    git_commit: git_commit::GitCommitConfig<'a>,
    git_state: git_state::GitStateConfig<'a>,
    git_status: git_status::GitStatusConfig<'a>,
    golang: go::GoConfig<'a>,
    helm: helm::HelmConfig<'a>,
    hg_branch: hg_branch::HgBranchConfig<'a>,
    hostname: hostname::HostnameConfig<'a>,
    java: java::JavaConfig<'a>,
    jobs: jobs::JobsConfig<'a>,
    julia: julia::JuliaConfig<'a>,
    kotlin: kotlin::KotlinConfig<'a>,
    kubernetes: kubernetes::KubernetesConfig<'a>,
    lua: lua::LuaConfig<'a>,
    memory_usage: memory_usage::MemoryConfig<'a>,
    nim: nim::NimConfig<'a>,
    nix_shell: nix_shell::NixShellConfig<'a>,
    nodejs: nodejs::NodejsConfig<'a>,
    ocaml: ocaml::OCamlConfig<'a>,
    openstack: openstack::OspConfig<'a>,
    package: package::PackageConfig<'a>,
    perl: perl::PerlConfig<'a>,
    php: php::PhpConfig<'a>,
    purescript: purescript::PureScriptConfig<'a>,
    python: python::PythonConfig<'a>,
    ruby: ruby::RubyConfig<'a>,
    rust: rust::RustConfig<'a>,
    scala: scala::ScalaConfig<'a>,
    shell: shell::ShellConfig<'a>,
    shlvl: shlvl::ShLvlConfig<'a>,
    singularity: singularity::SingularityConfig<'a>,
    status: status::StatusConfig<'a>,
    swift: swift::SwiftConfig<'a>,
    terraform: terraform::TerraformConfig<'a>,
    time: time::TimeConfig<'a>,
    username: username::UsernameConfig<'a>,
    vlang: vlang::VLangConfig<'a>,
    vagrant: vagrant::VagrantConfig<'a>,
    zig: zig::ZigConfig<'a>,
    custom: IndexMap<String, custom::CustomConfig<'a>>,
}
