use std::error::Error;

pub mod formats;

pub use formats::binary::{is_binary, parse_binary};
pub use formats::decimal::{is_decimal, parse_decimal};
pub use formats::hexadecimal::{is_hexadecimal, parse_hexadecimal};
pub use formats::str::{is_str_ascii, parse_str_ascii};

/// Parse a string that contains one or may of [hexadecimal, decimal, binary, ASCII()] into a Vec<u8>.
///
/// Examples:
/// ```rust
///     use str_to_bytes::str_to_bytes;
///
///     let my_str: &str = "12 0b11 0xff43 ASCII(HELLO)";
///     let bytes = str_to_bytes(my_str);
///     println!("{:?}", bytes);
/// ```
pub fn str_to_bytes(payload: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bytes: Vec<u8> = Vec::new();

    let args = match shlex::split(payload) {
        Some(a) => a,
        None => Err("Invalid payload")?,
    };

    if args.is_empty() {
        return Err("Empty payload")?;
    }

    for a in &args {
        if is_binary(a) {
            bytes = [bytes, parse_binary(a)].concat();
        } else if is_hexadecimal(a) {
            bytes = [bytes, parse_hexadecimal(a)].concat();
        } else if is_decimal(a) {
            bytes = [bytes, parse_decimal(a)].concat();
        } else if is_str_ascii(a) {
            bytes = [bytes, parse_str_ascii(a)].concat();
        } else {
            Err(format!("Invalid payload: {}", a))?;
        }
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_bytes() {
        // binary
        assert_eq!(str_to_bytes("0b10").unwrap(), [0b10]);
        assert_eq!(str_to_bytes("0b1").unwrap(), [0b1]);
        assert_eq!(str_to_bytes("0b01").unwrap(), [0b01]);
        assert_eq!(str_to_bytes("0b00000001").unwrap(), [0b1]);
        assert_eq!(str_to_bytes("0b11111111").unwrap(), [0xff]);
        assert_eq!(str_to_bytes("0b000000001").unwrap(), [0, 1]);
        assert_eq!(str_to_bytes("0b1111111100").unwrap(), [0xff, 0]);
        assert_eq!(
            str_to_bytes("0b111111110000000011111111").unwrap(),
            [0xff, 0, 0xff]
        );

        // ascii
        assert_eq!(str_to_bytes("ASCII(a)").unwrap(), [0x61]);
        assert_eq!(str_to_bytes("ASCII(abc)").unwrap(), [0x61, 0x62, 0x63]);
        assert_eq!(str_to_bytes("ASCII(0)").unwrap(), [0x30]);
        assert_eq!(str_to_bytes("ASCII(1)").unwrap(), [0x31]);

        // hexadecimal
        assert_eq!(str_to_bytes("0x10").unwrap(), [0x10]);
        assert_eq!(str_to_bytes("0x6").unwrap(), [0x6]);
        assert_eq!(str_to_bytes("0xff").unwrap(), [0xff]);
        assert_eq!(str_to_bytes("0x00").unwrap(), [0x00]);
        assert_eq!(str_to_bytes("0xab").unwrap(), [0xab]);
        assert_eq!(str_to_bytes("0x0f").unwrap(), [0x0f]);
        assert_eq!(str_to_bytes("0xff00ff").unwrap(), [255, 0, 255]);
        assert_eq!(str_to_bytes("0xabcdef").unwrap(), [0xab, 0xcd, 0xef]);
        assert_eq!(
            str_to_bytes("0xabcdef123456").unwrap(),
            [0xab, 0xcd, 0xef, 0x12, 0x34, 0x56]
        );

        // decimal
        assert_eq!(str_to_bytes("10").unwrap(), [10]);
        assert_eq!(str_to_bytes("6").unwrap(), [6]);
        assert_eq!(str_to_bytes("1").unwrap(), [1]);
        assert_eq!(str_to_bytes("256").unwrap(), [1, 0]);
        assert_eq!(str_to_bytes("1024").unwrap(), [0b100, 0]);
        assert_eq!(
            str_to_bytes("123456789").unwrap(),
            [0b111, 0b01011011, 0b11001101, 0b00010101]
        );

        // everything
        assert_eq!(
            str_to_bytes("10 0b10 ASCII(a) 0xff").unwrap(),
            [10, 0b10, 97, 255]
        );
    }
}
