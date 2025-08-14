fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10_000_000)
}

type N = u32;

fn solve() -> N {
    solve_up_to(1_000_000)
}

fn solve_up_to(n: N) -> N {
    let mut n_div = n / (2*3*5*7);
    let mut num_primes_required = 5;  // Take 1 more, to account for integer division rounding down
    while n_div > 0 {
        n_div /= 10;
        num_primes_required += 1;
    }

    let mut primes: Vec<N> = Vec::with_capacity(num_primes_required);
    let mut prime_product = 1;
    for small_prime in [2, 3] {
        if prime_product * small_prime > n {
            return prime_product;
        }
        prime_product *= small_prime;
        primes.push(small_prime);
    }

    // All higher primes are of the form 6m+1 or 6m-1, otherwise they would be divisible by 2 or 3
    for i in (6..).step_by(6) {
        // Case 6m - 1:
        let prime = i - 1;
        if prime_product * prime > n {
            return prime_product;
        }

        if primes.iter().all(|&p| prime % p != 0) {
            // It's prime
            prime_product *= prime;
            primes.push(prime);
        }

        // Case 6m+1:
        let prime = i + 1;
        if prime_product * prime > n {
            return prime_product;
        }

        if primes.iter().all(|&p| prime % p != 0) {
            // It's prime
            prime_product *= prime;
            primes.push(prime);
        }
    }

    unreachable!();
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
    fn test_up_to_10() {
        assert_eq!(solve_up_to(10), 6);
    }
}
