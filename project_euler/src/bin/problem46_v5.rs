use num::integer::Roots;
use std::collections::HashSet;
use std::time::Instant;

const REPETITIONS: u32 = 1000;

fn main() {
    let mut start_time = Instant::now();

    for i in 0..=REPETITIONS {
        let solution = solve().unwrap();

        if i == 0 {
            println!("Solution: {solution}");
            println!(
                "Duration (incl. writing to terminal): {:?}",
                start_time.elapsed()
            );
            start_time = Instant::now();
        }
    }

    let avg_duration = start_time.elapsed() / REPETITIONS;
    println!("Avg duration (excl I/O) over {REPETITIONS} repetitions: {avg_duration:?}");
}

fn solve() -> Option<u64> {
    for i in (3..).step_by(2) {
        if !is_odd_prime(i) && !composite_satisfies_conjecture(i) {
            return Some(i);
        }
    }
    None
}

#[inline]
fn is_odd_prime(n: u64) -> bool {
    if n < 11 {
        return n == 3 || n == 5 || n == 7;
    }

    // if n is prime, it must be of the form 6k+1 or 6k+5...
    let mod_6 = n % 6;
    if mod_6 != 1 && mod_6 != 5 {
        return false;
    }

    // And if it has this form, but is not prime,
    // then one of its prime factors will also be of this form
    for multiple_of_6 in 1.. {
        let factor = 6 * multiple_of_6 - 1;
        if n % factor == 0 {
            return false;
        }

        let factor = 6 * multiple_of_6 + 1;
        if n % factor == 0 {
            return false;
        }

        if factor * factor >= n {
            return true;
        }
    }
    true
}

#[inline]
fn composite_satisfies_conjecture(n: u64) -> bool {
    for s in 1.. {
        let twice_square = 2 * s * s;
        if twice_square >= n - 2 {
            return false;
        }
        let p = n - twice_square;
        if is_odd_prime(p) {
            return true;
        }
    }
    false
}
