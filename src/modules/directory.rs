use std::path::{Component, PathBuf};

use unicode_segmentation::UnicodeSegmentation;

use super::{Context, Module};

use crate::config::{RootModuleConfig, SegmentConfig};
use crate::configs::directory::DirectoryConfig;

const HOME_SYMBOL: &str = "~";
const CURRENT_DIR: &str = ".";
const PARENT_DIR: &str = "..";

/// Creates a module with the current directory
///
/// Will perform path contraction and truncation.
/// **Contraction**
///     - Paths beginning with the home directory or with a git repo right
/// inside the home directory will be contracted to `~`
///     - Paths containing a git repo will contract to begin at the repo root
///
/// **Truncation**
/// Paths will be limited in length to `3` path components by default.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("directory");
    let config: DirectoryConfig = DirectoryConfig::try_load(module.config);

    module.set_style(config.style);

    let home_dir = Directory::new(&dirs::home_dir().unwrap());
    let home_dir_replacement = Directory::new(&PathBuf::from(HOME_SYMBOL));

    let mut current_fileystem_dir = Directory::new(&context.current_dir.clone());
    current_fileystem_dir.contract(&home_dir, &home_dir_replacement);

    let mut current_dir = if config.use_logical_path && context.logical_dir.is_some() {
        Directory::new(&context.logical_dir.as_ref().unwrap())
    } else {
        Directory::new(&context.current_dir)
    };
    current_dir.contract(&home_dir, &home_dir_replacement);

    let mut repo_dir = &context
        .get_repo()
        .ok()?
        .root
        .as_ref()
        .map(|repo| Directory::new(repo))
        .map(|mut dir| {
            dir.contract(&home_dir, &home_dir_replacement);
            dir
        });

    let repo_folder_name = match repo_dir
        .as_ref()
        .map(|dir| dir.components.last().unwrap().clone())
    {
        Some(DirectoryComponent::Normal(directory)) => Some(directory),
        _ => None,
    };

    log::debug!("Current directory: {:?}", &current_dir);
    log::debug!("Current filesystem directory: {:?}", &current_fileystem_dir);
    log::debug!("Home directory: {:?}", &home_dir);
    log::debug!("Repo directory: {:?}", &repo_dir);
    log::debug!("Repo folder: {:?}", &repo_folder_name);

    let dir = match &mut repo_dir {
        Some(repo_root) => {
            if config.truncate_to_repo && (repo_root != &home_dir) {
                if config.fish_style_pwd_dir_length > 0 {
                    // If user is using fish style path,
                    // Contract the path to the git repo root, using the physical drive (as git.get_repo() contains also the physical)
                    let mut preserved_elements = current_fileystem_dir.components.len();
                    let removed_elts = current_fileystem_dir.contract(
                        &repo_root,
                        &Directory::from(repo_folder_name.unwrap().as_str()),
                    );
                    preserved_elements -= removed_elts.len();
                    current_fileystem_dir.components =
                        [removed_elts, current_fileystem_dir.components].concat();
                    current_fileystem_dir.fish_style(
                        config.fish_style_pwd_dir_length as usize,
                        preserved_elements,
                    );
                } else {
                    // Contract the path to the git repo root, using the physical drive (as git.get_repo() contains also the physical)
                    current_fileystem_dir.contract(
                        &repo_root,
                        &Directory::from(repo_folder_name.unwrap().as_str()),
                    );
                    // Truncate the dir string to the maximum number of path components, does not occur for fish style
                    current_fileystem_dir.truncate(config.truncation_length as usize);
                }
            }
            current_fileystem_dir
        }
        // Contract the path to the home directory
        _ => {
            if config.fish_style_pwd_dir_length > 0 {
                // If user is using fish style path,
                current_dir.fish_style(config.fish_style_pwd_dir_length as usize, 1);
            } else {
                // Truncate the dir string to the maximum number of path components, does not occur for fish style
                current_dir.truncate(config.truncation_length as usize);
            }
            current_dir
        }
    };

    module.create_segment(
        "path",
        &SegmentConfig {
            value: &dir.to_separator(Some(config.separator), Some(config.home)),
            style: None,
        },
    );

    module.get_prefix().set_value(config.prefix);

    Some(module)
}

