use num::integer::Roots;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 100);
}

// Note: 1_000_000 = 6 * 166_666 + 4, so the last pair to check will be 999_995 and 999_997
const MAX_N: u32 = 999_999;

fn solve() -> u32 {
    let mut primes: Vec<u32> = Vec::with_capacity(MAX_N as usize);

    primes.push(2);
    primes.push(3);

    for n in (5..=MAX_N).step_by(6) {
        if is_prime(n) {
            primes.push(n);
        }
        let n2 = n + 2;
        if is_prime(n2) && n2 <= MAX_N {
            primes.push(n2);
        }
    }

    // Advance until we have used an even number of primes, and we are over the maximum
    let mut cum_sum: u32 = 0;
    let mut max_num_steps: usize = 0;
    for &p in &primes {
        cum_sum += p;
        max_num_steps += 1;
        if max_num_steps % 2 == 0 && cum_sum > MAX_N {
            break;
        }
    }

    let mut prev_even_sum = cum_sum;

    for num_steps in (1..max_num_steps).rev() {
        if num_steps % 2 == 0 {
            // An even number of terms can't all be odd, so must include 2:
            cum_sum = prev_even_sum - primes[num_steps] - primes[num_steps + 1];
            if primes.binary_search(&cum_sum).is_ok() {
                return cum_sum;
            }
            prev_even_sum = cum_sum;
        } else {
            // the previous iteration used a cum_sum that included 2
            cum_sum -= 2;
            let mut i = 1;
            let mut j = num_steps + 1;
            while cum_sum <= MAX_N {
                if primes.binary_search(&cum_sum).is_ok() {
                    return cum_sum;
                }
                cum_sum += primes[j] - primes[i];
                i += 1;
                j += 1;
            }
        }
    }
    0
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
