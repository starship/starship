use stylish_stringlike::text::{Pushable, Split};
/// Truncate a path to only have a set number of path components
///
/// Will truncate a path to only show the last `length` components in a path.
/// If a length of `0` is provided, the path will not be truncated.
pub fn truncate<'a, S>(dir_string: &'a S, length: usize, delimiter: &'a str) -> S
where
    S: stylish_stringlike::text::Splitable<'a, &'a str> + Pushable<S> + Clone + Default + 'a,
{
    if length == 0 {
        dir_string.clone()
    } else {
        let num_components: usize = dir_string.split(delimiter).count();
        let start = num_components.saturating_sub(length);
        // return an iterator referencing the input argument for lifetime reasons
        dir_string
            .split(delimiter)
            .enumerate()
            .filter_map(|(count, Split { segment, delim })| {
                if count < start && !((num_components == length + 1) && segment.is_none()) {
                    None
                } else {
                    Some(vec![segment, delim])
                }
            })
            // Flatten Vec
            .flatten()
            // Flatten Option to eliminate None
            .flatten()
            .fold(S::default(), |mut acc, span| {
                acc.push(&span);
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_smaller_path_than_provided_length() {
        let path = "~/starship";
        let output = truncate(&path.to_string(), 3, "/");
        assert_eq!(output, "~/starship")
    }

    #[test]
    fn truncate_same_path_as_provided_length() {
        let path = "~/starship/engines";
        let output = truncate(&path.to_string(), 3, "/");
        assert_eq!(output, "~/starship/engines")
    }

    #[test]
    fn truncate_slightly_larger_path_than_provided_length() {
        let path = "~/starship/engines/booster";
        let output = truncate(&path.to_string(), 3, "/");
        assert_eq!(output, "starship/engines/booster")
    }

    #[test]
    fn truncate_larger_path_than_provided_length() {
        let path = "~/starship/engines/booster/rocket";
        let output = truncate(&path.to_string(), 3, "/");
        assert_eq!(output, "engines/booster/rocket")
    }

    #[test]
    fn truncate_same_path_as_provided_length_from_root() {
        let path = "/starship/engines/booster";
        let output = truncate(&path.to_string(), 3, "/");
        assert_eq!(output, "/starship/engines/booster");
    }

    #[test]
    fn truncate_larger_path_than_provided_length_from_root() {
        let path = "/starship/engines/booster/rocket";
        let output = truncate(&path.to_string(), 3, "/");
        assert_eq!(output, "engines/booster/rocket");
    }
}
