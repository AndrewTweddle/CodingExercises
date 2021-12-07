use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let mut cloned_counts_by_days_left = counts_by_days_left;

    let total_lanternfish_part1 = get_total_lanternfish_after(&mut cloned_counts_by_days_left, 80);
    let total_lanternfish_part2 = get_total_lanternfish_after(&mut counts_by_days_left, 256);

    println!(
        "Total lanternfish after 80 days: {}",
        total_lanternfish_part1
    );
    println!(
        "Total lanternfish after 256 days: {}",
        total_lanternfish_part2
    );
}

fn get_total_lanternfish_after(counts_by_days_left: &mut [u128], days_of_breeding: usize) -> u128 {
    for day in 1..=days_of_breeding {
        // Instead of moving data around the array, just move an imaginary
        // index into the array which points to the "zero days left" slot for that day
        let eight_days_left_index = (day + 8) % 9 as usize;
        let six_days_left_index = (day + 6) % 9 as usize;

        // The new lanternfish are 8 days away from procreating.
        // They equal the number of day zero lanternfish on the previous day,
        // so that number is unchanged since yesterday's "0 days left" slot
        // points to today's "8 days left" after the index moves on.
        // But the previous day "0 days left" lanternfish
        // still need to be added to the "6 days to go" slot...
        counts_by_days_left[six_days_left_index] += counts_by_days_left[eight_days_left_index];
    }

    counts_by_days_left.iter().sum::<u128>()
}
