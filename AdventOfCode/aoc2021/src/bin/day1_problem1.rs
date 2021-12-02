use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "data/day1_input";
    let input = File::open(path).unwrap();
    let br = BufReader::new(input);
    let rows = br
        .lines()
        .map(|ln| ln.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let increasing_count = rows
        .iter()
        .zip(rows.iter().skip(1))
        .filter(|(&first, &second)| second > first)
        .count();
    println!("Count of increasing depths: {}", increasing_count);
}
