use thiserror::Error;

#[derive(Error, Debug)]
pub enum HexError {
    #[error("{0} is not a hex digit")]
    InvalidHexDigit(char),
}

pub fn hex_str_to_bytes(hex_input: &str) -> Result<Vec<u8>, HexError> {
    fn hex_digit_to_value(hex_char: char) -> Result<u8, HexError> {
        match hex_char {
            'A'..='F' => Ok(hex_char as u8 - b'A' + 10),
            'a'..='f' => Ok(hex_char as u8 - b'a' + 10),
            '0'..='9' => Ok(hex_char as u8 - b'0'),
            _ => Err(HexError::InvalidHexDigit(hex_char as char)),
        }
    }

    let hex_values: Vec<u8> = hex_input
        .chars()
        .map(hex_digit_to_value)
        .collect::<Result<Vec<u8>, HexError>>()?;

    let bytes = hex_values
        .chunks(2)
        .map(|chars_chunk| {
            let byte0 = if chars_chunk.len() > 0 {
                chars_chunk[0]
            } else {
                0
            };
            let byte1 = if chars_chunk.len() > 1 {
                chars_chunk[1]
            } else {
                0
            };
            16 * byte0 + byte1
        })
        .collect::<Vec<u8>>();

    Ok(bytes)
}

pub fn bytes_to_hex_str(bytes: &[u8]) -> String {

    fn byte_to_hex_digits(byte: &u8) -> [u8; 2] {
        [byte / 16, byte % 16].map(|nibble| if nibble < 10 {
            b'0' + nibble
        } else {
            b'a' + nibble - 10
        })
    }

    let hex_byte_vec = bytes
        .iter()
        .flat_map(byte_to_hex_digits)
        .collect::<Vec<u8>>();

    String::from_utf8(hex_byte_vec).expect("Unable to convert to a hex string")
}