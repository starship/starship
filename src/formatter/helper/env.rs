use std::env;

use crate::formatter::StringFormatterError;

/// Helper function compatible to specification of `ini` files
///
/// - `${env:VAR}`: Displays value of environment variable `VAR`. None if `VAR` does not exist.
/// - `${env:VAR:fallback value}`: Displays value of environment variable `VAR`. Returns
/// "fallback value" if `VAR` is not found.
pub fn env_helper(variable: &str) -> Option<Result<String, StringFormatterError>> {
    if variable.starts_with("env:") {
        let expression: &str = &variable[4..];
        if let Some(index) = expression.find(':') {
            let name: &str = &expression[..index];
            get_env_value(name).map(Ok).or_else(|| {
                let fallback_value: &str = &expression[index + 1..];
                Some(Ok(fallback_value.to_owned()))
            })
        } else {
            let name: &str = expression;
            get_env_value(name).map(Ok)
        }
    } else {
        None
    }
}

fn get_env_value(name: &str) -> Option<String> {
    let os_value = env::var_os(name)?;
    match os_value.into_string() {
        Ok(value) => Some(value),
        Err(_error) => {
            log::error!(
                "Environment variable `{}` is not a valid unicode string",
                &name
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::string_formatter::StringFormatter;

    // match_next(result: Iter<Segment>, value, style)
    macro_rules! match_next {
        ($iter:ident, $value:literal, $($style:tt)+) => {
            let _next = $iter.next().unwrap();
            assert_eq!(_next.value, $value);
            assert_eq!(_next.style, $($style)+);
        }
    }

    #[test]
    fn test_default() {
        env::set_var("_ENV_TEST", "SOME_VAR");

        const FORMAT_STR: &str = "${env:_ENV_TEST}";

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(env_helper);
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "SOME_VAR", None);
    }

    #[test]
    fn test_notfound() {
        const FORMAT_STR: &str = "${env:_ENV_NOTFOUND}";

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(env_helper);
        let result = formatter.parse(None).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_fallback() {
        const FORMAT_STR: &str = "${env:_ENV_NOTFOUND:fallback}";

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(env_helper);
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "fallback", None);
    }
}
