use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::collections::HashSet;
use std::iter;

const INPUT_FILE_PATH: &str = "data/day10_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 10 part 1", solve, 100);
}

// Set up the 4 movement directions
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

type Pos = (usize, usize);

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

    let mut reachable_peaks: Vec<Vec<HashSet<Pos>>> = grid
        .iter()
        .enumerate()
        .map(|(row, cols)| {
            cols.iter()
                .enumerate()
                .map(|(col, &byte)| {
                    let mut set: HashSet<Pos> = HashSet::new();
                    if byte == b'9' {
                        set.insert((row, col));
                    };
                    set
                })
                .collect::<Vec<HashSet<Pos>>>()
        })
        .collect();

    // Count distinct pairs of trailheads and 9's that are connected by a path
    let mut trailhead_count: usize = 0;

    for target_height in (b'0'..=b'8').rev() {
        for (row, cols) in grid.iter().enumerate() {
            for (col, &height) in cols.iter().enumerate() {
                if height != target_height {
                    continue;
                }
                let mut newly_reachable: HashSet<Pos> = HashSet::new();
                for dir in DIRS {
                    let r = (row as i32 + dir.1) as usize;
                    let c = (col as i32 + dir.0) as usize;
                    let adj = grid[r][c];
                    if adj == target_height + 1 {
                        newly_reachable.extend(reachable_peaks[r][c].iter());
                    }
                }
                if target_height == b'0' {
                    trailhead_count += newly_reachable.len();
                } else {
                    reachable_peaks[row][col] = newly_reachable;
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
        assert_eq!(trailhead_count, 36);
    }
}