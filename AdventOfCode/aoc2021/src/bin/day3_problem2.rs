use std::fs::File;
use std::io::{BufRead, BufReader};

const BIT_COUNT: usize = 12;

fn main() {
    let input_file = File::open("data/day3_input").unwrap();
    let br = BufReader::new(input_file);
    let bytes: Vec<u32> = br
        .lines()
        .map(|ln| u32::from_str_radix(ln.unwrap().as_str(), 2).unwrap())
        .collect();
    let (oxygen_generator_rating, co2_scrubber_rating) = get_ratings(&bytes, BIT_COUNT).unwrap();
    println!(
        "product of ratings = {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}

fn get_ratings(bytes: &[u32], bit_count: usize) -> Option<(u32, u32)> {
    let oxygen_generator_rating = filter_bytes(bytes, bit_count, false);
    let co2_scrubber_rating = filter_bytes(bytes, bit_count, true);
    oxygen_generator_rating.zip(co2_scrubber_rating)
}

fn filter_bytes(bytes: &[u32], bit_count: usize, use_min: bool) -> Option<u32> {
    let mut mask: u32 = 1 << bit_count;
    let mut rem_bytes: Vec<u32> = bytes.to_owned();
    for _ in 0..bit_count {
        mask >>= 1;
        let (zeroes, ones): (Vec<u32>, Vec<u32>) =
            rem_bytes.iter().partition(|&byte| byte & mask == 0);

        rem_bytes = if use_min {
            if zeroes.is_empty() {
                ones
            } else if ones.is_empty() || zeroes.len() <= ones.len() {
                zeroes
            } else {
                ones
            }
        } else if ones.len() >= zeroes.len() {
            ones
        } else {
            zeroes
        };
        if rem_bytes.len() == 1 {
            return Some(rem_bytes[0]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::get_ratings;

    #[test]
    fn test_example() {
        let bit_count: usize = 5;
        let bytes: Vec<u32> = vec![
            0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
            0b11001, 0b00010, 0b01010,
        ];
        let ratings = get_ratings(&bytes, bit_count);
        assert_eq!(ratings, Some((23, 10)));
    }
}
