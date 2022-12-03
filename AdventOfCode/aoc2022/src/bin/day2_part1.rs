use std::fs;

fn main() {
    let contents = fs::read_to_string("data/day2_input.txt").unwrap();
    let score: u32 = contents
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            score_round(bytes[0], bytes[2]) as u32
        })
        .sum();
    println!("Day 2 part 1: {score}");
}

fn score_round(opponent_byte: u8, my_byte: u8) -> u8 {
    (my_byte - b'W') + 3 * ((my_byte - opponent_byte + 2) % 3)
}

#[cfg(test)]
mod tests {
    use crate::score_round;

    #[test]
    fn test_a_vs_y() {
        assert_eq!(score_round(b'A', b'Y'), 8);
    }

    #[test]
    fn test_b_vs_x() {
        assert_eq!(score_round(b'B', b'X'), 1);
    }

    #[test]
    fn test_c_vs_z() {
        assert_eq!(score_round(b'C', b'Z'), 6);
    }
}