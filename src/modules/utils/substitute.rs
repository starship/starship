use indexmap::IndexMap;

/// Perform a list of string substitutions on the text
///
/// Given a list of (from, to) pairs, this will perform the string
/// substitutions, in order, on the input text. Any non-pair of strings is ignored.
pub fn substitute_text(text: String, substitutions: &IndexMap<String, &str>) -> String {
    let mut substituted_text = text;
    for substitution_pair in substitutions {
        substituted_text = substituted_text.replace(substitution_pair.0, substitution_pair.1);
    }
    substituted_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitute_prefix_and_middle() {
        let full_text = "my-prefix-some-middle-text";
        let mut substitutions = IndexMap::new();
        substitutions.insert("my-prefix-".to_string(), "");
        substitutions.insert("-middle-".to_string(), "-");

        let output = substitute_text(full_text.to_string(), &substitutions);
        assert_eq!(output, "some-text");
    }

    #[test]
    fn substitution_order() {
        let full_text = "some-super-random-text";
        let mut substitutions = IndexMap::new();
        substitutions.insert("super-random".to_string(), "correct");
        substitutions.insert("super".to_string(), "wrong");

        let output = substitute_text(full_text.to_string(), &substitutions);

        assert_eq!(output, "some-correct-text");
    }
}
