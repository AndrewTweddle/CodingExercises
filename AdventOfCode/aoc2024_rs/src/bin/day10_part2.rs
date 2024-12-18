use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::iter;

const INPUT_FILE_PATH: &str = "data/day10_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 10 part 2", solve, 1000);
}

// Set up the 4 movement directions
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn solve(contents: &str) -> usize {
    // Create a grid of bytes, padded with 1 extra lines on each of the 4 sides,
    // to avoid needing to do bounds checking:
    let mut grid: Vec<Vec<u8>> = iter::once(vec![])
        .chain(
            contents
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| {
                    [b'.']
                        .iter()
                        .cloned()
                        .chain(line.bytes())
                        .chain([b'.'])
                        .collect::<Vec<u8>>()
                }),
        )
        .chain([vec![]])
        .collect();

    // Fill in the first and last padded rows to match the length of the other rows
    let padded_col_count = grid[1].len();

    for i in [0, padded_col_count - 1] {
        grid[i].extend(iter::repeat(b'.').take(padded_col_count));
    }

    let mut reachable_peaks: Vec<Vec<usize>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|&byte| if byte == b'9' { 1 } else { 0 })
                .collect::<Vec<usize>>()
        })
        .collect();

    // Count all paths upwards to a 9 from each trailhead
    let mut trailhead_count: usize = 0;

    for target_height in (b'0'..=b'8').rev() {
        for (row, cols) in grid.iter().enumerate() {
            for (col, &height) in cols.iter().enumerate() {
                if height != target_height {
                    continue;
                }
                let paths_up = DIRS
                    .iter()
                    .map(|(row_offset, col_offset)| {
                        let r = (row as i32 + row_offset) as usize;
                        let c = (col as i32 + col_offset) as usize;
                        let adj = grid[r][c];
                        if adj == target_height + 1 {
                            reachable_peaks[r][c]
                        } else {
                            0
                        }
                    })
                    .sum();
                if target_height == b'0' {
                    trailhead_count += paths_up
                } else {
                    reachable_peaks[row][col] = paths_up;
                }
            }
        }
    }

    trailhead_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";

    const LARGE_EXAMPLE: &str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_small_example() {
        let trailhead_count = solve(SMALL_EXAMPLE);
        assert_eq!(trailhead_count, 3);
    }

    #[test]
    fn test_large_example() {
        let trailhead_count = solve(LARGE_EXAMPLE);
        assert_eq!(trailhead_count, 81);
    }
}
