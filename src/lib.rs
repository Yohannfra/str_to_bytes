use std::error::Error;

mod formats;

use formats::binary::{is_binary, parse_binary};
use formats::hexadecimal::{is_hexadecimal, parse_hexadecimal};
use formats::decimal::{is_decimal, parse_decimal};

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
        }

        else if is_hexadecimal(a) {
            bytes = [bytes, parse_hexadecimal(a)].concat();
        }

        else if is_decimal(a) {
            bytes = [bytes, parse_decimal(a)].concat();
        }
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_bytes() {
        assert!(str_to_bytes("0xff 0xfa 0b11 89").unwrap() == [0xff, 0xfa, 0b11, 89]);
    }
}
