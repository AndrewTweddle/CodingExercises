use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day7_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 7 part 2", solve, 10_000);
}

fn solve(contents: &str) -> usize {
    let mut line_iter = contents.lines();
    let top_row = line_iter.next().expect("No input");
    let col_count = top_row.len();

    let mut even_row: Vec<usize> = vec![0; col_count];
    let mut odd_row: Vec<usize> = vec![0; col_count];

    // track the number of rows, excluding the top row
    let mut row_index = 0;

    let initial_pos = top_row.find('S').unwrap();
    even_row[initial_pos] = 1;

    for line in line_iter {
        row_index += 1;

        let (prev_row, curr_row) = if row_index % 2 == 0 {
            (&odd_row, &mut even_row)
        } else {
            (&even_row, &mut odd_row)
        };

        let mut this_beam = 0;
        let mut next_beam = 0;

        for (j, c) in line.chars().enumerate() {
            let path_count = prev_row[j];
            if path_count > 0 {
                if c == '^' {
                    curr_row[j] = 0;
                    if j > 0 {
                        curr_row[j - 1] += path_count;
                    }
                    next_beam = path_count;
                } else {
                    curr_row[j] = path_count;
                }
            } else {
                curr_row[j] = 0;
            }
            curr_row[j] += this_beam;
            this_beam = next_beam;
            next_beam = 0;
        }
    }
    if row_index % 2 == 0 {
        even_row.iter().sum()
    } else {
        odd_row.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        .......S.......\n\
        ...............\n\
        .......^.......\n\
        ...............\n\
        ......^.^......\n\
        ...............\n\
        .....^.^.^.....\n\
        ...............\n\
        ....^.^...^....\n\
        ...............\n\
        ...^.^...^.^...\n\
        ...............\n\
        ..^...^.....^..\n\
        ...............\n\
        .^.^.^.^.^...^.\n\
        ...............";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE), 40);
    }
}
