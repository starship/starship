#[cfg(not(target_os = "windows"))]
use super::utils::directory_nix as directory_utils;
#[cfg(target_os = "windows")]
use super::utils::directory_win as directory_utils;
use path_slash::PathExt;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use unicode_segmentation::UnicodeSegmentation;

use super::{Context, Module};

use super::utils::directory::truncate;
use crate::config::RootModuleConfig;
use crate::configs::directory::DirectoryConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current directory
///
/// Will perform path contraction, substitution, and truncation.
/// **Contraction**
///     - Paths beginning with the home directory or with a git repo right
/// inside the home directory will be contracted to `~`
///     - Paths containing a git repo will contract to begin at the repo root
///
/// **Substitution**
/// Paths will undergo user-provided substitutions of substrings
///
/// **Truncation**
/// Paths will be limited in length to `3` path components by default.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const HOME_SYMBOL: &str = "~";

    let mut module = context.new_module("directory");
    let config: DirectoryConfig = DirectoryConfig::try_load(module.config);

    // Using environment PWD is the standard approach for determining logical path
    // If this is None for any reason, we fall back to reading the os-provided path
    let physical_current_dir = if config.use_logical_path {
        match std::env::var("PWD") {
            Ok(x) => Some(PathBuf::from(x)),
            Err(e) => {
                log::debug!("Error getting PWD environment variable: {}", e);
                None
            }
        }
    } else {
        match std::env::current_dir() {
            Ok(x) => Some(x),
            Err(e) => {
                log::debug!("Error getting physical current directory: {}", e);
                None
            }
        }
    };
    let current_dir = Path::new(
        physical_current_dir
            .as_ref()
            .unwrap_or_else(|| &context.current_dir),
    );

    let home_dir = dirs_next::home_dir().unwrap();
    log::debug!("Current directory: {:?}", current_dir);

    let repo = &context.get_repo().ok()?;

    let dir_string = match &repo.root {
        Some(repo_root) if config.truncate_to_repo && (repo_root != &home_dir) => {
            log::debug!("Repo root: {:?}", repo_root);
            // Contract the path to the git repo root
            contract_repo_path(current_dir, repo_root)
                .unwrap_or_else(|| contract_path(current_dir, &home_dir, HOME_SYMBOL))
        }
        // Contract the path to the home directory
        _ => contract_path(current_dir, &home_dir, HOME_SYMBOL),
    };
    log::debug!("Dir string: {}", dir_string);

    let substituted_dir = substitute_path(dir_string, &config.substitutions);

    // Truncate the dir string to the maximum number of path components
    let truncated_dir_string = truncate(substituted_dir, config.truncation_length as usize);

    // Substitutions could have changed the prefix, so don't allow them and
    // fish-style path contraction together
    let fish_prefix = if config.fish_style_pwd_dir_length > 0 && config.substitutions.is_empty() {
        // If user is using fish style path, we need to add the segment first
        let contracted_home_dir = contract_path(&current_dir, &home_dir, HOME_SYMBOL);
        to_fish_style(
            config.fish_style_pwd_dir_length as usize,
            contracted_home_dir,
            &truncated_dir_string,
        )
    } else {
        String::from("")
    };
    let final_dir_string = format!("{}{}", fish_prefix, truncated_dir_string);
    let lock_symbol = String::from(config.read_only_symbol);

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                "read_only_style" => Some(Ok(config.read_only_symbol_style)),
                _ => None,
            })
            .map(|variable| match variable {
                "path" => Some(Ok(&final_dir_string)),
                "read_only" => {
                    if is_readonly_dir(context.current_dir.to_str()?) {
                        Some(Ok(&lock_symbol))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `directory`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn is_readonly_dir(path: &str) -> bool {
    match directory_utils::is_write_allowed(path) {
        Ok(res) => !res,
        Err(e) => {
            log::debug!(
                "Failed to detemine read only status of directory '{}': {}",
                path,
                e
            );
            false
        }
    }
}

/// Contract the root component of a path
///
/// Replaces the `top_level_path` in a given `full_path` with the provided
/// `top_level_replacement`.
fn contract_path(full_path: &Path, top_level_path: &Path, top_level_replacement: &str) -> String {
    if !full_path.starts_with(top_level_path) {
        return full_path.to_slash().unwrap();
    }

    if full_path == top_level_path {
        return top_level_replacement.to_string();
    }

    format!(
        "{replacement}{separator}{path}",
        replacement = top_level_replacement,
        separator = "/",
        path = full_path
            .strip_prefix(top_level_path)
            .unwrap()
            .to_slash()
            .unwrap()
    )
}

/// Contract the root component of a path based on the real path
///
/// Replaces the `top_level_path` in a given `full_path` with the provided
/// `top_level_replacement` by walking ancestors and comparing its real path.
fn contract_repo_path(full_path: &Path, top_level_path: &Path) -> Option<String> {
    let top_level_real_path = real_path(top_level_path);
    // Walk ancestors to preserve logical path in `full_path`.
    // If we'd just `full_real_path.strip_prefix(top_level_real_path)`,
    // then it wouldn't preserve logical path. It would've returned physical path.
    for (i, ancestor) in full_path.ancestors().enumerate() {
        let ancestor_real_path = real_path(ancestor);
        if ancestor_real_path != top_level_real_path {
            continue;
        }

        let components: Vec<_> = full_path.components().collect();
        let repo_name = components[components.len() - i - 1].as_os_str().to_str()?;

        if i == 0 {
            return Some(repo_name.to_string());
        }

        let path = PathBuf::from_iter(&components[components.len() - i..]);
        return Some(format!(
            "{repo_name}{separator}{path}",
            repo_name = repo_name,
            separator = "/",
            path = path.to_slash()?
        ));
    }
    None
}

fn real_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    let mut buf = PathBuf::new();
    for component in path.components() {
        let next = buf.join(component);
        if let Ok(realpath) = next.read_link() {
            if realpath.is_absolute() {
                buf = realpath;
            } else {
                buf.push(realpath);
            }
        } else {
            buf = next;
        }
    }
    buf.canonicalize().unwrap_or_else(|_| path.into())
}

/// Perform a list of string substitutions on the path
///
/// Given a list of (from, to) pairs, this will perform the string
/// substitutions, in order, on the path. Any non-pair of strings is ignored.
fn substitute_path(dir_string: String, substitutions: &HashMap<String, &str>) -> String {
    let mut substituted_dir = dir_string;
    for substitution_pair in substitutions.iter() {
        substituted_dir = substituted_dir.replace(substitution_pair.0, substitution_pair.1);
    }
    substituted_dir
}

/// Takes part before contracted path and replaces it with fish style path
///
/// Will take the first letter of each directory before the contracted path and
/// use that in the path instead. See the following example.
///
/// Absolute Path: `/Users/Bob/Projects/work/a_repo`
/// Contracted Path: `a_repo`
/// With Fish Style: `~/P/w/a_repo`
///
/// Absolute Path: `/some/Path/not/in_a/repo/but_nested`
/// Contracted Path: `in_a/repo/but_nested`
/// With Fish Style: `/s/P/n/in_a/repo/but_nested`
fn to_fish_style(pwd_dir_length: usize, dir_string: String, truncated_dir_string: &str) -> String {
    let replaced_dir_string = dir_string.trim_end_matches(truncated_dir_string).to_owned();
    let components = replaced_dir_string.split('/').collect::<Vec<&str>>();

    if components.is_empty() {
        return replaced_dir_string;
    }

    components
        .into_iter()
        .map(|word| -> String {
            let chars = UnicodeSegmentation::graphemes(word, true).collect::<Vec<&str>>();
            match word {
                "" => "".to_string(),
                _ if chars.len() <= pwd_dir_length => word.to_string(),
                _ if word.starts_with('.') => chars[..=pwd_dir_length].join(""),
                _ => chars[..pwd_dir_length].join(""),
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_home_directory() {
        let full_path = Path::new("/Users/astronaut/schematics/rocket");
        let home = Path::new("/Users/astronaut");

        let output = contract_path(full_path, home, "~");
        assert_eq!(output, "~/schematics/rocket");
    }

    #[test]
    fn contract_repo_directory() {
        let full_path = Path::new("/Users/astronaut/dev/rocket-controls/src");
        let repo_root = Path::new("/Users/astronaut/dev/rocket-controls");

        let output = contract_path(full_path, repo_root, "rocket-controls");
        assert_eq!(output, "rocket-controls/src");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn contract_windows_style_home_directory() {
        let full_path = Path::new("C:\\Users\\astronaut\\schematics\\rocket");
        let home = Path::new("C:\\Users\\astronaut");

        let output = contract_path(full_path, home, "~");
        assert_eq!(output, "~/schematics/rocket");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn contract_windows_style_repo_directory() {
        let full_path = Path::new("C:\\Users\\astronaut\\dev\\rocket-controls\\src");
        let repo_root = Path::new("C:\\Users\\astronaut\\dev\\rocket-controls");

        let output = contract_path(full_path, repo_root, "rocket-controls");
        assert_eq!(output, "rocket-controls/src");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn contract_windows_style_no_top_level_directory() {
        let full_path = Path::new("C:\\Some\\Other\\Path");
        let top_level_path = Path::new("C:\\Users\\astronaut");

        let output = contract_path(full_path, top_level_path, "~");
        assert_eq!(output, "C:/Some/Other/Path");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn contract_windows_style_root_directory() {
        let full_path = Path::new("C:\\");
        let top_level_path = Path::new("C:\\Users\\astronaut");

        let output = contract_path(full_path, top_level_path, "~");
        assert_eq!(output, "C:");
    }

    #[test]
    fn substitute_prefix_and_middle() {
        let full_path = "/absolute/path/foo/bar/baz";
        let mut substitutions = HashMap::new();
        substitutions.insert("/absolute/path".to_string(), "");
        substitutions.insert("/bar/".to_string(), "/");

        let output = substitute_path(full_path.to_string(), &substitutions);
        assert_eq!(output, "/foo/baz");
    }

    #[test]
    fn fish_style_with_user_home_contracted_path() {
        let path = "~/starship/engines/booster/rocket";
        let output = to_fish_style(1, path.to_string(), "engines/booster/rocket");
        assert_eq!(output, "~/s/");
    }

    #[test]
    fn fish_style_with_user_home_contracted_path_and_dot_dir() {
        let path = "~/.starship/engines/booster/rocket";
        let output = to_fish_style(1, path.to_string(), "engines/booster/rocket");
        assert_eq!(output, "~/.s/");
    }

    #[test]
    fn fish_style_with_no_contracted_path() {
        // `truncatation_length = 2`
        let path = "/absolute/Path/not/in_a/repo/but_nested";
        let output = to_fish_style(1, path.to_string(), "repo/but_nested");
        assert_eq!(output, "/a/P/n/i/");
    }

    #[test]
    fn fish_style_with_pwd_dir_len_no_contracted_path() {
        // `truncatation_length = 2`
        let path = "/absolute/Path/not/in_a/repo/but_nested";
        let output = to_fish_style(2, path.to_string(), "repo/but_nested");
        assert_eq!(output, "/ab/Pa/no/in/");
    }

    #[test]
    fn fish_style_with_duplicate_directories() {
        let path = "~/starship/tmp/C++/C++/C++";
        let output = to_fish_style(1, path.to_string(), "C++");
        assert_eq!(output, "~/s/t/C/C/");
    }

    #[test]
    fn fish_style_with_unicode() {
        let path = "~/starship/tmp/目录/a̐éö̲/目录";
        let output = to_fish_style(1, path.to_string(), "目录");
        assert_eq!(output, "~/s/t/目/a̐/");
    }
}
