use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day3_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 3 part 1", solve, 10_000);
}

fn solve(contents: &str) -> u64 {
    contents
        .lines()
        .map(|ln| {
            let mut tens_digit: u8 = 0;
            let mut ones_digit: u8 = 0;
            for b in ln.bytes() {
                let next_digit = b - b'0';
                if ones_digit > tens_digit {
                    tens_digit = ones_digit;
                    ones_digit = next_digit;
                } else if next_digit > ones_digit {
                    ones_digit = next_digit;
                }
            }
            (10 * tens_digit + ones_digit) as u64
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    mod examples {
        use crate::solve;

        const EXAMPLE_1: &str = "987654321111111";
        const EXAMPLE_2: &str = "811111111111119";
        const EXAMPLE_3: &str = "234234234234278";
        const EXAMPLE_4: &str = "818181911112111";

        #[test]
        fn test_example_1() {
            assert_eq!(solve(EXAMPLE_1), 98);
        }

        #[test]
        fn test_example_2() {
            assert_eq!(solve(EXAMPLE_2), 89);
        }

        #[test]
        fn test_example_3() {
            assert_eq!(solve(EXAMPLE_3), 78);
        }

        #[test]
        fn test_example_4() {
            assert_eq!(solve(EXAMPLE_4), 92);
        }

        #[test]
        fn test_combined_example() {
            let example = [EXAMPLE_1, EXAMPLE_2, EXAMPLE_3, EXAMPLE_4].join("\n");
            assert_eq!(solve(&example), 357);
        }
    }
}
