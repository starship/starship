use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use futures::stream::StreamExt;
use jj_lib::backend::CommitId;
use jj_lib::config::{ConfigLayer, ConfigSource, StackedConfig};
use jj_lib::fileset::FilesetAliasesMap;
use jj_lib::id_prefix::IdPrefixContext;
use jj_lib::object_id::ObjectId as _;
use jj_lib::op_store::RefTarget;
use jj_lib::op_store::RemoteRef;
use jj_lib::repo::ReadonlyRepo;
use jj_lib::repo::Repo as _;
use jj_lib::repo_path::RepoPathUiConverter;
use jj_lib::revset::RevsetDiagnostics;
use jj_lib::revset::RevsetParseContext;
use jj_lib::revset::RevsetWorkspaceContext;
use jj_lib::revset::SymbolResolver;
use jj_lib::revset::UserRevsetExpression;
use jj_lib::revset::{self, ResolvedRevsetExpression};
use jj_lib::revset::{RevsetAliasesMap, RevsetExtensions};
use jj_lib::settings::UserSettings;
use jj_lib::str_util::StringExpression;
use jj_lib::workspace::{Workspace, default_working_copy_factories};
use pollster::FutureExt;

use crate::context::Context;

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
    fileset_aliases: FilesetAliasesMap,
    revset_aliases: RevsetAliasesMap,
    revset_extensions: RevsetExtensions,
}

impl JujutsuRepo {
    pub fn load(context: &Context<'_>) -> Option<Self> {
        let workspace_root = context.begin_ancestor_scan().set_folders(&[".jj"]).scan()?;
        let settings = load_settings(context, &workspace_root)?;

        let workspace = Workspace::load(
            &settings,
            &workspace_root,
            &jj_lib::repo::StoreFactories::default(),
            &default_working_copy_factories(),
        )
        .ok()?;

        let workspace_name = workspace.workspace_name().to_owned();
        let repo = workspace.repo_loader().load_at_head().block_on().ok()?;

        let fileset_aliases = load_fileset_aliases(&settings);
        let revset_aliases = load_revset_aliases(&settings);
        let revset_extensions = RevsetExtensions::default();

        Some(Self {
            repo,
            settings,
            workspace_name,
            workspace_root: workspace_root.to_path_buf(),
            fileset_aliases,
            revset_aliases,
            revset_extensions,
        })
    }

    pub fn repo(&self) -> &Arc<ReadonlyRepo> {
        &self.repo
    }

    pub fn workspace_name(&self) -> &jj_lib::ref_name::WorkspaceName {
        &self.workspace_name
    }

    pub fn resolve_revset_expression(
        &self,
        expr: Arc<UserRevsetExpression>,
    ) -> Option<Arc<ResolvedRevsetExpression>> {
        let resolver = SymbolResolver::new(
            self.repo.as_ref(),
            self.revset_extensions.symbol_resolvers(),
        );
        expr.resolve_user_expression(self.repo.as_ref(), &resolver)
            .ok()
    }

