use std::time::Instant;

pub fn read_and_solve_and_time_more_runs<S, T>(
    file_path: &str,
    problem_desc: &str,
    solve: S,
    repetitions: u32,
) where
    S: Fn(&str) -> T,
    T: std::fmt::Debug,
{
    // first read file contents into memory, so that timings don't include file I/O
    let contents = std::fs::read_to_string(file_path).expect("Input file not readable");

    solve_and_time_more_runs(problem_desc, || solve(&contents), repetitions);
}

pub fn solve_and_time_more_runs<S, T>(problem_desc: &str, solve: S, repetitions: u32)
where
    S: Fn() -> T,
    T: std::fmt::Debug,
{
    let mut start_time = Instant::now();
    for i in 0..=repetitions {
        let solution = solve();
        if i == 0 {
            println!("{problem_desc}: {solution:?}");
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
