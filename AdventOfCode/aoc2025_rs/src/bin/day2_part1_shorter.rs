use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day2_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 2 part 1 (shorter)", solve, 100_000);
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

    let mut init = start / pow_10;
    if init < pow_10 / 10 {
        init = pow_10 / 10;
    } else if init * (pow_10 + 1) < start {
        init += 1;
    }
    let mut fin = pow_10 - 1;
    let mut sum_of_rep_nums = 0;

    while init * (pow_10 + 1) <= end {
        if fin * (pow_10 + 1) > end {
            fin = end / pow_10;
            if fin * (pow_10 + 1) > end {
                fin -= 1;
            };
        }
        let sum_with_same_digit_count = (init..=fin).map(|i| i * (pow_10 + 1)).sum::<u64>();
        sum_of_rep_nums += sum_with_same_digit_count;
        init = pow_10;
        pow_10 *= 10;
        fin = pow_10 - 1;
        pow_100 *= 100;
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
