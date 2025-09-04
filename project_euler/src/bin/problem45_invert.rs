use num::integer::Roots;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 1000;

// Start from the next values of n, for the hexagonal number
// after the solution given in the problem statement...
const MIN_HEX_N: u64 = 144;

struct Solution {
    n_tri: u64,
    n_pen: u64,
    n_hex: u64,
    answer: u64,
}

fn main() {
    let mut start_time = Instant::now();
    for i in 0..=NUM_REPETITIONS {
        let solution = solve().unwrap();

        // Only print out the solution once...
        if i == 0 {
            println!(
                "Solution: T({}) = P({}) = H({}) = {}",
                solution.n_tri, solution.n_pen, solution.n_hex, solution.answer
            );
            println!("Duration (incl. I/O): {:?}", start_time.elapsed());

            // restart timer to exclude I/O
            start_time = Instant::now();
        }
    }

    let total_duration = start_time.elapsed();
    println!(
        "Average duration over {NUM_REPETITIONS} repetitions: {:?}",
        total_duration / NUM_REPETITIONS
    );
}

fn solve() -> Option<Solution> {
    for n_hex in MIN_HEX_N.. {
        let h = hex(n_hex);
        if let Some(n_pen) = invert_pentagonal(h)
            && let Some(n_tri) = invert_triangular(h)
        {
            return Some(Solution {
                n_tri,
                n_pen,
                n_hex,
                answer: h,
            });
        }
    }
    None
}

#[inline]
fn invert_triangular(n: u64) -> Option<u64> {
    let m = 8 * n + 1;
    let s = m.sqrt();
    (s * s == m).then_some((s - 1) / 2)
}

#[inline]
fn invert_pentagonal(p: u64) -> Option<u64> {
    let m = 24 * p + 1;
    let s = m.sqrt();
    ((s * s == m) && (s % 6 == 5)).then_some((s + 1) / 6)
}

#[inline]
fn hex(n: u64) -> u64 {
    n * (2 * n - 1)
}
