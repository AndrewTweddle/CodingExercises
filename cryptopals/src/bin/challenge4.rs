use set1::hex::hex_str_to_bytes;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

const USE_LETTER_FREQUENCY: bool = false;

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/4.txt").expect("Could not open file");
    let (best_line, decrypted_bytes, score) = contents
        .lines()
        .map(|line| {
            let encrypted_bytes =
                hex_str_to_bytes(line.trim()).expect("Unable to convert hex to bytes");
            let (best_key, min_score) = (0..=255_u8)
                .map(|key| {
                    let decrypted_bytes = xor_bytes(&encrypted_bytes, key);
                    let min_score = if USE_LETTER_FREQUENCY {
                        sum_of_squared_frequency_deviations(&decrypted_bytes) as i64
                    } else {
                        // Convert to a score to minimize by negating
                        -(get_alpha_score(&decrypted_bytes) as i64)
                    };
                    (key, min_score)
                })
                .min_by_key(|&(_, min_score)| min_score)
                .unwrap();
            let decrypted_bytes = xor_bytes(&encrypted_bytes, best_key);
            (line, decrypted_bytes, min_score)
        })
        .min_by_key(|&(_, _, min_score)| min_score)
        .unwrap();

    let duration = start_time.elapsed();

    println!("Encrypted line: {}", best_line);
    println!("Score: {}", score);
    println!("Duration: {:?}", duration);

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

// From: http://mathcenter.oxford.emory.edu/site/math125/englishLetterFreqs/
const CHAR_FREQUENCIES: &[u8; 26] = b"etaoinshrdlcumwfgypbvkjxqz";
const PENALTY_FOR_NON_ALPHA: u64 = 100 * 100;

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

// Count the score of alphabetic characters
fn get_alpha_score(bytes: &Vec<u8>) -> u64 {
    bytes
        .iter()
        .map(|&byte| {
            let ch = byte as char;
            if ch.is_ascii_lowercase() {
                10
            } else if ch.is_ascii_uppercase() {
                5
            } else if ch == ' ' {
                3
            } else if ch.is_ascii_digit() {
                2
            } else if ch.is_ascii_punctuation() {
                1
            } else {
                0
            }
        })
        .sum::<u64>()
}

/*
Comparison of 3 approaches:
===========================

1. Naive alphabetic score:
--------------------------

Using an alphabetic score with 2 per lowercase and 1 per uppercase, gives the wrong decryption,
because the letters can all be shifted left by 1 without going out of the alphabetic range.
This illustrates the benefit of using the letter frequencies, even though it runs much slower.
Also counting spaces would have fixed this though (in this particular case, at least).

Encrypted line: 7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
Alphabetic character score (lowercase doubled): 47
Lossy message is: Mlt#wkbw#wkf#sbqwz#jp#ivnsjmd
Message is: Mlt#wkbw#wkf#sbqwz#jp#ivnsjmd

2. Revised alphanumeric score:
------------------------------

Using the revised alphanumeric score shown above,
which also takes spaces, numbers and punctuation into account,
gives the right answer. See the output below (release mode):

Encrypted line: 7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
Score: -250
Duration: 14.742798ms
Lossy message is: Now that the party is jumping

Message is: Now that the party is jumping

3. Sum of squared deviations of letter frequency sequence number from expected letter frequencies:
--------------------------------------------------------------------------------------------------

Using the letter frequency histogram approach, in release mode, gives...

Encrypted line: 7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f
Score: 11432
Duration: 131.169761ms
Lossy message is: Now that the party is jumping

Message is: Now that the party is jumping

*/
