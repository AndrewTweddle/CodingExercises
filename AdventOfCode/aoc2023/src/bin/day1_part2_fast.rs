use aoc2023::read_and_solve_and_time_more_runs;

fn main() {
    read_and_solve_and_time_more_runs(
        "data/day1_input.txt",
        "Day 1 part 2 (fast)",
        get_sum_of_calibration_values,
        10_000,
    );
}

fn get_sum_of_calibration_values(contents: &str) -> u32 {
    contents
        .lines()
        .map(|ln| {
            let bytes = ln.as_bytes();
            let n = bytes.len();

            let start_digit = (0..n)
                .filter_map(|i| to_digit_if_starts_with(&bytes[i..]))
                .next()
                .expect("No first digit was found") as u32;

            let end_digit = (0..n)
                .filter_map(|i| to_digit_if_starts_with(&bytes[(n - i - 1)..]))
                .next()
                .expect("No last digit was found") as u32;

            start_digit * 10 + end_digit
        })
        .sum::<u32>()
}

fn to_digit_if_starts_with(bytes: &[u8]) -> Option<u8> {
    if bytes.is_empty() {
        None
    } else if bytes[0].is_ascii_digit() {
        Some(bytes[0] - b'0')
    } else {
        DIGIT_LOOKUPS
            .iter()
            .enumerate()
            .filter_map(|(digit, lkp)| {
                // check whether bytes starts with the lookup text
                if lkp.len() > bytes.len() {
                    None
                } else {
                    bytes
                        .iter()
                        .zip(lkp.iter())
                        .all(|(a, b)| a == b)
                        .then_some(digit as u8 + 1)
                }
            })
            .next()
    }
}

const DIGIT_LOOKUPS: [&[u8]; 9] = [
    b"one",
    b"two",
    b"three",
    b"four",
    b"five",
    b"six",
    b"seven",
    b"eight",
    b"nine",
];

#[cfg(test)]
mod tests {
    use super::get_sum_of_calibration_values;

    #[test]
    fn test_part1_example() {
        let contents = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let total = get_sum_of_calibration_values(contents);
        assert_eq!(total, 281);
    }

    #[test]
    fn test_9963onefourthree6oneightq() {
        let contents = "9963onefourthree6oneightq";
        let total = get_sum_of_calibration_values(contents);
        assert_eq!(total, 98);
    }
}
