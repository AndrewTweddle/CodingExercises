fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10_000)
}

const N: usize = 100;

fn solve() -> usize {
    solve_for_n_up_to(N)
}

fn solve_for_n_up_to(n: usize) -> usize {
    // Build up a memoization table of all (target, max_value) calculations of the number of ways
    // of reaching the target by choosing the highest term <= max_value, in the sum.
    // Note that this allows sums of just 1 term as well.
    let mut memo: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    memo[0].push(1);
    for target in 1..=n {
        let (prev_rows, next_rows) = memo.split_at_mut(target);
        let curr_row = next_rows.first_mut().unwrap();

        curr_row.push(0);
        curr_row.push(1);
        let mut num_ways = 1;

        for max_value in 2..=target {
            // If we don't use max_value, then use the same target but reduce max_value by 1.
            // But this is just the value currently in num_ways.
            // So update num_ways, by adding to it the number of ways using max_value at least once.
            num_ways += (max_value..=target)
                .step_by(max_value)
                .map(|reduction| {
                    let new_target = target - reduction;
                    let new_max_value = new_target.min(max_value - 1);
                    prev_rows[new_target][new_max_value]
                })
                .sum::<usize>();

            curr_row.push(num_ways);
        }
    }

    memo[n][n - 1]
}

use std::hint::black_box;

fn solve_and_print_solution_and_time_more_runs_without_printing<S, T>(solve: S, repetitions: u32)
where
    S: Fn() -> T,
    T: std::fmt::Debug,
{
    use std::time::Instant;

    let mut start_time = Instant::now();
    for i in 0..=repetitions {
        let solution = black_box(solve());
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
        let total_elapsed = start_time.elapsed();
        let avg_duration = total_elapsed / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
        println!("Total elapsed time for {repetitions} runs: {total_elapsed:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_for_n_up_to_5() {
        assert_eq!(solve_for_n_up_to(5), 6);
    }
}
