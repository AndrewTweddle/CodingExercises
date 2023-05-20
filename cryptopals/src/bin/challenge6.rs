use cryptopals::base64::base64_to_bytes;
use cryptopals::ciphers::repeating_key_xor::get_best_key_sizes_and_likely_decryptions;
use std::fs;
use std::time::Instant;

const MAX_KEY_SIZE: usize = 40;
const NUM_KEY_SIZES_TO_TRY: usize = 3;
const NUM_HAMMING_DISTANCE_SAMPLES: usize = 4;

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/6.txt").expect("Could not open file");
    let contents_str: Vec<&str> = contents.lines().collect();
    let base64_text = contents_str.join("");
    let encrypted_bytes =
        base64_to_bytes(base64_text.as_str()).expect("Unable to convert base 64 string to bytes");
    let key_size_and_plain_text_candidates = get_best_key_sizes_and_likely_decryptions(
        &encrypted_bytes,
        MAX_KEY_SIZE,
        NUM_HAMMING_DISTANCE_SAMPLES,
        NUM_KEY_SIZES_TO_TRY,
    );

    for (index, (key_size, plain_text)) in key_size_and_plain_text_candidates.iter().enumerate() {
        println!("{} - Key size: {}", index + 1, key_size);
        println!("  Plain text: {}", plain_text);
        println!();
    }

    println!("Duration: {:?}", start_time.elapsed());
}
