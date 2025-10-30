use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseValueError {
    Empty,
    InvalidFormat(String),
}

impl fmt::Display for ParseValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseValueError::Empty => write!(f, "value is empty"),
            ParseValueError::InvalidFormat(raw) => write!(
                f,
                "value '{raw}' is not a valid decimal or hexadecimal number"
            ),
        }
    }
}

impl Error for ParseValueError {}

pub fn parse_value<T: core::str::FromStr + num_traits::Num>(
    value: &str,
) -> Result<T, ParseValueError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ParseValueError::Empty);
    }

    if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        let payload = &trimmed[2..];
        return T::from_str_radix(payload, 16)
            .map_err(|_| ParseValueError::InvalidFormat(trimmed.to_string()));
    }

    trimmed
        .parse::<T>()
        .map_err(|_| ParseValueError::InvalidFormat(trimmed.to_string()))
}

pub fn hexdump(buf: &[u8], base_address: u32) {
    for i in (0..buf.len()).step_by(16) {
        print!("{:08x}: ", base_address as usize + i);
        let chunk_len = 16.min(buf.len() - i);
        for j in 0..chunk_len {
            print!("{:02x} ", buf[i + j]);
        }
        print!(" ");
        for _ in chunk_len..16 {
            print!("   ");
        }
        for byte in &buf[i..(i + chunk_len)] {
            if byte.is_ascii_graphic() || *byte == b' ' {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_value_u32() {
        assert_eq!(parse_value::<u32>("123"), Ok(123));
        assert_eq!(parse_value::<u32>("0x7b"), Ok(123));
        assert!(matches!(parse_value::<u32>(" 0x10 "), Ok(16)));
        assert!(matches!(
            parse_value::<u32>(""),
            Err(ParseValueError::Empty)
        ));
        assert!(matches!(
            parse_value::<u32>("zz"),
            Err(ParseValueError::InvalidFormat(raw)) if raw == "zz"
        ));
    }

    #[test]
    fn test_parse_value_usize() {
        assert_eq!(parse_value::<usize>("0x100"), Ok(256usize));
    }
}
