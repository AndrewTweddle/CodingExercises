use aoc2023::read_and_solve_and_time_more_runs;
use std::str::FromStr;

fn main() {
    read_and_solve_and_time_more_runs("data/day6_input.txt", "Day 6 part 1", solve_part1, 10_000);
    read_and_solve_and_time_more_runs("data/day6_input.txt", "Day 6 part 2", solve_part2, 10_000);
}

fn solve_part1(contents: &str) -> u64 {
    let (line1, line2) = contents
        .split_once('\n')
        .expect("The input should have 2 lines");
    let time_iter = line1[5..]
        .split_whitespace()
        .map(|time_str| u64::from_str(time_str).expect("The times should all be integers"));
    let distance_iter = line2[9..]
        .split_whitespace()
        .map(|dist_str| u64::from_str(dist_str).expect("The distance should all be integers"));
    time_iter
        .zip(distance_iter)
        .map(|(time, distance)| get_number_of_winning_charge_times(time, distance))
        .product()
}

fn solve_part2(contents: &str) -> u64 {
    let (line1, line2) = contents
        .split_once('\n')
        .expect("The input should have 2 lines");
    let race_time = line1[5..].trim().replace(' ', "").parse::<u64>().unwrap();
    let record_distance = line2[9..].trim().replace(' ', "").parse::<u64>().unwrap();
    get_number_of_winning_charge_times(race_time, record_distance)
}

// Suppose a race last n seconds, and has a record distance of d.
// If we charge for t seconds, then the distance is t * (n - t).
// For what real values of t is the record equalled?
//     t * (n - t) = d
// iff t^2 - n * t + d = 0
// Using the quadratic formula, this has solutions for:
//     t = (n +/- sqrt(n^2 - 4 * d)) / 2
// The number of integer solutions that beat the record will be
// the number of integers that fall strictly between these 2 values.

fn get_number_of_winning_charge_times(race_time: u64, record_distance: u64) -> u64 {
    let n: f64 = race_time as f64;
    let d: f64 = record_distance as f64;
    let discriminant = n * n - 4.0 * d;
    if discriminant < 0.0 {
        // no solutions
        return 0;
    }
    let s = discriminant.sqrt();
    let start = (((n - s) / 2.0).floor() + 0.001) as u64 + 1;
    let end = (((n + s) / 2.0).ceil() + 0.001) as u64 - 1;
    end - start + 1
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part1_example() {
        let total = solve_part1(EXAMPLE);
        assert_eq!(total, 288);
    }

    #[test]
    fn test_part2_example() {
        let total = solve_part2(EXAMPLE);
        assert_eq!(total, 71503);
    }
}
