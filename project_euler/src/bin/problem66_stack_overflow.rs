use itertools::{EitherOrBoth::*, Itertools};

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 0)
}

fn solve() -> usize {
    solve_for_d_up_to(60)
}

fn solve_for_d_up_to(max_d: usize) -> usize {
    let mut min_x_by_d: Vec<Option<usize>> = vec![None; max_d + 1];

    // We want to find the maximum x, so set invalid values of d to have an x value of 0.
    min_x_by_d[0] = Some(0);

    // All square numbers are excluded
    for i in 1.. {
        let s = i * i;
        if s > max_d {
            break;
        }
        min_x_by_d[s] = Some(0);
    }

    // Maintain a list of primes which will be populated as we encounter them.
    let mut primes: Vec<usize> = vec![2];

    // Factorize sets of 3 integers at a time (storing the last 2 and calculating the next 1).
    // This is because D y^2 = (x^2 - 1) = (x - 1)(x + 1).
    // So to get the factorization of x^2 - 1, we will calculate the factorization of x + 1,
    // and store it for later use (since x + 1 = (x + 2) - 1, we will re-use it in 2 loops' time.
    // We will then retrieve the factorization of x - 1, and combine these to get the prime
    // factorization of x^2 - 1.

    // Add factorizations for x = 1 and x = 2:
    let mut x1_vec = vec![];
    let mut x2_vec = vec![1];
    let mut x3_vec = vec![];

    let mut max_unfilled_d: usize = min_x_by_d
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, &x)| x.is_none())
        .map(|(i, _)| i)
        .next()
        .unwrap();

    let mut prev_factorization = &mut x1_vec;
    let mut curr_factorization = &mut x2_vec;
    let mut next_factorization = &mut x3_vec;

    for x in 2.. {
        next_factorization.clear();
        let mut xrem = x + 1;
        for &prime in &primes {
            let mut prime_exp = 0;
            while xrem % prime == 0 {
                prime_exp += 1;
                xrem /= prime;
            }
            next_factorization.push(prime_exp);
        }
        if xrem != 1 {
            // xrem must be a prime
            next_factorization.push(1);
            primes.push(xrem);
        }

        // Determine the factorization of x^2 - 1:
        let x_sq_minus_1: Vec<usize> = prev_factorization
            .iter()
            .zip_longest(next_factorization.iter())
            .map(|eob| match eob {
                Both(&a, &b) => a + b,
                Left(&a) => a,
                Right(&b) => b,
            })
            .collect();

        // Now we want to find divisors D of x^2 - 1, which don't yet have a minimum x value,
        // and for which (x^2 - 1) / D is a square. Generate possible values of D recursively.
        let candidate = search_for_d_updating_min_x_vals(
            1,
            &mut max_unfilled_d,
            &mut min_x_by_d,
            &primes,
            &x_sq_minus_1,
            x,
        );

        if let Some(best_d) = candidate {
            return best_d;
        }

        // Prepare for the next iteration:
        (prev_factorization, curr_factorization, next_factorization) =
            (curr_factorization, next_factorization, prev_factorization);
    }
    unreachable!()
}

fn search_for_d_updating_min_x_vals(
    mut d: usize,
    max_unfilled_d: &mut usize,
    min_x_by_d: &mut [Option<usize>],
    primes_left: &[usize],
    exponents_left: &[usize],
    x: usize,
) -> Option<usize> {
    if exponents_left.is_empty() {
        if d <= *max_unfilled_d && min_x_by_d[d].is_none() {
            min_x_by_d[d] = Some(x);

            /* For testing and debugging:
            let y = ((x * x - 1)/d).isqrt();
            println!("Found D = {d}, x = {x}, y = {y}");
             */

            if d == *max_unfilled_d {
                let new_max_unfilled_d = min_x_by_d
                    .iter()
                    .enumerate()
                    .take(*max_unfilled_d)
                    .rev()
                    .filter(|(_, &x)| x.is_none())
                    .map(|(i, _)| i)
                    .next();
                if let Some(new_max_unfilled_d) = new_max_unfilled_d {
                    *max_unfilled_d = new_max_unfilled_d;
                } else {
                    return Some(d);
                }
            }
        }
        return None;
    }

    let prime = primes_left[0];
    let exponent = exponents_left[0];

    // y^2 has an even number of each prime factor,
    // so D must have a factor of: prime ^ (exponent - 2 * i) for i = 0, 1, ...
    if exponent % 2 != 0 {
        d *= prime;
    }

    let prime_sq = prime * prime;
    for _y_pow in 0..=(exponent / 2) {
        if d > *max_unfilled_d {
            return None;
        }
        let best_d = search_for_d_updating_min_x_vals(
            d,
            max_unfilled_d,
            min_x_by_d,
            &primes_left[1..],
            &exponents_left[1..],
            x,
        );
        if best_d.is_some() {
            return best_d;
        }
        d *= prime_sq;
    }
    None
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
        let end_time = Instant::now();
        println!("Ending iterations at {end_time:?}");

        let avg_duration = total_elapsed / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
        println!("Total elapsed time for {repetitions} runs: {total_elapsed:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_d_of_2() {
        assert_eq!(solve_for_d_up_to(2), 2);
    }

    #[test]
    fn test_d_of_3() {
        assert_eq!(solve_for_d_up_to(3), 2);
    }

    #[test]
    fn test_d_of_7() {
        assert_eq!(solve_for_d_up_to(7), 5);
    }
}
