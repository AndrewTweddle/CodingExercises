use num::integer::Roots;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 100)
}

fn solve() -> usize {
    // Seed with the first 2 spiral layers, so that all further primes will be of the form 6n+/-1
    let mut prime_count = 3;
    let mut total_count = 5;
    let mut num = 9;

    for layer in 3.. {
        let jump = 2 * (layer - 1);

        for _ in 0..3 {
            num += jump;
            total_count += 1;
            if is_prime(num) {
                prime_count += 1;
            }
        }
        // The fourth is a square number so it is never prime
        total_count += 1;
        if 10 * prime_count < total_count {
            return jump + 1;
        }
        num += jump;
    }
    // Unreachable...
    panic!("No solution found");
}

// n is an odd number greater than 7. Test whether it is prime.
fn is_prime(n: usize) -> bool {
    if n % 3 == 0 {
        return false;
    }

    let int_sqrt = n.sqrt();

    // We already know n is not divisible by 2 or 3.
    // If it is composite, then its prime factors must have the form 6i +/- 1.
    // So only check divisors (possibly composite) of that form.
    // Also, consider the smaller of a pair of divisors (possibly equal) that multiply to n.
    // It must be <= sqrt(n), so stop checking divisors above this number (except maybe 1)
    let max_multiple_of_6 = (int_sqrt + 1) / 6;

    for multiple_of_6 in 1..=max_multiple_of_6 {
        let mut factor = 6 * multiple_of_6 - 1;
        if n % factor == 0 {
            return false;
        }

        factor += 2;
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
