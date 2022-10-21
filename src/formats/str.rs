use regex::Regex;

/// Check if a string contains the ASCII() marker.
///
/// Examples:
/// ```rust
/// assert!(is_str_ascii("ASCII(Hello)"));
/// assert!(is_str_ascii("ASCII(world)"));
/// assert!(is_str_ascii("Hi mom") == false);
/// ```
pub fn is_str_ascii(s: &str) -> bool {
    let re = Regex::new(r"ASCII\(.+\)").unwrap();

    return re.is_match(s);
}

/// Parse a string containing a ASCII() marker into a Vec<u8>.
///
/// Always use if is_str_ascii() returns true on that string.
///
/// Examples:
/// ```rust
/// let ascii_str: &str = "ASCII(hello world)";
/// if is_str_ascii(ascii_str) {
///     let bytes = parse_str_ascii(ascii_str);
///     println!("{:?}", bytes);
/// }
/// ```
pub fn parse_str_ascii(s: &str) -> Vec<u8> {
    let mut s: String = s[6..].to_owned(); // remove ASCII(
    s.pop(); // remove )

    let bytes: Vec<u8> = s.as_bytes().to_vec();

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        assert!(is_str_ascii("ASCII(hello)"));
        assert!(is_str_ascii("ASCII(12)"));
        assert!(is_str_ascii("ASCII(1)"));
        assert!(is_str_ascii("ASCII(0x44)"));
        assert!(is_str_ascii("ASCII(Hi mom)"));
        assert!(is_str_ascii("ASCII(1234)"));

        assert_eq!(parse_str_ascii("ASCII(a)"), [0x61]);
        assert_eq!(parse_str_ascii("ASCII(abc)"), [0x61, 0x62, 0x63]);
        assert_eq!(parse_str_ascii("ASCII(0)"), [0x30]);
        assert_eq!(parse_str_ascii("ASCII(1)"), [0x31]);
    }
}
