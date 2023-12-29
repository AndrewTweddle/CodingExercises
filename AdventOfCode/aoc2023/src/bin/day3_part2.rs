use std::iter;
use aoc2023::read_and_solve_and_time_more_runs;

fn main() {
    read_and_solve_and_time_more_runs("data/day3_input.txt", "Day 3 part 1", solve, 10_000);
}

struct PartNumber {
    start_col: usize,
    end_col: usize,
    value: u64,
}

struct Gear {
    row: usize,
    col: usize,
}

fn solve(contents: &str) -> u64 {
    let mut gears: Vec<Gear> = Vec::new();

    // Store all numbers by the row they're in, for efficient searching
    let mut numbers_by_row: Vec<Vec<PartNumber>> = Vec::new();

    // Treat the virtual grid as if it has an empty border around it. Add an empty top row...
    numbers_by_row.push(Vec::new());

    contents.lines().enumerate().for_each(|(row_sub_1, ln)| {
        numbers_by_row.push(Vec::new());
        let number_row = numbers_by_row.last_mut().unwrap();
        let row = row_sub_1 + 1;

        let mut is_parsing_number = false;
        let mut number_start_col = 0;
        let mut part_number: u64 = 0;
        
        for (col_sub_1, ch) in ln.chars().chain(iter::once('.')).enumerate() {
            let col = col_sub_1 + 1;
            match ch {
                '0'..='9' => {
                    let digit = ch.to_digit(10).unwrap() as u64;
                    if is_parsing_number {
                        part_number *= 10;
                        part_number += digit;
                    } else {
                        is_parsing_number = true;
                        number_start_col = col;
                        part_number = digit;
                    }
                }
                '*' => {
                    gears.push(Gear { row, col });
                    if is_parsing_number {
                        number_row.push(PartNumber {
                            start_col: number_start_col,
                            end_col: col - 1,
                            value: part_number,
                        });
                        is_parsing_number = false;
                    }
                }
                _ => {
                    if is_parsing_number {
                        number_row.push(PartNumber {
                            start_col: number_start_col,
                            end_col: col - 1,
                            value: part_number,
                        });
                        is_parsing_number = false;
                    }
                }
            };
        }
    });

    // Add an empty bottom row
    numbers_by_row.push(Vec::new());

    let sum_of_gear_numbers: u64 = gears
        .iter()
        .map(|gear| {
            let (adjacent_number_count, gear_ratio) = numbers_by_row
                .iter()
                .skip(gear.row - 1)
                .take(3)
                .flat_map(|row| row.iter())
                .filter(|number| {
                    (gear.col >= number.start_col - 1) && (gear.col <= number.end_col + 1)
                })
                .fold((0, 1), |(cnt, prod), number| (cnt + 1, prod * number.value));

            if adjacent_number_count == 2 {
                gear_ratio
            } else {
                0
            }
        })
        .sum();

    sum_of_gear_numbers
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
        assert_eq!(total, 467835);
    }
}
