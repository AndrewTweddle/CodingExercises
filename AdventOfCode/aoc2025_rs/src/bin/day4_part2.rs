use aoc2025_rs::load_and_solve_and_benchmark;
use std::iter;

const INPUT_FILE_PATH: &str = "data/day4_input.txt";

type Grid = Vec<Vec<bool>>;

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 4 part 1", solve, 100);
}

fn solve(contents: &str) -> usize {
    // Build up a grid of booleans, where a value is true if there is a roll of paper in that cell.
    // Add a border of false values on each of the 4 sides to make comparisons easier.
    let mut grid: Grid = iter::once(vec![])
        .chain(contents.lines().map(|line| {
            iter::once(false)
                .chain(line.chars().map(|c| c == '@'))
                .chain(iter::once(false))
                .collect::<Vec<bool>>()
        }))
        .collect();

    let col_count = grid[1].len();
    grid[0].extend(iter::repeat_n(false, col_count));
    grid.push(vec![false; col_count]);
    let row_count = grid.len();

    let mut removed_count: usize = 0;
    let mut even_to_remove = vec![false; col_count];
    let mut odd_to_remove = vec![false; col_count];

    loop {
        let mut removed_this_round = 0;
        for row in 1..row_count {
            let (next_to_remove, prev_to_remove) = if row % 2 == 0 {
                (&mut even_to_remove, &odd_to_remove)
            } else {
                (&mut odd_to_remove, &even_to_remove)
            };

            // Calculate (and count) which cells will be removed from this row.
            // In the process record the new state of the row after these cells are removed.
            // Note that the update will only take place after the next row is processed.
            for col in 1..col_count - 1 {
                let mut has_roll = grid[row][col];
                if has_roll && has_4_or_fewer_neighbours(&grid, row, col) {
                    removed_this_round += 1;
                    has_roll = false;
                }
                next_to_remove[col] = has_roll;
            }

            // Update the previous row, now that it no longer affects the calculations
            if row > 1 {
                grid[row - 1].copy_from_slice(prev_to_remove);
            }
        }
        if removed_this_round == 0 {
            break;
        }
        removed_count += removed_this_round;
    }
    removed_count
}

fn has_4_or_fewer_neighbours(grid: &[Vec<bool>], row: usize, col: usize) -> bool {
    grid[row - 1..=row + 1]
        .iter()
        .map(|row| row[col - 1..=col + 1].iter().filter(|cell| **cell).count())
        .sum::<usize>()
        <= 4
    // The cell is counted as well as its neighbours, so bump the required count by 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        ..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE), 43);
    }
}
