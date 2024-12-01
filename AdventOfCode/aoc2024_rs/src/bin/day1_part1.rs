use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day1_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 1 part 1", solve, 10_000);
}

fn solve(contents: &str) -> i64 {
    let (mut lvalues, mut rvalues): (Vec<i64>, Vec<i64>) = contents
        .lines()
        .map(|line| {
            let (lval_str, rval_str) = line.split_once(' ').unwrap();
            let left = lval_str.trim().parse::<i64>().unwrap();
            let right = rval_str.trim().parse::<i64>().unwrap();
            (left, right)
        })
        .unzip();
    lvalues.sort();
    rvalues.sort();
    lvalues
        .iter()
        .zip(rvalues.iter())
        .map(|(l, r)| (r - l).abs())
        .sum()
}
