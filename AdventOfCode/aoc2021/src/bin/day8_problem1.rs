use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("data/day8_input").unwrap();
    let br = BufReader::new(input_file);
    let digit_count: usize = br
        .lines()
        .map(|ln| ln
            .unwrap()
            .split_once('|')
            .unwrap()
            .1
            .split(' ')
            .filter(|output| matches!(output.len(), 2..=4 | 7))
            .count()
        )
        .sum();
    println!("Digits 1, 4, 7 or 8 are in the outputs {} times", digit_count);
}