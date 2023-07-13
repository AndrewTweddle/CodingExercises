use num::integer::Roots;
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
    let mut odd_primes: Vec<u64> = Vec::with_capacity(1000);
    for i in (3..).step_by(2) {
        if is_odd_prime(i) {
            odd_primes.push(i);
        } else if !composite_satisfies_conjecture(i, &odd_primes) {
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

    // if n is prime, and greater than 3, then it must be of the form 6k+1 or 6k+5...
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
fn composite_satisfies_conjecture(n: u64, odd_primes: &[u64]) -> bool {
    for p in odd_primes {
        let s = (n - p) / 2;
        if is_square(s) {
            return true;
        }
    }
    false
}

#[inline]
fn is_square(n: u64) -> bool {
    let s = n.sqrt();
    s * s == n
}
