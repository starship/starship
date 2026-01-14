use crate::config::ModuleConfig;
use crate::context::Context;
use crate::modules::vcs;
use serde::Deserialize;
use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub(crate) struct BookmarkInfo {
    pub name: String,
    pub remote_ahead: usize,
    pub remote_behind: usize,
    pub is_tracked: bool,
}

#[derive(Debug, Deserialize)]
struct JjLogOutput {
    change_id: String,
    local_bookmarks: Vec<String>,
    tracked_remote_bookmarks: Vec<TrackedBookmarkOutput>,
    commit_id: String,
    conflict: bool,
    divergent: bool,
    hidden: bool,
    immutable: bool,
    bookmark_conflict: bool,
}

#[derive(Debug, Deserialize)]
struct TrackedBookmarkOutput {
    name: String,
    ahead: usize,
    behind: usize,
}

const TRACKED_BOOKMARKS_TEMPLATE: &str = r#"if(remote, '{"name":' ++ json(name) ++ ',"ahead":' ++ json(tracking_ahead_count.lower()) ++ ',"behind":' ++ json(tracking_behind_count.lower()) ++ '}' ++ "\n", "")"#;

fn jujutsu_log_template(change_id_length: usize, commit_hash_length: usize) -> String {
    format!(
        r#"'{{"change_id":' ++ json(change_id.short({change_id_length}))
        ++ ',"local_bookmarks":'
        ++ if(local_bookmarks,
            '[' ++ local_bookmarks.map(|b| json(b.name())).join(",") ++ ']',
            '[]')
        ++ ',"tracked_remote_bookmarks":'
        ++ if(remote_bookmarks.filter(|b| b.tracked()),
            '[' ++ remote_bookmarks.filter(|b| b.tracked()).map(|b|
                '{{"name":' ++ json(b.name())
                ++ ',"ahead":' ++ json(b.tracking_ahead_count().lower())
                ++ ',"behind":' ++ json(b.tracking_behind_count().lower())
                ++ '}}'
            ).join(",") ++ ']',
            '[]')
        ++ ',"commit_id":' ++ json(commit_id.short({commit_hash_length}))
        ++ ',"conflict":' ++ json(conflict)
        ++ ',"divergent":' ++ json(divergent)
        ++ ',"hidden":' ++ json(hidden)
        ++ ',"immutable":' ++ json(immutable)
        ++ ',"bookmark_conflict":'
        ++ json(local_bookmarks.any(|b| b.conflict()) || remote_bookmarks.any(|b| b.conflict()))
        ++ '}}'"#,
        change_id_length = change_id_length,
        commit_hash_length = commit_hash_length,
    )
}

fn parse_tracked_bookmarks(output: &str) -> HashMap<String, (usize, usize)> {
    output
        .lines()
        .filter(|entry| !entry.trim().is_empty())
        .filter_map(|entry| serde_json::from_str::<TrackedBookmarkOutput>(entry).ok())
        .map(|entry| (entry.name, (entry.ahead, entry.behind)))
        .fold(HashMap::new(), |mut map, (name, counts)| {
            map.entry(name)
                .and_modify(|existing| {
                    existing.0 = existing.0.max(counts.0);
                    existing.1 = existing.1.max(counts.1);
                })
                .or_insert(counts);
            map
        })
}

fn merge_tracked_bookmarks(
    target: &mut HashMap<String, (usize, usize)>,
    incoming: HashMap<String, (usize, usize)>,
) {
    for (name, (ahead, behind)) in incoming {
        target
            .entry(name)
            .and_modify(|counts| {
                counts.0 = counts.0.max(ahead);
                counts.1 = counts.1.max(behind);
            })
            .or_insert((ahead, behind));
    }
}

pub(crate) fn get_jujutsu_info(ctx: &Context, ignore_working_copy: &bool) -> Option<JjRepoInfo> {
    vcs::discover_repo_root(ctx, vcs::Vcs::Jujutsu)?;

    let change_id_length = crate::configs::jujutsu_change::JujutsuChangeConfig::try_load(
        ctx.config.get_module_config("jujutsu_change"),
    )
    .change_id_length;
    let commit_hash_length = crate::configs::jujutsu_commit::JujutsuCommitConfig::try_load(
        ctx.config.get_module_config("jujutsu_commit"),
    )
    .commit_hash_length;

    let output = ctx
        .exec_cmd(
            "jj",
            &[
                "log",
                "--no-graph",
                "-r",
                "@",
                if *ignore_working_copy {
                    "--ignore-working-copy"
                } else {
                    ""
                },
                "-T",
                &jujutsu_log_template(change_id_length, commit_hash_length),
            ],
        )?
        .stdout;

    let log_output: JjLogOutput = serde_json::from_str(output.trim()).ok()?;

    let mut tracked_bookmarks = log_output
        .tracked_remote_bookmarks
        .into_iter()
        .map(|entry| (entry.name, (entry.ahead, entry.behind)))
        .collect();

    let tracked_bookmarks_output = ctx
        .exec_cmd(
            "jj",
            &[
                "bookmark",
                "list",
                "--tracked",
                if *ignore_working_copy {
                    "--ignore-working-copy"
                } else {
                    ""
                },
                "-T",
                TRACKED_BOOKMARKS_TEMPLATE,
            ],
        )
        .map(|output| output.stdout)
        .unwrap_or_default();
    merge_tracked_bookmarks(
        &mut tracked_bookmarks,
        parse_tracked_bookmarks(&tracked_bookmarks_output),
    );

    let bookmarks = if log_output.local_bookmarks.is_empty() {
        Vec::new()
    } else {
        log_output
            .local_bookmarks
            .into_iter()
            .map(|name| {
                let (ahead, behind, is_tracked) = tracked_bookmarks
                    .get(&name)
                    .map(|(ahead, behind)| (*ahead, *behind, true))
                    .unwrap_or((0, 0, false));
                BookmarkInfo {
                    name,
                    remote_ahead: ahead,
                    remote_behind: behind,
                    is_tracked,
                }
            })
            .collect()
    };

    Some(JjRepoInfo {
        change_id: log_output.change_id,
        commit_id: log_output.commit_id,
        bookmarks,
        conflicted: log_output.conflict,
        divergent: log_output.divergent,
        hidden: log_output.hidden,
        immutable: log_output.immutable,
        bookmark_conflicted: log_output.bookmark_conflict,
    })
}

#[cfg(test)]
pub(crate) fn jujutsu_log_command(
    change_id_length: usize,
    commit_hash_length: usize,
) -> &'static str {
    Box::leak(
        format!(
            "jj log --no-graph -r @ --ignore-working-copy -T {}",
            jujutsu_log_template(change_id_length, commit_hash_length)
        )
        .into_boxed_str(),
    )
}

#[cfg(test)]
pub(crate) fn jujutsu_tracked_bookmarks_command() -> &'static str {
    Box::leak(
        format!(
            "jj bookmark list --tracked --ignore-working-copy -T {}",
            TRACKED_BOOKMARKS_TEMPLATE
        )
        .into_boxed_str(),
    )
}
