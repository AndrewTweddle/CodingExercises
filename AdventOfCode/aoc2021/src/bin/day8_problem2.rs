use std::time::Instant;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let start_time = Instant::now();
    let input_file = File::open("data/day8_input").unwrap();
    let br = BufReader::new(input_file);
    let output_value_sum: u64 = br
        .lines()
        .map(|ln| {
            let line = ln.unwrap();
            let (input_str, output_str) = line.split_once('|').unwrap();
            let letter_map = get_letter_mapping_from_inputs(input_str);
            decode_output_str(output_str, &letter_map)
        })
        .sum();
    let end_time = start_time.elapsed();
    println!("Sum of values is {}", output_value_sum);
    println!("Duration: {:?}", end_time);       // Duration: 553.259µs
}

fn get_letter_mapping_from_inputs(inputs: &str) -> HashMap<char, char> {
    let mut char_counts = HashMap::<char, usize>::new();
    for ch in inputs.chars().filter(|&ch| ch.is_alphabetic()) {
        char_counts
            .entry(ch)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }

    // To disambiguate a from c (both of which appear in 8 digits),
    // and d from g (which both appear in 7 digits),
    // add an extra 4 counts for each character in the digit with 4 characters...
    let chars_in_4_letter_digit = inputs
        .split(' ')
        .find(|&digit| digit.len() == 4)
        .unwrap();
    for ch in chars_in_4_letter_digit.chars() {
        char_counts
            .entry(ch)
            .and_modify(|value| *value += 4)
            .or_insert(4);
    }

    let char_to_char_mapping = char_counts
        .iter()
        .map(|(&ch, &count)| (ch, get_letter_from_counts(count).unwrap()))
        .collect();

    char_to_char_mapping
}

fn get_letter_from_counts(count: usize) -> Option<char> {
    match count {
        4 => Some('e'),
        7 => Some('g'),
        8 => Some('a'),
        10 => Some('b'),
        11 => Some('d'),
        12 => Some('c'),
        13 => Some('f'),
        _ => None,
    }
}

fn decode_output_str(output_str: &str, letter_mapping: &HashMap<char, char>) -> u64 {
    output_str
        .split_ascii_whitespace()
        .map(|output_digit_str| output_str_to_digit(output_digit_str, letter_mapping))
        .rev()
        .enumerate()
        .map(|(exponent, digit)| (digit as u64) * 10_u64.pow(exponent as u32))
        .sum::<u64>()
}

fn output_str_to_digit(output_str: &str, letter_mapping: &HashMap<char, char>) -> u8 {
    let digit_value: u8 = output_str
        .chars()
        .map(|ch| 1 << (*letter_mapping.get(&ch).unwrap() as u8 - b'a'))
        .sum();
    *VALUE_TO_DIGIT_MAP.get(&digit_value).unwrap()
}

lazy_static! {
    static ref VALUE_TO_DIGIT_MAP: HashMap<u8, u8> = {
        let mut m = HashMap::new();
        let digit_strs: [&str; 10] = [
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        ];
        for (i, &digit_str) in digit_strs.iter().enumerate() {
            m.insert(digit_str_to_digit_value(digit_str), i as u8);
        }
        m
    };
}

fn digit_str_to_digit_value(digit_str: &str) -> u8 {
    let digit_value = digit_str.chars().map(|ch| 1 << (ch as u8 - b'a')).sum();
    digit_value
}

#[cfg(test)]
mod tests {
    use super::decode_output_str;
    use super::get_letter_mapping_from_inputs;
    use super::output_str_to_digit;

    const TEST_INPUT: &str = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";

    #[test]
    fn test_letter_mapping() {
        let mapping = get_letter_mapping_from_inputs(TEST_INPUT);
        assert_eq!(mapping.get(&'a'), Some(&'c'));
        assert_eq!(mapping.get(&'b'), Some(&'f'));
        assert_eq!(mapping.get(&'c'), Some(&'g'));
        assert_eq!(mapping.get(&'d'), Some(&'a'));
        assert_eq!(mapping.get(&'e'), Some(&'b'));
        assert_eq!(mapping.get(&'f'), Some(&'d'));
        assert_eq!(mapping.get(&'g'), Some(&'e'));
    }

    #[test]
    fn test_digit_conversion() {
        let mapping = get_letter_mapping_from_inputs(TEST_INPUT);
        assert_eq!(output_str_to_digit("acedgfb", &mapping), 8);
        assert_eq!(output_str_to_digit("cdfbe", &mapping), 5);
        assert_eq!(output_str_to_digit("gcdfa", &mapping), 2);
        assert_eq!(output_str_to_digit("fbcad", &mapping), 3);
        assert_eq!(output_str_to_digit("dab", &mapping), 7);
        assert_eq!(output_str_to_digit("cefabd", &mapping), 9);
        assert_eq!(output_str_to_digit("cdfgeb", &mapping), 6);
        assert_eq!(output_str_to_digit("eafb", &mapping), 4);
        assert_eq!(output_str_to_digit("cagedb", &mapping), 0);
        assert_eq!(output_str_to_digit("ab", &mapping), 1);
    }

    #[test]
    fn test_decoding_output_str() {
        let mapping = get_letter_mapping_from_inputs(TEST_INPUT);
        assert_eq!(decode_output_str("cdfeb fcadb cdfeb cdbaf", &mapping), 5353);
    }
}
