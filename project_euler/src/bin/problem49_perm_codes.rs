use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000);
}

// We can search for the first new solution, as the problem statement says there is only one other.
// But to compare performance of different algorithms, it is better to search exhaustively.
// Otherwise an algorithm may seem better simply because it searched in a luckier order.
const EXHAUSTIVE_SEARCH: bool = false;

fn solve() -> u64 {
    let mut solution = 0;
    let mut perm_hash: HashMap<u32, Vec<u16>> = HashMap::new();

    // 4 digit prime numbers are all of the form 6m +/-1.
    // For each such number (if not divisible by 5) add it to a hash map, where the hash key
    // is the same for all numbers that are permutations of one another (and unique to that set).
    // Start from 1002, as it is the smallest value of 6m such that 6m-1 is a 4 digit number.
    for n in (1002..10_000).step_by(6) {
        add_to_hash(&mut perm_hash, (n - 1) as u16);
        add_to_hash(&mut perm_hash, (n + 1) as u16);
    }

    for perms in perm_hash.values() {
        let perm_count = perms.len();
        if perm_count < 4 {
            continue;
        }
        for i in 0..perm_count - 2 {
            let perm1 = perms[i];
            if !is_prime(perm1) {
                continue;
            }
            for j in (i + 1)..(perm_count - 1) {
                let perm2 = perms[j];
                let perm3 = 2 * perm2 - perm1;
                if perms.contains(&perm3)
                    && is_prime(perm2)
                    && is_prime(perm3)
                    && (perm1 != 1487 || perm2 != 4817)
                {
                    solution = (perm1 as u64) * 100_000_000 + (perm2 as u64) * 10_000 + (perm3 as u64);
                    if !EXHAUSTIVE_SEARCH {
                        // use the first solution found, as the problem statement says it's unique
                        return solution;
                    }
                }
            }
        }
    }

    if EXHAUSTIVE_SEARCH {
        return solution;
    }
    panic!("No solution found")
}

#[inline]
fn add_to_hash(perm_hash: &mut HashMap<u32, Vec<u16>>, n: u16) {
    if n % 5 != 0 {
        let code = perm_code(n);
        let mut entry = perm_hash.entry(code);
        match entry {
            Entry::Vacant(_) => {
                let mut new_vec = Vec::with_capacity(24); // = 4! (when all 4 digits are unique)
                new_vec.push(n);
                entry.or_insert(new_vec);
            }
            Entry::Occupied(ref mut occupied_entry) => {
                occupied_entry.get_mut().push(n);
            }
        }
    }
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
fn is_prime(n: u16) -> bool {
    // n is of the form 6n +/- 1, so is not divisible by 2 or 3.
    // Check for factors of the form 6k +/- 1, since all other primes must fit this form.
    for k in 1.. {
        let factor = 6 * k - 1;
        if n % factor == 0 {
            return false;
        }

        let factor = 6 * k + 1;
        if n % factor == 0 {
            return false;
        }

        if factor * factor >= n {
            return true;
        }
    }
    true
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
