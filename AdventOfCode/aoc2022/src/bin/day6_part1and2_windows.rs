use std::collections::HashSet;

fn main() {
    let contents = std::fs::read_to_string("data/day6_input.txt").unwrap();
    println!("2022 day 6 part 1 answer: {}", solve_part1(contents.trim()));
    println!("2022 day 6 part 2 answer: {}", solve_part2(contents.trim()));
}

fn solve_part1(msg: &str) -> usize {
    get_pos_of_nth_consecutive_unique_char(msg, 4).unwrap()
}
fn solve_part2(msg: &str) -> usize {
    get_pos_of_nth_consecutive_unique_char(msg, 14).unwrap()
}

fn get_pos_of_nth_consecutive_unique_char(msg: &str, n: usize) -> Option<usize> {
    msg.as_bytes()
        .windows(n)
        .enumerate()
        .filter(|(_, w)| w.iter().cloned().collect::<HashSet<u8>>().len() == n)
        .map(|(i, _)| i + n)
        .next()
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    #[test]
    fn test_part1_examples() {
        assert_eq!(solve_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(solve_part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(solve_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(solve_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(solve_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(solve_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(solve_part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(solve_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(solve_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
