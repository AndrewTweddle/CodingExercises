use aoc2025_rs::load_and_solve_and_benchmark;
use std::collections::BTreeSet;

const INPUT_FILE_PATH: &str = "data/day2_input.txt";

type Set<T> = BTreeSet<T>; // Slightly faster than HashSet<T>

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 2 part 2", solve, 10_000);
}

fn solve(contents: &str) -> u64 {
    let mut repeating_numbers: Set<u64> = Set::new();
    contents.trim_end().split(',').for_each(|range| {
        let (start_str, end_str) = range.split_once('-').unwrap();
        let (start, end) = (
            start_str.parse::<u64>().unwrap(),
            end_str.parse::<u64>().unwrap(),
        );
        expand_set_with_repeating_numbers_in_range(&mut repeating_numbers, start, end)
    });
    repeating_numbers.iter().sum::<u64>()
}

fn expand_set_with_repeating_numbers_in_range(
    repeating_numbers: &mut Set<u64>,
    start: u64,
    end: u64,
) {
    let max_digit_count = end.to_string().len() / 2;
    for digit_count in 1..=max_digit_count {
        expand_set_with_repeating_numbers_in_range_with_digit_count(
            repeating_numbers,
            start,
            end,
            digit_count,
        );
    }
}

fn expand_set_with_repeating_numbers_in_range_with_digit_count(
    repeating_numbers: &mut Set<u64>,
    start: u64,
    end: u64,
    digit_count: usize,
) {
    if start > end {
        return;
    }

    // The repeater is the number to multiply a sequence of digits by to repeat it digit_count times
    let mut repeater = 1;
    let first_pow_10 = 10_u64.pow(digit_count as u32);

    // Find the power of 10 that extracts the last set of up to digit_count digits from `start`
    let mut last_pow_10 = 1;
    while last_pow_10 * first_pow_10 <= start {
        last_pow_10 *= first_pow_10;
        repeater += last_pow_10;
    }

    // Check if there were enough groups of digits for any repetitions to occur.
    if last_pow_10 == 1 {
        // There weren't enough digits in `start` to evenly divide by the number of digits.
        // So start at the first number that has the required number of digits.
        // We do this because `end` might have enough digits, even though `start` doesn't.
        expand_set_with_repeating_numbers_in_range_with_digit_count(
            repeating_numbers,
            first_pow_10,
            end,
            digit_count,
        );
        return;
    }

    let mut init = start / last_pow_10;
    if init < first_pow_10 / 10 {
        // There aren't enough digits in `start` to evenly divide by the number of digits.
        // So start at the first number that has the required number of digits.
        init = first_pow_10 / 10;
    } else if init * repeater < start {
        // The start number is higher than the repetition of the first few digits,
        // So increment the first set of digits to be repeated, so that it will be in range.
        init += 1;
    }

    // Detect when we have more digits than end and return None to stop further iterations
    if init * repeater > end {
        return;
    }

    while init * repeater <= end {
        let mut fin = first_pow_10 - 1;
        if fin * repeater > end {
            fin = end / last_pow_10;
            if fin * repeater > end {
                // The end number is lower than the repetition of the last few digits.
                // So decrement the last set of digits to be repeated, so that it will be in range.
                fin -= 1;
            };
        }
        repeating_numbers.extend((init..=fin).map(|i| i * repeater));

        // Prepare for the next iteration
        init = first_pow_10 / 10;
        last_pow_10 *= first_pow_10;
        repeater += last_pow_10;
    }
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
            assert_eq!(solve("95-115"), 99 + 111);
        }

        #[test]
        fn test_998_to_1012() {
            assert_eq!(solve("998-1012"), 999 + 1010);
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
            assert_eq!(solve(FULL_EXAMPLE), 4174379265);
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
    }
}
