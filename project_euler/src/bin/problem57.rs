fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000);
}

fn solve() -> usize {
    let mut n: Vec<u8> = Vec::with_capacity(1000);
    let mut d: Vec<u8> = Vec::with_capacity(1000);
    n.push(1);
    d.push(1);

    (1..=1000)
        .filter(|_iteration| {
            // 1 + 1 / (1 + n/d) = 1 + 1 / [(d + n) / d] = 1 + d / (d + n) = (2d + n)/(d + n). So:
            // next n = 2d + n
            // next d = d + n
            // Not that the gcd of (2d + n, d + n) also divides
            // d = (2d+n) - (d+n) and n = 2(d + n) - (2d + n).
            // So if d and n are relatively prime then gcd(d, n) = 1. So gcd(next d, next n) = 1.
            // Initial n/d = 3/2. So by induction all n and d are relatively prime and in their
            // simplest form. So we don't need to simplify the fractions.

            // Update n and d one digit at a time:
            let n_len = n.len();
            let d_len = d.len();
            let max_digit = n_len.max(d_len) + 1;
            let mut n_carry = 0;
            let mut d_carry = 0;
            for i in 0..max_digit {
                let n_digit = get_digit(&n, i);
                let d_digit = get_digit(&d, i);
                n_carry += 2 * d_digit + n_digit;
                d_carry += d_digit + n_digit;
                set_digit(&mut n, i, n_carry % 10);
                set_digit(&mut d, i, d_carry % 10);
                n_carry /= 10;
                d_carry /= 10;
            }
            n.len() > d.len()
        })
        .count()
}

#[inline(always)]
fn get_digit(digits: &[u8], index: usize) -> u8 {
    if index < digits.len() {
        digits[index]
    } else {
        0
    }
}

#[inline(always)]
fn set_digit(digits: &mut Vec<u8>, index: usize, value: u8) {
    if value == 0 && index >= digits.len() {
        return;
    }
    while index > digits.len() {
        digits.push(0);
    }
    if index == digits.len() {
        digits.push(value);
    } else {
        digits[index] = value;
    }
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
