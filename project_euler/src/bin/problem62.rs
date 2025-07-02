use rustc_hash::FxHashMap as HashMap;

const TARGET_CUBE_COUNT: usize = 5;
const BITS_PER_DIGIT_IN_CODE: u32 = 4;

type CodeSize = u64;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000)
}

fn solve() -> CodeSize {
    let mut mult: CodeSize = 1;
    let mut digit_count: CodeSize = 0;
    let mut cubes_by_permutation_code: HashMap<CodeSize, Vec<CodeSize>> = HashMap::default();
    let mut permutation_codes_with_enough_cubes: Vec<CodeSize> = Vec::new();

    for i in 1.. {
        let cube = i * i * i;

        // Whenever the number of digits in the cube goes up, look for a solution.
        // If none is found, clear the data and try with the extra number of digits.
        if cube > mult {
            // Check if a solution has been found yet
            if !permutation_codes_with_enough_cubes.is_empty() {
                let min_cubic_permutation = permutation_codes_with_enough_cubes
                    .iter()
                    .filter_map(|code| {
                        if cubes_by_permutation_code[code].len() == TARGET_CUBE_COUNT {
                            cubes_by_permutation_code[code].iter().min()
                        } else {
                            None
                        }
                    })
                    .min();
                if let Some(&min_cube) = min_cubic_permutation {
                    return min_cube;
                }
            }

            // Clear the data and try again
            while cube > mult {
                mult *= 10;
                digit_count += 1;
            }
            cubes_by_permutation_code.clear();
            permutation_codes_with_enough_cubes.clear();

            // A 15 digit number has 10 ^ 15 = (10 ^ 3) ^ 5 < (2^10) ^ 5 = 2 ^ 50 < 2 ^ 64.
            if digit_count > 15 {
                panic!("Using a u64, at most 15 digits are supported");
            }
        }

        // Convert the cube to a permutation code - a number representing the
        // count of the decimal digits in the cubic number.
        // All numbers sharing the same permutation code are permutations of one another.
        let mut code = 0;
        let mut rem_digits = cube;
        for _ in 0..digit_count {
            let digit = 1 << (BITS_PER_DIGIT_IN_CODE * (rem_digits % 10) as u32);
            code += digit;
            rem_digits /= 10;
        }
        let cubes = cubes_by_permutation_code.entry(code).or_default();
        cubes.push(cube);
        if cubes.len() == TARGET_CUBE_COUNT {
            permutation_codes_with_enough_cubes.push(code);
        }
    }
    panic!("Should never be able to reach this point");
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