#[derive(PartialEq, Clone, Debug)]
pub enum DirectoryComponent {
    Prefix(String),
    Root,
    Current,
    Parent,
    Home,
    Normal(String),
}

#[derive(PartialEq, Debug)]
pub struct Directory {
    pub components: Vec<DirectoryComponent>,
}

impl Directory {
    /// Create a new directory representation based on str
    ///
    /// Will create an array of components representing elements of interest to the
    pub fn from(string: &str) -> Directory {
        Directory::new(&PathBuf::from(string))
    }

    /// Create a new directory representation based on FileSystem's PathBuf
    ///
    /// Will create an array of components representing elements of interest to the
    pub fn new(path: &std::path::PathBuf) -> Directory {
        let mut is_first_normal = true;
        let mut elements = Vec::<DirectoryComponent>::new();

        for comp in path.components() {
            match comp {
                Component::Prefix(prefix) => {
                    elements.push(DirectoryComponent::Prefix(
                        prefix.as_os_str().to_string_lossy().to_string(),
                    ));
                }
                Component::RootDir => {
                    elements.push(DirectoryComponent::Root);
                }
                Component::CurDir => {
                    elements.push(DirectoryComponent::Current);
                }
                Component::ParentDir => {
                    elements.push(DirectoryComponent::Parent);
                }
                Component::Normal(component) => {
                    if is_first_normal && component.to_str().unwrap().ends_with(':') {
                        is_first_normal = false;
                        let element = component.to_string_lossy().to_string();
                        elements.push(DirectoryComponent::Prefix(element));
                        elements.push(DirectoryComponent::Root);
                    } else if component == HOME_SYMBOL {
                        elements.push(DirectoryComponent::Home)
                    } else {
                        let element = component.to_string_lossy().to_string();
                        elements.push(DirectoryComponent::Normal(element))
                    }
                }
            }
        }
        Directory {
            components: { elements },
        }
    }
    /// Truncate a path to only have a set number of path components
    ///
    /// Will truncate a path to only show the last `length` components in a path.
    /// If a length of `0` is provided, the path will not be truncated.
    /// Returns the truncated path components
    pub fn truncate(&mut self, length: usize) -> Vec<DirectoryComponent> {
        if self.components.is_empty() || length == 0 {
            return vec![];
        }

        let component_count: usize = self.components.iter().fold(0u32, |count, comp| match comp {
            DirectoryComponent::Normal(_) => count + 1,
            DirectoryComponent::Home => count + 1,
            _ => count,
        }) as usize;

        if component_count <= length {
            // the directory contains less elements than required, no need to truncate
            return vec![];
        }

        // remove unwanted path components
        self.components
            .drain(..self.components.len() - length)
            .collect()
    }

    /// Contract the root component of a path
    ///
    /// Replace the `top_level_path` components with the provided
    /// `top_level_replacement`.
    /// Returns the removed components
    fn contract(
        &mut self,
        top_level_dir: &Directory,
        top_replacement_dir: &Directory,
    ) -> Vec<DirectoryComponent> {
        if top_level_dir.components.len() > self.components.len() {
            return vec![];
        }

        let mut top_level_iter = top_level_dir.components.iter();
        let mut self_iter = self.components.iter();

        while let (Some(top), Some(selfie)) = (top_level_iter.next(), self_iter.next()) {
            if top != selfie {
                return vec![];
            }
        }
        // we have a match!
        let mut contracted: Vec<DirectoryComponent> = self
            .components
            .splice(
                ..top_level_dir.components.len(),
                top_replacement_dir.components.clone(),
            )
            .collect();
        contracted.pop();
        contracted
    }

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
    fn fish_style(&mut self, pwd_dir_length: usize, preserved_number: usize) {
        let normal_count: usize = self.components.iter().fold(0u32, |count, comp| match comp {
            DirectoryComponent::Normal(_) => count + 1,
            _ => count,
        }) as usize;

        if normal_count <= preserved_number {
            // not enough components to "fish style",
            return;
        }

        let mut to_shorten = normal_count - preserved_number;

        for comp in self.components.iter_mut() {
            if let DirectoryComponent::Normal(dir) = comp {
                let chars =
                    UnicodeSegmentation::graphemes(dir.as_str(), true).collect::<Vec<&str>>();

                if !dir.is_empty() && chars.len() > pwd_dir_length {
                    if dir.starts_with('.') {
                        *dir = chars[..=pwd_dir_length].join("");
                    } else {
                        *dir = chars[..pwd_dir_length].join("");
                    }
                    to_shorten -= 1;
                    if to_shorten == 0 {
                        return;
                    }
                }
            }
        }
    }

