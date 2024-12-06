use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day4_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 4 part 2", solve, 1000);
}

fn solve(contents: &str) -> usize {
    let grid: Vec<Vec<u8>> = contents
        .lines()
        .map(|line| line.bytes().collect::<Vec<u8>>())
        .collect();

    let row_count = grid.len();
    let col_count = grid[0].len();

    // Look for A's and search in the 4 diagonal directions when one is found:
    grid.iter()
        .enumerate()
        .skip(1)
        .take(row_count - 2)
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .skip(1)
                .take(col_count - 2)
                .filter(|(c, &byte)| {
                    (byte == b'A')
                        && match grid[r - 1][c - 1] {
                            b'M' => grid[r + 1][c + 1] == b'S',
                            b'S' => grid[r + 1][c + 1] == b'M',
                            _ => false,
                        }
                        && match grid[r - 1][c + 1] {
                            b'M' => grid[r + 1][c - 1] == b'S',
                            b'S' => grid[r + 1][c - 1] == b'M',
                            _ => false,
                        }
                })
                .count()
        })
        .sum::<usize>()
}
