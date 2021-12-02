use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "data/day1_input";
    let input = File::open(path).unwrap();
    let br = BufReader::new(input);
    let depths = br
        .lines()
        .map(|ln| ln.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let sliding_window_sums = depths
        .windows(3)
        .map(|sliding_window| sliding_window
            .iter()
            .sum::<u32>());
    let increasing_count = sliding_window_sums
        .clone()
        .zip(sliding_window_sums.skip(1))
        .filter(|&(sum1, sum2)| sum2 > sum1)
        .count();
    println!("Number of sliding windows that increased: {}", increasing_count);
}