use aoc2024_rs::read_and_solve_and_time_more_runs;
use regex::Regex;

const INPUT_FILE_PATH: &str = "data/day3_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 3 part 1", solve, 1000);
}

fn solve(contents: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(contents)
        .map(|num_pairs| {
            let (_, [num1_str, num2_str]) = num_pairs.extract();
            let num1: i64 = num1_str.parse().unwrap();
            let num2: i64 = num2_str.parse().unwrap();
            num1 * num2
        })
        .sum::<i64>()
}
