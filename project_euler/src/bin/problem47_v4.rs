fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10);
}

fn solve() -> u64 {
    let mut primes: Vec<u64> = Vec::with_capacity(1_000_000);
    let mut distinct_factors: Vec<u64> = Vec::with_capacity(1_000_000);
    distinct_factors.extend([1, 1]);

    let mut count_of_ints_with_4_distinct_prime_factors = 0;

    for i in 2.. {
        let mut distinct_prime_factor_count = 1; // initialize to a common outcome
        if let Some(p) = primes.iter().find(|&p| i % p == 0) {
            let mut q = i / p;
            while q % p == 0 {
                q /= p;
            }
            if q != 1 {
                // Look up the distinct factors of the smaller number that remains
                // after reducing by the first prime factor found
                distinct_prime_factor_count = distinct_factors[q as usize] + 1;

                // Check for 4 prime factors. If found, either return or continue with the next i.
                if distinct_prime_factor_count == 4 {
                    count_of_ints_with_4_distinct_prime_factors += 1;
                    if count_of_ints_with_4_distinct_prime_factors == 4 {
                        return i - 3;
                    } else {
                        distinct_factors.push(4);
                        continue;
                    }
                }
            }
        } else {
            // No smaller prime factor found, so this number must be prime
            primes.push(i);
        }

        // Note that we only get here if there weren't exactly 4 distinct prime factors
        count_of_ints_with_4_distinct_prime_factors = 0;
        distinct_factors.push(distinct_prime_factor_count);
    }
    // We should never get here...
    0
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
