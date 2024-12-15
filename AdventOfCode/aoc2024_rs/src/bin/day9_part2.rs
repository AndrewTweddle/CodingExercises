use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day9_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 9 part 2", solve, 10);
}

#[derive(Debug, Copy, Clone)]
struct File {
    num_file_blocks: usize,
    file_id: usize,
}

impl File {
    fn new(num_file_blocks: usize, file_id: usize) -> Self {
        Self {
            num_file_blocks,
            file_id,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Contiguity {
    Empty(usize),
    File(File),
}

fn solve(contents: &str) -> usize {
    let mut contiguities = get_contiguities_by_expanding_str(contents);
    move_files(&mut contiguities);
    get_file_system_check_sum(&contiguities)
}

fn get_contiguities_by_expanding_str(contents: &str) -> Vec<Contiguity> {
    let mut is_file = true;
    let mut file_id = 0;
    let expanded: Vec<Contiguity> = contents
        .trim()
        .bytes()
        .map(|c| {
            let num = (c - b'0') as usize;
            if is_file {
                is_file = false;
                file_id += 1;
                Contiguity::File(File::new(num, file_id - 1))
            } else {
                is_file = true;
                Contiguity::Empty(num)
            }
        })
        .collect();
    expanded
}

fn move_files(contiguities: &mut Vec<Contiguity>) {
    // find the last file
    let last_file_id = contiguities
        .iter()
        .rev()
        .filter_map(|contig| {
            if let Contiguity::File(last_file) = contig {
                Some(last_file.file_id)
            } else {
                None
            }
        })
        .next()
        .unwrap();

    // move each file from last to first
    for file_id in (0..=last_file_id).rev() {
        move_blocks_for_file(contiguities, file_id);
    }
}

fn move_blocks_for_file(contiguities: &mut Vec<Contiguity>, file_id: usize) {
    let (j, file_to_move) = contiguities
        .iter()
        .enumerate()
        .filter_map(|(index, c)| match c {
            Contiguity::File(file) if file.file_id == file_id => Some((index, *file)),
            _ => None,
        })
        .next()
        .unwrap();

    // Find a matching gap for the file to be moved to, searching from left to right.
    // Make sure the file is moved leftwards, or not at all.
    let mut gap_iter = contiguities
        .iter_mut()
        .take(j)
        .enumerate()
        .filter_map(|(index, contig)| match contig {
            Contiguity::Empty(blocks) if *blocks >= file_to_move.num_file_blocks => {
                let block_size = *blocks;
                Some((index, contig, block_size))
            }
            _ => None,
        });

    // Only move the block if a suitable gap was found for it
    if let Some((i, first_gap, gap_size)) = gap_iter.next() {
        // Move the file into the correct position
        if gap_size == file_to_move.num_file_blocks {
            // the gap is replaced by the block
            contiguities.swap(i, j);
        } else {
            // the gap is reduced in size, and the block to move
            *first_gap = Contiguity::Empty(gap_size - file_to_move.num_file_blocks);

            // Move the file
            contiguities[j] = Contiguity::Empty(file_to_move.num_file_blocks);
            contiguities.insert(i, Contiguity::File(file_to_move));
        }
        // This might lead to adjacent empty blocks.
        // Don't bother consolidating them, as a file won't ever be moved to later gaps.
    }
}

fn get_file_system_check_sum(contiguities: &[Contiguity]) -> usize {
    let mut start_index = 0;
    contiguities
        .iter()
        .map(|contig| match contig {
            Contiguity::Empty(num_blocks) => {
                start_index += *num_blocks;
                0
            }
            Contiguity::File(file) => {
                let mut checksum = (file.num_file_blocks - 1) * file.num_file_blocks / 2;
                checksum += file.num_file_blocks * start_index;
                checksum *= file.file_id;
                start_index += file.num_file_blocks;
                checksum
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    // NB: This only works when there are at most 10 file id's
    fn contiguities_to_str(contiguities: &[Contiguity]) -> String {
        contiguities
            .iter()
            .flat_map(|contig| match contig {
                Contiguity::File(file) => iter::repeat_n(
                    char::from_digit(file.file_id as u32, 10).unwrap(),
                    file.num_file_blocks,
                ),
                Contiguity::Empty(num_blocks) => iter::repeat_n('.', *num_blocks),
            })
            .collect::<String>()
    }

    fn str_to_contiguities(file_blocks_str: &str) -> Vec<Contiguity> {
        let mut last_char: char = '.';
        let mut last_char_count: usize = 0;

        let mut contiguities: Vec<Contiguity> = Vec::with_capacity(file_blocks_str.len());

        for ch in file_blocks_str.chars() {
            if ch == last_char {
                last_char_count += 1;
                if ch == '.' {
                    *contiguities.last_mut().unwrap() = Contiguity::Empty(last_char_count);
                } else {
                    match contiguities.last_mut() {
                        Some(Contiguity::File(file)) => {
                            file.num_file_blocks = last_char_count;
                        }
                        _ => panic!("File expected, but not found"),
                    }
                }
            } else {
                last_char = ch;
                last_char_count = 1;
                if ch == '.' {
                    contiguities.push(Contiguity::Empty(1));
                } else {
                    let file_id = ch.to_digit(10).unwrap() as usize;
                    contiguities.push(Contiguity::File(File::new(last_char_count, file_id)));
                }
            }
        }
        contiguities
    }

    const EXAMPLE_CONTENTS: &str = "2333133121414131402\n";
    const EXAMPLE_EXPANDED_FILE_BLOCKS_STR: &str = "00...111...2...333.44.5555.6666.777.888899";
    const EXAMPLE_MOVED_FILE_ID_9_STR: &str = "0099.111...2...333.44.5555.6666.777.8888..";
    const EXAMPLE_MOVED_FILE_BLOCKS_STR: &str = "00992111777.44.333....5555.6666.....8888..";
    const EXAMPLE_CHECKSUM: usize = 2858;

    #[test]
    fn test_expanding_example() {
        let file_blocks = get_contiguities_by_expanding_str(EXAMPLE_CONTENTS);
        let expanded_str = contiguities_to_str(&file_blocks);
        assert_eq!(&expanded_str, EXAMPLE_EXPANDED_FILE_BLOCKS_STR);
    }

    #[test]
    fn test_moving_file_id_9() {
        let mut file_blocks = str_to_contiguities(EXAMPLE_EXPANDED_FILE_BLOCKS_STR);
        move_blocks_for_file(&mut file_blocks, 9);
        let file_blocks_str = contiguities_to_str(&file_blocks);
        assert_eq!(&file_blocks_str, EXAMPLE_MOVED_FILE_ID_9_STR);
    }

    #[test]
    fn test_moving_blocks() {
        let mut file_blocks = str_to_contiguities(EXAMPLE_EXPANDED_FILE_BLOCKS_STR);
        move_files(&mut file_blocks);
        let file_blocks_str = contiguities_to_str(&file_blocks);
        assert_eq!(&file_blocks_str, EXAMPLE_MOVED_FILE_BLOCKS_STR);
    }

    #[test]
    fn test_check_sum() {
        let file_blocks = str_to_contiguities(EXAMPLE_MOVED_FILE_BLOCKS_STR);
        let check_sum = get_file_system_check_sum(&file_blocks);
        assert_eq!(check_sum, EXAMPLE_CHECKSUM);
    }

    #[test]
    fn check_example() {
        let check_sum = solve(EXAMPLE_CONTENTS);
        assert_eq!(check_sum, EXAMPLE_CHECKSUM);
    }
}
