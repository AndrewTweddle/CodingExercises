use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day2_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 2 part 1", solve, 1000);
}

fn solve(contents: &str) -> usize {
    contents
        .lines()
        .filter(|report| {
            let levels: Vec<i64> = report
                .split_whitespace()
                .map(|lvl_str| lvl_str.parse::<i64>().unwrap())
                .collect();
            let (low, hi) = if levels[1] - levels[0] >= 0 {
                (1, 3)
            } else {
                (-3, -1)
            };
            levels.windows(2).all(|pairs| {
                let diff = pairs[1] - pairs[0];
                diff >= low && diff <= hi
            })
        })
        .count()
}
