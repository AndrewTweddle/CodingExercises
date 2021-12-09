use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("data/day8_input").unwrap();
    let br = BufReader::new(input_file);
    let digit_count: usize = br
        .lines()
        .map(|ln| ln
            .unwrap()
            .split_once("|")
            .unwrap()
            .1
            .split(" ")
            .filter(|output| match output.len() {
                2..=4 => true,   // digit 1 has len 2, digit 7 has len 3, digit 4 has len 4
                7 => true,       // digit 8 has len 7
                _ => false,
            })
            .count()
        )
        .sum();
    println!("Digits 1, 4, 7 or 8 are in the outputs {} times", digit_count);
}