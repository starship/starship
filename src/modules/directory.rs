use ansi_term::Color;
use path_slash::PathExt;
use std::path::Path;

use super::{Context, Module};

/// Creates a module with the current directory
///
/// Will perform path contraction and truncation.
/// **Contraction**
///     - Paths beginning with the home directory will be contracted to `~`
///     - Paths containing a git repo will contract to begin at the repo root
///
/// **Truncation**
/// Paths will be limited in length to `3` path components by default.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const HOME_SYMBOL: &str = "~";
    const DIR_TRUNCATION_LENGTH: i64 = 3;
    const FISH_STYLE_PWD_DIR_LENGTH: i64 = 0;
    let module_color = Color::Cyan.bold();

    let mut module = context.new_module("directory");
    module.set_style(module_color);

    let truncation_length = module
        .config_value_i64("truncation_length")
        .unwrap_or(DIR_TRUNCATION_LENGTH);
    let truncate_to_repo = module.config_value_bool("truncate_to_repo").unwrap_or(true);
    let fish_style_pwd_dir_length = module
        .config_value_i64("fish_style_pwd_dir_length")
        .unwrap_or(FISH_STYLE_PWD_DIR_LENGTH);

    let home_dir = dirs::home_dir().unwrap();
    let current_dir = &context.current_dir;
    log::debug!("Current directory: {:?}", current_dir);

    let repo = &context.get_repo().ok()?;

    let dir_string = match &repo.root {
        Some(repo_root) if truncate_to_repo => {
            let repo_folder_name = repo_root.file_name().unwrap().to_str().unwrap();

            // Contract the path to the git repo root
            contract_path(current_dir, repo_root, repo_folder_name)
        }
        // Contract the path to the home directory
        _ => contract_path(current_dir, &home_dir, HOME_SYMBOL),
    };

    // Truncate the dir string to the maximum number of path components
    let truncated_dir_string = truncate(dir_string, truncation_length as usize);

    if fish_style_pwd_dir_length > 0 {
        // If user is using fish style path, we need to add the segment first
        let contracted_home_dir = contract_path(current_dir, &home_dir, HOME_SYMBOL);
        let fish_style_dir = to_fish_style(
            fish_style_pwd_dir_length as usize,
            contracted_home_dir,
            &truncated_dir_string,
        );

        module.new_segment("path", &fish_style_dir);
    }

    module.new_segment("path", &truncated_dir_string);

    module.get_prefix().set_value("in ");

    Some(module)
}

/// Contract the root component of a path
///
/// Replaces the `top_level_path` in a given `full_path` with the provided
/// `top_level_replacement`.
fn contract_path(full_path: &Path, top_level_path: &Path, top_level_replacement: &str) -> String {
    if !full_path.starts_with(top_level_path) {
        return replace_c_dir(full_path.to_slash().unwrap());
    }

    if full_path == top_level_path {
        return replace_c_dir(top_level_replacement.to_string());
    }

    format!(
        "{replacement}{separator}{path}",
        replacement = top_level_replacement,
        separator = "/",
        path = replace_c_dir(
            full_path
                .strip_prefix(top_level_path)
                .unwrap()
                .to_slash()
                .unwrap()
        )
    )
}

/// Replaces "C://" with "/c/" within a Windows path
///
/// On non-Windows OS, does nothing
#[cfg(target_os = "windows")]
fn replace_c_dir(path: String) -> String {
    return path.replace("C:/", "/c");
}

/// Replaces "C://" with "/c/" within a Windows path
///
/// On non-Windows OS, does nothing
#[cfg(not(target_os = "windows"))]
const fn replace_c_dir(path: String) -> String {
    path
}

/// Truncate a path to only have a set number of path components
///
/// Will truncate a path to only show the last `length` components in a path.
/// If a length of `0` is provided, the path will not be truncated.
fn truncate(dir_string: String, length: usize) -> String {
    if length == 0 {
        return dir_string;
    }

    let components = dir_string.split('/').collect::<Vec<&str>>();
    if components.len() <= length {
        return dir_string;
    }

    let truncated_components = &components[components.len() - length..];
    truncated_components.join("/")
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
    let replaced_dir_string = dir_string.replace(truncated_dir_string, "");
    let components = replaced_dir_string.split('/').collect::<Vec<&str>>();

    if components.is_empty() {
        return replaced_dir_string;
    }

    components
        .into_iter()
        .map(|word| match word {
            "" => "",
            _ if word.len() <= pwd_dir_length => word,
            _ if word.starts_with('.') => &word[..=pwd_dir_length],
            _ => &word[..pwd_dir_length],
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
        assert_eq!(output, "/c/Some/Other/Path");
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn contract_windows_style_root_directory() {
        let full_path = Path::new("C:\\");
        let top_level_path = Path::new("C:\\Users\\astronaut");

        let output = contract_path(full_path, top_level_path, "~");
        assert_eq!(output, "/c");
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
}
