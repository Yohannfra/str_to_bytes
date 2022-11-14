use regex::Regex;

/// Check if a string match a binary number representation.
///
/// Accepts 0b and 0B
///
/// Examples:
/// ```rust
/// use str_to_bytes::is_binary;
///
/// assert!(is_binary("0b11"));
/// assert!(is_binary("0B01"));
/// assert!(is_binary("0b123") == false);
/// ```
pub fn is_binary(s: &str) -> bool {
    let re = Regex::new(r"^(0b|0B)[10]+$").unwrap();

    re.is_match(s)
}

/// Parse a string representing a binary number into a Vec<u8>.
///
/// Always use if is_binary() returns true on that string.
///
/// Examples:
/// ```rust
/// use str_to_bytes::{is_binary, parse_binary};
///
/// let bin_str: &str = "0b1010001";
/// if is_binary(bin_str) {
///     let bytes = parse_binary(bin_str);
///     println!("{:?}", bytes);
/// }
/// ```
pub fn parse_binary(n: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut bytes_str: Vec<String> = Vec::new();

    let mut s: String = n[2..].to_owned(); // remove 0b

    while !s.is_empty() {
        // regroup in groups of 8 bits
        let tmp = s[0..std::cmp::min(8, s.len())].to_owned();
        s = s[std::cmp::min(8, s.len())..].to_owned();
        bytes_str.push(tmp);
    }

    for bs in &bytes_str {
        // parse all groups of 8 bits into u8
        bytes.push(u8::from_str_radix(bs, 2).expect("Not a binary number"));
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary() {
        // b or B
        assert!(is_binary("0b11"));
        assert!(is_binary("0B11"));

        // empty or just 0b
        assert!(is_binary("0b") == false);
        assert!(is_binary("0B") == false);
        assert!(is_binary("") == false);

        // any size should work
        assert!(is_binary("0b0"));
        assert!(is_binary("0b01"));
        assert!(is_binary("0b1"));
        assert!(is_binary("0b0101010010011010010100101001"));

        // simple binary
        assert_eq!(parse_binary("0b10"), [0b10]);
        assert_eq!(parse_binary("0b1"), [0b1]);
        assert_eq!(parse_binary("0b01"), [0b01]);
        assert_eq!(parse_binary("0b00000001"), [0b1]);
        assert_eq!(parse_binary("0b11111111"), [0xff]);

        // multi bytes
        assert_eq!(parse_binary("0b000000001"), [0, 1]);
        assert_eq!(parse_binary("0b1111111100"), [0xff, 0]);
        assert_eq!(
            parse_binary("0b11111111000000001111111100000000"),
            [0xff, 0, 0xff, 0]
        );
    }
}
