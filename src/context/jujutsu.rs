use std::path::{Path, PathBuf};
use std::sync::Arc;

use jj_lib::config::{ConfigLayer, ConfigSource, StackedConfig};
use jj_lib::repo::ReadonlyRepo;
use jj_lib::repo::StoreFactories;
use jj_lib::revset::{RevsetAliasesMap, RevsetExtensions};
use jj_lib::settings::UserSettings;
use jj_lib::workspace::{Workspace, default_working_copy_factories};

use super::Context;

const DEFAULT_REVSET_ALIASES: &str = r#"
[revset-aliases]
'trunk()' = '''
latest(
  remote_bookmarks(exact:"main", exact:"origin") |
  remote_bookmarks(exact:"master", exact:"origin") |
  remote_bookmarks(exact:"trunk", exact:"origin") |
  remote_bookmarks(exact:"main", exact:"upstream") |
  remote_bookmarks(exact:"master", exact:"upstream") |
  remote_bookmarks(exact:"trunk", exact:"upstream") |
  root()
)
'''
'builtin_immutable_heads()' = 'trunk() | tags() | untracked_remote_bookmarks()'
'immutable_heads()' = 'builtin_immutable_heads()'
'immutable()' = '::(immutable_heads() | root())'
'mutable()' = '~immutable()'
"#;

pub struct JujutsuRepo {
    repo: Arc<ReadonlyRepo>,
    settings: UserSettings,
    workspace_name: jj_lib::ref_name::WorkspaceNameBuf,
    workspace_root: PathBuf,
    revset_aliases: RevsetAliasesMap,
    revset_extensions: RevsetExtensions,
}

impl JujutsuRepo {
    pub fn load(context: &Context) -> Option<Self> {
        let workspace_root = context.begin_ancestor_scan().set_folders(&[".jj"]).scan()?;
        let settings = load_settings(context, &workspace_root)?;

        let workspace = Workspace::load(
            &settings,
            &workspace_root,
            &StoreFactories::default(),
            &default_working_copy_factories(),
        )
        .ok()?;

        let workspace_name = workspace.workspace_name().to_owned();
        let repo = workspace.repo_loader().load_at_head().ok()?;

        let revset_aliases = load_revset_aliases(&settings);
        let revset_extensions = RevsetExtensions::default();

        Some(Self {
            repo,
            settings,
            workspace_name,
            workspace_root: workspace_root.to_path_buf(),
            revset_aliases,
            revset_extensions,
        })
    }

    pub fn repo(&self) -> &Arc<ReadonlyRepo> {
        &self.repo
    }

    pub fn settings(&self) -> &UserSettings {
        &self.settings
    }

    pub fn workspace_name(&self) -> &jj_lib::ref_name::WorkspaceName {
        &self.workspace_name
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn revset_aliases(&self) -> &RevsetAliasesMap {
        &self.revset_aliases
    }

    pub fn revset_extensions(&self) -> &RevsetExtensions {
        &self.revset_extensions
    }
}

fn load_settings(context: &Context, workspace_root: &Path) -> Option<UserSettings> {
    let mut config = StackedConfig::with_defaults();
    let default_layer = ConfigLayer::parse(ConfigSource::Default, DEFAULT_REVSET_ALIASES).ok()?;
    config.add_layer(default_layer);

    if let Some(path) = context.get_env("JJ_CONFIG") {
        if let Err(error) = config.load_file(ConfigSource::User, path) {
            log::debug!("Failed to read JJ_CONFIG file: {error}");
        }
    } else {
        for path in user_config_paths(context) {
            if let Err(error) = config.load_file(ConfigSource::User, path) {
                log::debug!("Failed to read JJ config file: {error}");
            }
        }
        for dir in user_config_dirs(context) {
            if let Err(error) = config.load_dir(ConfigSource::User, dir) {
                log::debug!("Failed to read JJ config dir: {error}");
            }
        }
    }

    if let Some(path) = repo_config_path(workspace_root) {
        if let Err(error) = config.load_file(ConfigSource::Repo, path) {
            log::debug!("Failed to read JJ repo config file: {error}");
        }
    }

    UserSettings::from_config(config).ok()
}

fn repo_config_path(workspace_root: &Path) -> Option<PathBuf> {
    let repo_config = workspace_root.join(".jj").join("repo").join("config.toml");
    repo_config.exists().then_some(repo_config)
}

fn user_config_paths(context: &Context) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(home) = context.get_home() {
        paths.push(home.join(".jjconfig.toml"));

        let config_home = context
            .get_env("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"));
        paths.push(config_home.join("jj").join("config.toml"));
    }

    paths
}

fn user_config_dirs(context: &Context) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(home) = context.get_home() {
        let config_home = context
            .get_env("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| home.join(".config"));
        paths.push(config_home.join("jj").join("conf.d"));
    }

    paths
}

fn load_revset_aliases(settings: &UserSettings) -> RevsetAliasesMap {
    let mut aliases = RevsetAliasesMap::new();
    let config = settings.config();
    for alias in config.table_keys("revset-aliases") {
        if let Ok(value) = config.get::<String>(["revset-aliases", alias]) {
            if let Err(error) = aliases.insert(alias, value) {
                log::debug!("Failed to load revset alias '{alias}': {error}");
            }
        }
    }
    aliases
}
