use num::integer::div_rem;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 100)
}

const MAX_N: usize = 1_000_000;
const MAX_SUM_OF_FACTORIALS: usize = 6 * (2 * 3 * 4 * 5 * 6 * 7 * 8 * 9); // 999_999 -> 6 x 9!

fn solve() -> usize {
    let mut count_of_chains_with_sixty_terms: usize = 0;
    let mut chain_lengths: Vec<usize> = vec![0; MAX_SUM_OF_FACTORIALS + 1];

    // The problem statement tells us that the following are the only cycles:
    add_cycle_to_chain_lengths(&mut chain_lengths, 169, 3);
    add_cycle_to_chain_lengths(&mut chain_lengths, 871, 2);
    add_cycle_to_chain_lengths(&mut chain_lengths, 872, 2);

    for n in 0..MAX_N {
        if get_chain_length_from_cache(&mut chain_lengths, n) == 60 {
            count_of_chains_with_sixty_terms += 1;
        }
    }
    count_of_chains_with_sixty_terms
}

fn add_cycle_to_chain_lengths(chain_lengths: &mut [usize], start: usize, cycle_length: usize) {
    let mut curr = start;
    for _ in 0..cycle_length {
        chain_lengths[curr] = cycle_length;
        curr = sum_of_factorials_of_digits(curr);
    }
}

fn get_chain_length_from_cache(chain_lengths: &mut [usize], n: usize) -> usize {
    let mut chain_length = chain_lengths[n];
    if chain_length == 0 {
        let sum_of_factorials = sum_of_factorials_of_digits(n);
        if sum_of_factorials == n {
            chain_length = 1;
        } else {
            chain_length = 1 + get_chain_length_from_cache(chain_lengths, sum_of_factorials);
        }
        chain_lengths[n] = chain_length;
    }
    chain_length
}

fn sum_of_factorials_of_digits(mut n: usize) -> usize {
    let mut total = 0;
    let mut digit: usize;
    while n != 0 {
        (n, digit) = div_rem(n, 10);
        total += factorial(digit);
    }
    total
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
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
    fn test_sum_of_factorials_of_digits_of_145() {
        assert_eq!(sum_of_factorials_of_digits(145), 145);
    }

    #[test]
    fn test_zero_factorial() {
        assert_eq!(factorial(0), 1);
    }
}
