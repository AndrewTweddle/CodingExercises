use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BytesToBase64Error {
    #[error("Out of range byte: {0} > 63")]
    OutOfRangeByte(u8),
}

pub fn bytes_to_base64(input: &[u8]) -> Result<String, BytesToBase64Error> {
    input
        .chunks(3)
        .flat_map(|bytes| {
            let six_bits_0 = (bytes[0] & 0b1111_1100) >> 2;
            let mut six_bits_1 = (bytes[0] & 0b0000_0011) << 4;
            let mut opt_six_bits_2: Option<u8> = None;
            let mut opt_six_bits_3: Option<u8> = None;

            if bytes.len() > 1 {
                six_bits_1 |= (bytes[1] & 0b1111_0000) >> 4;

                let mut six_bits_2 = (bytes[1] & 0b0000_1111) << 2;
                if bytes.len() > 2 {
                    six_bits_2 |= (bytes[2] & 0b1100_0000) >> 6;
                    opt_six_bits_3 = Some(bytes[2] & 0b0011_1111);
                }
                opt_six_bits_2 = Some(six_bits_2);
            }

            [
                Some(six_bits_0),
                Some(six_bits_1),
                opt_six_bits_2,
                opt_six_bits_3,
            ]
        })
        .map(|opt_six_bits| {
            if let Some(six_bits) = opt_six_bits {
                match six_bits {
                    0..=25 => Ok((b'A' + six_bits) as char),
                    26..=51 => Ok((b'a' + six_bits - 26) as char),
                    52..=61 => Ok((b'0' + six_bits - 52) as char),
                    62 => Ok((b'+') as char),
                    63 => Ok((b'/') as char),
                    _ => Err(BytesToBase64Error::OutOfRangeByte(six_bits)),
                }
            } else {
                Ok('=')
            }
        })
        .collect::<Result<String, BytesToBase64Error>>()
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Base64ToBytesError {
    #[error("Padding characters should only appear at the end of the base 64 string")]
    PaddingCharacterNotAtEnd,

    #[error("Unrecognized base 64 character: {0}")]
    UnrecognizedBase64Character(char),

    #[error(
        "At most 2 padding characters are allowed at the end of a base 64 string, but {0} found."
    )]
    ExcessPaddingChars(usize),

    #[error("Invalid base 64 inputs due to non-zero padding bits")]
    NonZeroPaddingBits,
}

