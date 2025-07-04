fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1_000_000)
}

// 10^n has n + 1 digits, so suppose d is a digit between 1 and 9.
// d^n has n digits if floor(log10(d^n)) = n - 1, i.e. if n - 1 <= n * log10(d) < n.
// Now n * log10(d) < n, since d < 10.
// So we want to find the largest n for which n - 1 <= n * log10(d)
// i.e. 1 - 1/n <= log10(d), i.e. -1/n <= log10(d) - 1, so 1/n >= 1 - log10(d),
// so n <= 1/(1 - log10(d)), i.e. n = floor(1/(1-log10(d))

fn solve() -> usize {
    (1..10)
        .map(|d| (1.0 / (1.0 - (d as f64).log10())).floor() as usize)
        .sum()
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
            println!("Starting repetitions at {:?}", Instant::now());
            start_time = Instant::now();
        }
    }

    if repetitions > 0 {
        let total_elapsed = start_time.elapsed();
        let end_time = Instant::now();
        println!("Ending iterations at {end_time:?}");

        let avg_duration = total_elapsed / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
        println!("Total elapsed time for {repetitions} runs: {total_elapsed:?}");
    }
}
