use aoc2025_rs::load_and_solve_and_benchmark;

const INPUT_FILE_PATH: &str = "data/day5_input.txt";

fn main() {
    load_and_solve_and_benchmark(INPUT_FILE_PATH, "Day 5 part 2", solve, 100_000);
}

fn solve(contents: &str) -> usize {
    let mut line_iter = contents.lines();
    let mut fresh_id_ranges = (&mut line_iter)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (min_str, max_str) = line.split_once('-').unwrap();
            (
                min_str.parse::<usize>().unwrap(),
                max_str.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    fresh_id_ranges.sort();

    let mut overlap_index = 0;
    while overlap_index < fresh_id_ranges.len() - 1 {
        let (lo1, hi1) = fresh_id_ranges[overlap_index];
        let (lo2, hi2) = fresh_id_ranges[overlap_index + 1];
        if hi1 < lo2 {
            overlap_index += 1;
        } else {
            fresh_id_ranges[overlap_index] = (lo1.min(lo2), hi1.max(hi2));
            fresh_id_ranges.remove(overlap_index + 1);
        }
    }

    fresh_id_ranges.iter().map(|(lo, hi)| hi - lo + 1).sum()
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
        assert_eq!(solve(EXAMPLE), 14);
    }
}
