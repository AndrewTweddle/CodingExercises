use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::collections::HashMap;

const INPUT_FILE_PATH: &str = "data/day1_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 1 part 2", solve, 10_000);
}

fn solve(contents: &str) -> i64 {
    let mut counts: HashMap<i64, (i64, i64)> = HashMap::new();
    contents.lines().for_each(|line| {
        let (lval_str, rval_str) = line.split_once(' ').unwrap();
        let left = lval_str.trim().parse::<i64>().unwrap();
        let right = rval_str.trim().parse::<i64>().unwrap();
        counts.entry(left).or_default().0 += 1;
        counts.entry(right).or_default().1 += 1;
    });
    counts
        .iter()
        .map(|(&num, &(left_count, right_count))| left_count * right_count * num)
        .sum::<i64>()
}
