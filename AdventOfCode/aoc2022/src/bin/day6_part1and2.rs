fn main() {
    let contents = std::fs::read_to_string("data/day6_input.txt").unwrap();
    println!("2022 day 6 part 1 answer: {}", solve_part1(contents.trim()));
    println!("2022 day 6 part 2 answer: {}", solve_part2(contents.trim()));
}

fn solve_part1(msg: &str) -> usize { get_pos_of_nth_consecutive_unique_char(msg, 4).unwrap() }
fn solve_part2(msg: &str) -> usize { get_pos_of_nth_consecutive_unique_char(msg, 14).unwrap() }

fn get_pos_of_nth_consecutive_unique_char(msg: &str, n: usize) -> Option<usize> {
    // Work with the zero-based index of each character in the alphabet...
    let indices: Vec<usize> = msg.trim().bytes().map(|b| (b - b'a') as usize).collect();
    let mut counts = [0_i8; 26];  // count number of each character in the last n characters seen
    let mut distinct = 0;  // the number of distinct characters in the last n characters seen

    for (i, &ch) in indices.iter().enumerate() {
        if i >= n {
            // remove the character n+1 steps back from the batch of last n characters seen
            let old_ch = indices[i - n];
            counts[old_ch] -= 1;
            match counts[old_ch] {
                0 => distinct -= 1,
                1 => distinct += 1,
                _ => {},
            }
        }
        // Add the new character to the batch of last n characters seen
        counts[ch] += 1;
        match counts[ch] {
            1 => distinct += 1,
            2 => distinct -= 1,
            _ => {},
        }
        if distinct == n {
            return Some(i + 1);
        }
    }
    None
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