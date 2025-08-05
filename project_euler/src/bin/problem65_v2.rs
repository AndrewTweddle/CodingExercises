use num::Zero;
use num_bigint::BigInt;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000)
}

fn solve() -> BigInt {
    solve_for_n(100)
}

/// Uses the formula for convergents of a continued fraction on
/// [Wikipedia](https://en.wikipedia.org/wiki/Simple_continued_fraction#Infinite_continued_fractions_and_convergents).
fn solve_for_n(n: usize) -> BigInt {
    let mut prev_h: BigInt = BigInt::from(0); // h[-2]
    let mut curr_h: BigInt = BigInt::from(1); // h[-1]
    let mut prev_k: BigInt = BigInt::from(1); // k[-2]
    let mut curr_k: BigInt = BigInt::from(0); // k[-1]

    let mut next_a: usize;
    let mut next_h: BigInt;
    let mut next_k: BigInt;

    for i in 0..n {
        // e has the continued fraction form: [2; 1, 2, 1, 1, 4, 1, 1, 6, 1, ... 1, 2k, 1, ...]
        next_a = if i == 0 {
            2
        } else {
            match i % 3 {
                2 => 2 * ((i + 1) / 3),
                _ => 1,
            }
        };
        next_h = next_a * &curr_h + &prev_h;
        next_k = next_a * &curr_k + &prev_k;

        // Prepare for the next iteration
        prev_h = curr_h;
        curr_h = next_h;
        prev_k = curr_k;
        curr_k = next_k;
    }

    // Calculate the sum of the digits of the numerator
    let mut digit_sum = BigInt::zero();

    while curr_h > BigInt::zero() {
        digit_sum += &curr_h % 10;
        curr_h /= 10;
    }
    digit_sum
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
    fn test_for_1st_convergent() {
        let sum_of_digits = solve_for_n(1);
        assert_eq!(sum_of_digits, BigInt::from(2));
    }

    #[test]
    fn test_for_4th_convergent() {
        // Fourth convergent = 11/4, so numerator = 11 and its digits sum to 2.
        let sum_of_digits = solve_for_n(4);
        assert_eq!(sum_of_digits, BigInt::from(2));
    }

    #[test]
    fn test_for_10th_convergent() {
        // Tenth convergent = 1457/536, so numerator = 1457 and its digits sum to 17.
        let sum_of_digits = solve_for_n(10);
        assert_eq!(sum_of_digits, BigInt::from(17));
    }
}
