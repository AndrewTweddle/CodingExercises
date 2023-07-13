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
    (3..)
        .step_by(2)
        .find(is_composite_but_does_not_satisfy_conjecture)
}

#[inline]
fn is_composite_but_does_not_satisfy_conjecture(&n: &u64) -> bool {
    if is_odd_prime(n) {
        return false;
    }

    // It's a composite, so check the conjecture
    for s in 1.. {
        let twice_square = 2 * s * s;
        if twice_square >= n - 2 {
            return true;
        }
        let p = n - twice_square;
        if is_odd_prime(p) {
            return false;
        }
    }
    true
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
