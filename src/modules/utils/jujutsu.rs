use crate::config::ModuleConfig;
use crate::context::Context;
use crate::modules::vcs;
use jj_lib::backend::CommitId;
use jj_lib::config::{ConfigLayer, ConfigSource, StackedConfig};
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
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

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

pub(crate) struct JujutsuRepo {
    repo: Arc<ReadonlyRepo>,
    settings: UserSettings,
    workspace_name: jj_lib::ref_name::WorkspaceNameBuf,
    workspace_root: PathBuf,
    revset_aliases: RevsetAliasesMap,
    revset_extensions: RevsetExtensions,
}

impl JujutsuRepo {
    pub(crate) fn load(context: &Context) -> Option<Self> {
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

    pub(crate) fn repo(&self) -> &Arc<ReadonlyRepo> {
        &self.repo
    }

    pub(crate) fn settings(&self) -> &UserSettings {
        &self.settings
    }

    pub(crate) fn workspace_name(&self) -> &jj_lib::ref_name::WorkspaceName {
        &self.workspace_name
    }

    pub(crate) fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub(crate) fn revset_aliases(&self) -> &RevsetAliasesMap {
        &self.revset_aliases
    }

    pub(crate) fn revset_extensions(&self) -> &RevsetExtensions {
        &self.revset_extensions
    }
}

#[derive(Debug, Clone)]
pub(crate) struct JjRepoInfo {
    pub change_id: String,
    pub commit_id: String,
    pub bookmarks: Vec<BookmarkInfo>,
    pub conflicted: bool,
    pub divergent: bool,
    pub hidden: bool,
    pub immutable: bool,
    pub bookmark_conflicted: bool,
}

pub(crate) struct JjClosestBookmarksInfo {
    pub bookmarks: Vec<BookmarkInfo>,
}

#[derive(Debug, Clone)]
pub(crate) struct BookmarkInfo {
    pub name: String,
    pub remote_ahead: usize,
    pub remote_behind: usize,
    pub is_tracked: bool,
}

pub(crate) fn get_jujutsu_info(ctx: &Context, _ignore_working_copy: &bool) -> Option<JjRepoInfo> {
    vcs::discover_repo_root(ctx, vcs::Vcs::Jujutsu)?;
    let repo = ctx.get_jujutsu_repo()?;
    let change_id_length = crate::configs::jujutsu_change::JujutsuChangeConfig::try_load(
        ctx.config.get_module_config("jujutsu_change"),
    )
    .change_id_length;
    let commit_hash_length = crate::configs::jujutsu_commit::JujutsuCommitConfig::try_load(
        ctx.config.get_module_config("jujutsu_commit"),
    )
    .commit_hash_length;

    let commit_id = working_copy_commit_id(repo)?;
    let commit = repo.repo().store().get_commit(&commit_id).ok()?;

    let change_id = shorten_id(&commit.change_id().reverse_hex(), change_id_length);
    let commit_id_short = shorten_id(&commit.id().hex(), commit_hash_length);

    let tracked_bookmarks = tracked_bookmarks(repo.repo());
    let (bookmarks, bookmark_conflicted) =
        commit_bookmarks(repo.repo(), &commit_id, &tracked_bookmarks);

    let hidden = commit.is_hidden(repo.repo().as_ref()).ok()?;
    let divergent = repo
        .repo()
        .as_ref()
        .resolve_change_id(commit.change_id())
        .ok()
        .and_then(|targets| targets.map(|targets| targets.is_divergent()))
        .unwrap_or(false);
    let immutable = is_commit_immutable(repo, &commit_id).unwrap_or(false);

    Some(JjRepoInfo {
        change_id,
        commit_id: commit_id_short,
        bookmarks,
        conflicted: commit.has_conflict(),
        divergent,
        hidden,
        immutable,
        bookmark_conflicted,
    })
}

pub(crate) fn get_closest_jujutsu_bookmarks_info(
    ctx: &Context,
    _ignore_working_copy: &bool,
) -> Option<JjClosestBookmarksInfo> {
    vcs::discover_repo_root(ctx, vcs::Vcs::Jujutsu)?;
    let repo = ctx.get_jujutsu_repo()?;
    let commit_id = closest_bookmark_commit_id(repo)?;
    let tracked_bookmarks = tracked_bookmarks(repo.repo());
    let (bookmarks, _) = commit_bookmarks(repo.repo(), &commit_id, &tracked_bookmarks);

    Some(JjClosestBookmarksInfo { bookmarks })
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
) -> (Vec<BookmarkInfo>, bool) {
    let mut bookmark_conflicted = false;
    let mut bookmarks = Vec::new();

    for (name, local_target) in repo.view().local_bookmarks_for_commit(commit_id) {
        let name_str = name.as_str().to_string();
        if local_target.has_conflict() {
            bookmark_conflicted = true;
        }

        let remote_conflict = repo
            .view()
            .bookmarks()
            .filter(|(bookmark_name, _)| *bookmark_name == name)
            .flat_map(|(_, target)| target.remote_refs)
            .filter(|(_, remote_ref)| remote_ref.target.added_ids().any(|id| id == commit_id))
            .any(|(_, remote_ref)| remote_ref.target.has_conflict());

        if remote_conflict {
            bookmark_conflicted = true;
        }

        let (ahead, behind, is_tracked) = tracked_bookmarks
            .get(&name_str)
            .map(|(ahead, behind)| (*ahead, *behind, true))
            .unwrap_or((0, 0, false));
        bookmarks.push(BookmarkInfo {
            name: name_str,
            remote_ahead: ahead,
            remote_behind: behind,
            is_tracked,
        });
    }

    (bookmarks, bookmark_conflicted)
}

fn closest_bookmark_commit_id(repo: &JujutsuRepo) -> Option<CommitId> {
    let working_copy = UserRevsetExpression::working_copy(repo.workspace_name().to_owned());
    let bookmarks = UserRevsetExpression::bookmarks(StringExpression::all());
    let expr = working_copy.ancestors().intersection(&bookmarks).heads();
    let resolved = resolve_revset_expression(repo, expr)?;
    let revset = resolved.evaluate(repo.repo().as_ref()).ok()?;

    revset.iter().find_map(|entry| entry.ok())
}

fn resolve_revset_expression(
    repo: &JujutsuRepo,
    expr: Arc<UserRevsetExpression>,
) -> Option<Arc<ResolvedRevsetExpression>> {
    let resolver = SymbolResolver::new(
        repo.repo().as_ref(),
        repo.revset_extensions().symbol_resolvers(),
    );
    expr.resolve_user_expression(repo.repo().as_ref(), &resolver)
        .ok()
}

fn parse_revset_expression(
    repo: &JujutsuRepo,
    expression: &str,
) -> Option<Arc<UserRevsetExpression>> {
    let mut diagnostics = RevsetDiagnostics::new();
    let path_converter = RepoPathUiConverter::Fs {
        cwd: repo.workspace_root().to_path_buf(),
        base: repo.workspace_root().to_path_buf(),
    };
    let context = RevsetParseContext {
        aliases_map: repo.revset_aliases(),
        local_variables: HashMap::new(),
        user_email: repo.settings().user_email(),
        date_pattern_context: chrono::Utc::now().fixed_offset().into(),
        default_ignored_remote: None,
        use_glob_by_default: true,
        extensions: repo.revset_extensions(),
        workspace: Some(RevsetWorkspaceContext {
            path_converter: &path_converter,
            workspace_name: repo.workspace_name(),
        }),
    };

    revset::parse(&mut diagnostics, expression, &context).ok()
}

fn is_commit_immutable(repo: &JujutsuRepo, commit_id: &CommitId) -> Option<bool> {
    let expr = parse_revset_expression(repo, "immutable()")?;
    let resolved = resolve_revset_expression(repo, expr)?;
    let revset = resolved.evaluate(repo.repo().as_ref()).ok()?;
    let contains = revset.containing_fn();
    contains(commit_id).ok()
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

fn shorten_id(id: &str, length: usize) -> String {
    if length == 0 {
        return String::new();
    }
    id.chars().take(length).collect()
}
