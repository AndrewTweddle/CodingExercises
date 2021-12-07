use std::fs::File;
use std::io::{BufRead, BufReader};

const DAYS_OF_BREEDING: usize = 80;

fn main() {
    let input_file = File::open("data/day6_input").unwrap();
    let br = BufReader::new(input_file);
    let days_left_string = br.lines().next().unwrap().unwrap();

    // Count the number of lanternfish by the number of days left until they procreate
    let mut counts_by_days_left = [0_u128; 9];
    for days_left in days_left_string
        .split(",")
        .map(|num_str| num_str.parse::<usize>().unwrap())
    {
        counts_by_days_left[days_left] += 1;
    }

    for day in 1..=DAYS_OF_BREEDING {
        // Instead of moving data around the array, just move an imaginary
        // index into the array which points to the "zero days left" slot for that day
        let eight_days_left_index = (day + 8) % 9 as usize;
        let six_days_left_index = (day + 6) % 9 as usize;

        // The new lanternfish are 8 days away from procreating.
        // They equal the number of day zero lanternfish on the previous day,
        // so that number is unchanged since yesterday's "0 days left" equals today's "8 days left".
        // But the previous day "0 days left" lanternfish
        // need to be added to the "6 days to go" slot...
        counts_by_days_left[six_days_left_index] += counts_by_days_left[eight_days_left_index];
    }

    let total_lanternfish: u128 = counts_by_days_left.iter().sum();

    println!(
        "Total lanternfish after {} days: {}",
        DAYS_OF_BREEDING, total_lanternfish
    );
}
