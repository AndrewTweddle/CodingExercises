fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000);
}

const TEN_TO_THE_TEN: u64 = 10_000_000_000;

fn solve() -> u64 {
    let mut answer = 0;
    for i in 1..=1000 {
        answer += self_power_mod_10_to_the_10(i);
        answer %= TEN_TO_THE_TEN;
    }
    answer
}

fn self_power_mod_10_to_the_10(n: u64) -> u64 {
    let mut q = n;
    let mut result = n as u128;
    while q % 2 == 0 {
        result *= result;
        result %= TEN_TO_THE_TEN as u128;
        q /= 2;
    }
    for i in (3..).step_by(2) {
        while q % i == 0 {
            let mut p = result;
            for _ in 1..i {
                p *= result;
                p %= TEN_TO_THE_TEN as u128;
            }
            result = p;
            q /= i;
        }
        if q == 1 {
            break;
        }
    }
    result as u64
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
