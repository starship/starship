use std::path::PathBuf;
use std::ffi::OsStr;    


pub struct Criteria<'a> {
    pub files: Vec<&'a str>,
    pub extension: String,
    pub folder: String,
}

// based on the directory do any of this criteria match or exist
pub fn is_lang_project(dir_entry: &Vec<PathBuf>, criteria: &Criteria) -> bool {
    dir_entry.into_iter().any(|path|
      match path.is_dir() {
        true => has_folder(&path, &criteria.folder),
        false => has_files(&path, &criteria.files) || has_files_with_extension(&path, &criteria.extension)
    }
    )
  
}

pub fn has_files(dir_entry: &PathBuf, files: &Vec<&str>) -> bool {
    let found_file = files.into_iter().find(|file| 
        dir_entry
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or_default() == **file
    );

    match found_file {
        Some(file) => !file.is_empty(),
        None => false
    }
}

pub fn has_files_with_extension(dir_entry: &PathBuf, extension: &String) -> bool {
    match dir_entry.extension().and_then(OsStr::to_str) {
        Some(ext) => {  
            let stirng = format!("{}", ext);
            println!("this is a string {}", stirng);
            return ext == extension
        },
        None => false    
    }
}

pub fn has_folder(dir_entry: &PathBuf, folder: &String) -> bool {
     match dir_entry.file_name().and_then(OsStr::to_str) {
        Some(ext) => {  
            let stirng = format!("{}", ext);
            println!("this is a string {}", stirng);
            return ext == folder
        },
        None => false    
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_files() {
        let mut buf = PathBuf::from("/");
        let mut files = vec!["no-package.json"];
        
        assert_eq!(has_files(&buf, &files), false);

        buf.set_file_name("some-file.js");
        assert_eq!(has_files(&buf, &files), false);

        files.push("package.json");
        assert_eq!(has_files(&buf, &files), true);
    }

    #[test]
    fn test_has_files_with_extension() {
        let mut buf = PathBuf::from("/");

        assert_eq!(has_files_with_extension(&buf, &String::from("js")), false);

        buf.set_file_name("some-file.rs");
        assert_eq!(has_files_with_extension(&buf, &String::from("js")), false);

        buf.set_file_name("some-file.js");
        assert_eq!(has_files_with_extension(&buf, &String::from("js")), true)
    }

    #[test]
    fn test_has_folder() {
        let mut buf = PathBuf::from("/");

        assert_eq!(has_folder(&buf, &String::from("node_modules")), false);

        buf.set_file_name("some-file.rs");
        assert_eq!(has_folder(&buf, &String::from("node_modules")), false);

        buf.set_file_name("node_modules");
        assert_eq!(has_folder(&buf, &String::from("node_modules")), true)
    }

    #[test]
    fn test_is_lang_project() {
        let mut buf = vec![
            PathBuf::from("/"), 
            PathBuf::from("/cat"), 
            PathBuf::from("/dog")
        ];

        let criteria = Criteria { 
            files: vec!["package.json"],  
            extension: "js".to_string(),
            folder: "node_modules".to_string()
        };

        assert_eq!(is_lang_project(&buf, &criteria), false);

        buf[0].set_file_name("some-file.rs");
        assert_eq!(is_lang_project(&buf, &criteria), false);

        buf[2].set_file_name("node_modules");
        assert_eq!(is_lang_project(&buf, &criteria), true);
    }
}
