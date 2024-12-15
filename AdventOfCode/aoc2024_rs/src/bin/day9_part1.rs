use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::iter;

const INPUT_FILE_PATH: &str = "data/day9_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 9 part 1", solve, 1000);
}

fn solve(contents: &str) -> usize {
    let mut file_blocks = get_file_blocks_by_expanding_str(contents);
    move_file_blocks(&mut file_blocks);
    get_file_system_check_sum(&file_blocks)
}

fn get_file_blocks_by_expanding_str(contents: &str) -> Vec<Option<usize>> {
    let mut is_length_of_file = true;
    let mut file_id = 0;
    let expanded: Vec<Option<usize>> = contents
        .trim()
        .bytes()
        .map(|c| (c - b'0') as usize)
        .flat_map(|num| {
            if is_length_of_file {
                is_length_of_file = false;
                file_id += 1;
                iter::repeat_n(Some(file_id - 1), num)
            } else {
                is_length_of_file = true;
                iter::repeat_n(None, num)
            }
        })
        .collect();
    expanded
}

fn move_file_blocks(file_blocks: &mut [Option<usize>]) {
    let mut last_j = file_blocks.len();

    for i in 0..last_j {
        if file_blocks[i].is_some() {
            continue;
        }
        if last_j <= i {
            break;
        }
        for j in (i..last_j).rev() {
            if file_blocks[j].is_none() {
                continue;
            }
            file_blocks.swap(i, j);
            last_j = j;
            break;
        }
    }
}

fn get_file_system_check_sum(file_blocks: &[Option<usize>]) -> usize {
    file_blocks
        .iter()
        .enumerate()
        .filter(|(_, &v)| v.is_some())
        .map(|(i, &v)| i * v.unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // NB: This only works when there are at most 10 file id's
    fn file_blocks_to_str(file_blocks: &[Option<usize>]) -> String {
        file_blocks
            .iter()
            .map(|block| {
                if let Some(file_id) = block {
                    char::from_digit(*file_id as u32, 10).unwrap()
                } else {
                    '.'
                }
            })
            .collect::<String>()
    }

    fn str_to_file_blocks(file_blocks_str: &str) -> Vec<Option<usize>> {
        file_blocks_str
            .chars()
            .map(|block_str| {
                if block_str.is_digit(10) {
                    Some(block_str.to_digit(10).unwrap() as usize)
                } else {
                    None
                }
            })
            .collect()
    }

    const EXAMPLE_CONTENTS: &str = "2333133121414131402\n";
    const EXAMPLE_EXPANDED_FILE_BLOCKS_STR: &str = "00...111...2...333.44.5555.6666.777.888899";
    const EXAMPLE_MOVED_FILE_BLOCKS_STR: &str = "0099811188827773336446555566..............";

    #[test]
    fn test_expanding_example() {
        let file_blocks = get_file_blocks_by_expanding_str(EXAMPLE_CONTENTS);
        let expanded_str = file_blocks_to_str(&file_blocks);
        assert_eq!(&expanded_str, EXAMPLE_EXPANDED_FILE_BLOCKS_STR);
    }

    #[test]
    fn test_moving_blocks() {
        let mut file_blocks = str_to_file_blocks(EXAMPLE_EXPANDED_FILE_BLOCKS_STR);
        move_file_blocks(&mut file_blocks);
        let file_blocks_str = file_blocks_to_str(&file_blocks);
        assert_eq!(&file_blocks_str, EXAMPLE_MOVED_FILE_BLOCKS_STR);
    }

    #[test]
    fn test_check_sum() {
        let expected_check_sum = 1928;
        let file_blocks = str_to_file_blocks(EXAMPLE_MOVED_FILE_BLOCKS_STR);
        let check_sum = get_file_system_check_sum(&file_blocks);
        assert_eq!(check_sum, expected_check_sum);
    }

    #[test]
    fn check_example() {
        let expected_check_sum = 1928;
        let check_sum = solve(EXAMPLE_CONTENTS);
        assert_eq!(check_sum, expected_check_sum);
    }
}
