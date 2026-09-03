use unicode_width::UnicodeWidthStr;

/// Truncate a path to only have a set number of path components or a maximum width
///
/// Will truncate a path to only show the last `length` components in a path,
/// and ensure the total length does not exceed `max_width`.
/// If both `length` and `max_width` are `0`, the path will not be truncated.
/// A value will only be returned if the path has been truncated.
pub fn truncate(dir_string: &str, length: usize, max_width: usize) -> Option<String> {
    if length == 0 && max_width == 0 {
        return None;
    }

    let mut components = dir_string.split('/').collect::<Vec<&str>>();

    // If the first element is "" then there was a leading "/" and we should remove it so we can check the actual count of components
    if components[0].is_empty() {
        components.remove(0);
    }

    let truncated_components = if length == 0 {
        &components
    } else {
        let truncated_index = components.len().saturating_sub(length);
        &components[truncated_index..]
    };

    if max_width == 0 {
        if components.len() == truncated_components.len() {
            return None;
        } else {
            return Some(truncated_components.join("/"));
        }
    }

    let count = truncated_components
        .iter()
        .rev()
        .scan(0, |width, comp| {
            let sep = if *width == 0 { 0 } else { 1 };
            *width += comp.width() + sep;
            Some(*width)
        })
        .enumerate()
        .take_while(|&(i, width)| i == 0 || width <= max_width)
        .count();

    let final_components = &truncated_components[truncated_components.len() - count..];

    if components.len() == final_components.len() {
        return None;
    }

    Some(final_components.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_smaller_path_than_provided_length() {
        let path = "~/starship";
        let output = truncate(path, 3, 0);
        assert_eq!(output, None);
    }

    #[test]
    fn truncate_same_path_as_provided_length() {
        let path = "~/starship/engines";
        let output = truncate(path, 3, 0);
        assert_eq!(output, None);
    }

    #[test]
    fn truncate_slightly_larger_path_than_provided_length() {
        let path = "~/starship/engines/booster";
        let output = truncate(path, 3, 0);
        assert_eq!(output.as_deref(), Some("starship/engines/booster"));
    }

    #[test]
    fn truncate_larger_path_than_provided_length() {
        let path = "~/starship/engines/booster/rocket";
        let output = truncate(path, 3, 0);
        assert_eq!(output.as_deref(), Some("engines/booster/rocket"));
    }

    #[test]
    fn truncate_same_path_as_provided_length_from_root() {
        let path = "/starship/engines/booster";
        let output = truncate(path, 3, 0);
        assert_eq!(output, None);
    }

    #[test]
    fn truncate_larger_path_than_provided_length_from_root() {
        let path = "/starship/engines/booster/rocket";
        let output = truncate(path, 3, 0);
        assert_eq!(output.as_deref(), Some("engines/booster/rocket"));
    }

    #[test]
    fn truncate_larger_path_than_max_width() {
        let path = "~/starship/engines/booster";
        let output = truncate(path, 5, 10);
        assert_eq!(output.as_deref(), Some("booster"));
    }

    #[test]
    fn truncate_by_length_and_max_width() {
        let path = "~/starship/engines/booster/rocket";
        let output = truncate(path, 2, 15);
        assert_eq!(output.as_deref(), Some("booster/rocket"));
    }

    #[test]
    fn truncate_by_length_and_max_width_truncates_further() {
        let path = "~/starship/engines/booster/rocket";
        let output = truncate(path, 3, 15);
        assert_eq!(output.as_deref(), Some("booster/rocket"));
    }

    #[test]
    fn truncate_max_width_keeps_at_least_one_component() {
        let path = "~/starship/engines/super_heavy_booster";
        let output = truncate(path, 3, 10);
        assert_eq!(output.as_deref(), Some("super_heavy_booster"));
    }

    #[test]
    fn truncate_same_path_as_max_width() {
        let path = "~/starship/engines";
        let output = truncate(path, 3, 16);
        assert_eq!(output.as_deref(), Some("starship/engines"));
    }

    #[test]
    fn truncate_with_zero_length_and_zero_max_width() {
        let path = "~/starship/engines/booster/rocket";
        let output = truncate(path, 0, 0);
        assert_eq!(output, None);
    }

    #[test]
    fn truncate_length_zero_with_max_width() {
        let path = "~/starship/engines/booster/rocket";
        let output = truncate(path, 0, 15);
        assert_eq!(output.as_deref(), Some("booster/rocket"));
    }

    #[test]
    fn truncate_path_with_unicode_characters() {
        let path = "~/starship/engines/booster/ロケット";
        let output = truncate(path, 0, 16);
        assert_eq!(output.as_deref(), Some("booster/ロケット"));
    }
}
