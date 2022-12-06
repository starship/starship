use unicode_segmentation::UnicodeSegmentation;

/// Truncate a string to only have a set number of characters
///
/// Will truncate a string to only show the last `length` character in the string.
/// If a length of `0` is provided, the string will not be truncated and the original
/// will be returned.
pub fn truncate_text(text: &str, length: usize, truncation_symbol: &str) -> String {
    if length == 0 {
        return String::from(text);
    }

    let truncated_graphemes = get_graphemes(text, length);
    // The truncation symbol should only be added if we truncated
    let truncated_and_symbol = if length < graphemes_len(text) {
        let truncation_symbol = get_graphemes(truncation_symbol, 1);
        truncated_graphemes + truncation_symbol.as_str()
    } else {
        truncated_graphemes
    };

    truncated_and_symbol
}

fn get_graphemes(text: &str, length: usize) -> String {
    UnicodeSegmentation::graphemes(text, true)
        .take(length)
        .collect::<Vec<&str>>()
        .concat()
}

fn graphemes_len(text: &str) -> usize {
    UnicodeSegmentation::graphemes(text, true).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_char_truncation_symbol() {
        let actual = truncate_text("1337_hello_world", 15, "apple");

        assert_eq!("1337_hello_worla", actual);
    }

    #[test]
    fn test_changed_truncation_symbol() {
        test_truncate_length("1337_hello_world", 15, "1337_hello_worl", "%")
    }

    #[test]
    fn test_no_truncation_symbol() {
        test_truncate_length("1337_hello_world", 15, "1337_hello_worl", "")
    }

    #[test]
    fn test_ascii_boundary_below() {
        test_truncate_length("1337_hello_world", 15, "1337_hello_worl", "…")
    }

    #[test]
    fn test_ascii_boundary_on() {
        test_truncate_length("1337_hello_world", 16, "1337_hello_world", "")
    }

    #[test]
    fn test_ascii_boundary_above() {
        test_truncate_length("1337_hello_world", 17, "1337_hello_world", "")
    }

    #[test]
    fn test_one() {
        test_truncate_length("1337_hello_world", 1, "1", "…")
    }

    #[test]
    fn test_negative() {
        test_truncate_length("1337_hello_world", -1, "1337_hello_world", "")
    }

    #[test]
    fn test_hindi_truncation() {
        test_truncate_length("नमस्ते", 3, "नमस्", "…")
    }

    #[test]
    fn test_hindi_truncation2() {
        test_truncate_length("नमस्त", 3, "नमस्", "…")
    }

    #[test]
    fn test_japanese_truncation() {
        test_truncate_length("がんばってね", 4, "がんばっ", "…")
    }

    fn test_truncate_length(
        text: &str,
        truncate_length: i64,
        expected: &str,
        truncation_symbol: &str,
    ) {
        let actual = truncate_text(text, truncate_length as usize, truncation_symbol);

        assert_eq!(format!("{}{}", expected, truncation_symbol), actual);
    }
}
