use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day11_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 11 part 2", solve, 100);
}

const BLINK_COUNT: usize = 75;
const MAX_MEMO_I: usize = 2024;

type Memo = Vec<Vec<Option<usize>>>;

fn solve(contents: &str) -> usize {
    solve_after_n_blinks(contents, BLINK_COUNT)
}

fn solve_after_n_blinks(contents: &str, n: usize) -> usize {
    let mut memo: Memo = vec![vec![None; BLINK_COUNT + 1]; MAX_MEMO_I + 1];
    contents
        .split_ascii_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .map(|i| count_after_processing_n_times(i, n, &mut memo))
        .sum()
}

fn count_after_processing_n_times(i: u128, n: usize, memo: &mut Memo) -> usize {
    if n == 0 {
        return 1;
    }
    if i <= MAX_MEMO_I as u128 {
        if let Some(count) = memo[i as usize][n] {
            return count;
        }
    };
    let count = if i == 0 {
        count_after_processing_n_times(1, n - 1, memo)
    } else if let Some((a, b)) = split_if_even(i) {
        count_after_processing_n_times(a, n - 1, memo)
            + count_after_processing_n_times(b, n - 1, memo)
    } else {
        count_after_processing_n_times(i * 2024, n - 1, memo)
    };
    if i <= MAX_MEMO_I as u128 {
        memo[i as usize][n] = Some(count);
    };
    count
}

fn split_if_even(i: u128) -> Option<(u128, u128)> {
    let mut pow_of_100 = 100;
    let mut pow_of_10 = 10;

    while i / pow_of_100 > 0 {
        pow_of_100 *= 100;
        pow_of_10 *= 10;
    }

    if (i * 10) / pow_of_100 == 0 {
        // `i` must have an odd number of digits
        None
    } else {
        Some((i / pow_of_10, i % pow_of_10))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "125 17";
    const COUNT_AFTER_1_BLINK: usize = 3; // 253000 1 7
    const COUNT_AFTER_2_BLINKS: usize = 4; // 253 0 2024 14168
    const COUNT_AFTER_3_BLINKS: usize = 5; // 512072 1 20 24 28676032
    const COUNT_AFTER_4_BLINKS: usize = 9; // 512 72 2024 2 0 2 4 2867 6032

    #[test]
    fn test_1_blink() {
        let count = solve_after_n_blinks(EXAMPLE, 1);
        assert_eq!(count, COUNT_AFTER_1_BLINK);
    }

    #[test]
    fn test_2_blinks() {
        let count = solve_after_n_blinks(EXAMPLE, 2);
        assert_eq!(count, COUNT_AFTER_2_BLINKS);
    }

    #[test]
    fn test_3_blinks() {
        let count = solve_after_n_blinks(EXAMPLE, 3);
        assert_eq!(count, COUNT_AFTER_3_BLINKS);
    }

    #[test]
    fn test_4_blinks() {
        let count = solve_after_n_blinks(EXAMPLE, 4);
        assert_eq!(count, COUNT_AFTER_4_BLINKS);
    }
}
