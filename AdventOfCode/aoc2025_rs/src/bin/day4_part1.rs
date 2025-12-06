use aoc2025_rs::load_and_solve_and_benchmark;
use std::iter;

const INPUT_FILE_PATH: &str = "data/day4_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 4 part 1", solve, 10_000);
}

fn solve(contents: &str) -> usize {
    // Build up a grid of booleans, where a value is true if there is a roll of paper in that cell.
    // Add a border of false values on each of the 4 sides to make comparisons easier.
    let mut grid: Vec<Vec<bool>> = iter::once(vec![])
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

    (1..grid.len() - 1)
        .map(|row| {
            (1..col_count - 1)
                .filter(|&col| grid[row][col] && has_4_or_fewer_neighbours(&grid, row, col))
                .count()
        })
        .sum()
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
        assert_eq!(solve(EXAMPLE), 13);
    }
}
