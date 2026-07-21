use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use jj_lib::backend::CommitId;
use jj_lib::config::{ConfigLayer, ConfigSource, StackedConfig};
use jj_lib::fileset::FilesetAliasesMap;
use jj_lib::id_prefix::IdPrefixContext;
use jj_lib::object_id::ObjectId;
use jj_lib::op_store::{RefTarget, RemoteRef};
use jj_lib::ref_name::{RefName, WorkspaceNameBuf};
use jj_lib::repo::{ReadonlyRepo, Repo, StoreFactories};
use jj_lib::repo_path::RepoPathUiConverter;
use jj_lib::revset;
use jj_lib::revset::{
    ResolvedRevsetExpression, RevsetAliasesMap, RevsetDiagnostics, RevsetExtensions,
    RevsetParseContext, RevsetWorkspaceContext, SymbolResolver, UserRevsetExpression,
};
use jj_lib::settings::UserSettings;
use jj_lib::str_util::StringMatcher;
use jj_lib::view::View;
use jj_lib::workspace::{Workspace, default_working_copy_factories};
use pollster::FutureExt;

use crate::context::Context;
use crate::modules::vcs;

pub struct JujutsuRepo {
    repo: Arc<ReadonlyRepo>,
    settings: UserSettings,
    workspace_name: WorkspaceNameBuf,
    workspace_root: PathBuf,
    fileset_aliases: FilesetAliasesMap,
    revset_aliases: RevsetAliasesMap,
    revset_extensions: RevsetExtensions,
}

