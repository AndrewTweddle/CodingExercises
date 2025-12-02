use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day1_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 1 part 1", solve, 10_000);
}

fn solve(contents: &str) -> i32 {
    contents
        .lines()
        .map(|line| {
            let (dir, times_str) = line.split_at(1);
            let abs_times = times_str.parse::<i64>().unwrap();
            if dir == "L" { -abs_times } else { abs_times }
        })
        .scan(50, |dial_pos, clicks| {
            *dial_pos += clicks;
            *dial_pos %= 100;
            Some(if *dial_pos == 0 { 1 } else { 0 })
        })
        .sum()
}
