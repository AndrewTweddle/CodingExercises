fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000);
}

fn solve() -> u16 {
    (2_u16..100)
        .map(|a| {
            let mut digits: [u8; 203] = [0; 203];
            digits[0] = 1;
            let mut max_nonzero_digit = 0;
            (1..100)
                .map(|_| {
                    let mut digit_sum: u16 = 0;
                    let mut carry = 0;
                    for digit in digits.iter_mut().take(max_nonzero_digit + 4) {
                        let new_digit = a * (*digit as u16) + carry;
                        let units = new_digit % 10;
                        *digit = units as u8;
                        digit_sum += units;
                        carry = new_digit / 10;
                    }
                    max_nonzero_digit = match digits[max_nonzero_digit..=(max_nonzero_digit + 3)] {
                        [_, 0, 0, 0] => max_nonzero_digit,
                        [_, _, 0, 0] => max_nonzero_digit + 1,
                        [_, _, _, 0] => max_nonzero_digit + 2,
                        _ => max_nonzero_digit + 3,
                    };
                    digit_sum
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
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
