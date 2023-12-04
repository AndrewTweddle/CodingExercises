use aoc2023::read_and_solve_and_time_more_runs;

fn main() {
    read_and_solve_and_time_more_runs("data/day3_input.txt", "Day 3 part 1", solve, 10_000);
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Entry {
    Empty,
    Symbol,
    Digit(u32),
}

fn solve(contents: &str) -> u32 {
    // Create a grid, padded on all sides with empty tokens
    let width = contents
        .lines()
        .next()
        .expect("The first line could not be read")
        .len()
        + 2;
    let mut grid: Vec<Vec<Entry>> = Vec::new();
    // Add empty top row
    grid.push(vec![Entry::Empty; width]);

    contents.lines().for_each(|ln| {
        grid.push(vec![Entry::Empty; width]);
        let new_row = grid.last_mut().unwrap();
        for (i, ch) in ln.chars().enumerate() {
            let token = match ch {
                '.' => Entry::Empty,
                dgt if ch.is_ascii_digit() => Entry::Digit(dgt.to_digit(10).unwrap()),
                _ => Entry::Symbol,
            };
            new_row[i + 1] = token;
        }
    });
    // Add an empty bottom row
    grid.push(vec![Entry::Empty; width]);

    let mut sum_of_part_numbers = 0;

    for (row, row_entries) in grid.iter().enumerate() {
        let mut is_parsing_number = false;
        let mut is_part_number = false;
        let mut part_number = 0;

        for (col, entry) in row_entries.iter().enumerate() {
            match entry {
                Entry::Digit(ref digit) => {
                    if !is_parsing_number {
                        // This is the start of a number
                        is_parsing_number = true;

                        // Check for symbols to the left and adjacent to the first digit
                        if grid[row - 1][col - 1] == Entry::Symbol
                            || grid[row][col - 1] == Entry::Symbol
                            || grid[row + 1][col - 1] == Entry::Symbol
                        {
                            is_part_number = true;
                        }
                    }

                    // Check for symbols above and below the digit
                    if !is_part_number
                        && (grid[row - 1][col] == Entry::Symbol
                            || grid[row + 1][col] == Entry::Symbol)
                    {
                        is_part_number = true;
                    }

                    part_number *= 10;
                    part_number += digit;
                }
                _ => {
                    if is_parsing_number {
                        // check if there is a symbol to the right and adjacent to the last digit
                        if !is_part_number
                            && (grid[row - 1][col] == Entry::Symbol
                                || grid[row][col] == Entry::Symbol
                                || grid[row + 1][col] == Entry::Symbol)
                        {
                            is_part_number = true;
                        }
                        if is_part_number {
                            sum_of_part_numbers += part_number;
                        }
                        is_parsing_number = false;
                        is_part_number = false;
                        part_number = 0;
                    }
                }
            }
        }
    }

    sum_of_part_numbers
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_part1_example() {
        let contents = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let total = solve(contents);
        assert_eq!(total, 4361);
    }
}
