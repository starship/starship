use std::env;
use std::ffi::OsString;
use std::process::Command;

const STD_EDITOR: &str = "vi";

pub fn edit_configuration() {
    let editor = get_editor();
    let config_path = get_config_path();

    Command::new(editor)
        .arg(config_path)
        .status()
        .expect("failed to open file");
}

fn get_editor() -> OsString {
    get_editor_internal(env::var_os("VISUAL"), env::var_os("EDITOR"))
}

fn get_editor_internal(visual: Option<OsString>, editor: Option<OsString>) -> OsString {
    let mut editor_name = visual.unwrap_or_else(|| "".into());
    if !editor_name.is_empty() {
        return editor_name;
    }
    editor_name = editor.unwrap_or_else(|| "".into());
    if !editor_name.is_empty() {
        return editor_name;
    }
    STD_EDITOR.into()
}

fn get_config_path() -> OsString {
    dirs::home_dir()
        .expect("Couldn't find home directory")
        .join(".config/starship.toml")
        .as_os_str()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    // This is every possible permutation, 3² = 9.

    #[test]
    fn visual_set_editor_set() {
        let actual = get_editor_internal(Some("foo".into()), Some("bar".into()));
        assert_eq!("foo", actual);
    }
    #[test]
    fn visual_set_editor_empty() {
        let actual = get_editor_internal(Some("foo".into()), None);
        assert_eq!("foo", actual);
    }
    #[test]
    fn visual_set_editor_not_set() {
        let actual = get_editor_internal(Some("foo".into()), None);
        assert_eq!("foo", actual);
    }

    #[test]
    fn visual_empty_editor_set() {
        let actual = get_editor_internal(Some("".into()), Some("bar".into()));
        assert_eq!("bar", actual);
    }
    #[test]
    fn visual_empty_editor_empty() {
        let actual = get_editor_internal(Some("".into()), Some("".into()));
        assert_eq!("vi", actual);
    }
    #[test]
    fn visual_empty_editor_not_set() {
        let actual = get_editor_internal(Some("".into()), None);
        assert_eq!("vi", actual);
    }

    #[test]
    fn visual_not_set_editor_set() {
        let actual = get_editor_internal(None, Some("bar".into()));
        assert_eq!("bar", actual);
    }
    #[test]
    fn visual_not_set_editor_empty() {
        let actual = get_editor_internal(None, Some("".into()));
        assert_eq!("vi", actual);
    }
    #[test]
    fn visual_not_set_editor_not_set() {
        let actual = get_editor_internal(None, None);
        assert_eq!("vi", actual);
    }
}
