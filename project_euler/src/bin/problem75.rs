use num::integer::gcd;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 100)
}

const MAX_L: usize = 1_500_000;
const MAX_HALF_L: usize = MAX_L / 2;

fn solve() -> usize {
    solve_for_l_up_to(MAX_L)
}

#[derive(Default, Copy, Clone)]
struct WireLength {
    count: usize,

    // When the count is 1, track the smallest side of the triangle.
    // This is to make it easy to check for duplicate vs novel configurations.
    smaller_side: usize,
}

fn solve_for_l_up_to(l: usize) -> usize {
    let mut wire_lengths_by_half_perimeter = vec![WireLength::default(); MAX_HALF_L + 1];

    let half_l = l / 2;
    for m in 2..=half_l.isqrt() {
        for n in 1..m {
            // m > n > 0

            // Choose m and n coprime
            if gcd(m, n) != 1 {
                continue;
            }

            let min_a = m * m - n * n;
            let min_b = 2 * m * n;

            // This uses Euclid's formula with a multiplier k.
            // See: https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple
            //
            // For k >= 1,
            // let a = k (m^2 - n^2),
            //     b = 2kmn,
            //     c = k(m^2 + n^2)
            // Then:  (a, b c) is a Pythagorean triple.
            // Perimeter of the right-angled triangle is:
            //     p = k(m^2 - n ^2 + 2mn + m^2 + n^2) = 2km(m + n)

            // Get the half-perimeter h when k = 1:
            let h = m * (m + n);
            if h > half_l {
                break;
            }
            let mut half_perimeter = h;
            let smaller_side_delta = min_a.min(min_b);
            let mut smaller_side = smaller_side_delta;

            while half_perimeter <= half_l {
                let wire_length = &mut wire_lengths_by_half_perimeter[half_perimeter];
                match (wire_length.count, wire_length.smaller_side) {
                    (0, _) => {
                        *wire_length = WireLength {
                            count: 1,
                            smaller_side,
                        }
                    }
                    (1, ss) if ss != smaller_side => {
                        wire_length.count = 2;
                    }
                    _ => {
                        // Nothing to do. Either a duplicate configuration.
                        // Or the count at this wire length is already more than 1.
                    }
                }
                half_perimeter += h;
                smaller_side += smaller_side_delta;
            }
        }
    }

    wire_lengths_by_half_perimeter
        .iter()
        .filter(|&&wire_length| wire_length.count == 1)
        .count()
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
    fn test_solve_for_l_of_20() {
        assert_eq!(solve_for_l_up_to(20), 1);
    }

    #[test]
    fn test_solve_for_l_of_30() {
        assert_eq!(solve_for_l_up_to(30), 3);
    }

    #[test]
    fn test_solve_for_l_of_120() {
        assert_eq!(solve_for_l_up_to(120), 12);
    }
}
