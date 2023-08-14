use num::integer::Roots;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10);
}

type IndexPrimeAndCumSum = (usize, u32, u64);

// Note: 1_000_000 = 6 * 166_666 + 4, so the last pair to check will be 999_995 and 999_997
const MAX_N: u32 = 1_000_000;

fn solve() -> u32 {
    let mut cum_primes: Vec<IndexPrimeAndCumSum> = Vec::with_capacity(MAX_N as usize);
    cum_primes.push((0, 0, 0));
    cum_primes.push((1, 2, 2));
    cum_primes.push((2, 3, 5));
    let mut ix: usize = 2;
    let mut cum: u64 = 5;

    for n in (5..=MAX_N).step_by(6) {
        if is_prime(n) {
            ix += 1;
            cum += n as u64;
            cum_primes.push((ix, n, cum));
        }
        let n2 = n + 2;
        if is_prime(n2) && n2 <= MAX_N {
            ix += 1;
            cum += n2 as u64;
            cum_primes.push((ix, n2, cum));
        }
    }

    let mut best_num_steps: usize = 0;
    let mut best_prime: u32 = 0;

    for &(j, _, upper_cum_sum) in &mut cum_primes.iter().skip(1).rev() {
        for &(i, _, lower_cum_sum) in &mut cum_primes.iter().take(j - 1) {
            if i + best_num_steps >= j {
                break;
            }
            let sum_of_primes = upper_cum_sum - lower_cum_sum;
            if sum_of_primes > MAX_N as u64 {
                continue;
            }
            if cum_primes
                .binary_search_by(|&(_, prime, _)| prime.cmp(&(sum_of_primes as u32)))
                .is_ok()
            {
                best_num_steps = j - i;
                best_prime = sum_of_primes as u32;

                #[cfg(debug_assertions)]
                {
                    let prime_sum_strs: Vec<String> = cum_primes[(i + 1)..=j]
                        .iter()
                        .map(|&(_, p, _)| p.to_string())
                        .collect();
                    println!(
                        "{} terms: {} = {}",
                        j - i,
                        best_prime,
                        prime_sum_strs.join(" + "),
                    );
                }
            }
        }
    }

    best_prime
}

#[inline]
fn is_prime(n: u32) -> bool {
    let int_sqrt = n.sqrt();

    // This function is only called with candidate values for n of the form form 6k +/- 1.
    // So we already know it is not divisible by 2 or 3. If n has this form, but is not prime,
    // then one of its prime factors will also be of this form.
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
        let avg_duration = start_time.elapsed() / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
    }
}
