fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10)
}

type N = u64;
const MAX_N: N = 1_000_000;

fn solve() -> N {
    solve_up_to(MAX_N)
}

fn solve_up_to(max_n: N) -> N {
    // For each denominator n, we want to count the number of smaller numerators
    // that are coprime to n. But this is just the totient function.
    // So add up phi(n) for n between 2 and max_n, inclusive.
    let sqrt_max_n = max_n.isqrt();
    let mut totient_sum: N = 0;
    let mut n = 1;

    // Start from n = 2, since no proper fraction has 1 as a denominator.
    for sqrt_n in 1..=sqrt_max_n {
        for _ in 0..(2 * sqrt_n + 1) {
            n += 1;
            if n > max_n {
                break;
            }
            let phi = totient(n, sqrt_n);
            totient_sum += phi;
        }
    }
    totient_sum
}

fn totient(mut n: N, sqrt_n: N) -> N {
    // Factorize n into prime factors
    let mut phi = n;

    try_reduce_by_prime_factor(&mut n, &mut phi, 2);
    try_reduce_by_prime_factor(&mut n, &mut phi, 3);
    try_reduce_by_prime_factor(&mut n, &mut phi, 5);

    for mult_of_6 in (6..=sqrt_n).step_by(6) {
        if try_reduce_by_prime_factor(&mut n, &mut phi, mult_of_6 + 1) && n == 1 {
            break;
        }

        if try_reduce_by_prime_factor(&mut n, &mut phi, mult_of_6 + 5) && n == 1 {
            break;
        }
    }

    if n > 1 {
        let p = n;
        try_reduce_by_prime_factor(&mut n, &mut phi, p);
    }
    phi
}

fn try_reduce_by_prime_factor(n: &mut N, phi: &mut N, p: N) -> bool {
    if *n % p != 0 {
        return false;
    }
    *phi /= p;
    *phi *= p - 1;
    while *n % p == 0 {
        *n /= p;
    }
    true
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
    fn test_solve_up_to_8() {
        assert_eq!(solve_up_to(8), 21);
    }

    #[test]
    fn test_totient_of_25() {
        assert_eq!(totient(25, 5), 20);
    }

    #[test]
    fn test_first_69_totients() {
        // From https://oeis.org/A000010
        let expected_totients: [N; 69] = [
            1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8, 12, 10, 22, 8, 20, 12,
            18, 12, 28, 8, 30, 16, 20, 16, 24, 12, 36, 18, 24, 16, 40, 12, 42, 20, 24, 22, 46, 16,
            42, 20, 32, 24, 52, 18, 40, 24, 36, 28, 58, 16, 60, 30, 36, 32, 48, 20, 66, 32, 44,
        ];
        for i in 0..69 {
            let n: N = 1 + i as N;
            let sqrt_n = n.isqrt();
            assert_eq!(totient(n, sqrt_n), expected_totients[i]);
        }
    }
}
