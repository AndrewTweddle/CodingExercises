fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1_000_000_000)
}

fn solve() -> u32 {
    let mut n = 1;

    // We don't need more than 3 primes above 10, as 2 * 5 = 10, 3 * 7 = 21 > 10,
    // and the product of 4 or more primes above 10 will be > 10^4.
    for p in [2, 3, 5, 7, 11, 13, 17] {
        let product = n * p;
        if product > 1_000_000 {
            return n;
        }
        n = product;
    }
    n
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
