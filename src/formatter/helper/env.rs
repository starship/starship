use std::env;

/// Helper function that maps `env:ENV` to the corresponding environment variable `$ENV`
pub fn env_helper(variable: &str) -> Option<String> {
    if variable.starts_with("env:") {
        let name: &str = &variable[4..];
        get_env_value(name)
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
    fn test_format() {
        env::set_var("_ENV_TEST", "SOME_VAR");

        const FORMAT_STR: &str = "${env:_ENV_TEST}";

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(env_helper);
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, "SOME_VAR", None);
    }
}
