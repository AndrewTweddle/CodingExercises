use num::{One, Zero};
use num_bigint::{BigUint, ToBigUint};

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000)
}

fn solve() -> usize {
    solve_for_d_up_to(1000)
}

fn solve_for_d_up_to(max_d: usize) -> usize {
    let mut max_min_x: BigUint = BigUint::from(0u32);
    let mut best_d = 0;

    for d in (max_d / 4) + 1..=max_d {
        let sqrt_d = d.isqrt();
        if sqrt_d * sqrt_d == d {
            continue;
        }
        let min_x = calculate_min_x_in_pells_equation(d as u32);
        if min_x > max_min_x {
            max_min_x = min_x;
            best_d = d;
        }
    }
    best_d
}

fn calculate_min_x_in_pells_equation(d: u32) -> BigUint {
    let a0 = d.isqrt().to_biguint().unwrap();
    let s = BigUint::from(d);
    // Note that a variable `d` is used in the Wikipedia article for continued fractions.
    // So the variable `s` will now represent the D value in Pell's Equation,
    // and the `d` parameter will be shodowed by a new `d` variable.

    // Variables used for calculating partial fractions: h[i] / k[i]
    let mut h_sub_2: BigUint = BigUint::zero(); // h[-2]
    let mut h_sub_1: BigUint = BigUint::one(); // h[-1]
    let mut h: BigUint;

    let mut k_sub_2: BigUint = BigUint::one(); // k[-2]
    let mut k_sub_1: BigUint = BigUint::zero(); // k[-1]
    let mut k: BigUint;

    // Variables used for calculating a[i]
    let mut m = BigUint::zero();
    let mut d = BigUint::one();
    let mut a = a0.clone();
    let mut stopping_i: Option<usize> = None;

    for i in 0.. {
        // To get the partial fraction, use the formula for convergents of a continued fraction at
        // [Wikipedia](https://en.wikipedia.org/wiki/Simple_continued_fraction#Infinite_continued_fractions_and_convergents)

        // Calculate h[i] and k[i]:
        h = &a * &h_sub_1 + &h_sub_2;
        k = &a * &k_sub_1 + &k_sub_2;

        // To get the index where the numerator equals the minimum value of x,
        // use the formula from the Pell's equation article at
        // [Wikipedia](https://en.wikipedia.org/wiki/Pell%27s_equation#Fundamental_solution_via_continued_fractions)
        if let Some(last_i) = stopping_i {
            if i == last_i {
                return h;
            }
        } else if a == &a0 * 2_u32 {
            if i % 2 == 0 {
                return h_sub_1; // h[i-1] for even i
            } else if i == 1 {
                return h; // h[2i-1] for odd i, but for i = 1, i == 2i-1, so just return h[i]
            } else {
                stopping_i = Some(2 * i - 1); // h[2i-1] for odd i
            }
        }

        // Prepare for next iteration
        h_sub_2 = h_sub_1;
        h_sub_1 = h;
        k_sub_2 = k_sub_1;
        k_sub_1 = k;

        // Use the formula at https://en.wikipedia.org/wiki/Periodic_continued_fraction#Canonical_form_and_repetend
        // to get the next a[n] value.
        m = &d * &a - &m;
        d = (&s - &m * &m) / &d;
        a = (&a0 + &m) / &d; // a[i+1]
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
    fn test_d_of_13() {
        assert_eq!(
            calculate_min_x_in_pells_equation(13),
            BigUint::from(649_u32)
        );
    }

    #[test]
    fn test_d_of_3() {
        assert_eq!(calculate_min_x_in_pells_equation(3), BigUint::from(2_u32));
    }

    #[test]
    fn test_up_to_d_of_2() {
        assert_eq!(solve_for_d_up_to(2), 2);
    }

    #[test]
    fn test_up_to_d_of_3() {
        assert_eq!(solve_for_d_up_to(3), 2);
    }

    #[test]
    fn test_up_to_d_of_7() {
        assert_eq!(solve_for_d_up_to(7), 5);
    }
}
