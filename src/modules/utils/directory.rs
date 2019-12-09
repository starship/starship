/// Truncate a path to only have a set number of path components
///
/// Will truncate a path to only show the last `length` components in a path.
/// If a length of `0` is provided, the path will not be truncated.
pub fn truncate(dir_string: String, length: usize) -> String {
    if length == 0 {
        return dir_string;
    }

    let mut components = dir_string.split('/').collect::<Vec<&str>>();

    // If the first element is "" then there was a leading "/" and we should remove it so we can check the actual count of components
    if components[0] == "" {
        components.remove(0);
    }

    if components.len() <= length {
        return dir_string;
    }

    let truncated_components = &components[components.len() - length..];
    truncated_components.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn truncate_same_path_as_provided_length_from_root() {
        let path = "/starship/engines/booster";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "/starship/engines/booster");
    }

    #[test]
    fn truncate_larger_path_than_provided_length_from_root() {
        let path = "/starship/engines/booster/rocket";
        let output = truncate(path.to_string(), 3);
        assert_eq!(output, "engines/booster/rocket");
    }
}
