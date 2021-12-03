use set1::hex::hex_str_to_bytes;
use std::collections::HashMap;

const ENCRYPTED_HEX: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

// From: http://mathcenter.oxford.emory.edu/site/math125/englishLetterFreqs/
const CHAR_FREQUENCIES: &[u8; 26] = b"etaoinshrdlcumwfgypbvkjxqz";

const PENALTY_FOR_NON_ALPHA: u64 = 100 * 100;

fn main() {
    let encrypted_bytes = hex_str_to_bytes(ENCRYPTED_HEX);
    let best_key = (0..=255_u8)
        .map(|key| {
            let decrypted_bytes = xor_bytes(&encrypted_bytes, key);
            let squared_devs = sum_of_squared_frequency_deviations(&decrypted_bytes);
            println!(
                "Message attempted: {}",
                String::from_utf8_lossy(&decrypted_bytes)
            );
            println!("    Key: {}", key);
            println!("  Score: {}\n", squared_devs);
            (key, squared_devs)
        })
        .min_by_key(|&(_, squared_deviation)| squared_deviation)
        .unwrap()
        .0;
    let decrypted_bytes = xor_bytes(&encrypted_bytes, best_key);

    // First print the message in a failsafe way...
    let lossy_message = String::from_utf8_lossy(&decrypted_bytes);
    println!("Lossy message is: {}", lossy_message);

    // Now attempt to convert to UTF8 (at the risk of failure, if there are invalid characters)...
    let message = String::from_utf8(decrypted_bytes).expect("Could not convert the bytes to UTF-8");
    println!("Message is: {}", message);
}

fn xor_bytes(bytes: &Vec<u8>, key: u8) -> Vec<u8> {
    bytes.iter().map(|&byte| byte ^ key).collect::<Vec<u8>>()
}

// Take the squared deviation from the position in the sequence of letter frequencies...
fn sum_of_squared_frequency_deviations(bytes: &Vec<u8>) -> u64 {
    let mut histogram: HashMap<u8, usize> = HashMap::new();
    for &byte in bytes {
        histogram
            .entry(byte)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    histogram
        .keys()
        .map(|&key| {
            let ch = key as char;
            if ch == ' ' {
                Some(key)
            } else if ch.is_ascii_alphabetic() {
                if ch.is_ascii_lowercase() {
                    Some(key)
                } else {
                    Some(ch.to_ascii_lowercase() as u8)
                }
            } else {
                None
            }
        })
        .enumerate()
        .map(|(actual_index, opt_key)| {
            match opt_key {
                Some(b' ') => 0, // No penalty for spaces
                Some(key) => {
                    let ideal_index = CHAR_FREQUENCIES
                        .iter()
                        .position(|&ch_u8| key == ch_u8)
                        .unwrap();
                    let offset_from_index = (actual_index as i64) - (ideal_index as i64);

                    // Take the square of the differences in position...
                    (offset_from_index * offset_from_index) as u64
                }
                _ => PENALTY_FOR_NON_ALPHA,
            }
        })
        .sum()
}