    fn to_separator(&self, separator_option: Option<&str>, home_options: Option<&str>) -> String {
        let default_separator = std::path::MAIN_SEPARATOR.to_string();
        let separator = separator_option.unwrap_or(&default_separator);
        let home = home_options.unwrap_or(HOME_SYMBOL);

        let mut size = self
            .components
            .iter()
            .fold(0u32 as usize, |count, comp| match comp {
                DirectoryComponent::Prefix(prefix) => count + prefix.len(),
                DirectoryComponent::Home => count + home.len(),
                DirectoryComponent::Current => count + CURRENT_DIR.len(),
                DirectoryComponent::Parent => count + PARENT_DIR.len(),
                DirectoryComponent::Root => count + separator.len(),
                DirectoryComponent::Normal(element) => count + element.len(),
            }) as usize;

        size += self.components.len() * separator.len();

        let mut string = String::with_capacity(size);

        let mut iter = self.components.iter().peekable();
        while let Some(component) = iter.next() {
            match component {
                DirectoryComponent::Prefix(prefix) => {
                    string.push_str(&prefix);
                }
                DirectoryComponent::Home => {
                    string.push_str(home);
                }
                DirectoryComponent::Current => {
                    string.push_str(CURRENT_DIR);
                }
                DirectoryComponent::Parent => {
                    string.push_str(PARENT_DIR);
                }
                DirectoryComponent::Root => {
                    string.push_str(&separator);
                }
                DirectoryComponent::Normal(element) => {
                    string.push_str(&element);
                }
            }
            if iter.peek().is_some() {
                match component {
                    DirectoryComponent::Prefix(_) | DirectoryComponent::Root => {}
                    _ => {
                        string.push_str(&separator);
                    }
                }
            }
        }
        assert!(size >= string.len());
        string
    }
}

