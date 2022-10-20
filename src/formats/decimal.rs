use regex::Regex;

pub fn is_decimal(s: &str) -> bool {
    let re = Regex::new(r"[a-fA-F0-9]+$").unwrap();

    return re.is_match(s);
}

pub fn parse_decimal(n: &str) -> Vec<u8> {
    let val = u64::from_str_radix(n, 10).expect("Not a decimal number");

    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(&val.to_be_bytes());

    while bytes.len() > 1 && bytes[0] == 0 {
        bytes.remove(0);
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert!(is_decimal("0"));
        assert!(is_decimal("1"));
        assert!(is_decimal("12"));
        assert!(is_decimal("31"));

        // empty
        assert!(is_decimal("") == false);

        // any size should work
        assert!(is_decimal("12"));
        assert!(is_decimal("1"));
        assert!(is_decimal("0"));
        assert!(is_decimal("18130184813"));
        assert!(is_decimal("189120931130184813"));

        // simple decimal
        assert_eq!(parse_decimal("10"), [10]);
        assert_eq!(parse_decimal("6"), [6]);
        assert_eq!(parse_decimal("1"), [1]);
        assert_eq!(parse_decimal("256"), [1, 0]);

        // multi bytes
        assert_eq!(parse_decimal("1024"), [0b100, 0]);
        assert_eq!(
            parse_decimal("123456789"),
            [0b111, 0b01011011, 0b11001101, 0b00010101]
        );
    }
}
