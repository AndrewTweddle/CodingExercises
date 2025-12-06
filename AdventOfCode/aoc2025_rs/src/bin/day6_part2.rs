use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day6_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 6 part 2", solve, 10_000);
}

fn solve(contents: &str) -> u64 {
    // Turn the lines into a matrix of characters
    let mut lines = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    // Extract the operators line
    let mut operator_line = lines.pop().unwrap();

    // Add a special operator to signal that we must do clean up before the end
    operator_line.push('!');

    let (total, _, _) =
        operator_line
            .iter()
            .enumerate()
            .fold((0_u64, 0_u64, '+'), |acc, (i, &operator)| {
                let (total, partial_total, prev_operator) = acc;
                if operator == '!' {
                    // The termination indicator gives a chance to process the last partial number
                    (total + partial_total, 0, prev_operator)
                } else {
                    let next_number = lines.iter().fold(0_u64, |n, line| {
                        let ch = line[i];
                        if ch.is_ascii_digit() {
                            let digit = ch.to_digit(10).unwrap() as u64;
                            10 * n + digit
                        } else {
                            n
                        }
                    });
                    if next_number == 0 {
                        // This will be a column of all spaces, so we can ignore it
                        (total, partial_total, prev_operator)
                    } else {
                        match (operator, prev_operator) {
                            (' ', '+') => (total, partial_total + next_number, prev_operator),
                            (' ', '*') => (total, partial_total * next_number, prev_operator),
                            _ => (total + partial_total, next_number, operator),
                        }
                    }
                }
            });
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE), 3263827);
    }
}
