use num::integer::Roots;

const TARGET_PRIME_COUNT: usize = 8;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 100)
}

fn solve() -> u64 {
    for num_digits in 1.. {
        // The first digit cannot be a zero
        if let Some(solution) = (1..10).fold(None, |acc, digit| {
            acc.or_else(|| solve_next_digit(num_digits - 1, digit, None, 0, 0))
        }) {
            return solution;
        }
    }
    // Unreachable...
    panic!("No solution found");
}

/// index is the index in the solution of the next digit.
/// d is the next digit.
/// w is the lowest value for the wildcard digit (that must be prime).
///   It can be None, i.e. not yet decided, if the first wildcard position is yet to come.
/// n is the number being built up, excluding the value of the wildcard digits.
/// m is the multiplier of the wildcard digit
fn solve_next_digit(index: usize, d: u8, w: Option<u8>, n: u64, m: u64) -> Option<u64> {
    if d <= (10 - TARGET_PRIME_COUNT) as u8 {
        // Consider two cases, where this digit is part of the mask and where it is not.
        // Solve for both and take the solution with the lower prime.
        if let Some(wc_soln) = solve_wildcard_digit(index, d, w, n, m) {
            if let Some(non_wc_soln) = solve_non_wc_digit(index, d, w, n, m) {
                Some(wc_soln.min(non_wc_soln))
            } else {
                Some(wc_soln)
            }
        } else {
            solve_non_wc_digit(index, d, w, n, m)
        }
    } else {
        solve_non_wc_digit(index, d, w, n, m)
    }
}

fn solve_non_wc_digit(index: usize, d: u8, w: Option<u8>, mut n: u64, mut m: u64) -> Option<u64> {
    n = n * 10 + d as u64;
    m *= 10;

    if index == 0 {
        if let Some(wildcard) = w {
            evaluate_candidate(n, m, wildcard)
        } else {
            // At least one wildcard is required
            None
        }
    } else {
        solve_for_each_digit(index - 1, w, n, m)
    }
}

fn solve_wildcard_digit(
    index: usize,
    d: u8,
    mut w: Option<u8>,
    mut n: u64,
    mut m: u64,
) -> Option<u64> {
    if let Some(wildcard) = w {
        // The same digit must be in all the wildcard positions.
        // Since d is a wildcard digit, it must match the previously chosen lowest wildcard value.
        if wildcard != d {
            return None;
        }
    } else {
        w = Some(d);
    }

    n *= 10;
    m = m * 10 + 1;
    if index == 0 {
        evaluate_candidate(n, m, d)
    } else {
        solve_for_each_digit(index - 1, w, n, m)
    }
}

#[inline]
fn evaluate_candidate(n: u64, m: u64, wildcard: u8) -> Option<u64> {
    let candidate_solution = n + m * wildcard as u64;
    if !is_prime(candidate_solution) {
        None
    } else {
        for lower_wc in 0..wildcard {
            if is_prime(n + m * lower_wc as u64) {
                // This set of numbers sharing the same wildcard pattern has been
                // checked previously, without finding a solution, so don't try again
                return None;
            }
        }
        let max_non_primes = 10 - (TARGET_PRIME_COUNT as u8);
        let mut non_prime_count = wildcard;
        let mut prime_count = 1;
        for digit in (wildcard + 1)..10 {
            if !is_prime(n + m * digit as u64) {
                non_prime_count += 1;
                if non_prime_count > max_non_primes {
                    return None;
                }
            } else {
                prime_count += 1;
                if prime_count > TARGET_PRIME_COUNT {
                    // The family size must be exactly the target, not more
                    return None;
                }
            }
        }

        #[cfg(debug_assertions)]
        {
            for digit in wildcard..10 {
                let prime_candidate = n + m * digit as u64;
                if is_prime(prime_candidate) {
                    println!("{prime_candidate} is prime");
                }
            }
            println!("Mask: {m}")
        }

        Some(candidate_solution)
    }
}

#[inline]
fn solve_for_each_digit(index: usize, w: Option<u8>, n: u64, m: u64) -> Option<u64> {
    (0..10).fold(None, |acc, digit| {
        acc.or_else(|| solve_next_digit(index, digit, w, n, m))
    })
}

// n is a multi-digit number. Test whether it is prime.
fn is_prime(n: u64) -> bool {
    if n < 11 {
        return n == 2 || n == 3 || n == 5 || n == 7;
    }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let int_sqrt = n.sqrt();

    // We already know n is not divisible by 2 or 3.
    // If it is composite, then its prime factors must have the form 6i +/- 1.
    // So only check divisors (possibly composite) of that form.
    // Also, consider the smaller of a pair of divisors (possibly equal) that multiply to n.
    // It must be <= sqrt(n), so stop checking divisors above this number.
    for multiple_of_6 in 1.. {
        let factor = 6 * multiple_of_6 - 1;
        if factor > int_sqrt {
            return true;
        }
        if n % factor == 0 {
            return false;
        }

        let factor = 6 * multiple_of_6 + 1;
        if factor > int_sqrt {
            return true;
        }
        if n % factor == 0 {
            return false;
        }
    }
    panic!("Should be unreachable, in is_prime()")
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