    pub fn parse_revset_expression(&self, expression: &str) -> Option<Arc<UserRevsetExpression>> {
        let mut diagnostics = RevsetDiagnostics::new();
        let path_converter = RepoPathUiConverter::Fs {
            cwd: self.workspace_root.to_path_buf(),
            base: self.workspace_root.to_path_buf(),
        };
        let context = RevsetParseContext {
            aliases_map: &self.revset_aliases,
            local_variables: HashMap::new(),
            user_email: self.settings.user_email(),
            date_pattern_context: chrono::Utc::now().fixed_offset().into(),
            default_ignored_remote: None,
            fileset_aliases_map: &self.fileset_aliases,
            extensions: &self.revset_extensions,
            workspace: Some(RevsetWorkspaceContext {
                path_converter: &path_converter,
                workspace_name: &self.workspace_name,
            }),
        };

        revset::parse(&mut diagnostics, expression, &context).ok()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct JujutsuRepoInfo {
    pub conflicted: bool,
    pub divergent: bool,
    pub hidden: bool,
    pub immutable: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct JujutsuBookmarkInfo {
    pub name: String,
    pub remote_ahead: usize,
    pub remote_behind: usize,
    pub is_tracked: bool,
    pub is_conflicted: bool,
}

pub fn get_jujutsu_change_id(ctx: &Context) -> Option<(String, usize)> {
    let repo = ctx.get_jujutsu_repo()?;

    let commit_id = working_copy_commit_id(repo)?;
    let commit = repo.repo().store().get_commit(&commit_id).ok()?;

    let default_log_expression =
        repo.parse_revset_expression("present(@) | ancestors(immutable_heads().., 2) | trunk()")?;
    let id_prefix_context = IdPrefixContext::new(Arc::new(RevsetExtensions::default()))
        .disambiguate_within(default_log_expression);
    let id_prefix_index = id_prefix_context.populate(repo.repo().as_ref()).ok()?;
    let prefix_len = id_prefix_index
        .shortest_change_prefix_len(repo.repo().as_ref(), &commit.change_id())
        .ok()?;

    Some((commit.change_id().reverse_hex(), prefix_len))
}

pub fn get_jujutsu_commit_id(ctx: &Context) -> Option<(String, usize)> {
    let repo = ctx.get_jujutsu_repo()?;

    let commit_id = working_copy_commit_id(repo)?;

    let default_log_expression =
        repo.parse_revset_expression("present(@) | ancestors(immutable_heads().., 2) | trunk()")?;
    let id_prefix_context = IdPrefixContext::new(Arc::new(RevsetExtensions::default()))
        .disambiguate_within(default_log_expression);
    let id_prefix_index = id_prefix_context.populate(repo.repo().as_ref()).ok()?;
    let prefix_len = id_prefix_index
        .shortest_commit_prefix_len(repo.repo().as_ref(), &commit_id)
        .ok()?;

    Some((commit_id.hex(), prefix_len))
}

pub(crate) fn get_jujutsu_bookmarks(
    ctx: &Context,
    find_closest: bool,
) -> Option<Vec<JujutsuBookmarkInfo>> {
    let repo = ctx.get_jujutsu_repo()?;
    let commit_id = working_copy_commit_id(repo)?;
    let tracked_bookmarks = tracked_bookmarks(repo.repo());

    let bookmarks = commit_bookmarks(repo.repo(), &commit_id, &tracked_bookmarks);

    if !bookmarks.is_empty() || !find_closest {
        return Some(bookmarks);
    }

    let closest_bookmark_commit_id = closest_bookmark_commit_id(repo)?;
    Some(commit_bookmarks(
        repo.repo(),
        &closest_bookmark_commit_id,
        &tracked_bookmarks,
    ))
}

pub fn get_jujutsu_state(ctx: &Context) -> Option<JujutsuRepoInfo> {
    let repo = ctx.get_jujutsu_repo()?;

    let commit_id = working_copy_commit_id(repo)?;
    let commit = repo.repo().store().get_commit(&commit_id).ok()?;

    let hidden = commit.is_hidden(repo.repo().as_ref()).ok()?;
    let divergent = repo
        .repo()
        .as_ref()
        .resolve_change_id(commit.change_id())
        .ok()
        .and_then(|targets| targets.map(|targets| targets.is_divergent()))
        .unwrap_or(false);
    let immutable = is_commit_immutable(repo, &commit_id).unwrap_or(false);

    Some(JujutsuRepoInfo {
        conflicted: commit.has_conflict(),
        divergent,
        hidden,
        immutable,
    })
}

fn working_copy_commit_id(repo: &JujutsuRepo) -> Option<CommitId> {
    repo.repo()
        .view()
        .get_wc_commit_id(repo.workspace_name())
        .cloned()
}

fn tracked_bookmarks(repo: &ReadonlyRepo) -> HashMap<String, (usize, usize)> {
    let mut tracked = HashMap::new();

    for (name, targets) in repo.view().bookmarks() {
        let local_target = targets.local_target;
        for (_, remote_ref) in targets.remote_refs {
            if let Some((ahead, behind)) = tracking_counts(repo, local_target, remote_ref)
                .ok()
                .flatten()
            {
                let counts = tracked
                    .entry(name.as_str().to_string())
                    .or_insert((ahead, behind));
                counts.0 = counts.0.max(ahead);
                counts.1 = counts.1.max(behind);
            }
        }
    }

    tracked
}

fn tracking_counts(
    repo: &dyn jj_lib::repo::Repo,
    local_target: &RefTarget,
    remote_ref: &RemoteRef,
) -> Result<Option<(usize, usize)>, jj_lib::revset::RevsetEvaluationError> {
    if !remote_ref.is_tracked() {
        return Ok(None);
    }

    let local_ids: Vec<CommitId> = local_target.added_ids().cloned().collect();
    let remote_ids: Vec<CommitId> = remote_ref.target.added_ids().cloned().collect();

    let ahead = revset::walk_revs(repo, &remote_ids, &local_ids)?
        .count_estimate()?
        .0;
    let behind = revset::walk_revs(repo, &local_ids, &remote_ids)?
        .count_estimate()?
        .0;

    Ok(Some((ahead, behind)))
}

fn commit_bookmarks(
    repo: &ReadonlyRepo,
    commit_id: &CommitId,
    tracked_bookmarks: &HashMap<String, (usize, usize)>,
) -> Vec<JujutsuBookmarkInfo> {
    let mut bookmarks = Vec::new();

    for (name, local_target) in repo.view().local_bookmarks_for_commit(commit_id) {
        let mut is_conflicted = false;
        let name_str = name.as_str().to_string();
        if local_target.has_conflict() {
            is_conflicted = true;
        }

        let remote_conflict = repo
            .view()
            .bookmarks()
            .filter(|(bookmark_name, _)| *bookmark_name == name)
            .flat_map(|(_, target)| target.remote_refs)
            .filter(|(_, remote_ref)| remote_ref.target.added_ids().any(|id| id == commit_id))
            .any(|(_, remote_ref)| remote_ref.target.has_conflict());

        if remote_conflict {
            is_conflicted = true;
        }

        let (ahead, behind, is_tracked) = tracked_bookmarks
            .get(&name_str)
            .map(|(ahead, behind)| (*ahead, *behind, true))
            .unwrap_or((0, 0, false));
        bookmarks.push(JujutsuBookmarkInfo {
            name: name_str,
            remote_ahead: ahead,
            remote_behind: behind,
            is_tracked,
            is_conflicted,
        });
    }

    bookmarks
}

fn closest_bookmark_commit_id(repo: &JujutsuRepo) -> Option<CommitId> {
    let working_copy = UserRevsetExpression::working_copy(repo.workspace_name().to_owned());
    let bookmarks = UserRevsetExpression::bookmarks(StringExpression::all());
    let expr = working_copy.ancestors().intersection(&bookmarks).heads();
    let resolved = repo.resolve_revset_expression(expr)?;
    let revset = resolved.evaluate(repo.repo().as_ref()).ok()?;

    revset
        .stream()
        .next()
        .block_on()
        .and_then(|entry| entry.ok())
}

fn is_commit_immutable(repo: &JujutsuRepo, commit_id: &CommitId) -> Option<bool> {
    let expr = repo.parse_revset_expression("immutable()")?;
    let resolved = repo.resolve_revset_expression(expr)?;
    let revset = resolved.evaluate(repo.repo().as_ref()).ok()?;
    let contains = revset.containing_fn();
    contains(commit_id).ok()
}

fn load_settings(context: &Context, workspace_root: &Path) -> Option<UserSettings> {
    let mut config = StackedConfig::with_defaults();
    let default_layer = ConfigLayer::parse(ConfigSource::Default, DEFAULT_REVSET_ALIASES).ok()?;
    config.add_layer(default_layer);

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

    if let Some(path) = repo_config_path(workspace_root)
        && let Err(error) = config.load_file(ConfigSource::Repo, path)
    {
        log::debug!("Failed to read JJ repo config file: {error}");
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

fn load_fileset_aliases(settings: &UserSettings) -> FilesetAliasesMap {
    let mut aliases = FilesetAliasesMap::new();
    let config = settings.config();
    for alias in config.table_keys("fileset-aliases") {
        if let Ok(value) = config.get::<String>(["fileset-aliases", alias])
            && let Err(error) = aliases.insert(alias, value, None)
        {
            log::debug!("Failed to load fileset alias '{alias}': {error}");
        }
    }
    aliases
}

fn load_revset_aliases(settings: &UserSettings) -> RevsetAliasesMap {
    let mut aliases = RevsetAliasesMap::new();
    let config = settings.config();
    for alias in config.table_keys("revset-aliases") {
        if let Ok(value) = config.get::<String>(["revset-aliases", alias])
            && let Err(error) = aliases.insert(alias, value, None)
        {
            log::debug!("Failed to load revset alias '{alias}': {error}");
        }
    }
    aliases
}
