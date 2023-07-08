use std::time::Instant;

const NUM_REPETITIONS: u32 = 1000;

// Start from the next values of n after the solution given in the problem statement
const MIN_TRI_N: u64 = 286;
const MIN_PEN_N: u64 = 166;
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

#[allow(clippy::comparison_chain)]
fn solve() -> Option<Solution> {
    let mut n_tri_iter = MIN_TRI_N..;
    let mut n_pen_iter = MIN_PEN_N..;

    for n_hex in MIN_HEX_N.. {
        let h = hex(n_hex);
        for n_pen in &mut n_pen_iter {
            let p = pen(n_pen);
            if p == h {
                for n_tri in &mut n_tri_iter {
                    let t = tri(n_tri);
                    if t == p {
                        return Some(Solution {
                            n_tri,
                            n_pen,
                            n_hex,
                            answer: h,
                        });
                    }
                    if t > p {
                        break;
                    }
                }
                break;
            }
            if p > h {
                break;
            }
        }
    }
    None
}

#[inline]
fn tri(n: u64) -> u64 {
    n * (n + 1) / 2
}

#[inline]
fn pen(n: u64) -> u64 {
    n * (3 * n - 1) / 2
}

#[inline]
fn hex(n: u64) -> u64 {
    n * (2 * n - 1)
}
