use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day7_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 7 part 1", solve, 10_000);
}

fn solve(contents: &str) -> usize {
    let mut line_iter = contents.lines();
    let top_row = line_iter.next().expect("No input");
    let col_count = top_row.len();

    let mut odd_row: Vec<bool> = vec![false; col_count];
    let mut even_row: Vec<bool> = vec![false; col_count];

    let initial_pos = top_row.find('S').unwrap();
    odd_row[initial_pos] = true;

    let mut split_count: usize = 0;

    for (i, line) in line_iter.enumerate() {
        let (prev_row, curr_row) = if i % 2 == 0 {
            (&odd_row, &mut even_row)
        } else {
            (&even_row, &mut odd_row)
        };

        let mut this_beam = false;
        let mut next_beam = false;

        for (j, c) in line.chars().enumerate() {
            if prev_row[j] {
                if c == '^' {
                    split_count += 1;
                    curr_row[j] = false;
                    if j > 0 {
                        curr_row[j - 1] = true;
                    }
                    next_beam = true;
                } else {
                    curr_row[j] = true;
                }
            } else {
                curr_row[j] = false;
            }
            if this_beam {
                curr_row[j] = true;
            }
            this_beam = next_beam;
            next_beam = false;
        }
    }
    split_count
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
        assert_eq!(solve(EXAMPLE), 21);
    }
}