pub fn base64_to_bytes(base64_str: &str) -> Result<Vec<u8>, Base64ToBytesError> {
    let mut padding_char_count: usize = 0;
    let mut six_bit_bytes = base64_str
        .as_bytes()
        .iter()
        .map(|&byte| {
            if byte != b'=' && padding_char_count > 0 {
                Err(Base64ToBytesError::PaddingCharacterNotAtEnd)
            } else {
                match byte {
                    b'A'..=b'Z' => Ok(byte - b'A'),
                    b'a'..=b'z' => Ok(26 + byte - b'a'),
                    b'0'..=b'9' => Ok(52 + byte - b'0'),
                    b'+' => Ok(62),
                    b'/' => Ok(63),
                    b'=' => {
                        padding_char_count += 1;
                        // treat as a zero, for convenience.
                        // (extra bytes will be filtered out at the end,
                        // based on the # of padding characters).
                        Ok(0)
                    }
                    _ => Err(Base64ToBytesError::UnrecognizedBase64Character(
                        byte as char,
                    )),
                }
            }
        })
        .collect::<Result<Vec<u8>, Base64ToBytesError>>()?;

    if padding_char_count > 2 {
        return Err(Base64ToBytesError::ExcessPaddingChars(padding_char_count));
    }

    let expected_byte_count = six_bit_bytes.len() / 4 * 3 - padding_char_count;

    // Add extra zero bytes at the end to make a multiple of 4 bytes
    let padding_bytes_required = 4 - six_bit_bytes.len() % 4;
    if padding_bytes_required != 4 {
        six_bit_bytes.reserve(padding_bytes_required);
        for _ in 0..padding_bytes_required {
            six_bit_bytes.push(0);
        }
    }

    let mut bytes: Vec<u8> = six_bit_bytes
        .chunks(4)
        .flat_map(|chunks| {
            if chunks.len() != 4 {
                panic!("The base 64 input string should have a multiple of 4 characters");
            }

            // byte0 is the first 6 bits of chunk 0 followed by the first 2 bits of chunk 1
            let byte0 = (chunks[0] << 2) | (chunks[1] >> 4);

            // byte1 is the last 4 bits of chunk 1 followed by the first 4 bits of chunk 2
            let byte1 = ((chunks[1] & 0x0f) << 4) | (chunks[2] >> 2);

            // byte2 is the last 2 bits of chunk 2, followed by all 6 bits of chunk 3
            let byte2 = ((chunks[2] & 0x03) << 6) | chunks[3];

            [byte0, byte1, byte2]
        })
        .collect();

    if bytes
        .drain(expected_byte_count..)
        .any(|extra_byte| extra_byte != 0)
    {
        Err(Base64ToBytesError::NonZeroPaddingBits)
    } else {
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::{base64_to_bytes, bytes_to_base64, Base64ToBytesError};

    mod bytes_to_base64_tests {
        use super::bytes_to_base64;

        #[test]
        fn test_bytes_to_base64_with_string_len_11() {
            // Based on example at https://en.wikipedia.org/wiki/Base64#Output_padding
            let input_str = "light work.";
            let base_64_str = bytes_to_base64(input_str.as_bytes()).unwrap();
            let expected_str = "bGlnaHQgd29yay4=";
            assert_eq!(expected_str, base_64_str);
        }

        #[test]
        fn test_bytes_to_base64_with_string_len_10() {
            // Based on example at https://en.wikipedia.org/wiki/Base64#Output_padding
            let input_str = "light work";
            let base_64_str = bytes_to_base64(input_str.as_bytes()).unwrap();
            let expected_str = "bGlnaHQgd29yaw==";
            assert_eq!(expected_str, base_64_str);
        }

        #[test]
        fn test_bytes_to_base64_with_string_len_9() {
            // Based on example at https://en.wikipedia.org/wiki/Base64#Output_padding
            let input_str = "light wor";
            let base_64_str = bytes_to_base64(input_str.as_bytes()).unwrap();
            let expected_str = "bGlnaHQgd29y";
            assert_eq!(expected_str, base_64_str);
        }

        #[test]
        fn test_bytes_to_base64_with_string_len_8() {
            // Based on example at https://en.wikipedia.org/wiki/Base64#Output_padding
            let input_str = "light wo";
            let base_64_str = bytes_to_base64(input_str.as_bytes()).unwrap();
            let expected_str = "bGlnaHQgd28=";
            assert_eq!(expected_str, base_64_str);
        }

        #[test]
        fn test_bytes_to_base64_with_string_len_7() {
            // Based on example at https://en.wikipedia.org/wiki/Base64#Output_padding
            let input_str = "light w";
            let base_64_str = bytes_to_base64(input_str.as_bytes()).unwrap();
            let expected_str = "bGlnaHQgdw==";
            assert_eq!(expected_str, base_64_str);
        }
    }

    mod base64_to_bytes_tests {
        use super::{base64_to_bytes, Base64ToBytesError};

        #[test]
        fn test_failing_case() {
            let base_64_str = "++d=";
            let base_64_bytes_result = base64_to_bytes(base_64_str);
            assert_eq!(
                base_64_bytes_result,
                Result::Err(Base64ToBytesError::NonZeroPaddingBits)
            );
        }
    }

    mod prop_tests {
        use super::{base64_to_bytes, bytes_to_base64, Base64ToBytesError};
        use proptest::{collection, *};

        proptest! {
            #[test]
            fn base64_chars_to_bytes_and_back_again_is_the_same(
                base_64_str in "([A-Za-z0-9+/]{4})*(|[A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{2}==)") {
                let base_64_bytes_result = base64_to_bytes(base_64_str.as_str());

                // Not all of these base 64 strings are valid, so first check the result
                if let Ok(base_64_bytes) = base_64_bytes_result {
                    let conv_str = bytes_to_base64(&base_64_bytes).unwrap();
                    assert_eq!(base_64_str, conv_str);
                } else {
                    // The only invalid base 64 strings should be those with non-zero padding bits
                    assert_eq!(base_64_bytes_result, Err(Base64ToBytesError::NonZeroPaddingBits));
                }
            }

            #[test]
            fn bytes_to_base64_and_back_again_is_the_same(
                byte_vec in collection::vec(0_u8..255, 0..20),
            ) {
                let base64_string = bytes_to_base64(&byte_vec[..]).unwrap();
                let conv_byte_vec = base64_to_bytes(base64_string.as_str()).unwrap();
                assert_eq!(byte_vec, conv_byte_vec);
            }
        }
    }
}
