fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000);
}

const MAX_ITERATIONS: usize = 49;
const MAX_TO_CHECK: u128 = 9999;

fn solve() -> usize {
    let lychrel_count: usize = (1_u128..=MAX_TO_CHECK).filter(|&n| is_lychrel(n)).count();
    lychrel_count
}

#[inline(always)]
fn is_lychrel(n: u128) -> bool {
    count_iterations_until_not_lychrel(n).is_none()
}

#[inline(always)]
fn count_iterations_until_not_lychrel(mut n: u128) -> Option<usize> {
    // At least 1 iteration is required
    n += reverse_digits(n);

    for i in 1..=MAX_ITERATIONS {
        let r = reverse_digits(n);
        if n == r {
            return Some(i);
        }
        n += r;
    }
    None
}

#[inline(always)]
fn reverse_digits(mut n: u128) -> u128 {
    let mut r = 0;
    while n != 0 {
        r *= 10;
        r += n % 10;
        n /= 10;
    }
    r
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
    use crate::{count_iterations_until_not_lychrel, is_lychrel, reverse_digits};

    #[test]
    fn test_reverse_349() {
        assert_eq!(reverse_digits(349), 943);
    }

    #[test]
    fn test_349_iteration_count() {
        let count = count_iterations_until_not_lychrel(349);
        assert_eq!(count, Some(3));
    }

    #[test]
    fn test_349_is_not_lychrel() {
        assert!(!is_lychrel(349));
    }

    #[test]
    fn test_196_is_lychrel() {
        assert!(is_lychrel(196));
    }

    #[test]
    fn test_4994_is_lychrel() {
        assert!(is_lychrel(4994));
    }
}
