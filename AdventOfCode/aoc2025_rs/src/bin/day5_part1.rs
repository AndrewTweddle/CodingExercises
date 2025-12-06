use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day5_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 5 part 1", solve, 1000);
}

fn solve(contents: &str) -> usize {
    let mut line_iter = contents.lines();
    let fresh_id_ranges = (&mut line_iter)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (min_str, max_str) = line.split_once('-').unwrap();
            (
                min_str.parse::<u64>().unwrap(),
                max_str.parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    line_iter
        .filter(|line| {
            let id = line.parse::<u64>().unwrap();
            fresh_id_ranges
                .iter()
                .any(|(min, max)| id >= *min && id <= *max)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        3-5\n\
        10-14\n\
        16-20\n\
        12-18\n\
        \n\
        1\n\
        5\n\
        8\n\
        11\n\
        17";

    #[test]
    fn test_example() {
        assert_eq!(solve(EXAMPLE), 3);
    }
}
