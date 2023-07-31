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

    // A hash map will be used to group all numbers which have the same set of digits.
    // The hash key will be a code calculated in such a way that it is is shared by all permutations
    // of a particular set of 4 digits, but not by any other set of 4 digits
    // In particular, use 30 bits of a u32, with 3 bits used per decimal digit from 0 to 9.
    // Each set of 3 bits will count the number of that digit in the set of digits.
    // The values will be 4 digit numbers with those digits (and hence permutations of one another).
    let mut perm_hash: HashMap<u32, Vec<u16>> = HashMap::new();

    // Build up the digits a, b, c, d of each possible 4 digit number "abcd" which might be prime
    for a in 1..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                // multi-digit prime numbers always end in 1, 3, 7 or 9.
                for d in [1, 3, 7, 9] {
                    let code = (1 << (3 * a)) + (1 << (3 * b)) + (1 << (3 * c)) + (1 << (3 * d));
                    let n = (1000 * a + 100 * b + 10 * c + d) as u16;
                    let mut entry = perm_hash.entry(code);
                    match entry {
                        Entry::Vacant(_) => {
                            let mut new_vec = Vec::with_capacity(24); // 4! permutations of 4 digits
                            new_vec.push(n);
                            entry.or_insert(new_vec);
                        }
                        Entry::Occupied(ref mut occupied_entry) => {
                            occupied_entry.get_mut().push(n);
                        }
                    }
                }
            }
        }
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
                    solution =
                        (perm1 as u64) * 100_000_000 + (perm2 as u64) * 10_000 + (perm3 as u64);
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
fn is_prime(n: u16) -> bool {
    // n is not divisible by 2 or 5, since its last digit is 1, 3, 7 or 9
    if n % 3 == 0 {
        return false;
    }

    // So n's prime factors must be of the form 6k +/- 1.
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
