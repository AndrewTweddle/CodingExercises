use aoc2023::read_and_solve_and_time_more_runs;

fn main() {
    read_and_solve_and_time_more_runs(
        "data/day1_input.txt",
        "Day 1 part 2 (iterator approach)",
        get_sum_of_calibration_values,
        10_000,
    );
}

fn get_sum_of_calibration_values(contents: &str) -> u32 {
    contents
        .lines()
        .map(|ln| {
            let mut digit_iter = DigitIter::new(ln);
            let first = digit_iter.next().expect("A first digit was not found");
            let last = digit_iter.last().unwrap_or(first);
            first * 10 + last
        })
        .sum::<u32>()
}

// Structure for iterating over digits (numeric or textual)
struct DigitIter<'a> {
    digits_str: &'a str,
}

impl<'a> DigitIter<'a> {
    fn new(digits_str: &'a str) -> Self {
        Self { digits_str }
    }
}

const DIGIT_STRS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

impl Iterator for DigitIter<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut rem_str = self.digits_str;
        while let Some(ch) = rem_str.chars().next() {
            // Check for a digit
            if let Some(digit) = ch.to_digit(10) {
                self.digits_str = &rem_str[1..];
                return Some(digit);
            }

            // Check for the text of a digit
            for (num, num_str) in DIGIT_STRS.iter().enumerate() {
                if rem_str.starts_with(num_str) {
                    self.digits_str = &rem_str[1..];
                    return Some(num as u32 + 1);
                }
            }

            // Keep checking...
            rem_str = &rem_str[1..];
        }
        self.digits_str = rem_str;
        None
    }
}

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
