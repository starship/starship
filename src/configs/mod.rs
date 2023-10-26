use indexmap::IndexMap;
use serde::{self, Deserialize, Serialize};

pub mod aws;
pub mod azure;
pub mod battery;
pub mod buf;
pub mod bun;
pub mod c;
pub mod character;
pub mod cmake;
pub mod cmd_duration;
pub mod cobol;
pub mod conda;
pub mod container;
pub mod crystal;
pub mod custom;
pub mod daml;
pub mod dart;
pub mod deno;
pub mod directory;
pub mod docker_context;
pub mod dotnet;
pub mod elixir;
pub mod elm;
pub mod env_var;
pub mod erlang;
pub mod fennel;
pub mod fill;
pub mod fossil_branch;
pub mod fossil_metrics;
pub mod gcloud;
pub mod git_branch;
pub mod git_commit;
pub mod git_metrics;
pub mod git_state;
pub mod git_status;
pub mod go;
pub mod gradle;
pub mod guix_shell;
pub mod haskell;
pub mod haxe;
pub mod helm;
pub mod hg_branch;
pub mod hostname;
pub mod java;
pub mod jobs;
pub mod julia;
pub mod kotlin;
pub mod kubernetes;
pub mod line_break;
pub mod localip;
pub mod lua;
pub mod memory_usage;
pub mod meson;
pub mod nim;
pub mod nix_shell;
pub mod nodejs;
pub mod ocaml;
pub mod opa;
pub mod openstack;
pub mod os;
pub mod package;
pub mod perl;
pub mod php;
pub mod pijul_channel;
pub mod pulumi;
pub mod purescript;
pub mod python;
pub mod raku;
pub mod red;
pub mod rlang;
pub mod ruby;
pub mod rust;
pub mod scala;
pub mod shell;
pub mod shlvl;
pub mod singularity;
pub mod solidity;
pub mod spack;
mod starship_root;
pub mod status;
pub mod sudo;
pub mod swift;
pub mod terraform;
pub mod time;
pub mod username;
pub mod v;
pub mod vagrant;
pub mod vcsh;
pub mod zig;
pub mod typst;

pub use starship_root::*;

