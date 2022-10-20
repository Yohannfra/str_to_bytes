use regex::Regex;

pub fn is_hexadecimal(s: &str) -> bool {
    let re = Regex::new(r"^(0x|0X)[a-fA-F0-9]+$").unwrap();

    return re.is_match(s);
}

pub fn parse_hexadecimal(n: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut bytes_str: Vec<String> = Vec::new();

    let mut s: String = n[2..].to_owned(); // remove 0x

    while s.is_empty() == false {
        // regroup in groups of 2 digits
        let tmp = s[0..std::cmp::min(2, s.len())].to_owned();
        s = s[std::cmp::min(2, s.len())..].to_owned();
        bytes_str.push(tmp);
    }

    for bs in &bytes_str {
        // parse all groups of 2 digits into u8
        bytes.push(u8::from_str_radix(&bs, 16).expect("Not a hexadecimal number"));
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexadecimal() {
        // x or X
        assert!(is_hexadecimal("0x00"));
        assert!(is_hexadecimal("0XFF"));
        assert!(is_hexadecimal("0XFf"));
        assert!(is_hexadecimal("0xFf"));

        // empty or just 0x
        assert!(is_hexadecimal("0x") == false);
        assert!(is_hexadecimal("0X") == false);
        assert!(is_hexadecimal("") == false);

        // any size should work
        assert!(is_hexadecimal("0x0"));
        assert!(is_hexadecimal("0x01"));
        assert!(is_hexadecimal("0xf"));
        assert!(is_hexadecimal("0xff01938422749abdef13131fee3"));

        // simple binary
        assert_eq!(parse_hexadecimal("0x10"), [0x10]);
        assert_eq!(parse_hexadecimal("0x6"), [0x6]);
        assert_eq!(parse_hexadecimal("0xff"), [0xff]);
        assert_eq!(parse_hexadecimal("0x00"), [0x00]);
        assert_eq!(parse_hexadecimal("0xab"), [0xab]);
        assert_eq!(parse_hexadecimal("0x0f"), [0x0f]);

        // multi bytes
        assert_eq!(parse_hexadecimal("0xff00ff"), [255, 0, 255]);
        assert_eq!(parse_hexadecimal("0xabcdef"), [0xab, 0xcd, 0xef]);
        assert_eq!(
            parse_hexadecimal("0xabcdef123456"),
            [0xab, 0xcd, 0xef, 0x12, 0x34, 0x56]
        );
    }
}
