fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000);
}

const MIN_COMBINATORIAL: usize = 1_000_000;
const MIN_N: usize = 23;
const MAX_N: usize = 100;

fn solve() -> usize {
    solve_with_params(MIN_N, MAX_N, MIN_COMBINATORIAL)
}

fn solve_with_params(min_n: usize, max_n: usize, min_combinatorial: usize) -> usize {
    let mut count: usize = 0;
    for n in min_n..=max_n {
        let mut r = 0;
        let mut c = 1;
        while r < n.div_ceil(2) && c < min_combinatorial {
            r += 1;
            c *= n + 1 - r;
            c /= r;
        }
        if c >= min_combinatorial {
            // All values of r from r to n - r will be over the minimum
            count += n + 1 - 2 * r;
        }
    }
    count
}

fn solve_and_print_solution_and_time_more_runs_without_printing<S, T>(solve: S, repetitions: u32)
where
    S: Fn() -> T,
    T: std::fmt::Debug,
{
    use std::time::Instant;

    let mut start_time = Instant::now();
    for i in 0..=repetitions {
        let solution = solve();
        if i == 0 {
            println!("Solution: {solution:?}");
            println!(
                "Solved (including writing to terminal) in {:?}",
                start_time.elapsed()
            );

            // Now restart the timer, so that the timings don't include I/O...
            start_time = Instant::now();
        }
    }

    if repetitions > 0 {
        let avg_duration = start_time.elapsed() / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
    }
}

#[cfg(test)]
mod tests {
    use crate::solve_with_params;

    #[test]
    fn test_over_10_for_n_between_1_and_6() {
        let count = solve_with_params(1, 6, 10);
        // Answers:
        // C(5, 2) = C(5, 3) = 10,
        // C(6, 2) = C(6, 4) = 6
        // C(6, 3) = 20
        assert_eq!(count, 5);
    }
}
