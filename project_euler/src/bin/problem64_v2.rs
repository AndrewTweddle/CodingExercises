fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000)
}

fn solve() -> usize {
    solve_up_to(10_000)
}

fn solve_up_to(max_n: usize) -> usize {
    let mut odd_period_count: usize = 0;
    'outer: for i in 1.. {
        for j in 1..(2 * i) {
            let n = i * i + j;
            if n > max_n {
                break 'outer;
            }
            let period = calc_period(n as i64, i as i64);
            if period % 2 == 1 {
                odd_period_count += 1;
            }
        }
    }
    odd_period_count
}

/// Uses the formula at https://en.wikipedia.org/wiki/Periodic_continued_fraction#Canonical_form_and_repetend
fn calc_period(s: i64, a0: i64) -> usize {
    let mut m = 0;
    let mut d = 1;
    let mut a = a0;
    let mut period = 0; // period

    while a != 2 * a0 {
        m = d * a - m;
        d = (s - m * m) / d;
        a = (a0 + m) / d;
        period += 1;
    }
    period
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
        let end_time = Instant::now();
        println!("Ending iterations at {end_time:?}");

        let avg_duration = total_elapsed / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
        println!("Total elapsed time for {repetitions} runs: {total_elapsed:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_period_of_root_23() {
        let period = calc_period(23, 4);
        assert_eq!(period, 4);
    }

    #[test]
    fn test_solve_under_13() {
        assert_eq!(solve_up_to(13), 4);
    }
}
