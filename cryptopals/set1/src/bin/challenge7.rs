use set1::base64::base64_to_bytes;
use set1::ciphers::aes::decrypt_aes_128_ecb;
use std::fs;
use std::time::Instant;

const KEY: &[u8; 16] = b"YELLOW SUBMARINE";

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/7.txt").expect("Unable to read input file 7.txt");
    let base64_contents: String = contents.lines().collect();
    let encrypted_bytes = base64_to_bytes(&base64_contents).expect("Unable to parse base 64 text");
    let decrypted_bytes =
        decrypt_aes_128_ecb(KEY, &encrypted_bytes).expect("Unable to decrypt using AES 128 ECB");
    let plain_text = String::from_utf8_lossy(&decrypted_bytes);
    println!("Plain text: {}", plain_text);
    let duration = start_time.elapsed();
    println!("Duration: {:?}", duration);
}
