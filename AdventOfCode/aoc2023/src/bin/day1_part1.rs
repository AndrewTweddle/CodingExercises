fn main() {
    let contents = std::fs::read_to_string("data/day1_input.txt").expect("Input file not found");
    let total = get_sum_of_calibration_values(&contents);
    println!("Day 1 part 1 answer: {total}");
}

fn get_sum_of_calibration_values(contents: &str) -> u32 {
    contents
        .lines()
        .map(|ln| {
            let mut digit_iter = ln.chars().filter_map(|ch| ch.to_digit(10));
            let first = digit_iter.next().expect("A first digit was not found");
            let last = digit_iter.last().unwrap_or(first);
            first * 10 + last
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::get_sum_of_calibration_values;

    #[test]
    fn test_part1_example() {
        let contents = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let total = get_sum_of_calibration_values(contents);
        assert_eq!(total, 142);
    }
}
