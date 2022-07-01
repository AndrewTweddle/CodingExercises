pub fn score_bytes_as_ascii_message(bytes: &[u8]) -> i64 {
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
        .sum::<i64>()
}
