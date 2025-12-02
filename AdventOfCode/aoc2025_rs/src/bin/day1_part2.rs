use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day1_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 1 part 2", solve, 10_000);
}

fn solve(contents: &str) -> i32 {
    contents
        .lines()
        .map(|line| {
            let (dir, times_str) = line.split_at(1);
            let abs_times = times_str.parse::<i32>().unwrap();
            if dir == "L" { -abs_times } else { abs_times }
        })
        .scan(50, |dial_pos, clicks| {
            *dial_pos += clicks;
            let num_passes_through_zero: i32 = if *dial_pos <= 0 {
                if clicks == *dial_pos {
                    // Started from zero, so don't count as passing through zero
                    -*dial_pos / 100
                } else {
                    // Passed through zero at least once
                    1 - *dial_pos / 100
                }
            } else {
                *dial_pos / 100
            };

            // Wrap around (so that the position is between 0 and 99)
            *dial_pos %= 100;
            if *dial_pos < 0 {
                *dial_pos += 100;
            }

            Some(num_passes_through_zero)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68\n\
        L30\n\
        R48\n\
        L5\n\
        R60\n\
        L55\n\
        L1\n\
        L99\n\
        R14\n\
        L82\n";

    #[test]
    fn test_example() {
        let answer = solve(TEST_INPUT);
        assert_eq!(answer, 6);
    }
}
