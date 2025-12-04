use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day3_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 3 part 2", solve, 10_000);
}

fn solve(contents: &str) -> u64 {
    contents
        .lines()
        .map(|ln| {
            let mut digits: [u8; 13] = [0; 13];
            for b in ln.bytes() {
                digits[12] = b - b'0';
                for i in 0..12 {
                    if digits[i] < digits[i + 1] {
                        digits.copy_within(i + 1..13, i);
                        break;
                    }
                }
            }
            digits[0..12]
                .iter()
                .fold(0_u64, |left, &right| 10 * left + right as u64)
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
            assert_eq!(solve(EXAMPLE_1), 987654321111);
        }

        #[test]
        fn test_example_2() {
            assert_eq!(solve(EXAMPLE_2), 811111111119);
        }

        #[test]
        fn test_example_3() {
            assert_eq!(solve(EXAMPLE_3), 434234234278);
        }

        #[test]
        fn test_example_4() {
            assert_eq!(solve(EXAMPLE_4), 888911112111);
        }

        #[test]
        fn test_combined_example() {
            let example = [EXAMPLE_1, EXAMPLE_2, EXAMPLE_3, EXAMPLE_4].join("\n");
            assert_eq!(solve(&example), 3121910778619);
        }
    }
}
