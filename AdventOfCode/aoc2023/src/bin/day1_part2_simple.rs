use aoc2023::read_and_solve_and_time_more_runs;

fn main() {
    read_and_solve_and_time_more_runs(
        "data/day1_input.txt",
        "Day 1 part 2 (simpler approach)",
        get_sum_of_calibration_values,
        10_000,
    );
}

fn get_sum_of_calibration_values(contents: &str) -> u32 {
    contents
        .lines()
        .map(|ln| {
            let mut digit_iter = ln.chars().enumerate().filter_map(|(i, ch)| {
                if let Some(digit) = ch.to_digit(10) {
                    Some(digit)
                } else {
                    let sub_str = &ln[i..]; // Note: taking slices repeatedly could be slow
                    DIGIT_STRS
                        .iter()
                        .enumerate()
                        .filter_map(|(digit, digit_str)| {
                            sub_str.starts_with(digit_str).then_some(digit as u32 + 1)
                        })
                        .next()
                }
            });
            let first = digit_iter.next().expect("A first digit was not found");
            let last = digit_iter.last().unwrap_or(first);
            first * 10 + last
        })
        .sum::<u32>()
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