impl ToString for Directory {
    fn to_string(&self) -> String {
        self.to_separator(None, None)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    impl Directory {
        fn contract_path(
            &mut self,
            top_level_path: &PathBuf,
            top_level_replacement: &PathBuf,
        ) -> Vec<DirectoryComponent> {
            let top_level_dir = Directory::new(&top_level_path);
            let top_replacement_dir = Directory::new(&top_level_replacement);
            self.contract(&top_level_dir, &top_replacement_dir)
        }
    }
    #[test]
    fn parse_directory_unix() {
        let dir = Directory::from("/home/users/astronaut/starship");
        assert_eq!(dir.components.len(), 5);
        assert_eq!(
            dir.to_separator(Some("/"), None),
            "/home/users/astronaut/starship"
        );
    }

    #[test]
    fn parse_directory_in_home() {
        let dir = Directory::from("~/astronaut/starship");
        assert_eq!(dir.components.len(), 3);
        assert_eq!(dir.to_separator(Some("/"), None), "~/astronaut/starship");
    }

    #[test]
    fn parse_directory_current() {
        let dir = Directory::from(".");
        assert_eq!(dir.components.len(), 1);
        assert_eq!(dir.to_separator(None, None), ".");
    }
    #[test]
    fn parse_directory_parent() {
        let dir = Directory::from("..");
        assert_eq!(dir.components.len(), 1);
        assert_eq!(dir.to_separator(None, None), "..");
    }
    #[test]
    fn parse_directory_home() {
        let dir = Directory::from("~");
        assert_eq!(dir.components.len(), 1);
        assert_eq!(dir.to_separator(None, None), "~");
    }
    #[test]
    fn parse_directory_windows() {
        let dir = Directory::from(r"c:\home\users\astronaut\starship");
        assert_eq!(dir.components.len(), 6);
        assert_eq!(dir.to_string(), r"c:\home\users\astronaut\starship");
    }
    #[test]
    fn parse_directory_windows_with_separator() {
        let dir = Directory::from(r"c:\home\users\astronaut\starship");
        assert_eq!(dir.components.len(), 6);
        assert_eq!(
            dir.to_separator(Some("|"), None),
            r"c:|home|users|astronaut|starship"
        );
    }
    #[test]
    fn parse_directory_windows_with_long_separator() {
        let dir = Directory::from(r"c:\home\users\astronaut\starship");
        assert_eq!(dir.components.len(), 6);
        assert_eq!(
            dir.to_separator(Some("-=-=->"), None),
            r"c:-=-=->home-=-=->users-=-=->astronaut-=-=->starship"
        );
    }

    #[test]
    fn parse_directory_windows_with_emoji_separator() {
        let dir = Directory::from(r"c:\home\users\astronaut\starship");
        assert_eq!(dir.components.len(), 6);
        assert_eq!(
            dir.to_separator(Some("-=-=->"), None),
            r"c:-=-=->home-=-=->users-=-=->astronaut-=-=->starship"
        );
    }
    #[test]
    fn parse_directory_powershell() {
        let dir = Directory::from(r"proj:\starship\src");
        assert_eq!(dir.components.len(), 4);
        assert_eq!(dir.to_separator(Some("/"), None), r"proj:/starship/src");
    }
    #[test]
    fn truncate_smaller_path_than_provided_length() {
        let mut dir = Directory::from("~/starship");
        dir.truncate(3);
        assert_eq!(dir.to_separator(Some("/"), None), "~/starship")
    }

    #[test]
    fn truncate_same_path_as_provided_length() {
        let mut dir = Directory::from("~/starship/engines");
        dir.truncate(3);
        assert_eq!(dir.to_separator(Some("/"), None), "~/starship/engines")
    }

    #[test]
    fn truncate_slightly_larger_path_than_provided_length() {
        let mut dir = Directory::from("~/starship/engines/booster");
        dir.truncate(3);
        assert_eq!(
            dir.to_separator(Some("/"), None),
            "starship/engines/booster"
        )
    }

    #[test]
    fn truncate_larger_path_than_provided_length() {
        let mut dir = Directory::from("~/starship/engines/booster/rocket");
        dir.truncate(3);
        assert_eq!(dir.to_separator(Some("/"), None), "engines/booster/rocket")
    }

    #[test]
    fn truncate_same_path_as_provided_length_from_root_unix() {
        let mut dir = Directory::from("/starship/engines/booster");
        dir.truncate(3);
        assert_eq!(
            dir.to_separator(Some("/"), None),
            "/starship/engines/booster"
        );
    }

    #[test]
    fn truncate_same_path_as_provided_length_from_root_windows() {
        let mut dir = Directory::from(r"C:\starship\engines\booster");
        dir.truncate(3);
        assert_eq!(dir.to_string(), r"C:\starship\engines\booster");
    }

    #[test]
    fn truncate_larger_path_than_provided_length_from_root_unix() {
        let mut dir = Directory::from("/starship/engines/booster/rocket");
        dir.truncate(3);
        assert_eq!(dir.to_separator(Some("/"), None), "engines/booster/rocket");
    }
    #[test]
    fn truncate_larger_path_than_provided_length_from_root_windows() {
        let mut dir = Directory::from(r"C:\starship\engines\booster\rocket");
        dir.truncate(3);
        assert_eq!(dir.to_string(), r"engines\booster\rocket");
    }

    #[test]
    fn contract_path_simple() {
        let mut dir = Directory::from(r"C:\starship\engines\booster\rocket");
        let top_level = PathBuf::from(r"C:\starship\engines");
        let replacement = PathBuf::from("engie");

        dir.contract_path(&top_level, &replacement);
        assert_eq!(dir.to_separator(Some("/"), None), r"engie/booster/rocket");
    }

    #[test]
    fn contract_path_no_match() {
        let mut dir = Directory::from(r"C:\starship\engines\booster\rocket");
        let top_level = PathBuf::from(r"C:\starsip\engines");
        let replacement = PathBuf::from("engie");

        dir.contract_path(&top_level, &replacement);
        assert_eq!(
            dir.to_separator(Some("/"), None),
            r"C:/starship/engines/booster/rocket"
        );
    }

    #[test]
    fn contract_path_longer_top_level() {
        let mut dir = Directory::from(r"C:\starship\engines\booster\rocket");
        let top_level = PathBuf::from(r"C:\starship\engines\booster\rocket\ignition\start");
        let replacement = PathBuf::from("engie");

        dir.contract_path(&top_level, &replacement);
        assert_eq!(
            dir.to_separator(Some("/"), None),
            r"C:/starship/engines/booster/rocket"
        );
    }

    #[test]
    fn contract_path_longer_partial() {
        let mut dir = Directory::from(r"C:\starship\engines\booster\rocket");
        let top_level = PathBuf::from(r"starship\engines\booster");
        let replacement = PathBuf::from("engie");

        dir.contract_path(&top_level, &replacement);
        assert_eq!(
            dir.to_separator(Some("/"), None),
            r"C:/starship/engines/booster/rocket"
        );
    }
    #[test]
    fn contract_path_longer_last_only_difference() {
        let mut dir = Directory::from(r"C:\starship\engines\booster\rocket");
        let top_level = PathBuf::from(r"C:\starship\engine");
        let replacement = PathBuf::from("engie");

        dir.contract_path(&top_level, &replacement);
        assert_eq!(
            dir.to_separator(Some("/"), None),
            r"C:/starship/engines/booster/rocket"
        );
    }

    #[test]
    fn contract_path_home_unix() {
        let mut dir = Directory::from("/home/astronaut/starship/engines/booster/rocket");
        let top_level = PathBuf::from("/home/astronaut");
        let replacement = PathBuf::from("~");

        dir.contract_path(&top_level, &replacement);
        assert_eq!(
            dir.to_separator(Some("/"), None),
            "~/starship/engines/booster/rocket"
        );
    }
    #[test]
    fn contract_path_home_windows() {
        let mut dir = Directory::from(r"C:\Users\astronaut\starship\engines\booster\rocket");
        let top_level = PathBuf::from(r"C:\Users\astronaut");
        let replacement = PathBuf::from("~");

        dir.contract_path(&top_level, &replacement);
        assert_eq!(
            dir.to_separator(Some(r"\"), None),
            r"~\starship\engines\booster\rocket"
        );
    }

    #[test]
    fn separator_emoji() {
        let dir = Directory::from(r"C:\Users\astronaut\starship\engines\booster\rocket");

        assert_eq!(
            dir.to_separator(Some(r"↗"), None),
            r"C:↗Users↗astronaut↗starship↗engines↗booster↗rocket"
        );
    }
    #[test]
    fn home_emoji() {
        let dir = Directory::from(r"~\starship\engines\booster\rocket");

        assert_eq!(
            dir.to_separator(Some(r"/"), Some("🏠")),
            r"🏠/starship/engines/booster/rocket"
        );
    }
    #[test]
    fn home_and_separator_emoji() {
        let dir = Directory::from(r"~\starship\engines\booster\rocket");

        assert_eq!(
            dir.to_separator(Some(r"↗"), Some("🏠")),
            r"🏠↗starship↗engines↗booster↗rocket"
        );
    }

    #[test]
    fn fish_style_absolute() {
        let mut dir = Directory::from(r"/some/Path/not/in_a/repo/but_nested");

        dir.fish_style(2, 2);

        assert_eq!(
            dir.to_separator(Some("/"), None),
            r"/so/Pa/no/in/repo/but_nested"
        );
    }

    #[test]
    fn contract_home_directory() {
        let mut dir = Directory::from(r"/Users/astronaut/schematics/rocket");
        let home = PathBuf::from("/Users/astronaut");

        dir.contract_path(&home, &PathBuf::from("~"));
        assert_eq!(
            dir.to_separator(Some("/"), Some("~")),
            "~/schematics/rocket"
        );
    }

    #[test]
    fn contract_repo_directory() {
        let mut dir = Directory::from(r"/Users/astronaut/dev/rocket-controls/src");
        let repo_root = PathBuf::from("/Users/astronaut/dev/rocket-controls");

        dir.contract_path(&repo_root, &PathBuf::from("rocket-controls"));
        assert_eq!(
            dir.to_separator(Some("/"), Some("~")),
            "rocket-controls/src"
        );
    }

    #[test]
    fn contract_windows_style_repo_directory() {
        let mut dir = Directory::from(r"C:/Users/astronaut/dev/rocket-controls/src");
        let repo_root = PathBuf::from("C:/Users/astronaut/dev/rocket-controls");

        dir.contract_path(&repo_root, &PathBuf::from("rocket-controls"));
        assert_eq!(
            dir.to_separator(Some("/"), Some("~")),
            "rocket-controls/src"
        );
    }

    #[test]
    fn contract_windows_style_no_top_level_directory() {
        let mut dir = Directory::from(r"C:\Some\Other\Path");
        let top_level = PathBuf::from("C:/Users/astronaut");

        dir.contract_path(&top_level, &PathBuf::from("~"));
        assert_eq!(
            dir.to_separator(Some(r"\"), Some("~")),
            r"C:\Some\Other\Path"
        );
    }

    #[test]
    fn contract_windows_style_root_directory() {
        let mut dir = Directory::from(r"C:\");
        let top_level = PathBuf::from(r"C:\Users\astronaut");

        dir.contract_path(&top_level, &PathBuf::from("~"));
        assert_eq!(dir.to_separator(Some(r"\"), Some("~")), r"C:\");
    }

    #[test]
    fn prefered_separator_backslash_windows_style() {
        let mut dir = Directory::from(r"C:/Users/astronaut/schematics/rocket");
        let home = PathBuf::from(r"C:/Users/astronaut");

        dir.contract_path(&home, &PathBuf::from("~"));
        assert_eq!(
            dir.to_separator(Some(r"/"), Some("~")),
            "~/schematics/rocket"
        );
    }

    #[test]
    fn fish_style_with_user_home_contracted_path() {
        let mut dir = Directory::from("~/starship/engines/booster/rocket");
        dir.fish_style(1, 1);
        assert_eq!(dir.to_separator(Some(r"/"), Some("~")), "~/s/e/b/rocket");
    }

    #[test]
    fn fish_style_with_user_home_contracted_path_and_dot_dir() {
        let mut dir = Directory::from("~/.starship/engines/booster/rocket");

        dir.fish_style(2, 2);
        assert_eq!(
            dir.to_separator(Some(r"/"), Some("~")),
            "~/.st/en/booster/rocket"
        );
    }

    #[test]
    fn fish_style_with_no_contracted_path() {
        let mut dir = Directory::from("/absolute/Path/not/in_a/repo/but_nested");
        dir.fish_style(1, 2);
        assert_eq!(
            dir.to_separator(Some(r"/"), Some("~")),
            "/a/P/n/i/repo/but_nested"
        );
    }

    #[test]
    fn fish_style_with_pwd_dir_len_no_contracted_path() {
        let mut dir = Directory::from(r"C:\absolute\Path\not\in_a\repo\but_nested");
        dir.fish_style(2, 2);
        assert_eq!(
            dir.to_separator(Some(r"\"), None),
            r"C:\ab\Pa\no\in\repo\but_nested"
        );
    }

    #[test]
    fn fish_style_with_duplicate_directories() {
        let mut dir = Directory::from(r"~/starship/tmp/C++/C++/C++");
        dir.fish_style(1, 1);
        assert_eq!(dir.to_separator(Some(r"/"), Some("~")), "~/s/t/C/C/C++");
    }

    #[test]
    fn fish_style_with_unicode() {
        let mut dir = Directory::from(r"~/starship/tmp/目录/a̐éö̲/目录");
        dir.fish_style(1, 1);
        assert_eq!(dir.to_separator(Some(r"/"), Some("~")), "~/s/t/目/a̐/目录");
    }
}
