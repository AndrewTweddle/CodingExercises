use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day2_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 2 part 2", solve, 1000);
}

fn solve(contents: &str) -> usize {
    contents
        .lines()
        .filter(|report| {
            let levels: Vec<i64> = report
                .split_whitespace()
                .map(|lvl_str| lvl_str.parse::<i64>().unwrap())
                .collect();

            // Treat the cases of dampening the first or second level separately,
            // since these define whether the levels are increasing or decreasing.
            // (This also helps to simplify later code by checking the first triple separately.)
            [(Some(0), 1_usize, 2_usize), (Some(1), 0, 2), (None, 0, 1)]
                .iter()
                .any(|&(initial_damp_index, first, second)| {
                    is_report_safe(&levels, initial_damp_index, first, second)
                })
        })
        .count()
}

fn is_report_safe(
    levels: &[i64],
    mut damp_index: Option<usize>,
    first: usize,
    second: usize,
) -> bool {
    let initial_diff = levels[second] - levels[first];

    // Check the safety of the first pair of numbers chosen from the first triple:
    if initial_diff < -3 || initial_diff == 0 || initial_diff > 3 {
        false
    } else {
        let (lo, hi) = if initial_diff >= 0 { (1, 3) } else { (-3, -1) };

        // Check triples of 3 adjacent levels (using a sliding window).
        // Only check whether the 3rd level needs to be dampened by comparing it to the 2nd level,
        // or in the case that the 2nd level has been dampened compare it to the 1st level.
        // If it's not safe, and damp_index is still None, then dampen (i.e. skip) the 3rd level.
        levels.windows(3).enumerate().all(|(i, triple)| {
            match damp_index {
                Some(di) if di == i + 1 => is_safe(triple[0], triple[2], lo, hi),
                Some(_) => is_safe(triple[1], triple[2], lo, hi),
                None => {
                    if !is_safe(triple[1], triple[2], lo, hi) {
                        // Skip the last level in the triple
                        damp_index = Some(i + 2)
                    }
                    true
                }
            }
        })
    }
}

#[inline(always)]
fn is_safe(level0: i64, level1: i64, lo: i64, hi: i64) -> bool {
    let diff = level1 - level0;
    lo <= diff && diff <= hi
}
