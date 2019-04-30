use std::ffi::OsStr;
use std::path::PathBuf;

pub struct Criteria<'a> {
    pub files: Vec<&'a str>,
    pub extension: Vec<&'a str>,
    pub folder: Vec<&'a str>,
}

impl Criteria {
    fn new() -> Criteria {
        Criteria {}
    }
}

// based on the directory do any of this criteria match or exist
pub fn is_lang_project(dir_entry: &Vec<PathBuf>, criteria: &Criteria) -> bool {
    dir_entry.into_iter().any(|path| match path.is_dir() {
        true => path_has_name(&path, &criteria.folder),
        false => path_has_name(&path, &criteria.files) || has_extension(&path, &criteria.extension),
    })
}

pub fn path_has_name(dir_entry: &PathBuf, names: &Vec<&str>) -> bool {
    let found_file_or_folder_name = names.into_iter().find(|file_or_folder_name| {
        dir_entry
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            == **file_or_folder_name
    });

    match found_file_or_folder_name {
        Some(name) => !name.is_empty(),
        None => false,
    }
}

pub fn has_extension(dir_entry: &PathBuf, extensions: &Vec<&str>) -> bool {
    let found_ext = extensions.into_iter().find(|ext| {
        dir_entry
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default()
            == **ext
    });

    match found_ext {
        Some(extension) => !extension.is_empty(),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_has_name() {
        let mut buf = PathBuf::from("/");
        let mut files = vec!["package.json"];

        assert_eq!(path_has_name(&buf, &files), false);

        buf.set_file_name("some-file.js");
        assert_eq!(path_has_name(&buf, &files), false);

        files.push("package.json");
        assert_eq!(path_has_name(&buf, &files), true);
    }

    #[test]
    fn test_has_extension() {
        let mut buf = PathBuf::from("/");
        let extensions = vec!["js"];

        assert_eq!(has_extension(&buf, &extensions), false);

        buf.set_file_name("some-file.rs");
        assert_eq!(has_extension(&buf, &extensions), false);

        buf.set_file_name("some-file.js");
        assert_eq!(has_extension(&buf, &extensions), true)
    }

    #[test]
    fn test_is_lang_project() {
        let mut buf = vec![PathBuf::new()];

        let criteria = Criteria {
            files: vec!["package.json"],
            extension: vec!["js"],
            folder: vec!["node_modules"],
        };

        assert_eq!(is_lang_project(&buf, &criteria), false);

        buf[0].set_file_name("package.json");
        assert_eq!(is_lang_project(&buf, &criteria), true);
    }
}
