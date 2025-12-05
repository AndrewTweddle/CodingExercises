use std::hint::black_box;
use std::time::Instant;

pub fn load_and_solve_and_benchmark<S, T>(
    file_path: &str,
    problem_desc: &str,
    solve: S,
    repetitions: u32,
) where
    S: Fn(&str) -> T,
    T: std::fmt::Debug,
{
    // The first timing includes file I/O
    let start_time = Instant::now();

    // first read the file contents into memory so that benchmarks don't include file I/O
    let contents = std::fs::read_to_string(file_path).expect("Input file not readable");

    // Solve once and print out the solution and its duration (including file I/O)
    let solution = solve(&contents);
    println!("{problem_desc}: {solution:?}");
    println!(
        "Solved (including writing to terminal) in {:?}",
        start_time.elapsed()
    );

    // Benchmark many runs of the solver (excluding file I/O)
    if repetitions > 0 {
        benchmark(solve, &contents, repetitions);
    }
}

pub fn benchmark<S, T>(solve: S, contents: &str, repetitions: u32)
where
    S: Fn(&str) -> T,
{
    let start_time = Instant::now();
    for _ in 0..repetitions {
        black_box(solve(black_box(contents)));
    }

    if repetitions > 0 {
        let avg_duration = start_time.elapsed() / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
    }
}
