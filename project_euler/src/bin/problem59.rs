use std::fs;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let data_file_path = "data/0059_cipher.txt";
    let text = fs::read_to_string(data_file_path).unwrap();
    let cipher_bytes: Vec<u8> = text.split(',').map(|i| i.parse::<u8>().unwrap()).collect();

    let mut best_score: u64 = 0;
    let mut best_plain_text: Vec<u8> = Vec::with_capacity(cipher_bytes.len());
    for a in b'a'..=b'z' {
        for b in b'a'..=b'z' {
            for c in b'a'..=b'z' {
                let key = [a, b, c];
                let decoded_iter = key
                    .iter()
                    .cycle()
                    .zip(cipher_bytes.iter())
                    .map(|(&k, &encoded_byte)| k ^ encoded_byte);
                let score = decoded_iter.clone().map(score_char).sum();
                if score > best_score {
                    best_score = score;
                    best_plain_text.clear();
                    best_plain_text.extend(decoded_iter);
                }
            }
        }
    }
    let answer: u64 = best_plain_text.iter().map(|&byte| byte as u64).sum();
    let plain_text = String::from_utf8(best_plain_text).unwrap();
    let duration = start_time.elapsed();
    println!("Best score: {}\n", best_score);
    println!("Plain text: {:?}\n", plain_text);
    println!("Duration incl reading text file: {:?}\n", duration);
    println!("Sum of ASCII values: {answer}");
}

fn score_char(c: u8) -> u64 {
    match c {
        b'a'..=b'z' => 20,
        b'A'..=b'Z' => 10,
        b' ' => 5,
        b'.' => 3,
        b',' | b';' | b'!' | b'?' => 2,
        b'0'..=b'9' => 1,
        _ => 0,
    }
}
