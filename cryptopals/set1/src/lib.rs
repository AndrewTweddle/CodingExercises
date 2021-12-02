pub mod hex {
    pub fn hex_str_to_bytes(hex_input: &str) -> Vec<u8> {
        fn hex_digit_to_value(hex_char: char) -> u8 {
            match hex_char {
                'A'..='F' => hex_char as u8 - b'A' + 10,
                'a'..='f' => hex_char as u8 - b'a' + 10,
                '0'..='9' => hex_char as u8 - b'0',
                _ => panic!("{} is not a hex digit", hex_char as char),
            }
        }

        let hex_values = hex_input
            .chars()
            .map(hex_digit_to_value)
            .collect::<Vec<u8>>();

        hex_values
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
            .collect::<Vec<u8>>()
    }
}

pub mod base64 {
    pub fn bytes_to_base64(input: &[u8]) -> String {
        input
            .chunks(3)
            .flat_map(|bytes| {
                let byte0 = bytes[0];
                let byte1 = if bytes.len() > 1 { bytes[1] } else { 0 };
                let byte2 = if bytes.len() > 2 { bytes[2] } else { 0 };
                [
                    (byte0 & 0b11111100) >> 2,
                    ((byte0 & 0b00000011) << 4) | ((byte1 & 0b11110000) >> 4),
                    ((byte1 & 0b00001111) << 2) | ((byte2 & 0b11000000) >> 6),
                    byte2 & 0b00111111,
                ]
            })
            .map(|six_bits| {
                let base64_byte = match six_bits {
                    0..=25 => b'A' + six_bits,
                    26..=51 => b'a' + six_bits - 26,
                    52..=61 => b'0' + six_bits - 52,
                    62 => b'+',
                    63 => b'/',
                    _ => panic!("Unexpected out of range nibble"),
                };
                base64_byte as char
            })
            .collect::<String>()
    }
}
