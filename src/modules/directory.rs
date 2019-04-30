use ansi_term::Color;
use std::path::Path;

use super::{Segment, PromptComponent};
use crate::context::Context;

/// Creates a segment with the current directory
///
/// Will perform path contraction and truncation.
/// **Contraction**
///     - Paths begining with the home directory will be contracted to `~`
///     - Paths containing a git repo will contract to begin at the repo root
///
/// **Truncation**
/// Paths will be limited in length to `3` path components by default.
pub fn segment(context: &Context) -> PromptComponent {
    const HOME_SYMBOL: &str = "~";
    const DIR_TRUNCATION_LENGTH: usize = 3;
    const SEGMENT_COLOR: Color = Color::Cyan;

    let mut segment = Segment::new("dir");
    let current_dir = &context.current_dir;

    let dir_string;
    if let Some(repo) = &context.repository {
        // Contract the path to the git repo root
        let repo_root = repo.workdir().unwrap();
        let repo_folder_name = repo_root.file_name().unwrap().to_str().unwrap();

        dir_string = contract_path(&current_dir, repo_root, repo_folder_name);
    } else {
        // Contract the path to the home directory
        let home_dir = dirs::home_dir().unwrap();

        dir_string = contract_path(&current_dir, &home_dir, HOME_SYMBOL);
    }

    // Truncate the dir string to the maximum number of path components
    let truncated_dir_string = truncate(dir_string, DIR_TRUNCATION_LENGTH);

    segment
        .set_value(truncated_dir_string)
        .set_style(SEGMENT_COLOR.bold());

    Some(Box::new(segment))
}

/// Contract the root component of a path
///
/// Replaces the `top_level_path` in a given `full_path` with the provided
/// `top_level_replacement`.
fn contract_path(full_path: &Path, top_level_path: &Path, top_level_replacement: &str) -> String {
    if !full_path.starts_with(top_level_path) {
        return full_path.to_str().unwrap().to_string();
    }

    if full_path == top_level_path {
        return top_level_replacement.to_string();
    }

    format!(
        "{replacement}{separator}{path}",
        replacement = top_level_replacement,
        separator = std::path::MAIN_SEPARATOR,
        path = full_path
            .strip_prefix(top_level_path)
            .unwrap()
            .to_str()
            .unwrap()
    )
}

/// Truncate a path to only have a set number of path components
///
/// Will truncate a path to only show the last `length` components in a path.
/// If a length of `0` is provided, the path will not be truncated.
fn truncate(dir_string: String, length: usize) -> String {
    if length == 0 {
        return dir_string;
    }

    let components = dir_string
        .split(std::path::MAIN_SEPARATOR)
        .collect::<Vec<&str>>();
    if components.len() <= length {
        return dir_string;
    }

    let truncated_components = &components[components.len() - length..];
    truncated_components.join(&std::path::MAIN_SEPARATOR.to_string())
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
    fn truncate_smaller_path_than_provided_length() {
        let path = "~/starship";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "~/starship")
    }

    #[test]
    fn truncate_same_path_as_provided_length() {
        let path = "~/starship/engines";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "~/starship/engines")
    }

    #[test]
    fn truncate_slightly_larger_path_than_provided_length() {
        let path = "~/starship/engines/booster";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "starship/engines/booster")
    }

    #[test]
    fn truncate_larger_path_than_provided_length() {
        let path = "~/starship/engines/booster/rocket";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "engines/booster/rocket")
    }
}
