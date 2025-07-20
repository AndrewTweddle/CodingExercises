use num::Zero;
use num_bigint::BigInt;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000)
}

fn solve() -> BigInt {
    solve_for_3n_plus_1(33)
}

fn solve_for_3n_plus_1(n: usize) -> BigInt {
    let mut num: BigInt = BigInt::from(0);
    let mut denom: BigInt = BigInt::from(1);
    
    // Calculate the numerator and denominator in reverse order
    for i in 0..n {
        num += &denom;
        simplify(&mut num, &mut denom);
        (denom, num) = (num, denom);
        num += 2 * (n - i) * &denom;
        simplify(&mut num, &mut denom);
        (denom, num) = (num, denom);
        num += &denom;
        simplify(&mut num, &mut denom);
        (denom, num) = (num, denom);
    }
    num += 2 * &denom;
    simplify(&mut num, &mut denom);
    
    // Calculate the sum of the digits of the numerator
    let mut digit_sum = BigInt::zero();
    
    while num > BigInt::zero() {
        digit_sum += &num % 10;
        num /= 10;
    }
    digit_sum
}

fn simplify(num: &mut BigInt, denom: &mut BigInt) {
    let g = gcd(num.clone(), denom.clone());
    *num /= &g;
    *denom /= g;
}

fn gcd(a: BigInt, b: BigInt) -> BigInt {
    if b == BigInt::zero() {
        a
    } else {
        let rem = a % &b;
        gcd(b, rem)
    }
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
        let sum_of_digits = solve_for_3n_plus_1(0);
        assert_eq!(sum_of_digits, BigInt::from(2));
    }
    
    #[test]
    fn test_for_4th_convergent() {
        // Fourth convergent = 11/4, so numerator = 11 and its digits sum to 2.
        let sum_of_digits = solve_for_3n_plus_1(1);
        assert_eq!(sum_of_digits, BigInt::from(2));
    }
    
    #[test]
    fn test_for_10th_convergent() {
        // Tenth convergent = 1457/536, so numerator = 1457 and its digits sum to 17.
        let sum_of_digits = solve_for_3n_plus_1(3);
        assert_eq!(sum_of_digits, BigInt::from(17));
    }
}
