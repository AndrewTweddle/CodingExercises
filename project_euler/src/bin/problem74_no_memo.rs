use num::integer::div_rem;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10)
}

const MAX_N: usize = 1_000_000;

fn solve() -> usize {
    let mut count_of_chains_with_sixty_terms: usize = 0;
    for n in 1..MAX_N {
        let mut chain_length = 0;
        let mut last_4_visited = [0; 4];
        let mut next_index = 0;
        let mut curr = n;

        while !last_4_visited.contains(&curr) {
            last_4_visited[next_index] = curr;
            next_index = (next_index + 1) % 4;
            curr = sum_of_factorials_of_digits(curr);
            chain_length += 1;
        }

        if chain_length == 60 {
            count_of_chains_with_sixty_terms += 1;
        }
    }
    count_of_chains_with_sixty_terms
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
