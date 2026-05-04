fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10)
}

const N: usize = 100;

fn solve() -> usize {
    solve_for_n_up_to(N)
}

fn solve_for_n_up_to(n: usize) -> usize {
    num_ways(n, n - 1)
}

fn num_ways(target: usize, max_value: usize) -> usize {
    if max_value == 1 {
        1
    } else {
        (0..=target)
            .step_by(max_value)
            .map(|reduction| num_ways(target - reduction, max_value - 1))
            .sum::<usize>()
    }
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
