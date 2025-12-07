use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day7_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 7 part 2", solve, 10_000);
}

fn solve(contents: &str) -> usize {
    let mut line_iter = contents.lines();
    let top_row = line_iter.next().expect("No input");
    let mut curr_row: Vec<usize> = top_row
        .chars()
        .map(|c| if c == 'S' { 1 } else { 0 })
        .collect();

    for line in line_iter {
        let mut this_beam = 0;
        let mut next_beam = 0;

        for (j, c) in line.chars().enumerate() {
            let path_count = curr_row[j];
            if path_count > 0 && c == '^' {
                curr_row[j] = 0;
                if j > 0 {
                    curr_row[j - 1] += path_count;
                }
                next_beam = path_count;
            }
            curr_row[j] += this_beam;
            this_beam = next_beam;
            next_beam = 0;
        }
    }
    curr_row.iter().sum()
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
