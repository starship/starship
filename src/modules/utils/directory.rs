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
