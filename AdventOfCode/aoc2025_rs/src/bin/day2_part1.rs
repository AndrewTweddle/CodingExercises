use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day2_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 2 part 1", solve, 100_000);
}

fn solve(contents: &str) -> u64 {
    contents
        .trim_end()
        .split(',')
        .map(|range| {
            let (start_str, end_str) = range.split_once('-').unwrap();
            let (start, end) = (
                start_str.parse::<u64>().unwrap(),
                end_str.parse::<u64>().unwrap(),
            );
            sum_of_repeating_numbers_in_range(start, end)
        })
        .sum::<u64>()
}

fn sum_of_repeating_numbers_in_range(start: u64, end: u64) -> u64 {
    // Split the start number into halves by finding the power of 10 that splits it evenly
    let mut pow_10 = 1;
    let mut pow_100 = 1;
    while pow_100 <= start {
        pow_100 *= 100;
        pow_10 *= 10;
    }

    let mut start_half = if start * 10 < pow_100 {
        // There are an odd number of digits in the start number
        pow_10 / 10
    } else {
        let lower_half = start % pow_10;
        let upper_half = start / pow_10;
        if upper_half < lower_half {
            // The upper half repeated is below the starting number, so out of range.
            // Increment it. Then its repetition will be more than the starting number.
            upper_half + 1
        } else {
            upper_half
        }
    };

    let mut sum_of_rep_nums = 0;
    let mut rep_num = start_half * (pow_10 + 1);

    while rep_num <= end {
        let end_half = if pow_100 <= end {
            pow_10 - 1
        } else {
            let end_lower_half = end % pow_10;
            let end_upper_half = end / pow_10;
            if end_upper_half > end_lower_half {
                // The upper half repeated is above the ending number, so out of range.
                // Decrement it. Then its repetition will be less than the ending number.
                end_upper_half - 1
            } else {
                end_upper_half
            }
        };

        if end_half < start_half {
            break;
        }

        // The sum of "half numbers" from start_half to end_half (inclusive) is found
        // by taking the count of the numbers multiplied by the average number.
        // Then multiply this sum by (pow_10 + 1) to form the sum of the repeating numbers.
        let sum_of_rep_nums_with_same_digits =
            (end_half - start_half + 1) * (end_half + start_half) / 2 * (pow_10 + 1);
        sum_of_rep_nums += sum_of_rep_nums_with_same_digits;

        // Prepare for the next iteration
        start_half = pow_10;
        pow_10 *= 10;
        pow_100 *= 100;
        rep_num = start_half * (pow_10 + 1)
    }
    sum_of_rep_nums
}

#[cfg(test)]
mod tests {
    mod examples {
        use crate::solve;

        #[test]
        fn test_11_to_22() {
            assert_eq!(solve("11-22"), 33);
        }

        #[test]
        fn test_95_to_115() {
            assert_eq!(solve("95-115"), 99);
        }

        #[test]
        fn test_998_to_1012() {
            assert_eq!(solve("998-1012"), 1010);
        }

        #[test]
        fn test_1188511880_to_1188511890() {
            assert_eq!(solve("1188511880-1188511890"), 1188511885);
        }

        #[test]
        fn test_1698522_to_1698528() {
            assert_eq!(solve("1698522-1698528"), 0);
        }

        const FULL_EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
                                    1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
                                    824824821-824824827,2121212118-2121212124";

        #[test]
        fn test_full_example() {
            assert_eq!(solve(FULL_EXAMPLE), 1227775554);
        }
    }

    mod edge_cases {
        use crate::solve;

        #[test]
        fn test_with_upper_half_of_start_less_than_lower_half() {
            assert_eq!(solve("78-79"), 0);
            assert_eq!(solve("77-79"), 77);
        }

        #[test]
        fn test_with_lower_half_of_end_less_than_upper_half() {
            assert_eq!(solve("70-76"), 0);
            assert_eq!(solve("70-77"), 77);
        }

        #[test]
        fn test_1_to_100() {
            let total = (1..10).map(|i| 11 * i).sum::<u64>();
            assert_eq!(solve("1-100"), total);
        }

        #[test]
        fn test_1_to_10000() {
            let units_sum = (1..10).map(|i| 11 * i).sum::<u64>();
            let tens_sum = (10..100).map(|i| 101 * i).sum::<u64>();
            let total = units_sum + tens_sum;
            assert_eq!(solve("1-10000"), total);
        }

        #[test]
        fn test_1_to_100_000() {
            let units_sum = (1..10).map(|i| 11 * i).sum::<u64>();
            let tens_sum = (10..100).map(|i| 101 * i).sum::<u64>();
            let total = units_sum + tens_sum;
            assert_eq!(solve("1-100000"), total);
        }

        #[test]
        fn test_5_to_500_000() {
            let units_sum = (1..10).map(|i| 11 * i).sum::<u64>();
            let tens_sum = (10..100).map(|i| 101 * i).sum::<u64>();
            let hundreds_sum = (100..500).map(|i| 1001 * i).sum::<u64>();
            let total = units_sum + tens_sum + hundreds_sum;
            assert_eq!(solve("5-500000"), total);
        }

        #[test]
        fn test_50_to_500_000() {
            let units_sum = (5..10).map(|i| 11 * i).sum::<u64>();
            let tens_sum = (10..100).map(|i| 101 * i).sum::<u64>();
            let hundreds_sum = (100..500).map(|i| 1001 * i).sum::<u64>();
            let total = units_sum + tens_sum + hundreds_sum;
            assert_eq!(solve("50-500000"), total);
        }
    }
}
