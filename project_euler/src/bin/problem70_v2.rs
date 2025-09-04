fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1)
}

type N = u64;
type DigitCount = [u8; 10];
const MAX_N: N = 10_000_000;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Ratio {
    /// The numerator of the ratio
    n: N,

    /// The denominator of the ratio
    phi: N,
}

fn solve() -> N {
    // We want to find n and phi(n) such that:
    // 1. phi(n) is a permutation of n (so has the same # of digits)
    // 2. Of all such 1 < n <= 10^7, n/phi(n) is minimal.
    //
    // Solution notes:
    // 1. Note that n/phi(n) = product{p/(p-1)} for all primes p such that p divides n.
    //    See https://en.wikipedia.org/wiki/Euler%27s_totient_function#Euler's_product_formula
    // 2. If n is a prime, then phi(n) = n-1, which cannot be a permutation of n.
    //    So the values of n we care about have at least 2 factors (other than 1),
    //    So either all of the prime factors, or all but 1 of the prime factors, are <= sqrt(n)
    //    otherwise their product would be > n.
    // 3. So we only need to consider primes <= sqrt(n). If we've reduced a candidate n by all such
    //    prime factors, then either we are left with 1 or the unreduced value of n is the final
    //    prime factor.
    // 4. We don't need to keep a list of primes <= sqrt(10^7). We can just consider
    //    2, 3 and all integers <= sqrt(max_n) of the form 6m+1 or 6m-1, where m is an integer.
    //    (Any other integers are divisible by 2 or 3, so they are not prime).
    //    This will be more efficient than retrieving values from a list of primes.
    // 5. We can keep track of the number of digits in n, factorizing n into prime factors,
    //    and deriving phi(n) by starting it out as n, and repeatedly multiplying it by (p - 1)/p.
    //    We can reject n as soon as this partial phi(n) has fewer digits than n.
    // 6. If we encounter a prime factor p that divides n more than once, then we can ignore n.
    //    This is because phi(n/p) == phi(n). So (n/p)/phi(n/p) < n/phi(n).
    // 7. Once fully factorized, check if n/phi(n) is less than the min ratio found so far.
    //    We can do this efficiently by using the numerator and denominator of n, phi(n),
    //    best_n and best_phi, avoiding a floating point division.
    // 8. If it is, or if no minimum ratio has been found, check if phi(n) is a permutation of n.
    // 9. If it is, then update best_n and best_phi.
    //
    // NB: All these steps are the same as version 1, except for step 6 being inserted.

    let mut min_ratio: Option<Ratio> = None;
    let mut pow_of_10: N = 1;
    let mut next_pow_of_10: N = 10;

    let sqrt_max_n = MAX_N.isqrt();
    let mut n = 3;

    // Start from n = 4, since 2 and 3 are prime, so aren't candidates for n.
    for sqrt_n in 2..=sqrt_max_n {
        // i is the integer sqrt of n
        'next_n: for _ in 0..(2 * sqrt_n + 1) {
            n += 1;
            if n == next_pow_of_10 {
                pow_of_10 = next_pow_of_10;
                if pow_of_10 == MAX_N {
                    return min_ratio.unwrap().n;
                }
                next_pow_of_10 *= 10;

                // phi(n) < n, so we can skip this n
                continue;
            }

            // Factorize n into prime factors
            let mut reduced_n = n;
            let mut phi = n;

            if try_reduce_by_prime_factor(&mut reduced_n, &mut phi, 2) == Reduction::MultipleFactor {
                continue;
            }
            if try_reduce_by_prime_factor(&mut reduced_n, &mut phi, 3) == Reduction::MultipleFactor {
                continue;
            }
            for mid_p in (6..=sqrt_n).step_by(6) {
                match try_reduce_by_prime_factor(&mut reduced_n, &mut phi, mid_p - 1) {
                    Reduction::NotAFactor => {}
                    Reduction::SingleFactor => {
                        if reduced_n == 1 {
                            break;
                        }
                        if phi < pow_of_10 {
                            continue 'next_n;
                        }
                    }
                    Reduction::MultipleFactor => continue 'next_n,
                }

                match try_reduce_by_prime_factor(&mut reduced_n, &mut phi, mid_p + 1) {
                    Reduction::NotAFactor => {}
                    Reduction::SingleFactor => {
                        if reduced_n == 1 {
                            break;
                        }
                        if phi < pow_of_10 {
                            continue 'next_n;
                        }
                    }
                    Reduction::MultipleFactor => continue 'next_n,
                }
            }

            if reduced_n > 1 {
                let p = reduced_n;
                try_reduce_by_prime_factor(&mut reduced_n, &mut phi, p); 
                if phi < pow_of_10 {
                    continue 'next_n;
                }
            }

            if let Some(r) = min_ratio
                && (n * r.phi >= r.n * phi)
            {
                // This would not produce a higher n / phi ration than the best ratio found so far
                continue;
            }

            let n_digit_count: DigitCount = count_digits(n);
            let phi_digit_count: DigitCount = count_digits(phi);
            if n_digit_count == phi_digit_count {
                min_ratio = Some(Ratio { n, phi });

                #[cfg(debug_assertions)]
                println!(
                    "New minimum ratio. n = {n}, phi(n) = {phi}. Ratio = {}",
                    (n as f32) / (phi as f32)
                );
            }
        }
    }

    unreachable!();
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
enum Reduction {
    NotAFactor,
    SingleFactor,
    MultipleFactor,
}

fn try_reduce_by_prime_factor(n: &mut N, phi: &mut N, p: N) -> Reduction {
    if *n % p != 0 {
        return Reduction::NotAFactor;
    }
    *phi /= p;
    *phi *= p - 1;
    *n /= p;
    if *n % p == 0 {
        Reduction::MultipleFactor
    } else {
        Reduction::SingleFactor
    }
}

fn count_digits(mut n: N) -> DigitCount {
    let mut digit_count = [0; 10];
    while n > 0 {
        let d = n % 10;
        n /= 10;
        digit_count[d as usize] += 1;
    }
    digit_count
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
