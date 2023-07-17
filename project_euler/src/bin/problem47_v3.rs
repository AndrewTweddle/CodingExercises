fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10);
}

fn solve() -> u64 {
    let mut count_of_ints_with_4_distinct_prime_factors = 0;

    for i in (2 * 3 * 5 * 7)..u64::MAX {
        if has_exactly_4_distinct_prime_factors(i) {
            count_of_ints_with_4_distinct_prime_factors += 1;
            if count_of_ints_with_4_distinct_prime_factors == 4 {
                return i - 3;
            }
        } else {
            count_of_ints_with_4_distinct_prime_factors = 0;
        }
    }
    // We should never get here...
    0
}

fn has_exactly_4_distinct_prime_factors(mut n: u64) -> bool {
    let mut distinct_prime_factor_count = 0;

    // reduce by 2 first, since it's an easily optimized case:
    if n % 2 == 0 {
        distinct_prime_factor_count = 1;
        n /= 2;
        while n % 2 == 0 {
            n /= 2;
        }
    }

    if reduce_by_factor(&mut n, 3) {
        distinct_prime_factor_count += 1;
    }

    // all primes above 3 are of the form 6n +/- 1, so only check such numbers...
    for i in 1_u64.. {
        if reduce_by_factor(&mut n, 6 * i - 1) {
            distinct_prime_factor_count += 1;
        }
        if n != 1 && reduce_by_factor(&mut n, 6 * i + 1) {
            distinct_prime_factor_count += 1;
        }
        if n == 1 {
            return distinct_prime_factor_count == 4;
        }
        if distinct_prime_factor_count == 4 {
            // Since n != 1, there will be more than 4 distinct prime factors
            return false;
        }
    }
    false
}

#[inline]
fn reduce_by_factor(n: &mut u64, p: u64) -> bool {
    if *n % p == 0 {
        *n /= p;
        while *n % p == 0 {
            *n /= p;
        }
        true
    } else {
        false
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
        let avg_duration = start_time.elapsed() / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
    }
}