#[derive(Serialize, Deserialize, Clone, Default)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct FullConfig<'a> {
    // Meta
    #[serde(rename = "$schema")]
    schema: String,
    // Root config
    #[serde(flatten)]
    root: StarshipRootConfig,
    // modules
    #[serde(borrow)]
    aws: aws::AwsConfig<'a>,
    #[serde(borrow)]
    azure: azure::AzureConfig<'a>,
    #[serde(borrow)]
    battery: battery::BatteryConfig<'a>,
    #[serde(borrow)]
    buf: buf::BufConfig<'a>,
    #[serde(borrow)]
    bun: bun::BunConfig<'a>,
    #[serde(borrow)]
    c: c::CConfig<'a>,
    #[serde(borrow)]
    character: character::CharacterConfig<'a>,
    #[serde(borrow)]
    cmake: cmake::CMakeConfig<'a>,
    #[serde(borrow)]
    cmd_duration: cmd_duration::CmdDurationConfig<'a>,
    #[serde(borrow)]
    cobol: cobol::CobolConfig<'a>,
    #[serde(borrow)]
    conda: conda::CondaConfig<'a>,
    #[serde(borrow)]
    container: container::ContainerConfig<'a>,
    #[serde(borrow)]
    crystal: crystal::CrystalConfig<'a>,
    #[serde(borrow)]
    daml: daml::DamlConfig<'a>,
    #[serde(borrow)]
    dart: dart::DartConfig<'a>,
    #[serde(borrow)]
    deno: deno::DenoConfig<'a>,
    #[serde(borrow)]
    directory: directory::DirectoryConfig<'a>,
    #[serde(borrow)]
    docker_context: docker_context::DockerContextConfig<'a>,
    #[serde(borrow)]
    dotnet: dotnet::DotnetConfig<'a>,
    #[serde(borrow)]
    elixir: elixir::ElixirConfig<'a>,
    #[serde(borrow)]
    elm: elm::ElmConfig<'a>,
    #[serde(borrow)]
    env_var: IndexMap<String, env_var::EnvVarConfig<'a>>,
    #[serde(borrow)]
    erlang: erlang::ErlangConfig<'a>,
    #[serde(borrow)]
    fennel: fennel::FennelConfig<'a>,
    #[serde(borrow)]
    fill: fill::FillConfig<'a>,
    #[serde(borrow)]
    fossil_branch: fossil_branch::FossilBranchConfig<'a>,
    #[serde(borrow)]
    fossil_metrics: fossil_metrics::FossilMetricsConfig<'a>,
    #[serde(borrow)]
    gcloud: gcloud::GcloudConfig<'a>,
    #[serde(borrow)]
    git_branch: git_branch::GitBranchConfig<'a>,
    #[serde(borrow)]
    git_commit: git_commit::GitCommitConfig<'a>,
    #[serde(borrow)]
    git_metrics: git_metrics::GitMetricsConfig<'a>,
    #[serde(borrow)]
    git_state: git_state::GitStateConfig<'a>,
    #[serde(borrow)]
    git_status: git_status::GitStatusConfig<'a>,
    #[serde(borrow)]
    golang: go::GoConfig<'a>,
    #[serde(borrow)]
    gradle: gradle::GradleConfig<'a>,
    #[serde(borrow)]
    guix_shell: guix_shell::GuixShellConfig<'a>,
    #[serde(borrow)]
    haskell: haskell::HaskellConfig<'a>,
    #[serde(borrow)]
    haxe: haxe::HaxeConfig<'a>,
    #[serde(borrow)]
    helm: helm::HelmConfig<'a>,
    #[serde(borrow)]
    hg_branch: hg_branch::HgBranchConfig<'a>,
    #[serde(borrow)]
    hostname: hostname::HostnameConfig<'a>,
    #[serde(borrow)]
    java: java::JavaConfig<'a>,
    #[serde(borrow)]
    jobs: jobs::JobsConfig<'a>,
    #[serde(borrow)]
    julia: julia::JuliaConfig<'a>,
    #[serde(borrow)]
    kotlin: kotlin::KotlinConfig<'a>,
    #[serde(borrow)]
    kubernetes: kubernetes::KubernetesConfig<'a>,
    line_break: line_break::LineBreakConfig,
    #[serde(borrow)]
    localip: localip::LocalipConfig<'a>,
    #[serde(borrow)]
    lua: lua::LuaConfig<'a>,
    #[serde(borrow)]
    memory_usage: memory_usage::MemoryConfig<'a>,
    #[serde(borrow)]
    meson: meson::MesonConfig<'a>,
    #[serde(borrow)]
    nim: nim::NimConfig<'a>,
    #[serde(borrow)]
    nix_shell: nix_shell::NixShellConfig<'a>,
    #[serde(borrow)]
    nodejs: nodejs::NodejsConfig<'a>,
    #[serde(borrow)]
    ocaml: ocaml::OCamlConfig<'a>,
    #[serde(borrow)]
    opa: opa::OpaConfig<'a>,
    #[serde(borrow)]
    openstack: openstack::OspConfig<'a>,
    #[serde(borrow)]
    os: os::OSConfig<'a>,
    #[serde(borrow)]
    package: package::PackageConfig<'a>,
    #[serde(borrow)]
    perl: perl::PerlConfig<'a>,
    #[serde(borrow)]
    php: php::PhpConfig<'a>,
    #[serde(borrow)]
    pijul_channel: pijul_channel::PijulConfig<'a>,
    #[serde(borrow)]
    pulumi: pulumi::PulumiConfig<'a>,
    #[serde(borrow)]
    purescript: purescript::PureScriptConfig<'a>,
    #[serde(borrow)]
    python: python::PythonConfig<'a>,
    #[serde(borrow)]
    raku: raku::RakuConfig<'a>,
    #[serde(borrow)]
    red: red::RedConfig<'a>,
    #[serde(borrow)]
    rlang: rlang::RLangConfig<'a>,
    #[serde(borrow)]
    ruby: ruby::RubyConfig<'a>,
    #[serde(borrow)]
    rust: rust::RustConfig<'a>,
    #[serde(borrow)]
    scala: scala::ScalaConfig<'a>,
    #[serde(borrow)]
    shell: shell::ShellConfig<'a>,
    #[serde(borrow)]
    shlvl: shlvl::ShLvlConfig<'a>,
    #[serde(borrow)]
    singularity: singularity::SingularityConfig<'a>,
    #[serde(borrow)]
    solidity: solidity::SolidityConfig<'a>,
    #[serde(borrow)]
    spack: spack::SpackConfig<'a>,
    #[serde(borrow)]
    status: status::StatusConfig<'a>,
    #[serde(borrow)]
    sudo: sudo::SudoConfig<'a>,
    #[serde(borrow)]
    swift: swift::SwiftConfig<'a>,
    #[serde(borrow)]
    terraform: terraform::TerraformConfig<'a>,
    #[serde(borrow)]
    time: time::TimeConfig<'a>,
    #[serde(borrow)]
    typst: typst::TypstConfig<'a>,
    #[serde(borrow)]
    username: username::UsernameConfig<'a>,
    #[serde(borrow)]
    vagrant: vagrant::VagrantConfig<'a>,
    #[serde(borrow)]
    vcsh: vcsh::VcshConfig<'a>,
    #[serde(borrow)]
    vlang: v::VConfig<'a>,
    #[serde(borrow)]
    zig: zig::ZigConfig<'a>,
    #[serde(borrow)]
    custom: IndexMap<String, custom::CustomConfig<'a>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::module::ALL_MODULES;
    use toml::value::Value;

    #[test]
    fn test_all_modules_in_full_config() {
        let full_cfg = Value::try_from(FullConfig::default()).unwrap();
        let cfg_table = full_cfg.as_table().unwrap();
        for module in ALL_MODULES {
            assert!(cfg_table.contains_key(*module));
        }
    }
}