impl JujutsuRepo {
    pub fn load(context: &Context<'_>) -> Option<Self> {
        let workspace_root = vcs::discover_repo_root(context, vcs::Vcs::Jujutsu)?;
        let settings = create_minimal_settings()?;

        let workspace = Workspace::load(
            &settings,
            &workspace_root,
            &StoreFactories::default(),
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

    fn get_working_copy_commit_id(&self) -> Option<CommitId> {
        self.repo
            .view()
            .get_wc_commit_id(&self.workspace_name)
            .cloned()
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
pub(crate) struct JujutsuChangeInfo {
    pub change_id: String,
    pub prefix_len: usize,
    pub description: String,
}

#[derive(Debug, Clone)]
pub(crate) struct JujutsuRepoInfo {
    pub conflicted: bool,
    pub divergent: bool,
    pub empty: bool,
    pub hidden: bool,
    pub immutable: bool,
    pub missing_description: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct JujutsuBookmarkInfo {
    pub name: String,
    pub remote_ahead: usize,
    pub remote_behind: usize,
    pub is_tracked: bool,
    pub distance: usize,
}

pub fn get_jujutsu_change_id(ctx: &Context) -> Option<JujutsuChangeInfo> {
    let repo = ctx.get_jujutsu_repo()?;

    let commit_id = repo.get_working_copy_commit_id()?;
    let commit = repo.repo().store().get_commit(&commit_id).ok()?;
    let prefix_len = repo
        .repo()
        .shortest_unique_change_id_prefix_len(commit.change_id())
        .ok()?;

    Some(JujutsuChangeInfo {
        change_id: commit.change_id().reverse_hex(),
        prefix_len,
        description: commit.description().to_owned(),
    })
}

pub fn get_jujutsu_commit_id(ctx: &Context) -> Option<(String, usize)> {
    let repo = ctx.get_jujutsu_repo()?;

    let commit_id = repo.get_working_copy_commit_id()?;

    // TODO - is this worth the effort or can it be done without revsets?
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

pub fn get_jujutsu_bookmarks(ctx: &Context, max_depth: usize) -> Option<Vec<JujutsuBookmarkInfo>> {
    let repo = ctx.get_jujutsu_repo()?;
    let commit_id = repo.get_working_copy_commit_id()?;

    let repo = repo.repo().as_ref();
    let view = repo.view();
    let mut bookmarks = view
        .local_bookmarks_for_commit(&commit_id)
        .map(|(name, target)| collect_bookmark_details(name, target, repo, 0))
        .collect::<Vec<_>>();

    if max_depth > 0 {
        let ancestors = find_ancestor_bookmarks(repo, view, &commit_id, max_depth)?;
        bookmarks.extend(ancestors);
    }

    Some(bookmarks)
}

pub fn get_jujutsu_state(ctx: &Context) -> Option<JujutsuRepoInfo> {
    let repo = ctx.get_jujutsu_repo()?;

    let commit_id = repo.get_working_copy_commit_id()?;
    let commit = repo.repo().store().get_commit(&commit_id).ok()?;

    let hidden = commit.is_hidden(repo.repo().as_ref()).unwrap_or(false);
    let divergent = repo
        .repo()
        .as_ref()
        .resolve_change_id(commit.change_id())
        .ok()
        .and_then(|targets| targets.map(|targets| targets.is_divergent()))
        .unwrap_or(false);
    let empty = commit
        .is_empty(repo.repo().as_ref())
        .block_on()
        .unwrap_or(false);
    let immutable_heads = find_immutable_heads(repo.repo().view());
    let immutable = immutable_heads.contains(&commit_id);

    Some(JujutsuRepoInfo {
        conflicted: commit.has_conflict(),
        divergent,
        empty,
        hidden,
        immutable,
        missing_description: commit.description().is_empty() && !empty,
    })
}

fn collect_bookmark_details(
    name: &RefName,
    target: &RefTarget,
    repo: &dyn Repo,
    depth: usize,
) -> JujutsuBookmarkInfo {
    let name_str = name.as_str().to_string();

    let matching_bookmarks = repo
        .view()
        .bookmarks()
        .filter(|(bookmark_name, _)| *bookmark_name == name)
        .flat_map(|(_, target)| target.remote_refs)
        .collect::<Vec<_>>();

    let mut ahead_behind = None;
    for (_, remote_ref) in matching_bookmarks {
        if let Some((ahead, behind)) = tracking_counts(repo, target, remote_ref).ok().flatten() {
            match &ahead_behind {
                None => ahead_behind = Some((ahead, behind)),
                Some((old_ahead, old_behind)) => {
                    ahead_behind = Some((*old_ahead.max(&ahead), *old_behind.max(&behind)));
                }
            }
        }
    }

    let (ahead, behind, is_tracked) = ahead_behind
        .map(|(ahead, behind)| (ahead, behind, true))
        .unwrap_or((0, 0, false));

    JujutsuBookmarkInfo {
        name: name_str,
        remote_ahead: ahead,
        remote_behind: behind,
        is_tracked,
        distance: depth,
    }
}

/// Find immutable head commits (trunk + tags + untracked remote bookmarks)
/// Mirrors jj's `builtin_immutable_heads()` without revset evaluation
fn find_immutable_heads(view: &View) -> HashSet<CommitId> {
    let mut immutable = HashSet::new();

    // Single pass over all remote bookmarks
    for (symbol, remote_ref) in
        view.remote_bookmarks_matching(&StringMatcher::All, &StringMatcher::All)
    {
        let name = symbol.name.as_str();
        let remote = symbol.remote.as_str();

        if remote == "git" {
            continue;
        }

        // trunk: main/master/trunk on origin/upstream
        let is_trunk =
            matches!(remote, "origin" | "upstream") && matches!(name, "main" | "master" | "trunk");

        // untracked: no local counterpart
        let is_untracked = view.get_local_bookmark(symbol.name).is_absent();

        if (is_trunk || is_untracked)
            && let Some(id) = remote_ref.target.as_normal()
        {
            immutable.insert(id.clone());
        }
    }

    // Tags (usually few)
    for (_, target) in view.tags() {
        if let Some(id) = target.local_target.as_normal() {
            immutable.insert(id.clone());
        }
    }

    immutable
}

/// Search for all bookmarks on ancestor commits using BFS
/// Returns bookmarks sorted by distance (closest first)
fn find_ancestor_bookmarks(
    repo: &dyn Repo,
    view: &View,
    commit_id: &CommitId,
    max_depth: usize,
) -> Option<Vec<JujutsuBookmarkInfo>> {
    use std::collections::{HashMap, HashSet, VecDeque};

    let mut queue: VecDeque<(CommitId, usize)> = VecDeque::new();
    let mut visited = HashSet::new();
    let mut bookmarks_with_details = HashMap::new();

    // Pre-compute immutable heads to stop traversal at trunk/tags/untracked remotes
    let immutable_heads = find_immutable_heads(view);

    // Start BFS from WC commit parents
    let wc_commit = repo.store().get_commit(commit_id).ok()?;

    for parent_id in wc_commit.parent_ids() {
        queue.push_back((parent_id.clone(), 1));
    }

    while let Some((commit_id, depth)) = queue.pop_front() {
        // Stop if we exceed max depth
        if depth > max_depth {
            continue;
        }

        // Skip if already visited
        if !visited.insert(commit_id.clone()) {
            continue;
        }

        // Collect all bookmarks at this commit
        for (name, target) in view.local_bookmarks_for_commit(&commit_id) {
            let details = collect_bookmark_details(name, target, repo, depth);
            let name = details.name.clone();
            // Only record first (shortest) distance for each bookmark
            bookmarks_with_details.entry(name).or_insert(details);
        }

        // Stop at immutable heads - don't traverse past trunk/tags/untracked remotes
        if immutable_heads.contains(&commit_id) {
            continue;
        }

        // Add parents to queue for next level
        if depth < max_depth {
            let commit = repo.store().get_commit(&commit_id).ok()?;

            for parent_id in commit.parent_ids() {
                queue.push_back((parent_id.clone(), depth + 1));
            }
        }
    }

    // Convert to vec and sort by distance
    let mut result = bookmarks_with_details.into_values().collect::<Vec<_>>();
    result.sort_by_key(|details| details.distance);
    Some(result)
}

fn tracking_counts(
    repo: &dyn Repo,
    local_target: &RefTarget,
    remote_ref: &RemoteRef,
) -> Result<Option<(usize, usize)>, revset::RevsetEvaluationError> {
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

fn create_minimal_settings() -> Option<UserSettings> {
    let mut config = StackedConfig::with_defaults();

    // Minimal config required by UserSettings
    let mut user_layer = ConfigLayer::empty(ConfigSource::User);
    user_layer.set_value("user.name", "jujutsu-starship").ok()?;
    user_layer
        .set_value("user.email", "jujutsu-starship@localhost")
        .ok()?;
    config.add_layer(user_layer);

    UserSettings::from_config(config).ok()
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
