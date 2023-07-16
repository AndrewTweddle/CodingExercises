fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10);
}

fn solve() -> u64 {
    let mut primes: Vec<u64> = Vec::with_capacity(1000);
    primes.extend([2, 3, 5, 7]);
    let mut count_of_4_prime_divisor_ints = 0;

    for i in 8.. {
        let mut distinct_prime_factor_count = 0;
        let mut q = i;
        for &p in &primes {
            if q % p == 0 {
                distinct_prime_factor_count += 1;
                q /= p;
                while q % p == 0 {
                    q /= p;
                }
                if q == 1 {
                    break;
                }

                // No point in continuing if more than 4 distinct prime factors encountered
                if distinct_prime_factor_count > 4 {
                    break;
                }
            }
        }
        if distinct_prime_factor_count == 4 {
            count_of_4_prime_divisor_ints += 1;
            if count_of_4_prime_divisor_ints == 4 {
                return i - 3;
            }
        } else {
            if distinct_prime_factor_count == 0 {
                primes.push(i);
            }

            // The sequence is broken, so start over...
            count_of_4_prime_divisor_ints = 0;
        }
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
