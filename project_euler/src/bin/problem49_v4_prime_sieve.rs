fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000);
}

// We can search for the first new solution, as the problem statement says there is only one other.
// But to compare performance of different algorithms, it is better to search exhaustively.
// Otherwise an algorithm may seem better simply because it searched in a luckier order.
const EXHAUSTIVE_SEARCH: bool = true;

type PrimeAndHashCode = (u16, u32);

fn solve() -> u64 {
    let mut solution = 0;
    let primes_and_hashes = get_primes_and_hash_codes();
    for i in 0..primes_and_hashes.len() {
        let (prime1, code1) = primes_and_hashes[i];
        for j in (i + 1)..primes_and_hashes.len() {
            let (prime2, code2) = primes_and_hashes[j];
            if (code1 != code2) || (prime1 == 1487 && prime2 == 4817) {
                continue;
            }
            let prime3 = 2 * prime2 - prime1;
            if prime3 < 10_000 && primes_and_hashes.binary_search(&(prime3, code1)).is_ok() {
                solution =
                    (prime1 as u64) * 100_000_000 + (prime2 as u64) * 10_000 + (prime3 as u64);
                if !EXHAUSTIVE_SEARCH {
                    // use the first solution found, as the problem statement says it's unique
                    return solution;
                }
            }
        }
    }

    if EXHAUSTIVE_SEARCH {
        return solution;
    }
    panic!("No solution found")
}

fn get_primes_and_hash_codes() -> Vec<PrimeAndHashCode> {
    let mut is_prime = [true; 10_001];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=10_000 {
        if is_prime[i] {
            for j in ((i * i)..=10_000).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .skip(1000)
        .filter(|&(_, &isp)| isp)
        .map(|(index, _)| {
            let prime = index as u16;
            let hash_code = perm_code(prime);
            (prime, hash_code)
        })
        .collect::<Vec<PrimeAndHashCode>>()
}

#[inline]
fn perm_code(n: u16) -> u32 {
    let mut code: u32 = 0;
    for d in digits(n) {
        // Use 3 bits per decimal digit to count the # of that digit in n.
        // With 10 possible digits this uses 3 * 10 bits to form a unique number per set of digits.
        code += 1 << (3 * d);
    }
    code
}

#[inline]
fn digits(mut n: u16) -> [u8; 4] {
    let units = n % 10;
    n /= 10;
    let tens = n % 10;
    n /= 10;
    let hundreds = n % 10;
    let thousands = n / 10;
    [thousands as u8, hundreds as u8, tens as u8, units as u8]
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
