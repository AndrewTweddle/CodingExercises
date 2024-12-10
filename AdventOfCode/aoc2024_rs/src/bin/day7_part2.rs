use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day7_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 7 part 2", solve, 20);
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
            || matches_test_value(test_value, concat(partial_value, next_num), &rem_nums[1..])
    }
}

fn concat(mut curr_num: usize, next_num: usize) -> usize {
    let mut digit_reducer = next_num;
    loop {
        curr_num *= 10;
        digit_reducer /= 10;
        if digit_reducer == 0 {
            break;
        }
    }
    curr_num += next_num;
    curr_num
}

#[cfg(test)]
mod tests {
    use super::{concat, evaluate_to_test_value, solve};

    #[test]
    fn test_concatenate_with_zero() {
        let partial_value = 123;
        let concatenated = concat(partial_value, 0);
        assert_eq!(1230, concatenated);
    }

    #[test]
    fn test_concatenate_with_nonzero_1_digit_number() {
        let partial_value = 1234;
        let concatenated = concat(partial_value, 9);
        assert_eq!(12349, concatenated);
    }

    #[test]
    fn test_concatenate_with_2_digit_number() {
        let partial_value = 123;
        let concatenated = concat(partial_value, 40);
        assert_eq!(12340, concatenated);
    }

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_concatenation_with_test_value_of_156() {
        assert_eq!(evaluate_to_test_value(156, &[15, 6]), Some(156));
    }

    #[test]
    fn test_examples() {
        let answer = solve(EXAMPLE);
        assert_eq!(11387, answer);
    }
}
