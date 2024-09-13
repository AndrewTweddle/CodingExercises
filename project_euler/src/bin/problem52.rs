fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000);
}

type DigitCount = [u8; 10];

fn solve() -> u128 {
    for x in 1.. {
        let digits = get_digit_count(x);
        if (2..=6).all(|m| get_digit_count(m * x) == digits) {
            #[cfg(debug_assertions)]
            {
                for m in 1..=6 {
                    println!("{m} x {x} = {}", m * x);
                }
            }
            return x;
        }
    }
    panic!("No digits found");
}

fn get_digit_count(x: u128) -> DigitCount {
    let mut digits = [0; 10];
    let mut q = x;
    let mut r;
    
    while q != 0 {
        r = q % 10;
        digits[r as usize] += 1;
        q /= 10;
    }
    digits
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
