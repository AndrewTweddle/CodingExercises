use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day7_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 7 part 1", solve, 100);
}

fn solve(contents: &str) -> usize {
    contents
        .lines()
        .filter_map(|line| {
            let (test_value_str, nums_str) = line.split_once(':').unwrap();
            let test_value = test_value_str.parse::<usize>().unwrap();
            let nums: Vec<usize> = nums_str
                .split_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect();
            evaluate_to_test_value(test_value, &nums)
        })
        .sum::<usize>()
}

fn evaluate_to_test_value(test_value: usize, nums: &[usize]) -> Option<usize> {
    if !nums.is_empty() && matches_test_value(test_value, nums[0], &nums[1..]) {
        Some(test_value)
    } else {
        None
    }
}

fn matches_test_value(test_value: usize, partial_value: usize, rem_nums: &[usize]) -> bool {
    if rem_nums.is_empty() {
        test_value == partial_value
    } else {
        let next_num = rem_nums[0];
        matches_test_value(test_value, partial_value + next_num, &rem_nums[1..])
            || matches_test_value(test_value, partial_value * next_num, &rem_nums[1..])
    }
}
