use num::Rational64;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 100)
}

type R = Rational64;

// At each level, we calculate m * sqrt(n) + c, where m and c are rational numbers.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Level {
    m: R,
    c: R,
}

impl Level {
    fn new(m: R, c: R) -> Self {
        Self { m, c }
    }

    fn advance(&mut self, n: R, root: f64) {
        let float_m = (*self.m.numer() as f64) / (*self.m.denom() as f64);
        let float_c = (*self.c.numer() as f64) / (*self.c.denom() as f64);
        let x = float_m * root + float_c;
        let next_a = R::from(x.floor() as i64);

        self.c -= next_a;

        let m = self.m;
        let c = self.c;

        let d = m * m * n - c * c;
        self.m = m / d;
        self.c = -c / d;
    }
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
            let period = calc_period(n as i64, i as i64, j as i64);
            if period % 2 == 1 {
                odd_period_count += 1;
            }
        }
    }
    odd_period_count
}

fn calc_period(n: i64, floor_of_n: i64, remainder: i64) -> usize {
    let root = (n as f64).sqrt();
    let denominator = R::from(remainder);
    let level0 = Level::new(denominator.recip(), R::from(floor_of_n) / denominator);
    let n_rational = R::from(n);

    let mut curr_level = level0;
    let mut period = 1;
    curr_level.advance(n_rational, root);
    while curr_level != level0 {
        period += 1;
        curr_level.advance(n_rational, root);
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
        let period = calc_period(23, 4, 7);
        assert_eq!(period, 4);
    }

    #[test]
    fn test_solve_under_13() {
        assert_eq!(solve_up_to(13), 4);
    }
}
