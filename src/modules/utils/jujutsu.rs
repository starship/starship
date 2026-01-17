use crate::config::ModuleConfig;
use crate::context::Context;
use crate::context::JujutsuRepo;
use crate::modules::vcs;
use jj_lib::backend::CommitId;
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
use jj_lib::str_util::StringExpression;
use std::collections::HashMap;
use std::sync::Arc;

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

fn shorten_id(id: &str, length: usize) -> String {
    if length == 0 {
        return String::new();
    }
    id.chars().take(length).collect()
}
