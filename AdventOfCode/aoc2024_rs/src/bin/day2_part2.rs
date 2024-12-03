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

            // Treat the cases of skipping the first or second level separately,
            // since these define whether the levels are increasing or decreasing.
            // Additionally, all future checks will take triples of 3 adjacent levels
            // and will only check whether the 3rd level needs to be dampened.
            // (The 2nd and 3rd numbers will be checked, unless the second was dampened in a
            // previous round, in which case the 1st and 3rd levels in the triple will be checked.)
            // So these cases also ensure that each pair from the first triple will get checked.
            [(Some(0), 1_usize, 2_usize), (Some(1), 0, 2), (None, 0, 1)]
                .iter()
                .any(|&(initial_skip_index, first, second)| {
                    let mut skip_index = initial_skip_index;
                    let initial_diff = levels[second] - levels[first];

                    // Check the safety of the first pair of numbers chosen from the first triple:
                    if initial_diff < -3 || initial_diff == 0 || initial_diff > 3 {
                        false
                    } else {
                        let (lo, hi) = if initial_diff >= 0 { (1, 3) } else { (-3, -1) };
                        levels.windows(3).enumerate().all(|(i, triple)| {
                            match skip_index {
                                Some(si) if si == i + 1 => is_safe(triple[0], triple[2], lo, hi),
                                Some(_) => is_safe(triple[1], triple[2], lo, hi),
                                None => {
                                    if !is_safe(triple[1], triple[2], lo, hi) {
                                        // Skip the last level in the triple
                                        skip_index = Some(i + 2)
                                    }
                                    true
                                }
                            }
                        })
                    }
                })
        })
        .count()
}

#[inline(always)]
fn is_safe(level0: i64, level1: i64, lo: i64, hi: i64) -> bool {
    let diff = level1 - level0;
    lo <= diff && diff <= hi
}
