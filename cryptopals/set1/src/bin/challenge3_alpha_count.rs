use set1::hex::hex_str_to_bytes;

const ENCRYPTED_HEX: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

fn main() {
    let encrypted_bytes = hex_str_to_bytes(ENCRYPTED_HEX);
    let best_key = (0..=255_u8)
        .map(|key| {
            let decrypted_bytes = xor_bytes(&encrypted_bytes, key);
            let alpha_score = get_alpha_score(&decrypted_bytes);
            (key, alpha_score)
        })
        .max_by_key(|&(_, alpha_score)| alpha_score)
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

// Count the number of alphabetic characters, doubling lowercase
// (since we want to find more lowercase than uppercase, instead of the inverted case solution)
fn get_alpha_score(bytes: &Vec<u8>) -> u64 {
    bytes
        .iter()
        .map(|&byte| {
            let ch = byte as char;
            if ch.is_ascii_lowercase() {
                2
            } else if ch.is_ascii_uppercase() {
                1
            } else {
                0
            }
        })
        .sum::<u64>()
}
