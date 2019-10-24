use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    combinator::rest,
    sequence::{preceded, tuple},
    IResult,
};

fn is_version(c: char) -> bool {
    c >= '0' && c <= '9' || c == '.'
}

fn version(input: &str) -> IResult<&str, &str> {
    take_while1(&is_version)(input)
}

fn zulu(input: &str) -> IResult<&str, &str> {
    let zulu_prefix_value = preceded(take_until("("), tag("("));
    preceded(zulu_prefix_value, version)(input)
}

fn jre_prefix(input: &str) -> IResult<&str, &str> {
    preceded(take_until("JRE ("), tag("JRE ("))(input)
}

fn j9_prefix(input: &str) -> IResult<&str, &str> {
    preceded(take_until("VM ("), tag("VM ("))(input)
}

fn suffix(input: &str) -> IResult<&str, &str> {
    rest(input)
}

/// Parse the java version from `java -Xinternalversion` format.
///
/// The expected format is similar to:
///     "JRE (1.8.0_222-b10)"
///     "JRE (Zulu 8.40.0.25-CA-linux64) (1.8.0_222-b10)"
///     "VM (1.8.0_222-b10)".
///
/// Some Java vendors might not follow this format.
pub fn parse_jre_version(input: &str) -> IResult<&str, &str> {
    let prefix = alt((jre_prefix, j9_prefix));
    let version_or_zulu = alt((version, zulu));
    let (input, (_, version, _)) = tuple((prefix, version_or_zulu, suffix))(input)?;

    Ok((input, version))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_jre_version_eclipse_openj9() {
        let java_8 = "Eclipse OpenJ9 OpenJDK 64-bit Server VM (1.8.0_222-b10) from linux-amd64 JRE with Extensions for OpenJDK for Eclipse OpenJ9 8.0.222.0, built on Jul 17 2019 21:29:18 by jenkins with g++ (GCC) 7.3.1 20180303 (Red Hat 7.3.1-5)";
        let java_11 = "Eclipse OpenJ9 OpenJDK 64-bit Server VM (11.0.4+11) from linux-amd64 JRE with Extensions for OpenJDK for Eclipse OpenJ9 11.0.4.0, built on Jul 17 2019 21:51:37 by jenkins with g++ (GCC) 7.3.1 20180303 (Red Hat 7.3.1-5)";
        assert_eq!(parse_jre_version(java_8), Ok(("", "1.8.0")));
        assert_eq!(parse_jre_version(java_11), Ok(("", "11.0.4")));
    }
}
