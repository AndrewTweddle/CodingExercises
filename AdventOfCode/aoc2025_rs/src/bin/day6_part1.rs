use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day6_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 6 part 1", solve, 10_000);
}

fn solve(contents: &str) -> u64 {
    let mut lines = contents.lines().collect::<Vec<_>>();

    // Get the operators
    let last_line = lines.pop().unwrap();
    let operators = last_line.split_whitespace().collect::<Vec<_>>();

    // Get the matrix of numbers
    let rows = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    operators
        .iter()
        .enumerate()
        .map(|(i, op)| {
            let num_iter = rows.iter().map(|row| row[i]);
            match *op {
                "*" => num_iter.product::<u64>(),
                "+" => num_iter.sum::<u64>(),
                _ => panic!("Invalid operator: {}", op),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
123 328  51 64 \n\
 45 64  387 23 \n\
  6 98  215 314\n\
*   +   *   +\n";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE), 4277556);
    }
}
