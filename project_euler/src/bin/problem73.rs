fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10)
}

type N = u64;
const ORDER_OF_FAREY_SEQUENCE: N = 12_000;

fn solve() -> N {
    solve_for_order(ORDER_OF_FAREY_SEQUENCE)
}

type Frac = (N, N);

fn solve_for_order(seq_order: N) -> N {
    let half = (1, 2);
    let third = (1, 3);
    count_terms_between(half, third, seq_order)
}

/// Count the number of terms in the Farey sequence of order `seq_order` that exist strictly between
/// two given fractions `a/b` and `c/d`. The Farey sequence of order `n` is defined as the
/// sequence of all reduced fractions `p/q` such that `0 <= p <= q <= n; q <> 0`,
/// sorted in ascending order.
fn count_terms_between((a, b): Frac, (c, d): Frac, seq_order: N) -> N {
    // Calculate the mediant
    let m = reduce((a + c, b + d));
    if m.1 > seq_order {
        0
    } else {
        count_terms_between((a, b), m, seq_order) + 1 + count_terms_between(m, (c, d), seq_order)
    }
}

fn reduce((n, d): Frac) -> Frac {
    let g = gcd(n, d);
    (n / g, d / g)
}

fn gcd(a: N, b: N) -> N {
    if b == 0 { a } else { gcd(b, a % b) }
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
    fn test_solve_for_order_8() {
        assert_eq!(solve_for_order(8), 3);
    }
}
