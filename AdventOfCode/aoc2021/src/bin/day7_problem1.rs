use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("data/day7_input").unwrap();
    let br = BufReader::new(input_file);
    let first_line = br.lines().next().unwrap().unwrap();

    // The middle position (for odd number of crabs),
    // or anywhere on or between the two median positions (if even and medians are different),
    // is optimal because this balances the number of crabs on either side.
    // Any move of the target away from this point/points will increase
    // the total distance by the number of crabs moved away from
    // (including a crab at a median position) less the number moved towards.

    let mut positions: Vec<i64> = first_line
        .split(',')
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .collect();
    positions.sort();

    let median = positions[positions.len() / 2];
    let fuel: i64 = positions.iter().map(|&pos| (median - pos).abs()).sum();

    println!("Fuel required to reach position {}: {}", median, fuel);
}
