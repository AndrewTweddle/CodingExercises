use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day4_input.txt";
const MAS_BYTES: [u8; 3] = [b'M', b'A', b'S'];

// Set up the 8 search directions
const SEARCH_DIRS: [(i64, i64); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 4 part 1", solve, 1000);
}

fn solve(contents: &str) -> usize {
    // Create a grid of bytes, padded with 3 extra lines on each of the 4 sides,
    // to avoid needing to do bounds checking:
    let mut grid: Vec<Vec<u8>> = [const { Vec::<u8>::new() }; 3]
        .iter()
        .cloned()
        .chain(contents.lines().map(|line| {
            [b' ', b' ', b' ']
                .iter()
                .cloned()
                .chain(line.bytes())
                .chain([b' ', b' ', b' '])
                .collect::<Vec<u8>>()
        }))
        .chain([vec![], vec![], vec![]])
        .collect();

    let padded_row_count = grid.len();
    let original_row_count = padded_row_count - 6;

    // Fill in the first 3 and last 3 padded rows to match the length of the other rows
    let padded_col_count = grid[3].len();
    let original_col_count = padded_col_count - 6;

    for i in [
        0,
        1,
        2,
        padded_col_count - 3,
        padded_col_count - 2,
        padded_col_count - 1,
    ] {
        grid[i].extend(std::iter::repeat(b' ').take(padded_col_count));
    }

    // Look for X's and search in all 8 directions when one is found:
    grid.iter()
        .enumerate()
        .skip(3)
        .take(original_row_count)
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .skip(3)
                .take(original_col_count)
                .map(|(c, &byte)| {
                    if byte == b'X' {
                        SEARCH_DIRS
                            .iter()
                            .filter(|dir| {
                                (1..4).zip(&MAS_BYTES).all(|(i, &expected_byte)| {
                                    let row = (r as i64 + i * dir.0) as usize;
                                    let col = (c as i64 + i * dir.1) as usize;
                                    grid[row][col] == expected_byte
                                })
                            })
                            .count()
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}
